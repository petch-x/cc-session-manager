use crate::models::{Project, Session, Statistics};
use anyhow::{anyhow, Result};
use serde_json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct SessionManager {
    claude_dir: Option<PathBuf>,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let manager = Self { claude_dir: None };
        Ok(manager)
    }

    pub fn find_claude_directory(&mut self) -> Result<Option<PathBuf>> {
        if let Some(ref dir) = self.claude_dir {
            return Ok(Some(dir.clone()));
        }

        // Look for ~/.claude directory (cross-platform)
        if let Some(home_dir) = dirs::home_dir() {
            let claude_dir = home_dir.join(".claude");
            if claude_dir.exists() && claude_dir.is_dir() {
                self.claude_dir = Some(claude_dir.clone());
                return Ok(Some(claude_dir));
            }
        }

        Ok(None)
    }

    pub fn scan_projects(&mut self) -> Result<Vec<Project>> {
        let claude_dir = match self.find_claude_directory()? {
            Some(dir) => dir,
            None => return Err(anyhow!("Claude directory not found")),
        };

        let projects_dir = claude_dir.join("projects");
        
        // If projects directory doesn't exist, return empty list
        if !projects_dir.exists() || !projects_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        for entry in fs::read_dir(projects_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let project_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let mut project = Project::new(project_name.clone(), path.clone());
                
                // Scan sessions in this project
                let sessions = self.scan_sessions(&path)?;
                for session in sessions {
                    project.add_session(session);
                }

                // Extract latest content from the most recent session
                if let Some(latest_session) = project.sessions.first() {
                    project.latest_content = self.extract_session_preview(&latest_session.path);
                }

                // Add projects even if they have no sessions (user might want to delete empty projects)
                projects.push(project);
            }
        }

        // Sort projects by name
        projects.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(projects)
    }

    pub fn scan_sessions(&self, project_path: &Path) -> Result<Vec<Session>> {
        let mut sessions = Vec::new();

        if !project_path.exists() || !project_path.is_dir() {
            return Ok(sessions);
        }

        for entry in fs::read_dir(project_path)? {
            let entry = entry?;
            let path = entry.path();

            // Look for JSONL files (Claude sessions)
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "jsonl" {
                        let metadata = fs::metadata(&path)?;
                        let size = metadata.len();
                        let modified = metadata.modified()?;

                        let session_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let mut session = Session::new(session_name, path, size, modified);
                        
                        // Extract content preview for this session
                        session.content_preview = self.extract_session_preview(&session.path);

                        sessions.push(session);
                    }
                }
            }
        }

        // Sort sessions by modification time (newest first)
        sessions.sort_by(|a, b| b.modified.cmp(&a.modified));

        Ok(sessions)
    }

    pub fn delete_sessions(&self, sessions: &[Session]) -> Result<usize> {
        let mut deleted_count = 0;

        for session in sessions {
            match fs::remove_file(&session.path) {
                Ok(_) => deleted_count += 1,
                Err(e) => eprintln!("Failed to delete {}: {}", session.path.display(), e),
            }
        }

        Ok(deleted_count)
    }

    pub fn delete_project(&self, project: &Project) -> Result<()> {
        fs::remove_dir_all(&project.path)?;
        Ok(())
    }

    pub fn get_statistics(&mut self) -> Result<Statistics> {
        let projects = self.scan_projects()?;
        let total_projects = projects.len();
        let total_sessions = projects.iter().map(|p| p.sessions.len()).sum();
        let total_size = projects.iter().map(|p| p.total_size).sum();

        Ok(Statistics {
            total_projects,
            total_sessions,
            total_size,
        })
    }

    pub fn filter_by_age<'a>(&self, sessions: &'a [Session], days: u64) -> Vec<&'a Session> {
        sessions
            .iter()
            .filter(|session| session.get_age_days() > days)
            .collect()
    }

    fn extract_session_preview(&self, session_path: &Path) -> Option<String> {
        let file = match File::open(session_path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let reader = BufReader::new(file);
        
        // Read first few lines to find a meaningful content
        for line in reader.lines().take(5) {
            match line {
                Ok(content) => {
                    let trimmed = content.trim();
                    if !trimmed.is_empty() {
                        // Try to parse as JSON to extract meaningful content
                        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(trimmed) {
                            // Look for common message content fields
                            if let Some(content_field) = self.extract_content_from_json(&json_value) {
                                return Some(self.truncate_content(&content_field, 80));
                            }
                        } else {
                            // If not JSON, use the line as-is
                            return Some(self.truncate_content(trimmed, 80));
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        None
    }

    fn extract_content_from_json(&self, json: &serde_json::Value) -> Option<String> {
        // Look for content in Claude session JSON format
        if let Some(obj) = json.as_object() {
            // First check if there's a message field with content
            if let Some(message) = obj.get("message") {
                if let Some(message_obj) = message.as_object() {
                    if let Some(content) = message_obj.get("content") {
                        match content {
                            serde_json::Value::String(s) => return Some(s.clone()),
                            serde_json::Value::Array(arr) => {
                                // If it's an array, try to get text from first text element
                                for item in arr {
                                    if let Some(item_obj) = item.as_object() {
                                        if let Some(text) = item_obj.get("text").and_then(|v| v.as_str()) {
                                            return Some(text.to_string());
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            // Fallback: try direct content field
            if let Some(content) = obj.get("content") {
                match content {
                    serde_json::Value::String(s) => return Some(s.clone()),
                    serde_json::Value::Array(arr) => {
                        for item in arr {
                            if let Some(item_obj) = item.as_object() {
                                if let Some(text) = item_obj.get("text").and_then(|v| v.as_str()) {
                                    return Some(text.to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn truncate_content(&self, content: &str, max_len: usize) -> String {
        if content.chars().count() <= max_len {
            content.to_string()
        } else {
            let truncated: String = content.chars().take(max_len.saturating_sub(3)).collect();
            format!("{}...", truncated)
        }
    }


}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self { claude_dir: None })
    }
}
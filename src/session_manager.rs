use crate::models::{Project, Session, Statistics};
use anyhow::{anyhow, Result};
use std::fs;
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

                        sessions.push(Session::new(session_name, path, size, modified));
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


}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self { claude_dir: None })
    }
}
use crate::models::Session;
use crate::SessionManager;
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ProjectDto {
    pub name: String,
    pub path: String,
    pub session_count: usize,
    pub total_size: String,
    pub sessions: Vec<SessionDto>,
}

#[derive(Serialize)]
pub struct SessionDto {
    pub name: String,
    pub path: String,
    pub size: String,
    pub age_days: u64,
    pub content_preview: Option<String>,
}

#[derive(Serialize)]
pub struct StatisticsDto {
    pub total_projects: usize,
    pub total_sessions: usize,
    pub total_size: String,
}

#[tauri::command]
pub fn get_statistics() -> Result<StatisticsDto, String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let stats = manager.get_statistics()
        .map_err(|e| format!("Failed to get statistics: {}", e))?;
    
    Ok(StatisticsDto {
        total_projects: stats.total_projects,
        total_sessions: stats.total_sessions,
        total_size: stats.format_total_size(),
    })
}

#[tauri::command]
pub fn scan_projects() -> Result<Vec<ProjectDto>, String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let projects = manager.scan_projects()
        .map_err(|e| format!("Failed to scan projects: {}", e))?;
    
    let dtos: Vec<ProjectDto> = projects
        .into_iter()
        .map(|p| {
            let total_size = p.format_size();
            let sessions: Vec<SessionDto> = p.sessions
                .into_iter()
                .map(|s| {
                    let size = s.format_size();
                    let age_days = s.get_age_days();
                    SessionDto {
                        name: s.name,
                        path: s.path.to_string_lossy().to_string(),
                        size,
                        age_days,
                        content_preview: s.content_preview,
                    }
                })
                .collect();
            ProjectDto {
                name: p.name,
                path: p.path.to_string_lossy().to_string(),
                session_count: sessions.len(),
                total_size,
                sessions,
            }
        })
        .collect();
    
    Ok(dtos)
}

#[tauri::command]
pub fn get_project_sessions(project_path: String) -> Result<Vec<SessionDto>, String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let projects = manager.scan_projects()
        .map_err(|e| format!("Failed to scan projects: {}", e))?;
    
    let path = PathBuf::from(project_path);
    let project = projects
        .into_iter()
        .find(|p| p.path == path)
        .ok_or_else(|| anyhow!("Project not found").to_string())?;
    
    let dtos: Vec<SessionDto> = project
        .sessions
        .into_iter()
        .map(|s| {
            let size = s.format_size();
            let age_days = s.get_age_days();
            SessionDto {
                name: s.name,
                path: s.path.to_string_lossy().to_string(),
                size,
                age_days,
                content_preview: s.content_preview,
            }
        })
        .collect();
    
    Ok(dtos)
}

#[tauri::command]
pub fn delete_sessions(session_paths: Vec<String>) -> Result<usize, String> {
    let manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let sessions: Vec<Session> = session_paths
        .into_iter()
        .filter_map(|p| {
            let path = PathBuf::from(&p);
            if path.exists() {
                std::fs::metadata(&path).ok().map(|meta| Session {
                    name: path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    path,
                    size: meta.len(),
                    modified: meta.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
                    content_preview: None,
                })
            } else {
                None
            }
        })
        .collect();
    
    let deleted_count = manager.delete_sessions(&sessions)
        .map_err(|e| format!("Failed to delete sessions: {}", e))?;
    
    Ok(deleted_count)
}

#[tauri::command]
pub fn delete_project(project_path: String) -> Result<(), String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let projects = manager.scan_projects()
        .map_err(|e| format!("Failed to scan projects: {}", e))?;
    
    let path = PathBuf::from(project_path);
    let project = projects
        .into_iter()
        .find(|p| p.path == path)
        .ok_or_else(|| anyhow!("Project not found").to_string())?;
    
    manager.delete_project(&project)
        .map_err(|e| format!("Failed to delete project: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub fn filter_sessions_by_age(days: u64) -> Result<Vec<SessionDto>, String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let projects = manager.scan_projects()
        .map_err(|e| format!("Failed to scan projects: {}", e))?;
    
    let mut all_sessions: Vec<(String, Session)> = Vec::new();
    
    for project in &projects {
        let old_sessions = manager.filter_by_age(&project.sessions, days);
        for session in old_sessions {
            all_sessions.push((project.path.to_string_lossy().to_string(), session.clone()));
        }
    }
    
    let dtos: Vec<SessionDto> = all_sessions
        .into_iter()
        .map(|(_, s)| {
            let size = s.format_size();
            let age_days = s.get_age_days();
            SessionDto {
                name: s.name,
                path: s.path.to_string_lossy().to_string(),
                size,
                age_days,
                content_preview: s.content_preview,
            }
        })
        .collect();
    
    Ok(dtos)
}

#[tauri::command]
pub fn delete_old_sessions(days: u64) -> Result<usize, String> {
    let mut manager = SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    let projects = manager.scan_projects()
        .map_err(|e| format!("Failed to scan projects: {}", e))?;
    
    let mut sessions_to_delete: Vec<Session> = Vec::new();
    
    for project in &projects {
        let old_sessions = manager.filter_by_age(&project.sessions, days);
        for session in old_sessions {
            sessions_to_delete.push(session.clone());
        }
    }
    
    let deleted_count = manager.delete_sessions(&sessions_to_delete)
        .map_err(|e| format!("Failed to delete sessions: {}", e))?;
    
    Ok(deleted_count)
}

#[tauri::command]
pub fn get_session_content(session_path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&session_path)
        .map_err(|e| format!("Failed to read session content: {}", e))?;
    
    let formatted = format_session_content(&content);
    Ok(formatted)
}

fn format_session_content(content: &str) -> String {
    let mut result = String::new();
    let mut has_any_content = false;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        match serde_json::from_str::<serde_json::Value>(line) {
            Ok(json) => {
                let msg_type = json.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
                let role = json.get("message").and_then(|m| m.get("role")).and_then(|v| v.as_str()).unwrap_or("entry");
                
                let role_label = match (msg_type, role) {
                    ("user", _) => "👤 USER",
                    ("assistant", _) => "🤖 CLAUDE",
                    ("system", _) => "⚙️ SYSTEM",
                    ("tool", _) => "🛠️ TOOL",
                    ("function", _) => "📌 FUNCTION",
                    _ => match role {
                        "user" => "👤 USER",
                        "assistant" => "🤖 CLAUDE",
                        "system" => "⚙️ SYSTEM",
                        "tool" => "🛠️ TOOL",
                        "function" => "📌 FUNCTION",
                        _ => "📝 ENTRY",
                    },
                };
                
                let extracted = extract_all_text(&json);
                
                if extracted.is_empty() {
                    continue;
                }
                
                has_any_content = true;
                result.push_str(&format!("─── {} ───\n", role_label));
                result.push_str(&extracted);
                result.push_str("\n\n");
            }
            Err(_) => {
                if !line.is_empty() {
                    has_any_content = true;
                    result.push_str(line);
                    result.push_str("\n\n");
                }
            }
        }
    }
    
    if !has_any_content {
        "No content available".to_string()
    } else {
        result.trim_end().to_string()
    }
}

fn extract_all_text(value: &serde_json::Value) -> String {
    let mut texts = Vec::new();
    extract_text_recursive(value, &mut texts, 0);
    texts.join("\n\n").trim().to_string()
}

fn extract_text_recursive(value: &serde_json::Value, texts: &mut Vec<String>, depth: usize) {
    if depth > 15 {
        return;
    }
    
    if let Some(message) = value.get("message") {
        if let Some(content_val) = message.get("content") {
            if let Some(content_arr) = content_val.as_array() {
                for block in content_arr {
                    extract_content_block(block, texts, depth);
                }
                return;
            }
            if let Some(content_str) = content_val.as_str() {
                let converted = content_str.replace("\\n", "\n").trim().to_string();
                if !converted.is_empty() && converted != "{}" {
                    texts.push(converted);
                }
            }
        }
    }
    
    if let Some(content_val) = value.get("content") {
        if let Some(content_arr) = content_val.as_array() {
            for block in content_arr {
                extract_content_block(block, texts, depth);
            }
            return;
        }
    }
    
    match value {
        serde_json::Value::String(s) => {
            if !s.trim().is_empty() {
                let converted = s.replace("\\n", "\n");
                let trimmed = converted.trim();
                if !trimmed.is_empty() && trimmed != "{}" && !trimmed.starts_with("{") {
                    texts.push(trimmed.to_string());
                }
            }
        }
        serde_json::Value::Object(obj) => {
            for (key, val) in obj.iter() {
                match key.as_str() {
                    "text" | "content" | "message" | "output" | "result" | "input" | "thought" | "reasoning" => {
                        extract_text_recursive(val, texts, depth + 1);
                    }
                    _ => {}
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter() {
                extract_content_block(item, texts, depth);
            }
        }
        _ => {}
    }
}

fn extract_content_block(block: &serde_json::Value, texts: &mut Vec<String>, depth: usize) {
    if let Some(block_obj) = block.as_object() {
        if let Some(text) = block_obj.get("text").and_then(|v| v.as_str()) {
            let converted = text.replace("\\n", "\n").trim().to_string();
            if !converted.is_empty() {
                texts.push(converted);
            }
        }
        
        if let Some(thinking) = block_obj.get("thinking").and_then(|v| v.as_str()) {
            let converted = thinking.replace("\\n", "\n").trim().to_string();
            if !converted.is_empty() {
                texts.push(format!("[Thinking]\n{}", converted));
            }
        }
        
        if let Some(tool_name) = block_obj.get("name").and_then(|v| v.as_str()) {
            let mut tool_text = format!("[Tool: {}]", tool_name);
            if let Some(input_val) = block_obj.get("input") {
                if let Some(input) = input_val.as_object() {
                    let input_str = format!("{:?}", input);
                    if input_str.len() > 10 {
                        tool_text.push_str(&format!("\n{}", input_str));
                    }
                }
            }
            texts.push(tool_text);
        }
        
        if let Some(result) = block_obj.get("content") {
            match result {
                serde_json::Value::String(s) => {
                    let converted = s.replace("\\n", "\n").trim().to_string();
                    if !converted.is_empty() {
                        texts.push(converted);
                    }
                }
                serde_json::Value::Object(obj) => {
                    if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
                        let converted = text.replace("\\n", "\n").trim().to_string();
                        if !converted.is_empty() {
                            texts.push(converted);
                        }
                    }
                }
                _ => {}
            }
        }
        
        if let Some(nested_val) = block_obj.get("content") {
            if let Some(nested) = nested_val.as_array() {
                for item in nested {
                    extract_content_block(item, texts, depth + 1);
                }
            }
        }
        
        if let Some(content_str) = block_obj.get("content").and_then(|v| v.as_str()) {
            let converted = content_str.replace("\\n", "\n").trim().to_string();
            if !converted.is_empty() {
                texts.push(converted);
            }
        }
    }
}

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub modified: SystemTime,
}

impl Session {
    pub fn new(name: String, path: PathBuf, size: u64, modified: SystemTime) -> Self {
        Self {
            name,
            path,
            size,
            modified,
        }
    }

    pub fn get_modified_datetime(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        let datetime: DateTime<Utc> = self.modified.into();
        Ok(datetime.with_timezone(&Local))
    }

    pub fn get_age_days(&self) -> u64 {
        let now = SystemTime::now();
        let duration = now.duration_since(self.modified).unwrap_or_default();
        duration.as_secs() / (60 * 60 * 24)
    }

    pub fn format_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = self.size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub sessions: Vec<Session>,
    pub total_size: u64,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            sessions: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.total_size += session.size;
        self.sessions.push(session);
    }

    pub fn format_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = self.total_size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

#[derive(Debug)]
pub struct Statistics {
    pub total_projects: usize,
    pub total_sessions: usize,
    pub total_size: u64,
}

impl Statistics {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            total_projects: 0,
            total_sessions: 0,
            total_size: 0,
        }
    }

    pub fn format_total_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = self.total_size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuChoice {
    Statistics,
    ManageProjects,
    DeleteByAge,
    DeleteProject,
    Exit,
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::SystemTime;

    #[test]
    fn test_session_format_size() {
        let session = Session::new(
            "test.json".to_string(),
            PathBuf::from("/test/test.json"),
            1024,
            SystemTime::now(),
        );
        assert_eq!(session.format_size(), "1.0 KB");

        let session = Session::new(
            "test.json".to_string(),
            PathBuf::from("/test/test.json"),
            1500,
            SystemTime::now(),
        );
        assert_eq!(session.format_size(), "1.5 KB");

        let session = Session::new(
            "test.json".to_string(),
            PathBuf::from("/test/test.json"),
            1024 * 1024,
            SystemTime::now(),
        );
        assert_eq!(session.format_size(), "1.0 MB");
    }

    #[test]
    fn test_project_new() {
        let project = Project::new(
            "test-project".to_string(),
            PathBuf::from("/test/test-project"),
        );
        assert_eq!(project.name, "test-project");
        assert_eq!(project.sessions.len(), 0);
        assert_eq!(project.total_size, 0);
    }

    #[test]
    fn test_project_add_session() {
        let mut project = Project::new(
            "test-project".to_string(),
            PathBuf::from("/test/test-project"),
        );

        let session = Session::new(
            "session1.json".to_string(),
            PathBuf::from("/test/test-project/session1.json"),
            1024,
            SystemTime::now(),
        );

        project.add_session(session);
        assert_eq!(project.sessions.len(), 1);
        assert_eq!(project.total_size, 1024);
    }

    #[test]
    fn test_statistics_new() {
        let stats = Statistics::new();
        assert_eq!(stats.total_projects, 0);
        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.total_size, 0);
    }

    #[test]
    fn test_menu_choice_equality() {
        assert_eq!(MenuChoice::Statistics, MenuChoice::Statistics);
        assert_ne!(MenuChoice::Statistics, MenuChoice::Exit);
    }
}
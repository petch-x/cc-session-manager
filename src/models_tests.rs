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
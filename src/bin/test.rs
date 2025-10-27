use cc_session_manager::SessionManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Claude Code Session Manager...");
    
    let mut session_manager = SessionManager::new()?;
    
    // Test finding Claude directory
    match session_manager.find_claude_directory()? {
        Some(dir) => println!("✅ Found Claude directory: {}", dir.display()),
        None => {
            println!("❌ Claude directory not found");
            println!("This is normal if you don't have Claude Code installed");
            return Ok(());
        }
    }
    
    // Test scanning projects
    match session_manager.scan_projects() {
        Ok(projects) => {
            println!("✅ Found {} projects", projects.len());
            for project in &projects {
                println!("  - {} ({} sessions, {} bytes)", 
                    project.name, 
                    project.sessions.len(), 
                    project.total_size);
            }
        }
        Err(e) => println!("❌ Error scanning projects: {}", e),
    }
    
    // Test getting statistics
    match session_manager.get_statistics() {
        Ok(stats) => {
            println!("✅ Statistics:");
            println!("  Total projects: {}", stats.total_projects);
            println!("  Total sessions: {}", stats.total_sessions);
            println!("  Total size: {} bytes", stats.total_size);
        }
        Err(e) => println!("❌ Error getting statistics: {}", e),
    }
    
    println!("✅ Test completed successfully!");
    Ok(())
}
use cc_session_manager::SessionManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Project Management Flow");
    println!("==================================");
    
    let mut session_manager = SessionManager::new()?;
    
    // Test 1: Find Claude directory
    match session_manager.find_claude_directory()? {
        Some(dir) => {
            println!("✅ Found Claude directory: {}", dir.display());
            let projects_dir = dir.join("projects");
            if projects_dir.exists() {
                println!("✅ Projects directory exists: {}", projects_dir.display());
            } else {
                println!("❌ Projects directory not found");
                return Ok(());
            }
        }
        None => {
            println!("❌ Claude directory not found");
            return Ok(());
        }
    }
    
    // Test 2: Scan projects
    let projects = session_manager.scan_projects()?;
    println!("✅ Found {} projects", projects.len());
    
    // Show only our test projects
    let test_projects: Vec<_> = projects.iter()
        .filter(|p| p.name.starts_with("test-project"))
        .collect();
    
    println!("📋 Test projects found:");
    for project in &test_projects {
        println!("  📁 {} ({} sessions, {})", 
            project.name, 
            project.sessions.len(), 
            project.format_size());
            
        // Show sessions in each project
        for (i, session) in project.sessions.iter().enumerate() {
            let age = session.get_age_days();
            println!("    [{}] {} ({}, {} days ago)", 
                i + 1, 
                session.name, 
                session.format_size(),
                age);
        }
    }
    
    // Test 3: Session selection simulation
    if !test_projects.is_empty() {
        let test_project = &test_projects[0];
        if !test_project.sessions.is_empty() {
            println!("\n🎯 Simulating session selection for project: {}", test_project.name);
            
            // Simulate selecting first session
            let selected_session = &test_project.sessions[0];
            println!("📌 Selected session: {} ({})", 
                selected_session.name, 
                selected_session.format_size());
            
            // Simulate deletion (commented out for safety)
            // let deleted = session_manager.delete_sessions(&[selected_session.clone()])?;
            // println!("✅ Deleted {} session", deleted);
        }
    }
    
    println!("\n✅ Project management test completed!");
    println!("🚀 Ready to run: ./target/release/cc-session-manager");
    
    Ok(())
}
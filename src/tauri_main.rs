#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cc_session_manager::{
    commands::{
        delete_old_sessions,
        delete_project,
        delete_sessions,
        filter_sessions_by_age,
        get_project_sessions,
        get_statistics,
        scan_projects,
    },
};

#[tauri::command]
fn find_claude_directory() -> Result<Option<String>, String> {
    let mut manager = cc_session_manager::SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;
    
    manager.find_claude_directory()
        .map(|opt| opt.map(|p| p.to_string_lossy().to_string()))
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_statistics,
            scan_projects,
            get_project_sessions,
            delete_sessions,
            delete_project,
            filter_sessions_by_age,
            delete_old_sessions,
            find_claude_directory,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("Claude Code Session Manager")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

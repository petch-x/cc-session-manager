#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn find_claude_directory() -> Result<Option<String>, String> {
    let mut manager = cc_session_manager::SessionManager::new()
        .map_err(|e| format!("Failed to create session manager: {}", e))?;

    manager.find_claude_directory()
        .map(|opt| opt.map(|p| p.to_string_lossy().to_string()))
        .map_err(|e| e.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("Claude Code Session Manager")?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cc_session_manager::commands::get_statistics,
            cc_session_manager::commands::scan_projects,
            cc_session_manager::commands::get_project_sessions,
            cc_session_manager::commands::delete_sessions,
            cc_session_manager::commands::delete_project,
            cc_session_manager::commands::filter_sessions_by_age,
            cc_session_manager::commands::delete_old_sessions,
            cc_session_manager::commands::get_session_content,
            find_claude_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

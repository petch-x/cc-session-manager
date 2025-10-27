mod models;
mod session_manager;
mod ui;
mod utils;

use anyhow::Result;
use models::MenuChoice;
use session_manager::SessionManager;
use ui::UI;

fn main() -> Result<()> {
    let mut session_manager = SessionManager::new()?;
    let ui = UI::new();

    // Check if Claude directory exists
    if session_manager.find_claude_directory()?.is_none() {
        ui.show_error("Claude directory not found. Please check if Claude Code is installed");
        return Ok(());
    }

    loop {
        match ui.show_main_menu()? {
            MenuChoice::Statistics => {
                let stats = session_manager.get_statistics()?;
                ui.show_statistics(&stats);
            }
            MenuChoice::ManageProjects => {
                let projects = session_manager.scan_projects()?;
                if let Some(project_index) = ui.show_projects(&projects)? {
                    let project = &projects[project_index];
                    let mut ui_clone = ui.clone();
                    match ui_clone.show_sessions(project) {
                        Ok(selected_indices) => {
                            if !selected_indices.is_empty() {
                                let selected_sessions: Vec<_> = selected_indices
                                    .iter()
                                    .map(|&i| project.sessions[i].clone())
                                    .collect();
                                match session_manager.delete_sessions(&selected_sessions) {
                                    Ok(deleted_count) => {
                                        ui.show_deletion_result(deleted_count, "sessions");
                                    }
                                    Err(e) => {
                                        ui.show_error(&format!("Failed to delete sessions: {}", e));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            ui.show_error(&format!("An error occurred: {}", e));
                        }
                    }
                }
            }
            MenuChoice::DeleteByAge => {
                let days = ui.prompt_age_days()?;
                let projects = session_manager.scan_projects()?;
                let mut all_old_sessions = Vec::new();
                let mut session_paths = Vec::new();

                for project in &projects {
                    let old_sessions = session_manager.filter_by_age(&project.sessions, days);
                    for session in old_sessions {
                        session_paths.push(session.clone());
                        all_old_sessions.push(session);
                    }
                }

                if all_old_sessions.is_empty() {
                    ui.clear_screen()?;
                    println!("No sessions older than {} days found", days);
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    continue;
                }

                match ui.show_old_sessions(&all_old_sessions, days) {
                    Ok(selected_indices) => {
                        if !selected_indices.is_empty() {
                            let selected_sessions: Vec<_> = selected_indices
                                .iter()
                                .map(|&i| session_paths[i].clone())
                                .collect();
                            match session_manager.delete_sessions(&selected_sessions) {
                                Ok(deleted_count) => {
                                    ui.show_deletion_result(deleted_count, "sessions");
                                }
                                Err(e) => {
                                    ui.show_error(&format!("Failed to delete sessions: {}", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        ui.show_error(&format!("An error occurred: {}", e));
                    }
                }
            }
            MenuChoice::DeleteProject => {
                let projects = session_manager.scan_projects()?;
                if let Some(project_index) = ui.show_projects(&projects)? {
                    let project = &projects[project_index];
                    if ui.confirm_project_deletion(&project.name) {
                        match session_manager.delete_project(project) {
                            Ok(_) => {
                                ui.show_deletion_result(1, "project");
                            }
                            Err(e) => {
                                ui.show_error(&format!("Failed to delete project: {}", e));
                            }
                        }
                    } else {
                        println!("Project deletion cancelled");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                }
            }
            MenuChoice::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

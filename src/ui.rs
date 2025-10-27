use crate::models::{MenuChoice, Project, Session, Statistics};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use std::vec::Vec;

#[derive(Clone)]
pub struct UI {
    selected_sessions: Vec<bool>,
}

impl UI {
    pub fn new() -> Self {
        Self {
            selected_sessions: Vec::new(),
        }
    }

    pub fn clear_screen(&self) -> Result<()> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        print!("\x1B[1;1H");
        io::stdout().flush()?;
        Ok(())
    }

    pub fn show_main_menu(&self) -> Result<MenuChoice> {
        loop {
            self.clear_screen()?;
            println!("Claude Code Session Manager");
            println!("===========================");
            println!("[1] 📊 Show Statistics");
            println!("[2] 🗂️  Manage by Project");
            println!("[3] 📅 Delete by Age");
            println!("[4] 🗑️  Delete Project");
            println!("[5] ❌ Exit");
            println!();
            print!("Select menu (1-5): ");
            io::stdout().flush()?;

            let input = self.read_single_char()?;

            match input {
                '1' => return Ok(MenuChoice::Statistics),
                '2' => return Ok(MenuChoice::ManageProjects),
                '3' => return Ok(MenuChoice::DeleteByAge),
                '4' => return Ok(MenuChoice::DeleteProject),
                '5' => return Ok(MenuChoice::Exit),
                _ => {
                    println!("Invalid choice, please select 1-5");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }

    pub fn show_projects(&self, projects: &[Project]) -> Result<Option<usize>> {
        loop {
            self.clear_screen()?;
            println!("Projects ({} total)", projects.len());
            println!("==================");

            if projects.is_empty() {
                println!("No projects found");
                println!();
                println!("[0] Back");
                print!("Select: ");
                io::stdout().flush()?;

                let input = self.read_single_char()?;

                if input == '0' {
                    return Ok(None);
                }
                continue;
            }

            for (i, project) in projects.iter().enumerate() {
                println!(
                    "[{}] {} ({} sessions, {})",
                    i + 1,
                    project.name,
                    project.sessions.len(),
                    project.format_size()
                );
            }
            println!();
            println!("[0] Back");
            print!("Select project: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "0" {
                return Ok(None);
            }

            if let Ok(index) = input.parse::<usize>() {
                if index > 0 && index <= projects.len() {
                    return Ok(Some(index - 1));
                }
            }

            println!("Invalid choice. Please select 0-{}", projects.len());
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    pub fn show_sessions(&mut self, project: &Project) -> Result<Vec<usize>> {
        // Initialize selection vector
        self.selected_sessions = vec![false; project.sessions.len()];

        loop {
            self.clear_screen()?;
            println!(
                "Sessions in '{}' ({} total, {})",
                project.name,
                project.sessions.len(),
                project.format_size()
            );
            println!("=========================================");

            for (i, session) in project.sessions.iter().enumerate() {
                let selected = if self.selected_sessions[i] { "[x]" } else { "[ ]" };
                let age = session.get_age_days();
                let _modified = session.get_modified_datetime()
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                
                println!(
                    "{} {} ({}, {} วันที่แล้ว)",
                    selected,
                    session.name,
                    session.format_size(),
                    age
                );
            }
            println!();
            println!("[a] Select All");
            println!("[d] Deselect All");
            println!("[x] Delete Selected");
            println!("[0] Back");
            print!("Select: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "0" => return Ok(Vec::new()),
                "a" => {
                    for selected in &mut self.selected_sessions {
                        *selected = true;
                    }
                }
                "d" => {
                    for selected in &mut self.selected_sessions {
                        *selected = false;
                    }
                }
                "x" => {
                    let selected_indices: Vec<usize> = self
                        .selected_sessions
                        .iter()
                        .enumerate()
                        .filter(|(_, &selected)| selected)
                        .map(|(i, _)| i)
                        .collect();

                    if selected_indices.is_empty() {
                        println!("No sessions selected");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        continue;
                    }

                    if self.confirm_deletion(selected_indices.len()) {
                        return Ok(selected_indices);
                    } else {
                        println!("ยกเลิกการลบ");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                }
                _ => {
                    if let Ok(index) = input.parse::<usize>() {
                        if index > 0 && index <= self.selected_sessions.len() {
                            self.selected_sessions[index - 1] = !self.selected_sessions[index - 1];
                        } else {
                            println!("Invalid session number. Please select 1-{}", self.selected_sessions.len());
                            std::thread::sleep(std::time::Duration::from_millis(1000));
                        }
                    } else {
                        println!("Invalid input. Please enter a number, 'a', 'd', 'x', or '0'");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                }
            }
        }
    }

    pub fn show_statistics(&self, stats: &Statistics) {
        self.clear_screen().unwrap();
        println!("📊 Statistics");
        println!("========");
        println!("Total projects: {}", stats.total_projects);
        println!("Total sessions: {}", stats.total_sessions);
        println!("Total sessions size: {}", stats.format_total_size());
        println!();
        println!("Press Enter to go back...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    pub fn confirm_deletion(&self, count: usize) -> bool {
        println!();
        println!("⚠️  Warning: Will delete {} items", count);
        print!("Confirm deletion? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        input == "y" || input == "yes"
    }

    pub fn confirm_project_deletion(&self, project_name: &str) -> bool {
        println!();
        println!("⚠️  Warning: Will delete project '{}' and all its sessions", project_name);
        println!("This action cannot be undone!");
        println!();
        print!("confirm delete? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        input == "y" || input == "yes"
    }

    pub fn prompt_age_days(&self) -> Result<u64> {
        loop {
            self.clear_screen()?;
            println!("📅 Delete by Age");
            println!("===============");
            print!("Delete sessions older than how many days: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim().parse::<u64>() {
                Ok(days) => return Ok(days),
                Err(_) => {
                    println!("Please enter a valid number");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }

    pub fn show_old_sessions(&self, sessions: &[&Session], days: u64) -> Result<Vec<usize>> {
        loop {
            self.clear_screen()?;
            println!(
                "Sessions older than {} days ({} sessions)",
                days,
                sessions.len()
            );
            println!("===============================");

            for (i, session) in sessions.iter().enumerate() {
                let age = session.get_age_days();
                let _modified = session.get_modified_datetime()
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                
                println!(
                    "[{}] {} ({}, {} วันที่แล้ว)",
                    i + 1,
                    session.name,
                    session.format_size(),
                    age
                );
            }
            println!();
            println!("[a] Delete All");
            println!("[0] Back");
            print!("Select: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "0" {
                return Ok(Vec::new());
            }

            if input == "a" {
                if self.confirm_deletion(sessions.len()) {
                    let indices: Vec<usize> = (0..sessions.len()).collect();
                    return Ok(indices);
                } else {
                    println!("ยกเลิกการลบ");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
                continue;
            }

            if let Ok(index) = input.parse::<usize>() {
                if index > 0 && index <= sessions.len() {
                    if self.confirm_deletion(1) {
                        return Ok(vec![index - 1]);
                    } else {
                        println!("Deletion cancelled");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                } else {
                    println!("Invalid choice. Please select 0-{}, 'a', or '0'", sessions.len());
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            } else {
                println!("Invalid input. Please enter a number, 'a', or '0'");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    }

    pub fn show_deletion_result(&self, deleted_count: usize, item_type: &str) {
        println!("✅ Successfully deleted {} {}", deleted_count, item_type);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }

    pub fn show_error(&self, error: &str) {
        println!("❌ Error: {}", error);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }

    fn read_single_char(&self) -> Result<char> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.chars().next().unwrap_or('\r'))
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}
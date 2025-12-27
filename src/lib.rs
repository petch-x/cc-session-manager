#[cfg(feature = "cli")]
pub mod ui;

pub mod models;
pub mod session_manager;
pub mod utils;

#[cfg(feature = "gui")]
pub mod commands;

pub use models::{MenuChoice, Project, Session, Statistics};
pub use session_manager::SessionManager;

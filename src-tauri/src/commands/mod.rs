//! Tauri commands

use tauri::State;

pub mod project;
pub mod ai;
pub mod cli;
pub mod system;

/// App state
pub struct AppState {
    pub db: std::sync::Mutex<rusqlite::Connection>,
}

// Re-export commands
pub use project::*;
pub use ai::*;
pub use cli::*;
pub use system::*;

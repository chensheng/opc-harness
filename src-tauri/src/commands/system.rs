//! System commands

use crate::models::ToolInfo;
use crate::services::Services;
use crate::db::check_health;
use tauri::State;
use serde::Serialize;

/// Greet command (example)
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Get app version
#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Detect all tools
#[tauri::command]
pub fn detect_tools(services: State<'_, Services>) -> Vec<ToolInfo> {
    services.tool_detection.detect_all()
}

/// Get tool status
#[tauri::command]
pub fn get_tool_status(services: State<'_, Services>, tool: String) -> bool {
    services.tool_detection.is_installed(&tool)
}

/// Open directory in VS Code
#[tauri::command]
pub async fn open_in_vscode(path: String) -> Result<(), String> {
    std::process::Command::new("code")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    Ok(())
}

/// Select directory
#[tauri::command]
pub async fn select_directory(
    window: tauri::Window,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let result = window
        .dialog()
        .file()
        .set_parent(&window)
        .blocking_pick_folder();

    Ok(result.map(|p| p.to_string()))
}

/// Database health info (for frontend)
#[derive(Debug, Clone, Serialize)]
pub struct DatabaseHealthInfo {
    pub sqlite_version: String,
    pub page_count: i64,
    pub page_size: i64,
    pub freelist_count: i64,
    pub database_size_bytes: i64,
    pub journal_mode: String,
    pub foreign_keys_enabled: bool,
}

/// Check database health
#[tauri::command]
pub fn check_db_health(services: State<'_, Services>) -> Result<DatabaseHealthInfo, String> {
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    let health = check_health(&db).map_err(|e| format!("Failed to check database health: {}", e))?;
    
    Ok(DatabaseHealthInfo {
        sqlite_version: health.sqlite_version,
        page_count: health.page_count,
        page_size: health.page_size,
        freelist_count: health.freelist_count,
        database_size_bytes: health.database_size_bytes,
        journal_mode: health.journal_mode,
        foreign_keys_enabled: health.foreign_keys_enabled,
    })
}

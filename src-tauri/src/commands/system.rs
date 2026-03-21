//! System commands

use crate::models::ToolInfo;
use crate::services::Services;
use tauri::State;

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

//! CLI commands

use crate::models::{CLISession, CLIOutput};
use crate::services::Services;
use tauri::State;

/// Start CLI session
#[tauri::command]
pub async fn start_cli_session(
    services: State<'_, Services>,
    tool: String,
    project_path: String,
) -> Result<CLISession, String> {
    let cli = services.cli.lock().await;
    cli.start_session(tool, project_path)
        .await
        .map_err(|e| e.to_string())
}

/// Send command to CLI session
#[tauri::command]
pub async fn send_cli_command(
    services: State<'_, Services>,
    session_id: String,
    command: String,
) -> Result<(), String> {
    let cli = services.cli.lock().await;
    cli.send_command(session_id, command)
        .await
        .map_err(|e| e.to_string())
}

/// Kill CLI session
#[tauri::command]
pub async fn kill_cli_session(
    services: State<'_, Services>,
    session_id: String,
) -> Result<(), String> {
    let cli = services.cli.lock().await;
    cli.kill_session(session_id).await.map_err(|e| e.to_string())
}

/// Get CLI output (placeholder for streaming)
#[tauri::command]
pub fn get_cli_output(_session_id: String) -> Result<Vec<CLIOutput>, String> {
    // TODO: Implement output streaming
    Ok(vec![])
}

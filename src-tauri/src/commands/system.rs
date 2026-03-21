//! System commands

use crate::models::ToolInfo;
use crate::services::Services;
use crate::db::{check_health, migrations};
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

/// Tool info with category (for frontend)
#[derive(Debug, Clone, Serialize)]
pub struct ToolInfoWithCategory {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub category: String,
    pub is_required: bool,
    pub description: String,
}

/// Detect all tools with categories
#[tauri::command]
pub fn detect_tools_detailed(services: State<'_, Services>) -> Vec<ToolInfoWithCategory> {
    services
        .tool_detection
        .detect_all_detailed()
        .into_iter()
        .map(|r| ToolInfoWithCategory {
            name: r.tool.name,
            installed: r.tool.installed,
            version: r.tool.version,
            path: r.tool.path,
            category: format!("{:?}", r.category),
            is_required: r.is_required,
            description: r.description,
        })
        .collect()
}

/// Detect required tools
#[tauri::command]
pub fn detect_required_tools(services: State<'_, Services>) -> Vec<ToolInfoWithCategory> {
    services
        .tool_detection
        .detect_required_tools()
        .into_iter()
        .map(|r| ToolInfoWithCategory {
            name: r.tool.name,
            installed: r.tool.installed,
            version: r.tool.version,
            path: r.tool.path,
            category: format!("{:?}", r.category),
            is_required: r.is_required,
            description: r.description,
        })
        .collect()
}

/// Get missing required tools
#[tauri::command]
pub fn get_missing_required_tools(services: State<'_, Services>) -> Vec<String> {
    services.tool_detection.detect_missing_required()
}

/// Check if all required tools are installed
#[tauri::command]
pub fn all_required_tools_installed(services: State<'_, Services>) -> bool {
    services.tool_detection.all_required_installed()
}

/// Get specific tool info
#[tauri::command]
pub fn get_tool_info(services: State<'_, Services>, tool: String) -> Option<ToolInfo> {
    services.tool_detection.get_tool_info(&tool)
}

/// Detect AI CLI tools (Kimi, Claude, Codex)
#[tauri::command]
pub fn detect_ai_cli_tools(services: State<'_, Services>) -> Vec<ToolInfo> {
    vec![
        services.tool_detection.get_tool_info("kimi").unwrap_or_else(|| ToolInfo {
            name: "Kimi CLI".to_string(),
            installed: false,
            version: None,
            path: None,
        }),
        services.tool_detection.get_tool_info("claude").unwrap_or_else(|| ToolInfo {
            name: "Claude Code".to_string(),
            installed: false,
            version: None,
            path: None,
        }),
        services.tool_detection.get_tool_info("codex").unwrap_or_else(|| ToolInfo {
            name: "Codex CLI".to_string(),
            installed: false,
            version: None,
            path: None,
        }),
    ]
}

/// Check if specific AI CLI tool is installed
#[tauri::command]
pub fn is_ai_cli_tool_installed(services: State<'_, Services>, tool: String) -> bool {
    match tool.as_str() {
        "kimi" | "claude" | "codex" => services.tool_detection.is_installed(&tool),
        _ => false,
    }
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

/// Database verification info (for frontend)
#[derive(Debug, Clone, Serialize)]
pub struct DatabaseVerificationInfo {
    pub is_valid: bool,
    pub integrity_check: String,
    pub tables: Vec<String>,
    pub indexes: Vec<String>,
    pub foreign_keys_enabled: bool,
}

/// Verify database integrity
#[tauri::command]
pub fn verify_database(services: State<'_, Services>) -> Result<DatabaseVerificationInfo, String> {
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    let verification = migrations::verify_database(&db)
        .map_err(|e| format!("Failed to verify database: {}", e))?;
    
    Ok(DatabaseVerificationInfo {
        is_valid: verification.is_valid,
        integrity_check: verification.integrity_check,
        tables: verification.tables,
        indexes: verification.indexes,
        foreign_keys_enabled: verification.foreign_keys_enabled,
    })
}

/// Database statistics (for frontend)
#[derive(Debug, Clone, Serialize)]
pub struct DatabaseStats {
    pub project_count: i64,
    pub prd_count: i64,
    pub persona_count: i64,
    pub competitor_count: i64,
    pub cli_session_count: i64,
    pub enabled_ai_config_count: i64,
    pub migration_version: i32,
}

/// Get database statistics
#[tauri::command]
pub fn get_db_stats(services: State<'_, Services>) -> Result<DatabaseStats, String> {
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    let stats = migrations::get_statistics(&db)
        .map_err(|e| format!("Failed to get database statistics: {}", e))?;
    
    Ok(DatabaseStats {
        project_count: stats.project_count,
        prd_count: stats.prd_count,
        persona_count: stats.persona_count,
        competitor_count: stats.competitor_count,
        cli_session_count: stats.cli_session_count,
        enabled_ai_config_count: stats.enabled_ai_config_count,
        migration_version: stats.migration_version,
    })
}

/// Reset database (clear all data, keep schema)
#[tauri::command]
pub fn reset_database(services: State<'_, Services>) -> Result<(), String> {
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    migrations::reset_database(&db)
        .map_err(|e| format!("Failed to reset database: {}", e))?;
    
    Ok(())
}

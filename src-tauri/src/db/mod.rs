use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::Manager;

pub async fn init_database(app_handle: &tauri::AppHandle) -> Result<()> {
    let app_dir = app_handle.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Ensure the directory exists
    if let Err(e) = std::fs::create_dir_all(&app_dir) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some(format!("Failed to create app data directory: {}", e))
        ));
    }
    
    let db_path = app_dir.join("opc-harness.db");
    
    let conn = Connection::open(db_path)?;
    
    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT DEFAULT 'idea',
            progress INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            idea TEXT,
            prd TEXT,
            user_personas TEXT,
            competitor_analysis TEXT
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_configs (
            provider TEXT PRIMARY KEY,
            model TEXT NOT NULL,
            api_key TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cli_sessions (
            id TEXT PRIMARY KEY,
            tool_type TEXT NOT NULL,
            project_path TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(())
}

//! Database module for SQLite operations

use rusqlite::Connection;
use std::path::PathBuf;

pub mod migrations;

/// Initialize database connection
pub fn init_db(app_dir: PathBuf) -> Result<Connection, rusqlite::Error> {
    let db_path = app_dir.join("opc-harness.db");
    let conn = Connection::open(db_path)?;
    
    // Run migrations
    migrations::run_migrations(&conn)?;
    
    Ok(conn)
}

/// Get database connection
pub fn get_connection(app_dir: PathBuf) -> Result<Connection, rusqlite::Error> {
    let db_path = app_dir.join("opc-harness.db");
    Connection::open(db_path)
}

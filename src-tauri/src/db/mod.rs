//! Database module for SQLite operations
//!
//! 提供SQLite数据库连接、初始化和迁移管理

use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub mod migrations;

/// 数据库连接池类型
pub type DbPool = Arc<Mutex<Connection>>;

/// Initialize database connection and run migrations
/// 
/// # Arguments
/// * `app_dir` - Application data directory path
/// 
/// # Returns
/// * `Result<Connection, rusqlite::Error>` - Database connection or error
pub fn init_db(app_dir: PathBuf) -> SqliteResult<Connection> {
    let db_path = app_dir.join("opc-harness.db");
    
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some(format!("Failed to create database directory: {}", e)),
            )
        })?;
    }
    
    let conn = Connection::open(&db_path)?;
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // Enable WAL mode for better concurrency
    // Note: PRAGMA journal_mode returns a result, so we use query_row instead of execute
    conn.query_row("PRAGMA journal_mode = WAL", [], |_| Ok(()))?;
    
    // Initialize database (migrations + default data)
    migrations::initialize_database(&conn)?;
    
    log::info!("Database initialized at: {:?}", db_path);
    
    Ok(conn)
}

/// Get a new database connection for the given app directory
/// 
/// # Arguments
/// * `app_dir` - Application data directory path
/// 
/// # Returns
/// * `Result<Connection, rusqlite::Error>` - Database connection or error
pub fn get_connection(app_dir: PathBuf) -> SqliteResult<Connection> {
    let db_path = app_dir.join("opc-harness.db");
    let conn = Connection::open(db_path)?;
    
    // Enable foreign keys for this connection
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    Ok(conn)
}

/// Create a database connection pool
/// 
/// # Arguments
/// * `conn` - Database connection to wrap in pool
/// 
/// # Returns
/// * `DbPool` - Thread-safe connection pool
pub fn create_pool(conn: Connection) -> DbPool {
    Arc::new(Mutex::new(conn))
}

/// Execute a transaction with automatic rollback on error
/// 
/// # Arguments
/// * `conn` - Database connection
/// * `f` - Closure containing transaction operations
/// 
/// # Returns
/// * `Result<T, rusqlite::Error>` - Result of the transaction
pub fn with_transaction<T, F>(conn: &mut Connection, f: F) -> SqliteResult<T>
where
    F: FnOnce(&Transaction) -> SqliteResult<T>,
{
    let tx = conn.transaction()?;
    match f(&tx) {
        Ok(result) => {
            tx.commit()?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback()?;
            Err(e)
        }
    }
}

/// Transaction helper type
pub type Transaction<'a> = rusqlite::Transaction<'a>;

/// Check database health and return status info
/// 
/// # Arguments
/// * `conn` - Database connection
/// 
/// # Returns
/// * `Result<DbHealthInfo, rusqlite::Error>` - Database health information
pub fn check_health(conn: &Connection) -> SqliteResult<DbHealthInfo> {
    let version: String = conn.query_row(
        "SELECT sqlite_version()",
        [],
        |row| row.get(0),
    )?;
    
    let page_count: i64 = conn.query_row(
        "PRAGMA page_count",
        [],
        |row| row.get(0),
    )?;
    
    let page_size: i64 = conn.query_row(
        "PRAGMA page_size",
        [],
        |row| row.get(0),
    )?;
    
    let freelist_count: i64 = conn.query_row(
        "PRAGMA freelist_count",
        [],
        |row| row.get(0),
    )?;
    
    let journal_mode: String = conn.query_row(
        "PRAGMA journal_mode",
        [],
        |row| row.get(0),
    )?;
    
    let foreign_keys: bool = conn.query_row(
        "PRAGMA foreign_keys",
        [],
        |row| row.get::<_, i32>(0).map(|v| v == 1),
    )?;
    
    Ok(DbHealthInfo {
        sqlite_version: version,
        page_count,
        page_size,
        freelist_count,
        database_size_bytes: page_count * page_size,
        journal_mode,
        foreign_keys_enabled: foreign_keys,
    })
}

/// Database health information
#[derive(Debug, Clone)]
pub struct DbHealthInfo {
    pub sqlite_version: String,
    pub page_count: i64,
    pub page_size: i64,
    pub freelist_count: i64,
    pub database_size_bytes: i64,
    pub journal_mode: String,
    pub foreign_keys_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init_db() {
        let temp_dir = std::env::temp_dir().join("opc-harness-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        
        let conn = init_db(temp_dir.clone()).expect("Failed to init db");
        drop(conn);
        
        // Verify database file exists
        assert!(temp_dir.join("opc-harness.db").exists());
        
        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
    
    #[test]
    fn test_crud_operations() {
        let temp_dir = std::env::temp_dir().join("opc-harness-test-crud");
        let _ = std::fs::remove_dir_all(&temp_dir);
        
        let conn = init_db(temp_dir.clone()).expect("Failed to init db");
        
        // Test INSERT
        conn.execute(
            "INSERT INTO projects (id, name, description, status, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["test-1", "Test Project", "Test Description", "draft", 1234567890i64, 1234567890i64],
        ).expect("Failed to insert");
        
        // Test SELECT
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE id = ?1",
            ["test-1"],
            |row| row.get(0),
        ).expect("Failed to query");
        assert_eq!(count, 1);
        
        // Test UPDATE
        conn.execute(
            "UPDATE projects SET name = ?1 WHERE id = ?2",
            rusqlite::params!["Updated Project", "test-1"],
        ).expect("Failed to update");
        
        // Test DELETE
        conn.execute(
            "DELETE FROM projects WHERE id = ?1",
            ["test-1"],
        ).expect("Failed to delete");
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE id = ?1",
            ["test-1"],
            |row| row.get(0),
        ).expect("Failed to query");
        assert_eq!(count, 0);
        
        // Cleanup
        drop(conn);
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

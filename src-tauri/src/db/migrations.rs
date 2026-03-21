//! Database migrations

use rusqlite::Connection;

const MIGRATIONS: &[&str] = &[
    // Migration 1: Initial schema
    r#"
    CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        status TEXT NOT NULL DEFAULT 'draft',
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        path TEXT
    );

    CREATE TABLE IF NOT EXISTS ai_configs (
        provider TEXT PRIMARY KEY,
        api_key TEXT,
        base_url TEXT,
        model TEXT NOT NULL,
        enabled INTEGER NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );
    "#,
];

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Create migrations table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS __migrations (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

    let applied_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM __migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    for (i, migration) in MIGRATIONS.iter().enumerate() {
        let version = (i + 1) as i32;
        if version > applied_version {
            conn.execute_batch(migration)?;
            conn.execute(
                "INSERT INTO __migrations (version, applied_at) VALUES (?1, ?2)",
                [version, chrono::Utc::now().timestamp() as i32],
            )?;
        }
    }

    Ok(())
}

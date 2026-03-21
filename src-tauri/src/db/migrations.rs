//! Database migrations

use rusqlite::Connection;

const MIGRATIONS: &[&str] = &[
    // Migration 1: Initial schema
    r#"
    -- Projects table
    CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        status TEXT NOT NULL DEFAULT 'draft',
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        path TEXT
    );

    -- AI configs table
    CREATE TABLE IF NOT EXISTS ai_configs (
        provider TEXT PRIMARY KEY,
        api_key TEXT,
        base_url TEXT,
        model TEXT NOT NULL,
        enabled INTEGER NOT NULL DEFAULT 0
    );

    -- Settings table
    CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL,
        updated_at INTEGER NOT NULL DEFAULT 0
    );

    -- PRDs table
    CREATE TABLE IF NOT EXISTS prds (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        content TEXT NOT NULL,
        version INTEGER DEFAULT 1,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    );

    -- User personas table
    CREATE TABLE IF NOT EXISTS user_personas (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        name TEXT NOT NULL,
        age_range TEXT,
        occupation TEXT,
        goals TEXT, -- JSON array
        pain_points TEXT, -- JSON array
        behaviors TEXT, -- JSON array
        created_at INTEGER NOT NULL
    );

    -- Competitors table
    CREATE TABLE IF NOT EXISTS competitors (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        name TEXT NOT NULL,
        url TEXT,
        strengths TEXT, -- JSON array
        weaknesses TEXT, -- JSON array
        differentiation TEXT,
        created_at INTEGER NOT NULL
    );

    -- CLI sessions table
    CREATE TABLE IF NOT EXISTS cli_sessions (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        tool_type TEXT NOT NULL,
        working_directory TEXT NOT NULL,
        status TEXT NOT NULL,
        start_time INTEGER NOT NULL,
        end_time INTEGER
    );

    -- Create indexes for better query performance
    CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
    CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at DESC);
    CREATE INDEX IF NOT EXISTS idx_prds_project_id ON prds(project_id);
    CREATE INDEX IF NOT EXISTS idx_personas_project_id ON user_personas(project_id);
    CREATE INDEX IF NOT EXISTS idx_competitors_project_id ON competitors(project_id);
    CREATE INDEX IF NOT EXISTS idx_cli_sessions_project_id ON cli_sessions(project_id);
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

use rusqlite::{Connection, Result};
use crate::utils::paths;

/// 获取数据库连接
pub fn get_connection(_app_handle: &tauri::AppHandle) -> Result<Connection> {
    let db_path = paths::get_database_path();
    Connection::open(&db_path)
}

/// 确保所有项目工作区目录存在（占位函数，实际逻辑在 utils/paths.rs 中）
pub fn ensure_all_project_workspaces(_app_handle: &tauri::AppHandle) -> Result<()> {
    // 此函数已废弃，保留仅用于向后兼容
    Ok(())
}

/// 初始化数据库连接和表结构
pub async fn init_database(app_handle: &tauri::AppHandle) -> Result<()> {
    // 确保应用目录结构存在
    paths::ensure_app_directories()
        .map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some(format!("Failed to create app directories: {}", e)),
            )
        })?;

    // 尝试从旧位置迁移数据
    match paths::migrate_legacy_data(app_handle) {
        Ok(true) => log::info!("Legacy data migrated successfully"),
        Ok(false) => log::info!("No migration needed"),
        Err(e) => log::warn!("Migration failed: {}, will use new location", e),
    }

    let db_path = paths::get_database_path();
    
    log::info!("Initializing database at: {:?}", db_path);

    let conn = Connection::open(&db_path)?;

    // Create projects table
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

    // Create ai_configs table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_configs (
            provider TEXT PRIMARY KEY,
            model TEXT NOT NULL
        )",
        [],
    )?;

    // Create cli_sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cli_sessions (
            id TEXT PRIMARY KEY,
            tool_type TEXT NOT NULL,
            project_path TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create agent_sessions table (VC-005)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_sessions (
            session_id TEXT NOT NULL,
            agent_id TEXT PRIMARY KEY,
            agent_type TEXT NOT NULL,
            project_path TEXT NOT NULL,
            status TEXT NOT NULL,
            phase TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            stdio_channel_id TEXT,
            registered_to_daemon INTEGER NOT NULL DEFAULT 0,
            metadata TEXT
        )",
        [],
    )?;

    // Create milestones table (DB-002)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS milestones (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            order_index INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            due_date TEXT,
            completed_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for milestones table
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_milestones_project_id ON milestones(project_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_milestones_status ON milestones(status)",
        [],
    )?;

    // Create issues table (DB-003)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS issues (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            milestone_id TEXT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            issue_type TEXT NOT NULL DEFAULT 'task',
            priority TEXT NOT NULL DEFAULT 'medium',
            status TEXT NOT NULL DEFAULT 'open',
            assignee TEXT,
            parent_issue_id TEXT,
            order_index INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (milestone_id) REFERENCES milestones(id) ON DELETE SET NULL,
            FOREIGN KEY (parent_issue_id) REFERENCES issues(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // Create indexes for issues table
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_issues_project_id ON issues(project_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_issues_milestone_id ON issues(milestone_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_issues_priority ON issues(priority)",
        [],
    )?;

    // Create user_stories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_stories (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            story_number TEXT NOT NULL,
            title TEXT NOT NULL,
            role TEXT NOT NULL,
            feature TEXT NOT NULL,
            benefit TEXT NOT NULL,
            description TEXT NOT NULL,
            acceptance_criteria TEXT NOT NULL,
            priority TEXT NOT NULL DEFAULT 'P2',
            story_points INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'draft',
            epic TEXT,
            labels TEXT,
            dependencies TEXT,
            sprint_id TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for user_stories table
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_project_id ON user_stories(project_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_status ON user_stories(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_priority ON user_stories(priority)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_story_number ON user_stories(story_number)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_sprint_id ON user_stories(sprint_id)",
        [],
    )?;

    // Create sprints table (story_ids 已移除，通过 user_stories.sprint_id 关联)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sprints (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            goal TEXT NOT NULL DEFAULT '',
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'planning',
            total_story_points INTEGER NOT NULL DEFAULT 0,
            completed_story_points INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for sprints table
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sprints_project_id ON sprints(project_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sprints_status ON sprints(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sprints_start_date ON sprints(start_date)",
        [],
    )?;

    // 运行迁移脚本以添加 sprint_id 支持（如果列不存在）
    migrate_add_sprint_id_support_internal(&conn).map_err(|e| {
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some(format!("Migration failed: {}", e)),
        )
    })?;

    // 运行迁移脚本以移除 sprints 表的 story_ids 字段
    migrate_remove_story_ids_from_sprints_internal(&conn).map_err(|e| {
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some(format!("Migration failed: {}", e)),
        )
    })?;

    Ok(())
}

/// 内部迁移函数：为 user_stories 表添加 sprint_id 字段支持
fn migrate_add_sprint_id_support_internal(conn: &Connection) -> Result<(), String> {
    println!("[DB Migration] Checking sprint_id column in user_stories table...");
    
    let column_exists = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('user_stories') WHERE name='sprint_id'",
        [],
        |row| row.get::<_, i64>(0)
    ).map_err(|e| format!("Failed to check column existence: {}", e))?;
    
    if column_exists == 0 {
        println!("[DB Migration] Adding sprint_id column to user_stories table");
        
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN sprint_id TEXT",
            []
        ).map_err(|e| format!("Failed to add sprint_id column: {}", e))?;
        
        println!("[DB Migration] ✓ sprint_id column added successfully");
    } else {
        println!("[DB Migration] ✓ sprint_id column already exists, skipping");
    }
    
    let index_exists = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='idx_user_stories_sprint_id'",
        [],
        |row| row.get::<_, i64>(0)
    ).map_err(|e| format!("Failed to check index existence: {}", e))?;
    
    if index_exists == 0 {
        println!("[DB Migration] Creating index on user_stories.sprint_id");
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_user_stories_sprint_id ON user_stories(sprint_id)",
            []
        ).map_err(|e| format!("Failed to create index: {}", e))?;
        
        println!("[DB Migration] ✓ Index created successfully");
    } else {
        println!("[DB Migration] ✓ Index already exists, skipping");
    }
    
    println!("[DB Migration] Sprint ID support migration completed");
    Ok(())
}

/// 内部迁移函数：从 sprints 表移除 story_ids 字段
fn migrate_remove_story_ids_from_sprints_internal(conn: &Connection) -> Result<(), String> {
    println!("[DB Migration] Checking story_ids column in sprints table...");
    
    let column_exists = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('sprints') WHERE name='story_ids'",
        [],
        |row| row.get::<_, i64>(0)
    ).map_err(|e| format!("Failed to check column existence: {}", e))?;
    
    if column_exists > 0 {
        println!("[DB Migration] Removing story_ids column from sprints table");
        
        // SQLite 不支持直接删除列，需要重建表
        conn.execute(
            "CREATE TABLE sprints_new (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                goal TEXT NOT NULL DEFAULT '',
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'planning',
                total_story_points INTEGER NOT NULL DEFAULT 0,
                completed_story_points INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            []
        ).map_err(|e| format!("Failed to create new sprints table: {}", e))?;
        
        conn.execute(
            "INSERT INTO sprints_new (id, project_id, name, goal, start_date, end_date, status, total_story_points, completed_story_points, created_at, updated_at)
             SELECT id, project_id, name, goal, start_date, end_date, status, total_story_points, completed_story_points, created_at, updated_at FROM sprints",
            []
        ).map_err(|e| format!("Failed to copy data: {}", e))?;
        
        conn.execute("DROP TABLE sprints", [])
            .map_err(|e| format!("Failed to drop old table: {}", e))?;
        
        conn.execute("ALTER TABLE sprints_new RENAME TO sprints", [])
            .map_err(|e| format!("Failed to rename table: {}", e))?;
        
        println!("[DB Migration] ✓ story_ids column removed successfully");
    } else {
        println!("[DB Migration] ✓ story_ids column does not exist, skipping");
    }
    
    println!("[DB Migration] Remove story_ids from sprints migration completed");
    Ok(())
}

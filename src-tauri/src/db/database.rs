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

    // Create agent_sessions table (VC-005) - 移除外键约束以支持灵活的项目关联
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_sessions_new (
            session_id TEXT NOT NULL,
            agent_id TEXT PRIMARY KEY,
            agent_type TEXT NOT NULL,
            project_id TEXT NOT NULL,
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

    // 如果旧表存在，迁移数据并替换
    let table_exists = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='agent_sessions'",
        [],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(0);

    if table_exists > 0 {
        log::info!("Migrating agent_sessions table to remove foreign key constraint...");
        
        // 迁移数据
        conn.execute(
            "INSERT OR REPLACE INTO agent_sessions_new SELECT * FROM agent_sessions",
            []
        )?;
        
        // 删除旧表
        conn.execute("DROP TABLE agent_sessions", [])?;
        
        // 重命名新表
        conn.execute("ALTER TABLE agent_sessions_new RENAME TO agent_sessions", [])?;
        
        log::info!("agent_sessions table migration completed");
    } else {
        // 如果旧表不存在，直接重命名新表
        conn.execute("ALTER TABLE agent_sessions_new RENAME TO agent_sessions", [])?;
    }

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

    // 迁移：如果 agent_sessions 表存在 project_path 列，则迁移到 project_id
    migrate_agent_sessions_project_path_to_id(&conn)?;

    Ok(())
}

/// 迁移 agent_sessions 表的 project_path 到 project_id
fn migrate_agent_sessions_project_path_to_id(conn: &Connection) -> Result<()> {
    // 检查是否存在 project_path 列
    let mut stmt = conn.prepare("PRAGMA table_info(agent_sessions)")?;
    let columns: Vec<String> = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(1)?) // name column
    })?.collect::<Result<Vec<_>, _>>()?;
    
    let has_project_path = columns.iter().any(|col| col == "project_path");
    let has_project_id = columns.iter().any(|col| col == "project_id");
    
    if has_project_path && !has_project_id {
        log::info!("Migrating agent_sessions: project_path -> project_id");
        
        // SQLite 不支持直接重命名列或修改列类型
        // 需要创建新表，复制数据，删除旧表，重命名新表
        
        // 1. 创建临时表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agent_sessions_new (
                session_id TEXT NOT NULL,
                agent_id TEXT PRIMARY KEY,
                agent_type TEXT NOT NULL,
                project_id TEXT NOT NULL,
                status TEXT NOT NULL,
                phase TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                stdio_channel_id TEXT,
                registered_to_daemon INTEGER NOT NULL DEFAULT 0,
                metadata TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;
        
        // 2. 复制数据（将 project_path 映射到第一个项目的 ID，或使用空字符串）
        // 注意：这是一个简化的迁移，实际项目中可能需要更复杂的逻辑来映射 path 到 id
        conn.execute(
            "INSERT INTO agent_sessions_new 
             SELECT session_id, agent_id, agent_type, 
                    COALESCE((SELECT id FROM projects LIMIT 1), '') as project_id,
                    status, phase, created_at, updated_at, 
                    stdio_channel_id, registered_to_daemon, metadata
             FROM agent_sessions",
            [],
        )?;
        
        // 3. 删除旧表
        conn.execute("DROP TABLE agent_sessions", [])?;
        
        // 4. 重命名新表
        conn.execute("ALTER TABLE agent_sessions_new RENAME TO agent_sessions", [])?;
        
        log::info!("✅ agent_sessions migration completed: project_path -> project_id");
    } else if !has_project_id {
        log::warn!("agent_sessions table exists but missing both project_path and project_id columns");
    } else {
        log::info!("No migration needed for agent_sessions table");
    }
    
    Ok(())
}

use rusqlite::{Connection, Result};
use crate::utils::paths;

/// 获取数据库连接
pub fn get_connection() -> Result<Connection> {
    let db_path = paths::get_database_path();
    Connection::open(&db_path)
}

/// 确保所有项目工作区目录存在（占位函数，实际逻辑在 utils/paths.rs 中）
pub fn ensure_all_project_workspaces(_app_handle: &tauri::AppHandle) -> Result<()> {
    // 此函数已废弃，保留仅用于向后兼容
    Ok(())
}

/// 迁移：为 projects 表添加 tags 字段（如果不存在）
fn migrate_add_project_tags_field(conn: &Connection) -> Result<()> {
    // 检查表是否存在
    let table_exists = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='projects'",
        [],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(0);

    if table_exists == 0 {
        // 表不存在，无需迁移
        return Ok(());
    }

    // 表存在，检查列结构
    let mut stmt = conn.prepare("PRAGMA table_info(projects)")?;
    let columns: Vec<String> = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(1)?) // name column
    })?.collect::<Result<Vec<_>, _>>()?;

    // 检查是否包含 tags 列
    if !columns.contains(&"tags".to_string()) {
        log::debug!("Adding 'tags' column to projects table...");
        conn.execute(
            "ALTER TABLE projects ADD COLUMN tags TEXT",
            [],
        )?;
        log::info!("✅ projects table updated with 'tags' column");
    } else {
        log::debug!("projects table already has 'tags' column");
    }

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
    // 检查表结构版本，如果不匹配则重建
    check_and_rebuild_agent_sessions_table(&conn)?;

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

    // 迁移：为 user_stories 表添加 Agent Loop 相关字段（如果不存在）
    migrate_add_agent_loop_fields(&conn)?;

    // 为 next_retry_at 字段创建索引，优化重试调度器查询性能（必须在迁移后执行）
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_stories_next_retry_at ON user_stories(next_retry_at)",
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

    Ok(())
}

/// 检查并重建 agent_sessions 表（如果结构不匹配）
fn check_and_rebuild_agent_sessions_table(conn: &Connection) -> Result<()> {
    // 检查表是否存在
    let table_exists = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='agent_sessions'",
        [],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(0);

    if table_exists == 0 {
        // 表不存在，创建新表
        log::info!("Creating agent_sessions table...");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agent_sessions (
                session_id TEXT NOT NULL,
                agent_id TEXT PRIMARY KEY,
                agent_type TEXT NOT NULL,
                project_id TEXT NOT NULL,
                name TEXT,
                status TEXT NOT NULL,
                phase TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                stdio_channel_id TEXT,
                registered_to_daemon INTEGER NOT NULL DEFAULT 0,
                metadata TEXT,
                agents_md_content TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;
        return Ok(());
    }

    // 表存在，检查列结构
    let mut stmt = conn.prepare("PRAGMA table_info(agent_sessions)")?;
    let columns: Vec<String> = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(1)?) // name column
    })?.collect::<Result<Vec<_>, _>>()?;

    // 期望的列列表
    let expected_columns = vec![
        "session_id", "agent_id", "agent_type", "project_id", "name",
        "status", "phase", "created_at", "updated_at", "stdio_channel_id",
        "registered_to_daemon", "metadata", "agents_md_content"
    ];

    // 检查是否包含所有必需的列
    let has_all_columns = expected_columns.iter().all(|col| columns.contains(&col.to_string()));
    
    // 检查是否有旧版本的列（project_path）
    let has_old_column = columns.iter().any(|col| col == "project_path");

    // 初始化可观测性相关表
    init_observability_tables(&conn)?;

    // 初始化重试引擎相关表
    init_retry_engine_tables(&conn)?;

    if !has_all_columns || has_old_column {
        // 表结构不匹配，删除并重建
        log::warn!("agent_sessions table structure mismatch, rebuilding...");
        conn.execute("DROP TABLE IF EXISTS agent_sessions", [])?;
        
        // 重新创建表
        conn.execute(
            "CREATE TABLE agent_sessions (
                session_id TEXT NOT NULL,
                agent_id TEXT PRIMARY KEY,
                agent_type TEXT NOT NULL,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'idle',
                phase TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                stdio_channel_id TEXT,
                registered_to_daemon BOOLEAN NOT NULL DEFAULT FALSE,
                metadata TEXT,
                agents_md_content TEXT
            )",
            [],
        )?;
        log::info!("agent_sessions table rebuilt successfully");
    }

    Ok(())
}

/// 迁移：为 user_stories 表添加 Agent Loop 相关字段
fn migrate_add_agent_loop_fields(conn: &Connection) -> Result<()> {
    use rusqlite::OptionalExtension;
    
    log::info!("Checking user_stories table for Agent Loop fields...");
    
    // 检查 assigned_agent 字段是否存在
    let has_assigned_agent: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='assigned_agent'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_assigned_agent.is_none() {
        log::info!("Adding assigned_agent column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN assigned_agent TEXT",
            [],
        )?;
    }
    
    // 检查 locked_at 字段
    let has_locked_at: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='locked_at'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_locked_at.is_none() {
        log::info!("Adding locked_at column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN locked_at TEXT",
            [],
        )?;
    }
    
    // 检查 started_at 字段
    let has_started_at: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='started_at'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_started_at.is_none() {
        log::info!("Adding started_at column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN started_at TEXT",
            [],
        )?;
    }
    
    // 检查 completed_at 字段
    let has_completed_at: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='completed_at'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_completed_at.is_none() {
        log::info!("Adding completed_at column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN completed_at TEXT",
            [],
        )?;
    }
    
    // 检查 failed_at 字段
    let has_failed_at: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='failed_at'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_failed_at.is_none() {
        log::info!("Adding failed_at column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN failed_at TEXT",
            [],
        )?;
    }
    
    // 检查 error_message 字段
    let has_error_message: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='error_message'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_error_message.is_none() {
        log::info!("Adding error_message column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN error_message TEXT",
            [],
        )?;
    }
    
    // 检查 retry_count 字段
    let has_retry_count: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='retry_count'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_retry_count.is_none() {
        log::info!("Adding retry_count column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN retry_count INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    
    // 检查 next_retry_at 字段（重试引擎新增）
    let has_next_retry_at: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='next_retry_at'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_next_retry_at.is_none() {
        log::info!("Adding next_retry_at column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN next_retry_at TEXT",
            [],
        )?;
    }
    
    // 检查 max_retries 字段（重试引擎新增）
    let has_max_retries: Option<String> = conn.query_row(
        "SELECT name FROM pragma_table_info('user_stories') WHERE name='max_retries'",
        [],
        |row| row.get(0)
    ).optional()?;
    
    if has_max_retries.is_none() {
        log::info!("Adding max_retries column to user_stories table");
        conn.execute(
            "ALTER TABLE user_stories ADD COLUMN max_retries INTEGER NOT NULL DEFAULT 3",
            [],
        )?;
    }
    
    log::info!("Agent Loop fields migration completed");
    Ok(())
}

/// 初始化可观测性相关表（日志、追踪、告警）
fn init_observability_tables(conn: &Connection) -> Result<()> {
    // 创建 agent_logs 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_logs (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            session_id TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            level TEXT NOT NULL,
            source TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (agent_id) REFERENCES agent_sessions(agent_id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建 agent_logs 索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_logs_agent_id ON agent_logs(agent_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_logs_timestamp ON agent_logs(timestamp)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_logs_level ON agent_logs(level)",
        [],
    )?;

    // 创建 agent_traces 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_traces (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            session_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            data TEXT NOT NULL,
            parent_id TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (agent_id) REFERENCES agent_sessions(agent_id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建 agent_traces 索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_traces_agent_id ON agent_traces(agent_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_traces_event_type ON agent_traces(event_type)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_traces_timestamp ON agent_traces(timestamp)",
        [],
    )?;

    // 创建 agent_alerts 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_alerts (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            level TEXT NOT NULL,
            alert_type TEXT NOT NULL,
            message TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            resolved_at TEXT,
            FOREIGN KEY (agent_id) REFERENCES agent_sessions(agent_id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建 agent_alerts 索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_alerts_agent_id ON agent_alerts(agent_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_alerts_status ON agent_alerts(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_agent_alerts_created_at ON agent_alerts(created_at)",
        [],
    )?;

    log::info!("Observability tables initialized successfully");
    Ok(())
}

/// 初始化重试引擎相关表
fn init_retry_engine_tables(conn: &Connection) -> Result<()> {
    // 创建 user_story_retry_history 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_story_retry_history (
            id TEXT PRIMARY KEY,
            user_story_id TEXT NOT NULL,
            retry_number INTEGER NOT NULL,
            triggered_at TEXT NOT NULL,
            error_message TEXT,
            error_type TEXT,
            decision TEXT NOT NULL,
            next_retry_at TEXT,
            completed_at TEXT,
            result TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_story_id) REFERENCES user_stories(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建 user_story_retry_history 索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_retry_history_user_story_id ON user_story_retry_history(user_story_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_retry_history_triggered_at ON user_story_retry_history(triggered_at)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_retry_history_result ON user_story_retry_history(result)",
        [],
    )?;

    log::info!("Retry engine tables initialized successfully");
    Ok(())
}

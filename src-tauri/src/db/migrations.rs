//! Database migrations
//!
//! 管理数据库架构版本和迁移

use rusqlite::Connection;

/// 数据库架构版本
pub const CURRENT_SCHEMA_VERSION: i32 = 1;

/// 所有迁移脚本
const MIGRATIONS: &[&str] = &[
    // Migration 1: Initial schema (v1.0)
    r#"
    -- ============================================
    -- 基础表结构
    -- ============================================

    -- Projects table: 项目信息
    CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        status TEXT NOT NULL DEFAULT 'draft',
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        path TEXT
    );

    -- AI configs table: AI厂商配置
    CREATE TABLE IF NOT EXISTS ai_configs (
        provider TEXT PRIMARY KEY,
        api_key TEXT,
        base_url TEXT,
        model TEXT NOT NULL,
        enabled INTEGER NOT NULL DEFAULT 0
    );

    -- Settings table: 应用设置
    CREATE TABLE IF NOT EXISTS settings (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL,
        updated_at INTEGER NOT NULL DEFAULT 0
    );

    -- PRDs table: 产品需求文档
    CREATE TABLE IF NOT EXISTS prds (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        content TEXT NOT NULL,
        version INTEGER DEFAULT 1,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    );

    -- User personas table: 用户画像
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

    -- Competitors table: 竞品分析
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

    -- CLI sessions table: CLI会话记录
    CREATE TABLE IF NOT EXISTS cli_sessions (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        tool_type TEXT NOT NULL,
        working_directory TEXT NOT NULL,
        status TEXT NOT NULL,
        start_time INTEGER NOT NULL,
        end_time INTEGER
    );

    -- Marketing strategies table: 营销策略 (MVP简化版)
    CREATE TABLE IF NOT EXISTS marketing_strategies (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        content TEXT NOT NULL, -- JSON格式存储策略内容
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    );

    -- Marketing copies table: 营销文案 (MVP简化版)
    CREATE TABLE IF NOT EXISTS marketing_copies (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        platform TEXT NOT NULL, -- twitter/xiaohongshu/jike等
        content TEXT NOT NULL,
        created_at INTEGER NOT NULL
    );

    -- File changes table: 文件变更记录
    CREATE TABLE IF NOT EXISTS file_changes (
        id TEXT PRIMARY KEY,
        session_id TEXT NOT NULL REFERENCES cli_sessions(id) ON DELETE CASCADE,
        file_path TEXT NOT NULL,
        change_type TEXT NOT NULL, -- created/modified/deleted
        created_at INTEGER NOT NULL
    );

    -- ============================================
    -- 索引优化
    -- ============================================

    CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
    CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at DESC);
    CREATE INDEX IF NOT EXISTS idx_prds_project_id ON prds(project_id);
    CREATE INDEX IF NOT EXISTS idx_personas_project_id ON user_personas(project_id);
    CREATE INDEX IF NOT EXISTS idx_competitors_project_id ON competitors(project_id);
    CREATE INDEX IF NOT EXISTS idx_cli_sessions_project_id ON cli_sessions(project_id);
    CREATE INDEX IF NOT EXISTS idx_marketing_strategies_project_id ON marketing_strategies(project_id);
    CREATE INDEX IF NOT EXISTS idx_marketing_copies_project_id ON marketing_copies(project_id);
    CREATE INDEX IF NOT EXISTS idx_file_changes_session_id ON file_changes(session_id);
    "#,
];

/// 默认数据插入脚本
const DEFAULT_DATA: &[&str] = &[
    // 默认AI配置 - OpenAI
    r#"
    INSERT OR IGNORE INTO ai_configs (provider, api_key, base_url, model, enabled) 
    VALUES ('openai', NULL, 'https://api.openai.com/v1', 'gpt-4o-mini', 0);
    "#,
    // 默认AI配置 - Anthropic
    r#"
    INSERT OR IGNORE INTO ai_configs (provider, api_key, base_url, model, enabled) 
    VALUES ('anthropic', NULL, 'https://api.anthropic.com', 'claude-3-sonnet-20240229', 0);
    "#,
    // 默认AI配置 - Kimi
    r#"
    INSERT OR IGNORE INTO ai_configs (provider, api_key, base_url, model, enabled) 
    VALUES ('kimi', NULL, 'https://api.moonshot.cn/v1', 'moonshot-v1-8k', 0);
    "#,
    // 默认AI配置 - GLM
    r#"
    INSERT OR IGNORE INTO ai_configs (provider, api_key, base_url, model, enabled) 
    VALUES ('glm', NULL, 'https://open.bigmodel.cn/api/paas/v4', 'glm-4-flash', 0);
    "#,
    // 默认应用设置
    r#"
    INSERT OR IGNORE INTO settings (key, value, updated_at) 
    VALUES ('theme', 'system', 0);
    "#,
    r#"
    INSERT OR IGNORE INTO settings (key, value, updated_at) 
    VALUES ('language', 'zh-CN', 0);
    "#,
    r#"
    INSERT OR IGNORE INTO settings (key, value, updated_at) 
    VALUES ('auto_save', 'true', 0);
    "#,
    r#"
    INSERT OR IGNORE INTO settings (key, value, updated_at) 
    VALUES ('default_ai_provider', 'openai', 0);
    "#,
];

/// 运行所有待执行的迁移
pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 创建迁移记录表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS __migrations (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 获取已应用的版本
    let applied_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM __migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // 执行待处理的迁移
    for (i, migration) in MIGRATIONS.iter().enumerate() {
        let version = (i + 1) as i32;
        if version > applied_version {
            conn.execute_batch(migration)?;
            conn.execute(
                "INSERT INTO __migrations (version, applied_at) VALUES (?1, ?2)",
                [version, chrono::Utc::now().timestamp() as i32],
            )?;
            log::info!("Applied migration version {}", version);
        }
    }

    Ok(())
}

/// 插入默认数据
pub fn insert_default_data(conn: &Connection) -> Result<(), rusqlite::Error> {
    for sql in DEFAULT_DATA.iter() {
        conn.execute_batch(sql)?;
    }
    log::info!("Default data inserted");
    Ok(())
}

/// 初始化数据库（迁移 + 默认数据）
pub fn initialize_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    run_migrations(conn)?;
    insert_default_data(conn)?;
    Ok(())
}

/// 重置数据库（删除所有数据，保留表结构）
pub fn reset_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 按依赖顺序清空表（先清空子表，再清空父表）
    let tables_in_order = vec![
        "file_changes",
        "cli_sessions",
        "marketing_copies",
        "marketing_strategies",
        "competitors",
        "user_personas",
        "prds",
        "projects",
        "ai_configs",
        "settings",
    ];
    
    // 临时禁用外键约束（对某些SQLite版本有效）
    conn.execute("PRAGMA foreign_keys = OFF", [])?;
    
    // 清空所有表
    for table in tables_in_order {
        match conn.execute(&format!("DELETE FROM {}", table), []) {
            Ok(rows) => log::info!("Cleared table: {} ({} rows)", table, rows),
            Err(e) => log::warn!("Failed to clear table {}: {}", table, e),
        }
    }
    
    // 重新启用外键约束
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // 重新插入默认数据
    insert_default_data(conn)?;
    
    log::info!("Database reset completed");
    Ok(())
}

/// 验证数据库完整性
pub fn verify_database(conn: &Connection) -> Result<DatabaseVerification, rusqlite::Error> {
    // 检查完整性
    let integrity_check: String = conn.query_row(
        "PRAGMA integrity_check",
        [],
        |row| row.get(0),
    )?;
    
    let is_valid = integrity_check == "ok";
    
    // 获取所有表
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
         WHERE type='table' 
         AND name NOT LIKE 'sqlite_%'
         ORDER BY name"
    )?;
    
    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    
    // 获取所有索引
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
         WHERE type='index' 
         AND name NOT LIKE 'sqlite_%'
         ORDER BY name"
    )?;
    
    let indexes: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    
    // 检查外键状态
    let foreign_keys: bool = conn.query_row(
        "PRAGMA foreign_keys",
        [],
        |row| row.get::<_, i32>(0).map(|v| v == 1),
    )?;
    
    Ok(DatabaseVerification {
        is_valid,
        integrity_check,
        tables,
        indexes,
        foreign_keys_enabled: foreign_keys,
    })
}

/// 数据库验证结果
#[derive(Debug, Clone)]
pub struct DatabaseVerification {
    pub is_valid: bool,
    pub integrity_check: String,
    pub tables: Vec<String>,
    pub indexes: Vec<String>,
    pub foreign_keys_enabled: bool,
}

/// 获取数据库统计信息
pub fn get_statistics(conn: &Connection) -> Result<DatabaseStatistics, rusqlite::Error> {
    let mut stats = DatabaseStatistics::default();
    
    // 项目数
    stats.project_count = conn.query_row(
        "SELECT COUNT(*) FROM projects",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // PRD数
    stats.prd_count = conn.query_row(
        "SELECT COUNT(*) FROM prds",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // 用户画像数
    stats.persona_count = conn.query_row(
        "SELECT COUNT(*) FROM user_personas",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // 竞品数
    stats.competitor_count = conn.query_row(
        "SELECT COUNT(*) FROM competitors",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // CLI会话数
    stats.cli_session_count = conn.query_row(
        "SELECT COUNT(*) FROM cli_sessions",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // 启用的AI配置数
    stats.enabled_ai_config_count = conn.query_row(
        "SELECT COUNT(*) FROM ai_configs WHERE enabled = 1",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // 迁移版本
    stats.migration_version = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM __migrations",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    Ok(stats)
}

/// 数据库统计信息
#[derive(Debug, Clone, Default)]
pub struct DatabaseStatistics {
    pub project_count: i64,
    pub prd_count: i64,
    pub persona_count: i64,
    pub competitor_count: i64,
    pub cli_session_count: i64,
    pub enabled_ai_config_count: i64,
    pub migration_version: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        
        // 运行迁移
        run_migrations(&conn).expect("Failed to run migrations");
        
        // 验证表已创建
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert!(count > 0, "No tables created");
    }

    #[test]
    fn test_default_data() {
        let conn = Connection::open_in_memory().unwrap();
        
        run_migrations(&conn).unwrap();
        insert_default_data(&conn).expect("Failed to insert default data");
        
        // 验证默认AI配置
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ai_configs",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(count, 4, "Expected 4 default AI configs");
        
        // 验证默认设置
        let theme: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'theme'",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(theme, "system");
    }

    #[test]
    fn test_verify_database() {
        let conn = Connection::open_in_memory().unwrap();
        
        run_migrations(&conn).unwrap();
        
        let verification = verify_database(&conn).expect("Failed to verify database");
        
        assert!(verification.is_valid, "Database integrity check failed");
        assert!(!verification.tables.is_empty(), "No tables found");
    }

    #[test]
    fn test_reset_database() {
        let conn = Connection::open_in_memory().unwrap();
        
        run_migrations(&conn).unwrap();
        insert_default_data(&conn).unwrap();
        
        // 插入测试数据
        conn.execute(
            "INSERT INTO projects (id, name, description, status, created_at, updated_at) 
             VALUES ('test', 'Test', NULL, 'draft', 1, 1)",
            [],
        ).unwrap();
        
        // 验证插入成功
        let count_before: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count_before, 1);
        
        // 重置
        reset_database(&conn).expect("Failed to reset database");
        
        // 验证项目已清空但默认配置仍在
        let project_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(project_count, 0, "Projects should be empty after reset");
        
        let config_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ai_configs",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(config_count, 4, "Default AI configs should be restored");
    }

    #[test]
    fn test_initialize_database() {
        let conn = Connection::open_in_memory().unwrap();
        
        // Test full initialization
        initialize_database(&conn).expect("Failed to initialize database");
        
        // Verify tables exist
        let table_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert!(table_count > 0, "Tables should be created");
        
        // Verify default AI configs
        let config_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ai_configs",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(config_count, 4, "Should have 4 default AI configs");
        
        // Verify default settings
        let setting_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM settings",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(setting_count, 4, "Should have 4 default settings");
        
        // Verify specific settings exist
        let theme: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'theme'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(theme, "system", "Default theme should be 'system'");
        
        let language: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'language'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(language, "zh-CN", "Default language should be 'zh-CN'");
    }
}

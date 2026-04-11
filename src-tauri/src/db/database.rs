use rusqlite::{Connection, Result};
use crate::utils::paths;
use uuid::Uuid;

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

    Ok(())
}

/// 检查并修复所有项目的工作区目录
/// 
/// 遍历数据库中所有项目，确保每个项目在 ~/.opc-harness/workspaces/ 下都有对应的子目录
/// 如果目录不存在，则自动创建
pub fn ensure_all_project_workspaces() -> Result<(), String> {
    log::info!("Checking project workspace directories...");
    
    // 获取数据库连接
    let db_path = paths::get_database_path();
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // 查询所有项目
    let mut stmt = conn.prepare("SELECT id, name FROM projects")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let projects = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    })
    .map_err(|e| format!("Failed to query projects: {}", e))?;
    
    let mut verified_count = 0;
    let mut error_count = 0;
    
    for project_result in projects {
        match project_result {
            Ok((_id, name)) => {
                // 确保工作区目录存在
                match ensure_project_workspace(&name) {
                    Ok(path) => {
                        log::debug!("Workspace directory verified for project '{}': {:?}", name, path);
                        verified_count += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to ensure workspace directory for project '{}': {}", name, e);
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to read project from database: {}", e);
                error_count += 1;
            }
        }
    }
    
    log::info!(
        "Workspace directory check completed: {} verified, {} errors",
        verified_count,
        error_count
    );
    
    if error_count > 0 {
        Err(format!("Failed to create {} workspace directories", error_count))
    } else {
        Ok(())
    }
}

/// 确保项目的工作区目录存在
/// 
/// 如果目录不存在则创建，如果存在则返回路径
fn ensure_project_workspace(project_name: &str) -> Result<std::path::PathBuf, String> {
    use std::fs;
    
    // 获取工作区根目录
    let workspaces_root = paths::get_workspaces_dir();
    
    // 确保工作区根目录存在
    fs::create_dir_all(&workspaces_root)
        .map_err(|e| format!("Failed to create workspaces directory: {}", e))?;
    
    // 生成安全的项目目录名（替换非法字符）
    let safe_project_name = sanitize_project_name_for_check(project_name);
    
    // 构建项目工作区路径
    let project_workspace = workspaces_root.join(&safe_project_name);
    
    // 如果目录已存在，添加时间戳避免冲突
    let final_path = if project_workspace.exists() {
        project_workspace
    } else {
        // 创建新的工作区目录
        fs::create_dir_all(&project_workspace)
            .map_err(|e| format!("Failed to create project workspace directory: {}", e))?;
        log::info!("Created workspace directory for project '{}': {:?}", project_name, project_workspace);
        project_workspace
    };
    
    Ok(final_path)
}

/// 简化的项目名称清理函数（用于检查）
/// 
/// 与 commands::database::sanitize_project_name 保持一致的逻辑
fn sanitize_project_name_for_check(name: &str) -> String {
    // 第一步：替换所有非法字符为下划线
    let replaced = name.chars()
        .map(|c| match c {
            // 允许的字符保持不变
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c,
            // 点号也替换为下划线（避免隐藏文件和扩展名问题）
            '.' => '_',
            // 空格和其他特殊字符替换为下划线
            _ => '_',
        })
        .collect::<String>();
    
    // 第二步：移除连续的下划线
    let cleaned = replaced
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_");
    
    // 第三步：确保不以点或下划线开头
    let trimmed = cleaned
        .trim_start_matches('.')
        .trim_start_matches('_');
    
    // 第四步：如果为空，使用默认名称
    if trimmed.is_empty() {
        "unnamed_project".to_string()
    } else {
        trimmed.to_string()
    }
}

/// 获取数据库连接
pub fn get_connection(_app_handle: &tauri::AppHandle) -> Result<Connection> {
    let db_path = paths::get_database_path();
    Connection::open(&db_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use uuid::Uuid;

    #[test]
    fn test_ensure_project_workspace_creates_directory() {
        // 使用临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-opc-harness-{}", Uuid::new_v4()));
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保基础目录存在
        paths::ensure_app_directories().expect("Failed to ensure app directories");
        
        // 测试确保项目工作区目录会被创建
        let test_project_name = "Test_Project_Workspace";
        let workspaces_root = paths::get_workspaces_dir();
        let test_dir = workspaces_root.join("Test_Project_Workspace");
        
        // 调用函数
        let result = ensure_project_workspace(test_project_name);
        
        // 验证结果
        assert!(result.is_ok(), "Failed to ensure project workspace: {:?}", result.err());
        assert!(test_dir.exists(), "Workspace directory was not created");
        
        // 清理
        fs::remove_dir_all(&temp_dir).ok();
        std::env::remove_var("OPC_HARNESS_HOME");
    }

    #[test]
    fn test_ensure_project_workspace_with_special_chars() {
        // 使用临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-opc-harness-{}", Uuid::new_v4()));
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保基础目录存在
        paths::ensure_app_directories().expect("Failed to ensure app directories");
        
        // 测试包含特殊字符的项目名称
        let test_project_name = "My Test@Project#2024!";
        let workspaces_root = paths::get_workspaces_dir();
        let expected_name = "My_Test_Project_2024";
        let test_dir = workspaces_root.join(expected_name);
        
        // 调用函数
        let result = ensure_project_workspace(test_project_name);
        
        // 验证结果
        assert!(result.is_ok(), "Failed to ensure project workspace: {:?}", result.err());
        assert!(test_dir.exists(), "Workspace directory with sanitized name was not created");
        
        // 清理
        fs::remove_dir_all(&temp_dir).ok();
        std::env::remove_var("OPC_HARNESS_HOME");
    }

    #[test]
    fn test_ensure_project_workspace_existing_directory() {
        // 使用临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-opc-harness-{}", Uuid::new_v4()));
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保基础目录存在
        paths::ensure_app_directories().expect("Failed to ensure app directories");
        
        // 测试已存在的目录不会被重复创建或报错
        let test_project_name = "Existing_Project";
        let workspaces_root = paths::get_workspaces_dir();
        let test_dir = workspaces_root.join("Existing_Project");
        
        // 先创建目录
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        
        // 调用函数
        let result = ensure_project_workspace(test_project_name);
        
        // 验证结果
        assert!(result.is_ok(), "Failed to ensure existing project workspace: {:?}", result.err());
        assert!(test_dir.exists(), "Existing workspace directory should still exist");
        
        // 清理
        fs::remove_dir_all(&temp_dir).ok();
        std::env::remove_var("OPC_HARNESS_HOME");
    }
}

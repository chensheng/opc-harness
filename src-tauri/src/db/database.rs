use rusqlite::{Connection, Result};
use crate::utils::paths;

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

    // Create sprints table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sprints (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            goal TEXT NOT NULL DEFAULT '',
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'planning',
            story_ids TEXT NOT NULL DEFAULT '[]',
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
    
    // 查询所有项目（只需要ID）
    let mut stmt = conn.prepare("SELECT id FROM projects")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let projects = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })
    .map_err(|e| format!("Failed to query projects: {}", e))?;
    
    let mut verified_count = 0;
    let mut error_count = 0;
    
    for project_result in projects {
        match project_result {
            Ok(project_id) => {
                // 确保工作区目录存在（使用项目ID）
                match ensure_project_workspace(&project_id) {
                    Ok(path) => {
                        log::debug!("Workspace directory verified for project '{}': {:?}", project_id, path);
                        verified_count += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to ensure workspace directory for project '{}': {}", project_id, e);
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
fn ensure_project_workspace(project_id: &str) -> Result<std::path::PathBuf, String> {
    use std::fs;
    
    // 获取工作区根目录
    let workspaces_root = paths::get_workspaces_dir();
    
    // 确保工作区根目录存在
    fs::create_dir_all(&workspaces_root)
        .map_err(|e| format!("Failed to create workspaces directory: {}", e))?;
    
    // 构建项目工作区路径（直接使用项目ID作为目录名）
    let project_workspace = workspaces_root.join(project_id);
    
    // 如果目录已存在，直接返回路径
    if project_workspace.exists() {
        return Ok(project_workspace);
    }
    
    // 创建新的工作区目录
    fs::create_dir_all(&project_workspace)
        .map_err(|e| format!("Failed to create project workspace directory: {}", e))?;
    log::info!("Created workspace directory for project '{}': {:?}", project_id, project_workspace);
    
    Ok(project_workspace)
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
    use crate::test_utils::{TEST_MUTEX, TestCleanup};

    #[test]
    fn test_ensure_project_workspace_creates_directory() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        // 清除可能存在的环境变量
        std::env::remove_var("OPC_HARNESS_HOME");
        
        // 使用唯一的临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-opc-harness-{}", Uuid::new_v4()));
        
        // 清理可能存在的旧测试目录（确保幂等性）
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
        
        // 创建 RAII 守卫，确保无论如何都会清理
        let _cleanup = TestCleanup::new(temp_dir.clone());
        
        // 先设置独立的环境变量，再调用任何 paths 函数
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 现在获取工作区根目录（应该在临时目录下）
        let workspaces_root = paths::get_workspaces_dir();
        
        // 验证临时目录路径正确（防止路径穿越）
        assert!(workspaces_root.starts_with(&temp_dir), 
                "Workspaces root {:?} should be under temp dir {:?}", 
                workspaces_root, temp_dir);
        
        // 确保基础目录存在
        paths::ensure_app_directories().expect("Failed to ensure app directories");
        
        // 使用 UUID 作为项目 ID
        let project_id = Uuid::new_v4().to_string();
        let test_dir = workspaces_root.join(&project_id);
        
        // 调用函数
        let result = ensure_project_workspace(&project_id);
        
        // 验证结果
        assert!(result.is_ok(), "Failed to ensure project workspace: {:?}", result.err());
        assert!(test_dir.exists(), "Workspace directory was not created");
        assert_eq!(test_dir.file_name().unwrap().to_string_lossy(), project_id);
        
        // 不需要手动清理，_cleanup 会在函数退出时自动调用 Drop
    }

    #[test]
    fn test_ensure_project_workspace_existing_directory() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        // 清除可能存在的环境变量
        std::env::remove_var("OPC_HARNESS_HOME");
        
        // 使用唯一的临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-opc-harness-{}", Uuid::new_v4()));
        
        // 清理可能存在的旧测试目录
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
        
        // 创建 RAII 守卫，确保无论如何都会清理
        let _cleanup = TestCleanup::new(temp_dir.clone());
        
        // 先设置环境变量
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 获取工作区根目录
        let workspaces_root = paths::get_workspaces_dir();
        
        // 确保基础目录存在
        paths::ensure_app_directories().expect("Failed to ensure app directories");
        
        // 使用 UUID 作为项目 ID
        let project_id = Uuid::new_v4().to_string();
        let test_dir = workspaces_root.join(&project_id);
        
        // 先创建目录
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        
        // 调用函数
        let result = ensure_project_workspace(&project_id);
        
        // 验证结果
        assert!(result.is_ok(), "Failed to ensure existing project workspace: {:?}", result.err());
        assert!(test_dir.exists(), "Existing workspace directory should still exist");
        assert_eq!(test_dir.file_name().unwrap().to_string_lossy(), project_id);
        
        // 不需要手动清理，_cleanup 会在函数退出时自动调用 Drop
    }
}

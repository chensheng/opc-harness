use crate::models::{AIConfig, AgentSession, CLISession, Issue, Milestone, Project};
use chrono::Utc;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::Manager;

/// 初始化数据库连接和表结构
pub async fn init_database(app_handle: &tauri::AppHandle) -> Result<()> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));

    // Ensure the directory exists
    if let Err(e) = std::fs::create_dir_all(&app_dir) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some(format!("Failed to create app data directory: {}", e)),
        ));
    }

    let db_path = app_dir.join("opc-harness.db");

    let conn = Connection::open(db_path)?;

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

    Ok(())
}

/// 获取数据库连接
pub fn get_connection(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    let db_path = app_dir.join("opc-harness.db");
    Connection::open(db_path)
}

// ==================== Project CRUD ====================

/// 创建新项目
pub fn create_project(conn: &Connection, project: &Project) -> Result<()> {
    conn.execute(
        "INSERT INTO projects (id, name, description, status, progress, created_at, updated_at, idea, prd, user_personas, competitor_analysis)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &project.id,
            &project.name,
            &project.description,
            &project.status,
            &project.progress.to_string(),
            &project.created_at,
            &project.updated_at,
            // 使用 as_deref() 将 Option<String> 转换为 Option<&str>
            // rusqlite 可以正确处理 Option<&str> (NULL 或字符串)
            project.idea.as_deref(),
            project.prd.as_deref(),
            project.user_personas.as_deref(),
            project.competitor_analysis.as_deref(),
        ),
    )?;
    Ok(())
}

/// 获取所有项目
pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT * FROM projects ORDER BY updated_at DESC")?;
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            progress: row.get::<_, String>(4)?.parse::<i32>().unwrap_or(0),
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            idea: row.get(7)?,
            prd: row.get(8)?,
            user_personas: row.get(9)?,
            competitor_analysis: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for project in projects {
        result.push(project?);
    }
    Ok(result)
}

/// 获取单个项目
pub fn get_project_by_id(conn: &Connection, id: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare("SELECT * FROM projects WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            progress: row.get::<_, String>(4)?.parse::<i32>().unwrap_or(0),
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            idea: row.get(7)?,
            prd: row.get(8)?,
            user_personas: row.get(9)?,
            competitor_analysis: row.get(10)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新项目信息
pub fn update_project(conn: &Connection, project: &Project) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects 
         SET name = ?2, description = ?3, status = ?4, progress = ?5, updated_at = ?6,
             idea = ?7, prd = ?8, user_personas = ?9, competitor_analysis = ?10
         WHERE id = ?1",
        (
            &project.id,
            &project.name,
            &project.description,
            &project.status,
            &project.progress.to_string(),
            &updated_at,
            // 使用 as_deref() 将 Option<String> 转换为 Option<&str>
            // rusqlite 可以正确处理 Option<&str> (NULL 或字符串)
            project.idea.as_deref(),
            project.prd.as_deref(),
            project.user_personas.as_deref(),
            project.competitor_analysis.as_deref(),
        ),
    )?;
    Ok(())
}

/// 删除项目
pub fn delete_project(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    Ok(())
}

// ==================== AI Config CRUD ====================

/// 保存 AI 配置 (仅存储 provider 和 model，不存储 API key)
pub fn save_ai_config(conn: &Connection, config: &AIConfig) -> Result<()> {
    // Only store provider and model in database
    // API key is stored securely in OS keychain
    conn.execute(
        "INSERT OR REPLACE INTO ai_configs (provider, model)
         VALUES (?1, ?2)",
        [&config.provider, &config.model],
    )?;
    Ok(())
}

/// 获取所有 AI 配置 (不包含 API key)
pub fn get_all_ai_configs(conn: &Connection) -> Result<Vec<AIConfig>> {
    let mut stmt = conn.prepare("SELECT provider, model FROM ai_configs")?;
    let configs = stmt.query_map([], |row| Ok(AIConfig::new(row.get(0)?, row.get(1)?)))?;

    let mut result = Vec::new();
    for config in configs {
        result.push(config?);
    }
    Ok(result)
}

/// 获取单个 AI 配置 (不包含 API key)
pub fn get_ai_config(conn: &Connection, provider: &str) -> Result<Option<AIConfig>> {
    let mut stmt = conn.prepare("SELECT provider, model FROM ai_configs WHERE provider = ?1")?;
    let mut rows = stmt.query_map([provider], |row| {
        Ok(AIConfig::new(row.get(0)?, row.get(1)?))
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 删除 AI 配置
pub fn delete_ai_config(conn: &Connection, provider: &str) -> Result<()> {
    conn.execute("DELETE FROM ai_configs WHERE provider = ?1", [provider])?;
    Ok(())
}

// ==================== CLI Session CRUD ====================

/// 创建 CLI 会话
pub fn create_cli_session(conn: &Connection, session: &CLISession) -> Result<()> {
    conn.execute(
        "INSERT INTO cli_sessions (id, tool_type, project_path, created_at)
         VALUES (?1, ?2, ?3, ?4)",
        [
            &session.id,
            &session.tool_type,
            &session.project_path,
            &session.created_at,
        ],
    )?;
    Ok(())
}

/// 获取所有 CLI 会话
pub fn get_all_cli_sessions(conn: &Connection) -> Result<Vec<CLISession>> {
    let mut stmt = conn.prepare("SELECT * FROM cli_sessions ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([], |row| {
        Ok(CLISession {
            id: row.get(0)?,
            tool_type: row.get(1)?,
            project_path: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 获取单个 CLI 会话
pub fn get_cli_session_by_id(conn: &Connection, id: &str) -> Result<Option<CLISession>> {
    let mut stmt = conn.prepare("SELECT * FROM cli_sessions WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(CLISession {
            id: row.get(0)?,
            tool_type: row.get(1)?,
            project_path: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 删除 CLI 会话
pub fn delete_cli_session(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM cli_sessions WHERE id = ?1", [id])?;
    Ok(())
}

// ==================== Agent Session CRUD (VC-005) ====================

/// 创建 Agent Session
pub fn create_agent_session(conn: &Connection, session: &crate::models::AgentSession) -> Result<()> {
    conn.execute(
        "INSERT INTO agent_sessions 
         (session_id, agent_id, agent_type, project_path, status, phase, created_at, updated_at, 
          stdio_channel_id, registered_to_daemon, metadata)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        [
            &session.session_id,
            &session.agent_id,
            &session.agent_type,
            &session.project_path,
            &session.status,
            &session.phase,
            &session.created_at,
            &session.updated_at,
            &session.stdio_channel_id.clone().unwrap_or_default(),
            &(if session.registered_to_daemon { "1".to_string() } else { "0".to_string() }),
            &session.metadata.clone().unwrap_or_default(),
        ],
    )?;
    Ok(())
}

/// 获取所有 Agent Sessions
pub fn get_all_agent_sessions(conn: &Connection) -> Result<Vec<crate::models::AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([], |row| {
        Ok(crate::models::AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_path: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, i32>(9)? == 1,
            metadata: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 获取单个 Agent Session
pub fn get_agent_session_by_id(conn: &Connection, agent_id: &str) -> Result<Option<crate::models::AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions WHERE agent_id = ?1")?;
    let mut rows = stmt.query_map([agent_id], |row| {
        Ok(crate::models::AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_path: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, i32>(9)? == 1,
            metadata: row.get(10)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 根据 Session ID 获取所有 Agents
pub fn get_agent_sessions_by_session_id(conn: &Connection, session_id: &str) -> Result<Vec<crate::models::AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions WHERE session_id = ?1 ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([session_id], |row| {
        Ok(crate::models::AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_path: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, i32>(9)? == 1,
            metadata: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 更新 Agent Session 状态
pub fn update_agent_session_status(conn: &Connection, agent_id: &str, status: &str, phase: &str) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE agent_sessions 
         SET status = ?1, phase = ?2, updated_at = ?3
         WHERE agent_id = ?4",
        [status, phase, &updated_at, agent_id],
    )?;
    Ok(())
}

/// 更新 Agent Session 完整信息
pub fn update_agent_session(conn: &Connection, session: &crate::models::AgentSession) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE agent_sessions 
         SET session_id = ?2, agent_type = ?3, project_path = ?4, status = ?5, phase = ?6, 
             updated_at = ?7, stdio_channel_id = ?8, registered_to_daemon = ?9, metadata = ?10
         WHERE agent_id = ?1",
        [
            &session.agent_id,
            &session.session_id,
            &session.agent_type,
            &session.project_path,
            &session.status,
            &session.phase,
            &updated_at,
            &session.stdio_channel_id.clone().unwrap_or_default(),
            &(if session.registered_to_daemon { "1".to_string() } else { "0".to_string() }),
            &session.metadata.clone().unwrap_or_default(),
        ],
    )?;
    Ok(())
}

/// 删除 Agent Session
pub fn delete_agent_session(conn: &Connection, agent_id: &str) -> Result<()> {
    conn.execute("DELETE FROM agent_sessions WHERE agent_id = ?1", [agent_id])?;
    Ok(())
}

// ==================== Milestone CRUD ====================

/// 创建里程碑
pub fn create_milestone(conn: &Connection, milestone: &Milestone) -> Result<()> {
    conn.execute(
        "INSERT INTO milestones (id, project_id, title, description, order_index, status, due_date, completed_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &milestone.id,
            &milestone.project_id,
            &milestone.title,
            &milestone.description,
            &milestone.order,
            &milestone.status,
            &milestone.due_date,
            &milestone.completed_at,
            &milestone.created_at,
            &milestone.updated_at,
        ),
    )?;
    Ok(())
}

/// 获取项目的所有里程碑
pub fn get_milestones_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Milestone>> {
    let mut stmt = conn.prepare("SELECT * FROM milestones WHERE project_id = ?1 ORDER BY order_index")?;
    let milestones = stmt.query_map([project_id], |row| {
        Ok(Milestone {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            order: row.get(4)?,
            status: row.get(5)?,
            due_date: row.get(6)?,
            completed_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut result = Vec::new();
    for milestone in milestones {
        result.push(milestone?);
    }
    Ok(result)
}

/// 获取单个里程碑
pub fn get_milestone_by_id(conn: &Connection, id: &str) -> Result<Option<Milestone>> {
    let mut stmt = conn.prepare("SELECT * FROM milestones WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Milestone {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            order: row.get(4)?,
            status: row.get(5)?,
            due_date: row.get(6)?,
            completed_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新里程碑信息
pub fn update_milestone(conn: &Connection, milestone: &Milestone) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE milestones 
         SET title = ?2, description = ?3, order_index = ?4, status = ?5, 
             due_date = ?6, completed_at = ?7, updated_at = ?8
         WHERE id = ?1",
        (
            &milestone.id,
            &milestone.title,
            &milestone.description,
            &milestone.order,
            &milestone.status,
            &milestone.due_date,
            &milestone.completed_at,
            &updated_at,
        ),
    )?;
    Ok(())
}

/// 删除里程碑
pub fn delete_milestone(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM milestones WHERE id = ?1", [id])?;
    Ok(())
}

// ==================== Issue CRUD ====================

/// 创建 Issue
pub fn create_issue(conn: &Connection, issue: &Issue) -> Result<()> {
    conn.execute(
        "INSERT INTO issues (id, project_id, milestone_id, title, description, issue_type, priority, status, assignee, parent_issue_id, order_index, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        (
            &issue.id,
            &issue.project_id,
            &issue.milestone_id,
            &issue.title,
            &issue.description,
            &issue.issue_type,
            &issue.priority,
            &issue.status,
            &issue.assignee,
            &issue.parent_issue_id,
            &issue.order,
            &issue.created_at,
            &issue.updated_at,
        ),
    )?;
    Ok(())
}

/// 获取项目的所有 Issues
pub fn get_issues_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE project_id = ?1 ORDER BY order_index")?;
    let issues = stmt.query_map([project_id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for issue in issues {
        result.push(issue?);
    }
    Ok(result)
}

/// 按里程碑获取 Issues
pub fn get_issues_by_milestone(conn: &Connection, milestone_id: &str) -> Result<Vec<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE milestone_id = ?1 ORDER BY order_index")?;
    let issues = stmt.query_map([milestone_id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for issue in issues {
        result.push(issue?);
    }
    Ok(result)
}

/// 获取单个 Issue
pub fn get_issue_by_id(conn: &Connection, id: &str) -> Result<Option<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新 Issue 信息
pub fn update_issue(conn: &Connection, issue: &Issue) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE issues 
         SET title = ?2, description = ?3, issue_type = ?4, priority = ?5, 
             status = ?6, assignee = ?7, parent_issue_id = ?8, order_index = ?9, updated_at = ?10
         WHERE id = ?1",
        (
            &issue.id,
            &issue.title,
            &issue.description,
            &issue.issue_type,
            &issue.priority,
            &issue.status,
            &issue.assignee,
            &issue.parent_issue_id,
            &issue.order,
            &updated_at,
        ),
    )?;
    Ok(())
}

/// 删除 Issue
pub fn delete_issue(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM issues WHERE id = ?1", [id])?;
    Ok(())
}

//! 数据库服务
//!
//! 提供SQLite数据库访问

use crate::models::{Project, ProjectStatus};
use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// 数据库服务
pub struct DBService {
    conn: Connection,
}

impl DBService {
    /// 创建新的数据库服务
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // 初始化数据库表
        Self::init_tables(&conn)?;
        
        Ok(Self { conn })
    }

    /// 初始化数据库表
    fn init_tables(conn: &Connection) -> Result<()> {
        // 项目表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                local_path TEXT NOT NULL UNIQUE,
                default_ai_provider TEXT,
                default_cli_tool TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // PRD表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS prds (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id),
                content TEXT NOT NULL,
                version INTEGER DEFAULT 1,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // 用户画像表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_personas (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id),
                name TEXT NOT NULL,
                demographics TEXT,
                pain_points TEXT,
                goals TEXT,
                behaviors TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // CLI会话表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cli_sessions (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id),
                tool_type TEXT NOT NULL,
                working_directory TEXT NOT NULL,
                status TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT
            )",
            [],
        )?;

        // 设置表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    // 项目相关操作

    /// 创建项目
    pub fn create_project(&self, project: &Project) -> Result<()> {
        self.conn.execute(
            "INSERT INTO projects (id, name, description, status, local_path, 
             default_ai_provider, default_cli_tool, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                project.id,
                project.name,
                project.description,
                format!("{:?}", project.status).to_lowercase(),
                project.local_path,
                project.default_ai_provider,
                project.default_cli_tool,
                project.created_at,
                project.updated_at,
            ],
        )?;
        Ok(())
    }

    /// 获取所有项目
    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, status, local_path, 
             default_ai_provider, default_cli_tool, created_at, updated_at
             FROM projects ORDER BY updated_at DESC"
        )?;

        let projects = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: match row.get::<_, String>(3)?.as_str() {
                    "designing" => ProjectStatus::Designing,
                    "coding" => ProjectStatus::Coding,
                    "marketing" => ProjectStatus::Marketing,
                    "completed" => ProjectStatus::Completed,
                    _ => ProjectStatus::Designing,
                },
                local_path: row.get(4)?,
                default_ai_provider: row.get(5)?,
                default_cli_tool: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    /// 获取单个项目
    pub fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, status, local_path, 
             default_ai_provider, default_cli_tool, created_at, updated_at
             FROM projects WHERE id = ?1"
        )?;

        let project = stmt.query_row([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: match row.get::<_, String>(3)?.as_str() {
                    "designing" => ProjectStatus::Designing,
                    "coding" => ProjectStatus::Coding,
                    "marketing" => ProjectStatus::Marketing,
                    "completed" => ProjectStatus::Completed,
                    _ => ProjectStatus::Designing,
                },
                local_path: row.get(4)?,
                default_ai_provider: row.get(5)?,
                default_cli_tool: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        }).optional()?;

        Ok(project)
    }

    /// 更新项目
    pub fn update_project(&self, project: &Project) -> Result<()> {
        self.conn.execute(
            "UPDATE projects SET 
             name = ?2, description = ?3, status = ?4, local_path = ?5,
             default_ai_provider = ?6, default_cli_tool = ?7, updated_at = ?8
             WHERE id = ?1",
            params![
                project.id,
                project.name,
                project.description,
                format!("{:?}", project.status).to_lowercase(),
                project.local_path,
                project.default_ai_provider,
                project.default_cli_tool,
                project.updated_at,
            ],
        )?;
        Ok(())
    }

    /// 删除项目
    pub fn delete_project(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        Ok(())
    }

    // 设置相关操作

    /// 保存设置
    pub fn save_setting(&self, key: &str, value: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            params![key, value, now],
        )?;
        Ok(())
    }

    /// 获取设置
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let value = stmt.query_row([key], |row| row.get(0)).optional()?;
        Ok(value)
    }
}

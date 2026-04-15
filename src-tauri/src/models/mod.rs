use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub progress: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idea: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_personas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    // Note: api_key is no longer stored in database
    // It's stored securely in OS keychain
    #[serde(skip_serializing, skip_deserializing)]
    pub api_key: String,
}

impl AIConfig {
    /// Create a new AIConfig without API key (for DB storage)
    pub fn new(provider: String, model: String) -> Self {
        Self {
            provider,
            model,
            api_key: String::new(), // Empty placeholder
        }
    }

    /// Create a new AIConfig with API key (for runtime use only)
    #[allow(dead_code)]
    pub fn with_key(provider: String, model: String, api_key: String) -> Self {
        Self {
            provider,
            model,
            api_key,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CLISession {
    pub id: String,
    pub tool_type: String,
    pub project_path: String,
    pub created_at: String,
}

/// Agent Session 会话记录
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSession {
    /// Session 唯一标识
    pub session_id: String,
    /// Agent ID
    pub agent_id: String,
    /// Agent 类型
    pub agent_type: String,
    /// 项目ID
    pub project_id: String,
    /// 智能体名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 当前状态
    pub status: String,
    /// 当前阶段
    pub phase: String,
    /// 创建时间
    pub created_at: String,
    /// 最后更新时间
    pub updated_at: String,
    /// Stdio 通道 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdio_channel_id: Option<String>,
    /// 是否已注册到 Daemon
    pub registered_to_daemon: bool,
    /// 元数据（JSON）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

/// 项目里程碑
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    /// 里程碑 ID
    pub id: String,
    /// 所属项目 ID
    pub project_id: String,
    /// 里程碑标题
    pub title: String,
    /// 详细描述
    pub description: String,
    /// 排序顺序
    pub order: i32,
    /// 状态
    pub status: String,
    /// 截止日期（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// 完成时间（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

// ==================== Entity Trait Implementations ====================

use crate::db::Entity;
use rusqlite::Row;

impl Entity for Project {
    fn table_name() -> &'static str { "projects" }
    
    fn primary_key() -> &'static str { "id" }
    
    fn get_primary_key(&self) -> &str { &self.id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            idea: row.get(7)?,
            prd: row.get(8)?,
            user_personas: row.get(9)?,
            competitor_analysis: row.get(10)?,
        })
    }
}

impl Entity for Milestone {
    fn table_name() -> &'static str { "milestones" }
    
    fn primary_key() -> &'static str { "id" }
    
    fn get_primary_key(&self) -> &str { &self.id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
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
    }
}

impl Entity for Issue {
    fn table_name() -> &'static str { "issues" }
    
    fn primary_key() -> &'static str { "id" }
    
    fn get_primary_key(&self) -> &str { &self.id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
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
    }
}

impl Entity for AgentSession {
    fn table_name() -> &'static str { "agent_sessions" }
    
    fn primary_key() -> &'static str { "agent_id" }
    
    fn get_primary_key(&self) -> &str { &self.agent_id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_id: row.get(3)?,
            name: row.get(4)?,
            status: row.get(5)?,
            phase: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            stdio_channel_id: row.get(9)?,
            registered_to_daemon: row.get::<_, i32>(10)? != 0,
            metadata: row.get(11)?,
        })
    }
}

/// 项目任务/问题
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    /// Issue ID
    pub id: String,
    /// 所属项目 ID
    pub project_id: String,
    /// 关联的里程碑 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<String>,
    /// Issue 标题
    pub title: String,
    /// 详细描述
    pub description: String,
    /// 类型：feature/bug/task
    pub issue_type: String,
    /// 优先级：critical/high/medium/low
    pub priority: String,
    /// 状态：open/in_progress/done/closed
    pub status: String,
    /// 负责人（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// 父 Issue ID（可选，用于任务分解）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_issue_id: Option<String>,
    /// 排序顺序
    pub order: i32,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 用户故事
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStory {
    /// 故事 ID
    pub id: String,
    /// 所属项目 ID
    pub project_id: String,
    /// 故事编号 (如 US-001)
    pub story_number: String,
    /// 标题
    pub title: String,
    /// 角色
    pub role: String,
    /// 功能
    pub feature: String,
    /// 价值
    pub benefit: String,
    /// 描述 (As a... I want... So that...)
    pub description: String,
    /// 验收标准 (JSON数组)
    pub acceptance_criteria: String,
    /// 优先级: P0/P1/P2/P3
    pub priority: String,
    /// 故事点
    pub story_points: i32,
    /// 状态: draft/backlog/in_progress/done
    pub status: String,
    /// 模块/史诗
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epic: Option<String>,
    /// 标签 (JSON数组)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// 依赖关系 (JSON数组)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<String>,
    /// 所属 Sprint ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprint_id: Option<String>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

impl Entity for UserStory {
    fn table_name() -> &'static str { "user_stories" }
    
    fn primary_key() -> &'static str { "id" }
    
    fn get_primary_key(&self) -> &str { &self.id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(UserStory {
            id: row.get("id")?,
            project_id: row.get("project_id")?,
            story_number: row.get("story_number")?,
            title: row.get("title")?,
            role: row.get("role")?,
            feature: row.get("feature")?,
            benefit: row.get("benefit")?,
            description: row.get("description")?,
            acceptance_criteria: row.get("acceptance_criteria")?,
            priority: row.get("priority")?,
            story_points: row.get("story_points")?,
            status: row.get("status")?,
            epic: row.get("epic")?,
            labels: row.get("labels")?,
            dependencies: row.get("dependencies")?,
            sprint_id: row.get("sprint_id")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

/// Sprint 迭代计划
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprint {
    /// Sprint ID
    pub id: String,
    /// 所属项目 ID
    pub project_id: String,
    /// Sprint 名称
    pub name: String,
    /// Sprint 目标
    pub goal: String,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 状态: planning/active/completed/cancelled
    pub status: String,
    /// 总故事点（通过查询 user_stories.sprint_id 计算得出，不存储在数据库中）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_story_points: Option<i32>,
    /// 已完成故事点（通过查询 user_stories 计算得出，不存储在数据库中）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_story_points: Option<i32>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

impl Entity for Sprint {
    fn table_name() -> &'static str { "sprints" }
    
    fn primary_key() -> &'static str { "id" }
    
    fn get_primary_key(&self) -> &str { &self.id }
    
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Sprint {
            id: row.get(0)?,
            project_id: row.get(1)?,
            name: row.get(2)?,
            goal: row.get(3)?,
            start_date: row.get(4)?,
            end_date: row.get(5)?,
            status: row.get(6)?,
            total_story_points: Some(row.get(7)?),
            completed_story_points: Some(row.get(8)?),
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }
}

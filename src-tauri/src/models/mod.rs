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
    /// 项目路径
    pub project_path: String,
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

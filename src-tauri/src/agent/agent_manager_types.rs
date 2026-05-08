//! Agent Manager 类型定义
//!
//! 包含 AgentHandle、AgentManagerStats 等核心数据结构

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::types::{AgentPhase, AgentStatus, AgentType};

/// Agent 句柄信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandle {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型
    pub agent_type: AgentType,
    /// Session ID
    pub session_id: String,
    /// 项目ID
    pub project_id: String,
    /// 智能体名称（可选）
    pub name: Option<String>,
    /// 创建时间戳
    pub created_at: i64,
    /// 最后更新时间戳
    pub updated_at: i64,
    /// 当前状态
    pub status: AgentStatus,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 关联的 Stdio 通道 ID
    pub stdio_channel_id: Option<String>,
    /// 是否已注册到 Daemon
    pub registered_to_daemon: bool,
    /// AGENTS.md 内容
    pub agents_md_content: Option<String>,
}

impl AgentHandle {
    /// 创建新的 Agent 句柄
    pub fn new(
        agent_type: AgentType,
        session_id: String,
        project_id: String,
        name: Option<String>,
        agents_md_content: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        let phase = match &agent_type {
            AgentType::Initializer => AgentPhase::Initializer,
            AgentType::Coding => AgentPhase::Coding,
            AgentType::MRCreation => AgentPhase::MRCreation,
        };

        Self {
            agent_id: Uuid::new_v4().to_string(),
            agent_type,
            session_id,
            project_id,
            name,
            created_at: now,
            updated_at: now,
            status: AgentStatus::Idle,
            phase,
            stdio_channel_id: None,
            registered_to_daemon: false,
            agents_md_content,
        }
    }

    /// 更新状态
    pub fn update_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// 设置 Stdio 通道 ID
    pub fn set_stdio_channel(&mut self, channel_id: String) {
        self.stdio_channel_id = Some(channel_id);
    }

    /// 标记为已注册到 Daemon
    pub fn mark_registered(&mut self) {
        self.registered_to_daemon = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Agent Manager 统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentManagerStats {
    /// Agent 总数
    pub total_agents: u32,
    /// 运行中的 Agent 数量
    pub running_agents: u32,
    /// 空闲的 Agent 数量
    pub idle_agents: u32,
    /// 已完成的 Agent 数量
    pub completed_agents: u32,
    /// 失败的 Agent 数量
    pub failed_agents: u32,
}

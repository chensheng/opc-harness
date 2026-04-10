//! Agent Manager 类型定义
//! 
//! 包含 AgentHandle、AgentManagerStats 等核心数据结构

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::types::{AgentType, AgentStatus, AgentPhase};

/// Agent 句柄信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandle {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型
    pub agent_type: AgentType,
    /// Session ID
    pub session_id: String,
    /// 创建时间戳
    pub created_at: i64,
    /// 最后更新时间戳
    pub updated_at: i64,
    /// 当前状态
    pub status: AgentStatus,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 项目路径
    pub project_path: String,
    /// 关联的 Stdio 通道 ID
    pub stdio_channel_id: Option<String>,
    /// 是否已注册到 Daemon
    pub registered_to_daemon: bool,
}

impl AgentHandle {
    /// 创建新的 Agent 句柄
    pub fn new(
        agent_type: AgentType,
        session_id: String,
        project_path: String,
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
            created_at: now,
            updated_at: now,
            status: AgentStatus::Idle,
            phase,
            project_path,
            stdio_channel_id: None,
            registered_to_daemon: false,
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
    /// 总 Agent 数
    pub total_agents: usize,
    /// 运行中的 Agent 数
    pub running_agents: usize,
    /// 空闲的 Agent 数
    pub idle_agents: usize,
    /// 已完成的 Agent 数
    pub completed_agents: usize,
    /// 失败的 Agent 数
    pub failed_agents: usize,
    /// 当前 Session 数
    pub active_sessions: usize,
    /// Stdio 通道总数
    pub stdio_channels: usize,
    /// WebSocket 连接数
    pub websocket_connections: usize,
}

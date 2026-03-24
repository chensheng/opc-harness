//! Agent 核心类型定义
//! 
//! 包含 Agent 生命周期、状态、配置等基础类型

use serde::{Deserialize, Serialize};

/// Agent 生命周期阶段
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentPhase {
    /// 初始化阶段：环境检查、任务分解
    Initializer,
    /// 编码阶段：代码生成、测试编写
    Coding,
    /// MR 创建阶段：汇总提交、创建合并请求
    MRCreation,
}

impl std::fmt::Display for AgentPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentPhase::Initializer => write!(f, "initializer"),
            AgentPhase::Coding => write!(f, "coding"),
            AgentPhase::MRCreation => write!(f, "mr_creation"),
        }
    }
}

/// Agent 运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// 空闲状态，等待任务
    Idle,
    /// 正在执行任务
    Running,
    /// 已暂停
    Paused,
    /// 任务完成
    Completed,
    /// 任务失败，包含错误信息
    Failed(String),
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Idle => write!(f, "idle"),
            AgentStatus::Running => write!(f, "running"),
            AgentStatus::Paused => write!(f, "paused"),
            AgentStatus::Completed => write!(f, "completed"),
            AgentStatus::Failed(msg) => write!(f, "failed:{}", msg),
        }
    }
}

/// Agent 类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    /// Initializer Agent: PRD 解析、环境检查、任务分解
    Initializer,
    /// Coding Agent: 代码生成、测试编写、质量修复
    Coding,
    /// MR Creation Agent: 分支合并、回归测试、MR 创建
    MRCreation,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Initializer => write!(f, "initializer"),
            AgentType::Coding => write!(f, "coding"),
            AgentType::MRCreation => write!(f, "mr_creation"),
        }
    }
}

/// Agent 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型
    #[serde(rename = "type")]
    pub agent_type: AgentType,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 当前状态
    pub status: AgentStatus,
    /// 项目路径
    pub project_path: String,
    /// 会话 ID
    pub session_id: String,
    /// AI 服务配置（可选）
    pub ai_config: Option<crate::ai::AIConfig>,
    /// 附加配置参数
    pub metadata: Option<serde_json::Value>,
}

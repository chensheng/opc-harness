//! Agent 协议模块
//! 
//! 本模块包含所有 Agent 相关的子模块

pub mod types;
pub mod messages;
pub mod prd_parser;  // VC-006: PRD 解析器
pub mod initializer_agent;
pub mod coding_agent;
pub mod branch_manager;
pub mod daemon;
pub mod agent_stdio;
pub mod websocket_manager;
pub mod agent_manager;

// 重新导出常用类型，方便外部使用
pub use types::{AgentPhase, AgentStatus, AgentConfig, AgentType};
pub use messages::{
    AgentRequest, AgentResponse, AgentMessage, MessageType,
    StdioCommand, StdioOutput, WebSocketMessage,
    // VC-001: 新增数据类型导出
    Issue, IssueStatus, Priority,
    CheckpointType, CheckpointRequest, CheckpointResponse, CheckpointDecision,
    QualityGateResult,
    AgentSessionState,
};
pub use initializer_agent::{
    InitializerAgent, InitializerAgentConfig, InitializerStatus,
    PRDParseResult, EnvironmentCheckResult, TaskDecompositionResult,
    InitializerResult,
};
pub use coding_agent::{
    CodingAgent, CodingAgentConfig, CodingTask, CodingTaskType,
    CodingResult, QualityCheckResult,
};
pub use branch_manager::{
    BranchManager, BranchManagerConfig, BranchInfo, BranchType,
    BranchOperationResult,
};
pub use daemon::{
    DaemonManager, DaemonConfig, DaemonStatus, DaemonSnapshot,
    DaemonCommand, DaemonEvent, AgentProcessInfo, ResourceUsage,
    SystemInfo, ConcurrencyStats,
};
pub use agent_stdio::{
    StdioChannel, StdioChannelManager, StdioChannelConfig, StdioChannelStatus, StdioChannelStats,
    StdioMessage, StdioMessageType,
};
// VC-004: 导出 Agent Manager 相关类型
pub use agent_manager::{
    AgentManager, AgentHandle, AgentManagerStats,
};

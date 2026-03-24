//! Agent 协议模块
//! 
//! 本模块定义了 Agent 与守护进程、前端之间的通信协议
//! 支持 Stdio 管道通信和 WebSocket 实时推送

pub mod types;
pub mod messages;
pub mod coding_agent;
pub mod branch_manager;
pub mod daemon;

// 重新导出常用类型，方便外部使用
pub use types::{AgentPhase, AgentStatus, AgentConfig};
pub use messages::{
    AgentRequest, AgentResponse, AgentMessage, MessageType,
    StdioCommand, StdioOutput, WebSocketMessage,
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

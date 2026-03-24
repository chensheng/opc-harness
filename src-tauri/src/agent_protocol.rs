//! Agent 通信协议定义（向后兼容的重导出模块）
//! 
//! 本模块已重构为多模块结构，详细内容请查看 agent/ 子模块
//! 
//! 模块结构:
//! - types: 基础类型定义 (AgentPhase, AgentStatus, AgentConfig 等)
//! - messages: 消息结构 (AgentRequest, AgentResponse, AgentMessage 等)
//! - coding_agent: Coding Agent 实现
//! - branch_manager: Branch Manager 实现
//! - daemon: Daemon Manager 实现

// 重新导出所有 agent 模块内容以保持向后兼容性
pub use crate::agent::*;
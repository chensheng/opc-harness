//! Agent 协议模块
//! 
//! 本模块包含所有 Agent 相关的子模块

pub mod types;
pub mod messages;
pub mod prd_parser;  // VC-006: PRD 解析器
pub mod initializer_agent;
pub mod coding_agent;
pub mod branch_manager;
pub mod mr_creation_agent;  // VC-016: MR Creation Agent
pub mod mr_description_generator;  // MR Description Generator
pub mod test_generator_agent;  // VC-021: Test Generator Agent
pub mod debug_agent;  // VC-022: Debug Agent
pub mod git_commit_assistant;  // VC-026: Git Commit Assistant
pub mod code_review_agent;  // VC-027: Code Review Agent
pub mod realtime_review_manager;  // VC-028: Real-time Review Manager
pub mod test_runner_agent;  // VC-029: Test Runner Agent
pub mod performance_benchmark_agent;  // VC-030: Performance Benchmark Agent
pub mod realtime_performance_monitor;  // VC-031: Real-time Performance Monitor
pub mod ai_code_generator;  // VC-032: AI Code Generator
pub mod realtime_code_suggestions;  // VC-033: Real-time Code Suggestions
pub mod code_change_tracker;  // VC-034: Code Change Tracker
pub mod daemon;
pub mod agent_stdio;
pub mod websocket_manager;
pub mod agent_manager;

// Agent Manager 子模块（内部使用）
mod agent_manager_types;
mod agent_manager_persistence;
mod agent_manager_core;
mod agent_manager_commands;

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
pub use mr_creation_agent::{
    MRCreationAgent, MRCreationConfig, MRCreationStatus,
    MRCreationResult, MRDescription, MergeConflict, ConflictType,
};
pub use test_generator_agent::{
    TestGeneratorAgent, TestGeneratorConfig, TestGeneratorStatus,
    TestGenerationResult, TestFile, TestCase, TestFramework, TestType,
    SourceAnalysis, FunctionInfo, ClassInfo, ParameterInfo, PropertyInfo,
};
pub use debug_agent::{
    DebugAgent, DebugAgentConfig, DebugStatus, DebugResult,
    ErrorInfo, ErrorType, ErrorSource, Diagnosis,
};
pub use git_commit_assistant::{
    GitCommitAssistant, GitCommitAssistantConfig, CommitStatus, CommitMessage,
    ChangeInfo, FileChangeType, CommitType,
};
pub use code_review_agent::{
    CodeReviewAgent, CodeReviewAgentConfig, CodeReviewStatus, ReviewResult,
    ReviewComment, ReviewSeverity, ReviewDimension, CodeChange,
};
pub use realtime_review_manager::{
    RealtimeReviewManager, WatchConfig, WatchStatus, FileChangeEvent, RealtimeReviewResult,
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
pub mod code_diff_visualizer;

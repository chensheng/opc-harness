//! Agent 协议模块
//! 
//! 本模块包含所有 Agent 相关的子模块

// 完全去中心化智能体系统 (Fully Decentralized Agent System)
pub mod agent_worker;
pub mod agent_worker_commands;

// Agent Manager (用于管理去中心化 Workers)
pub mod agent_manager;
pub mod agent_manager_core;
pub mod agent_manager_commands;
pub mod agent_manager_persistence;
pub mod agent_manager_types;

// 基础类型和工具
pub mod types;
pub mod messages;

// Daemon 进程管理
pub mod daemon;
pub mod daemon_core;
pub mod daemon_types;

// Worktree 管理
pub mod worktree_manager;

// AI CLI 交互
pub mod ai_cli_interaction;
pub mod agent_stdio;

// WebSocket 管理
pub mod websocket_manager;

// Git 分支管理
pub mod branch_manager;

// 各种 Agent 实现
pub mod initializer_agent;
pub mod mr_creation_agent;
pub mod coding_agent;
pub mod debug_agent;
pub mod git_commit_assistant;
pub mod realtime_code_suggestions;
pub mod mr_description_generator;
pub mod code_change_tracker;
pub mod code_diff_visualizer;
pub mod code_review_agent;
pub mod realtime_review_manager;
pub mod test_runner_agent;
pub mod performance_benchmark_agent;
pub mod realtime_performance_monitor;
pub mod ai_code_generator;
pub mod prd_parser;
pub mod test_generator_agent;

// 旧的去中心化智能体系统 (保留作为兼容，待移除)
pub mod decentralized {
    pub mod event_bus;
    pub mod distributed_lock;
    pub mod node;
}

// 去中心化命令接口
pub mod decentralized_commands;
//! Agent 协议模块
//! 
//! 本模块包含所有 Agent 相关的子模块

pub mod agent_loop;
pub mod agent_manager;
pub mod agent_manager_core;
pub mod agent_manager_commands;
pub mod agent_manager_persistence;
pub mod agent_manager_types;
pub mod types;
pub mod daemon;
pub mod daemon_core;
pub mod daemon_types;
pub mod worktree_manager;
pub mod ai_cli_interaction;
pub mod agent_stdio;
pub mod websocket_manager;
pub mod branch_manager;
pub mod initializer_agent;
pub mod mr_creation_agent;
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
pub mod messages;
pub mod prd_parser;
pub mod coding_agent;
pub mod test_generator_agent;

// 完全去中心化智能体系统 (Fully Decentralized Agent System)
pub mod agent_worker;
pub mod agent_worker_commands;

// 旧的去中心化智能体系统 (保留作为兼容)
pub mod decentralized {
    pub mod event_bus;
    pub mod distributed_lock;
    pub mod node;
}

// 去中心化命令接口
pub mod decentralized_commands;

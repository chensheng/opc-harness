//! Agent 协议模块
//! 
//! 本模块包含所有 Agent 相关的子模块

pub mod agent_loop;
pub mod agent_manager;
pub mod agent_manager_core;
pub mod agent_manager_commands;
pub mod agent_manager_persistence;
pub mod types;
pub mod daemon;
pub mod daemon_core;
pub mod daemon_types;
pub mod worktree_manager;
pub mod ai_cli_interaction;
pub mod story_status_updater;
pub mod git_ops;
pub mod code_writer;

// 去中心化智能体系统 (Decentralized Agent System)
pub mod decentralized {
    pub mod event_bus;
    pub mod distributed_lock;
    pub mod node;
}

// 去中心化命令接口
pub mod decentralized_commands;

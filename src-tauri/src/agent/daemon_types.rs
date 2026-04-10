//! Daemon Types and Data Structures
//! 
//! 包含所有 Daemon 相关的类型定义和数据结构

use serde::{Deserialize, Serialize};
use crate::agent::types::AgentStatus;

/// 守护进程运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaemonStatus {
    Starting,      // 启动中
    Running,       // 运行中
    Paused,        // 已暂停
    Stopping,      // 停止中
    Stopped,       // 已停止
    Failed(String), // 失败 (含错误信息)
}

/// 守护进程配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub session_id: String,           // 会话 ID
    pub project_path: String,         // 项目路径
    pub log_level: String,            // 日志级别：debug/info/warn/error
    pub max_concurrent_agents: usize, // 最大并发 Agent 数
    pub workspace_dir: String,        // 工作目录
}

/// Agent 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProcessInfo {
    pub agent_id: String,             // Agent 唯一标识
    pub agent_type: String,           // Agent 类型：initializer/coding/mr_creation
    pub pid: Option<u32>,             // 进程 ID
    pub status: AgentStatus,          // 运行状态
    pub started_at: i64,              // 启动时间戳
    pub resource_usage: ResourceUsage, // 资源使用情况
}

/// 资源使用情况
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_percent: f32,    // CPU 使用率 (%)
    pub memory_mb: usize,    // 内存使用量 (MB)
    pub disk_io_read: u64,   // 磁盘读取 (bytes)
    pub disk_io_write: u64,  // 磁盘写入 (bytes)
    pub network_rx: u64,     // 网络接收 (bytes)
    pub network_tx: u64,     // 网络发送 (bytes)
}

/// 守护进程状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSnapshot {
    pub daemon_id: String,              // 守护进程 ID
    pub status: DaemonStatus,           // 运行状态
    pub config: DaemonConfig,           // 配置信息
    pub active_agents: Vec<AgentProcessInfo>, // 活跃的 Agent 列表
    pub completed_tasks: Vec<String>,   // 已完成的任务列表
    pub pending_tasks: Vec<String>,     // 待处理的任务列表
    pub start_time: i64,                // 启动时间戳
    pub last_update: i64,               // 最后更新时间戳
    pub system_info: SystemInfo,        // 系统信息
}

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,                     // 操作系统
    pub arch: String,                   // 架构
    pub total_memory: u64,              // 总内存 (MB)
    pub available_memory: u64,          // 可用内存 (MB)
    pub cpu_cores: usize,               // CPU 核心数
    pub rust_version: String,           // Rust 版本
}

/// 守护进程命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {
    Start { config: DaemonConfig },     // 启动守护进程
    Stop { graceful: bool },            // 停止守护进程
    Pause,                               // 暂停
    Resume,                              // 恢复
    SpawnAgent { agent_type: String },  // 生成新的 Agent
    KillAgent { agent_id: String },     // 终止指定 Agent
    GetStatus,                           // 获取状态
    GetSnapshot,                         // 获取快照
}

/// 守护进程事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonEvent {
    Started,                             // 已启动
    Stopped,                             // 已停止
    AgentSpawned { agent_id: String },  // Agent 已生成
    AgentCompleted { agent_id: String }, // Agent 已完成
    AgentFailed { agent_id: String, error: String }, // Agent 失败
    ResourceWarning { message: String }, // 资源警告
    Error { message: String },           // 错误事件
}

/// 并发统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyStats {
    pub running_count: usize,      // 当前运行数
    pub max_concurrent: usize,     // 最大并发数
    pub queued_count: usize,       // 等待队列长度
    pub available_slots: usize,    // 可用槽位数
    pub utilization: f32,          // 并发利用率 (%)
}

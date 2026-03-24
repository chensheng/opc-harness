//! Daemon Manager 实现
//! 
//! 负责守护进程的启动、停止、Agent 调度和并发控制

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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

/// 守护进程管理器
pub struct DaemonManager {
    daemon_id: String,
    status: DaemonStatus,
    config: Option<DaemonConfig>,
    agents: HashMap<String, AgentProcessInfo>,
    completed_tasks: Vec<String>,
    pending_tasks: Vec<String>,
    start_time: i64,
    // ========== VC-013: 并发控制相关字段 ==========
    running_count: usize,                    // 当前运行中的 Agent 数量
    max_concurrent: usize,                   // 最大并发数 (从 config 同步)
    agent_queue: Vec<String>,                // 等待中的 Agent ID 队列
    running_agents: HashSet<String>, // 正在运行的 Agent ID 集合
}

impl DaemonManager {
    /// 创建新的守护进程管理器
    pub fn new() -> Self {
        Self {
            daemon_id: uuid::Uuid::new_v4().to_string(),
            status: DaemonStatus::Stopped,
            config: None,
            agents: HashMap::new(),
            completed_tasks: Vec::new(),
            pending_tasks: Vec::new(),
            start_time: 0,
            // ========== VC-013: 并发控制初始化 ==========
            running_count: 0,
            max_concurrent: 5, // 默认值，会在 start 时被 config 覆盖
            agent_queue: Vec::new(),
            running_agents: HashSet::new(),
        }
    }

    /// 启动守护进程
    pub fn start(&mut self, config: DaemonConfig) -> Result<(), String> {
        if self.status == DaemonStatus::Running {
            return Err("Daemon is already running".to_string());
        }

        // ========== VC-013: 初始化并发控制配置 ==========
        self.max_concurrent = config.max_concurrent_agents;
        
        self.config = Some(config);
        self.status = DaemonStatus::Starting;
        self.start_time = chrono::Utc::now().timestamp();
        
        // TODO: 初始化资源监控、日志系统等
        self.status = DaemonStatus::Running;
        
        Ok(())
    }

    /// 停止守护进程
    pub fn stop(&mut self, graceful: bool) -> Result<(), String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        self.status = DaemonStatus::Stopping;
        
        // TODO: 停止所有 Agent 进程
        if graceful {
            // 优雅停止：等待当前任务完成
            for agent in self.agents.values_mut() {
                if agent.status == AgentStatus::Running {
                    agent.status = AgentStatus::Paused;
                }
            }
        } else {
            // 强制停止：立即终止所有 Agent
            self.agents.clear();
        }
        
        self.status = DaemonStatus::Stopped;
        self.pending_tasks.clear();
        
        Ok(())
    }

    /// 暂停守护进程
    pub fn pause(&mut self) -> Result<(), String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        for agent in self.agents.values_mut() {
            if agent.status == AgentStatus::Running {
                agent.status = AgentStatus::Paused;
            }
        }
        
        self.status = DaemonStatus::Paused;
        Ok(())
    }

    /// 恢复守护进程
    pub fn resume(&mut self) -> Result<(), String> {
        if self.status != DaemonStatus::Paused {
            return Err("Daemon is not paused".to_string());
        }

        for agent in self.agents.values_mut() {
            if agent.status == AgentStatus::Paused {
                agent.status = AgentStatus::Running;
            }
        }
        
        self.status = DaemonStatus::Running;
        Ok(())
    }

    /// 生成新的 Agent 进程
    pub fn spawn_agent(&mut self, agent_type: &str) -> Result<String, String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        let agent_id = format!("{}-{}", agent_type, uuid::Uuid::new_v4());
        
        let agent_info = AgentProcessInfo {
            agent_id: agent_id.clone(),
            agent_type: agent_type.to_string(),
            pid: None, // TODO: 实际启动进程后设置
            status: AgentStatus::Idle,
            started_at: chrono::Utc::now().timestamp(),
            resource_usage: ResourceUsage::default(),
        };

        self.agents.insert(agent_id.clone(), agent_info);
        self.pending_tasks.push(agent_id.clone());
        
        Ok(agent_id)
    }

    /// 终止指定 Agent
    pub fn kill_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // TODO: 实际终止进程
        self.agents.remove(agent_id);
        self.pending_tasks.retain(|id| id != agent_id);
        
        Ok(())
    }

    /// 获取守护进程状态
    pub fn get_status(&self) -> DaemonStatus {
        self.status.clone()
    }

    /// 获取守护进程快照
    pub fn get_snapshot(&self) -> DaemonSnapshot {
        let system_info = self.get_system_info();
        
        DaemonSnapshot {
            daemon_id: self.daemon_id.clone(),
            status: self.status.clone(),
            config: self.config.clone().unwrap_or_else(|| DaemonConfig {
                session_id: String::new(),
                project_path: String::new(),
                log_level: "info".to_string(),
                max_concurrent_agents: 5,
                workspace_dir: String::new(),
            }),
            active_agents: self.agents.values().cloned().collect(),
            completed_tasks: self.completed_tasks.clone(),
            pending_tasks: self.pending_tasks.clone(),
            start_time: self.start_time,
            last_update: chrono::Utc::now().timestamp(),
            system_info,
        }
    }

    /// 获取系统信息
    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            total_memory: self.get_total_memory(),
            available_memory: self.get_available_memory(),
            cpu_cores: num_cpus::get(),
            rust_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 获取总内存 (MB)
    fn get_total_memory(&self) -> u64 {
        // TODO: 使用 sysinfo crate 获取真实值
        16384 // 默认 16GB
    }

    /// 获取可用内存 (MB)
    fn get_available_memory(&self) -> u64 {
        // TODO: 使用 sysinfo crate 获取真实值
        8192 // 默认 8GB
    }

    /// 更新资源使用情况
    pub fn update_resource_usage(&mut self) {
        for agent in self.agents.values_mut() {
            // TODO: 使用 sysinfo crate 获取真实资源使用情况
            agent.resource_usage = ResourceUsage {
                cpu_percent: 0.0,
                memory_mb: 0,
                disk_io_read: 0,
                disk_io_write: 0,
                network_rx: 0,
                network_tx: 0,
            };
        }
    }

    /// 标记任务完成
    pub fn mark_task_completed(&mut self, task_id: &str) {
        self.pending_tasks.retain(|id| id != task_id);
        self.completed_tasks.push(task_id.to_string());
    }

    // ========== VC-013: 并发控制核心方法 ==========

    /// 检查是否可以启动新的 Agent
    pub fn can_spawn_agent(&self) -> bool {
        self.running_count < self.max_concurrent
    }

    /// 获取可用的并发槽位数
    pub fn available_slots(&self) -> usize {
        if self.running_count >= self.max_concurrent {
            0
        } else {
            self.max_concurrent - self.running_count
        }
    }

    /// 尝试启动 Agent（受并发限制）
    /// 返回 true 表示可以立即启动，false 表示需要排队
    pub fn try_start_agent(&mut self, agent_id: &str) -> bool {
        // 检查是否已经在运行
        if self.running_agents.contains(agent_id) {
            return true;
        }

        // 检查是否有可用槽位
        if self.can_spawn_agent() {
            self.running_agents.insert(agent_id.to_string());
            self.running_count += 1;
            
            // 更新 Agent 状态为 Running
            if let Some(agent) = self.agents.get_mut(agent_id) {
                agent.status = AgentStatus::Running;
            }
            
            true
        } else {
            // 加入等待队列
            if !self.agent_queue.contains(&agent_id.to_string()) {
                self.agent_queue.push(agent_id.to_string());
                
                // 更新 Agent 状态为 Idle (等待中)
                if let Some(agent) = self.agents.get_mut(agent_id) {
                    agent.status = AgentStatus::Idle;
                }
            }
            false
        }
    }

    /// 停止 Agent 并释放槽位
    pub fn stop_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // 从运行集合中移除
        if self.running_agents.remove(agent_id) {
            self.running_count = self.running_count.saturating_sub(1);
        }

        // 更新 Agent 状态
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Completed;
        }

        // 从等待队列移除
        self.agent_queue.retain(|id| id != agent_id);

        // ========== VC-013: 调度队列中的下一个 Agent ==========
        self.schedule_next_agent();

        Ok(())
    }

    /// 调度队列中的下一个 Agent
    fn schedule_next_agent(&mut self) {
        // 检查是否有可用槽位和等待中的 Agent
        while self.can_spawn_agent() && !self.agent_queue.is_empty() {
            if let Some(next_agent_id) = self.agent_queue.first().cloned() {
                self.agent_queue.remove(0);
                
                // 启动该 Agent
                self.running_agents.insert(next_agent_id.clone());
                self.running_count += 1;
                
                // 更新 Agent 状态
                if let Some(agent) = self.agents.get_mut(&next_agent_id) {
                    agent.status = AgentStatus::Running;
                }
                
                // TODO: 实际启动 Agent 进程
            } else {
                break;
            }
        }
    }

    /// 获取当前并发统计信息
    pub fn get_concurrency_stats(&self) -> ConcurrencyStats {
        ConcurrencyStats {
            running_count: self.running_count,
            max_concurrent: self.max_concurrent,
            queued_count: self.agent_queue.len(),
            available_slots: self.available_slots(),
            utilization: if self.max_concurrent > 0 {
                (self.running_count as f32 / self.max_concurrent as f32) * 100.0
            } else {
                0.0
            },
        }
    }

    /// 动态调整最大并发数
    pub fn adjust_max_concurrent(&mut self, new_limit: usize) -> Result<(), String> {
        if new_limit == 0 {
            return Err("max_concurrent must be greater than 0".to_string());
        }

        let _old_limit = self.max_concurrent;
        self.max_concurrent = new_limit;

        // 如果新限制小于当前运行数，需要暂停部分 Agent
        if new_limit < self.running_count {
            // 暂停多余的 Agent（按启动时间排序，暂停最新的）
            let mut running_vec: Vec<_> = self.running_agents.iter().cloned().collect();
            running_vec.sort_by(|a, b| {
                let a_time = self.agents.get(a).map(|ag| ag.started_at).unwrap_or(0);
                let b_time = self.agents.get(b).map(|ag| ag.started_at).unwrap_or(0);
                b_time.cmp(&a_time) // 降序，最新的先暂停
            });

            let excess = self.running_count - new_limit;
            for agent_id in running_vec.into_iter().take(excess) {
                if let Some(agent) = self.agents.get_mut(&agent_id) {
                    if agent.status == AgentStatus::Running {
                        agent.status = AgentStatus::Paused;
                    }
                }
                self.running_agents.remove(&agent_id);
                self.agent_queue.push(agent_id);
            }
            
            self.running_count = new_limit;
        } else {
            // 如果新限制更大，尝试启动更多 Agent
            self.schedule_next_agent();
        }

        Ok(())
    }

    /// 获取所有等待中的 Agent
    pub fn get_queued_agents(&self) -> Vec<&String> {
        self.agent_queue.iter().collect()
    }

    /// 获取所有运行中的 Agent
    pub fn get_running_agents(&self) -> Vec<&String> {
        self.running_agents.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_manager_creation() {
        let manager = DaemonManager::new();
        
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
        assert!(manager.agents.is_empty());
        assert!(manager.completed_tasks.is_empty());
        assert!(manager.pending_tasks.is_empty());
        assert_eq!(manager.running_count, 0);
        assert_eq!(manager.max_concurrent, 5);
        assert!(manager.agent_queue.is_empty());
        assert!(manager.running_agents.is_empty());
    }

    #[test]
    fn test_daemon_manager_start() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "debug".to_string(),
            max_concurrent_agents: 3,
            workspace_dir: "/tmp".to_string(),
        };
        
        let result = manager.start(config);
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_manager_stop() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let result = manager.stop(true);
        
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
    }

    #[test]
    fn test_daemon_manager_spawn_agent() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let agent_id = manager.spawn_agent("initializer");
        
        assert!(agent_id.is_ok());
        assert!(!agent_id.as_ref().unwrap().is_empty());
        assert_eq!(manager.agents.len(), 1);
    }

    #[test]
    fn test_daemon_manager_pause_resume() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let pause_result = manager.pause();
        assert!(pause_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Paused);
        
        let resume_result = manager.resume();
        assert!(resume_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        
        assert_eq!(usage.cpu_percent, 0.0);
        assert_eq!(usage.memory_mb, 0);
        assert_eq!(usage.disk_io_read, 0);
        assert_eq!(usage.disk_io_write, 0);
    }

    #[test]
    fn test_concurrency_config_initialization() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 0);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_can_spawn_agent_when_slots_available() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        assert!(manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 2);
    }

    #[test]
    fn test_cannot_spawn_agent_when_slots_full() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        assert!(!manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 0);
    }

    #[test]
    fn test_agent_queuing_when_concurrent_limit_reached() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        let agent3 = manager.spawn_agent("coding").unwrap();
        let should_start = manager.try_start_agent(&agent3);
        
        assert!(!should_start);
        assert_eq!(manager.agent_queue.len(), 1);
        assert!(manager.agent_queue.contains(&agent3));
    }

    #[test]
    fn test_auto_schedule_next_agent_on_completion() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 1);
        
        manager.stop_agent(&agent1).unwrap();
        
        assert_eq!(manager.running_count, 2);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_concurrency_stats() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        let stats = manager.get_concurrency_stats();
        
        assert_eq!(stats.running_count, 2);
        assert_eq!(stats.max_concurrent, 4);
        assert_eq!(stats.queued_count, 0);
        assert_eq!(stats.available_slots, 2);
        assert!((stats.utilization - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_adjust_max_concurrent_increase() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        manager.adjust_max_concurrent(4).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 3);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_adjust_max_concurrent_decrease() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        for _ in 0..4 {
            let agent_id = manager.spawn_agent("coding").unwrap();
            manager.try_start_agent(&agent_id);
        }
        
        assert_eq!(manager.running_count, 4);
        
        manager.adjust_max_concurrent(2).unwrap();
        
        assert_eq!(manager.max_concurrent, 2);
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 2);
    }

    #[test]
    fn test_get_running_and_queued_agents() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        let running = manager.get_running_agents();
        let queued = manager.get_queued_agents();
        
        assert_eq!(running.len(), 2);
        assert_eq!(queued.len(), 1);
    }

    #[test]
    fn test_adjust_max_concurrent_zero_error() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let adjust_result = manager.adjust_max_concurrent(0);
        assert!(adjust_result.is_err());
        assert_eq!(adjust_result.unwrap_err(), "max_concurrent must be greater than 0");
    }
}

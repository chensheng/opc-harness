//! Daemon Manager Core Implementation
//! 
//! DaemonManager 核心业务逻辑实现

use std::collections::{HashMap, HashSet};

use super::daemon_types::*;
use crate::agent::types::AgentStatus;

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
    pub(super) running_count: usize,                    // 当前运行中的 Agent 数量
    pub(super) max_concurrent: usize,                   // 最大并发数 (从 config 同步)
    pub(super) agent_queue: Vec<String>,                // 等待中的 Agent ID 队列
    running_agents: HashSet<String>,         // 正在运行的 Agent ID 集合
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

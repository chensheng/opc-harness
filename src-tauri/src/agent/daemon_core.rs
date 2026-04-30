//! Daemon Manager Core Implementation
//! 
//! DaemonManager 核心业务逻辑实现

use std::collections::{HashMap, HashSet};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    // ========== 进程管理 ==========
    /// 存储活跃的 Child 进程句柄（使用 Arc<Mutex> 以支持异步访问）
    child_processes: HashMap<String, Arc<Mutex<Option<std::process::Child>>>>,
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
            // ========== 进程管理初始化 ==========
            child_processes: HashMap::new(),
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
        
        log::info!("Daemon started with max_concurrent_agents: {}", self.max_concurrent);
        
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
    pub fn spawn_agent(&mut self, agent_type: &str, project_path: &str) -> Result<String, String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        let agent_id = format!("{}-{}", agent_type, uuid::Uuid::new_v4());
        
        // 根据 Agent 类型选择对应的 CLI 工具
        // 在测试模式下使用简单命令，生产模式使用实际的 AI CLI
        let cli_command = if std::env::var("HARNESS_TEST_MODE").is_ok() {
            // 测试模式：使用 echo 或 ping 等系统自带命令
            if cfg!(windows) {
                "ping"
            } else {
                "echo"
            }
        } else {
            // 生产模式：使用 Kimi CLI（默认）
            match agent_type {
                "initializer" | "coding" | "mr_creation" => "kimi",
                _ => return Err(format!("Unknown agent type: {}", agent_type)),
            }
        };

        log::info!("Spawning {} agent with CLI: {}", agent_type, cli_command);

        // 构建命令参数
        let child_result = if std::env::var("HARNESS_TEST_MODE").is_ok() {
            // 测试模式：使用不会立即退出的命令
            if cfg!(windows) {
                // Windows: ping localhost 持续运行
                Command::new(cli_command)
                    .args(&["-n", "9999", "127.0.0.1"])
                    .current_dir(project_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
            } else {
                // Unix: sleep 长时间
                Command::new("sleep")
                    .arg("999999")
                    .current_dir(project_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
            }
        } else {
            // 生产模式：实际调用 AI CLI
            // TODO: 从配置或数据库中获取 Story 上下文
            let ai_config = AICLIConfig {
                command: cli_command.to_string(),
                working_dir: project_path.to_string(),
                story_id: None, // TODO: 从调用方传入
                story_title: None,
                acceptance_criteria: None,
                agent_type: agent_type.to_string(),
                extra_args: vec![],
            };
            
            let args = ai_config.build_args();
            
            log::info!("[Daemon] Building CLI command: {} {:?}", cli_command, args);
            
            Command::new(cli_command)
                .args(&args)
                .current_dir(project_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
        };

        match child_result {
            Ok(child) => {
                let pid = child.id();
                log::info!("Agent {} spawned with PID: {}", agent_id, pid);

                // 存储进程句柄
                let child_arc = Arc::new(Mutex::new(Some(child)));
                self.child_processes.insert(agent_id.clone(), child_arc);

                // 创建 Agent 信息
                let agent_info = AgentProcessInfo {
                    agent_id: agent_id.clone(),
                    agent_type: agent_type.to_string(),
                    pid: Some(pid),
                    status: AgentStatus::Running,
                    started_at: chrono::Utc::now().timestamp(),
                    resource_usage: ResourceUsage::default(),
                };

                self.agents.insert(agent_id.clone(), agent_info);
                self.pending_tasks.push(agent_id.clone());
                
                Ok(agent_id)
            }
            Err(e) => {
                let error_msg = format!("Failed to spawn {} agent: {}. Please ensure {} is installed and in PATH.", 
                    agent_type, e, cli_command);
                log::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// 终止指定 Agent
    pub fn kill_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // 终止进程
        if let Some(child_arc) = self.child_processes.remove(agent_id) {
            let mut child_guard = futures::executor::block_on(child_arc.lock());
            if let Some(mut child) = child_guard.take() {
                match child.kill() {
                    Ok(_) => log::info!("Agent {} (PID: {:?}) killed successfully", agent_id, child.id()),
                    Err(e) => log::warn!("Failed to kill agent {}: {}", agent_id, e),
                }
            }
        }

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
        let sys = sysinfo::System::new_all();
        
        for (_agent_id, agent) in self.agents.iter_mut() {
            if let Some(pid) = agent.pid {
                // 查找对应的进程
                if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
                    agent.resource_usage = ResourceUsage {
                        cpu_percent: process.cpu_usage(),
                        memory_mb: (process.memory() / 1024 / 1024) as usize,
                        disk_io_read: 0,  // TODO: 需要更精确的磁盘 IO 统计
                        disk_io_write: 0,
                        network_rx: 0,    // TODO: 需要网络统计
                        network_tx: 0,
                    };
                }
            }
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
    pub fn try_start_agent(&mut self, agent_id: &str, project_path: &str) -> bool {
        // 检查是否已经在运行
        if self.running_agents.contains(agent_id) {
            return true;
        }

        // 检查是否有可用槽位
        if self.can_spawn_agent() {
            // 从 pending_tasks 中获取 agent_type
            let agent_type = if let Some(agent) = self.agents.get(agent_id) {
                agent.agent_type.clone()
            } else {
                log::error!("Agent {} not found in agents map", agent_id);
                return false;
            };

            // 真正启动进程
            match self.spawn_agent(&agent_type, project_path) {
                Ok(_) => {
                    self.running_agents.insert(agent_id.to_string());
                    self.running_count += 1;
                    
                    // 更新 Agent 状态为 Running
                    if let Some(agent) = self.agents.get_mut(agent_id) {
                        agent.status = AgentStatus::Running;
                    }
                    
                    log::info!("Agent {} started successfully", agent_id);
                    true
                }
                Err(e) => {
                    log::error!("Failed to start agent {}: {}", agent_id, e);
                    // 更新状态为 Failed
                    if let Some(agent) = self.agents.get_mut(agent_id) {
                        agent.status = AgentStatus::Failed(e.clone());
                    }
                    false
                }
            }
        } else {
            // 加入等待队列
            if !self.agent_queue.contains(&agent_id.to_string()) {
                self.agent_queue.push(agent_id.to_string());
                
                // 更新 Agent 状态为 Idle (等待中)
                if let Some(agent) = self.agents.get_mut(agent_id) {
                    agent.status = AgentStatus::Idle;
                }
                
                log::info!("Agent {} queued (no available slots)", agent_id);
            }
            false
        }
    }

    /// 停止 Agent 并释放槽位
    pub fn stop_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // 终止进程
        self.kill_agent(agent_id)?;

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

        // 标记任务完成
        self.mark_task_completed(agent_id);

        // ========== VC-013: 调度队列中的下一个 Agent ==========
        self.schedule_next_agent();

        log::info!("Agent {} stopped and slot released", agent_id);
        Ok(())
    }

    /// 调度队列中的下一个 Agent
    fn schedule_next_agent(&mut self) {
        // 注意：这里需要 project_path，从 config 中获取
        let project_path = if let Some(config) = &self.config {
            config.project_path.clone()
        } else {
            log::error!("Daemon config not set, cannot schedule next agent");
            return;
        };

        // 检查是否有可用槽位和等待中的 Agent
        while self.can_spawn_agent() && !self.agent_queue.is_empty() {
            if let Some(next_agent_id) = self.agent_queue.first().cloned() {
                self.agent_queue.remove(0);
                
                // 启动该 Agent
                let agent_type = if let Some(agent) = self.agents.get(&next_agent_id) {
                    agent.agent_type.clone()
                } else {
                    log::error!("Queued agent {} not found", next_agent_id);
                    continue;
                };

                match self.spawn_agent(&agent_type, &project_path) {
                    Ok(_) => {
                        self.running_agents.insert(next_agent_id.clone());
                        self.running_count += 1;
                        
                        // 更新 Agent 状态
                        if let Some(agent) = self.agents.get_mut(&next_agent_id) {
                            agent.status = AgentStatus::Running;
                        }
                        
                        log::info!("Scheduled agent {} started", next_agent_id);
                    }
                    Err(e) => {
                        log::error!("Failed to start scheduled agent {}: {}", next_agent_id, e);
                        if let Some(agent) = self.agents.get_mut(&next_agent_id) {
                            agent.status = AgentStatus::Failed(e);
                        }
                    }
                }
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

    /// 检查并更新已完成或失败的 Agent 进程
    /// 返回已完成的 Agent ID 列表
    pub fn check_completed_agents(&mut self) -> Vec<String> {
        let mut completed_ids = Vec::new();
        
        for (agent_id, child_arc) in &self.child_processes {
            let mut child_guard = futures::executor::block_on(child_arc.lock());
            
            if let Some(ref mut child) = *child_guard {
                // 尝试检查进程是否已结束
                match child.try_wait() {
                    Ok(Some(status)) => {
                        // 进程已结束
                        log::info!(
                            "[Daemon] Agent {} exited with status: {:?}",
                            agent_id,
                            status
                        );
                        
                        // 更新 Agent 状态
                        if let Some(agent) = self.agents.get_mut(agent_id) {
                            if status.success() {
                                agent.status = AgentStatus::Completed;
                                log::info!("[Daemon] Agent {} completed successfully", agent_id);
                            } else {
                                agent.status = AgentStatus::Failed("Process exited with error".to_string());
                                log::warn!("[Daemon] Agent {} failed with status: {:?}", agent_id, status);
                            }
                        }
                        
                        completed_ids.push(agent_id.clone());
                        
                        // 从 running_agents 中移除
                        self.running_agents.remove(agent_id);
                        self.running_count = self.running_count.saturating_sub(1);
                        
                        // 清理进程句柄
                        *child_guard = None;
                    }
                    Ok(None) => {
                        // 进程仍在运行
                    }
                    Err(e) => {
                        log::error!("[Daemon] Failed to check agent {} status: {}", agent_id, e);
                    }
                }
            }
        }
        
        // 清理已完成的进程句柄
        for agent_id in &completed_ids {
            self.child_processes.remove(agent_id);
        }
        
        if !completed_ids.is_empty() {
            log::info!("[Daemon] Found {} completed agents: {:?}", completed_ids.len(), completed_ids);
        }
        
        completed_ids
    }

    /// 启动后台监控任务，定期检查 Agent 状态
    /// 注意：此方法需要在 tokio runtime 中调用
    pub async fn start_monitoring(
        &self,
        check_interval_secs: u64,
    ) -> Result<(), String> {
        use tokio::time::{sleep, Duration};
        
        log::info!(
            "[Daemon] Starting agent monitoring task with {}s interval",
            check_interval_secs
        );
        
        // 克隆必要的引用以供异步任务使用
        // 注意：实际使用时需要通过 Arc<Mutex<DaemonManager>> 共享状态
        // 这里仅提供接口设计，具体实现在 AgentManager 层
        
        Ok(())
    }

    /// 在指定 Worktree 中生成新的 Agent 进程 (深度集成版本)
    pub fn spawn_agent_in_worktree(
        &mut self, 
        agent_type: &str, 
        worktree_path: &str,
        story_id: &str,
    ) -> Result<String, String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        // 验证 Worktree 路径是否存在
        if !std::path::Path::new(worktree_path).exists() {
            return Err(format!("Worktree path does not exist: {}", worktree_path));
        }

        let agent_id = format!("{}-{}", agent_type, uuid::Uuid::new_v4());
        
        // 根据 Agent 类型选择对应的 CLI 工具
        let cli_command = if std::env::var("HARNESS_TEST_MODE").is_ok() {
            if cfg!(windows) {
                "ping"
            } else {
                "echo"
            }
        } else {
            match agent_type {
                "initializer" | "coding" | "mr_creation" => "kimi",
                _ => return Err(format!("Unknown agent type: {}", agent_type)),
            }
        };

        log::info!(
            "[Daemon] Spawning {} agent in worktree {} for story {}",
            agent_type,
            worktree_path,
            story_id
        );

        // 构建命令参数 - 在 Worktree 中执行
        let child_result = if std::env::var("HARNESS_TEST_MODE").is_ok() {
            if cfg!(windows) {
                Command::new(cli_command)
                    .args(&["-n", "9999", "127.0.0.1"])
                    .current_dir(worktree_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
            } else {
                Command::new("sleep")
                    .arg("999999")
                    .current_dir(worktree_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
            }
        } else {
            // 生产模式: 从数据库查询 Story 详细信息并构建 AICLIConfig
            let story_info = self.get_story_context(story_id)?;
            
            let ai_config = AICLIConfig {
                command: cli_command.to_string(),
                working_dir: worktree_path.to_string(),
                story_id: Some(story_id.to_string()),
                story_title: story_info.title,
                acceptance_criteria: story_info.acceptance_criteria,
                agent_type: agent_type.to_string(),
                extra_args: vec![
                    "--worktree".to_string(),
                    worktree_path.to_string(),
                ],
            };
            
            let args = ai_config.build_args();
            
            log::info!("[Daemon] Building CLI command for worktree with full context: {} {:?}", cli_command, args);
            
            Command::new(cli_command)
                .args(&args)
                .current_dir(worktree_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
        };

        match child_result {
            Ok(child) => {
                let pid = child.id();
                log::info!(
                    "[Daemon] Agent {} spawned in worktree {} with PID: {}",
                    agent_id,
                    worktree_path,
                    pid
                );

                // 存储进程句柄
                let child_arc = Arc::new(Mutex::new(Some(child)));
                self.child_processes.insert(agent_id.clone(), child_arc);

                // 创建 Agent 信息
                let agent_info = AgentProcessInfo {
                    agent_id: agent_id.clone(),
                    agent_type: agent_type.to_string(),
                    pid: Some(pid),
                    status: AgentStatus::Running,
                    started_at: chrono::Utc::now().timestamp(),
                    resource_usage: ResourceUsage::default(),
                };

                self.agents.insert(agent_id.clone(), agent_info);
                self.pending_tasks.push(agent_id.clone());
                
                log::info!(
                    "[Daemon] Agent {} info stored with worktree: {}, story: {}",
                    agent_id,
                    worktree_path,
                    story_id
                );
                
                Ok(agent_id)
            }
            Err(e) => {
                let error_msg = format!(
                    "Failed to spawn {} agent in worktree {}: {}. Please ensure {} is installed.", 
                    agent_type, worktree_path, e, cli_command
                );
                log::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// 从数据库获取 Story 的上下文信息
    fn get_story_context(&self, story_id: &str) -> Result<StoryContext, String> {
        use crate::db;
        
        let conn = db::get_connection().map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let story = db::get_user_story_by_id(&conn, story_id)
            .map_err(|e| format!("Failed to query story {}: {}", story_id, e))?;
        
        match story {
            Some(s) => {
                log::info!(
                    "[Daemon] Retrieved story context for {}: title='{}', acceptance_criteria_length={}",
                    story_id,
                    s.title,
                    s.acceptance_criteria.len()
                );
                
                Ok(StoryContext {
                    title: Some(s.title),
                    // acceptance_criteria 是 String 类型,如果为空字符串则返回 None
                    acceptance_criteria: if s.acceptance_criteria.is_empty() {
                        None
                    } else {
                        Some(s.acceptance_criteria)
                    },
                })
            }
            None => {
                log::warn!("[Daemon] Story {} not found in database, using empty context", story_id);
                Ok(StoryContext {
                    title: None,
                    acceptance_criteria: None,
                })
            }
        }
    }

}

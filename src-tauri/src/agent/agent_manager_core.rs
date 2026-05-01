//! Agent Manager 核心逻辑
//! 
//! 统一管理所有 Agent 的生命周期、通信和资源调度

use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::agent::daemon::{DaemonManager, DaemonConfig, DaemonStatus};
use crate::agent::websocket_manager::WebSocketManager;
use crate::agent::agent_stdio::StdioChannelManager;
use crate::agent::types::{AgentType, AgentStatus, AgentConfig};
use crate::agent::branch_manager::{BranchManager, BranchManagerConfig};
use crate::agent::agent_manager_types::{AgentHandle, AgentManagerStats};
use crate::agent::agent_manager_persistence;
use crate::agent::agent_loop::AgentLoop;
use crate::agent::decentralized::node::DecentralizedAgentNode;
use crate::agent::decentralized::event_bus::SharedEventBus;
use crate::agent::decentralized::distributed_lock::SharedLockManager;
use crate::agent::agent_worker::AgentWorker;

/// Agent Manager
/// 
/// 统一管理所有 Agent 的生命周期、通信和资源调度
pub struct AgentManager {
    /// Tauri 应用句柄
    pub app_handle: AppHandle,
    /// 所有 Agent 句柄
    pub agents: Arc<RwLock<HashMap<String, AgentHandle>>>,
    /// Daemon 管理器
    pub daemon: Arc<RwLock<DaemonManager>>,
    /// WebSocket 管理器
    pub websocket: Arc<RwLock<WebSocketManager>>,
    /// Stdio 通道管理器
    pub stdio: Arc<RwLock<StdioChannelManager>>,
    /// 分支管理器
    pub branch_manager: Arc<RwLock<Option<BranchManager>>>,
    /// 守护进程配置（使用 RwLock 包装以支持内部可变性）
    pub daemon_config: Arc<RwLock<Option<DaemonConfig>>>,
    /// 统计信息
    pub stats: Arc<RwLock<AgentManagerStats>>,
    /// Agent Loop 自动化执行引擎
    pub agent_loop: Arc<RwLock<Option<AgentLoop>>>,
    /// 去中心化 Agent Nodes (单机多实例 - 旧架构)
    pub decentralized_nodes: HashMap<String, Arc<RwLock<DecentralizedAgentNode>>>,
    /// 完全去中心化 Agent Workers (新架构 - 每个 Worker 拥有独立 Loop)
    pub agent_workers: HashMap<String, Arc<RwLock<AgentWorker>>>,
    /// 项目路径 (用于 Worktree Manager)
    pub project_path: Option<String>,
    /// 共享事件总线 (所有 Node 实例共享 - 旧架构)
    pub shared_event_bus: Option<Arc<SharedEventBus>>,
    /// 共享锁管理器 (所有 Node 实例共享 - 旧架构)
    pub shared_lock_manager: Option<Arc<SharedLockManager>>,
}

impl AgentManager {
    /// 创建新的 Agent Manager
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle: app_handle.clone(),
            agents: Arc::new(RwLock::new(HashMap::new())),
            daemon: Arc::new(RwLock::new(DaemonManager::new())),
            websocket: Arc::new(RwLock::new(WebSocketManager::new(app_handle))),
            stdio: Arc::new(RwLock::new(StdioChannelManager::new())),
            branch_manager: Arc::new(RwLock::new(None)),
            daemon_config: Arc::new(RwLock::new(None)),
            stats: Arc::new(RwLock::new(AgentManagerStats::default())),
            agent_loop: Arc::new(RwLock::new(None)),
            decentralized_nodes: HashMap::new(),
            agent_workers: HashMap::new(),
            project_path: None,
            shared_event_bus: None,
            shared_lock_manager: None,
        }
    }

    /// 初始化 Agent Manager（启动 Daemon 并恢复持久化的 Sessions）
    pub async fn initialize(&self, config: DaemonConfig) -> Result<(), String> {
        // 保存配置
        *self.daemon_config.write().await = Some(config.clone());

        // 启动 Daemon Manager
        {
            let mut daemon = self.daemon.write().await;
            daemon.start(config)?;
        }

        // 初始化 Agent Loop（传入 Daemon Manager 引用）
        {
            let daemon_clone = self.daemon.clone();
            let mut agent_loop_guard = self.agent_loop.write().await;
            let mut loop_instance = AgentLoop::new(daemon_clone);
            
            // 注意: Worktree 管理器将在每次执行 Agent Loop 时根据具体项目动态设置
            // 这里不预先初始化,因为不同项目有不同的工作目录
            log::info!("[AgentManager] Agent Loop initialized (Worktree manager will be set per-project during execution)");
            
            *agent_loop_guard = Some(loop_instance);
        }

        // 恢复持久化的 Agent Sessions
        if let Err(e) = self.restore_sessions().await {
            log::warn!("Failed to restore agent sessions: {}", e);
        }

        // 启动后台监控任务 (每 10 秒检查一次)
        if let Err(e) = self.start_agent_monitoring(10).await {
            log::error!("[AgentManager] Failed to start agent monitoring: {}", e);
        } else {
            log::info!("[AgentManager] Agent monitoring task started");
        }

        log::info!("Agent Manager initialized and Daemon started");
        Ok(())
    }

    /// 启动后台监控任务，定期检查 Agent 状态并清理已完成的 Worktree
    pub async fn start_agent_monitoring(&self, check_interval_secs: u64) -> Result<(), String> {
        use tokio::time::{sleep, Duration};
        
        let daemon_clone = self.daemon.clone();
        let agent_loop_clone = self.agent_loop.clone();
        
        log::info!(
            "[AgentManager] Starting agent monitoring task with {}s interval",
            check_interval_secs
        );
        
        // 在后台启动异步任务
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(check_interval_secs)).await;
                
                // 检查已完成的 Agent
                let completed_agents = {
                    let mut daemon = daemon_clone.write().await;
                    daemon.check_completed_agents()
                };
                
                if !completed_agents.is_empty() {
                    log::info!(
                        "[AgentManager] Found {} completed agents, cleaning up worktrees",
                        completed_agents.len()
                    );
                    
                    // 清理已完成 Agent 的 Worktree
                    if let Some(loop_guard) = agent_loop_clone.read().await.as_ref() {
                        match loop_guard.cleanup_completed_worktrees(&completed_agents).await {
                            Ok(cleaned_count) => {
                                log::info!(
                                    "[AgentManager] Successfully cleaned up {} worktrees",
                                    cleaned_count
                                );
                            }
                            Err(e) => {
                                log::error!("[AgentManager] Failed to cleanup worktrees: {}", e);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    /// 恢复持久化的 Sessions (VC-005)
    async fn restore_sessions(&self) -> Result<(), String> {
        agent_manager_persistence::restore_sessions(
            &self.app_handle,
            &self.agents,
            &self.stdio,
            &self.websocket,
            &self.stats,
        ).await
    }

    /// 创建新的 Agent
    pub async fn create_agent(
        &self,
        agent_type: AgentType,
        session_id: String,
        project_id: String,
        project_path: String,
        name: Option<String>,
        agents_md_content: Option<String>,
    ) -> Result<String, String> {
        log::info!("[AgentManager::create_agent] Creating agent: type={:?}, session_id={}, project_id={}, project_path={}, name={:?}, has_agents_content={}", 
            agent_type, session_id, project_id, project_path, name, agents_md_content.is_some());
        
        // 保存 agent_type 的引用，避免移动
        let agent_type_clone = agent_type.clone();
        
        // 创建 Agent 句柄（传入 project_id、name 和 agents_md_content）
        let mut handle = AgentHandle::new(agent_type, session_id.clone(), project_id.clone(), name, agents_md_content);
        
        log::info!("[AgentManager::create_agent] Agent handle created: agent_id={}", handle.agent_id);
        
        // 创建 Stdio 通道 - 直接传递 AgentConfig
        let agent_config = AgentConfig {
            agent_id: handle.agent_id.clone(),
            agent_type: agent_type_clone.clone(),
            phase: handle.phase.clone(),
            status: handle.status.clone(),
            project_path: project_path.clone(),
            session_id: handle.session_id.clone(),
            ai_config: None,
            metadata: None,
        };

        log::info!("[AgentManager::create_agent] Creating stdio channel...");
        let mut stdio_manager = self.stdio.write().await;
        let channel_result = stdio_manager.create_channel(agent_config);
        drop(stdio_manager);
        
        let channel_id = match channel_result {
            Ok(id) => {
                log::info!("[AgentManager::create_agent] Stdio channel created: {}", id);
                id
            }
            Err(e) => {
                log::error!("[AgentManager::create_agent] Failed to create stdio channel: {}", e);
                return Err(format!("Failed to create stdio channel: {}", e));
            }
        };

        // 更新句柄
        handle.set_stdio_channel(channel_id);

        // 持久化到数据库 (VC-005)
        log::info!("[AgentManager::create_agent] Persisting agent to database...");
        if let Err(e) = self.persist_agent(&handle).await {
            log::warn!("[AgentManager::create_agent] Failed to persist agent {}: {}", handle.agent_id, e);
        } else {
            log::info!("[AgentManager::create_agent] Agent persisted successfully");
        }

        // 添加到管理器
        let agent_id = handle.agent_id.clone();
        log::info!("[AgentManager::create_agent] Adding agent to manager: {}", agent_id);
        {
            let mut agents = self.agents.write().await;
            agents.insert(agent_id.clone(), handle);
        } // ✅ 在这里释放 agents 锁

        // 更新统计（在释放锁之后调用，避免死锁）
        log::info!("[AgentManager::create_agent] Updating stats...");
        self.update_stats().await;

        log::info!("[AgentManager::create_agent] Agent created successfully: {}", agent_id);
        Ok(agent_id)
    }

    /// 持久化 Agent 到数据库 (VC-005)
    async fn persist_agent(&self, handle: &AgentHandle) -> Result<(), String> {
        agent_manager_persistence::persist_agent(&self.app_handle, handle).await
    }

    /// 更新 Agent 状态并持久化 (VC-005)
    async fn update_and_persist_agent(&self, agent_id: &str, status: AgentStatus) -> Result<(), String> {
        agent_manager_persistence::update_and_persist_agent(
            &self.app_handle,
            &self.agents,
            agent_id,
            status,
        ).await
    }

    /// 启动 Agent
    pub async fn start_agent(&self, agent_id: &str) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        
        let handle = agents.get_mut(agent_id)
            .ok_or_else(|| format!("Agent {} not found", agent_id))?;

        if handle.status != AgentStatus::Idle {
            return Err(format!(
                "Agent {} is not in Idle state. Current state: {:?}",
                agent_id, handle.status
            ));
        }

        // 更新状态为 Running（不立即持久化，等待 update_and_persist_agent）
        handle.update_status(AgentStatus::Running);
        let agent_type = handle.agent_type.clone();
        let session_id = handle.session_id.clone();
        drop(agents);

        // 持久化状态变更 (VC-005)
        if let Err(e) = self.update_and_persist_agent(agent_id, AgentStatus::Running).await {
            log::warn!("Failed to persist agent status: {}", e);
        }

        // 通知 Daemon 启动 Agent
        {
            let mut daemon = self.daemon.write().await;
            
            // 获取 project_path（从 agent handle 中）
            let project_path = {
                let agents_read = self.agents.read().await;
                if let Some(handle) = agents_read.get(agent_id) {
                    handle.project_id.clone()  // 使用 project_id 作为 project_path
                } else {
                    return Err(format!("Agent {} not found", agent_id));
                }
            };
            
            // 尝试启动 Agent（受并发控制）
            let started = daemon.try_start_agent(agent_id, &project_path);
            
            if !started {
                log::warn!("Agent {} queued due to concurrency limit", agent_id);
                // 更新状态为 Idle（等待中）
                self.update_and_persist_agent(agent_id, AgentStatus::Idle).await?;
                
                return Err(format!(
                    "Agent {} is queued. Current running: {}/{}, max concurrent: {}",
                    agent_id,
                    daemon.running_count,
                    daemon.max_concurrent,
                    daemon.max_concurrent
                ));
            }
            
            drop(daemon);
            log::info!("Daemon started Agent {}", agent_id);
        }

        // 通过 WebSocket 发送状态更新
        self.websocket.read().await.send_status(
            &session_id,
            "running",
            Some(&format!("Agent {:?} started", agent_type)),
        ).await?;

        log::info!("Started Agent {}", agent_id);
        Ok(())
    }

    /// 停止 Agent
    pub async fn stop_agent(&self, agent_id: &str, graceful: bool) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        
        let handle = agents.get_mut(agent_id)
            .ok_or_else(|| format!("Agent {} not found", agent_id))?;

        if matches!(handle.status, AgentStatus::Completed | AgentStatus::Failed(_)) {
            return Err(format!(
                "Agent {} has already completed or failed",
                agent_id
            ));
        }

        // 更新状态（不立即持久化）
        let new_status = if graceful {
            AgentStatus::Idle
        } else {
            AgentStatus::Idle
        };
        
        handle.update_status(new_status.clone());
        let session_id = handle.session_id.clone();
        drop(agents);

        // 持久化状态变更 (VC-005)
        if let Err(e) = self.update_and_persist_agent(agent_id, new_status).await {
            log::warn!("Failed to persist agent status: {}", e);
        }

        // 通知 Daemon 停止 Agent
        {
            let mut daemon = self.daemon.write().await;
            
            // 实际调用 daemon.stop_agent 终止进程
            if let Err(e) = daemon.stop_agent(agent_id) {
                log::error!("Failed to stop agent {}: {}", agent_id, e);
                return Err(format!("Failed to stop agent: {}", e));
            }
            
            drop(daemon);
            log::info!("Daemon stopped Agent {}", agent_id);
        }

        // 通过 WebSocket 发送状态更新
        self.websocket.read().await.send_status(
            &session_id,
            if graceful { "paused" } else { "stopped" },
            Some(&format!("Agent {} stopped", agent_id)),
        ).await?;

        log::info!("Stopped Agent {} (graceful: {})", agent_id, graceful);
        Ok(())
    }

    /// 获取 Agent 状态
    pub async fn get_agent_status(&self, agent_id: &str) -> Result<AgentHandle, String> {
        let agents = self.agents.read().await;
        agents.get(agent_id)
            .cloned()
            .ok_or_else(|| format!("Agent {} not found", agent_id))
    }

    /// 获取所有 Agent
    pub async fn get_all_agents(&self) -> Vec<AgentHandle> {
        let agents = self.agents.read().await;
        agents.values().cloned().collect()
    }

    /// 获取指定 Session 的所有 Agent
    pub async fn get_agents_by_session(&self, session_id: &str) -> Vec<AgentHandle> {
        let agents = self.agents.read().await;
        agents.values()
            .filter(|h| h.session_id == session_id)
            .cloned()
            .collect()
    }

    /// 获取指定类型的 Agent
    pub async fn get_agents_by_type(&self, agent_type: &AgentType) -> Vec<AgentHandle> {
        let agents = self.agents.read().await;
        agents.values()
            .filter(|h| h.agent_type == *agent_type)
            .cloned()
            .collect()
    }

    /// 更新统计信息
    pub async fn update_stats(&self) {
        agent_manager_persistence::update_stats(
            &self.agents,
            &self.stdio,
            &self.websocket,
            &self.stats,
        ).await
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> AgentManagerStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// 获取 Daemon 状态
    pub async fn get_daemon_status(&self) -> DaemonStatus {
        let daemon = self.daemon.read().await;
        let status = daemon.get_status().clone();
        drop(daemon);
        status
    }

    /// 发送日志消息
    pub async fn send_log(
        &self,
        session_id: &str,
        level: &str,
        message: &str,
        source: Option<&str>,
    ) -> Result<(), String> {
        self.websocket.read().await.send_log(&session_id.to_string(), level, message, source).await
    }

    /// 发送进度更新
    pub async fn send_progress(
        &self,
        session_id: &str,
        phase: &str,
        current: u32,
        total: u32,
        description: Option<&str>,
    ) -> Result<(), String> {
        self.websocket.read().await.send_progress(&session_id.to_string(), phase, current, total, description).await
    }

    // ========================================================================
    // Branch Manager Methods (VC-015)
    // ========================================================================

    /// 获取或创建 BranchManager（异步版本）
    pub async fn get_or_create_branch_manager(&self) -> tokio::sync::RwLockWriteGuard<'_, Option<BranchManager>> {
        // 检查是否已存在
        {
            let bm = self.branch_manager.read().await;
            if bm.is_some() {
                drop(bm);
                return self.branch_manager.write().await;
            }
        }
        
        // 创建新的 BranchManager
        let mut bm = self.branch_manager.write().await;
        if bm.is_none() {
            *bm = Some(BranchManager::new(BranchManagerConfig {
                project_path: ".".to_string(),
                default_base_branch: "main".to_string(),
                name_prefix: None,
            }));
        }
        bm
    }

    /// 获取 BranchManager（只读）
    pub async fn get_branch_manager(&self) -> tokio::sync::RwLockReadGuard<'_, Option<BranchManager>> {
        self.branch_manager.read().await
    }

    // ========================================================================
    // Agent Loop 控制方法 (P0: 自动触发引擎)
    // ========================================================================

    /// 启动 Agent Loop 持续运行
    pub async fn start_agent_loop(&self, project_id: &str, interval_secs: u64) -> Result<(), String> {
        let agent_loop_guard = self.agent_loop.read().await;
        
        if let Some(_loop_mgr) = agent_loop_guard.as_ref() {
            // 克隆必要的参数用于异步任务
            let project_id_clone = project_id.to_string();
            
            // 在后台启动持续循环
            let loop_clone = self.agent_loop.clone();
            tokio::spawn(async move {
                if let Some(loop_mgr) = loop_clone.write().await.as_mut() {
                    loop_mgr.start_continuous(&project_id_clone, interval_secs).await;
                }
            });
            
            log::info!("Agent Loop started for project {} with {}s interval", project_id, interval_secs);
            Ok(())
        } else {
            Err("Agent Loop not initialized".to_string())
        }
    }

    /// 执行一次 Agent Loop（单次触发）
    pub async fn execute_agent_loop_once(&self, project_id: &str) -> Result<usize, String> {
        let mut agent_loop_guard = self.agent_loop.write().await;
        
        if let Some(loop_mgr) = agent_loop_guard.as_mut() {
            let count = loop_mgr.execute_once(project_id).await?;
            log::info!("Agent Loop executed once for project {}, started {} agents", project_id, count);
            Ok(count)
        } else {
            Err("Agent Loop not initialized".to_string())
        }
    }

    /// 停止 Agent Loop
    pub async fn stop_agent_loop(&self) -> Result<(), String> {
        let mut agent_loop_guard = self.agent_loop.write().await;
        
        if let Some(loop_mgr) = agent_loop_guard.as_mut() {
            loop_mgr.stop();
            log::info!("Agent Loop stopped");
            Ok(())
        } else {
            Err("Agent Loop not initialized".to_string())
        }
    }

    /// 检查 Agent Loop 是否正在运行
    pub async fn is_agent_loop_running(&self) -> bool {
        let agent_loop_guard = self.agent_loop.read().await;
        
        if let Some(loop_mgr) = agent_loop_guard.as_ref() {
            loop_mgr.is_running
        } else {
            false
        }
    }
}
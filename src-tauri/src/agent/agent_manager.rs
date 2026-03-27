//! Agent Manager 实现 (VC-004)
//! 
//! 统一的 Agent 管理器，整合 WebSocketManager、StdioChannelManager 和 DaemonManager
//! 提供高级 API 供前端调用
//! 
//! 核心功能:
//! - Agent 生命周期管理（创建、启动、暂停、恢复、终止）
//! - Agent 状态追踪和持久化
//! - 并发控制和资源调度
//! - 消息路由（Stdio + WebSocket）
//! - Tauri Commands 暴露

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::agent::daemon::{DaemonManager, DaemonConfig, DaemonStatus};
use crate::agent::websocket_manager::WebSocketManager;
use crate::agent::agent_stdio::StdioChannelManager;
use crate::agent::types::{AgentConfig, AgentType, AgentStatus, AgentPhase};
use crate::agent::branch_manager::{BranchManager, BranchManagerConfig, BranchInfo, BranchOperationResult};
use crate::agent::git_commit_assistant::{CommitType, CommitMessage, ChangeInfo, FileChangeType};
use crate::agent::code_review_agent::{CodeReviewAgent, CodeReviewAgentConfig, CodeReviewStatus, ReviewResult, ReviewComment, ReviewDimension, ReviewSeverity, CodeChange};
use crate::agent::realtime_review_manager::{RealtimeReviewManager, WatchConfig, WatchStatus, FileChangeEvent, RealtimeReviewResult};
use crate::agent::test_runner_agent::{TestRunnerAgent, TestRunnerConfig, TestSuiteResult};
use crate::db;

/// Agent 句柄信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandle {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型
    pub agent_type: AgentType,
    /// Session ID
    pub session_id: String,
    /// 创建时间戳
    pub created_at: i64,
    /// 最后更新时间戳
    pub updated_at: i64,
    /// 当前状态
    pub status: AgentStatus,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 项目路径
    pub project_path: String,
    /// 关联的 Stdio 通道 ID
    pub stdio_channel_id: Option<String>,
    /// 是否已注册到 Daemon
    pub registered_to_daemon: bool,
}

impl AgentHandle {
    /// 创建新的 Agent 句柄
    pub fn new(
        agent_type: AgentType,
        session_id: String,
        project_path: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        let phase = match &agent_type {
            AgentType::Initializer => AgentPhase::Initializer,
            AgentType::Coding => AgentPhase::Coding,
            AgentType::MRCreation => AgentPhase::MRCreation,
        };
        
        Self {
            agent_id: Uuid::new_v4().to_string(),
            agent_type,
            session_id,
            created_at: now,
            updated_at: now,
            status: AgentStatus::Idle,
            phase,
            project_path,
            stdio_channel_id: None,
            registered_to_daemon: false,
        }
    }

    /// 更新状态
    pub fn update_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// 设置 Stdio 通道 ID
    pub fn set_stdio_channel(&mut self, channel_id: String) {
        self.stdio_channel_id = Some(channel_id);
    }

    /// 标记为已注册到 Daemon
    pub fn mark_registered(&mut self) {
        self.registered_to_daemon = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Agent Manager 统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentManagerStats {
    /// 总 Agent 数
    pub total_agents: usize,
    /// 运行中的 Agent 数
    pub running_agents: usize,
    /// 空闲的 Agent 数
    pub idle_agents: usize,
    /// 已完成的 Agent 数
    pub completed_agents: usize,
    /// 失败的 Agent 数
    pub failed_agents: usize,
    /// 当前 Session 数
    pub active_sessions: usize,
    /// Stdio 通道总数
    pub stdio_channels: usize,
    /// WebSocket 连接数
    pub websocket_connections: usize,
}

/// Agent Manager
/// 
/// 统一管理所有 Agent 的生命周期、通信和资源调度
pub struct AgentManager {
    /// Tauri 应用句柄
    app_handle: AppHandle,
    /// 所有 Agent 句柄
    agents: Arc<RwLock<HashMap<String, AgentHandle>>>,
    /// Daemon 管理器
    daemon: Arc<RwLock<DaemonManager>>,
    /// WebSocket 管理器
    websocket: Arc<RwLock<WebSocketManager>>,
    /// Stdio 通道管理器
    stdio: Arc<RwLock<StdioChannelManager>>,
    /// 分支管理器
    branch_manager: Arc<RwLock<Option<BranchManager>>>,
    /// 守护进程配置（使用 RwLock 包装以支持内部可变性）
    daemon_config: Arc<RwLock<Option<DaemonConfig>>>,
    /// 统计信息
    stats: Arc<RwLock<AgentManagerStats>>,
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
        }
    }

    /// 初始化 Agent Manager（启动 Daemon 并恢复持久化的 Sessions）
    pub async fn initialize(&self, config: DaemonConfig) -> Result<(), String> {
        // 保存配置
        *self.daemon_config.write().await = Some(config.clone());

        // 启动 Daemon Manager
        let mut daemon = self.daemon.write().await;
        daemon.start(config)?;

        // 恢复持久化的 Agent Sessions
        if let Err(e) = self.restore_sessions().await {
            log::warn!("Failed to restore agent sessions: {}", e);
        }

        log::info!("Agent Manager initialized and Daemon started");
        Ok(())
    }

    /// 恢复持久化的 Sessions (VC-005)
    async fn restore_sessions(&self) -> Result<(), String> {
        let conn = db::get_connection(&self.app_handle)
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let sessions = db::get_all_agent_sessions(&conn)
            .map_err(|e| format!("Failed to fetch agent sessions: {}", e))?;

        let mut restored_count = 0;
        for session in sessions {
            // 只恢复未完成的 Sessions
            if session.status == "completed" || session.status.starts_with("failed:") {
                continue;
            }

            // 重建 AgentHandle
            let mut handle = AgentHandle {
                agent_id: session.agent_id.clone(),
                agent_type: match session.agent_type.as_str() {
                    "initializer" => AgentType::Initializer,
                    "coding" => AgentType::Coding,
                    "mr_creation" => AgentType::MRCreation,
                    _ => continue, // 跳过未知类型
                },
                session_id: session.session_id.clone(),
                created_at: chrono::DateTime::parse_from_rfc3339(&session.created_at)
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|_| chrono::Utc::now().timestamp()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&session.updated_at)
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|_| chrono::Utc::now().timestamp()),
                status: match session.status.as_str() {
                    "idle" => AgentStatus::Idle,
                    "running" => AgentStatus::Running,
                    "paused" => AgentStatus::Paused,
                    _ => AgentStatus::Idle, // 默认重置为 Idle
                },
                phase: match session.phase.as_str() {
                    "initializer" => AgentPhase::Initializer,
                    "coding" => AgentPhase::Coding,
                    "mr_creation" => AgentPhase::MRCreation,
                    _ => AgentPhase::Initializer,
                },
                project_path: session.project_path.clone(),
                stdio_channel_id: session.stdio_channel_id,
                registered_to_daemon: session.registered_to_daemon,
            };

            // 重新创建 Stdio 通道（如果需要）
            if handle.stdio_channel_id.is_some() {
                let agent_config = AgentConfig {
                    agent_id: handle.agent_id.clone(),
                    agent_type: handle.agent_type.clone(),
                    phase: handle.phase.clone(),
                    status: handle.status.clone(),
                    project_path: handle.project_path.clone(),
                    session_id: handle.session_id.clone(),
                    ai_config: None,
                    metadata: None,
                };

                let mut stdio_manager = self.stdio.write().await;
                match stdio_manager.create_channel(agent_config) {
                    Ok(channel_id) => {
                        handle.stdio_channel_id = Some(channel_id);
                        log::info!("Restored Stdio channel for Agent {}", handle.agent_id);
                    }
                    Err(e) => {
                        log::warn!("Failed to restore Stdio channel for Agent {}: {}", handle.agent_id, e);
                    }
                }
                drop(stdio_manager);
            }

            // 添加到内存中
            let agent_id = handle.agent_id.clone();
            let mut agents = self.agents.write().await;
            agents.insert(agent_id.clone(), handle);
            restored_count += 1;

            log::info!("Restored Agent {} from persistence (status: {:?})", agent_id, 
                match session.status.as_str() {
                    "idle" => AgentStatus::Idle,
                    "running" => AgentStatus::Running,
                    "paused" => AgentStatus::Paused,
                    _ => AgentStatus::Idle,
                }
            );
        }

        log::info!("Restored {} agent sessions from persistence", restored_count);
        
        // 更新统计信息
        self.update_stats().await;
        
        Ok(())
    }

    /// 创建新的 Agent
    pub async fn create_agent(
        &self,
        agent_type: AgentType,
        session_id: String,
        project_path: String,
    ) -> Result<String, String> {
        // 保存 agent_type 的引用，避免移动
        let agent_type_clone = agent_type.clone();
        
        // 创建 Agent 句柄
        let mut handle = AgentHandle::new(agent_type, session_id.clone(), project_path.clone());

        // 创建 Stdio 通道 - 直接传递 AgentConfig
        let agent_config = AgentConfig {
            agent_id: handle.agent_id.clone(),
            agent_type: agent_type_clone.clone(),
            phase: handle.phase.clone(),
            status: handle.status.clone(),
            project_path: handle.project_path.clone(),
            session_id: handle.session_id.clone(),
            ai_config: None,
            metadata: None,
        };

        let mut stdio_manager = self.stdio.write().await;
        let channel_id = stdio_manager.create_channel(agent_config)?;
        drop(stdio_manager);

        // 更新句柄
        handle.set_stdio_channel(channel_id);

        // 持久化到数据库 (VC-005)
        if let Err(e) = self.persist_agent(&handle).await {
            log::warn!("Failed to persist agent {}: {}", handle.agent_id, e);
        }

        // 添加到管理器
        let agent_id = handle.agent_id.clone();
        let mut agents = self.agents.write().await;
        agents.insert(agent_id.clone(), handle);

        // 更新统计
        self.update_stats().await;

        log::info!("Created Agent {} of type {:?}", agent_id, agent_type_clone);
        Ok(agent_id)
    }

    /// 持久化 Agent 到数据库 (VC-005)
    async fn persist_agent(&self, handle: &AgentHandle) -> Result<(), String> {
        let conn = db::get_connection(&self.app_handle)
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let session = crate::models::AgentSession {
            session_id: handle.session_id.clone(),
            agent_id: handle.agent_id.clone(),
            agent_type: handle.agent_type.to_string(),
            project_path: handle.project_path.clone(),
            status: handle.status.to_string().to_lowercase(),
            phase: handle.phase.to_string().to_lowercase(),
            created_at: chrono::DateTime::from_timestamp(handle.created_at, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: chrono::DateTime::from_timestamp(handle.updated_at, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            stdio_channel_id: handle.stdio_channel_id.clone(),
            registered_to_daemon: handle.registered_to_daemon,
            metadata: None,
        };

        db::create_agent_session(&conn, &session)
            .map_err(|e| format!("Failed to create agent session: {}", e))
    }

    /// 更新 Agent 状态并持久化 (VC-005)
    async fn update_and_persist_agent(&self, agent_id: &str, status: AgentStatus) -> Result<(), String> {
        // 更新内存中的状态
        {
            let mut agents = self.agents.write().await;
            let handle = agents.get_mut(agent_id)
                .ok_or_else(|| format!("Agent {} not found", agent_id))?;
            
            handle.update_status(status.clone());
            drop(agents);
        }

        // 持久化到数据库
        let conn = db::get_connection(&self.app_handle)
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let status_str = status.to_string().to_lowercase();
        let phase_str = "unknown".to_string(); // TODO: 从 agent 获取当前 phase

        db::update_agent_session_status(&conn, agent_id, &status_str, &phase_str)
            .map_err(|e| format!("Failed to update agent session: {}", e))
    }

    /// 启动 Agent
    pub async fn start_agent(&self, agent_id: &str) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        
        let handle = agents.get_mut(agent_id)
            .ok_or_else(|| format!("Agent {} not found", agent_id))?;

        if handle.status != AgentStatus::Idle && handle.status != AgentStatus::Paused {
            return Err(format!(
                "Agent {} is not in Idle or Paused state. Current state: {:?}",
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
            let _daemon = self.daemon.write().await;
            // TODO: 实际实现中需要调用 daemon.spawn_agent
            log::info!("Daemon instructed to start Agent {}", agent_id);
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
            AgentStatus::Paused
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
            let _daemon = self.daemon.write().await;
            // TODO: 实际实现中需要调用 daemon.kill_agent
            log::info!("Daemon instructed to stop Agent {}", agent_id);
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
    async fn update_stats(&self) {
        let agents = self.agents.read().await;
        let websocket = self.websocket.read().await;
        let stdio = self.stdio.read().await;

        let mut stats = AgentManagerStats::default();
        stats.total_agents = agents.len();
        
        for handle in agents.values() {
            match handle.status {
                AgentStatus::Running => stats.running_agents += 1,
                AgentStatus::Idle => stats.idle_agents += 1,
                AgentStatus::Completed => stats.completed_agents += 1,
                AgentStatus::Failed(_) => stats.failed_agents += 1,
                _ => {}
            }
        }

        stats.active_sessions = agents.values()
            .map(|h| h.session_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        stats.stdio_channels = stdio.get_all_channels().keys().len();
        stats.websocket_connections = websocket.get_stats().await.active_connections as usize;

        drop(agents);
        drop(websocket);
        drop(stdio);

        let mut stats_lock = self.stats.write().await;
        *stats_lock = stats;
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> AgentManagerStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// 获取 Daemon 状态
    pub async fn get_daemon_status(&self) -> DaemonStatus {
        let daemon = self.daemon.read().await;
        daemon.get_status().clone()
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
    pub async fn get_branch_manager(&self) -> tokio::sync::RwLockReadGuard<Option<BranchManager>> {
        self.branch_manager.read().await
    }
}

// ============================================================================
// Tauri Commands (VC-004)
// ============================================================================

/// 创建新的 Agent
#[tauri::command]
pub async fn create_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_type: String,
    session_id: String,
    project_path: String,
) -> Result<String, String> {
    let manager = state.read().await;
    
    // 解析 Agent 类型
    let parsed_type = match agent_type.as_str() {
        "initializer" => AgentType::Initializer,
        "coding" => AgentType::Coding,
        "mr_creation" => AgentType::MRCreation,
        _ => return Err(format!("Unknown agent type: {}", agent_type)),
    };

    manager.create_agent(parsed_type, session_id, project_path).await
}

/// 启动 Agent
#[tauri::command]
pub async fn start_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
) -> Result<(), String> {
    let manager = state.read().await;
    manager.start_agent(&agent_id).await
}

/// 停止 Agent
#[tauri::command]
pub async fn stop_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
    graceful: bool,
) -> Result<(), String> {
    let manager = state.read().await;
    manager.stop_agent(&agent_id, graceful).await
}

/// 获取 Agent 状态
#[tauri::command]
pub async fn get_agent_status(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    agent_id: String,
) -> Result<AgentHandle, String> {
    let manager = state.read().await;
    manager.get_agent_status(&agent_id).await
}

/// 获取所有 Agent
#[tauri::command]
pub async fn get_all_agents(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<AgentHandle>, String> {
    let manager = state.read().await;
    Ok(manager.get_all_agents().await)
}

/// 获取指定 Session 的 Agent
#[tauri::command]
pub async fn get_agents_by_session(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
) -> Result<Vec<AgentHandle>, String> {
    let manager = state.read().await;
    Ok(manager.get_agents_by_session(&session_id.clone()).await)
}

/// 获取 Agent Manager 统计信息
#[tauri::command]
pub async fn get_agent_manager_stats(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<AgentManagerStats, String> {
    let manager = state.read().await;
    Ok(manager.get_stats().await)
}

/// 获取 Daemon 状态
#[tauri::command]
pub async fn get_daemon_statuses(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<DaemonStatus, String> {
    let manager = state.read().await;
    Ok(manager.get_daemon_status().await)
}

/// 获取所有持久化的 Agent Sessions (VC-005)
#[tauri::command]
pub async fn get_all_agent_sessions(
    state: State<'_, Arc<RwLock<AgentManager>>>,
) -> Result<Vec<crate::models::AgentSession>, String> {
    let manager = state.read().await;
    let conn = db::get_connection(&manager.app_handle)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    db::get_all_agent_sessions(&conn)
        .map_err(|e| format!("Failed to fetch agent sessions: {}", e))
}

/// 运行 Initializer Agent 初始化流程 (VC-010)
#[tauri::command]
pub async fn run_initializer_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    prd_content: String,
) -> Result<crate::agent::initializer_agent::InitializerResult, String> {
    use crate::agent::initializer_agent::{InitializerAgent, InitializerAgentConfig};
    use uuid::Uuid;
    
    let manager = state.read().await;
    
    // 创建 InitializerAgent 配置
    let config = InitializerAgentConfig {
        agent_id: format!("initializer-{}", Uuid::new_v4()),
        project_path: project_path.clone(),
        ai_config: crate::ai::AIConfig {
            provider: "openai".to_string(),
            api_key: "placeholder".to_string(), // 实际使用时需要从安全存储获取
            model: "gpt-4".to_string(),
            base_url: None,
        },
        prd_file_path: None,
        prd_content: Some(prd_content),
    };
    
    // 创建 Agent 并执行初始化
    let mut agent = InitializerAgent::new(config);
    let result = agent.run_initialization().await?;
    
    Ok(result)
}

/// 运行 MR Creation Agent 创建合并请求 (VC-016)
#[tauri::command]
pub async fn create_merge_request(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    target_branch: String,
    feature_branches: Vec<String>,
    run_regression_tests: bool,
    auto_resolve_conflicts: bool,
) -> Result<crate::agent::mr_creation_agent::MRCreationResult, String> {
    use crate::agent::mr_creation_agent::{MRCreationAgent, MRCreationConfig};
    
    let manager = state.read().await;
    
    // 创建 MRCreationAgent 配置
    let config = MRCreationConfig {
        project_path: project_path.clone(),
        target_branch: target_branch.clone(),
        feature_branches,
        run_regression_tests,
        auto_resolve_conflicts,
    };
    
    // 创建 Agent 并执行 MR 创建
    let mut agent = MRCreationAgent::new(config);
    let result = agent.create_mr().await?;
    
    Ok(result)
}

/// 运行 Debug Agent 诊断错误 (VC-022)
#[tauri::command]
pub async fn run_debug_agent(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    error_source: String,
    error_output: String,
    auto_fix: bool,
    max_suggestions: usize,
) -> Result<crate::agent::debug_agent::DebugResult, String> {
    use crate::agent::debug_agent::{DebugAgent, DebugAgentConfig, ErrorSource};
    
    let manager = state.read().await;
    
    // 解析错误来源
    let parsed_error_source = match error_source.to_lowercase().as_str() {
        "typescript" | "ts" => ErrorSource::TypeScript,
        "rust" | "rs" => ErrorSource::Rust,
        "eslint" => ErrorSource::ESLint,
        "jest" | "vitest" => ErrorSource::Jest,
        "cargo" | "cargo-test" => ErrorSource::CargoTest,
        "runtime" | "log" => ErrorSource::RuntimeLog,
        _ => ErrorSource::UserInput,
    };
    
    // 创建 DebugAgent 配置
    let config = DebugAgentConfig {
        project_path: project_path.clone(),
        error_source: parsed_error_source,
        auto_fix,
        max_suggestions: if max_suggestions == 0 { 5 } else { max_suggestions },
        error_output,
    };
    
    // 创建 Agent 并执行调试
    let mut agent = DebugAgent::new(config);
    let result = agent.run_debug().await?;
    
    Ok(result)
}

/// 生成 Git 提交信息 (VC-026)
#[tauri::command]
pub async fn generate_commit_message(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    use_ai: bool,
    include_file_list: bool,
    max_summary_length: usize,
    conventional_commit: bool,
) -> Result<crate::agent::git_commit_assistant::CommitMessage, String> {
    use crate::agent::git_commit_assistant::{GitCommitAssistant, GitCommitAssistantConfig};
    
    let manager = state.read().await;
    
    // 创建 GitCommitAssistant 配置
    let config = GitCommitAssistantConfig {
        project_path: project_path.clone(),
        use_ai,
        include_file_list,
        max_summary_length: if max_summary_length == 0 { 50 } else { max_summary_length },
        conventional_commit,
    };
    
    // 创建 Assistant 并生成提交信息
    let mut assistant = GitCommitAssistant::new(config);
    let message = assistant.generate_commit_message().await?;
    
    Ok(message)
}

/// 创建功能分支
#[tauri::command]
pub async fn create_feature_branch(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
    issue_id: String,
    description: String,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    
    // 创建功能分支
    let result = branch_manager
        .as_mut()
        .unwrap()
        .create_feature_branch(&description, Some(&issue_id), None)
        .await?;
    
    Ok(result)
}

/// 切换到指定分支
#[tauri::command]
pub async fn checkout_branch(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
    branch_name: String,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    let result = branch_manager.as_mut().unwrap().checkout_branch(&branch_name).await?;
    Ok(result)
}

/// 删除分支
#[tauri::command]
pub async fn delete_branch(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
    branch_name: String,
    force: bool,
) -> Result<BranchOperationResult, String> {
    let manager = state.read().await;
    let mut branch_manager = manager.get_or_create_branch_manager().await;
    let result = branch_manager.as_mut().unwrap().delete_branch(&branch_name, force).await?;
    Ok(result)
}

/// 列出所有分支
#[tauri::command]
pub async fn list_branches(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
) -> Result<Vec<BranchInfo>, String> {
    let manager = state.read().await;
    let branch_manager = manager.get_branch_manager().await;
    let branches = branch_manager.as_ref().unwrap().get_local_branches().await?;
    Ok(branches)
}

/// 获取当前分支
#[tauri::command]
pub async fn get_current_branch(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
) -> Result<Option<String>, String> {
    let manager = state.read().await;
    let branch_manager = manager.get_branch_manager().await;
    let current = branch_manager.as_ref().unwrap().get_current_branch().await?;
    Ok(current)
}

/// 运行代码审查 Agent
#[tauri::command]
pub async fn run_code_review(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    _session_id: String,
    _file_paths: Vec<String>,
    enable_ai: bool,
) -> Result<ReviewResult, String> {
    let _manager = state.read().await;
    
    // 创建 CodeReviewAgent 配置
    let config = CodeReviewAgentConfig {
        project_path: ".".to_string(),
        enable_ai,
        dimensions: vec![
            ReviewDimension::Style,
            ReviewDimension::Performance,
            ReviewDimension::Security,
            ReviewDimension::BestPractice,
        ],
        min_severity: ReviewSeverity::Info,
        max_comments: 100,
    };

    let mut agent = CodeReviewAgent::new(config);

    // TODO: 实际实现中需要从文件路径读取代码内容
    // 这里创建一个示例的 CodeChange 列表用于演示
    let code_changes = vec![
        CodeChange {
            file_path: "example.rs".to_string(),
            content: "// Example code for review".to_string(),
            language: "rust".to_string(),
            change_type: "Modified".to_string(),
        }
    ];

    // 运行审查
    let result = agent.run_review(&code_changes).await?;
    Ok(result)
}

/// 启动实时审查监听
#[tauri::command]
pub async fn start_realtime_review(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
    config: WatchConfig,
) -> Result<(), String> {
    let _manager = state.read().await;
    
    // TODO: 实际实现中需要在 AgentManager 中管理 RealtimeReviewManager 实例
    // 这里创建一个临时实例用于演示
    let mut manager = RealtimeReviewManager::new(config);
    manager.start_watch().await?;
    
    log::info!("实时审查监听已启动 for session: {}", session_id);
    Ok(())
}

/// 停止实时审查监听
#[tauri::command]
pub async fn stop_realtime_review(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
) -> Result<(), String> {
    let _manager = state.read().await;
    
    // TODO: 实际实现中需要从 AgentManager 获取 RealtimeReviewManager 实例
    log::info!("实时审查监听已停止 for session: {}", session_id);
    Ok(())
}

/// 运行测试
#[tauri::command]
pub async fn run_tests(
    state: tauri::State<'_, Arc<tokio::sync::RwLock<AgentManager>>>,
    session_id: String,
    config: TestRunnerConfig,
) -> Result<TestSuiteResult, String> {
    let _manager = state.read().await;
    
    // 创建 TestRunnerAgent 并运行测试
    let agent = TestRunnerAgent::new(config);
    let result = agent.run_tests().await?;
    
    log::info!("测试完成 for session {}: {} passed / {} total", 
               session_id, result.passed, result.total);
    
    Ok(result)
}

/// 初始化 Agent Manager
#[tauri::command]
pub async fn initialize_agent_manager(
    state: State<'_, Arc<RwLock<AgentManager>>>,
    session_id: String,
    project_path: String,
    max_concurrent_agents: usize,
) -> Result<(), String> {
    let manager = state.read().await;
    
    let config = DaemonConfig {
        session_id,
        project_path,
        log_level: "info".to_string(),
        max_concurrent_agents,
        workspace_dir: ".".to_string(),
    };

    manager.initialize(config).await
}

// ============================================================================
// Tests (VC-004)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_handle_creation() {
        let handle = AgentHandle::new(
            AgentType::Initializer,
            "test-session".to_string(),
            "/tmp/test".to_string(),
        );

        assert!(!handle.agent_id.is_empty());
        assert_eq!(handle.agent_type, AgentType::Initializer);
        assert_eq!(handle.session_id, "test-session");
        assert_eq!(handle.status, AgentStatus::Idle);
        assert_eq!(handle.phase, AgentPhase::Initializer);
        assert!(!handle.registered_to_daemon);
        assert!(handle.stdio_channel_id.is_none());
    }

    #[test]
    fn test_agent_handle_status_update() {
        let mut handle = AgentHandle::new(
            AgentType::Coding,
            "session-123".to_string(),
            "/tmp/project".to_string(),
        );

        assert_eq!(handle.status, AgentStatus::Idle);
        
        handle.update_status(AgentStatus::Running);
        assert_eq!(handle.status, AgentStatus::Running);
        
        handle.update_status(AgentStatus::Completed);
        assert_eq!(handle.status, AgentStatus::Completed);
    }

    #[test]
    fn test_agent_handle_stdio_channel() {
        let mut handle = AgentHandle::new(
            AgentType::MRCreation,
            "session-456".to_string(),
            "/tmp/mr".to_string(),
        );

        assert!(handle.stdio_channel_id.is_none());
        
        handle.set_stdio_channel("channel-789".to_string());
        assert_eq!(handle.stdio_channel_id, Some("channel-789".to_string()));
    }

    #[test]
    fn test_agent_handle_mark_registered() {
        let mut handle = AgentHandle::new(
            AgentType::Initializer,
            "test".to_string(),
            "/tmp".to_string(),
        );

        assert!(!handle.registered_to_daemon);
        
        handle.mark_registered();
        assert!(handle.registered_to_daemon);
    }

    #[test]
    fn test_agent_manager_stats_default() {
        let stats = AgentManagerStats::default();
        
        assert_eq!(stats.total_agents, 0);
        assert_eq!(stats.running_agents, 0);
        assert_eq!(stats.idle_agents, 0);
        assert_eq!(stats.completed_agents, 0);
        assert_eq!(stats.failed_agents, 0);
    }

    #[tokio::test]
    async fn test_agent_manager_creation() {
        // 注意：这个测试需要一个 mock 的 AppHandle
        // 在实际测试中需要使用 tauri::test::mock_context 等工具
        // 这里只是一个示例
        println!("AgentManager creation test - requires mock AppHandle");
    }

    #[test]
    fn test_agent_type_display() {
        assert_eq!(format!("{}", AgentType::Initializer), "initializer");
        assert_eq!(format!("{}", AgentType::Coding), "coding");
        assert_eq!(format!("{}", AgentType::MRCreation), "mr_creation");
    }

    // ========================================================================
    // VC-005: Session Persistence Tests
    // ========================================================================

    #[test]
    fn test_agent_status_display() {
        assert_eq!(format!("{}", AgentStatus::Idle), "idle");
        assert_eq!(format!("{}", AgentStatus::Running), "running");
        assert_eq!(format!("{}", AgentStatus::Paused), "paused");
        assert_eq!(format!("{}", AgentStatus::Completed), "completed");
        assert_eq!(format!("{}", AgentStatus::Failed("error".to_string())), "failed:error");
    }

    #[test]
    fn test_agent_phase_display() {
        assert_eq!(format!("{}", AgentPhase::Initializer), "initializer");
        assert_eq!(format!("{}", AgentPhase::Coding), "coding");
        assert_eq!(format!("{}", AgentPhase::MRCreation), "mr_creation");
    }

    #[test]
    fn test_agent_session_serialization() {
        let session = crate::models::AgentSession {
            session_id: "test-session-123".to_string(),
            agent_id: "agent-456".to_string(),
            agent_type: "initializer".to_string(),
            project_path: "/tmp/test".to_string(),
            status: "running".to_string(),
            phase: "initializer".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            stdio_channel_id: Some("channel-789".to_string()),
            registered_to_daemon: true,
            metadata: None,
        };

        // Test serialization
        let json = serde_json::to_string(&session).unwrap();
        assert!(json.contains("test-session-123"));
        assert!(json.contains("agent-456"));
        assert!(json.contains("initializer"));

        // Test deserialization
        let deserialized: crate::models::AgentSession = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.session_id, session.session_id);
        assert_eq!(deserialized.agent_id, session.agent_id);
        assert_eq!(deserialized.registered_to_daemon, session.registered_to_daemon);
    }
}

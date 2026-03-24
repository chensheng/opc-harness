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
            daemon_config: Arc::new(RwLock::new(None)),
            stats: Arc::new(RwLock::new(AgentManagerStats::default())),
        }
    }

    /// 初始化 Agent Manager（启动 Daemon）
    pub async fn initialize(&self, config: DaemonConfig) -> Result<(), String> {
        // 保存配置
        *self.daemon_config.write().await = Some(config.clone());

        // 启动 Daemon Manager
        let mut daemon = self.daemon.write().await;
        daemon.start(config)?;

        log::info!("Agent Manager initialized and Daemon started");
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

        // 添加到管理器
        let agent_id = handle.agent_id.clone();
        let mut agents = self.agents.write().await;
        agents.insert(agent_id.clone(), handle);

        // 更新统计
        self.update_stats().await;

        log::info!("Created Agent {} of type {:?}", agent_id, agent_type_clone);
        Ok(agent_id)
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

        // 更新状态为 Running
        handle.update_status(AgentStatus::Running);
        let agent_type = handle.agent_type.clone();
        let session_id = handle.session_id.clone();
        drop(agents);

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

        // 更新状态
        handle.update_status(if graceful {
            AgentStatus::Paused
        } else {
            AgentStatus::Idle
        });
        
        let session_id = handle.session_id.clone();
        drop(agents);

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
}

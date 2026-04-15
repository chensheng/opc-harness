//! Agent Manager 持久化逻辑
//! 
//! 处理 Agent Session 的数据库持久化和恢复

use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::agent::agent_manager_types::{AgentHandle, AgentManagerStats};
use crate::agent::types::{AgentType, AgentStatus, AgentPhase, AgentConfig};
use crate::db;

/// 持久化 Agent 到数据库 (VC-005)
pub async fn persist_agent(
    app_handle: &AppHandle,
    handle: &AgentHandle,
) -> Result<(), String> {
    log::info!("[persist_agent] Persisting agent to database: agent_id={}, session_id={}, project_id={}", 
        handle.agent_id, handle.session_id, handle.project_id);
    
    let conn = db::get_connection(app_handle)
        .map_err(|e| {
            log::error!("[persist_agent] Failed to get database connection: {}", e);
            format!("Failed to get database connection: {}", e)
        })?;

    let session = crate::models::AgentSession {
        session_id: handle.session_id.clone(),
        agent_id: handle.agent_id.clone(),
        agent_type: handle.agent_type.to_string(),
        project_id: handle.project_id.clone(),
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
    
    log::info!("[persist_agent] Session data prepared: agent_type={}, status={}, phase={}", 
        session.agent_type, session.status, session.phase);

    match db::create_agent_session(&conn, &session) {
        Ok(_) => {
            log::info!("[persist_agent] Agent session persisted successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("[persist_agent] Failed to create agent session: {}", e);
            Err(format!("Failed to create agent session: {}", e))
        }
    }
}

/// 更新 Agent 状态并持久化 (VC-005)
pub async fn update_and_persist_agent(
    app_handle: &AppHandle,
    agents: &Arc<RwLock<std::collections::HashMap<String, AgentHandle>>>,
    agent_id: &str,
    status: AgentStatus,
) -> Result<(), String> {
    // 更新内存中的状态
    {
        let mut agents_lock = agents.write().await;
        let handle = agents_lock.get_mut(agent_id)
            .ok_or_else(|| format!("Agent {} not found", agent_id))?;
        
        handle.update_status(status.clone());
        drop(agents_lock);
    }

    // 持久化到数据库
    let conn = db::get_connection(app_handle)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let status_str = status.to_string().to_lowercase();
    let phase_str = "unknown".to_string(); // TODO: 从 agent 获取当前 phase

    db::update_agent_session_status(&conn, agent_id, &status_str, &phase_str)
        .map_err(|e| format!("Failed to update agent session: {}", e))
}

/// 恢复持久化的 Sessions (VC-005)
pub async fn restore_sessions(
    app_handle: &AppHandle,
    agents: &Arc<RwLock<std::collections::HashMap<String, AgentHandle>>>,
    stdio: &Arc<RwLock<crate::agent::agent_stdio::StdioChannelManager>>,
    websocket: &Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    stats: &Arc<RwLock<AgentManagerStats>>,
) -> Result<(), String> {
    let conn = db::get_connection(app_handle)
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
            project_id: session.project_id.clone(),
            stdio_channel_id: session.stdio_channel_id,
            registered_to_daemon: session.registered_to_daemon,
        };

        // 重新创建 Stdio 通道（如果需要）
        if handle.stdio_channel_id.is_some() {
            // 注意：这里需要project_path来创建Stdio通道，应该从项目信息中获取
            // 暂时使用空字符串，后续需要从project_id查询项目路径
            let project_path = String::new();
            let agent_config = AgentConfig {
                agent_id: handle.agent_id.clone(),
                agent_type: handle.agent_type.clone(),
                phase: handle.phase.clone(),
                status: handle.status.clone(),
                project_path,
                session_id: handle.session_id.clone(),
                ai_config: None,
                metadata: None,
            };

            let mut stdio_manager = stdio.write().await;
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
        let mut agents_lock = agents.write().await;
        agents_lock.insert(agent_id.clone(), handle);
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
    update_stats(agents, stdio, websocket, stats).await;
    
    Ok(())
}

/// 更新统计信息
pub async fn update_stats(
    agents: &Arc<RwLock<std::collections::HashMap<String, AgentHandle>>>,
    stdio: &Arc<RwLock<crate::agent::agent_stdio::StdioChannelManager>>,
    websocket: &Arc<RwLock<crate::agent::websocket_manager::WebSocketManager>>,
    stats: &Arc<RwLock<AgentManagerStats>>,
) {
    let agents_lock = agents.read().await;
    let websocket_lock = websocket.read().await;
    let stdio_lock = stdio.read().await;

    let mut new_stats = AgentManagerStats::default();
    new_stats.total_agents = agents_lock.len() as u32;
    
    for handle in agents_lock.values() {
        match handle.status {
            AgentStatus::Running => new_stats.running_agents += 1,
            AgentStatus::Idle => new_stats.idle_agents += 1,
            AgentStatus::Completed => new_stats.completed_agents += 1,
            AgentStatus::Failed(_) => new_stats.failed_agents += 1,
            _ => {}
        }
    }

    drop(agents_lock);
    drop(websocket_lock);
    drop(stdio_lock);

    let mut stats_lock = stats.write().await;
    *stats_lock = new_stats;
}

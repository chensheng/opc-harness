//! WebSocket 实时推送层 (VC-003) - Tauri Commands
//!
//! 注意：当前实现使用简化的方式，不依赖全局状态管理
//! 后续可以重构为使用 app.manage()
//!
//! 实现基于 Tauri Events 的实时消息推送机制:
//! - WebSocketManager: 全局事件管理器
//! - WebSocketConnection: 单个客户端连接
//! - 支持按 Session ID 隔离消息
//! - 支持广播和单播模式

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use uuid::Uuid;

/// WebSocket 连接 ID
pub type ConnectionId = String;

/// Session ID 类型
pub type SessionId = String;

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessageType {
    /// 日志消息
    Log {
        level: String,
        message: String,
        source: Option<String>,
    },
    /// 进度更新
    Progress {
        phase: String,
        current: u32,
        total: u32,
        description: Option<String>,
    },
    /// 状态更新
    Status {
        status: String,
        details: Option<String>,
    },
    /// Agent 响应
    AgentResponse {
        request_id: String,
        success: bool,
        data: Option<serde_json::Value>,
        error: Option<String>,
    },
    /// 错误消息
    Error {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },
    /// 心跳
    Heartbeat {
        timestamp: u64,
    },
}

/// WebSocket 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// 消息 ID
    pub id: ConnectionId,
    /// Session ID (用于消息隔离)
    pub session_id: SessionId,
    /// 消息类型和内容
    #[serde(flatten)]
    pub message_type: WsMessageType,
    /// 时间戳
    pub timestamp: u64,
}

impl WsMessage {
    /// 创建新的日志消息
    pub fn log(session_id: SessionId, level: &str, message: &str, source: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::Log {
                level: level.to_string(),
                message: message.to_string(),
                source: source.map(|s| s.to_string()),
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建进度更新消息
    pub fn progress(
        session_id: SessionId,
        phase: &str,
        current: u32,
        total: u32,
        description: Option<&str>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::Progress {
                phase: phase.to_string(),
                current,
                total,
                description: description.map(|s| s.to_string()),
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建状态更新消息
    pub fn status(session_id: SessionId, status: &str, details: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::Status {
                status: status.to_string(),
                details: details.map(|s| s.to_string()),
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建 Agent 响应消息
    pub fn agent_response(
        session_id: SessionId,
        request_id: String,
        success: bool,
        data: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::AgentResponse {
                request_id,
                success,
                data,
                error,
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建错误消息
    pub fn error(
        session_id: SessionId,
        code: &str,
        message: &str,
        details: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::Error {
                code: code.to_string(),
                message: message.to_string(),
                details,
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建心跳消息
    pub fn heartbeat(session_id: SessionId) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            message_type: WsMessageType::Heartbeat {
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
            },
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

/// WebSocket 连接信息
#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    /// 连接 ID
    pub id: ConnectionId,
    /// Session ID
    pub session_id: SessionId,
    /// 连接时间戳
    pub connected_at: u64,
    /// 最后活跃时间
    pub last_active: u64,
    /// 发送的消息数
    pub messages_sent: u32,
}

impl WebSocketConnection {
    /// 创建新的连接
    pub fn new(session_id: SessionId) -> Self {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            connected_at: now,
            last_active: now,
            messages_sent: 0,
        }
    }
}

/// WebSocket 管理器
/// 
/// 负责管理所有 WebSocket 连接，提供消息发送和广播功能
pub struct WebSocketManager {
    /// Tauri 应用句柄
    app_handle: AppHandle,
    /// 连接注册表 (按 Session ID 分组)
    connections: Arc<RwLock<HashMap<SessionId, Vec<WebSocketConnection>>>>,
    /// 统计信息
    stats: Arc<RwLock<WebSocketStats>>,
}

/// WebSocket 统计信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebSocketStats {
    /// 总连接数
    pub total_connections: u32,
    /// 当前活跃连接数
    pub active_connections: u32,
    /// 总发送消息数
    pub total_messages_sent: u32,
    /// 总错误数
    pub total_errors: u32,
}

impl WebSocketManager {
    /// 创建新的 WebSocket 管理器
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            connections: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(WebSocketStats::default())),
        }
    }

    /// 注册新的 WebSocket 连接
    pub async fn register_connection(&self, session_id: SessionId) -> ConnectionId {
        let mut connections = self.connections.write().await;
        let connection = WebSocketConnection::new(session_id.clone());
        let connection_id = connection.id.clone();

        // 添加到注册表
        connections
            .entry(session_id)
            .or_insert_with(Vec::new)
            .push(connection);

        // 更新统计
        let mut stats = self.stats.write().await;
        stats.total_connections += 1;
        stats.active_connections += 1;

        connection_id
    }

    /// 断开 WebSocket 连接
    pub async fn unregister_connection(&self, session_id: &SessionId, connection_id: &ConnectionId) {
        let mut connections = self.connections.write().await;
        
        if let Some(session_connections) = connections.get_mut(session_id) {
            session_connections.retain(|c| c.id != *connection_id);
            
            // 如果该 Session 没有连接了，删除整个条目
            if session_connections.is_empty() {
                connections.remove(session_id);
            }
        }

        // 更新统计
        let mut stats = self.stats.write().await;
        if stats.active_connections > 0 {
            stats.active_connections -= 1;
        }
    }

    /// 获取指定 Session 的所有连接
    pub async fn get_connections(&self, session_id: &SessionId) -> Vec<WebSocketConnection> {
        let connections = self.connections.read().await;
        connections
            .get(session_id)
            .cloned()
            .unwrap_or_default()
    }

    /// 获取所有活跃 Session IDs
    pub async fn get_all_session_ids(&self) -> Vec<SessionId> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    /// 发送消息到指定 Session (广播给该 Session 的所有连接)
    pub async fn send_to_session(&self, session_id: &SessionId, message: WsMessage) -> Result<(), String> {
        let event_name = format!("ws:{}", session_id);
        
        // 使用 Tauri emit API 发送事件
        self.app_handle
            .emit(&event_name, &message)
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        // 更新统计
        let mut connections = self.connections.write().await;
        if let Some(session_connections) = connections.get_mut(session_id) {
            for conn in session_connections.iter_mut() {
                conn.messages_sent += 1;
                conn.last_active = message.timestamp;
            }
        }

        let mut stats = self.stats.write().await;
        stats.total_messages_sent += 1;

        Ok(())
    }

    /// 广播消息到所有 Session
    pub async fn broadcast(&self, message: WsMessage) -> Result<(), String> {
        let session_ids = self.get_all_session_ids().await;
        
        for session_id in &session_ids {
            let _ = self.send_to_session(session_id, message.clone()).await;
        }

        Ok(())
    }

    /// 发送日志消息
    pub async fn send_log(
        &self,
        session_id: &SessionId,
        level: &str,
        message: &str,
        source: Option<&str>,
    ) -> Result<(), String> {
        let ws_message = WsMessage::log(session_id.clone(), level, message, source);
        self.send_to_session(session_id, ws_message).await
    }

    /// 发送进度更新
    pub async fn send_progress(
        &self,
        session_id: &SessionId,
        phase: &str,
        current: u32,
        total: u32,
        description: Option<&str>,
    ) -> Result<(), String> {
        let ws_message = WsMessage::progress(session_id.clone(), phase, current, total, description);
        self.send_to_session(session_id, ws_message).await
    }

    /// 发送状态更新
    pub async fn send_status(
        &self,
        session_id: &SessionId,
        status: &str,
        details: Option<&str>,
    ) -> Result<(), String> {
        let ws_message = WsMessage::status(session_id.clone(), status, details);
        self.send_to_session(session_id, ws_message).await
    }

    /// 发送 Agent 响应
    pub async fn send_agent_response(
        &self,
        session_id: &SessionId,
        request_id: String,
        success: bool,
        data: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Result<(), String> {
        let ws_message = WsMessage::agent_response(
            session_id.clone(),
            request_id,
            success,
            data,
            error,
        );
        self.send_to_session(session_id, ws_message).await
    }

    /// 发送错误消息
    pub async fn send_error(
        &self,
        session_id: &SessionId,
        code: &str,
        message: &str,
        details: Option<serde_json::Value>,
    ) -> Result<(), String> {
        let ws_message = WsMessage::error(session_id.clone(), code, message, details);
        self.send_to_session(session_id, ws_message).await
    }

    /// 发送心跳
    pub async fn send_heartbeat(&self, session_id: &SessionId) -> Result<(), String> {
        let ws_message = WsMessage::heartbeat(session_id.clone());
        self.send_to_session(session_id, ws_message).await
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> WebSocketStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// 获取连接数
    pub async fn get_connection_count(&self, session_id: &SessionId) -> usize {
        let connections = self.connections.read().await;
        connections
            .get(session_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// 清理断开的连接 (超时检测)
    pub async fn cleanup_stale_connections(&self, timeout_ms: u64) {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let mut connections = self.connections.write().await;
        
        let mut removed_sessions = Vec::new();
        
        for (session_id, session_connections) in connections.iter_mut() {
            session_connections.retain(|conn| {
                if now - conn.last_active > timeout_ms {
                    false
                } else {
                    true
                }
            });
            
            if session_connections.is_empty() {
                removed_sessions.push(session_id.clone());
            }
        }
        
        // 移除空的 Session
        for session_id in &removed_sessions {
            connections.remove(session_id);
        }

        // 更新统计
        let mut stats = self.stats.write().await;
        stats.active_connections = connections.values().map(|v| v.len() as u32).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_creation() {
        let msg = WsMessage::log(
            "test-session".to_string(),
            "info",
            "Test message",
            Some("test-source"),
        );

        assert!(!msg.id.is_empty());
        assert_eq!(msg.session_id, "test-session");
        
        match msg.message_type {
            WsMessageType::Log { level, message, source } => {
                assert_eq!(level, "info");
                assert_eq!(message, "Test message");
                assert_eq!(source, Some("test-source".to_string()));
            }
            _ => panic!("Expected Log message type"),
        }
    }

    #[test]
    fn test_ws_message_progress() {
        let msg = WsMessage::progress(
            "session-123".to_string(),
            "coding",
            5,
            10,
            Some("Generating code..."),
        );

        match msg.message_type {
            WsMessageType::Progress { phase, current, total, description } => {
                assert_eq!(phase, "coding");
                assert_eq!(current, 5);
                assert_eq!(total, 10);
                assert_eq!(description, Some("Generating code...".to_string()));
            }
            _ => panic!("Expected Progress message type"),
        }
    }

    #[test]
    fn test_ws_message_status() {
        let msg = WsMessage::status(
            "session-456".to_string(),
            "running",
            Some("Processing task"),
        );

        match msg.message_type {
            WsMessageType::Status { status, details } => {
                assert_eq!(status, "running");
                assert_eq!(details, Some("Processing task".to_string()));
            }
            _ => panic!("Expected Status message type"),
        }
    }

    #[test]
    fn test_websocket_connection_creation() {
        let conn = WebSocketConnection::new("test-session".to_string());
        
        assert!(!conn.id.is_empty());
        assert_eq!(conn.session_id, "test-session");
        assert!(conn.connected_at > 0);
        assert!(conn.last_active > 0);
        assert_eq!(conn.messages_sent, 0);
    }

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsMessage::log(
            "test".to_string(),
            "debug",
            "Serialized message",
            None,
        );

        let json = serde_json::to_string(&msg).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"Log\""));
        assert!(json.contains("\"level\":\"debug\""));

        let deserialized: WsMessage = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.session_id, msg.session_id);
    }
}

// ============================================================================
// Tauri Events Integration (VC-003)
// ============================================================================
//
// 使用说明:
// 
// Rust 端发送事件:
// ```rust
// let app_handle = tauri::State::<Arc<RwLock<WebSocketManager>>>::inner();
// let manager = WebSocketManager::new(app_handle);
// manager.send_log("session-123", "info", "Message", None).await?;
// ```
//
// TypeScript 端监听事件:
// ```typescript
// import { listen } from '@tauri-apps/api/event';
// 
// // 监听特定 session 的消息
// const unlisten = await listen(`ws:${sessionId}`, (event) => {
//   console.log('Received:', event.payload);
// });
// 
// // 清理监听器
// unlisten();
// ```
//
// 注意：当前版本未注册为 Tauri Commands，直接在 Rust 代码中使用 WebSocketManager

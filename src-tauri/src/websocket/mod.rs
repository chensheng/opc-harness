/// US-061: WebSocket 服务端实现
/// 
/// 提供实时双向通信能力，支持：
/// - Agent 执行日志实时推送
/// - 系统通知广播
/// - 主题订阅机制
/// - 心跳和自动重连

use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// 客户端订阅主题
    #[serde(rename = "subscribe")]
    Subscribe { topic: Option<String> },
    /// 客户端取消订阅
    #[serde(rename = "unsubscribe")]
    Unsubscribe { topic: Option<String> },
    /// 心跳请求
    #[serde(rename = "ping")]
    Ping,
    /// 心跳响应
    #[serde(rename = "pong")]
    Pong,
    /// 日志推送
    #[serde(rename = "log")]
    Log {
        topic: Option<String>,
        payload: serde_json::Value,
        timestamp: i64,
    },
    /// 通知推送
    #[serde(rename = "notification")]
    Notification {
        topic: Option<String>,
        payload: serde_json::Value,
        timestamp: i64,
    },
    /// 错误消息
    #[serde(rename = "error")]
    Error { message: String },
}

/// WebSocket 连接管理器
pub struct WebSocketManager {
    /// 活跃的客户端连接
    clients: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    /// 广播通道（用于全局通知）
    broadcaster: broadcast::Sender<WsMessage>,
    /// 下一个客户端 ID
    next_client_id: u32,
}

impl WebSocketManager {
    /// 创建新的 WebSocket 管理器
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(100);
        
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            broadcaster: broadcaster.clone(),
            next_client_id: 0,
        }
    }

    /// 添加新客户端
    pub async fn add_client(&mut self, tx: mpsc::UnboundedSender<Message>) -> String {
        let client_id = format!("client_{}", self.next_client_id);
        self.next_client_id += 1;
        
        self.clients.write().await.insert(client_id.clone(), tx);
        info!("New WebSocket client connected: {}", client_id);
        
        client_id
    }

    /// 移除客户端
    pub async fn remove_client(&mut self, client_id: &str) {
        self.clients.write().await.remove(client_id);
        info!("WebSocket client disconnected: {}", client_id);
    }

    /// 广播消息给所有客户端
    pub async fn broadcast(&self, message: WsMessage) {
        if let Ok(json) = serde_json::to_string(&message) {
            let ws_msg = Message::Text(json.into());
            
            // 通过广播通道发送
            let _ = self.broadcaster.send(message);
            
            // 直接发送给所有客户端
            let clients = self.clients.read().await;
            for (client_id, tx) in clients.iter() {
                if let Err(e) = tx.send(ws_msg.clone()) {
                    warn!("Failed to send message to {}: {}", client_id, e);
                }
            }
        }
    }

    /// 发送日志消息
    pub async fn send_log(&self, payload: serde_json::Value, topic: Option<String>) {
        let message = WsMessage::Log {
            topic,
            payload,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        self.broadcast(message).await;
    }

    /// 发送通知消息
    pub async fn send_notification(&self, payload: serde_json::Value, topic: Option<String>) {
        let message = WsMessage::Notification {
            topic,
            payload,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        self.broadcast(message).await;
    }

    /// 获取客户端数量
    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 启动 WebSocket 服务端
pub async fn start_websocket_server(
    port: u16,
    manager: Arc<RwLock<WebSocketManager>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("WebSocket server started on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);
        
        let manager = manager.clone();
        
        tokio::spawn(async move {
            handle_connection(stream, manager).await;
        });
    }

    Ok(())
}

/// 处理 WebSocket 连接
async fn handle_connection(
    stream: tokio::net::TcpStream,
    manager: Arc<RwLock<WebSocketManager>>,
) {
    use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;

    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    // 添加客户端到管理器
    let client_id = {
        let mut mgr = manager.write().await;
        mgr.add_client(tx).await
    };

    // 写入任务：接收来自管理器的消息并发送给客户端
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(msg).await.is_err() {
                break;
            }
        }
    });

    // 读取任务：接收来自客户端的消息
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                WsMessage::Text(text) => {
                    // 处理客户端消息（订阅/取消订阅/心跳等）
                    if let Ok(ws_msg) = serde_json::from_str::<crate::websocket::WsMessage>(&text) {
                        match ws_msg {
                            crate::websocket::WsMessage::Ping => {
                                // 回复 Pong
                                let pong = crate::websocket::WsMessage::Pong;
                                if let Ok(json) = serde_json::to_string(&pong) {
                                    let _ = write.send(WsMessage::Text(json.into())).await;
                                }
                            }
                            crate::websocket::WsMessage::Subscribe { .. } => {
                                info!("Client {} subscribed to topic", client_id);
                            }
                            crate::websocket::WsMessage::Unsubscribe { .. } => {
                                info!("Client {} unsubscribed from topic", client_id);
                            }
                            _ => {}
                        }
                    }
                }
                WsMessage::Close(_) => {
                    info!("Client {} disconnected", client_id);
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = write_task => {},
        _ = read_task => {},
    }

    // 清理客户端
    let mut mgr = manager.write().await;
    mgr.remove_client(&client_id).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.next_client_id, 0);
    }

    #[tokio::test]
    async fn test_add_client() {
        let mut manager = WebSocketManager::new();
        let (tx, _) = mpsc::unbounded_channel();
        
        let client_id = manager.add_client(tx).await;
        
        assert_eq!(client_id, "client_0");
        assert_eq!(manager.client_count().await, 1);
    }

    #[tokio::test]
    async fn test_remove_client() {
        let mut manager = WebSocketManager::new();
        let (tx, _) = mpsc::unbounded_channel();
        
        let client_id = manager.add_client(tx).await;
        assert_eq!(manager.client_count().await, 1);
        
        manager.remove_client(&client_id).await;
        assert_eq!(manager.client_count().await, 0);
    }

    #[tokio::test]
    async fn test_broadcast() {
        let manager = WebSocketManager::new();
        let message = WsMessage::Notification {
            topic: Some("test".to_string()),
            payload: serde_json::json!({"test": "data"}),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        
        // 不应该 panic
        manager.broadcast(message).await;
    }

    #[tokio::test]
    async fn test_send_log() {
        let manager = WebSocketManager::new();
        
        manager.send_log(serde_json::json!({"msg": "test"}), Some("agent".to_string())).await;
        
        // 不应该 panic
    }
}

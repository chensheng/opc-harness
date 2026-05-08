use log;
/// 去中心化智能体系统 - 事件总线 (纯内存实现)
///
/// 基于 tokio::sync::broadcast 实现进程内事件广播,无需 Redis
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// 事件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    StoryCreated,
    StoryLocked,
    AgentStarted,
    AgentCompleted,
    AgentFailed,
    WorktreeReady,
    MergeRequested,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::StoryCreated => write!(f, "story.created"),
            EventType::StoryLocked => write!(f, "story.locked"),
            EventType::AgentStarted => write!(f, "agent.started"),
            EventType::AgentCompleted => write!(f, "agent.completed"),
            EventType::AgentFailed => write!(f, "agent.failed"),
            EventType::WorktreeReady => write!(f, "worktree.ready"),
            EventType::MergeRequested => write!(f, "merge.requested"),
        }
    }
}

/// 事件消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    pub source_node_id: String,
}

/// 共享事件总线 (所有 Node 实例共享)
pub struct SharedEventBus {
    sender: broadcast::Sender<EventMessage>,
}

impl SharedEventBus {
    /// 创建新的共享事件总线
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000); // 容量 1000 条消息
        Self { sender }
    }

    /// 发布事件
    pub fn publish(
        &self,
        event_type: EventType,
        payload: impl Serialize,
        source_node_id: &str,
    ) -> Result<(), String> {
        let payload_json = serde_json::to_value(payload)
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        let message = EventMessage {
            event_type: event_type.to_string(),
            payload: payload_json,
            timestamp: chrono::Utc::now().timestamp(),
            source_node_id: source_node_id.to_string(),
        };

        self.sender
            .send(message)
            .map_err(|e| format!("Failed to publish event: {}", e))?;

        log::debug!(
            "[EventBus] Published event: {} from node {}",
            event_type,
            source_node_id
        );

        Ok(())
    }

    /// 订阅事件 (返回接收器)
    pub fn subscribe(&self) -> broadcast::Receiver<EventMessage> {
        self.sender.subscribe()
    }
}

/// 事件总线客户端 (每个 Node 持有一个引用)
#[derive(Clone)]
pub struct EventBusClient {
    shared_bus: Arc<SharedEventBus>,
    node_id: String,
}

impl EventBusClient {
    /// 创建新的事件总线客户端
    pub fn new(shared_bus: Arc<SharedEventBus>, node_id: &str) -> Self {
        Self {
            shared_bus,
            node_id: node_id.to_string(),
        }
    }

    /// 发布事件
    pub fn publish(&self, event_type: EventType, payload: impl Serialize) -> Result<(), String> {
        self.shared_bus.publish(event_type, payload, &self.node_id)
    }

    /// 订阅事件
    pub fn subscribe(&self) -> broadcast::Receiver<EventMessage> {
        self.shared_bus.subscribe()
    }
}

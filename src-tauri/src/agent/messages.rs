//! Agent 消息定义
//! 
//! 包含请求、响应、事件等消息结构

use serde::{Deserialize, Serialize};
use crate::agent::types::{AgentStatus};

/// Agent 请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    /// 请求唯一标识
    pub request_id: String,
    /// 发送请求的 Agent ID
    pub agent_id: String,
    /// 动作类型
    pub action: String,
    /// 请求载荷
    pub payload: serde_json::Value,
}

impl AgentRequest {
    /// 创建新的请求
    pub fn new(agent_id: String, action: String, payload: serde_json::Value) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            agent_id,
            action,
            payload,
        }
    }
}

/// Agent 响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    /// 响应唯一标识
    pub response_id: String,
    /// 对应的请求 ID
    pub request_id: String,
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

impl AgentResponse {
    /// 创建成功响应
    pub fn success(request_id: String, data: Option<serde_json::Value>) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: true,
            data,
            error: None,
        }
    }

    /// 创建失败响应
    pub fn error(request_id: String, error_msg: String) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: false,
            data: None,
            error: Some(error_msg),
        }
    }
}

/// 消息类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 日志消息
    Log,
    /// 状态更新
    Status,
    /// 进度更新
    Progress,
    /// 错误消息
    Error,
    /// 心跳消息
    Heartbeat,
}

/// Agent 消息 (用于实时推送)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// 消息唯一标识
    pub message_id: String,
    /// 时间戳 (Unix timestamp)
    pub timestamp: i64,
    /// 消息来源："agent" | "daemon" | "frontend"
    pub source: String,
    /// 消息类型
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// 消息内容
    pub content: String,
    /// 附加元数据
    pub metadata: Option<serde_json::Value>,
}

impl AgentMessage {
    /// 创建日志消息
    pub fn log(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Log,
            content,
            metadata: None,
        }
    }

    /// 创建进度消息
    pub fn progress(source: String, content: String, progress: f32) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Progress,
            content,
            metadata: Some(serde_json::json!({ "progress": progress })),
        }
    }

    /// 创建状态消息
    pub fn status(source: String, content: String, status: AgentStatus) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Status,
            content,
            metadata: Some(serde_json::to_value(&status).unwrap_or_default()),
        }
    }

    /// 创建错误消息
    pub fn error(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Error,
            content,
            metadata: None,
        }
    }
}

/// Stdio 管道命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioCommand {
    /// 命令 ID
    pub command_id: String,
    /// 命令类型
    #[serde(rename = "command")]
    pub cmd_type: String,
    /// 命令参数
    pub args: Vec<String>,
    /// 工作目录
    pub cwd: Option<String>,
    /// 环境变量
    pub env: Option<std::collections::HashMap<String, String>>,
}

impl StdioCommand {
    /// 创建新命令
    pub fn new(cmd_type: String, args: Vec<String>) -> Self {
        Self {
            command_id: uuid::Uuid::new_v4().to_string(),
            cmd_type,
            args,
            cwd: None,
            env: None,
        }
    }
}

/// Stdio 输出行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioOutput {
    /// 标准输出
    pub stdout: Option<String>,
    /// 标准错误
    pub stderr: Option<String>,
    /// 退出码
    pub exit_code: Option<i32>,
    /// 时间戳
    pub timestamp: i64,
}

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum WebSocketMessage {
    /// 客户端连接
    Connect {
        session_id: String,
    },
    /// 客户端断开
    Disconnect {
        session_id: String,
    },
    /// 普通消息
    Message {
        data: serde_json::Value,
    },
    /// 心跳消息
    Heartbeat {
        timestamp: i64,
    },
    /// 订阅 Agent 消息
    Subscribe {
        agent_id: String,
    },
    /// 取消订阅
    Unsubscribe {
        agent_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_request_creation() {
        let request = AgentRequest::new(
            "agent-001".to_string(),
            "initialize".to_string(),
            serde_json::json!({"project": "test"}),
        );
        
        assert!(!request.request_id.is_empty());
        assert_eq!(request.agent_id, "agent-001");
        assert_eq!(request.action, "initialize");
    }

    #[test]
    fn test_agent_response_success() {
        let response = AgentResponse::success(
            "req-001".to_string(),
            Some(serde_json::json!({"result": "ok"})),
        );
        
        assert!(response.success);
        assert!(response.error.is_none());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_agent_response_error() {
        let response = AgentResponse::error(
            "req-001".to_string(),
            "Something went wrong".to_string(),
        );
        
        assert!(!response.success);
        assert!(response.error.is_some());
        assert!(response.data.is_none());
    }

    #[test]
    fn test_agent_message_log() {
        let msg = AgentMessage::log("agent-001".to_string(), "Starting initialization...".to_string());
        
        assert_eq!(msg.message_type, MessageType::Log);
        assert_eq!(msg.source, "agent-001");
        assert_eq!(msg.content, "Starting initialization...");
    }

    #[test]
    fn test_agent_message_progress() {
        let msg = AgentMessage::progress("agent-001".to_string(), "Processing...".to_string(), 0.5);
        
        assert_eq!(msg.message_type, MessageType::Progress);
        assert!(msg.metadata.is_some());
    }

    #[test]
    fn test_stdio_command_creation() {
        let cmd = StdioCommand::new("git".to_string(), vec!["init".to_string()]);
        
        assert!(!cmd.command_id.is_empty());
        assert_eq!(cmd.cmd_type, "git");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_websocket_message_serialize() {
        let msg = WebSocketMessage::Connect {
            session_id: "session-001".to_string(),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"connect\""));
        assert!(json.contains("session-001"));
    }
}

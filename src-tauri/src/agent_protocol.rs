//! Agent 通信协议定义
//! 
//! 本模块定义了 Agent 与守护进程、前端之间的通信协议
//! 支持 Stdio 管道通信和 WebSocket 实时推送

use serde::{Deserialize, Serialize};

/// Agent 生命周期阶段
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentPhase {
    /// 初始化阶段：环境检查、任务分解
    Initializer,
    /// 编码阶段：代码生成、测试编写
    Coding,
    /// MR 创建阶段：汇总提交、创建合并请求
    MRCreation,
}

/// Agent 运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// 空闲状态，等待任务
    Idle,
    /// 正在执行任务
    Running,
    /// 已暂停
    Paused,
    /// 任务完成
    Completed,
    /// 任务失败，包含错误信息
    Failed(String),
}

/// Agent 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型："initializer" | "coding" | "mr_creation"
    #[serde(rename = "type")]
    pub agent_type: String,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 当前状态
    pub status: AgentStatus,
    /// 项目路径
    pub project_path: String,
    /// 会话 ID
    pub session_id: String,
}

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

/// Agent 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

/// 守护进程状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonState {
    /// 会话 ID
    pub session_id: String,
    /// 项目 ID
    pub project_id: String,
    /// 当前阶段
    pub current_phase: AgentPhase,
    /// 活跃的 Agent 列表
    pub active_agents: Vec<AgentStatus>,
    /// 已完成的任务/Issue 列表
    pub completed_issues: Vec<String>,
    /// 待处理的任务/Issue 列表
    pub pending_issues: Vec<String>,
    /// 日志文件路径
    pub log_file: Option<String>,
    /// 最后快照时间 (Unix timestamp)
    pub last_snapshot: i64,
    /// CPU 使用率 (0.0 - 1.0)
    pub cpu_usage: f32,
    /// 内存使用量 (字节)
    pub memory_usage: usize,
}

impl DaemonState {
    /// 创建新的状态快照
    pub fn new(session_id: String, project_id: String) -> Self {
        Self {
            session_id,
            project_id,
            current_phase: AgentPhase::Initializer,
            active_agents: Vec::new(),
            completed_issues: Vec::new(),
            pending_issues: Vec::new(),
            log_file: None,
            last_snapshot: chrono::Utc::now().timestamp(),
            cpu_usage: 0.0,
            memory_usage: 0,
        }
    }
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
    pub env: Option<HashMap<String, String>>,
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

use std::collections::HashMap;

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
        let msg = AgentMessage::progress("agent-001".to_string(), "Processing...".to_string(), 0.75);
        
        assert_eq!(msg.message_type, MessageType::Progress);
        assert!(msg.metadata.is_some());
    }

    #[test]
    fn test_daemon_state_creation() {
        let state = DaemonState::new("session-001".to_string(), "project-001".to_string());
        
        assert_eq!(state.session_id, "session-001");
        assert_eq!(state.project_id, "project-001");
        assert_eq!(state.current_phase, AgentPhase::Initializer);
        assert!(state.active_agents.is_empty());
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

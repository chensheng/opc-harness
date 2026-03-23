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

// ========== Daemon 相关类型定义 ==========

/// 守护进程运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaemonStatus {
    Starting,      // 启动中
    Running,       // 运行中
    Paused,        // 已暂停
    Stopping,      // 停止中
    Stopped,       // 已停止
    Failed(String), // 失败 (含错误信息)
}

/// 守护进程配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub session_id: String,           // 会话 ID
    pub project_path: String,         // 项目路径
    pub log_level: String,            // 日志级别：debug/info/warn/error
    pub max_concurrent_agents: usize, // 最大并发 Agent 数
    pub workspace_dir: String,        // 工作目录
}

/// Agent 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProcessInfo {
    pub agent_id: String,             // Agent 唯一标识
    pub agent_type: String,           // Agent 类型：initializer/coding/mr_creation
    pub pid: Option<u32>,             // 进程 ID
    pub status: AgentStatus,          // 运行状态
    pub started_at: i64,              // 启动时间戳
    pub resource_usage: ResourceUsage, // 资源使用情况
}

/// 资源使用情况
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_percent: f32,    // CPU 使用率 (%)
    pub memory_mb: usize,    // 内存使用量 (MB)
    pub disk_io_read: u64,   // 磁盘读取 (bytes)
    pub disk_io_write: u64,  // 磁盘写入 (bytes)
    pub network_rx: u64,     // 网络接收 (bytes)
    pub network_tx: u64,     // 网络发送 (bytes)
}

/// 守护进程状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSnapshot {
    pub daemon_id: String,              // 守护进程 ID
    pub status: DaemonStatus,           // 运行状态
    pub config: DaemonConfig,           // 配置信息
    pub active_agents: Vec<AgentProcessInfo>, // 活跃的 Agent 列表
    pub completed_tasks: Vec<String>,   // 已完成的任务列表
    pub pending_tasks: Vec<String>,     // 待处理的任务列表
    pub start_time: i64,                // 启动时间戳
    pub last_update: i64,               // 最后更新时间戳
    pub system_info: SystemInfo,        // 系统信息
}

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,                     // 操作系统
    pub arch: String,                   // 架构
    pub total_memory: u64,              // 总内存 (MB)
    pub available_memory: u64,          // 可用内存 (MB)
    pub cpu_cores: usize,               // CPU 核心数
    pub rust_version: String,           // Rust 版本
}

/// 守护进程命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {
    Start { config: DaemonConfig },     // 启动守护进程
    Stop { graceful: bool },            // 停止守护进程
    Pause,                               // 暂停
    Resume,                              // 恢复
    SpawnAgent { agent_type: String },  // 生成新的 Agent
    KillAgent { agent_id: String },     // 终止指定 Agent
    GetStatus,                           // 获取状态
    GetSnapshot,                         // 获取快照
}

/// 守护进程事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonEvent {
    Started,                             // 已启动
    Stopped,                             // 已停止
    AgentSpawned { agent_id: String },  // Agent 已生成
    AgentCompleted { agent_id: String }, // Agent 已完成
    AgentFailed { agent_id: String, error: String }, // Agent 失败
    ResourceWarning { message: String }, // 资源警告
    Error { message: String },           // 错误事件
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

/// 守护进程管理器
pub struct DaemonManager {
    daemon_id: String,
    status: DaemonStatus,
    config: Option<DaemonConfig>,
    agents: std::collections::HashMap<String, AgentProcessInfo>,
    completed_tasks: Vec<String>,
    pending_tasks: Vec<String>,
    start_time: i64,
}

impl DaemonManager {
    /// 创建新的守护进程管理器
    pub fn new() -> Self {
        Self {
            daemon_id: uuid::Uuid::new_v4().to_string(),
            status: DaemonStatus::Stopped,
            config: None,
            agents: std::collections::HashMap::new(),
            completed_tasks: Vec::new(),
            pending_tasks: Vec::new(),
            start_time: 0,
        }
    }

    /// 启动守护进程
    pub fn start(&mut self, config: DaemonConfig) -> Result<(), String> {
        if self.status == DaemonStatus::Running {
            return Err("Daemon is already running".to_string());
        }

        self.config = Some(config);
        self.status = DaemonStatus::Starting;
        self.start_time = chrono::Utc::now().timestamp();
        
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
    pub fn spawn_agent(&mut self, agent_type: &str) -> Result<String, String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        let agent_id = format!("{}-{}", agent_type, uuid::Uuid::new_v4());
        
        let agent_info = AgentProcessInfo {
            agent_id: agent_id.clone(),
            agent_type: agent_type.to_string(),
            pid: None, // TODO: 实际启动进程后设置
            status: AgentStatus::Idle,
            started_at: chrono::Utc::now().timestamp(),
            resource_usage: ResourceUsage::default(),
        };

        self.agents.insert(agent_id.clone(), agent_info);
        self.pending_tasks.push(agent_id.clone());
        
        Ok(agent_id)
    }

    /// 终止指定 Agent
    pub fn kill_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // TODO: 实际终止进程
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
        for agent in self.agents.values_mut() {
            // TODO: 使用 sysinfo crate 获取真实资源使用情况
            agent.resource_usage = ResourceUsage {
                cpu_percent: 0.0,
                memory_mb: 0,
                disk_io_read: 0,
                disk_io_write: 0,
                network_rx: 0,
                network_tx: 0,
            };
        }
    }

    /// 标记任务完成
    pub fn mark_task_completed(&mut self, task_id: &str) {
        self.pending_tasks.retain(|id| id != task_id);
        self.completed_tasks.push(task_id.to_string());
    }
}

impl Default for DaemonManager {
    fn default() -> Self {
        Self::new()
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

    // ========== Daemon 相关测试 ==========

    #[test]
    fn test_daemon_manager_creation() {
        let manager = DaemonManager::new();
        
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
        assert!(manager.agents.is_empty());
        assert!(manager.completed_tasks.is_empty());
        assert!(manager.pending_tasks.is_empty());
    }

    #[test]
    fn test_daemon_manager_start() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "debug".to_string(),
            max_concurrent_agents: 3,
            workspace_dir: "/tmp".to_string(),
        };
        
        let result = manager.start(config);
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_manager_stop() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let result = manager.stop(true);
        
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
    }

    #[test]
    fn test_daemon_manager_spawn_agent() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let agent_id = manager.spawn_agent("initializer");
        
        assert!(agent_id.is_ok());
        assert!(!agent_id.as_ref().unwrap().is_empty());
        assert_eq!(manager.agents.len(), 1);
    }

    #[test]
    fn test_daemon_manager_pause_resume() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 暂停
        let pause_result = manager.pause();
        assert!(pause_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Paused);
        
        // 恢复
        let resume_result = manager.resume();
        assert!(resume_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_snapshot() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        manager.spawn_agent("coding").unwrap();
        
        let snapshot = manager.get_snapshot();
        
        assert_eq!(snapshot.status, DaemonStatus::Running);
        assert_eq!(snapshot.active_agents.len(), 1);
        assert!(!snapshot.daemon_id.is_empty());
        assert!(snapshot.last_update > 0);
    }

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        
        assert_eq!(usage.cpu_percent, 0.0);
        assert_eq!(usage.memory_mb, 0);
        assert_eq!(usage.disk_io_read, 0);
        assert_eq!(usage.disk_io_write, 0);
    }

    #[test]
    fn test_system_info_creation() {
        let mut manager = DaemonManager::new();
        let snapshot = manager.get_snapshot();
        
        assert!(!snapshot.system_info.os.is_empty());
        assert!(!snapshot.system_info.arch.is_empty());
        assert!(snapshot.system_info.total_memory > 0);
        assert!(snapshot.system_info.cpu_cores > 0);
    }
}

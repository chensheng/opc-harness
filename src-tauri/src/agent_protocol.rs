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
    // ========== VC-013: 并发控制相关字段 ==========
    running_count: usize,                    // 当前运行中的 Agent 数量
    max_concurrent: usize,                   // 最大并发数 (从 config 同步)
    agent_queue: Vec<String>,                // 等待中的 Agent ID 队列
    running_agents: std::collections::HashSet<String>, // 正在运行的 Agent ID 集合
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
            // ========== VC-013: 并发控制初始化 ==========
            running_count: 0,
            max_concurrent: 5, // 默认值，会在 start 时被 config 覆盖
            agent_queue: Vec::new(),
            running_agents: std::collections::HashSet::new(),
        }
    }

    /// 启动守护进程
    pub fn start(&mut self, config: DaemonConfig) -> Result<(), String> {
        if self.status == DaemonStatus::Running {
            return Err("Daemon is already running".to_string());
        }

        // ========== VC-013: 初始化并发控制配置 ==========
        self.max_concurrent = config.max_concurrent_agents;
        
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

    // ========== VC-013: 并发控制核心方法 ==========

    /// 检查是否可以启动新的 Agent
    pub fn can_spawn_agent(&self) -> bool {
        self.running_count < self.max_concurrent
    }

    /// 获取可用的并发槽位数
    pub fn available_slots(&self) -> usize {
        if self.running_count >= self.max_concurrent {
            0
        } else {
            self.max_concurrent - self.running_count
        }
    }

    /// 尝试启动 Agent（受并发限制）
    /// 返回 true 表示可以立即启动，false 表示需要排队
    pub fn try_start_agent(&mut self, agent_id: &str) -> bool {
        // 检查是否已经在运行
        if self.running_agents.contains(agent_id) {
            return true;
        }

        // 检查是否有可用槽位
        if self.can_spawn_agent() {
            self.running_agents.insert(agent_id.to_string());
            self.running_count += 1;
            
            // 更新 Agent 状态为 Running
            if let Some(agent) = self.agents.get_mut(agent_id) {
                agent.status = AgentStatus::Running;
            }
            
            true
        } else {
            // 加入等待队列
            if !self.agent_queue.contains(&agent_id.to_string()) {
                self.agent_queue.push(agent_id.to_string());
                
                // 更新 Agent 状态为 Idle (等待中)
                if let Some(agent) = self.agents.get_mut(agent_id) {
                    agent.status = AgentStatus::Idle;
                }
            }
            false
        }
    }

    /// 停止 Agent 并释放槽位
    pub fn stop_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // 从运行集合中移除
        if self.running_agents.remove(agent_id) {
            self.running_count = self.running_count.saturating_sub(1);
        }

        // 更新 Agent 状态
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Completed;
        }

        // 从等待队列移除
        self.agent_queue.retain(|id| id != agent_id);

        // ========== VC-013: 调度队列中的下一个 Agent ==========
        self.schedule_next_agent();

        Ok(())
    }

    /// 调度队列中的下一个 Agent
    fn schedule_next_agent(&mut self) {
        // 检查是否有可用槽位和等待中的 Agent
        while self.can_spawn_agent() && !self.agent_queue.is_empty() {
            if let Some(next_agent_id) = self.agent_queue.first().cloned() {
                self.agent_queue.remove(0);
                
                // 启动该 Agent
                self.running_agents.insert(next_agent_id.clone());
                self.running_count += 1;
                
                // 更新 Agent 状态
                if let Some(agent) = self.agents.get_mut(&next_agent_id) {
                    agent.status = AgentStatus::Running;
                }
                
                // TODO: 实际启动 Agent 进程
            } else {
                break;
            }
        }
    }

    /// 获取当前并发统计信息
    pub fn get_concurrency_stats(&self) -> ConcurrencyStats {
        ConcurrencyStats {
            running_count: self.running_count,
            max_concurrent: self.max_concurrent,
            queued_count: self.agent_queue.len(),
            available_slots: self.available_slots(),
            utilization: if self.max_concurrent > 0 {
                (self.running_count as f32 / self.max_concurrent as f32) * 100.0
            } else {
                0.0
            },
        }
    }

    /// 动态调整最大并发数
    pub fn adjust_max_concurrent(&mut self, new_limit: usize) -> Result<(), String> {
        if new_limit == 0 {
            return Err("max_concurrent must be greater than 0".to_string());
        }

        let _old_limit = self.max_concurrent;
        self.max_concurrent = new_limit;

        // 如果新限制小于当前运行数，需要暂停部分 Agent
        if new_limit < self.running_count {
            // 暂停多余的 Agent（按启动时间排序，暂停最新的）
            let mut running_vec: Vec<_> = self.running_agents.iter().cloned().collect();
            running_vec.sort_by(|a, b| {
                let a_time = self.agents.get(a).map(|ag| ag.started_at).unwrap_or(0);
                let b_time = self.agents.get(b).map(|ag| ag.started_at).unwrap_or(0);
                b_time.cmp(&a_time) // 降序，最新的先暂停
            });

            let excess = self.running_count - new_limit;
            for agent_id in running_vec.into_iter().take(excess) {
                if let Some(agent) = self.agents.get_mut(&agent_id) {
                    if agent.status == AgentStatus::Running {
                        agent.status = AgentStatus::Paused;
                    }
                }
                self.running_agents.remove(&agent_id);
                self.agent_queue.push(agent_id);
            }
            
            self.running_count = new_limit;
        } else {
            // 如果新限制更大，尝试启动更多 Agent
            self.schedule_next_agent();
        }

        Ok(())
    }

    /// 获取所有等待中的 Agent
    pub fn get_queued_agents(&self) -> Vec<&String> {
        self.agent_queue.iter().collect()
    }

    /// 获取所有运行中的 Agent
    pub fn get_running_agents(&self) -> Vec<&String> {
        self.running_agents.iter().collect()
    }
}

/// 并发统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyStats {
    pub running_count: usize,      // 当前运行数
    pub max_concurrent: usize,     // 最大并发数
    pub queued_count: usize,       // 等待队列长度
    pub available_slots: usize,    // 可用槽位数
    pub utilization: f32,          // 并发利用率 (%)
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

    // ========== Daemon 相关测试 ==========

    #[test]
    fn test_daemon_manager_creation() {
        let manager = DaemonManager::new();
        
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
        assert!(manager.agents.is_empty());
        assert!(manager.completed_tasks.is_empty());
        assert!(manager.pending_tasks.is_empty());
        // ========== VC-013: 验证并发控制字段初始化 ==========
        assert_eq!(manager.running_count, 0);
        assert_eq!(manager.max_concurrent, 5); // 默认值
        assert!(manager.agent_queue.is_empty());
        assert!(manager.running_agents.is_empty());
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

    // ========== VC-013: 并发控制测试 ==========

    #[test]
    fn test_concurrency_config_initialization() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 0);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_can_spawn_agent_when_slots_available() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 初始时有可用槽位
        assert!(manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 2);
    }

    #[test]
    fn test_cannot_spawn_agent_when_slots_full() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，占满槽位
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        // 此时不能再启动新 Agent
        assert!(!manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 0);
    }

    #[test]
    fn test_agent_queuing_when_concurrent_limit_reached() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        // 第 3 个 Agent 需要排队
        let agent3 = manager.spawn_agent("coding").unwrap();
        let should_start = manager.try_start_agent(&agent3);
        
        assert!(!should_start); // 不能立即启动
        assert_eq!(manager.agent_queue.len(), 1);
        assert!(manager.agent_queue.contains(&agent3));
    }

    #[test]
    fn test_auto_schedule_next_agent_on_completion() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，第 3 个排队
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3); // 会进入队列
        
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 1);
        
        // 完成 agent1，agent3 应该自动启动
        manager.stop_agent(&agent1).unwrap();
        
        assert_eq!(manager.running_count, 2); // 保持 2 个运行
        assert!(manager.agent_queue.is_empty()); // 队列清空
    }

    #[test]
    fn test_concurrency_stats() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        let stats = manager.get_concurrency_stats();
        
        assert_eq!(stats.running_count, 2);
        assert_eq!(stats.max_concurrent, 4);
        assert_eq!(stats.queued_count, 0);
        assert_eq!(stats.available_slots, 2);
        assert!((stats.utilization - 50.0).abs() < 0.1); // 50% 利用率
    }

    #[test]
    fn test_adjust_max_concurrent_increase() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，第 3 个排队
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        // 提高并发限制
        manager.adjust_max_concurrent(4).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 3); // agent3 应该自动启动
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_adjust_max_concurrent_decrease() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 4 个 Agent
        for _ in 0..4 {
            let agent_id = manager.spawn_agent("coding").unwrap();
            manager.try_start_agent(&agent_id);
        }
        
        assert_eq!(manager.running_count, 4);
        
        // 降低并发限制到 2
        manager.adjust_max_concurrent(2).unwrap();
        
        assert_eq!(manager.max_concurrent, 2);
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 2); // 2 个被暂停的进入队列
    }

    #[test]
    fn test_get_running_and_queued_agents() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        let running = manager.get_running_agents();
        let queued = manager.get_queued_agents();
        
        assert_eq!(running.len(), 2);
        assert_eq!(queued.len(), 1);
    }

    #[test]
    fn test_adjust_max_concurrent_zero_error() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        // 启动守护进程
        let result = manager.start(config);
        assert!(result.is_ok());
        
        // 验证不能设置为 0
        let adjust_result = manager.adjust_max_concurrent(0);
        assert!(adjust_result.is_err());
        assert_eq!(adjust_result.unwrap_err(), "max_concurrent must be greater than 0");
    }

}

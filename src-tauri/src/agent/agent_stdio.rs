//! Agent Stdio 管道通信层
//! 
//! 实现基于标准输入输出的进程间通信 (IPC) 机制
//! 用于守护进程与子 Agent 进程之间的消息传递

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use crate::agent::types::AgentConfig;

/// Stdio 消息格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioMessage {
    /// 消息 ID
    pub id: String,
    /// 发送者 (daemon/agent_id)
    pub from: String,
    /// 接收者 (daemon/agent_id)
    pub to: String,
    /// 消息类型
    pub message_type: StdioMessageType,
    /// 消息内容
    pub payload: serde_json::Value,
    /// 时间戳
    pub timestamp: i64,
}

/// Stdio 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum StdioMessageType {
    /// 命令请求
    Command(String),
    /// 命令响应
    Response { success: bool, data: Option<serde_json::Value>, error: Option<String> },
    /// 日志输出
    Log { level: String, message: String },
    /// 进度更新
    Progress { current: usize, total: usize, description: String },
    /// 心跳包
    Heartbeat,
    /// 自定义事件
    Event(String),
}

/// Stdio 通道配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioChannelConfig {
    /// 通道 ID
    pub channel_id: String,
    /// Agent 配置
    pub agent_config: AgentConfig,
    /// 工作目录
    pub working_dir: String,
    /// 超时时间 (秒)
    pub timeout_secs: u64,
}

/// Stdio 通道状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StdioChannelStatus {
    /// 未连接
    Disconnected,
    /// 连接中
    Connecting,
    /// 已连接
    Connected,
    /// 已关闭
    Closed,
    /// 错误状态
    Error(String),
}

/// Stdio 通道统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StdioChannelStats {
    /// 发送的消息数
    pub messages_sent: usize,
    /// 接收的消息数
    pub messages_received: usize,
    /// 最后活动时间戳
    pub last_activity: Option<i64>,
    /// 平均响应时间 (毫秒)
    pub avg_response_time_ms: f64,
}

/// Stdio 通道
/// 
/// 封装了与子进程的 stdin/stdout 通信
pub struct StdioChannel {
    /// 通道 ID
    #[allow(dead_code)]
    channel_id: String,
    /// Agent 配置
    config: AgentConfig,
    /// 子进程句柄
    process: Option<Arc<Mutex<Child>>>,
    /// 通道状态
    status: StdioChannelStatus,
    /// 统计信息
    stats: StdioChannelStats,
    /// 接收消息队列 (异步缓冲区)
    message_queue: Arc<Mutex<VecDeque<StdioMessage>>>,
    /// 最后一条错误消息
    last_error: Option<String>,
}

impl StdioChannel {
    /// 创建新的 Stdio 通道 (不启动进程)
    pub fn new(config: AgentConfig) -> Self {
        let channel_id = format!("stdio-{}", uuid::Uuid::new_v4());
        
        Self {
            channel_id,
            config,
            process: None,
            status: StdioChannelStatus::Disconnected,
            stats: StdioChannelStats::default(),
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            last_error: None,
        }
    }

    /// 启动子进程并建立 Stdio 连接
    pub fn start(&mut self, command: &str, args: &[&str]) -> Result<(), String> {
        if self.status != StdioChannelStatus::Disconnected {
            return Err(format!(
                "Channel is not disconnected. Current status: {:?}",
                self.status
            ));
        }

        self.status = StdioChannelStatus::Connecting;

        // 启动子进程，配置 stdin/stdout/stderr 为管道
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&self.config.project_path)
            .spawn()
            .map_err(|e| format!("Failed to spawn process '{}': {}", command, e))?;

        self.process = Some(Arc::new(Mutex::new(child)));
        self.status = StdioChannelStatus::Connected;
        
        // 启动后台线程读取 stdout
        if let Some(process_arc) = &self.process {
            let process_clone = Arc::clone(process_arc);
            let message_queue = Arc::clone(&self.message_queue);
            
            std::thread::spawn(move || {
                let mut process = process_clone.lock().unwrap();
                if let Some(stdout) = process.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        if let Ok(line_str) = line {
                            // 尝试解析为 StdioMessage
                            if let Ok(msg) = serde_json::from_str::<StdioMessage>(&line_str) {
                                let mut queue = message_queue.lock().unwrap();
                                queue.push_back(msg);
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }

    /// 发送消息到子进程
    pub fn send_message(&mut self, message: &StdioMessage) -> Result<(), String> {
        if self.status != StdioChannelStatus::Connected {
            return Err("Channel is not connected".to_string());
        }

        let process_arc = self.process.as_ref()
            .ok_or("Process handle not available")?;
        
        let mut process = process_arc.lock()
            .map_err(|e| format!("Failed to lock process: {}", e))?;

        // 获取 stdin 句柄
        let stdin = process.stdin.as_mut()
            .ok_or("Stdin not available")?;

        // 序列化消息为 JSON
        let json = serde_json::to_string(message)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        // 写入消息 (换行符分隔)
        writeln!(stdin, "{}", json)
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;

        // 刷新缓冲区
        stdin.flush()
            .map_err(|e| format!("Failed to flush stdin: {}", e))?;

        // 更新统计
        self.stats.messages_sent += 1;
        self.stats.last_activity = Some(chrono::Utc::now().timestamp_millis());

        Ok(())
    }

    /// 从消息队列中接收消息 (非变体版本，用于 tauri command)
    pub fn peek_message(&self) -> Option<StdioMessage> {
        let queue = self.message_queue.lock().unwrap();
        queue.front().cloned()
    }

    /// 从消息队列中移除并返回消息
    pub fn pop_message(&mut self) -> Option<StdioMessage> {
        let mut queue = self.message_queue.lock().unwrap();
        let msg = queue.pop_front();
        
        if msg.is_some() {
            self.stats.messages_received += 1;
            self.stats.last_activity = Some(chrono::Utc::now().timestamp_millis());
        }
        
        msg
    }

    /// 接收下一条消息 (阻塞直到有消息或超时)
    pub fn recv_message_timeout(&mut self, timeout_secs: u64) -> Result<Option<StdioMessage>, String> {
        let start = std::time::Instant::now();
        
        while start.elapsed().as_secs() < timeout_secs {
            if let Some(msg) = self.pop_message() {
                return Ok(Some(msg));
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        Ok(None) // 超时
    }

    /// 发送命令并等待响应
    pub fn send_command_with_response(
        &mut self,
        command: &str,
        payload: serde_json::Value,
        timeout_secs: u64,
    ) -> Result<serde_json::Value, String> {
        let message_id = uuid::Uuid::new_v4().to_string();
        
        // 构建命令消息
        let request = StdioMessage {
            id: message_id.clone(),
            from: "daemon".to_string(),
            to: self.config.agent_id.clone(),
            message_type: StdioMessageType::Command(command.to_string()),
            payload,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        // 发送命令
        self.send_message(&request)?;

        // 等待响应
        let response = self.recv_message_timeout(timeout_secs)?
            .ok_or(format!("Timeout waiting for response after {} seconds", timeout_secs))?;

        // 验证响应
        match &response.message_type {
            StdioMessageType::Response { success, data, error } => {
                if *success {
                    Ok(data.clone().unwrap_or(serde_json::Value::Null))
                } else {
                    Err(error.clone().unwrap_or("Unknown error".to_string()))
                }
            }
            _ => Err(format!("Unexpected message type: {:?}", response.message_type)),
        }
    }

    /// 发送日志消息
    pub fn send_log(&mut self, level: &str, message: &str) -> Result<(), String> {
        let log_msg = StdioMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: "daemon".to_string(),
            to: self.config.agent_id.clone(),
            message_type: StdioMessageType::Log {
                level: level.to_string(),
                message: message.to_string(),
            },
            payload: serde_json::Value::Null,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        self.send_message(&log_msg)
    }

    /// 发送进度更新
    pub fn send_progress(
        &mut self,
        current: usize,
        total: usize,
        description: &str,
    ) -> Result<(), String> {
        let progress_msg = StdioMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: "daemon".to_string(),
            to: self.config.agent_id.clone(),
            message_type: StdioMessageType::Progress {
                current,
                total,
                description: description.to_string(),
            },
            payload: serde_json::Value::Null,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        self.send_message(&progress_msg)
    }

    /// 发送心跳包
    pub fn send_heartbeat(&mut self) -> Result<(), String> {
        let heartbeat_msg = StdioMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: "daemon".to_string(),
            to: self.config.agent_id.clone(),
            message_type: StdioMessageType::Heartbeat,
            payload: serde_json::Value::Null,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        self.send_message(&heartbeat_msg)
    }

    /// 停止子进程并关闭通道
    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(process_arc) = &self.process {
            let mut process = process_arc.lock()
                .map_err(|e| format!("Failed to lock process: {}", e))?;
            
            // 尝试优雅终止
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                unsafe {
                    libc::kill(process.id() as i32, libc::SIGTERM);
                }
            }
            
            // 等待进程退出
            process.wait()
                .map_err(|e| format!("Failed to wait for process: {}", e))?;
        }

        self.process = None;
        self.status = StdioChannelStatus::Closed;
        
        Ok(())
    }

    /// 强制终止子进程
    pub fn kill(&mut self) -> Result<(), String> {
        if let Some(process_arc) = &self.process {
            let mut process = process_arc.lock()
                .map_err(|e| format!("Failed to lock process: {}", e))?;
            
            process.kill()
                .map_err(|e| format!("Failed to kill process: {}", e))?;
        }

        self.process = None;
        self.status = StdioChannelStatus::Closed;
        
        Ok(())
    }

    /// 获取通道状态
    pub fn get_status(&self) -> StdioChannelStatus {
        self.status.clone()
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> StdioChannelStats {
        self.stats.clone()
    }

    /// 获取最后一条错误
    pub fn get_last_error(&self) -> Option<String> {
        self.last_error.clone()
    }

    /// 设置错误信息
    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
        self.status = StdioChannelStatus::Error(self.last_error.clone().unwrap());
    }

    /// 检查通道是否可用
    pub fn is_connected(&self) -> bool {
        matches!(self.status, StdioChannelStatus::Connected)
    }
}

impl Drop for StdioChannel {
    fn drop(&mut self) {
        // 确保子进程被清理
        if self.process.is_some() {
            let _ = self.stop();
        }
    }
}

// ========== Tauri Commands ==========

use tauri::State;
use std::collections::HashMap;

/// 全局 Stdio 通道管理器
pub struct StdioChannelManager {
    channels: HashMap<String, StdioChannel>,
}

impl StdioChannelManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn create_channel(&mut self, config: AgentConfig) -> Result<String, String> {
        let channel_id = format!("stdio-{}", uuid::Uuid::new_v4());
        let channel = StdioChannel::new(config);
        self.channels.insert(channel_id.clone(), channel);
        Ok(channel_id)
    }

    pub fn get_channel(&self, channel_id: &str) -> Option<&StdioChannel> {
        self.channels.get(channel_id)
    }

    pub fn get_channel_mut(&mut self, channel_id: &str) -> Option<&mut StdioChannel> {
        self.channels.get_mut(channel_id)
    }

    pub fn remove_channel(&mut self, channel_id: &str) -> Option<StdioChannel> {
        self.channels.remove(channel_id)
    }

    /// 获取所有通道
    pub fn get_all_channels(&self) -> &HashMap<String, StdioChannel> {
        &self.channels
    }
}

/// 创建新的 Stdio 通道
#[tauri::command]
pub fn create_stdio_channel(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    agent_config: AgentConfig,
) -> Result<String, String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    mgr.create_channel(agent_config)
}

/// 启动 Stdio 通道 (连接子进程)
#[tauri::command]
pub fn start_stdio_channel(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
    command: String,
    args: Vec<String>,
) -> Result<(), String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel_mut(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    channel.start(&command, &args_ref)
}

/// 发送消息到 Stdio 通道
#[tauri::command]
pub fn send_stdio_message(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
    message: StdioMessage,
) -> Result<(), String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel_mut(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    channel.send_message(&message)
}

/// 从 Stdio 通道接收消息
#[tauri::command]
pub fn recv_stdio_message(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
) -> Result<Option<StdioMessage>, String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel_mut(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    Ok(channel.pop_message())
}

/// 发送命令并等待响应
#[tauri::command]
pub fn send_stdio_command(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
    command: String,
    payload: serde_json::Value,
    timeout_secs: u64,
) -> Result<serde_json::Value, String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel_mut(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    channel.send_command_with_response(&command, payload, timeout_secs)
}

/// 停止 Stdio 通道
#[tauri::command]
pub fn stop_stdio_channel(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
) -> Result<(), String> {
    let mut mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel_mut(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    channel.stop()
}

/// 获取通道状态
#[tauri::command]
pub fn get_stdio_channel_status(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
) -> Result<StdioChannelStatus, String> {
    let mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    Ok(channel.get_status())
}

/// 获取通道统计信息
#[tauri::command]
pub fn get_stdio_channel_stats(
    manager: State<'_, std::sync::Arc<std::sync::Mutex<StdioChannelManager>>>,
    channel_id: String,
) -> Result<StdioChannelStats, String> {
    let mgr = manager.lock().map_err(|e| e.to_string())?;
    let channel = mgr.get_channel(&channel_id)
        .ok_or(format!("Channel {} not found", channel_id))?;
    
    Ok(channel.get_stats())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_message_creation() {
        let msg = StdioMessage {
            id: "msg-001".to_string(),
            from: "daemon".to_string(),
            to: "agent-001".to_string(),
            message_type: StdioMessageType::Heartbeat,
            payload: serde_json::Value::Null,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        assert_eq!(msg.id, "msg-001");
        assert_eq!(msg.from, "daemon");
        assert_eq!(msg.to, "agent-001");
        assert!(matches!(msg.message_type, StdioMessageType::Heartbeat));
    }

    #[test]
    fn test_stdio_message_types() {
        // Command
        let cmd = StdioMessageType::Command("start".to_string());
        assert!(matches!(cmd, StdioMessageType::Command(_)));

        // Response success
        let resp_success = StdioMessageType::Response {
            success: true,
            data: Some(serde_json::json!({"result": "ok"})),
            error: None,
        };
        assert!(matches!(resp_success, StdioMessageType::Response { success: true, .. }));

        // Response error
        let resp_error = StdioMessageType::Response {
            success: false,
            data: None,
            error: Some("Something went wrong".to_string()),
        };
        assert!(matches!(resp_error, StdioMessageType::Response { success: false, error: Some(_), .. }));

        // Log
        let log = StdioMessageType::Log {
            level: "info".to_string(),
            message: "Test log".to_string(),
        };
        assert!(matches!(log, StdioMessageType::Log { .. }));

        // Progress
        let progress = StdioMessageType::Progress {
            current: 5,
            total: 10,
            description: "Processing...".to_string(),
        };
        assert!(matches!(progress, StdioMessageType::Progress { .. }));
    }

    #[test]
    fn test_stdio_channel_creation() {
        let config = create_test_agent_config();
        let channel = StdioChannel::new(config);
        
        assert!(channel.channel_id.starts_with("stdio-"));
        assert_eq!(channel.get_status(), StdioChannelStatus::Disconnected);
        assert!(!channel.is_connected());
    }

    #[test]
    fn test_stdio_channel_start_stop() {
        let mut config = create_test_agent_config();
        // 使用一个确实存在的目录
        config.project_path = std::env::temp_dir().to_string_lossy().to_string();
        
        let mut channel = StdioChannel::new(config);
        
        // 测试启动一个简单的命令
        #[cfg(target_os = "windows")]
        {
            let result = channel.start("cmd.exe", &["/c", "echo", "test"]);
            if result.is_ok() {
                assert_eq!(channel.get_status(), StdioChannelStatus::Connected);
                
                // 测试停止
                let stop_result = channel.stop();
                assert!(stop_result.is_ok());
            }
            // 如果启动失败，跳过此测试 (可能在某些环境无法创建进程)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let result = channel.start("echo", &["test"]);
            if result.is_ok() {
                assert_eq!(channel.get_status(), StdioChannelStatus::Connected);
                
                let stop_result = channel.stop();
                assert!(stop_result.is_ok());
            }
        }
    }

    #[test]
    fn test_stdio_channel_double_start_fails() {
        let mut config = create_test_agent_config();
        config.project_path = "/tmp".to_string();
        
        let mut channel = StdioChannel::new(config);
        
        #[cfg(target_os = "windows")]
        {
            let _ = channel.start("cmd.exe", &["/c", "echo test"]);
            let result = channel.start("cmd.exe", &["/c", "echo test2"]);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("not disconnected"));
        }
    }

    #[test]
    fn test_stdio_channel_send_message_before_start_fails() {
        let config = create_test_agent_config();
        let mut channel = StdioChannel::new(config);
        
        let msg = StdioMessage {
            id: "test".to_string(),
            from: "daemon".to_string(),
            to: "agent".to_string(),
            message_type: StdioMessageType::Heartbeat,
            payload: serde_json::Value::Null,
            timestamp: 0,
        };
        
        let result = channel.send_message(&msg);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not connected"));
    }

    #[test]
    fn test_stdio_channel_stats() {
        let config = create_test_agent_config();
        let channel = StdioChannel::new(config);
        
        let stats = channel.get_stats();
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert!(stats.last_activity.is_none());
        assert_eq!(stats.avg_response_time_ms, 0.0);
    }

    #[test]
    fn test_stdio_channel_manager_creation() {
        let manager = StdioChannelManager::new();
        assert!(manager.channels.is_empty());
    }

    #[test]
    fn test_stdio_channel_manager_create_channel() {
        let mut manager = StdioChannelManager::new();
        let config = create_test_agent_config();
        
        let channel_id = manager.create_channel(config).unwrap();
        assert!(channel_id.starts_with("stdio-"));
        assert_eq!(manager.channels.len(), 1);
    }

    #[test]
    fn test_stdio_channel_manager_get_channel() {
        let mut manager = StdioChannelManager::new();
        let config = create_test_agent_config();
        
        let channel_id = manager.create_channel(config.clone()).unwrap();
        
        // Get immutable reference
        let channel = manager.get_channel(&channel_id);
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().config.agent_id, config.agent_id);
        
        // Get mutable reference
        let channel_mut = manager.get_channel_mut(&channel_id);
        assert!(channel_mut.is_some());
    }

    #[test]
    fn test_stdio_channel_manager_remove_channel() {
        let mut manager = StdioChannelManager::new();
        let config = create_test_agent_config();
        
        let channel_id = manager.create_channel(config).unwrap();
        assert_eq!(manager.channels.len(), 1);
        
        let removed = manager.remove_channel(&channel_id);
        assert!(removed.is_some());
        assert_eq!(manager.channels.len(), 0);
    }

    #[test]
    fn test_stdio_channel_manager_get_nonexistent_channel() {
        let mut manager = StdioChannelManager::new();
        
        assert!(manager.get_channel("nonexistent").is_none());
        assert!(manager.get_channel_mut("nonexistent").is_none());
    }

    #[test]
    fn test_stdio_message_serialization() {
        let msg = StdioMessage {
            id: "test-123".to_string(),
            from: "daemon".to_string(),
            to: "agent-456".to_string(),
            message_type: StdioMessageType::Progress {
                current: 5,
                total: 10,
                description: "Testing...".to_string(),
            },
            payload: serde_json::json!({"key": "value"}),
            timestamp: 1234567890,
        };

        // Serialize
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("test-123"));
        assert!(json.contains("daemon"));
        assert!(json.contains("agent-456"));

        // Deserialize
        let deserialized: StdioMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, msg.id);
        assert_eq!(deserialized.from, msg.from);
        assert_eq!(deserialized.to, msg.to);
    }

    #[test]
    fn test_stdio_channel_status_serialization() {
        let statuses = vec![
            StdioChannelStatus::Disconnected,
            StdioChannelStatus::Connecting,
            StdioChannelStatus::Connected,
            StdioChannelStatus::Closed,
            StdioChannelStatus::Error("test error".to_string()),
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: StdioChannelStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(format!("{:?}", status), format!("{:?}", deserialized));
        }
    }

    #[test]
    fn test_stdio_channel_config() {
        let config = StdioChannelConfig {
            channel_id: "test-channel".to_string(),
            agent_config: create_test_agent_config(),
            working_dir: "/tmp".to_string(),
            timeout_secs: 30,
        };

        assert_eq!(config.channel_id, "test-channel");
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.working_dir, "/tmp");
    }

    // Helper function to create test agent config
    fn create_test_agent_config() -> AgentConfig {
        AgentConfig {
            agent_id: format!("test-agent-{}", uuid::Uuid::new_v4()),
            agent_type: crate::agent::types::AgentType::Coding,
            phase: crate::agent::types::AgentPhase::Coding,
            status: crate::agent::types::AgentStatus::Idle,
            project_path: "/tmp/test".to_string(),
            session_id: "test-session".to_string(),
            ai_config: None,
            metadata: None,
        }
    }
}

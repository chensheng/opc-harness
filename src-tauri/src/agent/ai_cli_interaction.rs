//! AI CLI 交互管理器
//!
//! 负责与 Kimi/Claude Code 等 AI CLI 工具进行 STDIO 双向通信
//!
//! **DEPRECATED**: 此模块用于传统的 CLI-based Agent，已被 Native Coding Agent 取代。
//! Native Agent 直接调用 AI Provider API，无需通过 CLI 工具。
//! 
//! 迁移指南：
//! - 新功能应使用 `native_coding_agent.rs`
//! - CLI Agent 作为降级方案保留，但不再主动开发
//! - 计划在未来版本中完全移除此模块

#[deprecated(
    since = "0.2.0",
    note = "Use NativeCodingAgent instead. CLI-based agents are deprecated in favor of native Rust implementation."
)]

use serde_json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Child as TokioChild;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

/// AI CLI 输出的消息类型
#[derive(Debug, Clone)]
pub enum AICLIMessage {
    /// 标准输出 (AI 的思考过程、进度更新)
    Stdout(String),
    /// 标准错误 (错误信息、警告)
    Stderr(String),
    /// AI 生成的代码片段
    GeneratedCode { file_path: String, content: String },
    /// 任务完成信号
    TaskCompleted { success: bool, summary: String },
}

/// AI CLI 交互管理器
#[derive(Clone)]
pub struct AICLIInteraction {
    /// 子进程句柄 (使用 tokio::process::Child 以支持异步 IO)
    child: Arc<Mutex<Option<TokioChild>>>,
    /// 消息发送通道
    message_tx: mpsc::Sender<AICLIMessage>,
    /// Agent ID
    agent_id: String,
}

impl AICLIInteraction {
    /// 创建新的 AI CLI 交互管理器
    pub fn new(
        child: TokioChild,
        agent_id: String,
        message_tx: mpsc::Sender<AICLIMessage>,
    ) -> Self {
        Self {
            child: Arc::new(Mutex::new(Some(child))),
            message_tx,
            agent_id,
        }
    }

    /// 启动异步监听任务,实时读取 AI CLI 的输出 (带超时)
    pub async fn start_listening_with_timeout(&self, timeout_secs: u64) -> Result<(), String> {
        use tokio::time::{timeout, Duration};

        let timeout_duration = Duration::from_secs(timeout_secs);

        log::info!(
            "[AICLIInteraction] Starting to listen for agent {} with {}s timeout",
            self.agent_id,
            timeout_secs
        );

        // 使用 timeout 包装监听任务
        match timeout(timeout_duration, self.start_listening()).await {
            Ok(result) => result,
            Err(_) => {
                log::error!(
                    "[AICLI:{}] Agent process timed out after {} seconds",
                    self.agent_id,
                    timeout_secs
                );

                // 发送超时消息
                let _ = self
                    .message_tx
                    .send(AICLIMessage::TaskCompleted {
                        success: false,
                        summary: format!("Agent process timed out after {} seconds", timeout_secs),
                    })
                    .await;

                // 尝试终止进程
                let _ = self.terminate().await;

                Err(format!(
                    "Agent process timed out after {} seconds",
                    timeout_secs
                ))
            }
        }
    }

    /// 启动异步监听任务,实时读取 AI CLI 的输出
    pub async fn start_listening(&self) -> Result<(), String> {
        let child_arc = self.child.clone();
        let message_tx = self.message_tx.clone();
        let agent_id = self.agent_id.clone();

        log::info!(
            "[AICLIInteraction] Starting to listen for agent {} output",
            agent_id
        );

        // 在后台任务中读取 STDOUT 和 STDERR
        tokio::spawn(async move {
            let mut child_guard = child_arc.lock().await;

            if let Some(ref mut child) = *child_guard {
                let stdout = child.stdout.take();
                let stderr = child.stderr.take();

                // 异步读取 STDOUT
                if let Some(stdout) = stdout {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        log::debug!("[AICLI:{}] STDOUT: {}", agent_id, line);

                        // 尝试解析代码生成
                        if let Some(code_info) = Self::parse_generated_code(&line) {
                            let _ = message_tx
                                .send(AICLIMessage::GeneratedCode {
                                    file_path: code_info.file_path,
                                    content: code_info.content,
                                })
                                .await;
                        } else {
                            let _ = message_tx.send(AICLIMessage::Stdout(line)).await;
                        }
                    }
                }

                // 异步读取 STDERR
                if let Some(stderr) = stderr {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        log::warn!("[AICLI:{}] STDERR: {}", agent_id, line);
                        let _ = message_tx.send(AICLIMessage::Stderr(line)).await;
                    }
                }

                // 等待进程结束
                match child.wait().await {
                    Ok(status) => {
                        let success = status.success();
                        let _ = message_tx
                            .send(AICLIMessage::TaskCompleted {
                                success,
                                summary: format!("Process exited with status: {:?}", status),
                            })
                            .await;

                        log::info!(
                            "[AICLI:{}] Agent process completed with status: {:?}",
                            agent_id,
                            status
                        );
                    }
                    Err(e) => {
                        let _ = message_tx
                            .send(AICLIMessage::TaskCompleted {
                                success: false,
                                summary: format!("Failed to wait for process: {}", e),
                            })
                            .await;

                        log::error!(
                            "[AICLI:{}] Failed to wait for agent process: {}",
                            agent_id,
                            e
                        );
                    }
                }
            }
        });

        Ok(())
    }

    /// 向 AI CLI 发送指令 (通过 STDIN)
    pub async fn send_command(&self, command: &str) -> Result<(), String> {
        let mut child_guard = self.child.lock().await;

        if let Some(ref mut child) = *child_guard {
            if let Some(mut stdin) = child.stdin.take() {
                use tokio::io::AsyncWriteExt;
                stdin
                    .write_all(command.as_bytes())
                    .await
                    .map_err(|e| format!("Failed to write to stdin: {}", e))?;
                stdin
                    .write_all(b"\n")
                    .await
                    .map_err(|e| format!("Failed to write newline: {}", e))?;

                // 恢复 stdin
                child.stdin = Some(stdin);

                log::debug!("[AICLI:{}] Sent command: {}", self.agent_id, command);
                Ok(())
            } else {
                Err("STDIN not available".to_string())
            }
        } else {
            Err("Child process not found".to_string())
        }
    }

    /// 终止 AI CLI 进程
    pub async fn terminate(&self) -> Result<(), String> {
        let mut child_guard = self.child.lock().await;

        if let Some(ref mut child) = *child_guard {
            child
                .kill()
                .await
                .map_err(|e| format!("Failed to kill process: {}", e))?;
            log::info!("[AICLI:{}] Agent process terminated", self.agent_id);
            Ok(())
        } else {
            Err("Child process not found".to_string())
        }
    }

    /// 解析 AI 输出的代码生成标记
    ///
    /// 支持多种格式:
    /// 1. [GENERATED_CODE] file_path content
    /// 2. ```file_path\ncontent\n```
    /// 3. JSON 格式: {"file": "path", "code": "content"}
    fn parse_generated_code(output: &str) -> Option<GeneratedCodeInfo> {
        // 格式 1: [GENERATED_CODE] marker
        if output.starts_with("[GENERATED_CODE]") {
            let parts: Vec<&str> = output.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let file_path = parts[1].to_string();
                let content = parts[2].to_string();
                return Some(GeneratedCodeInfo { file_path, content });
            }
        }

        // 格式 2: Markdown code block (常见于 Claude Code)
        // ```src/main.rs
        // fn main() {}
        // ```
        if output.starts_with("```") && output.contains('\n') {
            let lines: Vec<&str> = output.lines().collect();
            if lines.len() >= 2 {
                // 第一行: ```file_path 或 ```file_path language
                let first_line = lines[0].trim_start_matches('`').trim();
                let file_path = first_line.split_whitespace().next()?.to_string();

                // 最后一行应该是 ```
                if lines.last()?.trim() == "```" {
                    // 提取中间的内容
                    let content_lines = &lines[1..lines.len() - 1];
                    let content = content_lines.join("\n");
                    return Some(GeneratedCodeInfo { file_path, content });
                }
            }
        }

        // 格式 3: JSON 格式 (如果 AI 输出结构化数据)
        if output.starts_with('{') && output.ends_with('}') {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
                if let (Some(file), Some(code)) = (json.get("file"), json.get("code")) {
                    if let (Some(file_path), Some(content)) = (file.as_str(), code.as_str()) {
                        return Some(GeneratedCodeInfo {
                            file_path: file_path.to_string(),
                            content: content.to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}

/// 生成的代码信息
struct GeneratedCodeInfo {
    file_path: String,
    content: String,
}

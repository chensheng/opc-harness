//! CLI工具服务
//!
//! 管理AI编码工具CLI的进程和会话

use crate::models::{CLISession, CLISessionStatus};
use anyhow::Result;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

/// CLI服务
pub struct CLIService {
    sessions: HashMap<String, CLISessionHandle>,
}

/// CLI会话句柄
pub struct CLISessionHandle {
    pub session: CLISession,
    pub process: Child,
    pub stdin: tokio::process::ChildStdin,
    pub stdout_rx: mpsc::Receiver<String>,
    pub stderr_rx: mpsc::Receiver<String>,
}

impl CLIService {
    /// 创建新的CLI服务
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// 创建新的CLI会话
    pub async fn create_session(
        &mut self,
        tool_type: &str,
        project_path: &str,
    ) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();

        // 根据工具类型确定命令
        let command = match tool_type {
            "kimi" => "kimi",
            "claude" => "claude",
            "codex" => "codex",
            _ => return Err(anyhow::anyhow!("Unknown tool type: {}", tool_type)),
        };

        // 启动进程
        let mut cmd = Command::new(command);
        cmd.current_dir(project_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut process = cmd.spawn()?;

        let stdin = process.stdin.take().unwrap();
        let stdout = process.stdout.take().unwrap();
        let stderr = process.stderr.take().unwrap();

        // 创建输出通道
        let (stdout_tx, stdout_rx) = mpsc::channel(100);
        let (stderr_tx, stderr_rx) = mpsc::channel(100);

        // 启动输出读取任务
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        Self::spawn_output_reader(stdout_reader, stdout_tx);
        Self::spawn_output_reader(stderr_reader, stderr_tx);

        let session = CLISession {
            id: session_id.clone(),
            project_id: "".to_string(), // TODO: 从参数获取
            tool_type: tool_type.to_string(),
            working_directory: project_path.to_string(),
            status: CLISessionStatus::Running,
            start_time: chrono::Utc::now().to_rfc3339(),
            end_time: None,
        };

        let handle = CLISessionHandle {
            session,
            process,
            stdin,
            stdout_rx,
            stderr_rx,
        };

        self.sessions.insert(session_id.clone(), handle);

        Ok(session_id)
    }

    /// 发送命令到CLI
    pub async fn send_command(&mut self, session_id: &str, command: &str) -> Result<()> {
        let handle = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;

        let formatted = format!("{}\n", command);
        handle.stdin.write_all(formatted.as_bytes()).await?;
        handle.stdin.flush().await?;

        Ok(())
    }

    /// 读取输出
    pub async fn read_output(&mut self, session_id: &str) -> Result<(Option<String>, Option<String>)> {
        let handle = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;

        let stdout = handle.stdout_rx.try_recv().ok();
        let stderr = handle.stderr_rx.try_recv().ok();

        Ok((stdout, stderr))
    }

    /// 停止会话
    pub async fn stop_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(mut handle) = self.sessions.remove(session_id) {
            handle.process.kill().await?;
        }
        Ok(())
    }

    /// 获取所有会话
    pub fn get_sessions(&self) -> Vec<&CLISession> {
        self.sessions.values().map(|h| &h.session).collect()
    }

    /// 生成输出读取任务
    fn spawn_output_reader<R: AsyncBufReadExt + Unpin + Send + 'static>(
        reader: R,
        tx: mpsc::Sender<String>,
    ) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if tx.send(line.trim().to_string()).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}

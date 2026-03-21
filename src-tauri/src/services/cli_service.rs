//! CLI service

use crate::models::{CLISession, SessionStatus};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

pub struct CLIService {
    sessions: Arc<tokio::sync::Mutex<HashMap<String, CLISessionHandle>>>,
}

struct CLISessionHandle {
    _child: Child,
    tx: mpsc::Sender<String>,
}

impl CLIService {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Start a new CLI session
    pub async fn start_session(
        &self,
        tool: String,
        project_path: String,
    ) -> Result<CLISession> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        // Check if tool is installed
        let tool_cmd = match tool.as_str() {
            "kimi" => "kimi",
            "claude" => "claude",
            "codex" => "codex",
            _ => return Err(anyhow::anyhow!("Unknown tool: {}", tool)),
        };

        // Start the process
        let mut child = Command::new(tool_cmd)
            .current_dir(&project_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context(format!("Failed to start {}", tool))?;

        let (tx, _rx) = mpsc::channel::<String>(100);

        // Handle stdout/stderr
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let _tx_clone = tx.clone();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("[stdout] {}", line);
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            let _tx_clone = tx.clone();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("[stderr] {}", line);
                }
            });
        }

        let handle = CLISessionHandle { _child: child, tx };
        let mut sessions = self.sessions.lock().await;
        sessions.insert(id.clone(), handle);

        Ok(CLISession {
            id,
            tool,
            project_path,
            status: SessionStatus::Running,
            started_at: now,
        })
    }

    /// Send command to session
    pub async fn send_command(&self, session_id: String, command: String) -> Result<()> {
        let sessions = self.sessions.lock().await;
        if let Some(handle) = sessions.get(&session_id) {
            handle.tx.send(command).await.context("Failed to send command")?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    /// Kill session
    pub async fn kill_session(&self, session_id: String) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(mut handle) = sessions.remove(&session_id) {
            handle._child.kill().await.context("Failed to kill session")?;
        }
        Ok(())
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.lock().await;
        sessions.keys().cloned().collect()
    }
}

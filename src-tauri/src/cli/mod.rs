//! CLI Tool integration module

pub mod kimi;
pub mod claude;
pub mod codex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Child;

/// CLI Tool trait for unified interface
#[async_trait]
pub trait CLITool: Send + Sync {
    /// Tool name
    fn name(&self) -> &str;
    
    /// Check if tool is installed
    async fn is_installed(&self) -> bool;
    
    /// Get tool version
    async fn version(&self) -> Result<String, CLIError>;
    
    /// Start a new session
    async fn start_session(&self, project_path: String) -> Result<Session, CLIError>;
    
    /// Send command to session
    async fn send_command(&self, session_id: String, command: String) -> Result<(), CLIError>;
    
    /// Kill session
    async fn kill_session(&self, session_id: String) -> Result<(), CLIError>;
}

/// CLI Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub tool: String,
    pub project_path: String,
    pub status: SessionStatus,
    pub started_at: i64,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Running,
    Stopped,
    Error,
}

/// CLI Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIOutput {
    pub session_id: String,
    pub output_type: OutputType,
    pub content: String,
    pub timestamp: i64,
}

/// Output type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
    Stderr,
    System,
}

/// CLI Error
#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("Tool not installed: {0}")]
    NotInstalled(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Command error: {0}")]
    CommandError(String),
}

/// CLI Process handle
pub struct CLIProcess {
    pub child: Child,
    pub session_id: String,
}

//! Codex CLI adapter

use super::{CLIError, CLITool, Session};
use async_trait::async_trait;

pub struct CodexCLI;

#[async_trait]
impl CLITool for CodexCLI {
    fn name(&self) -> &str {
        "codex"
    }

    async fn is_installed(&self) -> bool {
        which::which("codex").is_ok()
    }

    async fn version(&self) -> Result<String, CLIError> {
        let output = std::process::Command::new("codex")
            .arg("--version")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(CLIError::CommandError("Failed to get version".to_string()))
        }
    }

    async fn start_session(&self, _project_path: String) -> Result<Session, CLIError> {
        // TODO: Implement session start
        Err(CLIError::NotInstalled("codex".to_string()))
    }

    async fn send_command(&self, _session_id: String, _command: String) -> Result<(), CLIError> {
        // TODO: Implement command sending
        Ok(())
    }

    async fn kill_session(&self, _session_id: String) -> Result<(), CLIError> {
        // TODO: Implement session kill
        Ok(())
    }
}

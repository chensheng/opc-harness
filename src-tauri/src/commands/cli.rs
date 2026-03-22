use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub is_installed: bool,
    pub version: Option<String>,
    pub install_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectToolsResponse {
    pub tools: Vec<ToolStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub tool_type: String,
    pub project_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendPromptRequest {
    pub session_id: String,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadOutputResponse {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopSessionRequest {
    pub session_id: String,
}

// Global session manager
lazy_static::lazy_static! {
    static ref SESSIONS: Arc<Mutex<HashMap<String, CLISession>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct CLISession {
    pub id: String,
    pub tool_type: String,
    pub project_path: String,
}

#[tauri::command]
pub async fn detect_tools() -> Result<DetectToolsResponse, String> {
    // TODO: Implement actual tool detection
    let tools = vec![
        ToolStatus {
            name: "Node.js".to_string(),
            is_installed: true,
            version: Some("18.17.0".to_string()),
            install_url: Some("https://nodejs.org".to_string()),
        },
        ToolStatus {
            name: "Git".to_string(),
            is_installed: true,
            version: Some("2.42.0".to_string()),
            install_url: Some("https://git-scm.com".to_string()),
        },
        ToolStatus {
            name: "Kimi CLI".to_string(),
            is_installed: false,
            version: None,
            install_url: Some("https://www.moonshot.cn/docs/cli".to_string()),
        },
        ToolStatus {
            name: "Claude Code".to_string(),
            is_installed: false,
            version: None,
            install_url: Some("https://docs.anthropic.com/en/docs/agents-and-tools/claude-code".to_string()),
        },
    ];
    
    Ok(DetectToolsResponse { tools })
}

#[tauri::command]
pub async fn create_cli_session(
    request: CreateSessionRequest,
) -> Result<CreateSessionResponse, String> {
    let session_id = Uuid::new_v4().to_string();
    
    let session = CLISession {
        id: session_id.clone(),
        tool_type: request.tool_type,
        project_path: request.project_path,
    };
    
    let mut sessions = SESSIONS.lock().await;
    sessions.insert(session_id.clone(), session);
    
    Ok(CreateSessionResponse { session_id })
}

#[tauri::command]
pub async fn send_cli_prompt(
    request: SendPromptRequest,
) -> Result<(), String> {
    // TODO: Implement actual CLI communication
    Ok(())
}

#[tauri::command]
pub async fn read_cli_output(
    session_id: String,
) -> Result<ReadOutputResponse, String> {
    // TODO: Implement actual output reading
    Ok(ReadOutputResponse {
        stdout: Some("Mock output".to_string()),
        stderr: None,
    })
}

#[tauri::command]
pub async fn stop_cli_session(
    request: StopSessionRequest,
) -> Result<(), String> {
    let mut sessions = SESSIONS.lock().await;
    sessions.remove(&request.session_id);
    Ok(())
}

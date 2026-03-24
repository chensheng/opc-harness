use crate::agent_protocol::*;
use crate::agent::initializer_agent::{InitializerAgent, InitializerAgentConfig, EnvironmentCheckResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::State;

use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde_json;

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

// Git 相关数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
    pub is_git_repo: bool,
    pub git_version: Option<String>,
    pub branch: Option<String>,
    pub commit_count: Option<i32>,
    pub is_dirty: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub user_name: Option<String>,
    pub user_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitGitRepoRequest {
    pub path: String,
    pub initial_branch: Option<String>, // 默认 "main"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetGitConfigRequest {
    pub path: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGitConfigRequest {
    pub path: String,
    pub key: String,
}

// Agent 通信协议数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentPhase {
    Initializer,
    Coding,
    MRCreation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent_id: String,
    pub agent_type: String, // "initializer" | "coding" | "mr_creation"
    pub phase: AgentPhase,
    pub status: AgentStatus,
    pub project_path: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    pub request_id: String,
    pub agent_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub response_id: String,
    pub request_id: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub message_id: String,
    pub timestamp: i64,
    pub source: String, // "agent" | "daemon" | "frontend"
    pub message_type: String, // "log" | "status" | "progress" | "error"
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonState {
    pub session_id: String,
    pub project_id: String,
    pub current_phase: AgentPhase,
    pub active_agents: Vec<AgentStatus>,
    pub completed_issues: Vec<String>,
    pub pending_issues: Vec<String>,
    pub log_file: Option<String>,
    pub last_snapshot: i64,
    pub cpu_usage: f32,
    pub memory_usage: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    #[serde(rename = "type")]
    pub ws_type: String, // "connect" | "disconnect" | "message" | "heartbeat"
    pub data: Option<serde_json::Value>,
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

/// 检测单个工具的版本
async fn detect_tool_version(command: &str, args: Vec<&str>) -> Option<String> {
    match Command::new(command).args(&args).output().await {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                // 清理版本号，只保留核心版本
                Some(version.trim().to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// 检测工具是否安装
async fn is_tool_installed(command: &str) -> bool {
    #[cfg(windows)]
    {
        // Windows 使用 where 命令
        Command::new("where")
            .arg(command)
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg(unix)]
    {
        // Unix-like 系统使用 which 命令
        Command::new("which")
            .arg(command)
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[tauri::command]
pub async fn detect_tools() -> Result<DetectToolsResponse, String> {
    let mut tools = Vec::new();

    // 1. 检测 Node.js
    let node_installed = is_tool_installed("node").await;
    let node_version = if node_installed {
        detect_tool_version("node", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Node.js".to_string(),
        is_installed: node_installed,
        version: node_version,
        install_url: Some("https://nodejs.org".to_string()),
    });

    // 2. 检测 npm
    let npm_installed = is_tool_installed("npm").await;
    let npm_version = if npm_installed {
        detect_tool_version("npm", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "npm".to_string(),
        is_installed: npm_installed,
        version: npm_version,
        install_url: Some("https://www.npmjs.com".to_string()),
    });

    // 3. 检测 pnpm (可选)
    let pnpm_installed = is_tool_installed("pnpm").await;
    let pnpm_version = if pnpm_installed {
        detect_tool_version("pnpm", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "pnpm".to_string(),
        is_installed: pnpm_installed,
        version: pnpm_version,
        install_url: Some("https://pnpm.io".to_string()),
    });

    // 4. 检测 Git
    let git_installed = is_tool_installed("git").await;
    let git_version = if git_installed {
        detect_tool_version("git", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Git".to_string(),
        is_installed: git_installed,
        version: git_version,
        install_url: Some("https://git-scm.com".to_string()),
    });

    // 5. 检测 Rust/Cargo
    let cargo_installed = is_tool_installed("cargo").await;
    let cargo_version = if cargo_installed {
        detect_tool_version("cargo", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Rust/Cargo".to_string(),
        is_installed: cargo_installed,
        version: cargo_version,
        install_url: Some("https://www.rust-lang.org".to_string()),
    });

    // 6. 检测 Kimi CLI (可选工具)
    let kimi_installed = is_tool_installed("kimi").await;
    let kimi_version = if kimi_installed {
        detect_tool_version("kimi", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Kimi CLI".to_string(),
        is_installed: kimi_installed,
        version: kimi_version,
        install_url: Some("https://www.moonshot.cn/docs/cli".to_string()),
    });

    // 7. 检测 Claude Code (可选工具)
    let claude_installed = is_tool_installed("claude").await;
    let claude_version = if claude_installed {
        detect_tool_version("claude", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Claude Code".to_string(),
        is_installed: claude_installed,
        version: claude_version,
        install_url: Some(
            "https://docs.anthropic.com/en/docs/agents-and-tools/claude-code".to_string(),
        ),
    });

    Ok(DetectToolsResponse { tools })
}

/// 检测 Git 仓库状态
#[tauri::command]
pub async fn check_git_status(path: String) -> Result<GitStatus, String> {
    use std::path::PathBuf;
    
    let repo_path = PathBuf::from(&path);
    
    // 检查是否是 git 仓库
    let is_git_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(&repo_path)
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    if !is_git_repo {
        return Ok(GitStatus {
            is_git_repo: false,
            git_version: None,
            branch: None,
            commit_count: None,
            is_dirty: None,
        });
    }
    
    // 获取 Git 版本
    let git_version = detect_tool_version("git", vec!["--version"]).await;
    
    // 获取当前分支
    let branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&repo_path)
        .output()
        .await
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        });
    
    // 获取提交数量
    let commit_count = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(&repo_path)
        .output()
        .await
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<i32>()
                    .ok()
            } else {
                None
            }
        });
    
    // 检查是否有未提交的更改
    let is_dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&repo_path)
        .output()
        .await
        .ok()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            !stdout.trim().is_empty()
        });
    
    Ok(GitStatus {
        is_git_repo: true,
        git_version,
        branch,
        commit_count,
        is_dirty,
    })
}

/// 初始化 Git 仓库
#[tauri::command]
pub async fn init_git_repo(request: InitGitRepoRequest) -> Result<bool, String> {
    use std::path::PathBuf;
    
    let repo_path = PathBuf::from(&request.path);
    let initial_branch = request.initial_branch.as_deref().unwrap_or("main");
    
    // 创建目录 (如果不存在)
    tokio::fs::create_dir_all(&repo_path)
        .await
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    // 初始化 git 仓库
    let output = Command::new("git")
        .args(["init", "--initial-branch", initial_branch])
        .current_dir(&repo_path)
        .output()
        .await
        .map_err(|e| format!("Failed to run git init: {}", e))?;
    
    if !output.status.success() {
        // 如果 --initial-branch 不支持，回退到传统方式
        let output_fallback = Command::new("git")
            .arg("init")
            .current_dir(&repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to run git init: {}", e))?;
        
        if !output_fallback.status.success() {
            return Err("Failed to initialize git repository".to_string());
        }
    }
    
    // 创建初始的 .gitignore 文件
    let gitignore_path = repo_path.join(".gitignore");
    if !gitignore_path.exists() {
        let default_gitignore = "# Dependencies\nnode_modules/\n.pnpm-store/\n\n# Build output\ndist/\nbuild/\nout/\n\n# Environment\n.env\n.env.local\n.env.*.local\n\n# IDE\n.vscode/\n.idea/\n*.swp\n*.swo\n*~\n\n# OS\n.DS_Store\nThumbs.db\n\n# Logs\n*.log\nlogs/\n\n# Testing\ncoverage/\n.nyc_output/\n";
        tokio::fs::write(&gitignore_path, default_gitignore)
            .await
            .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
    }
    
    Ok(true)
}

/// 设置 Git 配置项
#[tauri::command]
pub async fn set_git_config(request: SetGitConfigRequest) -> Result<bool, String> {
    use std::path::PathBuf;
    
    let repo_path = PathBuf::from(&request.path);
    
    let output = Command::new("git")
        .args(["config", &request.key, &request.value])
        .current_dir(&repo_path)
        .output()
        .await
        .map_err(|e| format!("Failed to set git config: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to set git config: {}", stderr));
    }
    
    Ok(true)
}

/// 获取 Git 配置项
#[tauri::command]
pub async fn get_git_config(request: GetGitConfigRequest) -> Result<Option<String>, String> {
    use std::path::PathBuf;
    
    let repo_path = PathBuf::from(&request.path);
    
    let output = Command::new("git")
        .args(["config", &request.key])
        .current_dir(&repo_path)
        .output()
        .await
        .map_err(|e| format!("Failed to get git config: {}", e))?;
    
    if !output.status.success() {
        // 配置项不存在时返回 None
        return Ok(None);
    }
    
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(Some(value))
}

/// 获取完整的 Git 配置
#[tauri::command]
pub async fn get_all_git_config(path: String) -> Result<GitConfig, String> {
    use std::path::PathBuf;
    
    let repo_path = PathBuf::from(&path);
    
    let user_name = get_git_config(GetGitConfigRequest {
        path: path.clone(),
        key: "user.name".to_string(),
    })
    .await?;
    
    let user_email = get_git_config(GetGitConfigRequest {
        path: path.clone(),
        key: "user.email".to_string(),
    })
    .await?;
    
    Ok(GitConfig {
        user_name,
        user_email,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_git_status_serialization() {
        let status = GitStatus {
            is_git_repo: true,
            git_version: Some("2.42.0".to_string()),
            branch: Some("main".to_string()),
            commit_count: Some(10),
            is_dirty: Some(false),
        };
        
        let json = serde_json::to_string(&status).unwrap();
        // 验证实际序列化的字段名（snake_case）
        assert!(json.contains("\"is_git_repo\":true"));
        assert!(json.contains("\"git_version\":\"2.42.0\""));
    }
    
    #[test]
    fn test_git_config_serialization() {
        let config = GitConfig {
            user_name: Some("Test User".to_string()),
            user_email: Some("test@example.com".to_string()),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        // 验证实际序列化的字段名（snake_case）
        assert!(json.contains("\"user_name\":\"Test User\""));
        assert!(json.contains("\"user_email\":\"test@example.com\""));
    }
    
    #[test]
    fn test_init_git_repo_request_serialization() {
        let request = InitGitRepoRequest {
            path: "/tmp/test".to_string(),
            initial_branch: Some("develop".to_string()),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        // 验证实际序列化的字段名（snake_case）
        assert!(json.contains("\"path\":\"/tmp/test\""));
        assert!(json.contains("\"initial_branch\":\"develop\""));
    }
    
    #[test]
    fn test_set_git_config_request_serialization() {
        let request = SetGitConfigRequest {
            path: "/tmp/test".to_string(),
            key: "user.name".to_string(),
            value: "Test User".to_string(),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"key\":\"user.name\""));
    }
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
pub async fn send_cli_prompt(_request: SendPromptRequest) -> Result<(), String> {
    // TODO: Implement actual CLI communication
    Ok(())
}

#[tauri::command]
pub async fn read_cli_output(_session_id: String) -> Result<ReadOutputResponse, String> {
    // TODO: Implement actual output reading
    Ok(ReadOutputResponse {
        stdout: Some("Mock output".to_string()),
        stderr: None,
    })
}

#[tauri::command]
pub async fn stop_cli_session(request: StopSessionRequest) -> Result<(), String> {
    let mut sessions = SESSIONS.lock().await;
    sessions.remove(&request.session_id);
    Ok(())
}

// ========== Daemon 管理命令 ==========

/// 启动守护进程
#[tauri::command]
pub async fn start_daemon(
    session_id: String,
    project_path: String,
    log_level: Option<String>,
    max_concurrent_agents: Option<usize>,
) -> Result<DaemonSnapshot, String> {
    use std::sync::Mutex;
    
    // TODO: 使用真实的 DaemonManager 单例
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    
    let config = DaemonConfig {
        session_id,
        project_path,
        log_level: log_level.unwrap_or_else(|| "info".to_string()),
        max_concurrent_agents: max_concurrent_agents.unwrap_or(5),
        workspace_dir: std::env::current_dir()
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .to_string(),
    };
    
    manager.start(config)?;
    
    Ok(manager.get_snapshot())
}

/// 停止守护进程
#[tauri::command]
pub async fn stop_daemon(graceful: Option<bool>) -> Result<(), String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    manager.stop(graceful.unwrap_or(true))
}

/// 暂停守护进程
#[tauri::command]
pub async fn pause_daemon() -> Result<(), String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    manager.pause()
}

/// 恢复守护进程
#[tauri::command]
pub async fn resume_daemon() -> Result<(), String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    manager.resume()
}

/// 生成新的 Agent
#[tauri::command]
pub async fn spawn_agent(agent_type: String) -> Result<String, String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    manager.spawn_agent(&agent_type)
}

/// 终止指定 Agent
#[tauri::command]
pub async fn kill_agent(agent_id: String) -> Result<(), String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let mut manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    manager.kill_agent(&agent_id)
}

/// 获取守护进程状态
#[tauri::command]
pub async fn get_daemon_status() -> Result<DaemonStatus, String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_status())
}

/// 获取守护进程快照
#[tauri::command]
pub async fn get_daemon_snapshot() -> Result<DaemonSnapshot, String> {
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref DAEMON_MANAGER: Mutex<DaemonManager> = Mutex::new(DaemonManager::new());
    }
    
    let manager = DAEMON_MANAGER.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_snapshot())
}

/// VC-007: 检查开发环境
#[tauri::command]
pub async fn check_environment(project_path: String) -> Result<EnvironmentCheckResult, String> {
    // 创建 Initializer Agent 配置
    let config = InitializerAgentConfig {
        agent_id: format!("env-check-{}", Uuid::new_v4()),
        project_path: project_path.clone(),
        ai_config: crate::ai::AIConfig {
            provider: "openai".to_string(),
            api_key: "placeholder".to_string(), // 环境检查不需要真实的 API key
            model: "gpt-4".to_string(),
            base_url: None,
        },
        prd_file_path: None,
        prd_content: None,
    };
    
    // 创建 Agent 并执行环境检查
    let mut agent = InitializerAgent::new(config);
    let result = agent.check_environment().await?;
    
    Ok(result)
}

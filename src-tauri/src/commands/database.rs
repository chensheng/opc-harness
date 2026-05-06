use crate::db;
use crate::models::{AIConfig, AgentSession, CLISession, Issue, Milestone, Project};
use chrono::Utc;
use uuid::Uuid;

/// 创建新项目
#[tauri::command]
pub fn create_project(
    app_handle: tauri::AppHandle,
    name: String,
    description: String,
) -> Result<String, String> {
    // 先生成项目ID
    let project_id = Uuid::new_v4().to_string();
    
    // 使用项目ID创建工作区目录
    let workspace_path = create_workspace_directory(&project_id)?;
    
    // ✨ 新增：初始化 Git 仓库
    if let Err(e) = initialize_git_repository(&workspace_path.to_string_lossy()) {
        log::warn!("Git initialization failed (non-critical): {}", e);
        log::warn!("User can initialize Git manually later via GitDetector");
    } else {
        log::info!("Project workspace created with Git initialized at: {:?} (ID: {})", workspace_path, project_id);
    }

    let conn = db::get_connection().map_err(|e| e.to_string())?;

    let project = Project {
        id: project_id.clone(),
        name,
        description,
        status: "idea".to_string(),
        progress: 0,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        idea: None,
        prd: None,
        user_personas: None,
        competitor_analysis: None,
    };

    db::create_project(&conn, &project).map_err(|e| e.to_string())?;
    
    // 记录工作区路径（可以在后续版本中将此路径保存到数据库）
    log::info!("Project saved to database (ID: {})", project_id);
    
    Ok(project.id)
}

/// 为项目创建工作区目录
/// 
/// 在 ~/.opc-harness/workspaces/ 下创建以项目ID命名的子目录
/// 
/// # 参数
/// * `project_id` - 项目ID（UUID格式）
/// 
/// # 返回
/// * `Ok(PathBuf)` - 工作区目录路径
/// * `Err(String)` - 错误信息
fn create_workspace_directory(project_id: &str) -> Result<std::path::PathBuf, String> {
    use crate::utils::paths::get_workspaces_dir;
    
    // 获取工作区根目录
    let workspaces_root = get_workspaces_dir();
    
    // 确保工作区根目录存在
    std::fs::create_dir_all(&workspaces_root)
        .map_err(|e| format!("Failed to create workspaces directory: {}", e))?;
    
    // 构建项目工作区路径（直接使用项目ID作为目录名）
    let project_workspace = workspaces_root.join(project_id);
    
    // 如果目录已存在（理论上不应该发生，因为UUID是唯一的），添加时间戳避免冲突
    let final_path = if project_workspace.exists() {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        workspaces_root.join(format!("{}_{}", project_id, timestamp))
    } else {
        project_workspace
    };
    
    // 创建项目工作区目录
    std::fs::create_dir_all(&final_path)
        .map_err(|e| format!("Failed to create project workspace directory: {}", e))?;
    
    log::info!("Created workspace directory: {:?}", final_path);
    
    Ok(final_path)
}

/// 初始化 Git 仓库
/// 
/// 在项目工作区目录中执行 git init，创建 .gitignore 文件，并设置默认用户配置
/// 
/// # 参数
/// * `project_path` - 项目工作区路径
/// 
/// # 返回
/// * `Ok(())` - 成功
/// * `Err(String)` - 错误信息
fn initialize_git_repository(project_path: &str) -> Result<(), String> {
    use std::process::Command;
    use std::path::Path;
    
    log::info!("[initialize_git_repository] Initializing Git at: {}", project_path);
    
    let path = Path::new(project_path);
    
    // 检查目录是否存在
    if !path.exists() {
        return Err(format!("Directory does not exist: {}", project_path));
    }
    
    // 检查是否已经是 Git 仓库
    if path.join(".git").exists() {
        log::info!("[initialize_git_repository] Git already initialized, skipping");
        return Ok(());
    }
    
    // 执行 git init
    let output = Command::new("git")
        .current_dir(path)
        .args(&["init", "-b", "main"])
        .output()
        .map_err(|e| format!("Failed to execute git init: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git init failed: {}", stderr));
    }
    
    log::info!("[initialize_git_repository] Git initialized with branch: main");
    
    // 创建 .gitignore 文件
    create_gitignore_file(project_path)?;
    
    // 配置用户信息（如果全局未配置）
    ensure_git_user_config(project_path)?;
    
    // 创建初始空 commit
    let commit_output = Command::new("git")
        .current_dir(path)
        .args(&["commit", "--allow-empty", "-m", "Initial commit"])
        .output()
        .map_err(|e| format!("Failed to create initial commit: {}", e))?;
    
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        log::warn!("[initialize_git_repository] Initial commit warning: {}", stderr);
    } else {
        log::info!("[initialize_git_repository] Initial commit created");
    }
    
    Ok(())
}

/// 创建 .gitignore 文件
fn create_gitignore_file(project_path: &str) -> Result<(), String> {
    use std::fs;
    use std::path::Path;
    
    let gitignore_path = Path::new(project_path).join(".gitignore");
    
    // 如果 .gitignore 已存在，跳过创建
    if gitignore_path.exists() {
        log::info!("[create_gitignore_file] .gitignore already exists, skipping");
        return Ok(());
    }
    
    // OPC-HARNESS 项目的标准 .gitignore 内容
    let content = "# OPC-HARNESS context files\n.opc-harness/\n";
    
    fs::write(&gitignore_path, content)
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
    
    log::info!("[create_gitignore_file] Created .gitignore at {:?}", gitignore_path);
    Ok(())
}

/// 确保 Git 用户配置存在
fn ensure_git_user_config(project_path: &str) -> Result<(), String> {
    use std::process::Command;
    
    // 检查全局配置
    let user_name_check = Command::new("git")
        .args(&["config", "--global", "user.name"])
        .output();
    
    let needs_config = match user_name_check {
        Ok(output) => output.stdout.is_empty(),
        Err(_) => true,
    };
    
    if needs_config {
        log::info!("[ensure_git_user_config] Setting default Git user config");
        
        // 设置默认用户名
        Command::new("git")
            .current_dir(project_path)
            .args(&["config", "user.name", "OPC-HARNESS User"])
            .output()
            .map_err(|e| format!("Failed to set user.name: {}", e))?;
        
        // 设置默认邮箱
        Command::new("git")
            .current_dir(project_path)
            .args(&["config", "user.email", "harness@opc.local"])
            .output()
            .map_err(|e| format!("Failed to set user.email: {}", e))?;
        
        log::info!("[ensure_git_user_config] Default Git user config set");
    } else {
        log::info!("[ensure_git_user_config] Global Git config exists, using it");
    }
    
    Ok(())
}

/// 确保工作区目录和 Git 仓库存在
/// 
/// 在打开项目时调用，检查工作区目录是否存在，如果不存在则创建；
/// 检查 Git 是否已初始化，如果未初始化则执行初始化。
/// 
/// # 参数
/// * `project_id` - 项目 ID
/// * `app_handle` - Tauri 应用句柄
/// 
/// # 返回
/// * `Ok(())` - 成功
/// * `Err(String)` - 错误信息
fn ensure_workspace_and_git(project_id: &str, _app_handle: &tauri::AppHandle) -> Result<(), String> {
    use crate::utils::paths::get_workspaces_dir;
    
    let workspaces_root = get_workspaces_dir();
    let workspace_path = workspaces_root.join(project_id);
    
    // 如果工作区目录不存在，创建它
    if !workspace_path.exists() {
        log::info!("[ensure_workspace_and_git] Workspace directory missing, recreating: {:?}", workspace_path);
        std::fs::create_dir_all(&workspace_path)
            .map_err(|e| format!("Failed to create workspace directory: {}", e))?;
    }
    
    // 检查 Git 是否已初始化
    let git_dir = workspace_path.join(".git");
    if !git_dir.exists() {
        log::info!("[ensure_workspace_and_git] Git not initialized, initializing now...");
        // 调用 Git 初始化逻辑（复用 create_project 中的函数）
        initialize_git_repository(&workspace_path.to_string_lossy())?;
    } else {
        log::debug!("[ensure_workspace_and_git] Git already initialized");
    }
    
    Ok(())
}

/// 获取所有项目
#[tauri::command]
pub fn get_all_projects(app_handle: tauri::AppHandle) -> Result<Vec<Project>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_all_projects(&conn).map_err(|e| e.to_string())
}

/// 获取单个项目
#[tauri::command]
pub fn get_project_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Project>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    
    match db::get_project_by_id(&conn, &id).map_err(|e| e.to_string())? {
        Some(project) => {
            // ✨ 新增：检查工作区目录并初始化 Git（如果需要）
            if let Err(e) = ensure_workspace_and_git(&id, &app_handle) {
                log::warn!("[get_project_by_id] Git initialization warning: {}", e);
            }
            
            Ok(Some(project))
        }
        None => Ok(None),
    }
}

/// 更新项目
#[tauri::command]
pub fn update_project(app_handle: tauri::AppHandle, project: Project) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::update_project(&conn, &project).map_err(|e| e.to_string())
}

/// 删除项目
#[tauri::command]
pub fn delete_project(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::delete_project(&conn, &id).map_err(|e| e.to_string())
}

/// 保存 AI 配置 (同时保存到数据库和 OS keychain)
#[tauri::command]
pub fn save_ai_config(app_handle: tauri::AppHandle, config: AIConfig) -> Result<(), String> {
    // Validate inputs
    if config.provider.is_empty() {
        return Err("Provider name cannot be empty".to_string());
    }

    if config.model.is_empty() {
        return Err("Model name cannot be empty".to_string());
    }

    if config.api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    // Save API key to OS keychain first
    crate::utils::keychain::save_api_key(&config.provider, &config.api_key)
        .map_err(|e| format!("Failed to save API key to keychain: {}", e))?;

    // Then save provider and model to database
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    let config_for_db = AIConfig::new(config.provider, config.model);
    db::save_ai_config(&conn, &config_for_db).map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取所有 AI 配置 (从数据库获取 provider 和 model，尝试从 keychain 获取 api_key)
#[tauri::command]
pub fn get_all_ai_configs(app_handle: tauri::AppHandle) -> Result<Vec<AIConfig>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    // Get all configs from database
    let configs = db::get_all_ai_configs(&conn).map_err(|e| e.to_string())?;

    // Try to retrieve API keys from keychain for each config
    let mut result = Vec::new();
    for mut config in configs {
        if let Ok(api_key) = crate::utils::keychain::get_api_key(&config.provider) {
            config.api_key = api_key;
        }
        result.push(config);
    }

    Ok(result)
}

/// 获取单个 AI 配置 (从数据库获取 provider 和 model，从 keychain 获取 api_key)
#[tauri::command]
pub fn get_ai_config(
    app_handle: tauri::AppHandle,
    provider: String,
) -> Result<Option<AIConfig>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    // Get provider and model from database
    match db::get_ai_config(&conn, &provider) {
        Ok(Some(mut config)) => {
            // Try to retrieve API key from keychain
            match crate::utils::keychain::get_api_key(&provider) {
                Ok(api_key) => {
                    config.api_key = api_key;
                    Ok(Some(config))
                }
                Err(_) => {
                    // Key not found in keychain, return config without key
                    Ok(Some(config))
                }
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Failed to get AI config: {}", e)),
    }
}

/// 删除 AI 配置 (同时删除数据库记录和 OS keychain 中的密钥)
#[tauri::command]
pub fn delete_ai_config(app_handle: tauri::AppHandle, provider: String) -> Result<(), String> {
    // Delete from database
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::delete_ai_config(&conn, &provider).map_err(|e| e.to_string())?;

    // Also delete from OS keychain
    crate::utils::keychain::delete_api_key(&provider)
        .map_err(|e| format!("Failed to delete API key from keychain: {}", e))?;

    Ok(())
}

/// 创建 CLI 会话
#[tauri::command]
pub fn create_cli_session_db(
    app_handle: tauri::AppHandle,
    tool_type: String,
    project_path: String,
) -> Result<String, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    let session = CLISession {
        id: Uuid::new_v4().to_string(),
        tool_type,
        project_path,
        created_at: Utc::now().to_rfc3339(),
    };

    db::create_cli_session(&conn, &session).map_err(|e| e.to_string())?;
    Ok(session.id)
}

/// 获取所有 CLI 会话
#[tauri::command]
pub fn get_all_cli_sessions(app_handle: tauri::AppHandle) -> Result<Vec<CLISession>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_all_cli_sessions(&conn).map_err(|e| e.to_string())
}

/// 获取单个 CLI 会话
#[tauri::command]
pub fn get_cli_session_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<CLISession>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_cli_session_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 删除 CLI 会话
#[tauri::command]
pub fn delete_cli_session(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::delete_cli_session(&conn, &id).map_err(|e| e.to_string())
}

// ==================== Milestone Commands ====================

/// 创建里程碑
#[tauri::command]
pub fn create_milestone(
    app_handle: tauri::AppHandle,
    project_id: String,
    title: String,
    description: String,
    order: i32,
    due_date: Option<String>,
) -> Result<String, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    let milestone = Milestone {
        id: Uuid::new_v4().to_string(),
        project_id,
        title,
        description,
        order,
        status: "pending".to_string(),
        due_date,
        completed_at: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    db::create_milestone(&conn, &milestone).map_err(|e| e.to_string())?;
    Ok(milestone.id)
}

/// 获取项目的所有里程碑
#[tauri::command]
pub fn get_milestones_by_project(
    app_handle: tauri::AppHandle,
    project_id: String,
) -> Result<Vec<Milestone>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_milestones_by_project(&conn, &project_id).map_err(|e| e.to_string())
}

/// 获取单个里程碑
#[tauri::command]
pub fn get_milestone_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Milestone>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_milestone_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 更新里程碑
#[tauri::command]
pub fn update_milestone(app_handle: tauri::AppHandle, milestone: Milestone) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::update_milestone(&conn, &milestone).map_err(|e| e.to_string())
}

/// 删除里程碑
#[tauri::command]
pub fn delete_milestone(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::delete_milestone(&conn, &id).map_err(|e| e.to_string())
}

// ==================== Issue Commands ====================

/// 创建 Issue
#[tauri::command]
pub fn create_issue(
    app_handle: tauri::AppHandle,
    project_id: String,
    title: String,
    description: String,
    issue_type: String,
    priority: String,
    milestone_id: Option<String>,
    parent_issue_id: Option<String>,
    order: i32,
) -> Result<String, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    let issue = Issue {
        id: Uuid::new_v4().to_string(),
        project_id,
        milestone_id,
        title,
        description,
        issue_type,
        priority,
        status: "open".to_string(),
        assignee: None,
        parent_issue_id,
        order,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    db::create_issue(&conn, &issue).map_err(|e| e.to_string())?;
    Ok(issue.id)
}

/// 获取项目的所有 Issues
#[tauri::command]
pub fn get_issues_by_project(
    app_handle: tauri::AppHandle,
    project_id: String,
) -> Result<Vec<Issue>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_issues_by_project(&conn, &project_id).map_err(|e| e.to_string())
}

/// 按里程碑获取 Issues
#[tauri::command]
pub fn get_issues_by_milestone(
    app_handle: tauri::AppHandle,
    milestone_id: String,
) -> Result<Vec<Issue>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_issues_by_milestone(&conn, &milestone_id).map_err(|e| e.to_string())
}

/// 获取单个 Issue
#[tauri::command]
pub fn get_issue_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Issue>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_issue_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 更新 Issue
#[tauri::command]
pub fn update_issue(app_handle: tauri::AppHandle, issue: Issue) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::update_issue(&conn, &issue).map_err(|e| e.to_string())
}

/// 删除 Issue
#[tauri::command]
pub fn delete_issue(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::delete_issue(&conn, &id).map_err(|e| e.to_string())
}

// ==================== Agent Session Commands ====================

/// 创建 Agent Session
#[tauri::command]
pub fn create_agent_session(
    app_handle: tauri::AppHandle,
    session_id: String,
    agent_id: String,
    agent_type: String,
    project_id: String,
    status: String,
    phase: String,
    stdio_channel_id: Option<String>,
    metadata: Option<String>,
) -> Result<String, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;

    let session = AgentSession {
        session_id,
        agent_id: agent_id.clone(),
        agent_type,
        project_id,
        name: None,
        status,
        phase,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        stdio_channel_id,
        registered_to_daemon: false,
        metadata,
        agents_md_content: None,
    };

    db::create_agent_session(&conn, &session).map_err(|e| e.to_string())?;
    Ok(agent_id)
}

/// 获取项目的所有 Sessions
#[tauri::command]
pub fn get_sessions_by_project(
    app_handle: tauri::AppHandle,
    project_id: String,
) -> Result<Vec<AgentSession>, String> {
    let conn = db::get_connection().map_err(|e| {
        log::error!("[get_sessions_by_project] Failed to get database connection: {}", e);
        format!("Failed to get database connection: {}", e)
    })?;
    
    match db::get_sessions_by_project(&conn, &project_id) {
        Ok(sessions) => {
            Ok(sessions)
        },
        Err(e) => {
            log::error!("[get_sessions_by_project] Failed to query sessions: {}", e);
            Err(format!("Failed to query sessions: {}", e))
        }
    }
}

/// 获取单个 Agent Session（按 agent_id）
#[tauri::command]
pub fn get_agent_session_by_id(
    app_handle: tauri::AppHandle,
    agent_id: String,
) -> Result<Option<AgentSession>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_agent_session_by_id(&conn, &agent_id).map_err(|e| e.to_string())
}

/// 获取 Agent Session（按 session_id）
#[tauri::command]
pub fn get_agent_session_by_session_id(
    app_handle: tauri::AppHandle,
    session_id: String,
) -> Result<Option<AgentSession>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::get_agent_session_by_session_id(&conn, &session_id).map_err(|e| e.to_string())
}

/// 更新 Agent Session 状态
#[tauri::command]
pub fn update_agent_session_status(
    app_handle: tauri::AppHandle,
    agent_id: String,
    status: String,
    phase: String,
) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::update_agent_session_status(&conn, &agent_id, &status, &phase).map_err(|e| e.to_string())
}

/// 更新 Agent Session 完整信息
#[tauri::command]
pub fn update_agent_session(app_handle: tauri::AppHandle, session: AgentSession) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    db::update_agent_session(&conn, &session).map_err(|e| e.to_string())
}

/// 删除 Agent Session
#[tauri::command]
pub fn delete_agent_session(app_handle: tauri::AppHandle, agent_id: String) -> Result<(), String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    
    // 先获取智能体信息，检查状态
    let session = db::get_agent_session_by_id(&conn, &agent_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Agent not found: {}", agent_id))?;
    
    // 检查智能体是否处于运行状态
    if session.status == "running" || session.status == "paused" {
        return Err(format!(
            "无法删除正在运行的智能体。当前状态: {}。请先停止智能体后再删除。",
            session.status
        ));
    }
    
    // 状态允许删除，执行删除操作
    db::delete_agent_session(&conn, &agent_id).map_err(|e| e.to_string())
}

// ==================== Sprint Commands ====================

use serde::{Deserialize, Serialize};

/// 保存Sprint计划请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SaveSprintsRequest {
    /// 项目 ID
    pub project_id: String,
    /// Sprint列表
    pub sprints: Vec<crate::models::Sprint>,
}

/// 保存项目的Sprint计划（批量Upsert）
#[tauri::command]
pub fn save_sprints(
    app_handle: tauri::AppHandle,
    request: SaveSprintsRequest,
) -> Result<usize, String> {
    println!("[save_sprints] Received request for project_id: {}, sprints count: {}", 
             request.project_id, request.sprints.len());
    
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[save_sprints] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    // 批量保存到数据库
    match db::upsert_sprints(&conn, &request.project_id, &request.sprints) {
        Ok(_) => {
            println!("[save_sprints] Successfully saved {} sprints to database for project {}", 
                     request.sprints.len(), request.project_id);
            Ok(request.sprints.len())
        },
        Err(e) => {
            eprintln!("[save_sprints] Failed to save sprints: {}", e);
            Err(format!("Failed to save sprints: {}", e))
        }
    }
}

/// 获取Sprint计划请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSprintsRequest {
    /// 项目 ID
    pub project_id: String,
}

/// 获取项目的所有Sprint计划
#[tauri::command]
pub fn get_sprints_by_project(
    app_handle: tauri::AppHandle,
    request: GetSprintsRequest,
) -> Result<Vec<crate::models::Sprint>, String> {
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[get_sprints_by_project] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    match db::get_sprints_by_project(&conn, &request.project_id) {
        Ok(sprints) => {
           Ok(sprints)
        },
        Err(e) => {
            eprintln!("[get_sprints_by_project] Failed to get sprints: {}", e);
            Err(format!("Failed to get sprints: {}", e))
        }
    }
}

/// 删除Sprint计划请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSprintRequest {
    /// Sprint ID
    pub sprint_id: String,
}

/// 删除单个Sprint计划
#[tauri::command]
pub fn delete_sprint(
    app_handle: tauri::AppHandle,
    request: DeleteSprintRequest,
) -> Result<(), String> {
    println!("[delete_sprint] Deleting sprint_id: {}", request.sprint_id);
    
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[delete_sprint] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    match db::delete_sprint(&conn, &request.sprint_id) {
        Ok(deleted) => {
            if deleted == 0 {
                return Err(format!("Sprint not found: {}", request.sprint_id));
            }
            println!("[delete_sprint] Successfully deleted sprint");
            Ok(())
        },
        Err(e) => {
            eprintln!("[delete_sprint] Failed to delete sprint: {}", e);
            Err(format!("Failed to delete sprint: {}", e))
        }
    }
}

/// 获取指定 Sprint 下的用户故事列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSprintStoriesRequest {
    /// Sprint ID
    pub sprint_id: String,
}

/// 获取指定 Sprint 下的用户故事列表
#[tauri::command]
pub fn get_sprint_stories(
    app_handle: tauri::AppHandle,
    request: GetSprintStoriesRequest,
) -> Result<Vec<crate::models::UserStory>, String> {
    println!("[get_sprint_stories] Querying for sprint_id: {}", request.sprint_id);
    
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[get_sprint_stories] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    match db::get_user_stories_by_sprint(&conn, &request.sprint_id) {
        Ok(stories) => {
            println!("[get_sprint_stories] Retrieved {} stories", stories.len());
            Ok(stories)
        },
        Err(e) => {
            eprintln!("[get_sprint_stories] Failed to get stories: {}", e);
            Err(format!("Failed to get stories: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TEST_MUTEX, TestCleanup};
    
    #[test]
    fn test_create_workspace_directory_with_uuid() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        // 清除可能存在的环境变量
        std::env::remove_var("OPC_HARNESS_HOME");
        
        use crate::utils::paths::get_workspaces_dir;
        
        // 使用唯一的临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!("test-workspace-uuid-{}", Uuid::new_v4()));
        
        // 清理可能存在的旧测试目录（确保幂等性）
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }
        
        // 创建 RAII 守卫，确保无论如何都会清理
        let _cleanup = TestCleanup::new(temp_dir.clone());
        
        // 设置独立的环境变量，避免污染真实环境
        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保工作区根目录存在（在设置环境变量之后获取）
        let workspaces_root = get_workspaces_dir();
        std::fs::create_dir_all(&workspaces_root).expect("Failed to create workspaces root");
        
        // 验证临时目录路径正确（防止路径穿越）
        assert!(workspaces_root.starts_with(&temp_dir), 
                "Workspaces root {:?} should be under temp dir {:?}", 
                workspaces_root, temp_dir);
        
        // 使用 UUID 作为项目 ID
        let project_id = Uuid::new_v4().to_string();
        let result = create_workspace_directory(&project_id);
        
        assert!(result.is_ok(), "Failed to create workspace directory: {:?}", result.err());
        
        let workspace_path = result.unwrap();
        assert!(workspace_path.exists(), "Workspace directory should exist");
        assert!(workspace_path.is_dir(), "Workspace path should be a directory");
        
        // 验证路径结构正确（使用之前保存的 workspaces_root）
        assert!(workspace_path.starts_with(&workspaces_root), 
                "Workspace path {:?} should start with workspaces root {:?}", 
                workspace_path, workspaces_root);
        assert_eq!(workspace_path.file_name().unwrap().to_string_lossy(), project_id);
        
        // 不需要手动清理，_cleanup 会在函数退出时自动调用 Drop
    }
}


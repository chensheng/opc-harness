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
    // 创建工作区目录
    let workspace_path = create_workspace_directory(&name)?;

    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

    let project = Project {
        id: Uuid::new_v4().to_string(),
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
    log::info!("Project workspace created at: {:?}", workspace_path);
    
    Ok(project.id)
}

/// 为项目创建工作区目录
/// 
/// 在 ~/.opc-harness/workspaces/ 下创建以项目名称命名的子目录
/// 
/// # 参数
/// * `project_name` - 项目名称
/// 
/// # 返回
/// * `Ok(PathBuf)` - 工作区目录路径
/// * `Err(String)` - 错误信息
fn create_workspace_directory(project_name: &str) -> Result<std::path::PathBuf, String> {
    use crate::utils::paths::get_workspaces_dir;
    
    // 获取工作区根目录
    let workspaces_root = get_workspaces_dir();
    
    // 确保工作区根目录存在
    std::fs::create_dir_all(&workspaces_root)
        .map_err(|e| format!("Failed to create workspaces directory: {}", e))?;
    
    // 生成安全的项目目录名（替换非法字符）
    let safe_project_name = sanitize_project_name(project_name);
    
    // 构建项目工作区路径
    let project_workspace = workspaces_root.join(&safe_project_name);
    
    // 如果目录已存在，添加时间戳避免冲突
    let final_path = if project_workspace.exists() {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        workspaces_root.join(format!("{}_{}", safe_project_name, timestamp))
    } else {
        project_workspace
    };
    
    // 创建项目工作区目录
    std::fs::create_dir_all(&final_path)
        .map_err(|e| format!("Failed to create project workspace directory: {}", e))?;
    
    log::info!("Created workspace directory: {:?}", final_path);
    
    Ok(final_path)
}

/// 清理项目名称，使其适合作为目录名
/// 
/// 替换或移除文件系统不允许的字符
/// 
/// # 参数
/// * `name` - 原始项目名称
/// 
/// # 返回
/// * `String` - 安全的目录名
fn sanitize_project_name(name: &str) -> String {
    // 第一步：替换所有非法字符为下划线
    let replaced = name.chars()
        .map(|c| match c {
            // 允许的字符保持不变
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c,
            // 点号也替换为下划线（避免隐藏文件和扩展名问题）
            '.' => '_',
            // 空格和其他特殊字符替换为下划线
            _ => '_',
        })
        .collect::<String>();
    
    // 第二步：移除连续的下划线
    let cleaned = replaced
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_");
    
    // 第三步：确保不以点或下划线开头
    let trimmed = cleaned
        .trim_start_matches('.')
        .trim_start_matches('_');
    
    // 第四步：如果为空，使用默认名称
    if trimmed.is_empty() {
        "unnamed_project".to_string()
    } else {
        trimmed.to_string()
    }
}

/// 获取所有项目
#[tauri::command]
pub fn get_all_projects(app_handle: tauri::AppHandle) -> Result<Vec<Project>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_all_projects(&conn).map_err(|e| e.to_string())
}

/// 获取单个项目
#[tauri::command]
pub fn get_project_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Project>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_project_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 更新项目
#[tauri::command]
pub fn update_project(app_handle: tauri::AppHandle, project: Project) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::update_project(&conn, &project).map_err(|e| e.to_string())
}

/// 删除项目
#[tauri::command]
pub fn delete_project(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    let config_for_db = AIConfig::new(config.provider, config.model);
    db::save_ai_config(&conn, &config_for_db).map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取所有 AI 配置 (从数据库获取 provider 和 model，尝试从 keychain 获取 api_key)
#[tauri::command]
pub fn get_all_ai_configs(app_handle: tauri::AppHandle) -> Result<Vec<AIConfig>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_all_cli_sessions(&conn).map_err(|e| e.to_string())
}

/// 获取单个 CLI 会话
#[tauri::command]
pub fn get_cli_session_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<CLISession>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_cli_session_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 删除 CLI 会话
#[tauri::command]
pub fn delete_cli_session(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_milestones_by_project(&conn, &project_id).map_err(|e| e.to_string())
}

/// 获取单个里程碑
#[tauri::command]
pub fn get_milestone_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Milestone>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_milestone_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 更新里程碑
#[tauri::command]
pub fn update_milestone(app_handle: tauri::AppHandle, milestone: Milestone) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::update_milestone(&conn, &milestone).map_err(|e| e.to_string())
}

/// 删除里程碑
#[tauri::command]
pub fn delete_milestone(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_issues_by_project(&conn, &project_id).map_err(|e| e.to_string())
}

/// 按里程碑获取 Issues
#[tauri::command]
pub fn get_issues_by_milestone(
    app_handle: tauri::AppHandle,
    milestone_id: String,
) -> Result<Vec<Issue>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_issues_by_milestone(&conn, &milestone_id).map_err(|e| e.to_string())
}

/// 获取单个 Issue
#[tauri::command]
pub fn get_issue_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Issue>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_issue_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 更新 Issue
#[tauri::command]
pub fn update_issue(app_handle: tauri::AppHandle, issue: Issue) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::update_issue(&conn, &issue).map_err(|e| e.to_string())
}

/// 删除 Issue
#[tauri::command]
pub fn delete_issue(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    project_path: String,
    status: String,
    phase: String,
    stdio_channel_id: Option<String>,
    metadata: Option<String>,
) -> Result<String, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;

    let session = AgentSession {
        session_id,
        agent_id: agent_id.clone(),
        agent_type,
        project_path,
        status,
        phase,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        stdio_channel_id,
        registered_to_daemon: false,
        metadata,
    };

    db::create_agent_session(&conn, &session).map_err(|e| e.to_string())?;
    Ok(agent_id)
}

/// 获取项目的所有 Sessions
#[tauri::command]
pub fn get_sessions_by_project(
    app_handle: tauri::AppHandle,
    project_path: String,
) -> Result<Vec<AgentSession>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_sessions_by_project(&conn, &project_path).map_err(|e| e.to_string())
}

/// 获取单个 Agent Session（按 agent_id）
#[tauri::command]
pub fn get_agent_session_by_id(
    app_handle: tauri::AppHandle,
    agent_id: String,
) -> Result<Option<AgentSession>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_agent_session_by_id(&conn, &agent_id).map_err(|e| e.to_string())
}

/// 获取 Agent Session（按 session_id）
#[tauri::command]
pub fn get_agent_session_by_session_id(
    app_handle: tauri::AppHandle,
    session_id: String,
) -> Result<Option<AgentSession>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
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
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::update_agent_session_status(&conn, &agent_id, &status, &phase).map_err(|e| e.to_string())
}

/// 更新 Agent Session 完整信息
#[tauri::command]
pub fn update_agent_session(app_handle: tauri::AppHandle, session: AgentSession) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::update_agent_session(&conn, &session).map_err(|e| e.to_string())
}

/// 删除 Agent Session
#[tauri::command]
pub fn delete_agent_session(app_handle: tauri::AppHandle, agent_id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::delete_agent_session(&conn, &agent_id).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sanitize_project_name() {
        // 测试正常名称
        assert_eq!(sanitize_project_name("my-project"), "my-project");
        assert_eq!(sanitize_project_name("MyProject"), "MyProject");
        assert_eq!(sanitize_project_name("project_123"), "project_123");
        
        // 测试包含空格的名称
        assert_eq!(sanitize_project_name("my project"), "my_project");
        assert_eq!(sanitize_project_name("My Test Project"), "My_Test_Project");
        
        // 测试包含特殊字符的名称
        assert_eq!(sanitize_project_name("project@2024"), "project_2024");
        assert_eq!(sanitize_project_name("test#1"), "test_1");
        assert_eq!(sanitize_project_name("app&demo"), "app_demo");
        
        // 测试以点开头的名称（避免隐藏文件）
        assert_eq!(sanitize_project_name(".hidden"), "hidden");
        
        // 测试空名称
        assert_eq!(sanitize_project_name(""), "unnamed_project");
        assert_eq!(sanitize_project_name("   "), "unnamed_project");
        
        // 测试连续特殊字符
        assert_eq!(sanitize_project_name("test@@@project"), "test_project");
        assert_eq!(sanitize_project_name("a...b"), "a_b");
    }
    
    #[test]
    fn test_create_workspace_directory_structure() {
        use crate::utils::paths::get_workspaces_dir;
        use std::env;
        
        // 使用临时目录进行测试
        let temp_dir = env::temp_dir().join("test-workspace-creation");
        
        // 清理可能存在的旧测试目录
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }
        
        env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保工作区根目录存在
        let workspaces_root = get_workspaces_dir();
        std::fs::create_dir_all(&workspaces_root).expect("Failed to create workspaces root");
        
        // 测试创建项目工作区目录
        let project_name = "test-project";
        let result = create_workspace_directory(project_name);
        
        assert!(result.is_ok(), "Failed to create workspace directory: {:?}", result.err());
        
        let workspace_path = result.unwrap();
        assert!(workspace_path.exists(), "Workspace directory should exist");
        assert!(workspace_path.is_dir(), "Workspace path should be a directory");
        
        // 验证路径结构正确
        assert!(workspace_path.starts_with(&workspaces_root));
        assert_eq!(workspace_path.file_name().unwrap(), project_name);
        
        // 清理测试目录
        env::remove_var("OPC_HARNESS_HOME");
        std::fs::remove_dir_all(&temp_dir).ok();
    }
    
    #[test]
    fn test_create_workspace_directory_with_special_chars() {
        use crate::utils::paths::get_workspaces_dir;
        use std::env;
        
        // 使用临时目录进行测试
        let temp_dir = env::temp_dir().join("test-workspace-special-chars");
        
        // 清理可能存在的旧测试目录
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }
        
        env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保工作区根目录存在
        let workspaces_root = get_workspaces_dir();
        std::fs::create_dir_all(&workspaces_root).expect("Failed to create workspaces root");
        
        // 测试包含特殊字符的项目名称
        let project_name = "My Test@Project#2024";
        let result = create_workspace_directory(project_name);
        
        assert!(result.is_ok(), "Failed to create workspace directory: {:?}", result.err());
        
        let workspace_path = result.unwrap();
        assert!(workspace_path.exists(), "Workspace directory should exist");
        
        // 验证文件名已清理特殊字符
        let dir_name = workspace_path.file_name().unwrap().to_string_lossy();
        assert_eq!(dir_name, "My_Test_Project_2024");
        
        // 清理测试目录
        env::remove_var("OPC_HARNESS_HOME");
        std::fs::remove_dir_all(&temp_dir).ok();
    }
    
    #[test]
    fn test_create_workspace_directory_duplicate_handling() {
        use crate::utils::paths::get_workspaces_dir;
        use std::env;
        use std::thread;
        use std::time::Duration;
        
        // 使用临时目录进行测试
        let temp_dir = env::temp_dir().join("test-workspace-duplicate");
        env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());
        
        // 确保工作区根目录存在
        let workspaces_root = get_workspaces_dir();
        std::fs::create_dir_all(&workspaces_root).expect("Failed to create workspaces root");
        
        let project_name = "duplicate-test";
        
        // 第一次创建
        let result1 = create_workspace_directory(project_name);
        assert!(result1.is_ok());
        let path1 = result1.unwrap();
        
        // 短暂等待以确保时间戳不同
        thread::sleep(Duration::from_millis(1100));
        
        // 第二次创建同名项目
        let result2 = create_workspace_directory(project_name);
        assert!(result2.is_ok());
        let path2 = result2.unwrap();
        
        // 验证两个路径不同（第二个应该包含时间戳）
        assert_ne!(path1, path2, "Duplicate project names should have different paths");
        assert!(path2.to_string_lossy().contains(&project_name.replace(' ', "_")));
        
        // 清理测试目录
        env::remove_var("OPC_HARNESS_HOME");
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}

use crate::db;
use crate::models::{AIConfig, CLISession, Issue, Milestone, Project};
use chrono::Utc;
use uuid::Uuid;

/// 创建新项目
#[tauri::command]
pub fn create_project(
    app_handle: tauri::AppHandle,
    name: String,
    description: String,
) -> Result<String, String> {
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
    Ok(project.id)
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

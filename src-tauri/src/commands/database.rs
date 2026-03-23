use crate::db;
use crate::models::{Project, AIConfig, CLISession};
use uuid::Uuid;
use chrono::Utc;

/// 创建新项目
#[tauri::command]
pub fn create_project(app_handle: tauri::AppHandle, name: String, description: String) -> Result<String, String> {
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
pub fn get_project_by_id(app_handle: tauri::AppHandle, id: String) -> Result<Option<Project>, String> {
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

/// 保存 AI 配置
#[tauri::command]
pub fn save_ai_config(app_handle: tauri::AppHandle, config: AIConfig) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::save_ai_config(&conn, &config).map_err(|e| e.to_string())
}

/// 获取所有 AI 配置
#[tauri::command]
pub fn get_all_ai_configs(app_handle: tauri::AppHandle) -> Result<Vec<AIConfig>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_all_ai_configs(&conn).map_err(|e| e.to_string())
}

/// 获取单个 AI 配置
#[tauri::command]
pub fn get_ai_config(app_handle: tauri::AppHandle, provider: String) -> Result<Option<AIConfig>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_ai_config(&conn, &provider).map_err(|e| e.to_string())
}

/// 删除 AI 配置
#[tauri::command]
pub fn delete_ai_config(app_handle: tauri::AppHandle, provider: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::delete_ai_config(&conn, &provider).map_err(|e| e.to_string())
}

/// 创建 CLI 会话
#[tauri::command]
pub fn create_cli_session_db(app_handle: tauri::AppHandle, tool_type: String, project_path: String) -> Result<String, String> {
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
pub fn get_cli_session_by_id(app_handle: tauri::AppHandle, id: String) -> Result<Option<CLISession>, String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::get_cli_session_by_id(&conn, &id).map_err(|e| e.to_string())
}

/// 删除 CLI 会话
#[tauri::command]
pub fn delete_cli_session(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let conn = db::get_connection(&app_handle).map_err(|e| e.to_string())?;
    db::delete_cli_session(&conn, &id).map_err(|e| e.to_string())
}

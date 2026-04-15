use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersionResponse {
    pub version: String,
    pub name: String,
}

#[tauri::command]
pub fn get_app_version(app_handle: tauri::AppHandle) -> AppVersionResponse {
    let package_info = app_handle.package_info();
    AppVersionResponse {
        version: package_info.version.to_string(),
        name: package_info.name.to_string(),
    }
}

#[tauri::command]
pub async fn open_external_link(_url: String) -> Result<(), String> {
    // TODO: Implement opening external links
    Ok(())
}

/// 将 PRD 内容写入到项目目录下的 PRD.md 文件
/// 用于 CodeFree CLI 读取 PRD 文件
#[tauri::command]
pub async fn write_prd_to_file(project_path: Option<String>, prd_content: String) -> Result<String, String> {
    use tokio::fs;
    use std::env;
    
    // 如果未提供项目路径，使用当前工作目录
    let base_path = if let Some(path) = project_path {
        path
    } else {
        env::current_dir()
            .map_err(|e| format!("获取当前目录失败: {}", e))?
            .to_string_lossy()
            .to_string()
    };
    
    log::info!("[write_prd_to_file] Writing PRD to file in project: {}", base_path);
    
    // 构建文件路径
    let mut file_path = PathBuf::from(&base_path);
    file_path.push("PRD.md");
    
    log::info!("[write_prd_to_file] File path: {:?}", file_path);
    
    // 确保目录存在
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| {
            log::error!("[write_prd_to_file] Failed to create directory: {}", e);
            format!("创建目录失败: {}", e)
        })?;
    }
    
    // 写入文件
    fs::write(&file_path, &prd_content).await.map_err(|e| {
        log::error!("[write_prd_to_file] Failed to write file: {}", e);
        format!("写入文件失败: {}", e)
    })?;
    
    log::info!("[write_prd_to_file] Successfully wrote PRD to {:?}", file_path);
    
    Ok(file_path.to_string_lossy().to_string())
}

/// 从项目目录读取 PRD.md 文件内容
/// 用于 CodeFree CLI 优化后读取最终的 PRD 内容
#[tauri::command]
pub async fn read_prd_from_file(project_path: String) -> Result<String, String> {
    use tokio::fs;
    
    log::info!("[read_prd_from_file] Reading PRD from project: {}", project_path);
    
    // 构建文件路径
    let mut file_path = PathBuf::from(&project_path);
    file_path.push("PRD.md");
    
    log::info!("[read_prd_from_file] File path: {:?}", file_path);
    
    // 检查文件是否存在
    if !file_path.exists() {
        return Err(format!("PRD 文件不存在: {:?}", file_path));
    }
    
    // 读取文件内容
    let content = fs::read_to_string(&file_path).await.map_err(|e| {
        log::error!("[read_prd_from_file] Failed to read file: {}", e);
        format!("读取文件失败: {}", e)
    })?;
    
    log::info!("[read_prd_from_file] Successfully read {} bytes from {:?}", content.len(), file_path);
    
    Ok(content)
}

/// 获取项目的工作区目录路径
/// 
/// 返回: ~/.opc-harness/workspaces/{project_id}/.opc-harness
#[tauri::command]
pub fn get_project_workspace_path(project_id: String) -> Result<String, String> {
    use crate::utils::paths::get_workspaces_dir;
    
    let workspaces_root = get_workspaces_dir();
    let project_workspace = workspaces_root.join(&project_id);
    
    // 在项目中创建 .opc-harness 子目录，用于存放 CodeFree CLI 上下文文件
    let opc_harness_dir = project_workspace.join(".opc-harness");
    
    log::info!("[get_project_workspace_path] Project workspace path: {:?}", project_workspace);
    log::info!("[get_project_workspace_path] OPC-HARNESS context directory: {:?}", opc_harness_dir);
    
    Ok(opc_harness_dir.to_string_lossy().to_string())
}

/// 将内容写入到项目目录下的指定文件
/// 通用文件写入命令，用于 CodeFree CLI 读取上下文文件
#[tauri::command]
pub async fn write_file_to_project(
    project_path: String,
    file_name: String,
    content: String,
) -> Result<String, String> {
    use tokio::fs;
    
    log::info!("[write_file_to_project] Writing file '{}' to project: {}", file_name, project_path);
    
    // 构建文件路径
    let mut file_path = PathBuf::from(&project_path);
    file_path.push(&file_name);
    
    log::info!("[write_file_to_project] File path: {:?}", file_path);
    
    // 确保目录存在
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| {
            log::error!("[write_file_to_project] Failed to create directory: {}", e);
            format!("创建目录失败: {}", e)
        })?;
    }
    
    // 写入文件
    fs::write(&file_path, &content).await.map_err(|e| {
        log::error!("[write_file_to_project] Failed to write file: {}", e);
        format!("写入文件失败: {}", e)
    })?;
    
    log::info!("[write_file_to_project] Successfully wrote {} bytes to {:?}", content.len(), file_path);
    
    Ok(file_path.to_string_lossy().to_string())
}

/// 从项目目录读取指定文件的内容
/// 通用文件读取命令，用于读取 AGENTS.md 等上下文文件
#[tauri::command]
pub async fn read_file_from_project(
    project_path: String,
    file_name: String,
) -> Result<String, String> {
    use tokio::fs;
    
    log::info!("[read_file_from_project] Reading file '{}' from project: {}", file_name, project_path);
    
    // 构建文件路径
    let mut file_path = PathBuf::from(&project_path);
    file_path.push(&file_name);
    
    log::info!("[read_file_from_project] File path: {:?}", file_path);
    
    // 检查文件是否存在
    if !file_path.exists() {
        log::warn!("[read_file_from_project] File does not exist: {:?}", file_path);
        return Err(format!("文件不存在: {}", file_name));
    }
    
    // 读取文件内容
    let content = fs::read_to_string(&file_path).await.map_err(|e| {
        log::error!("[read_file_from_project] Failed to read file: {}", e);
        format!("读取文件失败: {}", e)
    })?;
    
    log::info!("[read_file_from_project] Successfully read {} bytes from {:?}", content.len(), file_path);
    
    Ok(content)
}

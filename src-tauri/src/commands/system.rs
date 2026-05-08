use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersionResponse {
    pub version: String,
    pub name: String,
}

/// Git 仓库状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
    pub is_git_repo: bool,
    pub git_version: Option<String>,
    pub branch: Option<String>,
    pub commit_count: Option<u32>,
    pub is_dirty: Option<bool>,
}

/// Git 配置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub user_name: Option<String>,
    pub user_email: Option<String>,
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
pub async fn write_prd_to_file(
    project_path: Option<String>,
    prd_content: String,
) -> Result<String, String> {
    use std::env;
    use tokio::fs;

    // 如果未提供项目路径，使用当前工作目录
    let base_path = if let Some(path) = project_path {
        path
    } else {
        env::current_dir()
            .map_err(|e| format!("获取当前目录失败: {}", e))?
            .to_string_lossy()
            .to_string()
    };

    log::info!(
        "[write_prd_to_file] Writing PRD to file in project: {}",
        base_path
    );

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

    log::info!(
        "[write_prd_to_file] Successfully wrote PRD to {:?}",
        file_path
    );

    Ok(file_path.to_string_lossy().to_string())
}

/// 从项目目录读取 PRD.md 文件内容
/// 用于 CodeFree CLI 优化后读取最终的 PRD 内容
#[tauri::command]
pub async fn read_prd_from_file(project_path: String) -> Result<String, String> {
    use tokio::fs;

    log::info!(
        "[read_prd_from_file] Reading PRD from project: {}",
        project_path
    );

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

    log::info!(
        "[read_prd_from_file] Successfully read {} bytes from {:?}",
        content.len(),
        file_path
    );

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

    log::info!(
        "[get_project_workspace_path] Project workspace path: {:?}",
        project_workspace
    );
    log::info!(
        "[get_project_workspace_path] OPC-HARNESS context directory: {:?}",
        opc_harness_dir
    );

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

    log::info!(
        "[write_file_to_project] Writing file '{}' to project: {}",
        file_name,
        project_path
    );

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

    log::info!(
        "[write_file_to_project] Successfully wrote {} bytes to {:?}",
        content.len(),
        file_path
    );

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

    log::info!(
        "[read_file_from_project] Reading file '{}' from project: {}",
        file_name,
        project_path
    );

    // 构建文件路径
    let mut file_path = PathBuf::from(&project_path);
    file_path.push(&file_name);

    log::info!("[read_file_from_project] File path: {:?}", file_path);

    // 检查文件是否存在
    if !file_path.exists() {
        log::warn!(
            "[read_file_from_project] File does not exist: {:?}",
            file_path
        );
        return Err(format!("文件不存在: {}", file_name));
    }

    // 读取文件内容
    let content = fs::read_to_string(&file_path).await.map_err(|e| {
        log::error!("[read_file_from_project] Failed to read file: {}", e);
        format!("读取文件失败: {}", e)
    })?;

    log::info!(
        "[read_file_from_project] Successfully read {} bytes from {:?}",
        content.len(),
        file_path
    );

    Ok(content)
}

// ============================================================================
// Git Management Commands
// ============================================================================

/// 检查指定路径的 Git 仓库状态
#[tauri::command]
pub async fn check_git_status(path: String) -> Result<GitStatus, String> {
    use tokio::process::Command;

    log::info!("[check_git_status] Checking Git status for: {}", path);

    let path_buf = PathBuf::from(&path);

    // 检查目录是否存在
    if !path_buf.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }

    // 检查是否是 Git 仓库
    let is_git_repo = tokio::fs::metadata(path_buf.join(".git")).await.is_ok();

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
    let git_version = Command::new("git")
        .arg("--version")
        .output()
        .await
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|v| v.trim().to_string());

    // 获取当前分支
    let branch = Command::new("git")
        .current_dir(&path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .await
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|b| b.trim().to_string());

    // 获取提交数量
    let commit_count = Command::new("git")
        .current_dir(&path)
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .await
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|c| c.trim().parse::<u32>().ok());

    // 检查工作区是否干净
    let is_dirty = Command::new("git")
        .current_dir(&path)
        .args(["status", "--porcelain"])
        .output()
        .await
        .ok()
        .map(|output| !output.stdout.is_empty());

    log::info!(
        "[check_git_status] Git repo: {}, branch: {:?}, commits: {:?}",
        is_git_repo,
        branch,
        commit_count
    );

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
pub async fn init_git_repo(path: String, initial_branch: Option<String>) -> Result<bool, String> {
    use tokio::process::Command;

    log::info!("[init_git_repo] Initializing Git repo at: {}", path);

    let path_buf = PathBuf::from(&path);

    // 检查目录是否存在
    if !path_buf.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }

    // 检查是否已经是 Git 仓库
    if path_buf.join(".git").exists() {
        return Err("Git repository already initialized".to_string());
    }

    // 执行 git init
    let branch_arg = initial_branch.unwrap_or_else(|| "main".to_string());
    let output = Command::new("git")
        .current_dir(&path)
        .args(["init", "-b", &branch_arg])
        .output()
        .await
        .map_err(|e| format!("Failed to execute git init: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git init failed: {}", stderr));
    }

    log::info!(
        "[init_git_repo] Git initialized with branch: {}",
        branch_arg
    );

    // 创建 .gitignore 文件
    create_gitignore(&path).await?;

    // 配置用户信息（如果全局未配置）
    ensure_git_user_config(&path).await?;

    // 创建初始空 commit
    let commit_output = Command::new("git")
        .current_dir(&path)
        .args(["commit", "--allow-empty", "-m", "Initial commit"])
        .output()
        .await
        .map_err(|e| format!("Failed to create initial commit: {}", e))?;

    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        log::warn!("[init_git_repo] Initial commit warning: {}", stderr);
    } else {
        log::info!("[init_git_repo] Initial commit created");
    }

    Ok(true)
}

/// 设置 Git 配置项
#[tauri::command]
pub async fn set_git_config(path: String, key: String, value: String) -> Result<bool, String> {
    use tokio::process::Command;

    log::info!("[set_git_config] Setting {} = {} in {}", key, value, path);

    let path_buf = PathBuf::from(&path);

    // 检查是否是 Git 仓库
    if !path_buf.join(".git").exists() {
        return Err("Not a Git repository".to_string());
    }

    let output = Command::new("git")
        .current_dir(&path)
        .args(["config", &key, &value])
        .output()
        .await
        .map_err(|e| format!("Failed to execute git config: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git config failed: {}", stderr));
    }

    Ok(true)
}

/// 获取单个 Git 配置项
#[tauri::command]
pub async fn get_git_config(path: String, key: String) -> Result<Option<String>, String> {
    use tokio::process::Command;

    log::info!("[get_git_config] Getting {} from {}", key, path);

    let output = Command::new("git")
        .current_dir(&path)
        .args(["config", &key])
        .output()
        .await
        .map_err(|e| format!("Failed to execute git config: {}", e))?;

    if !output.status.success() {
        // 配置项不存在时返回 None
        return Ok(None);
    }

    let value = String::from_utf8_lossy(&output.stdout);
    Ok(Some(value.trim().to_string()))
}

/// 获取所有 Git 配置（userName 和 userEmail）
#[tauri::command]
pub async fn get_all_git_config(path: String) -> Result<GitConfig, String> {
    log::info!("[get_all_git_config] Getting all Git config for {}", path);

    let user_name = get_git_config(path.clone(), "user.name".to_string()).await?;
    let user_email = get_git_config(path, "user.email".to_string()).await?;

    Ok(GitConfig {
        user_name,
        user_email,
    })
}

// ============================================================================
// Git Helper Functions
// ============================================================================

/// 创建 .gitignore 文件
async fn create_gitignore(project_path: &str) -> Result<(), String> {
    use tokio::fs;

    let gitignore_path = PathBuf::from(project_path).join(".gitignore");

    // 如果 .gitignore 已存在，跳过创建
    if gitignore_path.exists() {
        log::info!("[create_gitignore] .gitignore already exists, skipping");
        return Ok(());
    }

    // OPC-HARNESS 项目的标准 .gitignore 内容
    let content = "# OPC-HARNESS context files\n.opc-harness/\n";

    fs::write(&gitignore_path, content)
        .await
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;

    log::info!(
        "[create_gitignore] Created .gitignore at {:?}",
        gitignore_path
    );
    Ok(())
}

/// 确保 Git 用户配置存在
async fn ensure_git_user_config(project_path: &str) -> Result<(), String> {
    use tokio::process::Command;

    // 检查全局配置
    let user_name_check = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .await;

    let needs_config = match user_name_check {
        Ok(output) => output.stdout.is_empty(),
        Err(_) => true,
    };

    if needs_config {
        log::info!("[ensure_git_user_config] Setting default Git user config");

        // 设置默认用户名
        Command::new("git")
            .current_dir(project_path)
            .args(["config", "user.name", "OPC-HARNESS User"])
            .output()
            .await
            .map_err(|e| format!("Failed to set user.name: {}", e))?;

        // 设置默认邮箱
        Command::new("git")
            .current_dir(project_path)
            .args(["config", "user.email", "harness@opc.local"])
            .output()
            .await
            .map_err(|e| format!("Failed to set user.email: {}", e))?;

        log::info!("[ensure_git_user_config] Default Git user config set");
    } else {
        log::info!("[ensure_git_user_config] Global Git config exists, using it");
    }

    Ok(())
}

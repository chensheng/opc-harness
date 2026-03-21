use serde::{Deserialize, Serialize};
use tauri::command;

// 系统命令
#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to OPC-HARNESS!", name)
}

#[command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// AI配置相关命令
#[derive(Debug, Serialize, Deserialize)]
pub struct AIConfig {
    provider: String,
    api_key: String,
    base_url: Option<String>,
    default_model: String,
}

#[command]
pub async fn save_ai_config(config: AIConfig) -> Result<(), String> {
    // TODO: 使用keyring保存API密钥
    // TODO: 保存其他配置到SQLite
    println!("Saving AI config for provider: {}", config.provider);
    Ok(())
}

#[command]
pub async fn get_ai_config(provider: String) -> Result<Option<AIConfig>, String> {
    // TODO: 从keyring和SQLite读取配置
    println!("Getting AI config for provider: {}", provider);
    Ok(None)
}

#[command]
pub async fn validate_ai_key(provider: String, api_key: String) -> Result<bool, String> {
    // TODO: 调用各厂商API验证密钥有效性
    println!("Validating API key for provider: {}", provider);
    
    // 模拟验证
    match provider.as_str() {
        "openai" => Ok(api_key.starts_with("sk-")),
        "anthropic" => Ok(api_key.starts_with("sk-ant-")),
        "kimi" => Ok(!api_key.is_empty()),
        "glm" => Ok(!api_key.is_empty()),
        _ => Ok(false),
    }
}

// 工具检测命令
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolStatus {
    name: String,
    installed: bool,
    version: Option<String>,
    path: Option<String>,
}

#[command]
pub async fn detect_tool(command: String) -> Result<ToolStatus, String> {
    use std::process::Command;
    
    let output = Command::new(&command)
        .arg("--version")
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            Ok(ToolStatus {
                name: command.clone(),
                installed: true,
                version: Some(version),
                path: None,
            })
        }
        _ => Ok(ToolStatus {
            name: command,
            installed: false,
            version: None,
            path: None,
        }),
    }
}

#[command]
pub async fn get_tool_status() -> Result<Vec<ToolStatus>, String> {
    let tools = vec![
        "node",
        "npm",
        "git",
        "kimi",
        "claude",
        "codex",
        "docker",
    ];
    
    let mut statuses = Vec::new();
    for tool in tools {
        let status = detect_tool(tool.to_string()).await?;
        statuses.push(status);
    }
    
    Ok(statuses)
}

// 项目相关命令
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
    status: String,
    local_path: String,
}

#[command]
pub async fn create_project(name: String, description: Option<String>) -> Result<Project, String> {
    use uuid::Uuid;
    use chrono::Utc;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // TODO: 创建本地目录
    // TODO: 保存到SQLite
    
    Ok(Project {
        id,
        name,
        description: description.unwrap_or_default(),
        created_at: now.clone(),
        updated_at: now,
        status: "designing".to_string(),
        local_path: format!("~/OPC-HARNESS/{}", name),
    })
}

#[command]
pub async fn get_projects() -> Result<Vec<Project>, String> {
    // TODO: 从SQLite查询所有项目
    Ok(vec![])
}

#[command]
pub async fn get_project(id: String) -> Result<Option<Project>, String> {
    // TODO: 从SQLite查询单个项目
    println!("Getting project: {}", id);
    Ok(None)
}

#[command]
pub async fn update_project(id: String, name: Option<String>, description: Option<String>) -> Result<Project, String> {
    // TODO: 更新项目信息
    println!("Updating project: {}", id);
    Err("Not implemented".to_string())
}

#[command]
pub async fn delete_project(id: String) -> Result<(), String> {
    // TODO: 删除项目（标记删除或物理删除）
    println!("Deleting project: {}", id);
    Ok(())
}

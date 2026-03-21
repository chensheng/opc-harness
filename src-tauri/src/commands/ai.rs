//! AI configuration commands

use crate::models::{AIProviderConfig, AIProviderMeta};
use crate::services::Services;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Get all AI provider metadata (VD-001)
/// 获取所有 AI 厂商的元数据信息，用于 UI 显示
#[tauri::command]
pub fn get_ai_providers() -> Vec<AIProviderMeta> {
    AIProviderMeta::get_all_providers()
}

/// Get specific AI provider metadata (VD-001)
/// 获取指定 AI 厂商的元数据信息
#[tauri::command]
pub fn get_ai_provider(provider_id: String) -> Option<AIProviderMeta> {
    AIProviderMeta::get_provider(&provider_id)
}

/// Get AI configurations
#[tauri::command]
pub fn get_ai_configs(services: State<'_, Services>) -> Result<Vec<AIProviderConfig>, String> {
    // 从数据库获取配置列表（不包含密钥）
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    let mut stmt = db.prepare(
        "SELECT provider, base_url, model, enabled FROM ai_configs ORDER BY provider"
    ).map_err(|e| e.to_string())?;
    
    let configs = stmt.query_map([], |row| {
        let provider: String = row.get(0)?;
        let base_url: Option<String> = row.get(1)?;
        let model: String = row.get(2)?;
        let enabled: i32 = row.get(3)?;
        
        Ok(AIProviderConfig {
            provider,
            api_key: None, // 密钥不从前端获取，从 keyring 获取
            base_url,
            model,
            enabled: enabled == 1,
            name: None, // 暂时不支持自定义名称
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for config in configs {
        result.push(config.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

/// Save AI configuration
#[tauri::command]
pub fn save_ai_config(
    services: State<'_, Services>,
    config: AIProviderConfig,
) -> Result<(), String> {
    // 1. 将 API 密钥保存到 OS 密钥存储
    if let Some(api_key) = &config.api_key {
        if !api_key.is_empty() {
            services
                .keyring
                .set_ai_api_key(&config.provider, api_key)
                .map_err(|e| format!("Failed to store API key: {}", e))?;
            log::info!("API key stored for provider: {}", config.provider);
        }
    }
    
    // 2. 将其他配置保存到数据库（不包含密钥）
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    db.execute(
        "INSERT OR REPLACE INTO ai_configs (provider, base_url, model, enabled) 
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            &config.provider,
            config.base_url,
            &config.model,
            if config.enabled { 1 } else { 0 }
        ],
    ).map_err(|e| format!("Failed to save AI config: {}", e))?;
    
    log::info!("AI config saved for provider: {}", config.provider);
    Ok(())
}

/// Remove AI configuration
#[tauri::command]
pub fn remove_ai_config(services: State<'_, Services>, provider: String) -> Result<(), String> {
    // 1. 从 OS 密钥存储删除 API 密钥
    match services.keyring.delete_ai_api_key(&provider) {
        Ok(_) => log::info!("API key deleted for provider: {}", provider),
        Err(crate::services::keyring_service::KeyringError::Keyring(keyring::Error::NoEntry)) => {
            // 密钥不存在，忽略错误
            log::debug!("No API key found to delete for provider: {}", provider);
        }
        Err(e) => return Err(format!("Failed to delete API key: {}", e)),
    }
    
    // 2. 从数据库删除配置
    let db = services.project.get_db();
    let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    
    db.execute(
        "DELETE FROM ai_configs WHERE provider = ?1",
        [&provider],
    ).map_err(|e| format!("Failed to remove AI config: {}", e))?;
    
    log::info!("AI config removed for provider: {}", provider);
    Ok(())
}

/// Get AI API key (for internal use, not exposed to frontend)
pub fn get_ai_api_key(services: State<'_, Services>, provider: &str) -> Result<Option<String>, String> {
    services
        .keyring
        .get_ai_api_key(provider)
        .map_err(|e| format!("Failed to get API key: {}", e))
}

/// Check if AI API key exists
#[tauri::command]
pub fn has_ai_api_key(services: State<'_, Services>, provider: String) -> Result<bool, String> {
    services
        .keyring
        .has_ai_api_key(&provider)
        .map_err(|e| format!("Failed to check API key: {}", e))
}

/// Validate AI API key (VD-004)
#[tauri::command]
pub async fn validate_ai_key(
    services: State<'_, Services>,
    provider: String,
    api_key: String,
) -> Result<bool, String> {
    use crate::models::AIProviderConfig;
    use crate::services::ai_service::AIService;
    
    // 先从数据库获取配置信息（同步操作）
    let config_info = {
        let db = services.project.get_db();
        let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        
        db.query_row(
            "SELECT base_url, model FROM ai_configs WHERE provider = ?1",
            rusqlite::params![&provider],
            |row| {
                Ok((
                    row.get::<_, Option<String>>(0)?,
                    row.get::<_, String>(1)?,
                ))
            }
        )
    };
    
    let (base_url, model) = match config_info {
        Ok((url, m)) => (url, m),
        Err(_) => (None, String::new()),
    };
    
    // 创建临时的 AI 服务实例进行验证
    let config = AIProviderConfig {
        provider: provider.clone(),
        api_key: Some(api_key.clone()),
        base_url,
        model,
        enabled: false,
        name: None,
    };
    
    let ai_service = AIService::new(config);
    
    // 调用验证方法（异步操作，此时数据库锁已释放）
    match ai_service.validate_key().await {
        Ok(is_valid) => {
            if is_valid {
                log::info!("API key validation successful for provider: {}", provider);
                Ok(true)
            } else {
                log::warn!("API key validation failed for provider: {}", provider);
                Ok(false)
            }
        }
        Err(e) => {
            log::error!("API key validation error for {}: {}", provider, e);
            Err(format!("验证失败：{}", e))
        }
    }
}

/// Generate PRD (VD-017)
#[tauri::command]
pub async fn generate_prd(
    services: State<'_, Services>,
    idea: String,
) -> Result<String, String> {
    use crate::services::ai_service::AIService;
    
    // 获取默认的 AI Provider
    let default_provider = "openai"; // TODO: 从配置中读取
    
    // 调用 AI 服务生成 PRD
    let result = services.ai.generate_prd(default_provider, &idea).await;
    
    match result {
        Ok(prd) => Ok(prd),
        Err(e) => Err(format!("PRD generation failed: {}", e)),
    }
}

/// Get all AI provider key status (for UI display)
#[tauri::command]
pub fn get_ai_key_status(services: State<'_, Services>) -> Result<Vec<(String, bool)>, String> {
    services
        .keyring
        .get_all_ai_api_keys()
        .map_err(|e| format!("Failed to get AI key status: {}", e))
}

/// Get detailed API key status for a specific provider (VD-002)
/// 返回密钥是否已存储以及验证状态
#[derive(Debug, Clone, Serialize)]
pub struct ApiKeyStatus {
    pub has_key: bool,
    pub is_validated: bool,
    pub last_verified: Option<i64>,
}

#[tauri::command]
pub async fn get_api_key_status(
    services: State<'_, Services>,
    provider: String,
) -> Result<ApiKeyStatus, String> {
    // 检查是否有密钥
    let has_key = services
        .keyring
        .has_ai_api_key(&provider)
        .map_err(|e| format!("Failed to check API key status: {}", e))?;
    
    // TODO: 从配置中读取最后验证时间
    // 目前简化处理，如果密钥存在就认为已验证
    let is_validated = has_key;
    let last_verified = if has_key { 
        Some(chrono::Utc::now().timestamp()) 
    } else { 
        None 
    };
    
    Ok(ApiKeyStatus {
        has_key,
        is_validated,
        last_verified,
    })
}

// ==================== VD-014 AI 服务管理器统一入口命令 ====================

/// 获取所有可用的 AI Provider 列表
#[tauri::command]
pub fn get_available_providers(services: State<'_, Services>) -> Vec<String> {
    services.ai.registered_providers()
        .into_iter()
        .map(String::from)
        .collect()
}

/// 获取指定 Provider 支持的模型列表
#[tauri::command]
pub fn get_provider_models(
    services: State<'_, Services>,
    provider: String,
) -> Result<Vec<String>, String> {
    let models = services.ai.get_supported_models(&provider)
        .into_iter()
        .map(String::from)
        .collect();
    
    Ok(models)
}

/// 验证指定 Provider 的 API 密钥
#[tauri::command]
pub async fn validate_provider_key(
    services: State<'_, Services>,
    provider: String,
) -> Result<bool, String> {
    services.ai.validate_api_key(&provider).await
        .map_err(|e| format!("Validation failed: {}", e))
}

/// 使用指定 Provider 进行聊天
#[tauri::command]
pub async fn ai_chat(
    services: State<'_, Services>,
    provider: String,
    messages: Vec<serde_json::Value>,
) -> Result<String, String> {
    services.ai.chat(&provider, messages).await
        .map_err(|e| format!("Chat failed: {}", e))
}

/// 生成 PRD（产品需求文档）
#[tauri::command]
pub async fn ai_generate_prd(
    services: State<'_, Services>,
    provider: String,
    idea: String,
) -> Result<String, String> {
    services.ai.generate_prd(&provider, &idea).await
        .map_err(|e| format!("PRD generation failed: {}", e))
}

/// 生成用户画像
#[tauri::command]
pub async fn ai_generate_personas(
    services: State<'_, Services>,
    provider: String,
    idea: String,
) -> Result<Vec<serde_json::Value>, String> {
    services.ai.generate_personas(&provider, &idea).await
        .map_err(|e| format!("Personas generation failed: {}", e))
}

/// 生成竞品分析
#[tauri::command]
pub async fn ai_generate_competitor_analysis(
    services: State<'_, Services>,
    provider: String,
    idea: String,
) -> Result<String, String> {
    services.ai.generate_competitor_analysis(&provider, &idea).await
        .map_err(|e| format!("Competitor analysis failed: {}", e))
}

/// 流式聊天 - SSE（Server-Sent Events）
/// 这个命令会持续推送消息片段到前端回调函数
#[tauri::command]
pub async fn ai_stream_chat(
    services: State<'_, Services>,
    provider: String,
    messages: Vec<serde_json::Value>,
    callback: tauri::ipc::Channel<String>,
) -> Result<(), String> {
    // 将 callback 包装成 Box<dyn Fn(String) + Send + Sync>
    let callback = Box::new(move |chunk: String| {
        // 通过 Channel 发送数据到前端
        if let Err(e) = callback.send(chunk) {
            log::error!("Failed to send stream chunk: {}", e);
        }
    });
    
    services.ai.stream_chat(&provider, messages, callback).await
        .map_err(|e| format!("Stream chat failed: {}", e))
}

// ============================================================
// VD-021: PRD 保存功能
// ============================================================

use crate::models::PrdDocument;
use std::path::PathBuf;

/// Save PRD to database and local file (VD-021)
/// 保存 PRD 到数据库和本地文件
#[tauri::command]
pub async fn save_prd(
    services: State<'_, Services>,
    project_id: String,
    content: String,
    version: Option<i32>,
) -> Result<PrdDocument, String> {
    // 1. 获取项目信息，验证项目是否存在
    let project = services.project.get_project(&project_id)
        .map_err(|e| format!("Failed to get project: {}", e))?;
    
    if project.is_none() {
        return Err(format!("Project not found: {}", project_id));
    }
    
    let project = project.unwrap();
    
    // 2. 检查是否已有 PRD，如果有则创建新版本
    let existing_prd = services.project.get_latest_prd(&project_id)
        .map_err(|e| format!("Failed to get existing PRD: {}", e))?;
    
    let prd = match (existing_prd, version) {
        (Some(existing), Some(v)) if v == existing.version => {
            // 更新现有版本
            let mut updated_prd = existing;
            updated_prd.content = content;
            updated_prd.updated_at = chrono::Utc::now().timestamp();
            updated_prd
        }
        (Some(existing), _) => {
            // 创建新版本
            existing.new_version(&content)
        }
        _ => {
            // 创建新 PRD
            PrdDocument::new(&project_id, &content)
        }
    };
    
    // 3. 保存到数据库
    services.project.save_prd(&prd)
        .map_err(|e| format!("Failed to save PRD to database: {}", e))?;
    
    // 4. 保存到本地文件
    if let Some(project_path) = &project.path {
        let prd_dir = PathBuf::from(project_path).join(".opc-harness");
        std::fs::create_dir_all(&prd_dir)
            .map_err(|e| format!("Failed to create PRD directory: {}", e))?;
        
        let prd_file_path = prd_dir.join("prd.md");
        crate::utils::save_to_file(&prd_file_path, &prd.content)
            .map_err(|e| format!("Failed to save PRD to file: {}", e))?;
        
        log::info!("PRD saved to file: {:?}", prd_file_path);
    }
    
    log::info!("PRD saved successfully: id={}, version={}", prd.id, prd.version);
    Ok(prd)
}

/// Get PRD by ID (VD-021)
/// 根据 ID 获取 PRD
#[tauri::command]
pub fn get_prd(
    services: State<'_, Services>,
    prd_id: String,
) -> Result<Option<PrdDocument>, String> {
    services.project.get_prd(&prd_id)
        .map_err(|e| format!("Failed to get PRD: {}", e))
}

/// Get latest PRD for project (VD-021)
/// 获取项目的最新 PRD
#[tauri::command]
pub fn get_latest_prd(
    services: State<'_, Services>,
    project_id: String,
) -> Result<Option<PrdDocument>, String> {
    services.project.get_latest_prd(&project_id)
        .map_err(|e| format!("Failed to get latest PRD: {}", e))
}

/// Get all PRDs for project (VD-021)
/// 获取项目的所有 PRD 历史版本
#[tauri::command]
pub fn get_prds_by_project(
    services: State<'_, Services>,
    project_id: String,
) -> Result<Vec<PrdDocument>, String> {
    services.project.get_prds_by_project(&project_id)
        .map_err(|e| format!("Failed to get PRDs: {}", e))
}

// ============================================================
// VD-022: PRD 导出 Markdown 功能
// ============================================================

/// Export PRD to Markdown file
/// 导出 PRD 为 Markdown 文件到本地
#[tauri::command]
pub async fn export_prd_to_markdown(
    services: State<'_, Services>,
    project_id: String,
    content: String,
    filename: Option<String>,
) -> Result<String, String> {
    // 1. 获取项目信息，用于生成默认文件名
    let project = services.project.get_project(&project_id)
        .map_err(|e| format!("Failed to get project: {}", e))?;
    
    if project.is_none() {
        return Err(format!("Project not found: {}", project_id));
    }
    
    let project = project.unwrap();
    
    // 2. 生成文件名
    let file_name = match filename {
        Some(name) => name,
        None => {
            // 使用项目标题生成文件名，去除特殊字符
            let title = project.name.trim();
            let safe_title: String = title.chars()
                .filter(|c: &char| c.is_alphanumeric() || c.is_whitespace())
                .collect::<String>()
                .replace(" ", "-")
                .to_lowercase();
            
            format!("PRD-{}-{}.md", safe_title, chrono::Local::now().format("%Y%m%d"))
        }
    };

    // 3. 确定保存路径（用户选择的目录或默认目录）
    let save_path = services.project.get_default_save_dir()
        .join(&file_name);
    
    // 4. 保存到文件
    crate::utils::save_to_file(&save_path, &content)
        .map_err(|e| format!("Failed to save file: {}", e))?;
    
    log::info!("Exported PRD to: {:?}", save_path);
    
    Ok(save_path.to_string_lossy().to_string())
}

// ============================================================
// VD-024: 用户画像生成 API
// ============================================================

/// Generate user personas (VD-024)
/// 根据产品创意生成用户画像
#[tauri::command]
pub async fn generate_user_personas(
    services: State<'_, Services>,
    project_id: String,
    idea: String,
) -> Result<Vec<serde_json::Value>, String> {
    use crate::services::ai_service::AIService;
    
    // 1. 获取项目信息
    let project = services.project.get_project(&project_id)
        .map_err(|e| format!("Failed to get project: {}", e))?;
    
    if project.is_none() {
        return Err(format!("Project not found: {}", project_id));
    }
    
    let project = project.unwrap();
    
    // 2. 获取 AI 配置（从数据库和 keyring）
    let config_info = {
        let db = services.project.get_db();
        let db = db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
        
        db.query_row(
            "SELECT provider, base_url, model FROM ai_configs WHERE enabled = 1 LIMIT 1",
            [],
            |row| {
                let provider: String = row.get(0)?;
                let base_url: Option<String> = row.get(1)?;
                let model: String = row.get(2)?;
                Ok((provider, base_url, model))
            }
        ).ok()
    };
    
    let (provider, base_url, model) = config_info.ok_or_else(|| {
        "No enabled AI provider configured. Please configure at least one AI provider.".to_string()
    })?;
    
    // 3. 从 keyring 获取 API 密钥
    let api_key = services.keyring.get_ai_api_key(&provider)
        .map_err(|e| format!("Failed to get API key: {}", e))?
        .ok_or_else(|| {
            format!("API key not found for provider: {}. Please configure the API key in settings.", provider)
        })?;
    
    // 4. 创建 AI 服务实例
    let config = AIProviderConfig {
        provider,
        api_key: Some(api_key),
        base_url,
        model,
        enabled: true,
        name: None,
    };
    
    let ai_service = AIService::new(config);
    
    // 5. 调用 AI 生成用户画像
    let personas = ai_service.generate_personas(&idea).await
        .map_err(|e| format!("Failed to generate user personas: {}", e))?;
    
    log::info!("Generated {} user personas for project: {}", personas.len(), project_id);
    
    Ok(personas)
}

/// Save user personas to project (VD-024)
/// 保存用户画像到项目
#[tauri::command]
pub fn save_user_personas(
    services: State<'_, Services>,
    project_id: String,
    personas: Vec<serde_json::Value>,
) -> Result<(), String> {
    use std::path::PathBuf;
    use serde_json;
    
    // 1. 获取项目信息
    let project = services.project.get_project(&project_id)
        .map_err(|e| format!("Failed to get project: {}", e))?;
    
    if project.is_none() {
        return Err(format!("Project not found: {}", project_id));
    }
    
    let project = project.unwrap();
    
    // 2. 将 personas 转换为 JSON 字符串
    let json_content = serde_json::to_string_pretty(&personas)
        .map_err(|e| format!("Failed to serialize personas: {}", e))?;
    
    // 3. 保存到项目目录
    if let Some(project_path) = &project.path {
        let personas_dir = PathBuf::from(project_path).join(".opc-harness");
        std::fs::create_dir_all(&personas_dir)
            .map_err(|e| format!("Failed to create personas directory: {}", e))?;
        
        let personas_file_path = personas_dir.join("user_personas.json");
        crate::utils::save_to_file(&personas_file_path, &json_content)
            .map_err(|e| format!("Failed to save personas to file: {}", e))?;
        
        log::info!("User personas saved to file: {:?}", personas_file_path);
    }
    
    // 4. 同时保存到数据库（可选，如果需要结构化存储）
    // TODO: 如果需要，可以在数据库中添加 user_personas 表
    
    log::info!("User personas saved successfully for project: {}", project_id);
    Ok(())
}

/// Get user personas for project (VD-024)
/// 获取项目的用户画像
#[tauri::command]
pub fn get_user_personas(
    services: State<'_, Services>,
    project_id: String,
) -> Result<Option<Vec<serde_json::Value>>, String> {
    use std::path::PathBuf;
    use std::fs;
    
    // 1. 获取项目信息
    let project = services.project.get_project(&project_id)
        .map_err(|e| format!("Failed to get project: {}", e))?;
    
    if project.is_none() {
        return Err(format!("Project not found: {}", project_id));
    }
    
    let project = project.unwrap();
    
    // 2. 尝试从文件读取
    if let Some(project_path) = &project.path {
        let personas_file_path = PathBuf::from(project_path)
            .join(".opc-harness")
            .join("user_personas.json");
        
        if personas_file_path.exists() {
            let content = fs::read_to_string(&personas_file_path)
                .map_err(|e| format!("Failed to read personas file: {}", e))?;
            
            let personas: Vec<serde_json::Value> = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse personas JSON: {}", e))?;
            
            return Ok(Some(personas));
        }
    }
    
    // 3. 如果文件不存在，返回 None
    Ok(None)
}

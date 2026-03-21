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

/// Generate PRD
#[tauri::command]
pub async fn generate_prd(
    _services: State<'_, Services>,
    idea: String,
) -> Result<String, String> {
    // TODO: Call AI service to generate PRD
    Ok(format!("# PRD for: {}\n\n(Generated content)", idea))
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

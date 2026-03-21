//! AI configuration commands

use crate::models::AIProviderConfig;
use crate::services::Services;
use tauri::State;

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

/// Validate AI API key
#[tauri::command]
pub async fn validate_ai_key(
    services: State<'_, Services>,
    provider: String,
    api_key: String,
) -> Result<bool, String> {
    // 临时存储密钥进行验证
    services
        .keyring
        .set_ai_api_key(&format!("{}_temp", provider), &api_key)
        .map_err(|e| format!("Failed to store temporary API key: {}", e))?;
    
    // TODO: 实际调用 AI provider API 验证密钥有效性
    // 这里先模拟验证
    let is_valid = !api_key.is_empty() && api_key.len() > 10;
    
    // 清理临时密钥
    let _ = services.keyring.delete_ai_api_key(&format!("{}_temp", provider));
    
    log::info!("API key validation for {}: {}", provider, is_valid);
    Ok(is_valid)
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

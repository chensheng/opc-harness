use crate::ai::{AIProvider, AIProviderType};
use crate::utils::keychain;
use crate::commands::ai::types::{ValidateKeyRequest, SaveApiKeyRequest, GetApiKeyRequest, DeleteApiKeyRequest};

/// 验证 AI API Key
#[tauri::command]
pub async fn validate_ai_key(request: ValidateKeyRequest) -> Result<bool, String> {
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        _ => return Err("Unsupported provider".to_string()),
    };

    log::info!("Validating API key - Provider: {}, Model: {:?}", request.provider, request.model);

    // 对于 Kimi，需要根据 model 判断使用哪个 API
    if provider_type == AIProviderType::Kimi {
        // 检查是否是 Kimi Coding 模型
        let is_coding_model = request.model.as_ref().map_or(false, |m| {
            m.starts_with("kimi-coding") || m == "kimi-code"
        });
        
        log::info!("Is Kimi Coding model: {}", is_coding_model);
        
        if is_coding_model {
            log::info!("Validating Kimi Coding API key for model: {}", request.model.unwrap_or_default());
            // 使用 Kimi Coding API (Anthropic-compatible)
            return validate_kimi_coding_key(&request.api_key).await;
        } else {
            log::info!("Validating standard Kimi API key with model: {:?}", request.model);
            // 使用标准 Kimi API (OpenAI-compatible)
            let provider = AIProvider::new(provider_type, request.api_key);
            return provider.validate_key().await.map_err(|e| e.to_string());
        }
    }
    
    // 其他 provider 使用默认验证逻辑
    let provider = AIProvider::new(provider_type, request.api_key);
    provider.validate_key().await.map_err(|e| e.to_string())
}

/// 验证 Kimi Coding API Key (使用 Anthropic-compatible API)
async fn validate_kimi_coding_key(api_key: &str) -> Result<bool, String> {
    use reqwest::Client;
    use serde_json::json;
    
    let client = Client::new();
    let url = "https://api.kimi.com/coding/v1/messages";
    
    log::info!("Kimi Coding validation request:");
    log::info!("  URL: {}", url);
    log::info!("  Auth header: Bearer {}", api_key);
    log::info!("  API Key length: {}", api_key.len());
    log::info!("  API Key prefix: {}", &api_key[..8.min(api_key.len())]);
    
    let body = json!({
        "model": "kimi-for-coding",
        "messages": [
            {"role": "user", "content": "Hi"}
        ],
        "max_tokens": 1
    });
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            log::error!("Kimi Coding validation request failed: {}", e);
            format!("Kimi Coding API 验证请求失败：{}", e)
        })?;
    
    if response.status().is_success() {
        log::info!("Kimi Coding validation successful!");
        Ok(true)
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        log::error!("Kimi Coding validation failed ({}): {}", status, text);
        Err(format!("Kimi Coding API 返回错误 ({}): {}", status, text))
    }
}

/// Save API key securely to OS keychain
#[allow(dead_code)]
#[tauri::command]
pub fn save_api_key_to_keychain(request: SaveApiKeyRequest) -> Result<bool, String> {
    // Validate inputs
    if request.provider.is_empty() {
        return Err("Provider name cannot be empty".to_string());
    }

    if request.model.is_empty() {
        return Err("Model name cannot be empty".to_string());
    }

    if request.api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    // Save to OS keychain
    keychain::save_api_key(&request.provider, &request.api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))?;

    Ok(true)
}

/// Retrieve API key from OS keychain
#[allow(dead_code)]
#[tauri::command]
pub fn get_api_key_from_keychain(request: GetApiKeyRequest) -> Result<String, String> {
    keychain::get_api_key(&request.provider)
        .map_err(|e| format!("Failed to retrieve API key: {}", e))
}

/// Check if API key exists in OS keychain
#[allow(dead_code)]
#[tauri::command]
pub fn has_api_key_in_keychain(provider: String) -> Result<bool, String> {
    Ok(keychain::has_api_key(&provider))
}

/// Delete API key from OS keychain
#[allow(dead_code)]
#[tauri::command]
pub fn delete_api_key_from_keychain(request: DeleteApiKeyRequest) -> Result<(), String> {
    keychain::delete_api_key(&request.provider)
        .map_err(|e| format!("Failed to delete API key: {}", e))
}

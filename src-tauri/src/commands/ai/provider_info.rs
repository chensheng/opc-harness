use crate::commands::ai::types::ProviderInfo;

/// AI-005: 获取所有可用的 AI Provider 列表
#[tauri::command]
pub fn get_available_providers() -> Vec<ProviderInfo> {
    let mut providers = Vec::new();
    
    // OpenAI
    providers.push(ProviderInfo {
        id: "openai".to_string(),
        name: "OpenAI".to_string(),
        models: vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string(), "o1-preview".to_string(), "o1-mini".to_string()],
    });
    
    // Anthropic Claude
    providers.push(ProviderInfo {
        id: "anthropic".to_string(),
        name: "Anthropic Claude".to_string(),
        models: vec!["claude-3-5-sonnet-20241022".to_string(), "claude-3-opus-20240229".to_string(), "claude-3-sonnet-20240229".to_string(), "claude-3-haiku-20240307".to_string()],
    });
    
    // Kimi
    providers.push(ProviderInfo {
        id: "kimi".to_string(),
        name: "月之暗面 (Kimi)".to_string(),
        models: vec!["kimi-code".to_string(), "kimi-k1.5".to_string(), "kimi-k1".to_string()],
    });
    
    // GLM
    providers.push(ProviderInfo {
        id: "glm".to_string(),
        name: "智谱 AI (GLM)".to_string(),
        models: vec!["glm-4-plus".to_string(), "glm-4".to_string(), "glm-4-air".to_string(), "codegeex-4".to_string()],
    });
    
    // MiniMax
    providers.push(ProviderInfo {
        id: "minimax".to_string(),
        name: "MiniMax".to_string(),
        models: vec!["speech-2.5-turbo".to_string(), "speech-2-turbo".to_string(), "speech-v1".to_string()],
    });
    
    log::info!("[get_available_providers] Returning {} providers", providers.len());
    for provider in &providers {
        log::info!("[get_available_providers] Provider: {} - {}, Models: {:?}", 
                   provider.id, provider.name, provider.models);
    }
    
    providers
}

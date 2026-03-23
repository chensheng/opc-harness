use crate::ai::{
    AIProvider, AIProviderType, ChatRequest, StreamChunk, StreamComplete, StreamError,
};
use crate::utils::keychain;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateKeyRequest {
    pub provider: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveApiKeyRequest {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiKeyRequest {
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiKeyRequest {
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestPayload {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePRDRequest {
    pub idea: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PRDResponse {
    pub title: String,
    pub overview: String,
    pub target_users: Vec<String>,
    pub core_features: Vec<String>,
    pub tech_stack: Vec<String>,
    pub estimated_effort: String,
    pub business_model: Option<String>,
    pub pricing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPersonaResponse {
    pub id: String,
    pub name: String,
    pub age: String,
    pub occupation: String,
    pub background: String,
    pub goals: Vec<String>,
    pub pain_points: Vec<String>,
    pub behaviors: Vec<String>,
    pub quote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorResponse {
    pub name: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub market_share: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorAnalysisResponse {
    pub competitors: Vec<CompetitorResponse>,
    pub differentiation: String,
    pub opportunities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingChannelResponse {
    pub name: String,
    pub platform: String,
    pub priority: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingTimelineItem {
    pub phase: String,
    pub duration: String,
    pub activities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingStrategyResponse {
    pub channels: Vec<MarketingChannelResponse>,
    pub timeline: Vec<MarketingTimelineItem>,
    pub key_messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingCopyResponse {
    pub platform: String,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
}

#[tauri::command]
pub async fn validate_ai_key(request: ValidateKeyRequest) -> Result<bool, String> {
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key);
    provider.validate_key().await.map_err(|e| e.to_string())
}

/// Save API key securely to OS keychain
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
#[tauri::command]
pub fn get_api_key_from_keychain(request: GetApiKeyRequest) -> Result<String, String> {
    keychain::get_api_key(&request.provider)
        .map_err(|e| format!("Failed to retrieve API key: {}", e))
}

/// Check if API key exists in OS keychain
#[tauri::command]
pub fn has_api_key_in_keychain(provider: String) -> Result<bool, String> {
    Ok(keychain::has_api_key(&provider))
}

/// Delete API key from OS keychain
#[tauri::command]
pub fn delete_api_key_from_keychain(request: DeleteApiKeyRequest) -> Result<(), String> {
    keychain::delete_api_key(&request.provider)
        .map_err(|e| format!("Failed to delete API key: {}", e))
}

#[tauri::command]
pub async fn chat(request: ChatRequestPayload) -> Result<String, String> {
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key);

    let messages: Vec<crate::ai::Message> = request
        .messages
        .into_iter()
        .map(|m| crate::ai::Message {
            role: m.role,
            content: m.content,
        })
        .collect();

    let chat_request = ChatRequest {
        model: request.model,
        messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };

    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.content)
}

#[tauri::command]
pub async fn stream_chat(
    request: ChatRequestPayload,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        _ => return Err("Unsupported provider".to_string()),
    };

    let provider = AIProvider::new(provider_type, request.api_key.clone());

    let messages: Vec<crate::ai::Message> = request
        .messages
        .into_iter()
        .map(|m| crate::ai::Message {
            role: m.role,
            content: m.content,
        })
        .collect();

    let chat_request = ChatRequest {
        model: request.model,
        messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };

    // 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk,
            is_complete: false,
        };

        app_clone
            .emit("ai-stream-chunk", stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;

        Ok(())
    };

    // 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);

            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("ai-stream-error", error_data);

            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn generate_prd(_request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    // TODO: Implement actual PRD generation
    // For now, return mock data
    Ok(PRDResponse {
        title: "Generated Product".to_string(),
        overview: "This is an AI-generated product overview based on your idea.".to_string(),
        target_users: vec![
            "Independent developers".to_string(),
            "Freelancers".to_string(),
        ],
        core_features: vec!["Feature 1".to_string(), "Feature 2".to_string()],
        tech_stack: vec!["React".to_string(), "Node.js".to_string()],
        estimated_effort: "2-4 weeks".to_string(),
        business_model: Some("Freemium".to_string()),
        pricing: Some("Free tier + Pro $9/month".to_string()),
    })
}

#[tauri::command]
pub async fn generate_user_personas(
    _request: GeneratePRDRequest,
) -> Result<Vec<UserPersonaResponse>, String> {
    // TODO: Implement actual persona generation
    Ok(vec![UserPersonaResponse {
        id: "1".to_string(),
        name: "Alex".to_string(),
        age: "28".to_string(),
        occupation: "Full-stack Developer".to_string(),
        background: "Experienced developer working on side projects".to_string(),
        goals: vec!["Build passive income".to_string()],
        pain_points: vec!["Limited time".to_string()],
        behaviors: vec!["Active on Twitter".to_string()],
        quote: Some("I want to focus on creative work.".to_string()),
    }])
}

#[tauri::command]
pub async fn generate_competitor_analysis(
    _request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    // TODO: Implement actual competitor analysis
    Ok(CompetitorAnalysisResponse {
        competitors: vec![CompetitorResponse {
            name: "Competitor A".to_string(),
            strengths: vec!["Brand recognition".to_string()],
            weaknesses: vec!["High price".to_string()],
            market_share: Some("35%".to_string()),
        }],
        differentiation: "Our unique value proposition.".to_string(),
        opportunities: vec!["Growing market".to_string()],
    })
}

#[tauri::command]
pub async fn generate_marketing_strategy(
    _request: GeneratePRDRequest,
) -> Result<MarketingStrategyResponse, String> {
    // TODO: Implement actual marketing strategy generation
    Ok(MarketingStrategyResponse {
        channels: vec![MarketingChannelResponse {
            name: "Product Hunt".to_string(),
            platform: "producthunt".to_string(),
            priority: "high".to_string(),
            description: "Great for tech product launches".to_string(),
        }],
        timeline: vec![MarketingTimelineItem {
            phase: "Pre-launch".to_string(),
            duration: "1 week".to_string(),
            activities: vec!["Create landing page".to_string()],
        }],
        key_messages: vec!["Value proposition 1".to_string()],
    })
}

#[tauri::command]
pub async fn generate_marketing_copy(
    _request: GeneratePRDRequest,
) -> Result<Vec<MarketingCopyResponse>, String> {
    // TODO: Implement actual marketing copy generation
    Ok(vec![MarketingCopyResponse {
        platform: "twitter".to_string(),
        content: "🚀 New product launch!".to_string(),
        hashtags: Some(vec!["BuildInPublic".to_string()]),
    }])
}

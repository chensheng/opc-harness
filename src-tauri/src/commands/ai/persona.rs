use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage};
use crate::prompts::user_persona;
use crate::commands::ai::types::{GeneratePRDRequest, UserPersonaResponse};
use crate::commands::ai::parser::parse_user_personas_from_markdown;

/// 生成用户画像
#[tauri::command]
pub async fn generate_user_personas(
    request: GeneratePRDRequest,
) -> Result<Vec<UserPersonaResponse>, String> {
    log::info!("Generating user personas for idea: {}", request.idea);
    
    // 1. 构建产品信息
    let product_info = format!("基于以下产品想法生成用户画像：{}", request.idea);
    
    // 2. 根据 AI Provider 选择优化的提示词
    let prompt = match request.provider.as_str() {
        "minimax" => user_persona::generate_user_persona_prompt_minimax(&product_info),
        "glm" => user_persona::generate_user_persona_prompt_glm(&product_info),
        _ => user_persona::generate_user_persona_prompt(&product_info),
    };
    
    // 3. 创建 AI Provider
    let provider = match request.provider.as_str() {
        "openai" => AIProvider::new(AIProviderType::OpenAI, request.api_key),
        "anthropic" => AIProvider::new(AIProviderType::Anthropic, request.api_key),
        "kimi" => AIProvider::new(AIProviderType::Kimi, request.api_key),
        "glm" => AIProvider::new(AIProviderType::GLM, request.api_key),
        "minimax" => AIProvider::new(AIProviderType::MiniMax, request.api_key),
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", request.provider));
        }
    };
    
    // 4. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.8), // 稍微提高温度增加创造性
        max_tokens: Some(4096), // 用户画像需要较长文本
        stream: false,
    };
    
    // 5. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 6. 解析 AI 生成的用户画像
    let personas = parse_user_personas_from_markdown(&response.content)
        .map_err(|e| format!("用户画像解析失败：{}", e))?;
    
    log::info!("User personas generated successfully: {} personas", personas.len());
    
    Ok(personas)
}

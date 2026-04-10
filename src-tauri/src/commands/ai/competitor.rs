use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage};
use crate::prompts::competitor_analysis;
use crate::commands::ai::types::{GeneratePRDRequest, CompetitorAnalysisResponse};
use crate::commands::ai::parser::parse_competitor_analysis_from_markdown;

/// 生成竞品分析
#[tauri::command]
pub async fn generate_competitor_analysis(
    request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    log::info!("Generating competitor analysis for idea: {}", request.idea);
    
    // 1. 构建产品信息
    let product_info = format!("基于以下产品想法进行竞品分析：{}", request.idea);
    
    // 2. 根据 AI Provider 选择优化的提示词
    let prompt = match request.provider.as_str() {
        "minimax" => competitor_analysis::generate_competitor_analysis_prompt_minimax(&product_info),
        "glm" => competitor_analysis::generate_competitor_analysis_prompt_glm(&product_info),
        _ => competitor_analysis::generate_competitor_analysis_prompt(&product_info),
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
        temperature: Some(0.7),
        max_tokens: Some(6000),
        stream: false,
    };
    
    // 5. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 6. 使用已有的解析函数（在下方定义）
    let analysis = parse_competitor_analysis_from_markdown(&response.content)
        .map_err(|e| format!("竞品分析解析失败：{}", e))?;
    
    log::info!("Competitor analysis generated successfully: {} competitors", analysis.competitors.len());
    
    Ok(analysis)
}

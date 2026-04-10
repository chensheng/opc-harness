use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete, StreamError};
use crate::prompts::prd_template;
use crate::commands::ai::types::{GeneratePRDRequest, PRDResponse};
use crate::commands::ai::parser::parse_prd_from_markdown;
use tauri::Emitter;
use uuid::Uuid;

/// 生成 PRD（非流式）
#[tauri::command]
pub async fn generate_prd(request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    log::info!("Generating PRD for idea: {}", request.idea);
    
    // 1. 构建 PRD 提示词
    let prompt = prd_template::generate_prd_prompt(&request.idea, None);
    
    // 2. 创建 AI Provider
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
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096), // PRD 通常较长
        stream: false,
    };
    
    // 4. 调用 AI Provider
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("AI 调用失败：{}", e))?;
    
    // 5. 解析 AI 生成的 PRD 内容
    // AI 返回的是 Markdown 格式的 PRD，需要解析为结构化数据
    let prd = parse_prd_from_markdown(&response.content)
        .map_err(|e| format!("PRD 解析失败：{}", e))?;
    
    log::info!("PRD generated successfully: {}", prd.title);
    
    Ok(prd)
}

/// 流式生成 PRD（打字机效果）
#[tauri::command]
pub async fn stream_generate_prd(
    request: GeneratePRDRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming PRD generation for idea: {}", request.idea);
    
    // 1. 构建 PRD 提示词
    let prompt = prd_template::generate_prd_prompt(&request.idea, None);
    
    // 2. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    let provider = AIProvider::new(provider_type, request.api_key.clone());
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();

    
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk.clone(),
            is_complete: false,
        };
        
        // 发送 PRD 流式 chunk 事件
        app_clone
            .emit("prd-stream-chunk", stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("prd-stream-complete", complete_data);
            
            log::info!("Streaming PRD generation completed");
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("prd-stream-error", error_data);
            
            log::error!("Streaming PRD generation failed: {}", e);
            Err(e.to_string())
        }
    }
}

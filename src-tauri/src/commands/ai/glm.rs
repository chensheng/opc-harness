use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete, StreamError};
use crate::commands::ai::types::ChatRequestPayload;
use tauri::Emitter;
use uuid::Uuid;

/// GLM 聊天命令（非流式）
#[tauri::command]
pub async fn chat_glm(request: ChatRequestPayload) -> Result<crate::ai::ChatResponse, String> {
    log::info!("Sending chat request to GLM: {:?}", request);
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::GLM, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
    };
    
    // 调用 AI Provider (GLM uses OpenAI-compatible API)
    let response = provider.chat(chat_request)
        .await
        .map_err(|e| format!("GLM 调用失败：{}", e))?;
    
    log::info!("GLM chat response received: {} chars", response.content.len());
    
    Ok(response)
}

/// GLM 聊天命令（流式）
#[tauri::command]
pub async fn stream_chat_glm(
    app: tauri::AppHandle,
    request: ChatRequestPayload,
) -> Result<String, String> {
    log::info!("Sending streaming chat request to GLM: {:?}", request);
    
    // 生成会话 ID
    let session_id = Uuid::new_v4().to_string();
    
    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::GLM, request.api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request.messages.into_iter().map(|msg| AIMessage {
            role: msg.role,
            content: msg.content,
        }).collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
    };
    
    // 克隆 session_id 和 app handle 用于闭包
    let session_id_clone = session_id.clone();
    let app_handle_clone = app.clone();
    
    // 定义 chunk 处理回调
    let on_chunk = move |content: String| -> Result<(), crate::ai::AIError> {
        let chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content,
            is_complete: false,
        };
        
        // 发送事件到前端
        app_handle_clone.emit("ai-stream-chunk", chunk)
            .map_err(|e| crate::ai::AIError { 
                message: format!("Failed to emit chunk: {}", e) 
            })?;
        
        Ok(())
    };
    
    // 调用流式聊天
    match provider.stream_chat(chat_request, on_chunk).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("ai-stream-complete", complete_data);
            
            log::info!("GLM streaming chat completed: {} chars", final_content.len());
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

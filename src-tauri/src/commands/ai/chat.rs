use crate::ai::{AIProvider, AIProviderType, ChatRequest, StreamChunk, StreamComplete};
use crate::commands::ai::error_handler::emit_stream_error_detailed;
use crate::commands::ai::types::ChatRequestPayload;
use tauri::Emitter;
use uuid::Uuid;

/// 通用聊天命令（非流式）
#[tauri::command]
pub async fn chat(request: ChatRequestPayload) -> Result<String, String> {
    log::info!("[stream_chat] Messages count: {}", request.messages.len());

    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        "codefree" => AIProviderType::CodeFree,
        _ => return Err("Unsupported provider".to_string()),
    };

    log::info!("[stream_chat] Provider type resolved: {:?}", provider_type);

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
        project_id: None,
    };

    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response.content)
}

/// 通用流式聊天命令
#[tauri::command]
pub async fn stream_chat(
    request: ChatRequestPayload,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();

    log::info!("[stream_chat] ====== 开始流式调用 ======");
    log::info!(
        "[stream_chat] Provider: {}, Model: {}",
        request.provider,
        request.model
    );
    log::info!(
        "[stream_chat] API Key prefix: {}",
        &request.api_key[..8.min(request.api_key.len())]
    );
    log::info!("[stream_chat] Messages count: {}", request.messages.len());

    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        "codefree" => AIProviderType::CodeFree,
        _ => return Err("Unsupported provider".to_string()),
    };

    log::info!("[stream_chat] Provider type resolved: {:?}", provider_type);

    let provider = AIProvider::new(provider_type, request.api_key.clone());

    let messages: Vec<crate::ai::Message> = request
        .messages
        .into_iter()
        .map(|m| crate::ai::Message {
            role: m.role,
            content: m.content,
        })
        .collect();

    log::info!("[stream_chat] Messages converted successfully");

    let chat_request = ChatRequest {
        model: request.model.clone(),
        messages,
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
        project_id: request.project_id,
    };

    log::info!("[stream_chat] Calling provider.stream_chat...");

    // 4. 创建会话感知的 chunk 处理器
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
            // 使用统一的错误处理函数
            emit_stream_error_detailed(&app, "ai-stream-error", &session_id, &e);

            Err(e.to_string())
        }
    }
}

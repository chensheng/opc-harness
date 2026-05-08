use crate::ai::{
    AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete,
    StreamError,
};
use crate::commands::ai::parser::{
    parse_competitor_analysis_from_markdown, parse_user_persona_from_markdown,
};
use crate::commands::ai::types::{
    ChatRequestPayload, CompetitorAnalysisResponse, GeneratePRDRequest, UserPersonaResponse,
};
use crate::prompts::user_persona;
use tauri::Emitter;
use uuid::Uuid;

/// Claude 聊天命令（非流式）
#[tauri::command]
pub async fn chat_claude(request: ChatRequestPayload) -> Result<crate::ai::ChatResponse, String> {
    log::info!("Sending chat request to Claude: {:?}", request);

    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);

    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request
            .messages
            .into_iter()
            .map(|msg| AIMessage {
                role: msg.role,
                content: msg.content,
            })
            .collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: false,
        project_id: None,
    };

    // 调用 AI Provider
    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;

    log::info!(
        "Claude chat response received: {} chars",
        response.content.len()
    );

    Ok(response)
}

/// Claude 聊天命令（流式）
#[tauri::command]
pub async fn stream_chat_claude(
    app: tauri::AppHandle,
    request: ChatRequestPayload,
) -> Result<String, String> {
    log::info!("Sending streaming chat request to Claude: {:?}", request);

    // 生成会话 ID
    let session_id = Uuid::new_v4().to_string();

    // 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);

    // 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: request
            .messages
            .into_iter()
            .map(|msg| AIMessage {
                role: msg.role,
                content: msg.content,
            })
            .collect(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        stream: true,
        project_id: None,
    };

    // 克隆 session_id 用于闭包
    let session_id_clone = session_id.clone();
    // 克隆 app handle 用于闭包
    let app_handle_clone = app.clone();

    // 定义 chunk 处理回调
    let on_chunk = move |content: String| -> Result<(), crate::ai::AIError> {
        let chunk = StreamChunk {
            session_id: session_id_clone.clone(),
            content,
            is_complete: false,
        };

        // 发送事件到前端
        app_handle_clone
            .emit("ai-stream-chunk", chunk)
            .map_err(|e| crate::ai::AIError {
                message: format!("Failed to emit chunk: {}", e),
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

            log::info!(
                "Claude streaming chat completed: {} chars",
                final_content.len()
            );
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

/// 使用 Claude 生成用户画像
#[tauri::command]
pub async fn generate_personas_claude(
    request: GeneratePRDRequest,
) -> Result<UserPersonaResponse, String> {
    log::info!(
        "Generating user personas with Claude for idea: {}",
        request.idea
    );

    // 1. 构建用户画像提示词
    let prompt = user_persona::generate_user_persona_prompt(&request.idea);

    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);

    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(2048),
        stream: false,
        project_id: None,
    };

    // 4. 调用 AI Provider
    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;

    // 5. 解析 AI 生成的用户画像
    let persona = parse_user_persona_from_markdown(&response.content)
        .map_err(|e| format!("用户画像解析失败：{}", e))?;

    log::info!("User persona generated successfully: {}", persona.name);

    Ok(persona)
}

/// 使用 Claude 生成竞品分析
#[tauri::command]
pub async fn generate_competitor_analysis_claude(
    request: GeneratePRDRequest,
) -> Result<CompetitorAnalysisResponse, String> {
    log::info!(
        "Generating competitor analysis with Claude for idea: {}",
        request.idea
    );

    // 1. 构建竞品分析提示词（这里简化，实际应该有专门的模板）
    let prompt = format!(
        r#"请为以下产品想法进行详细的竞品分析：

{}

请分析：
1. 主要竞争对手（至少 3 个）
2. 每个竞争对手的优势和劣势
3. 市场份额或用户规模
4. 差异化机会
5. 市场空白点

请以结构化的方式呈现分析结果。"#,
        request.idea
    );

    // 2. 创建 AI Provider
    let provider = AIProvider::new(AIProviderType::Anthropic, request.api_key);

    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: false,
        project_id: None,
    };

    // 4. 调用 AI Provider
    let response = provider
        .chat(chat_request)
        .await
        .map_err(|e| format!("Claude 调用失败：{}", e))?;

    // 5. 解析 AI 生成的竞品分析
    let analysis = parse_competitor_analysis_from_markdown(&response.content)
        .map_err(|e| format!("竞品分析解析失败：{}", e))?;

    log::info!("Competitor analysis generated successfully");

    Ok(analysis)
}

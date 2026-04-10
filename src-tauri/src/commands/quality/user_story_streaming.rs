// 用户故事流式处理模块
// 负责流式分解用户故事并实时推送结果

use crate::commands::quality::types::DecomposeUserStoriesRequest;
use tauri::Emitter;

/// 分解用户故事(流式版本)
pub async fn decompose_user_stories_streaming(
    request: DecomposeUserStoriesRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use uuid::Uuid;
    
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming user story decomposition (provider: {}, model: {})", 
               request.provider, request.model);
    
    // 1. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => crate::ai::AIProviderType::OpenAI,
        "anthropic" => crate::ai::AIProviderType::Anthropic,
        "kimi" => crate::ai::AIProviderType::Kimi,
        "glm" => crate::ai::AIProviderType::GLM,
        "minimax" => crate::ai::AIProviderType::MiniMax,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    let api_key = request.api_key.ok_or_else(|| "未提供 API Key".to_string())?;
    let provider = crate::ai::AIProvider::new(provider_type, api_key);
    
    // 2. 生成提示词
    let prompt = crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt(&request.prd_content);
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = crate::ai::ChatRequest {
        model: request.model,
        messages: vec![
            crate::ai::Message {
                role: "system".to_string(),
                content: "你是一位经验丰富的敏捷开发专家和产品经理。请严格按照要求的 Markdown 表格格式输出用户故事列表。".to_string(),
            },
            crate::ai::Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = crate::ai::StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk.clone(),
            is_complete: false,
        };
        
        // 发送用户故事流式 chunk 事件
        app_clone
            .emit("user-story-stream-chunk", &stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = crate::ai::StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("user-story-stream-complete", &complete_data);
            
            log::info!("Streaming user story decomposition completed");
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = crate::ai::StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("user-story-stream-error", &error_data);
            
            log::error!("Streaming user story decomposition failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_module_structure() {
        // 简单的结构测试，确保模块可以正常导入
        assert!(true);
    }
}

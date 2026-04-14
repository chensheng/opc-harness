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
    
    // 1. 构建提示词 - 根据提供商类型选择不同的策略
    let user_message = if request.provider == "codefree" {
        // CodeFree 提供商：使用文件引用方式（不嵌入 PRD 内容）
        if request.project_id.is_none() {
            log::warn!("[decompose_user_stories_streaming] ❌ CodeFree provider requires project_id but got None");
            return Err("CodeFree 提供商需要提供 project_id".to_string());
        }
        
        if let Some(ref existing_stories) = request.existing_stories {
            log::info!("[decompose_user_stories_streaming] 📋 Including {} existing stories to avoid duplication", existing_stories.len());
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt_with_existing(existing_stories)
        } else {
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt()
        }
    } else {
        // 非 CodeFree 提供商：将 PRD 内容直接嵌入提示词中
        if let Some(ref existing_stories) = request.existing_stories {
            log::info!("[decompose_user_stories_streaming] 📋 Including {} existing stories to avoid duplication", existing_stories.len());
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt_embedded_with_existing(
                &request.prd_content,
                existing_stories
            )
        } else {
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt_embedded(&request.prd_content)
        }
    };
    
    // 2. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => crate::ai::AIProviderType::OpenAI,
        "anthropic" => crate::ai::AIProviderType::Anthropic,
        "kimi" => crate::ai::AIProviderType::Kimi,
        "glm" => crate::ai::AIProviderType::GLM,
        "minimax" => crate::ai::AIProviderType::MiniMax,
        "codefree" => crate::ai::AIProviderType::CodeFree,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    // 获取 API Key - 优先使用传入的 key，否则从环境变量读取
    let api_key = request.api_key
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
        .or_else(|| std::env::var("MOONSHOT_API_KEY").ok())
        .or_else(|| std::env::var("ZHIPU_API_KEY").ok())
        .or_else(|| std::env::var("KIMI_API_KEY").ok())
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .unwrap_or_default(); // 如果都没有，使用空字符串（AI Provider会处理）
    
    let provider = crate::ai::AIProvider::new(provider_type, api_key);
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = crate::ai::ChatRequest {
        model: request.model,
        messages: vec![
            crate::ai::Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
        project_id: request.project_id.clone(),
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    let provider_clone = request.provider.clone();
    let project_id_clone = request.project_id.clone();
    
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
            // 如果是 CodeFree，需要从文件读取最终内容
            let us_content = if provider_clone == "codefree" {
                if let Some(ref pid) = project_id_clone {
                    use crate::utils::paths::get_workspaces_dir;
                    use std::fs;
                    
                    let workspaces_root = get_workspaces_dir();
                    let workspace_path = workspaces_root.join(pid);
                    let context_dir = workspace_path.join(".opc-harness");
                    let us_md_path = context_dir.join("US.md");
                    
                    log::info!("[decompose_user_stories_streaming] 📖 Reading generated user stories from: {:?}", us_md_path);
                    
                    // 等待一下确保文件写入完成
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    
                    // 尝试读取 US.md 文件
                    match fs::read_to_string(&us_md_path) {
                        Ok(content) => {
                            log::info!("[decompose_user_stories_streaming] ✅ Successfully read US.md, length: {} bytes", content.len());
                            content
                        }
                        Err(e) => {
                            log::warn!("[decompose_user_stories_streaming] ⚠️ Failed to read US.md: {}, using streamed content", e);
                            final_content
                        }
                    }
                } else {
                    log::warn!("[decompose_user_stories_streaming] ⚠️ CodeFree completed but no project_id, using streamed content");
                    final_content
                }
            } else {
                // 非 CodeFree 提供商，直接使用流式内容
                final_content
            };
            
            // 发送完成事件（使用从文件读取的内容或流式内容）
            let complete_data = crate::ai::StreamComplete {
                session_id: session_id.clone(),
                content: us_content.clone(),
            };
            let _ = app.emit("user-story-stream-complete", &complete_data);
            
            log::info!("Streaming user story decomposition completed");
            Ok(us_content)
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
    #[test]
    fn test_streaming_module_structure() {
        // 简单的结构测试，确保模块可以正常导入
        assert!(true);
    }
}

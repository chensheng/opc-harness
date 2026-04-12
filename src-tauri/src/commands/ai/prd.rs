use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete, StreamError};
use crate::prompts::prd_template;
use crate::commands::ai::types::{GeneratePRDRequest, PRDResponse, PRDStreamRequest};
use crate::commands::ai::parser::parse_prd_from_markdown;
use serde::Serialize;
use tauri::Emitter;
use uuid::Uuid;

/// 生成 PRD（非流式）
#[tauri::command]
pub async fn generate_prd(request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    log::info!("Generating PRD for idea: {}", request.idea);
    
    // 1. 如果是 CodeFree，需要写入 AGENTS.md 文件并准备简短的用户消息
    let user_message = if request.provider == "codefree" {
        log::info!("[generate_prd] 🎯 CodeFree provider detected!");
        
        if let Some(ref pid) = request.project_id {
            use crate::utils::paths::get_workspaces_dir;
            use std::fs;
            
            let workspaces_root = get_workspaces_dir();
            let workspace_path = workspaces_root.join(pid);
            let context_dir = workspace_path.join(".opc-harness");
            
            log::info!("[generate_prd] 📁 Workspace path: {:?}", workspace_path);
            log::info!("[generate_prd] 📁 Context directory: {:?}", context_dir);
            
            // 确保 .opc-harness 目录存在
            fs::create_dir_all(&context_dir).map_err(|e| {
                log::error!("[generate_prd] Failed to create context directory: {}", e);
                format!("Failed to create context directory: {}", e)
            })?;
            
            // 写入 AGENTS.md 作为系统提示词（使用统一的 PRD 生成提示词）
            let agents_md_path = context_dir.join("AGENTS.md");
            let agents_content = prd_template::generate_prd_prompt(&request.idea, None);
            
            fs::write(&agents_md_path, &agents_content).map_err(|e| {
                log::error!("[generate_prd] Failed to write AGENTS.md: {}", e);
                format!("Failed to write AGENTS.md: {}", e)
            })?;
            
            log::info!("[generate_prd] ✅ AGENTS.md successfully written to: {:?}", agents_md_path);
            
            // 构建简短的用户消息，通过 @ 引用 AGENTS.md 文件
            format!(
                "请根据 @.opc-harness/AGENTS.md 中的要求，为以下产品想法生成完整的 PRD 文档，并将结果保存到 @.opc-harness/PRD.md 文件中。\n\n产品想法：{}",
                request.idea
            )
        } else {
            log::warn!("[generate_prd] ❌ CodeFree provider requires project_id but got None");
            // 如果没有 project_id，回退到完整提示词
            prd_template::generate_prd_prompt(&request.idea, None)
        }
    } else {
        // 非 CodeFree 提供商，使用完整的提示词
        prd_template::generate_prd_prompt(&request.idea, None)
    };
    
    // 2. 创建 AI Provider
    let provider = match request.provider.as_str() {
        "openai" => AIProvider::new(AIProviderType::OpenAI, request.api_key),
        "anthropic" => AIProvider::new(AIProviderType::Anthropic, request.api_key),
        "kimi" => AIProvider::new(AIProviderType::Kimi, request.api_key),
        "glm" => AIProvider::new(AIProviderType::GLM, request.api_key),
        "minimax" => AIProvider::new(AIProviderType::MiniMax, request.api_key),
        "codefree" => AIProvider::new(AIProviderType::CodeFree, request.api_key),
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", request.provider));
        }
    };
    
    // 3. 构建聊天请求
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: user_message,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096), // PRD 通常较长
        stream: false,
        project_id: request.project_id.clone(),
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
#[derive(Serialize)]
pub struct StartPRDStreamResponse {
    pub session_id: String,
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn start_prd_stream(
    request: PRDStreamRequest,
    app: tauri::AppHandle,
) -> Result<StartPRDStreamResponse, String> {
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming PRD generation for idea: {}", request.idea);
    log::info!("[start_prd_stream] ====== 接收到的参数 ======");
    log::info!("[start_prd_stream] Provider: {}", request.provider);
    log::info!("[start_prd_stream] Model: {}", request.model);
    log::info!("[start_prd_stream] API Key length: {}", request.api_key.len());
    log::info!("[start_prd_stream] Project ID: {:?}", request.project_id);
    
    // 调试：打印原始参数的类型信息
    log::info!("[start_prd_stream] Project ID is_some: {}", request.project_id.is_some());
    if let Some(ref pid) = request.project_id {
        log::info!("[start_prd_stream] Project ID value: '{}'", pid);
        log::info!("[start_prd_stream] Project ID length: {}", pid.len());
        log::info!("[start_prd_stream] Project ID is_empty: {}", pid.is_empty());
    }
    
    log::info!("[start_prd_stream] =========================");
    
    // 1. 如果是 CodeFree，需要写入 AGENTS.md 文件并准备简短的用户消息
    let user_message = if request.provider == "codefree" {
        log::info!("[start_prd_stream] 🎯 CodeFree provider detected!");
        log::info!("[start_prd_stream] Project ID: {:?}", request.project_id);
        
        if let Some(ref pid) = request.project_id {
            use crate::utils::paths::get_workspaces_dir;
            use std::fs;
            
            let workspaces_root = get_workspaces_dir();
            let workspace_path = workspaces_root.join(pid);
            let context_dir = workspace_path.join(".opc-harness");
            
            log::info!("[start_prd_stream] 📁 Workspace path: {:?}", workspace_path);
            log::info!("[start_prd_stream] 📁 Context directory: {:?}", context_dir);
            
            // 检查工作区目录是否存在
            if !workspace_path.exists() {
                log::warn!("[start_prd_stream] ⚠️ Workspace directory does not exist, creating...");
                fs::create_dir_all(&workspace_path).map_err(|e| {
                    log::error!("[start_prd_stream] Failed to create workspace directory: {}", e);
                    format!("Failed to create workspace directory: {}", e)
                })?;
            }
            
            // 确保 .opc-harness 目录存在
            log::info!("[start_prd_stream] Creating .opc-harness directory...");
            fs::create_dir_all(&context_dir).map_err(|e| {
                log::error!("[start_prd_stream] Failed to create context directory: {}", e);
                format!("Failed to create context directory: {}", e)
            })?;
            
            log::info!("[start_prd_stream] ✅ Context directory created/verified");
            
            // 写入 AGENTS.md 作为系统提示词（使用统一的 PRD 生成提示词）
            let agents_md_path = context_dir.join("AGENTS.md");
            
            // 使用与所有 AI 厂商统一的 PRD 生成提示词
            let agents_content = prd_template::generate_prd_prompt(&request.idea, None);
            
            fs::write(&agents_md_path, &agents_content).map_err(|e| {
                log::error!("[start_prd_stream] Failed to write AGENTS.md: {}", e);
                format!("Failed to write AGENTS.md: {}", e)
            })?;
            
            log::info!("[start_prd_stream] ✅ AGENTS.md successfully written to: {:?}", agents_md_path);
            log::info!("[start_prd_stream] 📝 AGENTS.md content length: {} bytes", agents_content.len());
            
            // 构建简短的用户消息，通过 @ 引用 AGENTS.md 文件
            // CodeFree 会自动读取当前工作目录下的文件
            format!(
                "请根据 @.opc-harness/AGENTS.md 中的要求，为以下产品想法生成完整的 PRD 文档，并将结果保存到 @.opc-harness/PRD.md 文件中。\n\n产品想法：{}",
                request.idea
            )
        } else {
            log::warn!("[start_prd_stream] ❌ CodeFree provider requires project_id but got None");
            // 如果没有 project_id，回退到完整提示词
            prd_template::generate_prd_prompt(&request.idea, None)
        }
    } else {
        // 非 CodeFree 提供商，使用完整的提示词
        prd_template::generate_prd_prompt(&request.idea, None)
    };
    
    // 2. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        "codefree" => AIProviderType::CodeFree,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    let provider = AIProvider::new(provider_type, request.api_key.clone());
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = ChatRequest {
        model: request.model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: user_message,
        }],
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
    
    // 5. 在后台启动流式请求
    let session_id_for_return = session_id.clone();
    tokio::spawn(async move {
        match provider.stream_chat(chat_request, chunk_handler).await {
            Ok(final_content) => {
                // 如果是 CodeFree，需要从文件读取最终内容
                let prd_content = if provider_clone == "codefree" {
                    if let Some(ref pid) = project_id_clone {
                        use crate::utils::paths::get_workspaces_dir;
                        use std::fs;
                        
                        let workspaces_root = get_workspaces_dir();
                        let workspace_path = workspaces_root.join(pid);
                        let context_dir = workspace_path.join(".opc-harness");
                        let prd_md_path = context_dir.join("PRD.md");
                        
                        log::info!("[start_prd_stream] 📖 Reading generated PRD from: {:?}", prd_md_path);
                        
                        // 等待一下确保文件写入完成
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        
                        // 尝试读取 PRD.md 文件
                        match fs::read_to_string(&prd_md_path) {
                            Ok(content) => {
                                log::info!("[start_prd_stream] ✅ Successfully read PRD.md, length: {} bytes", content.len());
                                content
                            }
                            Err(e) => {
                                log::warn!("[start_prd_stream] ⚠️ Failed to read PRD.md: {}, using streamed content", e);
                                final_content
                            }
                        }
                    } else {
                        log::warn!("[start_prd_stream] ⚠️ CodeFree completed but no project_id, using streamed content");
                        final_content
                    }
                } else {
                    // 非 CodeFree 提供商，直接使用流式内容
                    final_content
                };
                
                // 发送完成事件（使用从文件读取的内容或流式内容）
                let complete_data = StreamComplete {
                    session_id: session_id.clone(),
                    content: prd_content.clone(),
                };
                let _ = app.emit("prd-stream-complete", complete_data);
                
                log::info!("Streaming PRD generation completed, content length: {}", prd_content.len());
            }
            Err(e) => {
                // 发送错误事件
                let error_data = StreamError {
                    session_id: session_id.clone(),
                    error: e.to_string(),
                };
                let _ = app.emit("prd-stream-error", error_data);
                
                log::error!("Streaming PRD generation failed: {}", e);
            }
        }
    });
    
    // 立即返回 session_id
    Ok(StartPRDStreamResponse { session_id: session_id_for_return })
}

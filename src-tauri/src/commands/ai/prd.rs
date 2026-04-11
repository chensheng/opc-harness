use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message as AIMessage, StreamChunk, StreamComplete, StreamError};
use crate::prompts::prd_template;
use crate::commands::ai::types::{GeneratePRDRequest, PRDResponse};
use crate::commands::ai::parser::parse_prd_from_markdown;
use serde::Serialize;
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
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096), // PRD 通常较长
        stream: false,
        project_id: None,
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
    idea: String,
    provider: String,
    model: String,
    apiKey: String,
    project_id: Option<String>,
    app: tauri::AppHandle,
) -> Result<StartPRDStreamResponse, String> {
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming PRD generation for idea: {}", idea);
    log::info!("[start_prd_stream] Provider: {}, Model: {}", provider, model);
    log::info!("[start_prd_stream] Project ID: {:?}", project_id);
    
    // 1. 构建 PRD 提示词
    let prompt = prd_template::generate_prd_prompt(&idea, None);
    
    // 2. 如果是 CodeFree，需要写入 AGENTS.md 文件
    if provider == "codefree" {
        log::info!("[start_prd_stream] CodeFree provider detected, preparing to write AGENTS.md");
        
        if let Some(ref pid) = project_id {
            use crate::utils::paths::get_workspaces_dir;
            use std::fs;
            
            let workspaces_root = get_workspaces_dir();
            let workspace_path = workspaces_root.join(pid);
            let context_dir = workspace_path.join(".opc-harness");
            
            log::info!("[start_prd_stream] Workspace path: {:?}", workspace_path);
            log::info!("[start_prd_stream] Context directory: {:?}", context_dir);
            
            // 确保 .opc-harness 目录存在
            fs::create_dir_all(&context_dir).map_err(|e| {
                log::error!("[start_prd_stream] Failed to create context directory: {}", e);
                format!("Failed to create context directory: {}", e)
            })?;
            
            log::info!("[start_prd_stream] Context directory created/verified");
            
            // 写入 AGENTS.md 作为系统提示词
            let agents_md_path = context_dir.join("AGENTS.md");
            let agents_content = format!(
r#"# AI Assistant - PRD Generation

## Role
You are a professional Product Manager and PRD (Product Requirements Document) writer.

## Task
Generate a complete, structured PRD document based on the user's product idea.

## Output Rules
1. **Output ONLY the final PRD document** - no explanations, no reasoning process, no summaries
2. **Write in Chinese** (简体中文)
3. **Use Markdown format** with proper headings and structure
4. **Be comprehensive and detailed** - include all necessary sections
5. **Ensure professionalism and readability**

## Required PRD Structure
Your output must include these sections:
- Product Overview (产品概述)
- Target Users (目标用户)
- Core Features (核心功能)
- User Stories (用户故事)
- Technical Stack (技术栈)
- Development Effort Estimate (开发工作量评估)
- Business Model (商业模式)
- Pricing Strategy (定价策略)

## Context Files
- Current working directory contains @.opc-harness/PRD.md (if exists)
- Read existing PRD content if available for reference

Now, generate the complete PRD document based on the user's idea below.
"#
            );
            
            fs::write(&agents_md_path, agents_content).map_err(|e| {
                log::error!("[start_prd_stream] Failed to write AGENTS.md: {}", e);
                format!("Failed to write AGENTS.md: {}", e)
            })?;
            
            log::info!("[start_prd_stream] ✅ AGENTS.md successfully written to: {:?}", agents_md_path);
        } else {
            log::warn!("[start_prd_stream] ❌ CodeFree provider requires project_id but got None");
        }
    }
    
    // 3. 创建 AI Provider
    let provider_type = match provider.as_str() {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        "codefree" => AIProviderType::CodeFree,
        _ => return Err(format!("不支持的 AI 提供商：{}", provider)),
    };
    
    let provider = AIProvider::new(provider_type, apiKey.clone());
    
    // 4. 构建聊天请求（流式模式）
    let chat_request = ChatRequest {
        model,
        messages: vec![AIMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
        project_id: project_id.clone(),
    };
    
    // 5. 创建会话感知的 chunk 处理器
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
    
    // 6. 在后台启动流式请求
    let session_id_for_return = session_id.clone();
    tokio::spawn(async move {
        match provider.stream_chat(chat_request, chunk_handler).await {
            Ok(final_content) => {
                // 发送完成事件
                let complete_data = StreamComplete {
                    session_id: session_id.clone(),
                    content: final_content.clone(),
                };
                let _ = app.emit("prd-stream-complete", complete_data);
                
                log::info!("Streaming PRD generation completed");
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

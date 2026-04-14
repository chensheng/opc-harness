// Sprint 用户故事分配流式处理模块
// 负责流式分析并推荐最适合分配到指定Sprint的用户故事

use crate::commands::quality::types::{AssignStoriesToSprintRequest, UserStory};
use tauri::Emitter;

/// 格式化未分配的用户故事列表为Markdown格式
fn format_unassigned_stories(stories: &[UserStory]) -> String {
    let mut output = String::from("# 未分配的用户故事列表\n\n");
    
    for story in stories {
        output.push_str(&format!(
r#"## {story_number}：{title}

- **角色**：{role}
- **功能**：{feature}
- **价值**：{benefit}
- **优先级**：{priority}
- **故事点**：{story_points}
- **标签**：{labels}
- **依赖**：{dependencies}

"#,
            story_number = story.story_number,
            title = story.title,
            role = story.role,
            feature = story.feature,
            benefit = story.benefit,
            priority = story.priority,
            story_points = story.story_points.map(|p| p.to_string()).unwrap_or_else(|| "未估算".to_string()),
            labels = if story.labels.is_empty() { "无".to_string() } else { story.labels.join(", ") },
            dependencies = story.dependencies.as_ref().map(|d| d.join(", ")).unwrap_or_else(|| "无".to_string()),
        ));
    }
    
    output
}

/// Sprint分配（流式版本）
pub async fn assign_stories_to_sprint_streaming(
    request: AssignStoriesToSprintRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use uuid::Uuid;
    
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming sprint story assignment (provider: {}, model: {})", 
               request.provider, request.model);
    
    // 1. 如果是 CodeFree，需要写入 AGENTS.md、US.md 和 SPRINT_INFO.md 文件
    let user_message = if request.provider == "codefree" {
        log::info!("[assign_stories_to_sprint_streaming] 🎯 CodeFree provider detected!");
        
        if let Some(ref pid) = request.project_id {
            use crate::utils::paths::get_workspaces_dir;
            use std::fs;
            
            let workspaces_root = get_workspaces_dir();
            let workspace_path = workspaces_root.join(pid);
            let context_dir = workspace_path.join(".opc-harness");
            
            log::info!("[assign_stories_to_sprint_streaming] 📁 Workspace path: {:?}", workspace_path);
            log::info!("[assign_stories_to_sprint_streaming] 📁 Context directory: {:?}", context_dir);
            
            // 确保 .opc-harness 目录存在
            fs::create_dir_all(&context_dir).map_err(|e| {
                log::error!("[assign_stories_to_sprint_streaming] Failed to create context directory: {}", e);
                format!("Failed to create context directory: {}", e)
            })?;
            
            // 写入 AGENTS.md 作为系统提示词
            let agents_md_path = context_dir.join("AGENTS.md");
            let agents_content = crate::prompts::sprint_assignment::generate_sprint_assignment_system_prompt();
            
            fs::write(&agents_md_path, &agents_content).map_err(|e| {
                log::error!("[assign_stories_to_sprint_streaming] Failed to write AGENTS.md: {}", e);
                format!("Failed to write AGENTS.md: {}", e)
            })?;
            log::info!("[assign_stories_to_sprint_streaming] ✅ AGENTS.md written to: {:?}", agents_md_path);
            log::info!("[assign_stories_to_sprint_streaming] 📝 AGENTS.md content length: {} bytes", agents_content.len());
            
            // 写入 US.md（未分配的用户故事）
            let us_md_path = context_dir.join("US.md");
            let us_content = format_unassigned_stories(&request.stories);
            
            fs::write(&us_md_path, &us_content).map_err(|e| {
                log::error!("[assign_stories_to_sprint_streaming] Failed to write US.md: {}", e);
                format!("Failed to write US.md: {}", e)
            })?;
            log::info!("[assign_stories_to_sprint_streaming] ✅ US.md written to: {:?}", us_md_path);
            
            // 写入 SPRINT_INFO.md（Sprint信息）
            let sprint_info_md_path = context_dir.join("SPRINT_INFO.md");
            let goal_text = request.sprint.goal.as_deref().unwrap_or("未设置");
            
            let sprint_info_content = format!(
r#"# Sprint 信息

- **名称**：{name}
- **目标**：{goal}
- **时间范围**：{start_date} 至 {end_date}
- **当前容量**：{total_points} 故事点
- **已完成**：{completed_points} 故事点
"#,
                name = request.sprint.name,
                goal = goal_text,
                start_date = request.sprint.start_date,
                end_date = request.sprint.end_date,
                total_points = request.sprint.total_story_points,
                completed_points = request.sprint.completed_story_points,
            );
            
            fs::write(&sprint_info_md_path, &sprint_info_content).map_err(|e| {
                log::error!("[assign_stories_to_sprint_streaming] Failed to write SPRINT_INFO.md: {}", e);
                format!("Failed to write SPRINT_INFO.md: {}", e)
            })?;
            log::info!("[assign_stories_to_sprint_streaming] ✅ SPRINT_INFO.md written to: {:?}", sprint_info_md_path);
            
            // 构建简短的用户消息，通过 @ 引用文件，并将用户建议直接放入query中
            let mut message = String::from(
                "请读取 @.opc-harness/AGENTS.md 了解任务规则，"
            );
            message.push_str("读取 @.opc-harness/US.md 获取未分配的用户故事，");
            message.push_str("读取 @.opc-harness/SPRINT_INFO.md 获取Sprint信息，");
            message.push_str("然后将分配结果保存到 @.opc-harness/SPRINT.md 文件中。");
            
            // 如果有用户建议，直接追加到消息中
            if let Some(ref suggestions) = request.user_suggestions {
                if !suggestions.trim().is_empty() {
                    message.push_str(" 用户的特殊要求：");
                    message.push_str(suggestions);
                }
            }
            
            // ⚠️ 注意：移除换行符，避免 cmd.exe /c 解析错误
            message.replace('\n', " ").replace('\r', "")
        } else {
            log::warn!("[assign_stories_to_sprint_streaming] ❌ CodeFree provider requires project_id but got None");
            // 如果没有 project_id，回退到完整提示词
            generate_full_prompt(&request)
        }
    } else {
        // 非 CodeFree 提供商，使用完整的提示词
        generate_full_prompt(&request)
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
    
    // 获取 API Key
    let api_key = request.api_key
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
        .or_else(|| std::env::var("MOONSHOT_API_KEY").ok())
        .or_else(|| std::env::var("ZHIPU_API_KEY").ok())
        .or_else(|| std::env::var("KIMI_API_KEY").ok())
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .unwrap_or_default();
    
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
        
        // 发送 Sprint 分配流式 chunk 事件
        app_clone
            .emit("sprint-assignment-stream-chunk", &stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 如果是 CodeFree，需要从文件读取最终内容
            let sprint_content = if provider_clone == "codefree" {
                if let Some(ref pid) = project_id_clone {
                    use crate::utils::paths::get_workspaces_dir;
                    use std::fs;
                    
                    let workspaces_root = get_workspaces_dir();
                    let workspace_path = workspaces_root.join(pid);
                    let context_dir = workspace_path.join(".opc-harness");
                    let sprint_md_path = context_dir.join("SPRINT.md");
                    
                    log::info!("[assign_stories_to_sprint_streaming] 📖 Reading assignment results from: {:?}", sprint_md_path);
                    
                    // 等待一下确保文件写入完成
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    
                    // 尝试读取 SPRINT.md 文件
                    match fs::read_to_string(&sprint_md_path) {
                        Ok(content) => {
                            log::info!("[assign_stories_to_sprint_streaming] ✅ Successfully read SPRINT.md, length: {} bytes", content.len());
                            content
                        }
                        Err(e) => {
                            log::warn!("[assign_stories_to_sprint_streaming] ⚠️ Failed to read SPRINT.md: {}, using streamed content", e);
                            final_content
                        }
                    }
                } else {
                    log::warn!("[assign_stories_to_sprint_streaming] ⚠️ CodeFree completed but no project_id, using streamed content");
                    final_content
                }
            } else {
                // 非 CodeFree 提供商，直接使用流式内容
                final_content
            };
            
            // 发送完成事件
            let complete_data = crate::ai::StreamComplete {
                session_id: session_id.clone(),
                content: sprint_content.clone(),
            };
            let _ = app.emit("sprint-assignment-stream-complete", &complete_data);
            
            log::info!("Streaming sprint story assignment completed");
            Ok(sprint_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = crate::ai::StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("sprint-assignment-stream-error", &error_data);
            
            log::error!("Streaming sprint story assignment failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// 生成完整的提示词（用于非CodeFree模式或CodeFree无project_id时的fallback）
fn generate_full_prompt(request: &AssignStoriesToSprintRequest) -> String {
    let goal_text = request.sprint.goal.as_deref().unwrap_or("未设置");
    
    let sprint_info = format!(
r#"Sprint信息：
- 名称：{name}
- 目标：{goal}
- 时间范围：{start_date} 至 {end_date}
- 当前容量：{total_points} 故事点
- 已完成：{completed_points} 故事点
"#,
        name = request.sprint.name,
        goal = goal_text,
        start_date = request.sprint.start_date,
        end_date = request.sprint.end_date,
        total_points = request.sprint.total_story_points,
        completed_points = request.sprint.completed_story_points,
    );
    
    let stories_info = format_unassigned_stories(&request.stories);
    
    let user_suggestions_section = if let Some(ref suggestions) = request.user_suggestions {
        if !suggestions.trim().is_empty() {
            format!(
r#"

用户的分配建议和特殊要求：
{suggestions}

请特别注意并优先考虑上述用户建议，在推荐时充分考虑这些约束条件和要求。
"#
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    

    let system_prompt = crate::prompts::sprint_assignment::generate_sprint_assignment_system_prompt();
    
    format!(
r#"{system_prompt}

{sprint_info}

{stories_info}{user_suggestions_section}

请分析以上Sprint信息和未分配的用户故事，推荐最适合分配到该Sprint的故事。

请将推荐结果以Markdown表格格式输出。
"#
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_streaming_module_structure() {
        // 简单的结构测试，确保模块可以正常导入
        assert!(true);
    }
}

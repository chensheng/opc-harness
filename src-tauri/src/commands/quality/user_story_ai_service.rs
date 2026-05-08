// 用户故事AI服务模块
// 负责调用AI API进行用户故事分解

use crate::commands::quality::types::{ExistingStoryInfo, UserStory};
use crate::commands::quality::user_story_parser;
use chrono::Utc;

/// 使用 AI 进行用户故事拆分
pub async fn decompose_with_ai(
    prd_content: &str,
    provider: &str,
    model: &str,
    api_key: Option<&str>,
    existing_stories: Option<&[ExistingStoryInfo]>,
) -> Result<Vec<UserStory>, String> {
    use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message};

    // 获取 API Key - 优先使用传入的 key，否则从环境变量读取
    let api_key = api_key
        .map(|k| k.to_string())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
        .or_else(|| std::env::var("MOONSHOT_API_KEY").ok())
        .or_else(|| std::env::var("ZHIPU_API_KEY").ok())
        .or_else(|| std::env::var("KIMI_API_KEY").ok())
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .unwrap_or_default(); // 如果都没有，使用空字符串（AI Provider会处理）

    // 根据提供商类型和是否有已有用户故事，选择合适的提示词生成函数
    let prompt = if provider == "codefree" {
        // CodeFree 提供商：非流式版本不支持，因为需要 project_id 来写入文件
        log::warn!("CodeFree provider in non-streaming mode is not supported");
        return Err(
            "CodeFree 提供商需要使用流式接口（decompose_user_stories_streaming），请更新前端调用"
                .to_string(),
        );
    } else {
        // 非 CodeFree 提供商：将 PRD 内容直接嵌入提示词中
        if let Some(stories) = existing_stories {
            log::info!(
                "Including {} existing stories to avoid duplication",
                stories.len()
            );
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt_embedded_with_existing(
                prd_content,
                stories
            )
        } else {
            crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt_embedded(prd_content)
        }
    };

    log::info!("Calling AI service for user story decomposition...");
    log::debug!("Prompt length: {} characters", prompt.len());

    // 根据 provider 字符串创建对应的 AI Provider
    let provider_type = match provider {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", provider));
        }
    };

    // 创建 AI Provider
    let ai_provider = AIProvider::new(provider_type, api_key);

    // 构建聊天请求
    let chat_request = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一位经验丰富的敏捷开发专家和产品经理。请严格按照要求的 Markdown 表格格式输出用户故事列表，不要添加任何额外的解释或说明。".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: false,
            project_id: None,
        };

    // 调用 AI 服务
    let response = ai_provider
        .chat(chat_request)
        .await
        .map_err(|e| format!("AI 服务调用失败：{}", e.message))?;

    log::info!(
        "AI response received, length: {} characters",
        response.content.len()
    );

    // 检查响应是否有效
    let trimmed_content = response.content.trim();
    if trimmed_content.is_empty() {
        return Err("AI 返回了空响应".to_string());
    }

    // 检测异常响应模式
    if let Some(error_msg) = user_story_parser::detect_abnormal_response(trimmed_content) {
        log::error!("检测到异常AI响应: {}", error_msg);
        log::error!(
            "响应预览(前500字符): {}",
            if trimmed_content.len() > 500 {
                &trimmed_content[..500]
            } else {
                trimmed_content
            }
        );
        return Err(format!(
            "AI 返回了无效响应：{}\n\n\
             响应预览：{}\n\n\
             建议解决方案：\n\
             1. 检查 PRD 内容是否过长（建议精简到3000字以内）\n\
             2. 尝试切换到其他 AI 提供商（如 OpenAI GPT-4）\n\
             3. 检查 API Key 是否有足够配额且未过期\n\
             4. 确认使用的模型支持此任务（Kimi for Coding 可能不适合）\n\
             5. 简化 PRD 内容，只保留核心功能需求",
            error_msg,
            if trimmed_content.len() > 200 {
                &trimmed_content[..200]
            } else {
                trimmed_content
            }
        ));
    }

    log::debug!(
        "AI response preview (first 300 chars): {}",
        if response.content.len() > 300 {
            &response.content[..300]
        } else {
            &response.content
        }
    );

    // 解析 AI 响应中的 JSON
    let user_stories = user_story_parser::parse_ai_response_to_user_stories(&response.content)?;

    // 补充时间戳和状态
    let now = Utc::now().to_rfc3339();
    let mut stories_with_metadata: Vec<UserStory> = user_stories
        .into_iter()
        .map(|mut story| {
            story.created_at = now.clone();
            story.updated_at = now.clone();
            if story.status.is_empty() {
                story.status = "draft".to_string();
            }
            story
        })
        .collect();

    // 确保故事编号连续
    for (index, story) in stories_with_metadata.iter_mut().enumerate() {
        story.story_number = format!("US-{:03}", index + 1);
        story.id = format!("us-{:03}", index + 1);
    }

    Ok(stories_with_metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decompose_without_api_key() {
        // 测试没有 API Key 时的行为 - 现在不会报错，而是使用空字符串
        let result = decompose_with_ai(
            "我们需要一个任务管理系统",
            "openai",
            "gpt-4-turbo-preview",
            None,
            None, // existing_stories
        )
        .await;

        // 由于没有有效的 API Key，AI 调用会失败，但不会在参数检查阶段失败
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        // 错误应该是 AI 服务调用失败，而不是"未提供 API Key"
        assert!(error_msg.contains("AI 服务调用失败") || error_msg.contains("Invalid API key"));
    }

    #[tokio::test]
    async fn test_decompose_with_invalid_provider() {
        let result = decompose_with_ai(
            "测试内容",
            "invalid_provider",
            "test-model",
            Some("test-key"),
            None, // existing_stories
        )
        .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不支持的 AI 提供商"));
    }
}

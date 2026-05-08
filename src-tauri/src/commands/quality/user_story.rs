// 用户故事分解模块 - 主协调器
// 提供用户故事分解的公共API入口

use crate::commands::quality::types::{DecomposeUserStoriesRequest, DecomposeUserStoriesResponse};

// 导入子模块
use super::user_story_ai_service;
use super::user_story_streaming;

/// 分解用户故事
#[tauri::command]
pub async fn decompose_user_stories(
    request: DecomposeUserStoriesRequest,
) -> Result<DecomposeUserStoriesResponse, String> {
    log::info!(
        "Starting user story decomposition with AI (provider: {}, model: {})",
        request.provider,
        request.model
    );

    // 使用 AI 进行用户故事拆分，传递已有用户故事信息
    let existing_stories_ref = request.existing_stories.as_deref();

    match user_story_ai_service::decompose_with_ai(
        &request.prd_content,
        &request.provider,
        &request.model,
        request.api_key.as_deref(),
        existing_stories_ref,
    )
    .await
    {
        Ok(user_stories) => {
            log::info!(
                "User story decomposition completed. Generated {} stories",
                user_stories.len()
            );

            Ok(DecomposeUserStoriesResponse {
                success: true,
                user_stories,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("User story decomposition failed: {}", e);

            // 降级策略：如果 AI 调用失败，返回错误信息
            Ok(DecomposeUserStoriesResponse {
                success: false,
                user_stories: vec![],
                error_message: Some(format!("AI 拆分失败：{}", e)),
            })
        }
    }
}

/// 分解用户故事(流式版本)
#[tauri::command]
pub async fn decompose_user_stories_streaming(
    request: DecomposeUserStoriesRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    user_story_streaming::decompose_user_stories_streaming(request, app).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decompose_user_stories_without_api_key() {
        // 测试没有 API Key 时的行为 - 现在会尝试调用AI，但因API key无效而失败
        let request = DecomposeUserStoriesRequest {
            prd_content: "我们需要一个任务管理系统".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4-turbo-preview".to_string(),
            api_key: None,
            project_id: None,
            existing_stories: None,
        };

        let result = decompose_user_stories(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // 由于 API key 无效，AI 调用会失败
        assert!(!response.success);
        assert!(response.error_message.is_some());
        // 错误信息应该包含 AI 服务调用失败的相关信息
        let error_msg = response.error_message.unwrap();
        assert!(error_msg.contains("AI 拆分失败"));
    }

    #[tokio::test]
    #[ignore] // 需要真实的 API Key，默认忽略
    async fn test_decompose_user_stories_with_api_key() {
        // 这个测试需要设置 OPENAI_API_KEY 环境变量或在请求中提供 API Key
        let request = DecomposeUserStoriesRequest {
            prd_content: "我们需要一个任务管理系统，包含用户注册、登录、任务创建和管理功能"
                .to_string(),
            provider: "openai".to_string(),
            model: "gpt-4-turbo-preview".to_string(),
            api_key: None, // 将从环境变量读取
            project_id: None,
            existing_stories: None,
        };

        let result = decompose_user_stories(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        if response.success {
            assert!(!response.user_stories.is_empty());

            // 验证第一个故事的结构
            let first_story = &response.user_stories[0];
            assert!(!first_story.id.is_empty());
            assert!(!first_story.story_number.is_empty());
            assert!(!first_story.title.is_empty());
            assert!(!first_story.role.is_empty());
            assert!(!first_story.feature.is_empty());
            assert!(!first_story.benefit.is_empty());
            assert!(!first_story.acceptance_criteria.is_empty());
            assert!(["P0", "P1", "P2", "P3"].contains(&first_story.priority.as_str()));
        }
    }
}

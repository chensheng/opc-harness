//! Mock AI Provider for testing Native Coding Agent
//!
//! 提供模拟的 AI Provider 实现，用于单元测试而无需真实 API 调用。

use std::sync::{Arc, Mutex};

use crate::ai::{AIError, ChatRequest, ChatResponse, Message, Usage};

/// Mock AI Provider 配置
#[derive(Debug, Clone)]
pub struct MockAIConfig {
    /// 预设响应内容
    pub response_content: String,
    /// 是否模拟工具调用
    pub simulate_tool_calls: bool,
    /// 模拟的工具调用次数
    pub tool_call_count: usize,
    /// 模拟 Token 使用量
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
}

impl Default for MockAIConfig {
    fn default() -> Self {
        Self {
            response_content: "Task completed successfully.".to_string(),
            simulate_tool_calls: false,
            tool_call_count: 0,
            prompt_tokens: 100,
            completion_tokens: 50,
        }
    }
}

/// Mock AI Provider
pub struct MockAIProvider {
    config: MockAIConfig,
    call_count: Arc<Mutex<usize>>,
}

impl MockAIProvider {
    /// 创建新的 Mock AI Provider
    pub fn new(config: MockAIConfig) -> Self {
        Self {
            config,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    /// 获取调用次数
    pub fn get_call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// 模拟聊天请求
    pub async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse, AIError> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        let current_call = *count;

        // 模拟延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        if self.config.simulate_tool_calls && current_call <= self.config.tool_call_count {
            // 返回工具调用 JSON
            let tool_call_json =
                format!(r#"{{"name": "read_file", "arguments": {{"path": "src/main.rs"}}}}"#);

            Ok(ChatResponse {
                content: tool_call_json,
                model: "mock-model".to_string(),
                usage: Some(Usage {
                    prompt_tokens: self.config.prompt_tokens as i32,
                    completion_tokens: self.config.completion_tokens as i32,
                    total_tokens: (self.config.prompt_tokens + self.config.completion_tokens)
                        as i32,
                }),
            })
        } else {
            // 返回最终响应
            Ok(ChatResponse {
                content: self.config.response_content.clone(),
                model: "mock-model".to_string(),
                usage: Some(Usage {
                    prompt_tokens: self.config.prompt_tokens as i32,
                    completion_tokens: self.config.completion_tokens as i32,
                    total_tokens: (self.config.prompt_tokens + self.config.completion_tokens)
                        as i32,
                }),
            })
        }
    }

    /// 模拟流式聊天（简化版）
    pub async fn stream_chat<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let response = self.chat(request).await?;

        // 模拟流式输出
        for ch in response.content.chars() {
            on_chunk(ch.to_string())?;
        }

        Ok(response.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_basic_response() {
        let config = MockAIConfig {
            response_content: "Test response".to_string(),
            simulate_tool_calls: false,
            ..Default::default()
        };

        let provider = MockAIProvider::new(config);

        let request = ChatRequest {
            model: "test-model".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: None,
            max_tokens: None,
            stream: false,
            project_id: None,
        };

        let response = provider.chat(request).await.unwrap();

        assert_eq!(response.content, "Test response");
        assert_eq!(provider.get_call_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_provider_tool_calls() {
        let config = MockAIConfig {
            response_content: "Final response".to_string(),
            simulate_tool_calls: true,
            tool_call_count: 2,
            ..Default::default()
        };

        let provider = MockAIProvider::new(config);

        let request = ChatRequest::default();

        // 前两次调用应该返回工具调用
        for i in 1..=2 {
            let response = provider.chat(request.clone()).await.unwrap();
            assert!(response.content.contains("read_file"));
            assert_eq!(provider.get_call_count(), i);
        }

        // 第三次调用应该返回最终响应
        let response = provider.chat(request).await.unwrap();
        assert_eq!(response.content, "Final response");
        assert_eq!(provider.get_call_count(), 3);
    }

    #[tokio::test]
    async fn test_mock_provider_streaming() {
        let config = MockAIConfig {
            response_content: "Stream test".to_string(),
            ..Default::default()
        };

        let provider = MockAIProvider::new(config);
        let collected_chunks = Arc::new(Mutex::new(Vec::new()));

        let chunks_clone = collected_chunks.clone();
        let request = ChatRequest::default();

        provider
            .stream_chat(request, move |chunk| {
                chunks_clone.lock().unwrap().push(chunk);
                Ok(())
            })
            .await
            .unwrap();

        let chunks = collected_chunks.lock().unwrap();
        let full_content: String = chunks.iter().cloned().collect();

        assert_eq!(full_content, "Stream test");
    }

    #[tokio::test]
    async fn test_mock_provider_token_usage() {
        let config = MockAIConfig {
            prompt_tokens: 200,
            completion_tokens: 100,
            ..Default::default()
        };

        let provider = MockAIProvider::new(config);
        let request = ChatRequest::default();

        let response = provider.chat(request).await.unwrap();

        assert!(response.usage.is_some());
        let usage = response.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 200);
        assert_eq!(usage.completion_tokens, 100);
        assert_eq!(usage.total_tokens, 300);
    }
}

//! OpenAI Provider - Standalone Implementation
//!
//! 独立的 OpenAI Provider 实现，支持自定义 base_url

use futures::StreamExt;
use log::{debug, error, info, warn};
use reqwest::Client;

use super::ai_types::*;
use super::provider_core::{OpenAIChatResponse, OpenAIStreamChunk};

/// OpenAI Provider 实现
pub struct OpenAIProvider {
    pub(super) api_key: String,
    client: Client,
    pub(super) base_url: String,
}

impl OpenAIProvider {
    /// 创建新的 OpenAI Provider
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// 支持自定义 base_url（用于兼容 OpenAI 格式的第三方 API）
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url,
        }
    }

    /// 验证 API Key
    pub async fn validate_api_key(&self) -> Result<bool, AIError> {
        let url = format!("{}/models", self.base_url);

        info!("Validating OpenAI API key");

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| {
                error!("Failed to validate API key: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        if response.status().is_success() {
            info!("API key validation successful");
            Ok(true)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!("API key validation failed: {}", error_text);
            Err(AIError {
                message: format!("Invalid API key: {}", error_text),
            })
        }
    }

    /// 发送聊天请求
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
        });

        debug!("Sending chat request to {}", url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("Chat request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("OpenAI API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("OpenAI API error ({}): {}", status, error_text),
            });
        }

        let json: OpenAIChatResponse = response.json().await.map_err(|e| {
            error!("Failed to parse response: {}", e);
            AIError {
                message: e.to_string(),
            }
        })?;

        info!("Chat response received, tokens used: {:?}", json.usage);

        let content = json.choices[0].message.content.clone();

        Ok(ChatResponse {
            content,
            model: request.model,
            usage: json.usage.map(|u| Usage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
        })
    }

    /// 流式聊天
    pub async fn stream_chat<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let url = format!("{}/chat/completions", self.base_url);

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
            "stream": true,
        });

        info!("Sending stream chat request");

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("Stream request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Stream API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("Stream API error ({}): {}", status, error_text),
            });
        }

        // 处理流式响应
        let mut full_content = String::new();
        let mut stream = response.bytes_stream();

        let mut chunk_count = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| {
                error!("Failed to read stream chunk: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data.trim() == "[DONE]" {
                        info!("Stream completed");
                        break;
                    }

                    match serde_json::from_str::<OpenAIStreamChunk>(data) {
                        Ok(stream_data) => {
                            if let Some(content) = &stream_data.choices[0].delta.content {
                                full_content.push_str(content);
                                on_chunk(content.clone())?;
                                chunk_count += 1;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse stream chunk: {}", e);
                        }
                    }
                }
            }
        }

        info!(
            "Stream finished, chunks: {}, length: {}",
            chunk_count,
            full_content.len()
        );
        Ok(full_content)
    }
}

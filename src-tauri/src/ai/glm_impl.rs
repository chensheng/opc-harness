//! GLM (Zhipu) Provider Implementation

use futures::StreamExt;
use log::{debug, error, info};

use super::ai_types::*;
use super::provider_core::{AIProvider, OpenAIChatResponse};

impl AIProvider {
    /// GLM 非流式聊天
    pub(super) async fn chat_glm(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/chat/completions", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
        });

        info!("Sending GLM chat request to: {}", url);
        debug!("Request body: {:?}", body);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("GLM request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("GLM API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("GLM API error ({}): {}", status, error_text),
            });
        }

        let json: OpenAIChatResponse = response.json().await.map_err(|e| {
            error!("Failed to parse GLM response: {}", e);
            AIError {
                message: e.to_string(),
            }
        })?;

        info!(
            "GLM chat response received, tokens used: {:?}",
            json.usage
        );

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

    /// GLM 流式聊天
    pub(super) async fn stream_chat_glm<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let url = format!("{}/chat/completions", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
            "stream": true,
        });

        info!("Sending GLM stream chat request to: {}", url);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("GLM stream request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("GLM stream API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("GLM stream API error ({}): {}", status, error_text),
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

            // 解析 SSE 数据
            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data.trim() == "[DONE]" {
                        continue;
                    }

                    if let Ok(delta) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = delta["choices"][0]["delta"]["content"].as_str() {
                            full_content.push_str(content);
                            on_chunk(content.to_string())?;
                            chunk_count += 1;
                        }
                    }
                }
            }
        }

        info!("GLM streaming completed, processed {} chunks", chunk_count);
        Ok(full_content)
    }
}

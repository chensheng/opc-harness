//! Anthropic Provider Implementation

use futures::StreamExt;

use super::ai_types::*;
use super::provider_core::AIProvider;

impl AIProvider {
    /// Anthropic 非流式聊天
    pub(super) async fn chat_anthropic(
        &self,
        request: ChatRequest,
    ) -> Result<ChatResponse, AIError> {
        let url = format!("{}/messages", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_tokens": request.max_tokens.unwrap_or(1024),
            "temperature": request.temperature.unwrap_or(0.7),
        });

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError {
                message: e.to_string(),
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError {
                message: error_text,
            });
        }

        let json: serde_json::Value = response.json().await.map_err(|e| AIError {
            message: e.to_string(),
        })?;

        let content = json["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(ChatResponse {
            content,
            model: request.model,
            usage: None,
        })
    }

    /// Anthropic 流式聊天
    pub(super) async fn stream_chat_anthropic<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let url = format!("{}/messages", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_tokens": request.max_tokens.unwrap_or(1024),
            "temperature": request.temperature.unwrap_or(0.7),
            "stream": true,
        });

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError {
                message: e.to_string(),
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError {
                message: error_text,
            });
        }

        // 处理流式响应
        let mut full_content = String::new();
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AIError {
                message: e.to_string(),
            })?;

            // 解析 SSE 数据 (Anthropic 格式)
            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(event_type) = json["type"].as_str() {
                            if event_type == "content_block_delta" {
                                if let Some(content) = json["delta"]["text"].as_str() {
                                    full_content.push_str(content);
                                    on_chunk(content.to_string())?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(full_content)
    }
}

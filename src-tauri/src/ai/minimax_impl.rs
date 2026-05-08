//! MiniMax Provider Implementation

use super::ai_types::*;
use super::provider_core::AIProvider;

impl AIProvider {
    /// MiniMax 非流式聊天
    pub(super) async fn chat_minimax(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/text/chat", self.get_base_url());

        // 转换为 MiniMax 消息格式
        let minimax_messages: Vec<serde_json::Value> = request
            .messages
            .into_iter()
            .map(|msg| {
                let sender_type = if msg.role == "user" { "USER" } else { "BOT" };
                serde_json::json!({
                    "sender_type": sender_type,
                    "text": msg.content
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": request.model,
            "messages": minimax_messages,
            "stream": false,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(2048),
        });

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError {
                message: format!("请求失败：{}", e),
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError {
                message: format!("API 请求失败：{}", error_text),
            });
        }

        let response_body: serde_json::Value = response.json().await.map_err(|e| AIError {
            message: format!("解析响应失败：{}", e),
        })?;

        let reply = response_body["reply"].as_str().unwrap_or("").to_string();

        let total_tokens = response_body["usage"]["total_tokens"].as_u64().unwrap_or(0) as i32;

        Ok(ChatResponse {
            content: reply,
            model: request.model,
            usage: Some(Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens,
            }),
        })
    }

    /// MiniMax 流式聊天
    pub(super) async fn stream_chat_minimax<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let url = format!("{}/text/chat", self.get_base_url());

        let minimax_messages: Vec<serde_json::Value> = request
            .messages
            .into_iter()
            .map(|msg| {
                let sender_type = if msg.role == "user" { "USER" } else { "BOT" };
                serde_json::json!({
                    "sender_type": sender_type,
                    "text": msg.content
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": request.model,
            "messages": minimax_messages,
            "stream": true,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(2048),
        });

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError {
                message: format!("请求失败：{}", e),
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError {
                message: format!("API 请求失败：{}", error_text),
            });
        }

        // 处理 SSE 流
        let mut full_content = String::new();
        let text = response.text().await.map_err(|e| AIError {
            message: format!("读取流失败：{}", e),
        })?;

        for line in text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    break;
                }

                if let Ok(chunk) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(reply) = chunk.get("reply").and_then(|r| r.as_str()) {
                        full_content.push_str(reply);
                        on_chunk(reply.to_string())?;
                    }
                }
            }
        }

        Ok(full_content)
    }
}

//! Kimi Provider Implementation

use futures::StreamExt;
use log::{error, info, warn};

use super::ai_types::*;
use super::provider_core::AIProvider;

impl AIProvider {
    /// Kimi 非流式聊天（支持 Coding 模型）
    pub(super) async fn chat_kimi(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // Kimi Coding 模型使用 Anthropic Claude 兼容 API，其他模型使用 OpenAI 兼容 API
        let is_coding_model = request.model.starts_with("kimi-coding") || request.model == "kimi-code";
        
        if is_coding_model {
            info!("Using Kimi Coding (Anthropic-compatible) API for model: {}", request.model);
            
            // 检查 API Key 格式
            let auth_header_value = self.get_auth_header().1;
            let api_key = auth_header_value.strip_prefix("Bearer ").unwrap_or(&auth_header_value);
            if !api_key.starts_with("sk-kimi-") {
                warn!("Kimi Coding API Key 格式可能不正确。期望以 'sk-kimi-' 开头，实际：{}", api_key);
            }
            
            // 构建系统消息
            let mut messages = Vec::new();
            messages.push(Message {
                role: "system".to_string(),
                content: "你是 Kimi，由 Moonshot AI 提供的人工智能助手。你会为用户提供安全，有帮助，准确的回答。同时，你会拒绝一切涉及恐怖主义，种族歧视，黄色暴力等问题的回答。Moonshot AI 为专有名词，不可翻译成其他语言。".to_string(),
            });
            messages.extend(request.messages);
            
            // 使用 Anthropic Claude 兼容 API
            let url = "https://api.kimi.com/coding/v1/messages".to_string();
            
            let coding_model_name = if request.model == "kimi-code" {
                "kimi-for-coding"
            } else {
                &request.model
            };
            
            let body = serde_json::json!({
                "model": coding_model_name,
                "messages": messages,
                "max_tokens": request.max_tokens.unwrap_or(2048),
                "temperature": request.temperature.unwrap_or(0.7),
            });

            info!("Sending Kimi Coding chat request to: {} with model: {}", url, coding_model_name);

            let response = self.client
                .post(&url)
                .header(self.get_auth_header().0, self.get_auth_header().1)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    error!("Kimi Coding request failed: {}", e);
                    AIError { message: e.to_string() }
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                error!("Kimi Coding API error ({}): {}", status, error_text);
                
                let error_message = if status.as_u16() == 404 {
                    format!(
                        "Kimi Coding API 未找到 (404)\n\n\
                         可能的原因:\n\
                         1. API Key 格式不正确（需要以 'sk-kimi-' 开头）\n\
                         2. API Key 不是 Kimi Coding 专用的（需要在会员页面生成）\n\
                         3. 使用了普通的 Moonshot API Key\n\n\
                         当前配置:\n\
                         - Base URL: https://api.kimi.com/coding/v1/messages\n\
                         - 模型：{}\n\
                         - API Key 前缀：{}\n\n\
                         API 响应：{}",
                        coding_model_name,
                        if api_key.starts_with("sk-kimi-") { "sk-kimi-..." } else { &api_key[..8.min(api_key.len())] },
                        error_text
                    )
                } else {
                    format!("Kimi Coding API error ({}): {}", status, error_text)
                };
                
                return Err(AIError {
                    message: error_message,
                });
            }

            let json: serde_json::Value = response.json().await.map_err(|e| AIError { message: e.to_string() })?;
            let content = json["content"][0]["text"].as_str().unwrap_or("").to_string();

            Ok(ChatResponse { content, model: request.model, usage: None })
        } else {
            info!("Using standard Kimi (OpenAI-compatible) API for model: {}", request.model);
            // 标准 Kimi API - 添加 system prompt
            let mut messages = Vec::new();
            messages.push(Message {
                role: "system".to_string(),
                content: "你是 Kimi，由 Moonshot AI 提供的人工智能助手。你会为用户提供安全，有帮助，准确的回答。同时，你会拒绝一切涉及恐怖主义，种族歧视，黄色暴力等问题的回答。Moonshot AI 为专有名词，不可翻译成其他语言。".to_string(),
            });
            messages.extend(request.messages);
            
            let kimi_request = ChatRequest {
                model: request.model,
                messages,
                temperature: request.temperature,
                max_tokens: request.max_tokens,
                stream: false,
                project_id: None,
            };
            
            self.chat_openai(kimi_request).await
        }
    }

    /// Kimi 流式聊天（支持 Coding 模型）
    pub(super) async fn stream_chat_kimi<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // Kimi Coding 模型使用 Anthropic Claude 兼容 API，其他模型使用 OpenAI 兼容 API
        let is_coding_model = request.model.starts_with("kimi-coding") || request.model == "kimi-code";
        
        if is_coding_model {
            info!("Using Kimi Coding (Anthropic-compatible) streaming API for model: {}", request.model);
            
            // 检查 API Key 格式
            let auth_header_value = self.get_auth_header().1;
            let api_key = auth_header_value.strip_prefix("Bearer ").unwrap_or(&auth_header_value);
            if !api_key.starts_with("sk-kimi-") {
                warn!("Kimi Coding API Key 格式可能不正确。期望以 'sk-kimi-' 开头，实际：{}", api_key);
            }
            
            // 构建系统消息
            let mut messages = Vec::new();
            messages.push(Message {
                role: "system".to_string(),
                content: "你是 Kimi，由 Moonshot AI 提供的人工智能助手。你会为用户提供安全，有帮助，准确的回答。同时，你会拒绝一切涉及恐怖主义，种族歧视，黄色暴力等问题的回答。Moonshot AI 为专有名词，不可翻译成其他语言。".to_string(),
            });
            messages.extend(request.messages);
            
            // 使用 Anthropic Claude 兼容流式 API
            let url = "https://api.kimi.com/coding/v1/messages".to_string();
            
            let coding_model_name = if request.model == "kimi-code" {
                "kimi-for-coding"
            } else {
                &request.model
            };
            
            let body = serde_json::json!({
                "model": coding_model_name,
                "messages": messages,
                "max_tokens": request.max_tokens.unwrap_or(2048),
                "temperature": request.temperature.unwrap_or(0.7),
                "stream": true,
            });

            info!("Sending Kimi Coding stream request to: {} with model: {}", url, coding_model_name);

            let response = self.client
                .post(&url)
                .header(self.get_auth_header().0, self.get_auth_header().1)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    error!("Kimi Coding stream request failed: {}", e);
                    AIError { message: e.to_string() }
                })?;

            info!("Kimi Coding stream response status: {}", response.status());
            
            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                error!("Kimi Coding stream API error ({}): {}", status, error_text);
                
                if status.as_u16() == 404 {
                    return Err(AIError {
                        message: format!(
                            "Kimi Coding API 未找到 (404)\n\n\
                             可能的原因:\n\
                             1. API Key 格式不正确（需要以 'sk-kimi-' 开头）\n\
                             2. API Key 不是 Kimi Coding 专用的（需要在会员页面生成）\n\
                             3. 使用了普通的 Moonshot API Key\n\
                             4. Kimi Coding 可能不支持流式输出\n\n\
                             当前配置:\n\
                             - Base URL: https://api.kimi.com/coding/v1/messages\n\
                             - 模型：{}\n\
                             - API Key 前缀：{}\n\n\
                             API 响应：{}",
                            coding_model_name,
                            if api_key.starts_with("sk-kimi-") { "sk-kimi-..." } else { &api_key[..8.min(api_key.len())] },
                            error_text
                        ),
                    });
                }
                
                return Err(AIError {
                    message: format!("Kimi Coding stream API error ({}): {}", status, error_text),
                });
            }

            // 处理流式响应 (Anthropic 格式)
            let mut full_content = String::new();
            let mut stream = response.bytes_stream();

            let mut chunk_count = 0;
            let mut parse_error_count = 0;
            let mut no_data_count = 0;
            let mut sent_chunk_count = 0;
            
            info!("Starting to process Kimi Coding stream...");
            
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.map_err(|e| {
                    error!("Stream chunk read error: {}", e);
                    AIError { message: e.to_string() }
                })?;
                let text = String::from_utf8_lossy(&chunk);
                chunk_count += 1;
                
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data:") {
                        if data.trim() == "[DONE]" {
                            info!("Stream completed marker received");
                            break;
                        }
                        
                        match serde_json::from_str::<serde_json::Value>(data) {
                            Ok(event) => {
                                // 尝试从 content_block 提取
                                if let Some(content_block) = event.get("content_block") {
                                    if let Some(text) = content_block.get("text").and_then(|t| t.as_str()) {
                                        info!("Extracted text from content_block: {} chars", text.len());
                                        full_content.push_str(text);
                                        on_chunk(text.to_string())?;
                                        sent_chunk_count += 1;
                                    }
                                }
                                
                                // 尝试从 delta 提取
                                if let Some(delta) = event.get("delta") {
                                    if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                        full_content.push_str(text);
                                        on_chunk(text.to_string())?;
                                        sent_chunk_count += 1;
                                    }
                                }
                                
                                if event.get("content_block").is_none() && event.get("delta").is_none() {
                                    no_data_count += 1;
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse JSON: {}. Data: {}", e, data);
                                parse_error_count += 1;
                            }
                        }
                    }
                }
            }
            
            info!(
                "Stream finished. Total chunks: {}, Final content length: {}, Parse errors: {}, No-data events: {}, Sent chunks: {}", 
                chunk_count, 
                full_content.len(), 
                parse_error_count, 
                no_data_count,
                sent_chunk_count
            );

            Ok(full_content)
        } else {
            info!("Using standard Kimi (OpenAI-compatible) streaming API for model: {}", request.model);
            // 标准 Kimi API - 添加 system prompt
            let mut messages = Vec::new();
            messages.push(Message {
                role: "system".to_string(),
                content: "你是 Kimi，由 Moonshot AI 提供的人工智能助手。你会为用户提供安全，有帮助，准确的回答。同时，你会拒绝一切涉及恐怖主义，种族歧视，黄色暴力等问题的回答。Moonshot AI 为专有名词，不可翻译成其他语言。".to_string(),
            });
            messages.extend(request.messages);
            
            let kimi_request = ChatRequest {
                model: request.model,
                messages,
                temperature: request.temperature,
                max_tokens: request.max_tokens,
                stream: true,
                project_id: None,
            };
            
            self.stream_chat_openai(kimi_request, on_chunk).await
        }
    }
}

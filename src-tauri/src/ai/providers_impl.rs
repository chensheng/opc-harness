//! AI Provider Implementations - Module Coordinator
//! 
//! 此文件作为占位符，实际的 Provider 实现已拆分为独立文件：
//! - openai_impl.rs
//! - anthropic_impl.rs
//! - kimi_impl.rs
//! - glm_impl.rs
//! - minimax_impl.rs
//! - deepl_impl.rs
//! 
//! 这些模块在 ai/mod.rs 中直接声明

// 子模块声明
mod openai_impl;
mod anthropic_impl;
mod kimi_impl;
mod glm_impl;
mod minimax_impl;
mod deepl_impl;

// 注意：此文件不需要 re-export，因为所有实现都是 impl AIProvider 的扩展方法
// 它们在 provider_core.rs 中已经被包含在 AIProvider 的实现块中

use log::{debug, error, info, warn};
use futures::StreamExt;

use super::ai_types::*;
use super::provider_core::{AIProvider, OpenAIChatResponse, OpenAIStreamChunk};

impl AIProvider {
    // =====================================================================
    // OpenAI Implementation
    // =====================================================================

    pub(super) async fn chat_openai(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/chat/completions", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
        });

        log::info!("Sending OpenAI chat request to: {}", url);
        log::debug!("Request body: {:?}", body);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::error!("OpenAI request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            log::error!("OpenAI API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("OpenAI API error ({}): {}", status, error_text),
            });
        }

        let json: OpenAIChatResponse = response.json().await.map_err(|e| {
            log::error!("Failed to parse OpenAI response: {}", e);
            AIError {
                message: e.to_string(),
            }
        })?;

        log::info!(
            "OpenAI chat response received, tokens used: {:?}",
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

    pub(super) async fn stream_chat_openai<F>(
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

        log::info!("Sending OpenAI stream chat request to: {}", url);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::error!("OpenAI stream request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            log::error!("OpenAI stream API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("OpenAI stream API error ({}): {}", status, error_text),
            });
        }

        // 处理流式响应
        let mut full_content = String::new();
        let mut stream = response.bytes_stream();

        let mut chunk_count = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| {
                log::error!("Failed to read stream chunk: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

            // 解析 SSE 数据
            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data:") {
                    if data.trim() == "[DONE]" {
                        log::info!("OpenAI stream completed");
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
                            log::warn!("Failed to parse stream chunk: {}", e);
                        }
                    }
                }
            }
        }

        log::info!(
            "OpenAI stream finished, total chunks: {}, content length: {}",
            chunk_count,
            full_content.len()
        );

        Ok(full_content)
    }

    // =====================================================================
    // Anthropic Implementation
    // =====================================================================

    pub(super) async fn chat_anthropic(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
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

    // =====================================================================
    // Kimi Implementation
    // =====================================================================

    pub(super) async fn chat_kimi(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // Kimi Coding 模型使用 Anthropic Claude 兼容 API，其他模型使用 OpenAI 兼容 API
        let is_coding_model = request.model.starts_with("kimi-coding") || request.model == "kimi-code";
        
        if is_coding_model {
            log::info!("Using Kimi Coding (Anthropic-compatible) API for model: {}", request.model);
            
            // 检查 API Key 格式
            let auth_header_value = self.get_auth_header().1;
            let api_key = auth_header_value.strip_prefix("Bearer ").unwrap_or(&auth_header_value);
            if !api_key.starts_with("sk-kimi-") {
                log::warn!("Kimi Coding API Key 格式可能不正确。期望以 'sk-kimi-' 开头，实际：{}", api_key);
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

            log::info!("Sending Kimi Coding chat request to: {} with model: {}", url, coding_model_name);

            let response = self.client
                .post(&url)
                .header(self.get_auth_header().0, self.get_auth_header().1)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    log::error!("Kimi Coding request failed: {}", e);
                    AIError { message: e.to_string() }
                })?;

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                log::error!("Kimi Coding API error ({}): {}", status, error_text);
                
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
            log::info!("Using standard Kimi (OpenAI-compatible) API for model: {}", request.model);
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
            };
            
            self.chat_openai(kimi_request).await
        }
    }

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
            log::info!("Using Kimi Coding (Anthropic-compatible) streaming API for model: {}", request.model);
            
            // 检查 API Key 格式
            let auth_header_value = self.get_auth_header().1;
            let api_key = auth_header_value.strip_prefix("Bearer ").unwrap_or(&auth_header_value);
            if !api_key.starts_with("sk-kimi-") {
                log::warn!("Kimi Coding API Key 格式可能不正确。期望以 'sk-kimi-' 开头，实际：{}", api_key);
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

            log::info!("Sending Kimi Coding stream request to: {} with model: {}", url, coding_model_name);

            let response = self.client
                .post(&url)
                .header(self.get_auth_header().0, self.get_auth_header().1)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    log::error!("Kimi Coding stream request failed: {}", e);
                    AIError { message: e.to_string() }
                })?;

            log::info!("Kimi Coding stream response status: {}", response.status());
            
            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                log::error!("Kimi Coding stream API error ({}): {}", status, error_text);
                
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
            
            log::info!("Starting to process Kimi Coding stream...");
            
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.map_err(|e| {
                    log::error!("Stream chunk read error: {}", e);
                    AIError { message: e.to_string() }
                })?;
                let text = String::from_utf8_lossy(&chunk);
                chunk_count += 1;
                
                log::debug!("Received chunk {}: {} bytes", chunk_count, chunk.len());
                
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data:") {
                        if data.trim() == "[DONE]" {
                            log::info!("Stream completed marker received");
                            break;
                        }
                        
                        match serde_json::from_str::<serde_json::Value>(data) {
                            Ok(event) => {
                                // 尝试从 content_block 提取
                                if let Some(content_block) = event.get("content_block") {
                                    if let Some(text) = content_block.get("text").and_then(|t| t.as_str()) {
                                        log::info!("Extracted text from content_block: {} chars", text.len());
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
                                log::error!("Failed to parse JSON: {}. Data: {}", e, data);
                                parse_error_count += 1;
                            }
                        }
                    }
                }
            }
            
            log::info!(
                "Stream finished. Total chunks: {}, Final content length: {}, Parse errors: {}, No-data events: {}, Sent chunks: {}", 
                chunk_count, 
                full_content.len(), 
                parse_error_count, 
                no_data_count,
                sent_chunk_count
            );

            Ok(full_content)
        } else {
            log::info!("Using standard Kimi (OpenAI-compatible) streaming API for model: {}", request.model);
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
            };
            
            self.stream_chat_openai(kimi_request, on_chunk).await
        }
    }

    // =====================================================================
    // GLM (Zhipu) Implementation
    // =====================================================================

    pub(super) async fn chat_glm(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/chat/completions", self.get_base_url());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(1024),
        });

        log::info!("Sending GLM chat request to: {}", url);
        log::debug!("Request body: {:?}", body);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::error!("GLM request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            log::error!("GLM API error ({}): {}", status, error_text);
            return Err(AIError {
                message: format!("GLM API error ({}): {}", status, error_text),
            });
        }

        let json: OpenAIChatResponse = response.json().await.map_err(|e| {
            log::error!("Failed to parse GLM response: {}", e);
            AIError {
                message: e.to_string(),
            }
        })?;

        log::info!(
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

        log::info!("Sending GLM stream chat request to: {}", url);

        let response = self
            .client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::error!("GLM stream request failed: {}", e);
                AIError {
                    message: e.to_string(),
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            log::error!("GLM stream API error ({}): {}", status, error_text);
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
                log::error!("Failed to read stream chunk: {}", e);
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

        log::info!("GLM streaming completed, processed {} chunks", chunk_count);
        Ok(full_content)
    }

    // =====================================================================
    // MiniMax Implementation
    // =====================================================================

    pub(super) async fn chat_minimax(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/text/chat", self.get_base_url());
        
        // 转换为 MiniMax 消息格式
        let minimax_messages: Vec<serde_json::Value> = request.messages.into_iter().map(|msg| {
            let sender_type = if msg.role == "user" { "USER" } else { "BOT" };
            serde_json::json!({
                "sender_type": sender_type,
                "text": msg.content
            })
        }).collect();
        
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

        let reply = response_body["reply"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let total_tokens = response_body["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0) as i32;

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

    pub(super) async fn stream_chat_minimax<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        let url = format!("{}/text/chat", self.get_base_url());
        
        let minimax_messages: Vec<serde_json::Value> = request.messages.into_iter().map(|msg| {
            let sender_type = if msg.role == "user" { "USER" } else { "BOT" };
            serde_json::json!({
                "sender_type": sender_type,
                "text": msg.content
            })
        }).collect();
        
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

    // =====================================================================
    // DeepL Implementation (Translation Service Placeholder)
    // =====================================================================

    pub(super) async fn chat_deepl(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // DeepL 是翻译专用 API，这里提供一个简单的占位实现
        Ok(ChatResponse {
            content: "DeepL translation service placeholder".to_string(),
            model: request.model,
            usage: None,
        })
    }

    pub(super) async fn stream_chat_deepl<F>(&self, _request: ChatRequest, _on_chunk: F) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // DeepL streaming placeholder
        Ok("DeepL streaming service placeholder".to_string())
    }
}

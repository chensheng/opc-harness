//! AI 模块 - 提供 AI Provider 和智能路由功能

pub mod router;

use log::{debug, error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,  // Claude
    Kimi,
    GLM,
    MiniMax,
    DeepL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub session_id: String,
    pub content: String,
    pub is_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComplete {
    pub session_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamError {
    pub session_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// AI 服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub base_url: Option<String>,
}

impl AIConfig {
    /// 创建带有 API key 的配置
    pub fn with_key(provider: String, model: String, api_key: String) -> Self {
        Self {
            provider,
            model,
            api_key,
            base_url: None,
        }
    }

    /// 创建带有自定义 base_url 的配置
    pub fn with_base_url(provider: String, model: String, api_key: String, base_url: String) -> Self {
        Self {
            provider,
            model,
            api_key,
            base_url: Some(base_url),
        }
    }
}

#[derive(Debug)]
pub struct AIError {
    pub message: String,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AIError {}

pub struct AIProvider {
    provider_type: AIProviderType,
    api_key: String,
    client: Client,
}

impl AIProvider {
    pub fn new(provider_type: AIProviderType, api_key: String) -> Self {
        Self {
            provider_type,
            api_key,
            client: Client::new(),
        }
    }

    fn get_base_url(&self) -> String {
        match self.provider_type {
            AIProviderType::OpenAI => "https://api.openai.com/v1".to_string(),
            AIProviderType::Anthropic => "https://api.anthropic.com/v1".to_string(),
            AIProviderType::Kimi => "https://api.moonshot.cn/v1".to_string(),
            AIProviderType::GLM => "https://open.bigmodel.cn/api/paas/v4".to_string(),
            AIProviderType::MiniMax => "https://api.minimax.chat/v1".to_string(),
            AIProviderType::DeepL => "https://api-free.deepl.com/v2".to_string(),
        }
    }

    fn get_auth_header(&self) -> (String, String) {
        match self.provider_type {
            AIProviderType::Anthropic => ("x-api-key".to_string(), self.api_key.clone()),
            _ => (
                "Authorization".to_string(),
                format!("Bearer {}", self.api_key),
            ),
        }
    }

    pub async fn validate_key(&self) -> Result<bool, AIError> {
        // Simple validation - make a test request
        match self.provider_type {
            AIProviderType::OpenAI => {
                let response = self
                    .client
                    .get(format!("{}/models", self.get_base_url()))
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: e.to_string(),
                    })?;

                Ok(response.status().is_success())
            }
            _ => {
                // For other providers, assume valid for now
                Ok(true)
            }
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        match self.provider_type {
            AIProviderType::OpenAI => self.chat_openai(request).await,
            AIProviderType::Anthropic => self.chat_anthropic(request).await,
            AIProviderType::Kimi => self.chat_kimi(request).await,
            AIProviderType::GLM => self.chat_glm(request).await,
            AIProviderType::MiniMax => self.chat_minimax(request).await,
            AIProviderType::DeepL => self.chat_deepl(request).await,
        }
    }

    pub async fn stream_chat<F>(&self, request: ChatRequest, on_chunk: F) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        match self.provider_type {
            AIProviderType::OpenAI => self.stream_chat_openai(request, on_chunk).await,
            AIProviderType::Anthropic => self.stream_chat_anthropic(request, on_chunk).await,
            AIProviderType::Kimi => self.stream_chat_kimi(request, on_chunk).await,
            AIProviderType::GLM => self.stream_chat_glm(request, on_chunk).await,
            AIProviderType::MiniMax => self.stream_chat_minimax(request, on_chunk).await,
            AIProviderType::DeepL => self.stream_chat_deepl(request, on_chunk).await,
        }
    }

    async fn chat_openai(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
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

    async fn stream_chat_openai<F>(
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

        use futures::StreamExt;
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
                            // Continue processing other chunks
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

    async fn chat_anthropic(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
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

    async fn stream_chat_anthropic<F>(
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

        use futures::StreamExt;
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

    async fn chat_kimi(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // Kimi Coding 模型使用 Anthropic Claude 兼容 API，其他模型使用 OpenAI 兼容 API
        let is_coding_model = request.model.starts_with("kimi-coding") || request.model == "kimi-code";
        
        if is_coding_model {
            log::info!("Using Kimi Coding (Anthropic-compatible) API for model: {}", request.model);
            
            // 检查 API Key 格式
            let api_key = self.get_auth_header().1;
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
            let url = "https://api.kimi.com/coding/messages".to_string();
            
            // Kimi Coding 可能需要特定的模型名称
            let coding_model_name = if request.model == "kimi-code" {
                "kimi-for-coding"  // 根据官方文档，Kimi Coding API 使用的模型名称
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
            log::debug!("Request headers:");
            log::debug!("  Authorization: ***");
            log::debug!("  anthropic-version: 2023-06-01");
            log::debug!("  Content-Type: application/json");
            log::debug!("Request body: {:?}", body);

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
                
                // 提供更详细的错误提示
                let error_message = if status.as_u16() == 404 {
                    format!(
                        "Kimi Coding API 未找到 (404)\n\n\
                         可能的原因:\n\
                         1. API Key 格式不正确（需要以 'sk-kimi-' 开头）\n\
                         2. API Key 不是 Kimi Coding 专用的（需要在会员页面生成）\n\
                         3. 使用了普通的 Moonshot API Key\n\n\
                         当前配置:\n\
                         - Base URL: https://api.kimi.com/coding/\n\
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

    async fn stream_chat_kimi<F>(
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
            let api_key = self.get_auth_header().1;
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
            // 注意：Kimi Coding API 可能需要特殊的模型名称格式
            let url = "https://api.kimi.com/coding/messages".to_string();
            
            // Kimi Coding 可能需要特定的模型名称
            let coding_model_name = if request.model == "kimi-code" {
                "kimi-for-coding"  // 根据官方文档，Kimi Coding API 使用的模型名称
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
            log::debug!("Request headers:");
            log::debug!("  Authorization: ***");
            log::debug!("  anthropic-version: 2023-06-01");
            log::debug!("  Content-Type: application/json");
            log::debug!("Request body: {:?}", body);

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

            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                log::error!("Kimi Coding stream API error ({}): {}", status, error_text);
                
                // 如果是 404，提示用户可能不支持流式或 API Key 不正确
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
                             - Base URL: https://api.kimi.com/coding/\n\
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

            use futures::StreamExt;
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.map_err(|e| AIError { message: e.to_string() })?;
                let text = String::from_utf8_lossy(&chunk);
                
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim() == "[DONE]" {
                            break;
                        }
                        
                        if let Ok(event) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(content_block) = event.get("content_block") {
                                if let Some(text) = content_block.get("text").and_then(|t| t.as_str()) {
                                    full_content.push_str(text);
                                    on_chunk(text.to_string())?;
                                }
                            }
                            if let Some(delta) = event.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    full_content.push_str(text);
                                    on_chunk(text.to_string())?;
                                }
                            }
                        }
                    }
                }
            }

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

    async fn chat_glm(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // GLM uses OpenAI-compatible API
        self.chat_openai(request).await
    }

    async fn stream_chat_glm<F>(&self, request: ChatRequest, on_chunk: F) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // GLM uses OpenAI-compatible API
        self.stream_chat_openai(request, on_chunk).await
    }

    async fn chat_minimax(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // MiniMax API 调用
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

    async fn stream_chat_minimax<F>(
        &self,
        request: ChatRequest,
        mut on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // MiniMax 流式 API 调用
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

    /// 获取提供商 ID
    pub fn provider_id(&self) -> &str {
        match self.provider_type {
            AIProviderType::OpenAI => "openai",
            AIProviderType::Anthropic => "anthropic",
            AIProviderType::Kimi => "kimi",
            AIProviderType::GLM => "glm",
            AIProviderType::MiniMax => "minimax",
            AIProviderType::DeepL => "deepl",
        }
    }

    // =====================================================================
    // DeepL API 实现 (翻译专用)
    // =====================================================================

    async fn chat_deepl(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // DeepL 是翻译专用 API，这里提供一个简单的占位实现
        // 实际使用需要调用 DeepL API
        Ok(ChatResponse {
            content: "DeepL translation service placeholder".to_string(),
            model: request.model,
            usage: None,
        })
    }

    async fn stream_chat_deepl<F>(&self, _request: ChatRequest, _on_chunk: F) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // DeepL streaming placeholder
        Ok("DeepL streaming service placeholder".to_string())
    }
}

/// AI 服务管理器
/// 
/// 统一管理多个 AI Provider，提供统一的调用入口
pub struct AIServiceManager {
    services: HashMap<String, AIProvider>,
    default_provider: String,
}

impl AIServiceManager {
    /// 创建新的 AI 服务管理器
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            default_provider: "openai".to_string(),
        }
    }

    /// 从配置注册 AI 服务
    pub fn register_from_config(&mut self, config: AIConfig) -> Result<(), AIError> {
        let provider_type = match config.provider.as_str() {
            "openai" => AIProviderType::OpenAI,
            "anthropic" => AIProviderType::Anthropic,
            "kimi" => AIProviderType::Kimi,
            "glm" => AIProviderType::GLM,
            "minimax" => AIProviderType::MiniMax,
            _ => {
                return Err(AIError {
                    message: format!("Unknown provider: {}", config.provider),
                });
            }
        };

        let provider = AIProvider::new(provider_type, config.api_key);
        self.services.insert(config.provider, provider);
        Ok(())
    }

    /// 批量注册多个 AI 服务
    pub fn register_multiple(&mut self, configs: Vec<AIConfig>) -> Result<(), AIError> {
        for config in configs {
            self.register_from_config(config)?;
        }
        Ok(())
    }

    /// 获取指定的 AI 服务
    pub fn get(&self, provider: &str) -> Option<&AIProvider> {
        self.services.get(provider)
    }

    /// 获取可变引用的 AI 服务
    pub fn get_mut(&mut self, provider: &str) -> Option<&mut AIProvider> {
        self.services.get_mut(provider)
    }

    /// 获取默认的 AI 服务
    pub fn get_default(&self) -> Option<&AIProvider> {
        self.services.get(&self.default_provider)
    }

    /// 设置默认提供商
    pub fn set_default(&mut self, provider: String) {
        if self.services.contains_key(&provider) {
            self.default_provider = provider;
        } else {
            warn!("Trying to set non-registered provider as default: {}", provider);
        }
    }

    /// 获取所有已注册的提供商 ID
    pub fn registered_providers(&self) -> Vec<&str> {
        self.services.keys().map(|s| s.as_str()).collect()
    }

    /// 检查某个提供商是否已注册
    pub fn is_registered(&self, provider: &str) -> bool {
        self.services.contains_key(provider)
    }

    /// 获取已注册的服务数量
    pub fn count(&self) -> usize {
        self.services.len()
    }

    /// 清空所有注册的服务
    pub fn clear(&mut self) {
        self.services.clear();
    }

    /// 移除指定的 AI 服务
    pub fn remove(&mut self, provider: &str) -> bool {
        let removed = self.services.remove(provider).is_some();
        if removed && self.default_provider == provider {
            // 如果移除的是默认 provider，重新设置默认
            self.default_provider = self.services.keys().next().cloned().unwrap_or_else(|| "openai".to_string());
        }
        removed
    }
}

impl Default for AIServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenAI Provider 实现
pub struct OpenAIProvider {
    api_key: String,
    client: Client,
    base_url: String,
}

impl OpenAIProvider {
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

        use futures::StreamExt;
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

/// OpenAI API 响应结构
#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    index: i32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

/// OpenAI 流式响应块
#[derive(Debug, Deserialize)]
struct OpenAIStreamChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    index: i32,
    delta: OpenAIDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    role: Option<String>,
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_creation() {
        let api_key = "sk-test123".to_string();
        let provider = OpenAIProvider::new(api_key.clone());

        // 验证 provider 创建成功
        assert_eq!(provider.api_key, api_key);
    }

    #[test]
    fn test_openai_provider_with_custom_url() {
        let api_key = "sk-test123".to_string();
        let base_url = "https://custom.api.com/v1".to_string();
        let provider = OpenAIProvider::with_base_url(api_key.clone(), base_url.clone());

        assert_eq!(provider.api_key, api_key);
        assert_eq!(provider.base_url, base_url);
    }

    #[test]
    fn test_message_creation() {
        let message = Message {
            role: "user".to_string(),
            content: "Hello, OpenAI!".to_string(),
        };

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello, OpenAI!");
    }

    #[test]
    fn test_chat_request_creation() {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ];

        let request = ChatRequest {
            model: "gpt-4".to_string(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(1024),
            stream: false,
        };

        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.temperature, Some(0.7));
        assert!(!request.stream);
    }

    // ========== Kimi Provider Tests ==========

    #[test]
    fn test_kimi_provider_creation() {
        let api_key = "sk-kimi-test123".to_string();
        let provider = AIProvider::new(AIProviderType::Kimi, api_key.clone());

        // 验证 provider 创建成功
        assert_eq!(provider.get_base_url(), "https://api.moonshot.cn/v1");
    }

    #[test]
    fn test_kimi_provider_base_url() {
        let provider = AIProvider::new(AIProviderType::Kimi, "test-key".to_string());
        assert_eq!(provider.get_base_url(), "https://api.moonshot.cn/v1");
    }

    #[test]
    fn test_kimi_provider_auth_header() {
        let api_key = "sk-kimi-test123".to_string();
        let provider = AIProvider::new(AIProviderType::Kimi, api_key.clone());
        let (header_name, header_value) = provider.get_auth_header();

        assert_eq!(header_name, "Authorization");
        assert_eq!(header_value, format!("Bearer {}", api_key));
    }

    #[test]
    fn test_kimi_models() {
        let provider = AIProvider::new(AIProviderType::Kimi, "test-key".to_string());
        
        // Kimi 支持的模型列表
        let models = vec![
            "kimi-k2",
            "kimi-k2-0711", 
            "moonshot-v1-8k",
            "moonshot-v1-32k",
            "moonshot-v1-128k",
        ];
        
        // 验证 provider 已就绪（有 API key）
        // 实际验证需要网络请求，这里只检查 provider 类型
        assert_eq!(provider.get_base_url(), "https://api.moonshot.cn/v1");
    }

    #[tokio::test]
    async fn test_kimi_chat_request_structure() {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "你是一个有用的助手。".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "你好！".to_string(),
            },
        ];

        let request = ChatRequest {
            model: "moonshot-v1-8k".to_string(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(1024),
            stream: false,
        };

        assert_eq!(request.model, "moonshot-v1-8k");
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(1024));
        assert!(!request.stream);
    }

    #[tokio::test]
    async fn test_kimi_chat_error_without_key() {
        let provider = AIProvider::new(AIProviderType::Kimi, "".to_string());
        let request = ChatRequest {
            model: "moonshot-v1-8k".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: Some(0.7),
            max_tokens: Some(1024),
            stream: false,
        };

        let result = provider.chat(request).await;
        // 空 API key 应该导致请求失败
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_kimi_stream_chat_error_without_key() {
        let provider = AIProvider::new(AIProviderType::Kimi, "".to_string());
        let request = ChatRequest {
            model: "moonshot-v1-8k".to_string(),
            messages: vec![],
            temperature: None,
            max_tokens: None,
            stream: true,
        };

        let mut chunks = Vec::new();
        let on_chunk = |chunk: String| -> Result<(), AIError> {
            chunks.push(chunk);
            Ok(())
        };

        let result = provider.stream_chat(request, on_chunk).await;
        // 空 API key 应该导致请求失败
        assert!(result.is_err());
    }

    #[test]
    fn test_kimi_provider_type() {
        let provider = AIProvider::new(AIProviderType::Kimi, "test-key".to_string());
        
        // 验证 provider 类型
        match provider {
            AIProvider { provider_type: AIProviderType::Kimi, .. } => {
                // 正确匹配 Kimi 类型
                assert!(true);
            }
            _ => panic!("Expected Kimi provider type"),
        }
    }

    // ========== AI Service Manager Tests ==========

    #[test]
    fn test_ai_service_manager_creation() {
        let manager = AIServiceManager::new();
        assert_eq!(manager.default_provider, "openai");
        assert!(manager.registered_providers().is_empty());
    }

    #[test]
    fn test_ai_service_manager_register_openai() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-test".to_string());
        
        let result = manager.register_from_config(config);
        assert!(result.is_ok());
        
        let providers = manager.registered_providers();
        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&"openai"));
    }

    #[test]
    fn test_ai_service_manager_register_multiple() {
        let mut manager = AIServiceManager::new();
        let configs = vec![
            AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-openai".to_string()),
            AIConfig::with_key("kimi".to_string(), "moonshot-v1-8k".to_string(), "sk-kimi".to_string()),
            AIConfig::with_key("glm".to_string(), "glm-4-plus".to_string(), "sk-glm".to_string()),
        ];
        
        let result = manager.register_multiple(configs);
        assert!(result.is_ok());
        
        let providers = manager.registered_providers();
        assert_eq!(providers.len(), 3);
        assert!(providers.contains(&"openai"));
        assert!(providers.contains(&"kimi"));
        assert!(providers.contains(&"glm"));
    }

    #[test]
    fn test_ai_service_manager_get_service() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-test".to_string());
        manager.register_from_config(config).unwrap();
        
        let service = manager.get("openai");
        assert!(service.is_some());
    }

    #[test]
    fn test_ai_service_manager_get_default() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-test".to_string());
        manager.register_from_config(config).unwrap();
        
        let default = manager.get_default();
        assert!(default.is_some());
    }

    #[test]
    fn test_ai_service_manager_set_default() {
        let mut manager = AIServiceManager::new();
        let openai_config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-openai".to_string());
        let kimi_config = AIConfig::with_key("kimi".to_string(), "moonshot-v1-8k".to_string(), "sk-kimi".to_string());
        
        manager.register_from_config(openai_config).unwrap();
        manager.register_from_config(kimi_config).unwrap();
        
        manager.set_default("kimi".to_string());
        assert_eq!(manager.default_provider, "kimi");
        
        let default = manager.get_default().unwrap();
        assert_eq!(default.provider_id(), "kimi");
    }

    #[test]
    fn test_ai_service_manager_is_registered() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-test".to_string());
        manager.register_from_config(config).unwrap();
        
        assert!(manager.is_registered("openai"));
        assert!(!manager.is_registered("anthropic"));
    }

    #[test]
    fn test_ai_service_manager_count() {
        let mut manager = AIServiceManager::new();
        assert_eq!(manager.count(), 0);
        
        let configs = vec![
            AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-openai".to_string()),
            AIConfig::with_key("kimi".to_string(), "moonshot-v1-8k".to_string(), "sk-kimi".to_string()),
        ];
        manager.register_multiple(configs).unwrap();
        
        assert_eq!(manager.count(), 2);
    }

    #[test]
    fn test_ai_service_manager_clear() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key("openai".to_string(), "gpt-4o".to_string(), "sk-test".to_string());
        manager.register_from_config(config).unwrap();
        
        assert_eq!(manager.count(), 1);
        manager.clear();
        assert_eq!(manager.count(), 0);
        assert!(manager.registered_providers().is_empty());
    }
}

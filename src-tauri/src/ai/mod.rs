use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    Kimi,
    GLM,
    MiniMax,
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
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
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
        }
    }

    fn get_auth_header(&self) -> (String, String) {
        match self.provider_type {
            AIProviderType::Anthropic => (
                "x-api-key".to_string(),
                self.api_key.clone(),
            ),
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
                let response = self.client
                    .get(format!("{}/models", self.get_base_url()))
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .send()
                    .await
                    .map_err(|e| AIError { message: e.to_string() })?;
                
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
        }
    }

    async fn chat_openai(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/chat/completions", self.get_base_url());
        
        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens,
        });

        let response = self.client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError { message: e.to_string() })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError { message: error_text });
        }

        let json: serde_json::Value = response.json().await.map_err(|e| AIError { message: e.to_string() })?;
        
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(ChatResponse {
            content,
            model: request.model,
            usage: None,
        })
    }

    async fn chat_anthropic(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        let url = format!("{}/messages", self.get_base_url());
        
        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_tokens": request.max_tokens.unwrap_or(1024),
            "temperature": request.temperature.unwrap_or(0.7),
        });

        let response = self.client
            .post(&url)
            .header(self.get_auth_header().0, self.get_auth_header().1)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError { message: e.to_string() })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError { message: error_text });
        }

        let json: serde_json::Value = response.json().await.map_err(|e| AIError { message: e.to_string() })?;
        
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

    async fn chat_kimi(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // Kimi uses OpenAI-compatible API
        self.chat_openai(request).await
    }

    async fn chat_glm(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // GLM uses OpenAI-compatible API
        self.chat_openai(request).await
    }

    async fn chat_minimax(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // MiniMax implementation
        Ok(ChatResponse {
            content: "MiniMax response placeholder".to_string(),
            model: request.model,
            usage: None,
        })
    }
}

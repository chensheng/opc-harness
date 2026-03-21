//! AI Provider module for LLM integrations

pub mod openai;
pub mod anthropic;
pub mod kimi;
pub mod glm;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// AI Provider trait for unified LLM interface
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;
    
    /// Check if provider is configured and ready
    fn is_ready(&self) -> bool;
    
    /// Send a completion request
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse, AIError>;
    
    /// Stream completion response
    async fn stream_complete(
        &self,
        request: CompletionRequest,
    ) -> Result<Box<dyn Stream<Item = Result<String, AIError>> + Send + Unpin>, AIError>;
}

/// Completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub usage: Option<Usage>,
}

/// Token usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// AI Error
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Provider not configured")]
    NotConfigured,
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

use futures::Stream;

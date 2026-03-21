//! OpenAI adapter

use super::{AIError, AIProvider, CompletionRequest, CompletionResponse, Message, Role};
use async_trait::async_trait;
use futures::Stream;

pub struct OpenAIAdapter {
    api_key: Option<String>,
    base_url: String,
}

impl OpenAIAdapter {
    pub fn new(api_key: Option<String>, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com".to_string()),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIAdapter {
    fn name(&self) -> &str {
        "openai"
    }

    fn is_ready(&self) -> bool {
        self.api_key.is_some()
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse, AIError> {
        // TODO: Implement actual API call
        Ok(CompletionResponse {
            content: "OpenAI response placeholder".to_string(),
            usage: None,
        })
    }

    async fn stream_complete(
        &self,
        _request: CompletionRequest,
    ) -> Result<Box<dyn Stream<Item = Result<String, AIError>> + Send + Unpin>, AIError> {
        // TODO: Implement actual streaming
        Err(AIError::NotConfigured)
    }
}

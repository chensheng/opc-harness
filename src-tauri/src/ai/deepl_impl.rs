//! DeepL Provider Implementation (Translation Service Placeholder)

use super::ai_types::*;
use super::provider_core::AIProvider;

impl AIProvider {
    /// DeepL 非流式聊天（占位实现）
    pub(super) async fn chat_deepl(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // DeepL 是翻译专用 API，这里提供一个简单的占位实现
        Ok(ChatResponse {
            content: "DeepL translation service placeholder".to_string(),
            model: request.model,
            usage: None,
        })
    }

    /// DeepL 流式聊天（占位实现）
    pub(super) async fn stream_chat_deepl<F>(
        &self,
        _request: ChatRequest,
        _on_chunk: F,
    ) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        // DeepL streaming placeholder
        Ok("DeepL streaming service placeholder".to_string())
    }
}

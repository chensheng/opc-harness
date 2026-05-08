//! AI 模块 - 提供 AI Provider 和智能路由功能

#![allow(dead_code)]

pub mod router;

// 子模块
mod ai_types;
mod openai_provider;
mod provider_core;
mod service_manager;

// Provider 实现子模块（在 providers_impl 内部声明会导致路径问题）
mod anthropic_impl;
mod codefree_impl;
mod deepl_impl;
mod glm_impl;
mod kimi_impl;
mod minimax_impl;
mod openai_impl; // CodeFree CLI Provider

// 重新导出常用类型
pub use ai_types::*;
pub use provider_core::AIProvider;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::openai_provider::OpenAIProvider;
    use crate::ai::service_manager::AIServiceManager;

    #[test]
    fn test_openai_provider_creation() {
        let api_key = "sk-test123".to_string();
        let provider = OpenAIProvider::new(api_key.clone());
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
            project_id: None,
        };
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.temperature, Some(0.7));
        assert!(!request.stream);
    }

    #[test]
    fn test_kimi_provider_creation() {
        let api_key = "sk-kimi-test123".to_string();
        let provider = AIProvider::new(AIProviderType::Kimi, api_key.clone());
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
            project_id: None,
        };
        let result = provider.chat(request).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_ai_service_manager_creation() {
        let manager = AIServiceManager::new();
        assert_eq!(manager.default_provider, "openai");
        assert!(manager.registered_providers().is_empty());
    }

    #[test]
    fn test_ai_service_manager_register_openai() {
        let mut manager = AIServiceManager::new();
        let config = AIConfig::with_key(
            "openai".to_string(),
            "gpt-4o".to_string(),
            "sk-test".to_string(),
        );
        let result = manager.register_from_config(config);
        assert!(result.is_ok());
        let providers = manager.registered_providers();
        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&"openai"));
    }

    #[test]
    fn test_ai_service_manager_count() {
        let mut manager = AIServiceManager::new();
        assert_eq!(manager.count(), 0);
        let configs = vec![
            AIConfig::with_key(
                "openai".to_string(),
                "gpt-4o".to_string(),
                "sk-openai".to_string(),
            ),
            AIConfig::with_key(
                "kimi".to_string(),
                "moonshot-v1-8k".to_string(),
                "sk-kimi".to_string(),
            ),
        ];
        manager.register_multiple(configs).unwrap();
        assert_eq!(manager.count(), 2);
    }
}

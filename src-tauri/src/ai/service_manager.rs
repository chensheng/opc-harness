//! AI Service Manager
//!
//! 统一管理多个 AI Provider，提供统一的调用入口

use log::warn;
use std::collections::HashMap;

use super::ai_types::AIProviderType;
use super::ai_types::{AIConfig, AIError};
use super::provider_core::AIProvider;

/// AI 服务管理器
pub struct AIServiceManager {
    services: HashMap<String, AIProvider>,
    pub(super) default_provider: String,
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
            warn!(
                "Trying to set non-registered provider as default: {}",
                provider
            );
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
            self.default_provider = self
                .services
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| "openai".to_string());
        }
        removed
    }
}

impl Default for AIServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

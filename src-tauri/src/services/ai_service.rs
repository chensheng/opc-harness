//! AI服务
//! 
//! 提供统一的AI调用接口，支持多家厂商

use crate::models::{AIProviderConfig, AIModel};
use anyhow::Result;

/// AI服务
pub struct AIService {
    config: AIProviderConfig,
}

impl AIService {
    /// 创建新的AI服务实例
    pub fn new(config: AIProviderConfig) -> Self {
        Self { config }
    }

    /// 发送聊天请求
    pub async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        // TODO: 根据厂商调用不同API
        Ok("AI response placeholder".to_string())
    }

    /// 流式聊天
    pub async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        callback: impl Fn(String),
    ) -> Result<()> {
        // TODO: 实现流式输出
        callback("Streaming response placeholder".to_string());
        Ok(())
    }

    /// 生成PRD
    pub async fn generate_prd(&self, idea: &str) -> Result<String> {
        // TODO: 构造Prompt并调用AI
        Ok(format!("# PRD for: {}\n\n(Generated content)", idea))
    }

    /// 生成用户画像
    pub async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        // TODO: 生成用户画像
        Ok(vec![])
    }

    /// 生成竞品分析
    pub async fn generate_competitor_analysis(&self, idea: &str) -> Result<String> {
        // TODO: 生成竞品分析
        Ok(format!("# Competitor Analysis for: {}\n\n(Generated content)", idea))
    }

    /// 获取支持的模型列表
    pub fn get_models(&self) -> Vec<AIModel> {
        vec![
            AIModel {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                provider: "openai".to_string(),
                max_tokens: Some(128000),
            },
            AIModel {
                id: "gpt-4o-mini".to_string(),
                name: "GPT-4o Mini".to_string(),
                provider: "openai".to_string(),
                max_tokens: Some(128000),
            },
        ]
    }

    /// 验证API密钥
    pub async fn validate_key(&self) -> Result<bool> {
        // TODO: 调用厂商API验证密钥
        Ok(true)
    }
}

/// AI服务管理器
pub struct AIServiceManager {
    services: std::collections::HashMap<String, AIService>,
    default_provider: String,
}

impl AIServiceManager {
    /// 创建管理器
    pub fn new() -> Self {
        Self {
            services: std::collections::HashMap::new(),
            default_provider: "openai".to_string(),
        }
    }

    /// 注册服务
    pub fn register(&mut self, provider: String, service: AIService) {
        self.services.insert(provider, service);
    }

    /// 获取服务
    pub fn get(&self, provider: &str) -> Option<&AIService> {
        self.services.get(provider)
    }

    /// 获取默认服务
    pub fn get_default(&self) -> Option<&AIService> {
        self.services.get(&self.default_provider)
    }

    /// 设置默认提供商
    pub fn set_default(&mut self, provider: String) {
        self.default_provider = provider;
    }
}

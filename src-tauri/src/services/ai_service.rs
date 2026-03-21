//! AI 服务
//! 
//! 提供统一的 AI 调用接口，支持多家厂商

use crate::models::{AIProviderConfig, AIModel};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

/// AI 服务
pub struct AIService {
    config: AIProviderConfig,
    client: Client,
}

impl AIService {
    /// 创建新的 AI 服务实例
    pub fn new(config: AIProviderConfig) -> Self {
        Self { 
            config,
            client: Client::new(),
        }
    }

    /// 验证 API 密钥（通用方法）
    pub async fn validate_key(&self) -> Result<bool> {
        // 根据 provider 调用不同的验证接口
        match self.config.provider.as_str() {
            "openai" => self.validate_openai_key().await,
            "anthropic" => self.validate_anthropic_key().await,
            "kimi" => self.validate_kimi_key().await,
            "glm" => self.validate_glm_key().await,
            _ => Ok(false),
        }
    }

    /// 验证 OpenAI API 密钥
    async fn validate_openai_key(&self) -> Result<bool> {
        let api_key = match &self.config.api_key {
            Some(key) => key,
            None => return Ok(false),
        };

        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/models", base_url);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// 验证 Anthropic API 密钥
    async fn validate_anthropic_key(&self) -> Result<bool> {
        let api_key = match &self.config.api_key {
            Some(key) => key,
            None => return Ok(false),
        };

        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.anthropic.com");
        let url = format!("{}/v1/complete", base_url);

        // 使用一个轻量级的请求测试
        let body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens_to_sample": 10,
            "prompt": "\n\nHuman: test\n\nAssistant:"
        });

        let response = self.client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        // 401/403 表示密钥无效，其他错误可能是网络问题
        match response.status().as_u16() {
            401 | 403 => Ok(false),
            _ => Ok(true), // 其他状态码认为密钥有效（可能是请求格式问题）
        }
    }

    /// 验证 Kimi API 密钥
    async fn validate_kimi_key(&self) -> Result<bool> {
        let api_key = match &self.config.api_key {
            Some(key) => key,
            None => return Ok(false),
        };

        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.moonshot.cn/v1");
        let url = format!("{}/models", base_url);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// 验证智谱 GLM API 密钥
    async fn validate_glm_key(&self) -> Result<bool> {
        let api_key = match &self.config.api_key {
            Some(key) => key,
            None => return Ok(false),
        };

        let base_url = self.config.base_url.as_deref().unwrap_or("https://open.bigmodel.cn/api/paas/v4");
        let url = format!("{}/models", base_url);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// 发送聊天请求
    pub async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        // TODO: 根据厂商调用不同 API
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

    /// 生成 PRD
    pub async fn generate_prd(&self, idea: &str) -> Result<String> {
        // TODO: 构造 Prompt 并调用 AI
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

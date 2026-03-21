//! AI 服务
//! 
//! 提供统一的 AI 调用接口，支持多家厂商

use crate::models::{AIProviderConfig, AIModel};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use async_trait::async_trait;

/// AI Provider Trait - 定义所有 AI 厂商的统一接口
#[async_trait]
pub trait AIProvider {
    /// 获取提供商 ID
    fn provider_id(&self) -> &str;
    
    /// 获取支持的模型列表
    fn supported_models(&self) -> Vec<&str>;
    
    /// 验证 API 密钥
    async fn validate_api_key(&self) -> Result<bool>;
    
    /// 发送聊天请求（非流式）
    async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String>;
    
    /// 发送聊天请求（流式）
    async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        callback: impl Fn(String) + Send + Sync,
    ) -> Result<()>;
    
    /// 生成 PRD（产品需求文档）
    async fn generate_prd(&self, idea: &str) -> Result<String>;
    
    /// 生成用户画像
    async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>>;
    
    /// 生成竞品分析
    async fn generate_competitor_analysis(&self, idea: &str) -> Result<String>;
}

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

    /// 获取提供商 ID
    pub fn provider_id(&self) -> &str {
        &self.config.provider
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
        // TODO: 根据 provider 返回不同的模型列表
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

// 为 AIService 实现 AIProvider Trait
#[async_trait]
impl AIProvider for AIService {
    fn provider_id(&self) -> &str {
        &self.config.provider
    }
    
    fn supported_models(&self) -> Vec<&str> {
        // TODO: 根据 provider 返回不同的模型列表
        match self.config.provider.as_str() {
            "openai" => vec!["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"],
            "anthropic" => vec!["claude-3-5-sonnet-20241022", "claude-3-opus-20240229", "claude-3-haiku-20240307"],
            "kimi" => vec!["kimi-k2", "kimi-k2-0711", "moonshot-v1-8k", "moonshot-v1-32k", "moonshot-v1-128k"],
            "glm" => vec!["glm-4-plus", "glm-4-0520", "glm-4-air", "glm-4-flash"],
            _ => vec![],
        }
    }
    
    async fn validate_api_key(&self) -> Result<bool> {
        self.validate_key().await
    }
    
    async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        // TODO: 根据厂商调用不同 API
        Ok("AI response placeholder".to_string())
    }
    
    async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        callback: impl Fn(String) + Send + Sync,
    ) -> Result<()> {
        // TODO: 实现流式输出
        callback("Streaming response placeholder".to_string());
        Ok(())
    }
    
    async fn generate_prd(&self, idea: &str) -> Result<String> {
        // TODO: 构造 Prompt 并调用 AI
        Ok(format!("# PRD for: {}\n\n(Generated content)", idea))
    }
    
    async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        // TODO: 生成用户画像
        Ok(vec![])
    }
    
    async fn generate_competitor_analysis(&self, idea: &str) -> Result<String> {
        // TODO: 生成竞品分析
        Ok(format!("# Competitor Analysis for: {}\n\n(Generated content)", idea))
    }
}

/// AI服务管理器（基于 Trait 对象）
pub struct AIServiceManager {
    services: std::collections::HashMap<String, Box<dyn AIProvider + Send + Sync>>,
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

    /// 注册服务（接受任何实现 AIProvider Trait 的类型）
    pub fn register<T: AIProvider + Send + Sync + 'static>(&mut self, provider: String, service: T) {
        self.services.insert(provider, Box::new(service));
    }

    /// 获取服务
    pub fn get(&self, provider: &str) -> Option<&(dyn AIProvider + Send + Sync)> {
        self.services.get(provider).map(|s| s.as_ref())
    }

    /// 获取默认服务
    pub fn get_default(&self) -> Option<&(dyn AIProvider + Send + Sync)> {
        self.services.get(&self.default_provider).map(|s| s.as_ref())
    }

    /// 设置默认提供商
    pub fn set_default(&mut self, provider: String) {
        self.default_provider = provider;
    }
    
    /// 获取所有已注册的提供商 ID
    pub fn registered_providers(&self) -> Vec<&str> {
        self.services.keys().map(|s| s.as_str()).collect()
    }
}

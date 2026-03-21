//! AI 服务
//! 
//! 提供统一的 AI 调用接口，支持多家厂商

use crate::models::{AIProviderConfig, AIModel};
use crate::prompts::generate_prd_prompt;
use anyhow::{Result, Context};
use reqwest::Client;
use serde_json::json;
use async_trait::async_trait;
use std::time::Duration;

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
        callback: Box<dyn Fn(String) + Send + Sync>,
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
}

// 为 AIService 实现 AIProvider trait
#[async_trait]
impl AIProvider for AIService {
    fn provider_id(&self) -> &str {
        &self.config.provider
    }
    
    fn supported_models(&self) -> Vec<&str> {
        // TODO: 根据 provider 返回不同的模型列表
        match self.config.provider.as_str() {
            "openai" => vec!["gpt-4o", "gpt-4o-mini", "gpt-4-turbo"],
            "anthropic" => vec!["claude-3-opus", "claude-3-sonnet", "claude-3-haiku"],
            _ => vec![],
        }
    }
    
    async fn validate_api_key(&self) -> Result<bool> {
        self.validate_key().await
    }
    
    async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        match self.config.provider.as_str() {
            "openai" => self.chat_openai(messages).await,
            _ => self.chat_openai(messages).await, // 默认使用 OpenAI 兼容 API
        }
    }
    
    async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        match self.config.provider.as_str() {
            "openai" => self.stream_chat_openai(messages, callback).await,
            _ => self.stream_chat_openai(messages, callback).await,
        }
    }
    
    async fn generate_prd(&self, idea: &str) -> Result<String> {
        self.generate_prd(idea).await
    }
    
    async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        self.generate_personas(idea).await
    }
    
    async fn generate_competitor_analysis(&self, idea: &str) -> Result<String> {
        self.generate_competitor_analysis(idea).await
    }
}

impl AIService {
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

        // 使用 Messages API 进行验证（Anthropic 推荐使用）
        let url = "https://api.anthropic.com/v1/messages";
        
        let body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 10,
            "messages": [{"role": "user", "content": "Hi"}]
        });

        let response = self.client
            .post(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        // 401/403 表示密钥无效
        match response.status().as_u16() {
            401 | 403 => Ok(false),
            _ => Ok(true), // 其他状态码认为密钥有效
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

    /// Kimi 聊天对话 - Moonshot AI API
    pub async fn chat_kimi(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not set"))?;
        
        let base_url = self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.moonshot.cn/v1".to_string());
        
        let model = if self.config.model.is_empty() {
            "moonshot-v1-8k".to_string()
        } else {
            self.config.model.clone()
        };
        
        let url = format!("{}/chat/completions", base_url);
        
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
            "temperature": 0.7,
            "stream": false
        });
        
        log::info!("Sending Kimi chat request to: {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(30))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Kimi API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Kimi API error: {}", error_text));
        }
        
        let result: serde_json::Value = response.json().await
            .context("Failed to parse Kimi API response")?;
        
        // 解析响应
        result["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Invalid response format from Kimi API"))
    }

    /// Kimi 流式聊天 - SSE
    pub async fn stream_chat_kimi(
        &self,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        use eventsource_stream::Eventsource;
        use futures::StreamExt;
        
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not set"))?;
        
        let base_url = self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.moonshot.cn/v1".to_string());
        
        let model = if self.config.model.is_empty() {
            "moonshot-v1-8k".to_string()
        } else {
            self.config.model.clone()
        };
        
        let url = format!("{}/chat/completions", base_url);
        
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
            "temperature": 0.7,
            "stream": true
        });
        
        log::info!("Sending Kimi streaming request to: {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Kimi API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Kimi streaming error: {}", error_text));
        }
        
        let mut stream = response.bytes_stream().eventsource();
        
        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => {
                    let data = &event.data;
                    if data == "[DONE]" {
                        break;
                    }
                    
                    // 解析 Kimi 的 SSE 数据（与 OpenAI 格式相同）
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) = value["choices"].as_array() {
                            if !choices.is_empty() {
                                if let Some(delta) = choices[0]["delta"]["content"].as_str() {
                                    callback(delta.to_string());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("SSE error: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Kimi PRD 生成
    pub async fn generate_prd_kimi(&self, idea: &str) -> Result<String> {
        let system_prompt = r#"你是一位资深产品经理。请针对用户的产品创意，生成一份完整的产品需求文档（PRD）。

PRD 应包含以下内容：
1. 产品概述：产品定位、目标用户、核心价值
2. 功能需求：核心功能列表、功能详细描述
3. 技术架构：技术选型、系统架构图、关键技术点
4. 开发计划：分阶段开发计划、时间估算
5. 风险评估：技术风险、市场风险、应对策略

请使用 Markdown 格式输出，确保结构清晰、内容详实。"#;

        let messages = vec![json!({
            "role": "system",
            "content": system_prompt
        }), json!({
            "role": "user",
            "content": format!("请为以下产品创意生成PRD：{}", idea)
        })];
        
        self.chat_kimi(messages).await
    }

    /// Kimi 用户画像生成
    pub async fn generate_personas_kimi(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        let system_prompt = r#"你是一位用户体验专家。请针对用户的产品创意，创建 3-5 个典型的用户画像（Personas）。

每个画像应包含：
1. 基本信息：姓名、年龄、职业、收入水平
2. 痛点分析：当前面临的问题和挑战
3. 需求描述：对产品或服务的具体期望
4. 使用场景：在什么情况下会使用这个产品
5. 行为特征：上网习惯、消费偏好等

请以 JSON 数组格式返回，每个画像包含上述字段。"#;

        let messages = vec![json!({
            "role": "system",
            "content": system_prompt
        }), json!({
            "role": "user",
            "content": format!("请为以下产品创意生成用户画像：{}", idea)
        })];
        
        let response = self.chat_kimi(messages).await?;
        
        // 尝试解析 JSON 响应
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response) {
            if json_value.is_array() {
                return Ok(json_value.as_array().unwrap().clone());
            }
        }
        
        // 如果无法解析，返回包装后的响应
        Ok(vec![json!({
            "name": "典型用户",
            "description": response
        })])
    }

    /// Kimi 竞品分析生成
    pub async fn generate_competitor_analysis_kimi(&self, idea: &str) -> Result<String> {
        let system_prompt = r#"你是一位资深市场分析师。请针对用户的产品创意进行全面的竞品分析。

分析内容应包括：
1. 直接竞品：功能相似的产品列表及其特点
2. 间接竞品：解决同类问题的替代方案
3. 竞争优势：差异化机会和竞争优势
4. 市场机会：未满足的市场需求和空白点
5. 进入策略：建议的市场进入策略

请提供详细、可操作的分析报告。"#;

        let messages = vec![json!({
            "role": "system",
            "content": system_prompt
        }), json!({
            "role": "user",
            "content": format!("请对以下产品创意进行竞品分析：{}", idea)
        })];
        
        self.chat_kimi(messages).await
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
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.stream_chat_openai(messages, callback).await,
            _ => {
                callback("Streaming response placeholder".to_string());
                Ok(())
            }
        }
    }

    /// 生成 PRD (VD-018)
    pub async fn generate_prd(&self, idea: &str) -> Result<String> {
        // 使用提示词模板构造完整的提示词
        let prompt = generate_prd_prompt(idea, None);
        
        // 构造消息格式
        let messages = vec![json!({
            "role": "user",
            "content": prompt
        })];
        
        // 根据 provider 选择不同的实现
        match self.config.provider.as_str() {
            "openai" => {
                self.generate_prd_openai(messages).await
            }
            _ => {
                // 默认使用 OpenAI 兼容的 API
                self.generate_prd_generic(messages).await
            }
        }
    }

    /// 生成 PRD - OpenAI 实现
    async fn generate_prd_openai(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        if self.config.api_key.is_none() {
            return Err(anyhow::anyhow!("API key is missing"));
        }

        let api_key = self.config.api_key.as_ref().unwrap();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        let model = &self.config.model;

        // 构造请求体，增加 max_tokens 以支持长输出
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 8192,  // PRD 通常较长
            "temperature": 0.7,
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0,
        });

        log::info!("Sending PRD generation request to {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(120))  // PRD 生成可能需要较长时间
            .json(&body)
            .send()
            .await
            .context("Failed to send PRD generation request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            log::error!("PRD generation API error: {}", error_text);
            return Err(anyhow::anyhow!("PRD generation failed: {}", error_text));
        }

        let result: serde_json::Value = response.json().await
            .context("Failed to parse PRD generation response")?;

        // 提取回复内容
        let content = result["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in PRD generation response"))?;

        log::info!("PRD generated successfully, length: {} chars", content.len());
        
        Ok(content.to_string())
    }

    /// 生成 PRD - 通用实现（兼容其他 API）
    async fn generate_prd_generic(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        if self.config.api_key.is_none() {
            return Err(anyhow::anyhow!("API key is missing"));
        }

        let api_key = self.config.api_key.as_ref().unwrap();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        let model = &self.config.model;

        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 8192,
            "temperature": 0.7,
        });

        log::info!("Sending PRD generation request to {}", url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(120))
            .json(&body)
            .send()
            .await
            .context("Failed to send PRD generation request")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            log::error!("PRD generation API error: {}", error_text);
            return Err(anyhow::anyhow!("PRD generation failed: {}", error_text));
        }

        let result: serde_json::Value = response.json().await
            .context("Failed to parse PRD generation response")?;

        let content = result["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in PRD generation response"))?;

        log::info!("PRD generated successfully, length: {} chars", content.len());
        
        Ok(content.to_string())
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

    /// 发送聊天请求（OpenAI 完整实现）
    pub async fn chat_openai(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        if self.config.provider != "openai" {
            return Err(anyhow::anyhow!("Provider is not OpenAI"));
        }

        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key is missing"))?;
        
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        let model = &self.config.model;

        // 构造 OpenAI Chat Completions API 请求
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
            "temperature": 0.7,
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(30))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let result: serde_json::Value = response.json().await
            .context("Failed to parse OpenAI API response")?;

        // 提取回复内容
        result["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No content in OpenAI response"))
    }

    /// 流式聊天（OpenAI 完整实现）
    pub async fn stream_chat_openai(
        &self,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        if self.config.provider != "openai" {
            return Err(anyhow::anyhow!("Provider is not OpenAI"));
        }

        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key is missing"))?;
        
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url);
        let model = &self.config.model;

        // 构造流式请求
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 4096,
            "temperature": 0.7,
            "stream": true,
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenAI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        // 处理 SSE 流
        use eventsource_stream::Eventsource;
        use futures::StreamExt;

        let mut stream = response.bytes_stream().eventsource();

        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => {
                    // Event 是一个包含 event 和 data 字段的结构体
                    if event.data == "[DONE]" {
                        break;
                    }

                    // 解析 SSE 数据
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&event.data) {
                        if let Some(content) = data["choices"][0]["delta"]["content"].as_str() {
                            if !content.is_empty() {
                                callback(content.to_string());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("SSE error: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }
} // End of AIService impl

// ========== AI Service Manager =========
/// AI 服务管理器 - 统一管理多个 AI Provider
pub struct AIServiceManager {
    services: std::collections::HashMap<String, Box<dyn AIProvider + Send + Sync>>,
    default_provider: String,
}

impl AIServiceManager {
    pub fn new() -> Self {
        Self {
            services: std::collections::HashMap::new(),
            default_provider: "openai".to_string(),
        }
    }

    /// 获取所有已注册的 Provider
    pub fn registered_providers(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// 获取指定 Provider 支持的模型
    pub fn get_supported_models(&self, provider: &str) -> Vec<&str> {
        if let Some(service) = self.services.get(provider) {
            service.supported_models()
        } else {
            vec![]
        }
    }

    /// 注册 AI Provider
    pub fn register<T: AIProvider + Send + Sync + 'static>(&mut self, provider: String, service: T) {
        self.services.insert(provider, Box::new(service));
    }

    /// 获取默认的 AI Provider
    pub fn get_default(&self) -> Option<&(dyn AIProvider + Send + Sync)> {
        self.services.get(&self.default_provider).map(|s| s.as_ref())
    }

    /// 设置默认 Provider
    pub fn set_default(&mut self, provider: String) {
        self.default_provider = provider;
    }

    /// 从配置注册 Provider
    pub fn register_from_config(&mut self, config: crate::models::AIProviderConfig) -> Result<()> {
        use crate::services::ai_service::AIService;
        
        let provider_id = config.provider.clone();
        let ai_service = AIService::new(config);
        self.register(provider_id.clone(), ai_service);
        
        log::info!("Registered AI provider: {}", provider_id);
        Ok(())
    }

    /// 批量注册
    pub fn register_multiple(&mut self, configs: Vec<crate::models::AIProviderConfig>) -> Result<()> {
        for config in configs {
            self.register_from_config(config)?;
        }
        Ok(())
    }

    /// 检查是否已注册
    pub fn is_registered(&self, provider: &str) -> bool {
        self.services.contains_key(provider)
    }

    /// 验证 API 密钥
    pub async fn validate_api_key(&self, provider: &str) -> Result<bool> {
        if let Some(service) = self.services.get(provider) {
            service.validate_api_key().await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }

    /// 聊天
    pub async fn chat(&self, provider: &str, messages: Vec<serde_json::Value>) -> Result<String> {
        if let Some(service) = self.services.get(provider) {
            service.chat(messages).await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }

    /// 生成 PRD
    pub async fn generate_prd(&self, provider: &str, idea: &str) -> Result<String> {
        if let Some(service) = self.services.get(provider) {
            service.generate_prd(idea).await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }

    /// 生成用户画像
    pub async fn generate_personas(&self, provider: &str, idea: &str) -> Result<Vec<serde_json::Value>> {
        if let Some(service) = self.services.get(provider) {
            service.generate_personas(idea).await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }

    /// 生成竞品分析
    pub async fn generate_competitor_analysis(&self, provider: &str, idea: &str) -> Result<String> {
        if let Some(service) = self.services.get(provider) {
            service.generate_competitor_analysis(idea).await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }

    /// 流式聊天
    pub async fn stream_chat(
        &self,
        provider: &str,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        if let Some(service) = self.services.get(provider) {
            service.stream_chat(messages, callback).await
        } else {
            Err(anyhow::anyhow!("Provider not found: {}", provider))
        }
    }
}

impl Default for AIServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Claude (Anthropic) API 实现占位 =========
// TODO: 后续完善 Claude API 支持

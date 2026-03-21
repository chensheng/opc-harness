//! AI 服务
//! 
//! 提供统一的 AI 调用接口，支持多家厂商

use crate::models::{AIProviderConfig, AIModel};
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

    /// 生成 PRD（OpenAI 完整实现）
    pub async fn generate_prd_openai(&self, idea: &str) -> Result<String> {
        let messages = vec![json!({
            "role": "system",
            "content": "你是一位经验丰富的产品经理，擅长将产品想法转化为详细的产品需求文档（PRD）。请按照以下结构输出：\n\n1. 产品概述\n2. 目标用户\n3. 核心功能\n4. 技术架构\n5. 开发计划"
        }), json!({
            "role": "user",
            "content": format!("请将以下产品想法完善为详细的产品需求文档：\n\n{}", idea)
        })];

        self.chat_openai(messages).await
    }

    /// 生成用户画像（OpenAI 完整实现）
    pub async fn generate_personas_openai(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        let messages = vec![json!({
            "role": "system",
            "content": "你是一位用户研究专家，擅长创建详细的用户画像。请为目标产品创建 3-5 个典型用户画像，每个包含：姓名、年龄、职业、痛点、需求、使用场景。"
        }), json!({
            "role": "user",
            "content": format!("请为以下产品创建用户画像：\n\n{}", idea)
        })];

        let response = self.chat_openai(messages).await?;
        
        // 尝试解析为 JSON 数组
        if let Ok(personas) = serde_json::from_str::<Vec<serde_json::Value>>(&response) {
            Ok(personas)
        } else {
            // 如果不是 JSON 格式，返回包装后的结果
            Ok(vec![json!({
                "description": response,
                "source": "openai"
            })])
        }
    }

    /// 生成竞品分析（OpenAI 完整实现）
    pub async fn generate_competitor_analysis_openai(&self, idea: &str) -> Result<String> {
        let messages = vec![json!({
            "role": "system",
            "content": "你是一位战略咨询顾问，擅长竞品分析。请分析目标产品的潜在竞争对手，包括：\n1. 直接竞品\n2. 间接竞品\n3. 竞争优势\n4. 市场机会"
        }), json!({
            "role": "user",
            "content": format!("请对以下产品进行竞品分析：\n\n{}", idea)
        })];

        self.chat_openai(messages).await
    }
    
    // ========== Claude (Anthropic) API 实现 ==========
    
    /// 验证 Claude API 密钥
    pub async fn validate_claude_key(&self) -> Result<bool> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not set"))?;
        
        let url = "https://api.anthropic.com/v1/models";
        
        let response = self.client
            .get(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;
        
        Ok(response.status().is_success())
    }
    
    /// Claude 聊天对话 - Messages API
    pub async fn chat_claude(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not set"))?;
        
        let base_url = self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.anthropic.com".to_string());
        
        let model = if self.config.model.is_empty() {
            "claude-3-5-sonnet-20241022".to_string()
        } else {
            self.config.model.clone()
        };
        
        let url = format!("{}/v1/messages", base_url);
        
        // Claude 的消息格式与 OpenAI 不同
        let mut system_prompt = String::new();
        let mut claude_messages = Vec::new();
        
        for msg in messages {
            if let Some(role) = msg.get("role").and_then(|v| v.as_str()) {
                if role == "system" {
                    system_prompt = msg.get("content")
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();
                } else {
                    // Claude 只支持 user 和 assistant 角色
                    let claude_role = if role == "user" { "user" } else { "assistant" };
                    if let Some(content) = msg.get("content").cloned() {
                        claude_messages.push(json!({
                            "role": claude_role,
                            "content": content
                        }));
                    }
                }
            }
        }
        
        let body = json!({
            "model": model,
            "max_tokens": 4096,
            "messages": claude_messages,
            "system": system_prompt
        });
        
        log::info!("Sending Claude request to: {}", url);
        
        let response = self.client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(30))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Claude API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Claude API error: {}", error_text));
        }
        
        let result: serde_json::Value = response.json().await
            .context("Failed to parse Claude API response")?;
        
        log::debug!("Claude API response: {:?}", result);
        
        result["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No content in Claude response"))
    }
    
    /// Claude 流式聊天 - SSE
    pub async fn stream_chat_claude(
        &self,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        use eventsource_stream::Eventsource;
        use futures::StreamExt;
        
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("API key not set"))?;
        
        let base_url = self.config.base_url.clone()
            .unwrap_or_else(|| "https://api.anthropic.com".to_string());
        
        let model = if self.config.model.is_empty() {
            "claude-3-5-sonnet-20241022".to_string()
        } else {
            self.config.model.clone()
        };
        
        let url = format!("{}/v1/messages", base_url);
        
        // 处理消息格式
        let mut system_prompt = String::new();
        let mut claude_messages = Vec::new();
        
        for msg in messages {
            if let Some(role) = msg.get("role").and_then(|v| v.as_str()) {
                if role == "system" {
                    system_prompt = msg.get("content")
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();
                } else {
                    let claude_role = if role == "user" { "user" } else { "assistant" };
                    if let Some(content) = msg.get("content").cloned() {
                        claude_messages.push(json!({
                            "role": claude_role,
                            "content": content
                        }));
                    }
                }
            }
        }
        
        let body = json!({
            "model": model,
            "max_tokens": 4096,
            "messages": claude_messages,
            "system": system_prompt,
            "stream": true
        });
        
        log::info!("Sending Claude streaming request to: {}", url);
        
        let response = self.client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Claude API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Claude streaming error: {}", error_text));
        }
        
        let mut stream = response.bytes_stream().eventsource();
        
        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => {
                    // event.data is already a String, not Option
                    let data = &event.data;
                    if data == "[DONE]" {
                        break;
                    }
                    
                    // 解析 Claude 的 SSE 数据
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(data) {
                        if value["type"] == "content_block_delta" {
                            if let Some(text) = value["delta"]["text"].as_str() {
                                callback(text.to_string());
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
    
    /// Claude PRD 生成
    pub async fn generate_prd_claude(&self, idea: &str) -> Result<String> {
        let system_prompt = r#"你是一位专业的产品经理助手。请根据用户的产品创意，生成一份结构化的产品需求文档（PRD）。

PRD 应包含以下内容：
1. 产品概述：产品定位、目标市场、核心价值主张
2. 目标用户：主要用户群体、用户画像
3. 核心功能：功能列表、优先级排序、功能描述
4. 技术架构：技术栈建议、系统架构图、关键技术选型
5. 开发计划：MVP 范围、迭代规划、时间估算

请使用清晰的标题和结构化格式。"#;

        let messages = vec![json!({
            "role": "user",
            "content": format!("请为以下产品创意生成 PRD：{}", idea)
        })];
        
        // 使用 system prompt 构建消息
        let all_messages = vec![
            json!({
                "role": "system",
                "content": system_prompt
            }),
            messages[0].clone()
        ];
        
        self.chat_claude(all_messages).await
    }
    
    /// Claude 用户画像生成
    pub async fn generate_personas_claude(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        let system_prompt = r#"你是一位专业的用户研究专家。请根据产品创意创建 3-5 个详细的用户画像。

每个画像应包含：
- 基本信息：姓名、年龄、职业、收入水平
- 痛点分析：当前面临的问题和挑战
- 需求描述：对产品的期望和需求
- 使用场景：典型的使用情境
- 行为特征：技术熟练度、使用习惯等

请以 JSON 数组格式返回。"#;

        let messages = vec![json!({
            "role": "user",
            "content": format!("请为以下产品创意创建用户画像：{}", idea)
        })];
        
        let all_messages = vec![
            json!({
                "role": "system",
                "content": system_prompt
            }),
            messages[0].clone()
        ];
        
        let response = self.chat_claude(all_messages).await?;
        
        // 尝试解析 JSON
        if let Ok(value) = serde_json::from_str::<Vec<serde_json::Value>>(&response) {
            Ok(value)
        } else {
            // 如果不是 JSON 格式，包装为单个对象
            Ok(vec![json!({
                "name": "典型用户",
                "description": response
            })])
        }
    }
    
    /// Claude 竞品分析生成
    pub async fn generate_competitor_analysis_claude(&self, idea: &str) -> Result<String> {
        let system_prompt = r#"你是一位资深市场分析师。请针对用户的产品创意进行全面的竞品分析。

分析内容应包括：
1. 直接竞品：功能相似的产品列表及其特点
2. 间接竞品：解决同类问题的替代方案
3. 竞争优势：差异化机会和竞争优势
4. 市场机会：未满足的市场需求和空白点
5. 进入策略：建议的市场进入策略

请提供详细、可操作的分析报告。"#;

        let messages = vec![json!({
            "role": "user",
            "content": format!("请对以下产品创意进行竞品分析：{}", idea)
        })];
        
        let all_messages = vec![
            json!({
                "role": "system",
                "content": system_prompt
            }),
            messages[0].clone()
        ];
        
        self.chat_claude(all_messages).await
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
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.chat_openai(messages).await,
            "anthropic" => self.chat_claude(messages).await,
            _ => Ok("AI response placeholder".to_string()),
        }
    }
    
    async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        callback: Box<dyn Fn(String) + Send + Sync>,
    ) -> Result<()> {
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.stream_chat_openai(messages, callback).await,
            "anthropic" => self.stream_chat_claude(messages, callback).await,
            _ => {
                callback("Streaming response placeholder".to_string());
                Ok(())
            }
        }
    }
    
    async fn generate_prd(&self, idea: &str) -> Result<String> {
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.generate_prd_openai(idea).await,
            "anthropic" => self.generate_prd_claude(idea).await,
            _ => Ok(format!("# PRD for: {}\n\n(Generated content)", idea)),
        }
    }
    
    async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.generate_personas_openai(idea).await,
            "anthropic" => self.generate_personas_claude(idea).await,
            _ => Ok(vec![]),
        }
    }
    
    async fn generate_competitor_analysis(&self, idea: &str) -> Result<String> {
        // 根据提供商调用具体实现
        match self.config.provider.as_str() {
            "openai" => self.generate_competitor_analysis_openai(idea).await,
            "anthropic" => self.generate_competitor_analysis_claude(idea).await,
            _ => Ok(format!("# Competitor Analysis for: {}\n\n(Generated content)", idea)),
        }
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

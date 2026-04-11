//! AI Types and Data Structures
//! 
//! 包含所有 AI 相关的类型定义、配置和数据结构

use serde::{Deserialize, Serialize};
use std::fmt;
use std::error::Error;

/// AI 提供商类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,  // Claude
    Kimi,
    GLM,
    MiniMax,
    DeepL,
    CodeFree,   // CodeFree CLI
}

/// AI 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// AI 聊天请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub stream: bool,
    /// 项目 ID（用于 CodeFree CLI 切换工作目录）
    #[serde(default)]
    pub project_id: Option<String>,
}

impl Default for ChatRequest {
    fn default() -> Self {
        Self {
            model: String::new(),
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
            stream: false,
            project_id: None,
        }
    }
}

/// AI 聊天响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
    pub usage: Option<Usage>,
}

/// 流式响应块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub session_id: String,
    pub content: String,
    pub is_complete: bool,
}

/// 流式响应完成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComplete {
    pub session_id: String,
    pub content: String,
}

/// 流式响应错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamError {
    pub session_id: String,
    pub error: String,
}

/// Token 使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// AI 服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub base_url: Option<String>,
}

impl AIConfig {
    /// 创建带有 API key 的配置
    pub fn with_key(provider: String, model: String, api_key: String) -> Self {
        Self {
            provider,
            model,
            api_key,
            base_url: None,
        }
    }

    /// 创建带有自定义 base_url 的配置
    pub fn with_base_url(provider: String, model: String, api_key: String, base_url: String) -> Self {
        Self {
            provider,
            model,
            api_key,
            base_url: Some(base_url),
        }
    }
}

/// AI 错误类型
#[derive(Debug)]
pub struct AIError {
    pub message: String,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AIError {}

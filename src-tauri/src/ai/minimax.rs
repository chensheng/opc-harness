//! MiniMax API Provider 实现
//! 
//! 支持 abab 系列模型的聊天和生成任务
//! MiniMax 开放平台：https://api.minimax.chat/

use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::ai::{AIError, Message, AIProviderType};

/// MiniMax 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniMaxConfig {
    pub api_key: String,
    pub group_id: String,
    pub base_url: Option<String>,
}

/// MiniMax 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniMaxMessage {
    pub sender_type: String,  // "USER" or "BOT"
    pub text: String,
}

/// MiniMax 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniMaxRequest {
    pub model: String,
    pub messages: Vec<MiniMaxMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// MiniMax 响应结构
#[derive(Debug, Clone, Deserialize)]
pub struct MiniMaxResponse {
    pub reply: String,
    pub usage: MiniMaxUsage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MiniMaxUsage {
    pub total_tokens: u32,
}

/// MiniMax Provider
pub struct MiniMaxProvider {
    config: MiniMaxConfig,
    client: Client,
}

impl MiniMaxProvider {
    /// 创建新的 MiniMax Provider
    pub fn new(config: MiniMaxConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
    
    /// 获取基础 URL
    fn get_base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or("https://api.minimax.chat/v1")
    }
    
    /// 构建认证头
    fn build_auth_header(&self) -> (String, String) {
        ("Authorization".to_string(), format!("Bearer {}", self.config.api_key))
    }
    
    /// 将通用 Message 转换为 MiniMaxMessage
    fn convert_messages(&self, messages: Vec<Message>) -> Vec<MiniMaxMessage> {
        messages.into_iter().map(|msg| {
            let sender_type = if msg.role == "user" {
                "USER".to_string()
            } else {
                "BOT".to_string()
            };
            MiniMaxMessage {
                sender_type,
                text: msg.content,
            }
        }).collect()
    }
    
    /// 非流式聊天
    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, AIError> {
        let minimax_messages = self.convert_messages(messages);
        
        let request = MiniMaxRequest {
            model: "abab6.5".to_string(),
            messages: minimax_messages,
            stream: false,
            temperature: Some(0.7),
            max_tokens: Some(2048),
        };
        
        let url = format!("{}/text/chat", self.get_base_url());
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(format!("请求失败：{}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::ApiError(format!(
                "API 请求失败 ({}): {}",
                status, error_text
            )));
        }
        
        let response_body: MiniMaxResponse = response
            .json()
            .await
            .map_err(|e| AIError::ParseError(format!("解析响应失败：{}", e)))?;
        
        Ok(response_body.reply)
    }
    
    /// 流式聊天（SSE）
    pub async fn stream_chat(
        &self,
        messages: Vec<Message>,
        mut handler: impl FnMut(String) -> Result<(), AIError>
    ) -> Result<String, AIError> {
        let minimax_messages = self.convert_messages(messages);
        
        let request = MiniMaxRequest {
            model: "abab6.5".to_string(),
            messages: minimax_messages,
            stream: true,
            temperature: Some(0.7),
            max_tokens: Some(2048),
        };
        
        let url = format!("{}/text/chat", self.get_base_url());
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(format!("请求失败：{}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::ApiError(format!(
                "API 请求失败 ({}): {}",
                status, error_text
            )));
        }
        
        // 处理 SSE 流
        let mut full_reply = String::new();
        let mut lines = response
            .text()
            .await
            .map_err(|e| AIError::ParseError(format!("读取流失败：{}", e)))?
            .lines()
            .peekable();
        
        while let Some(line) = lines.next() {
            if line.starts_with("data: ") {
                let data = &line[6..];
                if data.trim() == "[DONE]" {
                    break;
                }
                
                // 尝试解析 SSE 数据
                if let Ok(chunk) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(reply) = chunk.get("reply").and_then(|r| r.as_str()) {
                        full_reply.push_str(reply);
                        handler(reply.to_string())?;
                    }
                }
            }
        }
        
        Ok(full_reply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimax_provider_creation() {
        let config = MiniMaxConfig {
            api_key: "test_key".to_string(),
            group_id: "test_group".to_string(),
            base_url: None,
        };
        
        let provider = MiniMaxProvider::new(config);
        assert_eq!(provider.get_base_url(), "https://api.minimax.chat/v1");
    }
    
    #[test]
    fn test_minimax_message_conversion() {
        let config = MiniMaxConfig {
            api_key: "test_key".to_string(),
            group_id: "test_group".to_string(),
            base_url: None,
        };
        
        let provider = MiniMaxProvider::new(config);
        let messages = vec![
            Message { role: "user".to_string(), content: "你好".to_string() },
            Message { role: "assistant".to_string(), content: "你好！有什么可以帮助你的吗？".to_string() },
        ];
        
        let minimax_messages = provider.convert_messages(messages);
        
        assert_eq!(minimax_messages.len(), 2);
        assert_eq!(minimax_messages[0].sender_type, "USER");
        assert_eq!(minimax_messages[0].text, "你好");
        assert_eq!(minimax_messages[1].sender_type, "BOT");
    }
    
    #[test]
    fn test_minimax_request_serialization() {
        let request = MiniMaxRequest {
            model: "abab6.5".to_string(),
            messages: vec![
                MiniMaxMessage {
                    sender_type: "USER".to_string(),
                    text: "测试消息".to_string(),
                }
            ],
            stream: false,
            temperature: Some(0.7),
            max_tokens: Some(1024),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"abab6.5\""));
        assert!(json.contains("\"stream\":false"));
    }
}

//! AI Provider Core Implementation
//! 
//! 包含 AIProvider 核心结构体和基础方法

use reqwest::Client;
use tokio::process::Command;

use super::ai_types::*;

/// OpenAI API 响应结构（内部使用）
#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<OpenAIChoice>,
    pub usage: Option<OpenAIUsage>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIChoice {
    pub index: i32,
    pub message: OpenAIMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// OpenAI 流式响应块
#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIStreamChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIStreamChoice {
    pub index: i32,
    pub delta: OpenAIDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OpenAIDelta {
    pub role: Option<String>,
    pub content: Option<String>,
}

/// AI 提供商实现
pub struct AIProvider {
    provider_type: AIProviderType,
    api_key: String,
    pub(super) client: Client,
}

impl AIProvider {
    /// 创建新的 AI Provider
    pub fn new(provider_type: AIProviderType, api_key: String) -> Self {
        Self {
            provider_type,
            api_key,
            client: Client::new(),
        }
    }

    /// 获取 Base URL
    pub fn get_base_url(&self) -> String {
        match self.provider_type {
            AIProviderType::OpenAI => "https://api.openai.com/v1".to_string(),
            AIProviderType::Anthropic => "https://api.anthropic.com/v1".to_string(),
            AIProviderType::Kimi => "https://api.moonshot.cn/v1".to_string(),
            AIProviderType::GLM => "https://open.bigmodel.cn/api/paas/v4".to_string(),
            AIProviderType::MiniMax => "https://api.minimax.chat/v1".to_string(),
            AIProviderType::DeepL => "https://api-free.deepl.com/v2".to_string(),
            AIProviderType::CodeFree => "cli://codefree".to_string(), // CLI-based provider
        }
    }

    /// 获取认证头
    pub fn get_auth_header(&self) -> (String, String) {
        match self.provider_type {
            AIProviderType::Anthropic => ("x-api-key".to_string(), self.api_key.clone()),
            _ => (
                "Authorization".to_string(),
                format!("Bearer {}", self.api_key),
            ),
        }
    }

    /// 验证 API Key
    pub async fn validate_key(&self) -> Result<bool, AIError> {
        // CodeFree CLI 不需要 API Key，只需要检测 CLI 是否安装
        if self.provider_type == AIProviderType::CodeFree {
            return self.validate_codefree_cli().await;
        }
        
        // Simple validation - make a test request
        match self.provider_type {
            AIProviderType::OpenAI => {
                let response = self
                    .client
                    .get(format!("{}/models", self.get_base_url()))
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: e.to_string(),
                    })?;

                Ok(response.status().is_success())
            }
            AIProviderType::Kimi => {
                // Kimi (Moonshot) API validation - use a simple chat request
                let url = format!("{}/chat/completions", self.get_base_url());
                let body = serde_json::json!({
                    "model": "moonshot-v1-8k",
                    "messages": [
                        {"role": "user", "content": "Hi"}
                    ],
                    "max_tokens": 1
                });

                let response = self
                    .client
                    .post(&url)
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: format!("Kimi API 验证请求失败：{}", e),
                    })?;

                if response.status().is_success() {
                    Ok(true)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(AIError {
                        message: format!("Kimi API 返回错误 ({}): {}", status, text),
                    })
                }
            }
            AIProviderType::GLM => {
                // GLM (Zhipu) API validation
                let url = format!("{}/chat/completions", self.get_base_url());
                let body = serde_json::json!({
                    "model": "glm-4-flash",
                    "messages": [
                        {"role": "user", "content": "Hi"}
                    ],
                    "max_tokens": 1
                });

                let response = self
                    .client
                    .post(&url)
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: format!("GLM API 验证请求失败：{}", e),
                    })?;

                if response.status().is_success() {
                    Ok(true)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(AIError {
                        message: format!("GLM API 返回错误 ({}): {}", status, text),
                    })
                }
            }
            AIProviderType::Anthropic => {
                // Anthropic API validation
                let response = self
                    .client
                    .get(format!("{}/models", self.get_base_url()))
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .header("anthropic-version", "2023-06-01")
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: format!("Anthropic API 验证失败：{}", e),
                    })?;

                if response.status().is_success() {
                    Ok(true)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(AIError {
                        message: format!("Anthropic API 返回错误 ({}): {}", status, text),
                    })
                }
            }
            AIProviderType::MiniMax => {
                // MiniMax API validation
                let url = format!("{}/text/chat", self.get_base_url());
                let body = serde_json::json!({
                    "model": "abab6.5",
                    "messages": [
                        {"sender_type": "USER", "text": "Hi"}
                    ],
                    "max_tokens": 1
                });

                log::info!("Validating MiniMax API key...");
                log::debug!("MiniMax validation request URL: {}", url);
                log::debug!("MiniMax validation request body: {:?}", body);

                let response = self
                    .client
                    .post(&url)
                    .header(self.get_auth_header().0, self.get_auth_header().1)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| AIError {
                        message: format!("MiniMax API 验证请求失败：{}", e),
                    })?;

                if response.status().is_success() {
                    log::info!("MiniMax API validation successful!");
                    Ok(true)
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    log::error!("MiniMax API validation failed ({}): {}", status, error_text);
                    Err(AIError {
                        message: format!("MiniMax API 返回错误 ({}): {}", status, error_text),
                    })
                }
            }
            _ => {
                // For other providers, assume valid for now
                Ok(true)
            }
        }
    }

    /// 验证 CodeFree CLI 是否安装
    async fn validate_codefree_cli(&self) -> Result<bool, AIError> {
        log::info!("Validating CodeFree CLI installation...");
        
        #[cfg(windows)]
        let check_cmd = Command::new("where").arg("codefree").output().await;
        
        #[cfg(unix)]
        let check_cmd = Command::new("which").arg("codefree").output().await;
        
        match check_cmd {
            Ok(output) => {
                if output.status.success() {
                    // CLI 已安装，进一步检查版本
                    let version_cmd = Command::new("codefree")
                        .arg("--version")
                        .output()
                        .await;
                    
                    match version_cmd {
                        Ok(ver_output) => {
                            if ver_output.status.success() {
                                let version = String::from_utf8_lossy(&ver_output.stdout);
                                log::info!("CodeFree CLI is installed, version: {}", version.trim());
                                Ok(true)
                            } else {
                                Err(AIError {
                                    message: "CodeFree CLI 已安装但无法获取版本信息".to_string(),
                                })
                            }
                        }
                        Err(e) => Err(AIError {
                            message: format!("CodeFree CLI 版本检查失败：{}", e),
                        }),
                    }
                } else {
                    Err(AIError {
                        message: "CodeFree CLI 未安装。请运行 'npm install -g @codefree/cli' 进行安装".to_string(),
                    })
                }
            }
            Err(e) => Err(AIError {
                message: format!("CodeFree CLI 检测失败：{}", e),
            }),
        }
    }

    /// 发送聊天请求
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        match self.provider_type {
            AIProviderType::OpenAI => self.chat_openai(request).await,
            AIProviderType::Anthropic => self.chat_anthropic(request).await,
            AIProviderType::Kimi => self.chat_kimi(request).await,
            AIProviderType::GLM => self.chat_glm(request).await,
            AIProviderType::MiniMax => self.chat_minimax(request).await,
            AIProviderType::DeepL => self.chat_deepl(request).await,
            AIProviderType::CodeFree => self.chat_codefree(request).await,
        }
    }

    /// 流式聊天
    pub async fn stream_chat<F>(&self, request: ChatRequest, on_chunk: F) -> Result<String, AIError>
    where
        F: FnMut(String) -> Result<(), AIError>,
    {
        match self.provider_type {
            AIProviderType::OpenAI => self.stream_chat_openai(request, on_chunk).await,
            AIProviderType::Anthropic => self.stream_chat_anthropic(request, on_chunk).await,
            AIProviderType::Kimi => self.stream_chat_kimi(request, on_chunk).await,
            AIProviderType::GLM => self.stream_chat_glm(request, on_chunk).await,
            AIProviderType::MiniMax => self.stream_chat_minimax(request, on_chunk).await,
            AIProviderType::DeepL => self.stream_chat_deepl(request, on_chunk).await,
            AIProviderType::CodeFree => self.stream_chat_codefree(request, on_chunk).await,
        }
    }

    /// 获取提供商 ID
    pub fn provider_id(&self) -> &str {
        match self.provider_type {
            AIProviderType::OpenAI => "openai",
            AIProviderType::Anthropic => "anthropic",
            AIProviderType::Kimi => "kimi",
            AIProviderType::GLM => "glm",
            AIProviderType::MiniMax => "minimax",
            AIProviderType::DeepL => "deepl",
            AIProviderType::CodeFree => "codefree",
        }
    }
}

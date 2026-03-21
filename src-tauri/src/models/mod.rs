//! Data models

use serde::{Deserialize, Serialize};

/// Project model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub path: Option<String>,
}

/// Project status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Draft,
    Designing,
    Coding,
    Marketing,
    Completed,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// AI Provider configuration
/// VD-001: AI 厂商配置数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    /// AI 厂商标识 (openai/anthropic/kimi/glm)
    pub provider: String,
    
    /// API 密钥（可选，支持从系统 keychain 读取）
    pub api_key: Option<String>,
    
    /// API 基础 URL
    pub base_url: Option<String>,
    
    /// 使用的模型名称
    pub model: String,
    
    /// 是否启用该配置
    pub enabled: bool,
    
    /// 配置名称（用户自定义）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for AIProviderConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            api_key: None,
            base_url: None,
            model: "gpt-4o-mini".to_string(),
            enabled: false,
            name: None,
        }
    }
}

impl AIProviderConfig {
    /// 创建新的 AI 配置
    pub fn new(provider: &str, model: &str, base_url: Option<&str>) -> Self {
        Self {
            provider: provider.to_string(),
            model: model.to_string(),
            base_url: base_url.map(|s| s.to_string()),
            ..Default::default()
        }
    }
    
    /// 设置 API 密钥
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }
    
    /// 设置是否启用
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// 设置配置名称
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}
/// AI Provider metadata information
/// 用于在 UI 中显示厂商信息和默认配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderMeta {
    /// 厂商标识
    pub id: String,
    
    /// 厂商显示名称
    pub name: String,
    
    /// 官方文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    
    /// API 密钥申请链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_url: Option<String>,
    
    /// 默认 API 基础 URL
    pub default_base_url: String,
    
    /// 支持的模型列表
    pub supported_models: Vec<String>,
    
    /// 默认模型
    pub default_model: String,
    
    /// 是否支持流式输出
    pub supports_streaming: bool,
    
    /// 是否支持视觉功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_vision: Option<bool>,
}

impl AIProviderMeta {
    /// 获取所有预定义的 AI 厂商元数据
    pub fn get_all_providers() -> Vec<Self> {
        vec![
            Self {
                id: "openai".to_string(),
                name: "OpenAI".to_string(),
                doc_url: Some("https://platform.openai.com/docs".to_string()),
                api_key_url: Some("https://platform.openai.com/api-keys".to_string()),
                default_base_url: "https://api.openai.com/v1".to_string(),
                supported_models: vec![
                    "gpt-4o".to_string(),
                    "gpt-4o-mini".to_string(),
                    "gpt-4-turbo".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
                default_model: "gpt-4o-mini".to_string(),
                supports_streaming: true,
                supports_vision: Some(true),
            },
            Self {
                id: "anthropic".to_string(),
                name: "Anthropic".to_string(),
                doc_url: Some("https://docs.anthropic.com/claude/docs".to_string()),
                api_key_url: Some("https://console.anthropic.com/settings/keys".to_string()),
                default_base_url: "https://api.anthropic.com".to_string(),
                supported_models: vec![
                    "claude-3-5-sonnet-20241022".to_string(),
                    "claude-3-opus-20240229".to_string(),
                    "claude-3-haiku-20240307".to_string(),
                ],
                default_model: "claude-3-5-sonnet-20241022".to_string(),
                supports_streaming: true,
                supports_vision: Some(true),
            },
            Self {
                id: "kimi".to_string(),
                name: "Kimi".to_string(),
                doc_url: Some("https://platform.moonshot.cn/docs".to_string()),
                api_key_url: Some("https://platform.moonshot.cn/console/api-keys".to_string()),
                default_base_url: "https://api.moonshot.cn/v1".to_string(),
                supported_models: vec![
                    "kimi-k2".to_string(),
                    "kimi-k2-0711".to_string(),
                    "moonshot-v1-8k".to_string(),
                    "moonshot-v1-32k".to_string(),
                    "moonshot-v1-128k".to_string(),
                ],
                default_model: "moonshot-v1-8k".to_string(),
                supports_streaming: true,
                supports_vision: Some(false),
            },
            Self {
                id: "glm".to_string(),
                name: "智谱 GLM".to_string(),
                doc_url: Some("https://open.bigmodel.cn/dev/api".to_string()),
                api_key_url: Some("https://open.bigmodel.cn/usercenter/proj".to_string()),
                default_base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
                supported_models: vec![
                    "glm-4-plus".to_string(),
                    "glm-4-0520".to_string(),
                    "glm-4-air".to_string(),
                    "glm-4-flash".to_string(),
                ],
                default_model: "glm-4-flash".to_string(),
                supports_streaming: true,
                supports_vision: Some(true),
            },
        ]
    }
    
    /// 根据 ID 获取特定厂商的元数据
    pub fn get_provider(id: &str) -> Option<Self> {
        Self::get_all_providers().into_iter().find(|p| p.id == id)
    }
}

/// AI Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub max_tokens: Option<u32>,
}

/// PRD (Product Requirements Document)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRD {
    pub id: String,
    pub project_id: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// User persona
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPersona {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub age_range: String,
    pub occupation: String,
    pub goals: Vec<String>,
    pub pain_points: Vec<String>,
    pub behaviors: Vec<String>,
}

/// Competitor analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub url: Option<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub differentiation: String,
}

/// Tool info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

/// CLI Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLISession {
    pub id: String,
    pub tool: String,
    pub project_path: String,
    pub status: SessionStatus,
    pub started_at: i64,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Running,
    Stopped,
    Error,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

/// CLI Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIOutput {
    pub session_id: String,
    pub output_type: OutputType,
    pub content: String,
    pub timestamp: i64,
}

/// Output type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
    Stderr,
    System,
}

/// App settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: "zh-CN".to_string(),
            auto_save: true,
        }
    }
}

/// PRD document model
/// VD-021: PRD 文档数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrdDocument {
    /// PRD 唯一标识
    pub id: String,
    
    /// 关联的项目 ID
    pub project_id: String,
    
    /// PRD 内容（Markdown 格式）
    pub content: String,
    
    /// 版本号
    pub version: i32,
    
    /// 创建时间戳
    pub created_at: i64,
    
    /// 更新时间戳
    pub updated_at: i64,
}

impl PrdDocument {
    /// 创建新的 PRD 文档
    pub fn new(project_id: &str, content: &str) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            content: content.to_string(),
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 创建新版本
    pub fn new_version(&self, content: &str) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: self.id.clone(),
            project_id: self.project_id.clone(),
            content: content.to_string(),
            version: self.version + 1,
            created_at: self.created_at,
            updated_at: now,
        }
    }
}

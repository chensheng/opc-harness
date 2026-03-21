// 数据模型定义

use serde::{Deserialize, Serialize};

/// AI厂商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub provider: String,
    pub api_key: String,
    pub base_url: Option<String>,
    pub default_model: String,
    pub models: Vec<AIModel>,
}

/// AI模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub id: String,
    pub name: String,
    pub max_tokens: i32,
    pub supports_vision: bool,
    pub supports_streaming: bool,
}

/// 项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub local_path: String,
    pub default_ai_provider: Option<String>,
    pub default_cli_tool: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 项目状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Designing,
    Coding,
    Marketing,
    Completed,
}

/// PRD文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRD {
    pub id: String,
    pub project_id: String,
    pub content: String,
    pub version: i32,
    pub created_at: String,
}

/// 用户画像
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPersona {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub demographics: serde_json::Value,
    pub pain_points: Vec<String>,
    pub goals: Vec<String>,
    pub behaviors: Vec<String>,
}

/// CLI工具配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIToolConfig {
    pub tool_type: String,
    pub command: String,
    pub install_url: String,
    pub description: String,
    pub features: Vec<String>,
}

/// CLI会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLISession {
    pub id: String,
    pub project_id: String,
    pub tool_type: String,
    pub working_directory: String,
    pub status: CLISessionStatus,
    pub start_time: String,
    pub end_time: Option<String>,
}

/// CLI会话状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CLISessionStatus {
    Running,
    Completed,
    Error,
}

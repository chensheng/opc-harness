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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub enabled: bool,
}

impl Default for AIProviderConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            api_key: None,
            base_url: None,
            model: "gpt-4o-mini".to_string(),
            enabled: false,
        }
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

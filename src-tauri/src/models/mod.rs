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

/// AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub enabled: bool,
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

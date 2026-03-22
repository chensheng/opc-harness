use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub progress: i32,
    pub created_at: String,
    pub updated_at: String,
    pub idea: Option<String>,
    pub prd: Option<String>,
    pub user_personas: Option<String>,
    pub competitor_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLISession {
    pub id: String,
    pub tool_type: String,
    pub project_path: String,
    pub created_at: String,
}

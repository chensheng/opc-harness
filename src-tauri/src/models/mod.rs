use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub progress: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idea: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_personas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CLISession {
    pub id: String,
    pub tool_type: String,
    pub project_path: String,
    pub created_at: String,
}

use serde::{Deserialize, Serialize};

// ============================================================================
// API Key 管理相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateKeyRequest {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
}

/// 验证 API Key 的响应结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateKeyResponse {
    pub success: bool,
    pub cli_path: Option<String>, // CodeFree CLI 等工具的完整路径（如果找到）
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveApiKeyRequest {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiKeyRequest {
    pub provider: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiKeyRequest {
    pub provider: String,
}

// ============================================================================
// 聊天相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestPayload {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// ============================================================================
// PRD 生成相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePRDRequest {
    pub idea: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PRDResponse {
    pub title: String,
    pub overview: String,
    pub target_users: Vec<String>,
    pub core_features: Vec<String>,
    pub tech_stack: Vec<String>,
    pub estimated_effort: String,
    pub business_model: Option<String>,
    pub pricing: Option<String>,
}

// ============================================================================
// 用户画像相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPersonaResponse {
    pub id: String,
    pub name: String,
    pub age: String,
    pub occupation: String,
    pub background: String,
    pub goals: Vec<String>,
    pub pain_points: Vec<String>,
    pub behaviors: Vec<String>,
    pub quote: Option<String>,
}

// ============================================================================
// 竞品分析相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorResponse {
    pub name: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub market_share: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompetitorAnalysisResponse {
    pub competitors: Vec<CompetitorResponse>,
    pub differentiation: String,
    pub opportunities: Vec<String>,
}

// ============================================================================
// 营销策略相关类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingChannelResponse {
    pub name: String,
    pub platform: String,
    pub priority: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingTimelineItem {
    pub phase: String,
    pub duration: String,
    pub activities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingStrategyResponse {
    pub channels: Vec<MarketingChannelResponse>,
    pub timeline: Vec<MarketingTimelineItem>,
    pub key_messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingCopyResponse {
    pub platform: String,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
}

// ============================================================================
// Provider 信息类型
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub models: Vec<String>,
}

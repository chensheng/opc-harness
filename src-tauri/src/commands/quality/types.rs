use serde::{Deserialize, Serialize};

// ============================================================================
// PRD 质量检查相关类型
// ============================================================================

/// PRD 一致性检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPRDConsistencyRequest {
    /// PRD 内容（Markdown 格式）
    pub prd_content: String,
}

/// PRD 一致性检查响应
pub type CheckPRDConsistencyResponse = crate::quality::prd_consistency_checker::PRDConsistencyReport;

/// PRD 可行性评估请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessPRDFeasibilityRequest {
    /// PRD 内容（Markdown 格式）
    pub prd_content: String,
}

/// PRD 可行性评估响应
#[allow(dead_code)]
pub type AssessPRDFeasibilityResponse = crate::quality::prd_feasibility_assessor::PRDFeasibilityReport;

// ============================================================================
// PRD 迭代管理相关类型
// ============================================================================

pub use crate::quality::prd_iteration_manager::{
    IterationRequest, 
    IterationResponse,
    CreateInitialVersionRequest,
    CreateInitialVersionResponse,
    RollbackRequest,
    RollbackResponse,
};

// ============================================================================
// PRD 反馈处理相关类型
// ============================================================================

/// 提交反馈并重新生成请求
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitFeedbackRequest {
    /// PRD ID
    pub prd_id: String,
    /// PRD 内容
    pub prd_content: String,
    /// 用户反馈内容
    pub feedback_content: String,
    /// 反馈针对的章节（None 表示整体反馈）
    pub section: Option<String>,
    /// 当前迭代次数
    pub iteration_count: usize,
}

/// 提交反馈并重新生成响应
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitFeedbackResponse {
    /// 新的 PRD 内容
    pub new_prd_content: String,
    /// 变更的章节列表
    pub changed_sections: Vec<String>,
    /// 迭代前的质量评分
    pub quality_score_before: f64,
    /// 迭代后的质量评分
    pub quality_score_after: f64,
    /// 当前迭代轮次
    pub iteration_number: usize,
    /// 是否成功
    pub success: bool,
}

// ============================================================================
// PRD 深度分析相关类型
// ============================================================================

/// PRD 深度分析请求
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzePRDDepthRequest {
    /// PRD 内容（Markdown 格式）
    pub prd_content: String,
    /// AI API Key（可选）
    pub api_key: Option<String>,
}

/// PRD 深度分析响应
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzePRDDepthResponse {
    /// 是否成功
    pub success: bool,
    /// 分析结果
    pub analysis: crate::quality::prd_deep_analyzer::PrdAnalysis,
    /// 错误消息
    pub error_message: Option<String>,
}

// ============================================================================
// 任务分解相关类型
// ============================================================================

/// 任务分解请求
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeTasksRequest {
    /// PRD 分析结果（功能列表）
    pub analysis: crate::quality::prd_deep_analyzer::PrdAnalysis,
}

/// 任务分解响应
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeTasksResponse {
    /// 是否成功
    pub success: bool,
    /// 任务依赖图
    pub task_graph: crate::quality::task_decomposer::TaskDependencyGraph,
    /// 错误消息
    pub error_message: Option<String>,
}

// ============================================================================
// 用户故事拆分相关类型
// ============================================================================

/// 用户故事数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    /// 用户故事 ID
    pub id: String,
    /// 故事编号（如 US-001）
    pub story_number: String,
    /// 故事标题
    pub title: String,
    /// 角色（As a ...）
    pub role: String,
    /// 功能（I want ...）
    pub feature: String,
    /// 价值（So that ...）
    pub benefit: String,
    /// 详细描述
    pub description: String,
    /// 验收标准
    pub acceptance_criteria: Vec<String>,
    /// 优先级
    pub priority: String,
    /// 状态
    pub status: String,
    /// 估算的故事点
    pub story_points: Option<u32>,
    /// 依赖的故事 ID
    pub dependencies: Option<Vec<String>>,
    /// 关联的功能模块
    pub feature_module: Option<String>,
    /// 标签
    pub labels: Vec<String>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 用户故事拆分请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeUserStoriesRequest {
    /// PRD 内容或功能描述
    #[serde(alias = "prdContent")]
    pub prd_content: String,
    /// AI 提供商 (openai, anthropic, kimi, glm, minimax)
    #[serde(alias = "provider", default = "default_provider")]
    pub provider: String,
    /// AI 模型名称
    #[serde(alias = "model", default = "default_model")]
    pub model: String,
    /// 可选的 AI API Key
    #[serde(alias = "apiKey")]
    pub api_key: Option<String>,
}

pub fn default_provider() -> String {
    "openai".to_string()
}

pub fn default_model() -> String {
    "gpt-4-turbo-preview".to_string()
}

/// 用户故事拆分响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecomposeUserStoriesResponse {
    /// 是否成功
    pub success: bool,
    /// 拆分出的用户故事列表
    pub user_stories: Vec<UserStory>,
    /// 错误消息
    pub error_message: Option<String>,
}

// ============================================================================
// 用户故事持久化相关类型
// ============================================================================

/// 保存用户故事到数据库请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveUserStoriesRequest {
    /// 项目 ID
    pub project_id: String,
    /// 用户故事列表
    pub user_stories: Vec<UserStory>,
}

/// 保存用户故事响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveUserStoriesResponse {
    /// 是否成功
    pub success: bool,
    /// 保存的故事数量
    pub count: usize,
    /// 错误信息（如果有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 获取项目的用户故事请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserStoriesRequest {
    /// 项目 ID
    pub project_id: String,
}

/// 获取用户故事响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserStoriesResponse {
    /// 是否成功
    pub success: bool,
    /// 用户故事列表
    pub user_stories: Vec<UserStory>,
    /// 错误信息（如果有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

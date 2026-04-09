/// PRD 质量检查相关的 Tauri Commands

use crate::quality::prd_consistency_checker::{PRDConsistencyChecker, PRDDocument as ConsistencyPRDDocument};
use crate::quality::prd_feasibility_assessor::{PRDFeasibilityAssessor, PRDDocument as FeasibilityPRDDocument};
use crate::quality::prd_iteration_manager::{
    PRDIterationManager, IterationRequest, IterationResponse, 
    CreateInitialVersionRequest, CreateInitialVersionResponse,
    RollbackRequest, RollbackResponse,
};
use crate::quality::feedback_processor::{Feedback, RegenerateRequest};
use crate::quality::prd_checker::{PRDDocument as QualityPRDDocument, PRDQualityChecker, PRDQualityReport};
use crate::quality::prd_deep_analyzer::{PrdDeepAnalyzer, PrdAnalysis};
use crate::quality::task_decomposer::{TaskDecomposer, TaskDependencyGraph};
use serde::{Deserialize, Serialize};

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
pub type AssessPRDFeasibilityResponse = crate::quality::prd_feasibility_assessor::PRDFeasibilityReport;

/// 检查 PRD 一致性
#[tauri::command]
pub async fn check_prd_consistency(
    request: CheckPRDConsistencyRequest,
) -> Result<CheckPRDConsistencyResponse, String> {
    // 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_consistency_prd(&request.prd_content);
    
    // 创建检查器并执行检查
    let checker = PRDConsistencyChecker::new();
    let report = checker.check_consistency(&prd);
    
    Ok(report)
}

/// 检查 PRD 质量
#[tauri::command]
pub async fn check_prd_quality(prd_content: String) -> Result<PRDQualityReport, String> {
    // 1. 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_quality_prd(&prd_content);

    // 2. 创建质量检查器
    let checker = PRDQualityChecker::with_defaults();

    // 3. 执行质量检查
    let report = checker.check_quality(&prd);

    Ok(report)
}

/// 评估 PRD 可行性
#[tauri::command]
pub async fn assess_prd_feasibility(
    request: AssessPRDFeasibilityRequest,
) -> Result<AssessPRDFeasibilityResponse, String> {
    // 解析 Markdown 内容为 PRDDocument
    let prd = parse_markdown_to_feasibility_prd(&request.prd_content);
    
    // 创建评估器并执行评估
    let assessor = PRDFeasibilityAssessor::new();
    let report = assessor.assess_feasibility(&prd);
    
    Ok(report)
}

/// 将 Markdown 内容解析为一致性检查用的 PRDDocument
fn parse_markdown_to_consistency_prd(markdown: &str) -> ConsistencyPRDDocument {
    let mut title = None;
    let mut overview = None;
    let mut target_users: Option<Vec<String>> = None;
    let mut core_features: Option<Vec<String>> = None;
    let mut tech_stack: Option<Vec<String>> = None;
    let mut estimated_effort = None;

    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        
        // 解析标题 (# 开头)
        if line.starts_with('#') && !line.starts_with("##") {
            title = Some(line.trim_start_matches('#').trim().to_string());
        }
        
        // 解析章节
        if line.starts_with("## ") {
            let section_name = line.trim_start_matches("## ").trim().to_lowercase();
            i += 1;
            
            let mut content = String::new();
            while i < lines.len() && !lines[i].trim().starts_with("## ") && !lines[i].trim().starts_with('#') {
                if !content.is_empty() {
                    content.push('\n');
                }
                content.push_str(lines[i]);
                i += 1;
            }
            
            // 根据章节名提取内容
            match section_name.as_str() {
                s if s.contains("概述") || s.contains("overview") => {
                    overview = Some(content.trim().to_string());
                }
                s if s.contains("用户") || s.contains("target") => {
                    target_users = Some(extract_list_items(&content));
                }
                s if s.contains("功能") || s.contains("feature") => {
                    core_features = Some(extract_list_items(&content));
                }
                s if s.contains("技术") || s.contains("tech") => {
                    tech_stack = Some(extract_list_items(&content));
                }
                s if s.contains("工作量") || s.contains("effort") => {
                    estimated_effort = Some(content.trim().to_string());
                }
                _ => {}
            }
            continue;
        }
        
        i += 1;
    }

    ConsistencyPRDDocument {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
    }
}

/// 将 Markdown 内容解析为质量检查用的 PRDDocument
fn parse_markdown_to_quality_prd(content: &str) -> QualityPRDDocument {
    let mut title = None;
    let mut overview = None;
    let mut target_users = None;
    let mut core_features = None;
    let mut tech_stack = None;
    let mut estimated_effort = None;

    let lines: Vec<&str> = content.lines().collect();
    let mut current_section = String::new();
    let mut current_content = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // 检测章节标题 (# 开头)
        if trimmed.starts_with('#') {
            // 保存之前的章节内容
            if !current_section.is_empty() {
                save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
            }

            // 开始新章节
            current_section = trimmed.trim_start_matches('#').trim().to_lowercase();
            current_content.clear();
        } else if !trimmed.is_empty() {
            // 收集章节内容
            current_content.push(trimmed);
        }
    }

    // 保存最后一个章节
    if !current_section.is_empty() {
        save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
    }

    QualityPRDDocument {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
    }
}

/// 将 Markdown 内容解析为可行性评估用的 PRDDocument
fn parse_markdown_to_feasibility_prd(content: &str) -> FeasibilityPRDDocument {
    let mut title = None;
    let mut overview = None;
    let mut target_users = None;
    let mut core_features = None;
    let mut tech_stack = None;
    let mut estimated_effort = None;

    let lines: Vec<&str> = content.lines().collect();
    let mut current_section = String::new();
    let mut current_content = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // 检测章节标题 (# 开头)
        if trimmed.starts_with('#') {
            // 保存之前的章节内容
            if !current_section.is_empty() {
                save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
            }

            // 开始新章节
            current_section = trimmed.trim_start_matches('#').trim().to_lowercase();
            current_content.clear();
        } else if !trimmed.is_empty() {
            // 收集章节内容
            current_content.push(trimmed);
        }
    }

    // 保存最后一个章节
    if !current_section.is_empty() {
        save_section(&current_section, &current_content, &mut title, &mut overview, &mut target_users, &mut core_features, &mut tech_stack, &mut estimated_effort);
    }

    FeasibilityPRDDocument {
        title,
        overview,
        target_users,
        core_features,
        tech_stack,
        estimated_effort,
    }
}

/// 保存章节内容到对应的字段
fn save_section(
    section: &str,
    content: &[&str],
    title: &mut Option<String>,
    overview: &mut Option<String>,
    target_users: &mut Option<Vec<String>>,
    core_features: &mut Option<Vec<String>>,
    tech_stack: &mut Option<Vec<String>>,
    estimated_effort: &mut Option<String>,
) {
    let text = content.join("\n");

    match section {
        s if s.contains("产品标题") || s.contains("title") => {
            *title = Some(text);
        }
        s if s.contains("概述") || s.contains("overview") || s.contains("介绍") => {
            *overview = Some(text);
        }
        s if s.contains("用户") || s.contains("target") || s.contains("persona") => {
            *target_users = Some(parse_list_items(&text));
        }
        s if s.contains("功能") || s.contains("feature") || s.contains("需求") => {
            *core_features = Some(parse_list_items(&text));
        }
        s if s.contains("技术") || s.contains("tech") || s.contains("stack") => {
            *tech_stack = Some(parse_list_items(&text));
        }
        s if s.contains("工作量") || s.contains("effort") || s.contains("时间") => {
            *estimated_effort = Some(text);
        }
        _ => {}
    }
}

/// 解析列表项（支持 -, *, • 等标记）
fn parse_list_items(text: &str) -> Vec<String> {
    let mut items = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('•') {
            // 使用 chars() 方法安全地跳过第一个字符
            let item = trimmed.chars().skip(1).collect::<String>().trim().to_string();
            if !item.is_empty() {
                items.push(item);
            }
        } else if !trimmed.is_empty() && !trimmed.contains('#') {
            // 也收集没有列表标记的行
            items.push(trimmed.to_string());
        }
    }

    items
}

/// 从文本中提取列表项（- 或 * 开头）
fn extract_list_items(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('-') || trimmed.starts_with('*') {
                Some(trimmed[1..].trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_prd() {
        let markdown = r#"
# 产品标题
测试产品

# 产品概述
这是一个测试产品

# 目标用户
- 用户 A
- 用户 B

# 核心功能
- 功能 1
- 功能 2
- 功能 3

# 技术栈
- React
- Rust

# 预估工作量
2 周
"#;

        let prd = parse_markdown_to_quality_prd(markdown);
        assert_eq!(prd.title, Some("测试产品".to_string()));
        assert_eq!(prd.overview, Some("这是一个测试产品".to_string()));
        assert!(prd.target_users.is_some());
        assert_eq!(prd.target_users.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_list_items_basic() {
        let text = r#"
- 项目 1
- 项目 2
- 项目 3
"#;

        let items = parse_list_items(text);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "项目 1");
        assert_eq!(items[1], "项目 2");
    }

    #[test]
    fn test_parse_list_items_mixed() {
        let text = r#"
- 项目 1
普通文本
* 项目 2
• 项目 3
"#;

        let items = parse_list_items(text);
        assert_eq!(items.len(), 4);
    }
}

#[cfg(test)]
mod tests_quality_checker {
    use super::*;

    #[test]
    fn test_parse_simple_prd() {
        let markdown = r#"
# 产品标题
测试产品

# 产品概述
这是一个测试产品

# 目标用户
- 用户 A
- 用户 B

# 核心功能
- 功能 1
- 功能 2
- 功能 3

# 技术栈
- React
- Rust

# 预估工作量
2 周
"#;

        let prd = parse_markdown_to_quality_prd(markdown);
        assert_eq!(prd.title, Some("测试产品".to_string()));
        assert_eq!(prd.overview, Some("这是一个测试产品".to_string()));
        assert!(prd.target_users.is_some());
        assert_eq!(prd.target_users.as_ref().unwrap().len(), 2);
    }
}

// ==================== PRD Iteration Commands (US-053) ====================

/// 创建初始版本
#[tauri::command]
pub async fn create_initial_version(
    request: CreateInitialVersionRequest,
) -> Result<CreateInitialVersionResponse, String> {
    // 创建迭代管理器（简化实现，不使用全局状态）
    let mut manager = PRDIterationManager::new();
    let version_id = manager.create_initial_version(&request.prd_json);
    
    Ok(CreateInitialVersionResponse {
        version_id,
    })
}

/// 执行 PRD 迭代
#[tauri::command]
pub async fn iterate_prd(
    request: IterationRequest,
) -> Result<IterationResponse, String> {
    
    let mut manager = PRDIterationManager::new();
    manager.iterate_with_feedback(&request)
}

/// 获取迭代历史
// #[tauri::command]
// pub async fn get_iteration_history() -> Result<GetIterationHistoryResponse, String> {
//     // 简化实现：返回空历史
//     Ok(GetIterationHistoryResponse {
//         history: crate::quality::prd_iteration_manager::IterationHistory {
//             current_version_id: String::new(),
//             versions: Vec::new(),
//         },
//     })
// }

/// 回滚到指定版本
#[tauri::command]
pub async fn rollback_to_version(
    _request: RollbackRequest,  // 添加下划线前缀
) -> Result<RollbackResponse, String> {
    
    // let mut manager = PRDIterationManager::new();  // 已注释未使用的变量
    // 简化版本不支持回滚
    Err("简化版本不支持回滚".to_string())
}

// ==================== PRD Feedback Commands (US-053) ====================

/// 提交反馈并重新生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
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

/// 提交反馈并重新生成 PRD
#[tauri::command]
#[allow(dead_code)]
pub async fn submit_feedback_and_regenerate(
    request: SubmitFeedbackRequest,
) -> Result<SubmitFeedbackResponse, String> {
    use crate::quality::feedback_processor::PRDFeedbackProcessor;
    
    // 创建反馈处理器
    let processor = PRDFeedbackProcessor::new();
    
    // 解析用户反馈
    let parsed_feedback = processor.parse_feedback(&request.feedback_content)?;
    
    // 保存需要克隆的字段
    let sentiment = parsed_feedback.sentiment.clone();
    let priority = parsed_feedback.priority.clone();
    
    // 创建反馈对象
    let feedback = Feedback {
        id: format!("fb_{}", chrono::Utc::now().timestamp()),
        prd_id: request.prd_id.clone(),
        section: request.section.clone(),
        content: request.feedback_content.clone(),
        sentiment,
        priority,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    // 识别受影响的章节
    let prd_structure = crate::quality::feedback_processor::PRDStructure::default();
    let affected_sections = processor.identify_affected_sections(
        &parsed_feedback,
        &prd_structure,
    );
    
    // 构建再生成请求
    let regen_request = RegenerateRequest {
        prd_content: request.prd_content,
        feedbacks: vec![feedback],
        sections_to_regenerate: if let Some(section) = request.section {
            vec![section]
        } else {
            affected_sections
        },
        iteration_count: request.iteration_count,
    };
    
    // 执行再生成
    let result = processor.regenerate_with_feedback(&regen_request)?;
    
    Ok(SubmitFeedbackResponse {
        new_prd_content: result.new_prd_content,
        changed_sections: result.changed_sections,
        quality_score_before: result.quality_score_before,
        quality_score_after: result.quality_score_after,
        iteration_number: result.iteration_number,
        success: result.success,
    })
}

#[cfg(test)]
mod feedback_tests {
    use super::*;

    #[tokio::test]
    async fn test_submit_feedback_basic() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD\n\nContent here.".to_string(),
            feedback_content: "用户画像部分需要更详细".to_string(),
            section: Some("用户画像".to_string()),
            iteration_count: 0,
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.iteration_number == 1);
        assert!(response.quality_score_after > response.quality_score_before);
    }

    #[tokio::test]
    async fn test_submit_feedback_overall() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD".to_string(),
            feedback_content: "整体不错，但有些小问题".to_string(),
            section: None,
            iteration_count: 0,
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
    }

    #[tokio::test]
    async fn test_max_iteration_exceeded() {
        let request = SubmitFeedbackRequest {
            prd_id: "test_prd".to_string(),
            prd_content: "# Test PRD".to_string(),
            feedback_content: "继续改进".to_string(),
            section: None,
            iteration_count: 10, // 超过最大限制
        };
        
        let result = submit_feedback_and_regenerate(request).await;
        assert!(result.is_err());
    }
}

// ============================================================================
// US-031: PRD 深度分析命令
// ============================================================================

/// PRD 深度分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AnalyzePRDDepthRequest {
    /// PRD 内容（Markdown 格式）
    pub prd_content: String,
    /// AI API Key（可选）
    pub api_key: Option<String>,
}

/// PRD 深度分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AnalyzePRDDepthResponse {
    /// 是否成功
    pub success: bool,
    /// 分析结果
    pub analysis: PrdAnalysis,
    /// 错误消息
    pub error_message: Option<String>,
}

/// PRD 深度分析
#[tauri::command]
#[allow(dead_code)]
pub async fn analyze_prd_depth(request: AnalyzePRDDepthRequest) -> Result<AnalyzePRDDepthResponse, String> {
    log::info!("Starting PRD deep analysis...");
    
    let analyzer = PrdDeepAnalyzer::new();
    
    match analyzer.analyze(&request.prd_content).await {
        Ok(mut analysis) => {
            // 如果有 API key，使用 AI 进行更深度的分析
            if let Some(api_key) = request.api_key {
                if !api_key.is_empty() {
                    match analyzer.analyze_with_ai(&request.prd_content, &api_key).await {
                        Ok(ai_analysis) => {
                            analysis = ai_analysis;
                        }
                        Err(e) => {
                            log::warn!("AI analysis failed, using basic analysis: {}", e);
                        }
                    }
                }
            }
            
            log::info!("PRD deep analysis completed. Found {} features", analysis.estimates.total_features);
            
            Ok(AnalyzePRDDepthResponse {
                success: true,
                analysis,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("PRD deep analysis failed: {}", e);
            
            Ok(AnalyzePRDDepthResponse {
                success: false,
                analysis: PrdAnalysis::empty(),
                error_message: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests_prd_analysis {
    use super::*;

    #[tokio::test]
    async fn test_analyze_prd_depth_basic() {
        let request = AnalyzePRDDepthRequest {
            prd_content: "# PRD\n这是一个用户管理系统，需要数据分析和报告功能".to_string(),
            api_key: None,
        };
        
        let result = analyze_prd_depth(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.analysis.estimates.total_features > 0);
    }
}

// ============================================================================
// US-032: 任务分解命令
// ============================================================================

/// 任务分解请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DecomposeTasksRequest {
    /// PRD 分析结果（功能列表）
    pub analysis: PrdAnalysis,
}

/// 任务分解响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DecomposeTasksResponse {
    /// 是否成功
    pub success: bool,
    /// 任务依赖图
    pub task_graph: TaskDependencyGraph,
    /// 错误消息
    pub error_message: Option<String>,
}

/// 分解任务
#[tauri::command]
#[allow(dead_code)]
pub async fn decompose_tasks(request: DecomposeTasksRequest) -> Result<DecomposeTasksResponse, String> {
    log::info!("Starting task decomposition...");
    
    let decomposer = TaskDecomposer::new();
    
    match decomposer.decompose_features(&request.analysis.features).await {
        Ok(task_graph) => {
            log::info!("Task decomposition completed. Generated {} tasks", task_graph.statistics.total_tasks);
            
            Ok(DecomposeTasksResponse {
                success: true,
                task_graph,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("Task decomposition failed: {}", e);
            
            Ok(DecomposeTasksResponse {
                success: false,
                task_graph: TaskDependencyGraph::empty(),
                error_message: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests_us032 {
    use super::*;
    use crate::quality::prd_deep_analyzer::{Feature, FeatureType, PrdAnalysis, Estimates};

    #[tokio::test]
    async fn test_decompose_tasks_basic() {
        let analysis = PrdAnalysis {
            features: vec![
                Feature {
                    id: "F001".to_string(),
                    name: "用户管理".to_string(),
                    description: "用户 CRUD 操作".to_string(),
                    feature_type: FeatureType::Core,
                    complexity: 3,
                    estimated_hours: 6.0,
                    priority: 8,
                    dependencies: vec![],
                }
            ],
            dependencies: vec![],
            risks: vec![],
            estimates: Estimates {
                total_features: 1,
                core_features: 1,
                auxiliary_features: 0,
                enhanced_features: 0,
                average_complexity: 3.0,
                total_estimated_hours: 6.0,
                high_risks_count: 0,
            },
        };
        
        let request = DecomposeTasksRequest { analysis };
        let result = decompose_tasks(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(response.task_graph.statistics.total_tasks > 0);
    }
}

// ============================================================================
// US-XXX: 用户故事拆分命令
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
    pub prd_content: String,
    /// 可选的 AI API Key
    pub api_key: Option<String>,
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

/// 分解用户故事
#[tauri::command]
pub async fn decompose_user_stories(
    request: DecomposeUserStoriesRequest,
) -> Result<DecomposeUserStoriesResponse, String> {
    log::info!("Starting user story decomposition...");
    
    // TODO: 实现实际的 AI 驱动的用户故事拆分逻辑
    // 目前返回 Mock 数据用于前端开发和测试
    
    let mock_stories = generate_mock_user_stories(&request.prd_content);
    
    log::info!("User story decomposition completed. Generated {} stories", mock_stories.len());
    
    Ok(DecomposeUserStoriesResponse {
        success: true,
        user_stories: mock_stories,
        error_message: None,
    })
}

/// 生成 Mock 用户故事（临时实现）
/// 注意：这是一个临时的 Mock 实现，未来会被真实的 AI 调用替换
fn generate_mock_user_stories(prd_content: &str) -> Vec<UserStory> {
    use chrono::Utc;
    
    let now = Utc::now().to_rfc3339();
    
    // 根据 PRD 内容提取关键词，生成更相关的 Mock 数据
    let prd_lower = prd_content.to_lowercase();
    
    // 检测 PRD 中是否包含特定领域的关键词
    let is_task_management = prd_lower.contains("任务") || prd_lower.contains("task");
    let is_ecommerce = prd_lower.contains("电商") || prd_lower.contains("购物") || prd_lower.contains("商品");
    let is_social = prd_lower.contains("社交") || prd_lower.contains("社区") || prd_lower.contains("用户");
    
    // 根据领域生成不同的 Mock 故事
    if is_task_management {
        // 任务管理系统的用户故事
        vec![
            UserStory {
                id: "us-001".to_string(),
                story_number: "US-001".to_string(),
                title: "用户注册与登录".to_string(),
                role: "新用户".to_string(),
                feature: "能够通过邮箱或手机号注册账号并登录系统".to_string(),
                benefit: "我可以访问系统的核心功能并保存我的个人数据".to_string(),
                description: "实现完整的用户认证流程，包括注册、登录、密码重置等功能".to_string(),
                acceptance_criteria: vec![
                    "用户可以通过邮箱注册，收到验证邮件".to_string(),
                    "用户可以通过手机号注册，收到短信验证码".to_string(),
                    "登录成功后跳转到首页".to_string(),
                    "密码强度验证（至少8位，包含字母和数字）".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: None,
                feature_module: Some("用户认证".to_string()),
                labels: vec!["认证".to_string(), "核心功能".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-002".to_string(),
                story_number: "US-002".to_string(),
                title: "任务创建与管理".to_string(),
                role: "已登录用户".to_string(),
                feature: "能够创建、编辑、删除和查看我的任务".to_string(),
                benefit: "我可以有效地管理我的工作和待办事项".to_string(),
                description: "提供完整的任务 CRUD 操作，支持富文本编辑和附件上传".to_string(),
                acceptance_criteria: vec![
                    "用户可以创建新任务，设置标题、描述、优先级".to_string(),
                    "用户可以编辑已有任务的所有字段".to_string(),
                    "用户可以删除任务，删除前有确认提示".to_string(),
                    "任务列表支持分页和搜索".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(8),
                dependencies: Some(vec!["us-001".to_string()]),
                feature_module: Some("任务管理".to_string()),
                labels: vec!["任务".to_string(), "CRUD".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-003".to_string(),
                story_number: "US-003".to_string(),
                title: "任务分类与标签".to_string(),
                role: "活跃用户".to_string(),
                feature: "能够为任务添加分类和标签".to_string(),
                benefit: "我可以更好地组织和筛选我的任务".to_string(),
                description: "实现任务的分类体系和标签系统，支持多维度筛选".to_string(),
                acceptance_criteria: vec![
                    "用户可以创建自定义分类".to_string(),
                    "一个任务可以属于多个分类".to_string(),
                    "用户可以为任务添加多个标签".to_string(),
                    "支持按分类和标签筛选任务".to_string(),
                ],
                priority: "P1".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: Some(vec!["us-002".to_string()]),
                feature_module: Some("任务管理".to_string()),
                labels: vec!["分类".to_string(), "标签".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-004".to_string(),
                story_number: "US-004".to_string(),
                title: "任务统计报表".to_string(),
                role: "管理者".to_string(),
                feature: "能够查看任务完成情况的统计报表".to_string(),
                benefit: "我可以了解团队的工作效率和进度".to_string(),
                description: "提供可视化的统计图表，展示任务完成率、趋势分析等".to_string(),
                acceptance_criteria: vec![
                    "显示任务总数、已完成、进行中的数量".to_string(),
                    "提供按日/周/月的完成率趋势图".to_string(),
                    "支持导出报表为 PDF 或 Excel".to_string(),
                    "报表数据实时更新".to_string(),
                ],
                priority: "P2".to_string(),
                status: "draft".to_string(),
                story_points: Some(8),
                dependencies: Some(vec!["us-002".to_string()]),
                feature_module: Some("统计分析".to_string()),
                labels: vec!["报表".to_string(), "可视化".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        ]
    } else if is_ecommerce {
        // 电商系统的用户故事
        vec![
            UserStory {
                id: "us-001".to_string(),
                story_number: "US-001".to_string(),
                title: "商品浏览与搜索".to_string(),
                role: "购物用户".to_string(),
                feature: "能够浏览商品列表并通过关键词搜索商品".to_string(),
                benefit: "我可以快速找到想要购买的商品".to_string(),
                description: "实现商品展示、分类浏览和搜索功能".to_string(),
                acceptance_criteria: vec![
                    "商品列表支持分页加载".to_string(),
                    "支持按价格、销量、评分排序".to_string(),
                    "搜索支持模糊匹配和关键词高亮".to_string(),
                    "搜索结果可以筛选和排序".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(8),
                dependencies: None,
                feature_module: Some("商品中心".to_string()),
                labels: vec!["商品".to_string(), "搜索".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-002".to_string(),
                story_number: "US-002".to_string(),
                title: "购物车管理".to_string(),
                role: "购物用户".to_string(),
                feature: "能够将商品添加到购物车并管理购物车内容".to_string(),
                benefit: "我可以集中管理想要购买的商品并进行批量结算".to_string(),
                description: "实现购物车的增删改查功能".to_string(),
                acceptance_criteria: vec![
                    "用户可以将商品添加到购物车".to_string(),
                    "可以修改商品数量和规格".to_string(),
                    "可以删除购物车中的商品".to_string(),
                    "购物车实时显示总价".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: Some(vec!["us-001".to_string()]),
                feature_module: Some("购物车".to_string()),
                labels: vec!["购物车".to_string(), "核心功能".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-003".to_string(),
                story_number: "US-003".to_string(),
                title: "订单创建与支付".to_string(),
                role: "购物用户".to_string(),
                feature: "能够从购物车创建订单并完成支付".to_string(),
                benefit: "我可以顺利完成购买流程".to_string(),
                description: "实现订单创建、地址选择、支付方式选择和支付流程".to_string(),
                acceptance_criteria: vec![
                    "用户可以选择收货地址".to_string(),
                    "支持多种支付方式（微信、支付宝、银行卡）".to_string(),
                    "支付成功后生成订单号".to_string(),
                    "支持订单状态追踪".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(13),
                dependencies: Some(vec!["us-002".to_string()]),
                feature_module: Some("订单系统".to_string()),
                labels: vec!["订单".to_string(), "支付".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        ]
    } else if is_social {
        // 社交系统的用户故事
        vec![
            UserStory {
                id: "us-001".to_string(),
                story_number: "US-001".to_string(),
                title: "用户个人资料管理".to_string(),
                role: "注册用户".to_string(),
                feature: "能够编辑和完善我的个人资料".to_string(),
                benefit: "其他用户可以更好地了解我".to_string(),
                description: "实现用户头像、昵称、简介等资料的编辑功能".to_string(),
                acceptance_criteria: vec![
                    "用户可以上传和更换头像".to_string(),
                    "可以编辑昵称和个人简介".to_string(),
                    "支持添加兴趣爱好标签".to_string(),
                    "资料修改后实时生效".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: None,
                feature_module: Some("用户中心".to_string()),
                labels: vec!["资料".to_string(), "核心功能".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-002".to_string(),
                story_number: "US-002".to_string(),
                title: "动态发布与互动".to_string(),
                role: "活跃用户".to_string(),
                feature: "能够发布动态并对他人动态进行点赞评论".to_string(),
                benefit: "我可以分享生活并与朋友互动".to_string(),
                description: "实现动态的发布、浏览、点赞、评论功能".to_string(),
                acceptance_criteria: vec![
                    "用户可以发布文字和图片动态".to_string(),
                    "动态支持@好友和添加话题标签".to_string(),
                    "可以对动态进行点赞和取消点赞".to_string(),
                    "可以发表评论并回复他人评论".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(8),
                dependencies: Some(vec!["us-001".to_string()]),
                feature_module: Some("动态系统".to_string()),
                labels: vec!["动态".to_string(), "互动".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-003".to_string(),
                story_number: "US-003".to_string(),
                title: "好友关系管理".to_string(),
                role: "社交用户".to_string(),
                feature: "能够添加好友并管理好友列表".to_string(),
                benefit: "我可以与感兴趣的人建立联系".to_string(),
                description: "实现好友申请、接受、拒绝和移除功能".to_string(),
                acceptance_criteria: vec![
                    "用户可以搜索并发送好友申请".to_string(),
                    "可以接受或拒绝好友申请".to_string(),
                    "可以查看好友列表和共同好友".to_string(),
                    "可以移除好友关系".to_string(),
                ],
                priority: "P1".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: Some(vec!["us-001".to_string()]),
                feature_module: Some("关系链".to_string()),
                labels: vec!["好友".to_string(), "社交".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        ]
    } else {
        // 通用默认故事（当无法识别领域时）
        vec![
            UserStory {
                id: "us-001".to_string(),
                story_number: "US-001".to_string(),
                title: "用户认证系统".to_string(),
                role: "新用户".to_string(),
                feature: "能够注册账号并安全登录系统".to_string(),
                benefit: "我可以访问系统的个性化功能".to_string(),
                description: "实现基础的用户认证功能".to_string(),
                acceptance_criteria: vec![
                    "支持邮箱注册和登录".to_string(),
                    "密码加密存储".to_string(),
                    "登录状态持久化".to_string(),
                    "支持忘记密码找回".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: None,
                feature_module: Some("用户系统".to_string()),
                labels: vec!["认证".to_string(), "核心功能".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-002".to_string(),
                story_number: "US-002".to_string(),
                title: "核心业务功能".to_string(),
                role: "已登录用户".to_string(),
                feature: "能够使用系统的主要业务功能".to_string(),
                benefit: "我可以完成我的主要工作目标".to_string(),
                description: "实现系统的核心业务流程".to_string(),
                acceptance_criteria: vec![
                    "核心功能可用且稳定".to_string(),
                    "操作流程清晰易懂".to_string(),
                    "有适当的错误提示".to_string(),
                    "支持基本的数据持久化".to_string(),
                ],
                priority: "P0".to_string(),
                status: "draft".to_string(),
                story_points: Some(8),
                dependencies: Some(vec!["us-001".to_string()]),
                feature_module: Some("核心业务".to_string()),
                labels: vec!["核心功能".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            UserStory {
                id: "us-003".to_string(),
                story_number: "US-003".to_string(),
                title: "数据统计与展示".to_string(),
                role: "管理者".to_string(),
                feature: "能够查看系统使用数据和业务统计".to_string(),
                benefit: "我可以了解系统运行状况和业务趋势".to_string(),
                description: "实现数据统计和可视化展示功能".to_string(),
                acceptance_criteria: vec![
                    "关键指标实时展示".to_string(),
                    "支持历史数据查询".to_string(),
                    "图表清晰易读".to_string(),
                    "支持数据导出".to_string(),
                ],
                priority: "P1".to_string(),
                status: "draft".to_string(),
                story_points: Some(5),
                dependencies: Some(vec!["us-002".to_string()]),
                feature_module: Some("数据分析".to_string()),
                labels: vec!["统计".to_string(), "可视化".to_string()],
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        ]
    }
}

#[cfg(test)]
mod tests_user_story_decomposition {
    use super::*;

    #[tokio::test]
    async fn test_decompose_user_stories_basic() {
        let request = DecomposeUserStoriesRequest {
            prd_content: "我们需要一个任务管理系统".to_string(),
            api_key: None,
        };
        
        let result = decompose_user_stories(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(!response.user_stories.is_empty());
        
        // 验证第一个故事的结构
        let first_story = &response.user_stories[0];
        assert!(!first_story.id.is_empty());
        assert!(!first_story.story_number.is_empty());
        assert!(!first_story.title.is_empty());
        assert!(!first_story.role.is_empty());
        assert!(!first_story.feature.is_empty());
        assert!(!first_story.benefit.is_empty());
        assert!(!first_story.acceptance_criteria.is_empty());
    }
}

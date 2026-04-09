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
use tauri::Emitter;  // 用于事件发射

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

fn default_provider() -> String {
    "openai".to_string()
}

fn default_model() -> String {
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

/// 分解用户故事
#[tauri::command]
pub async fn decompose_user_stories(
    request: DecomposeUserStoriesRequest,
) -> Result<DecomposeUserStoriesResponse, String> {
    log::info!("Starting user story decomposition with AI (provider: {}, model: {})...", 
               request.provider, request.model);
    
    // 使用 AI 进行用户故事拆分
    match decompose_with_ai(&request.prd_content, &request.provider, &request.model, request.api_key.as_deref()).await {
        Ok(user_stories) => {
            log::info!("User story decomposition completed. Generated {} stories", user_stories.len());
            
            Ok(DecomposeUserStoriesResponse {
                success: true,
                user_stories,
                error_message: None,
            })
        }
        Err(e) => {
            log::error!("User story decomposition failed: {}", e);
            
            // 降级策略：如果 AI 调用失败，返回错误信息
            Ok(DecomposeUserStoriesResponse {
                success: false,
                user_stories: vec![],
                error_message: Some(format!("AI 拆分失败：{}", e)),
            })
        }
    }
}

/// 分解用户故事(流式版本)
#[tauri::command]
pub async fn decompose_user_stories_streaming(
    request: DecomposeUserStoriesRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use uuid::Uuid;
    
    let session_id = Uuid::new_v4().to_string();
    
    log::info!("Starting streaming user story decomposition (provider: {}, model: {})", 
               request.provider, request.model);
    
    // 1. 创建 AI Provider
    let provider_type = match request.provider.as_str() {
        "openai" => crate::ai::AIProviderType::OpenAI,
        "anthropic" => crate::ai::AIProviderType::Anthropic,
        "kimi" => crate::ai::AIProviderType::Kimi,
        "glm" => crate::ai::AIProviderType::GLM,
        "minimax" => crate::ai::AIProviderType::MiniMax,
        _ => return Err(format!("不支持的 AI 提供商：{}", request.provider)),
    };
    
    let api_key = request.api_key.ok_or_else(|| "未提供 API Key".to_string())?;
    let provider = crate::ai::AIProvider::new(provider_type, api_key);
    
    // 2. 生成提示词
    let prompt = crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt(&request.prd_content);
    
    // 3. 构建聊天请求（流式模式）
    let chat_request = crate::ai::ChatRequest {
        model: request.model,
        messages: vec![
            crate::ai::Message {
                role: "system".to_string(),
                content: "你是一位经验丰富的敏捷开发专家和产品经理。请严格按照要求的 Markdown 表格格式输出用户故事列表。".to_string(),
            },
            crate::ai::Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: true,
    };
    
    // 4. 创建会话感知的 chunk 处理器
    let session_id_clone = session_id.clone();
    let app_clone = app.clone();
    
    let chunk_handler = move |chunk: String| -> Result<(), crate::ai::AIError> {
        let stream_chunk = crate::ai::StreamChunk {
            session_id: session_id_clone.clone(),
            content: chunk.clone(),
            is_complete: false,
        };
        
        // 发送用户故事流式 chunk 事件
        app_clone
            .emit("user-story-stream-chunk", stream_chunk)
            .map_err(|e| crate::ai::AIError {
                message: e.to_string(),
            })?;
        
        Ok(())
    };
    
    // 5. 执行流式请求
    match provider.stream_chat(chat_request, chunk_handler).await {
        Ok(final_content) => {
            // 发送完成事件
            let complete_data = crate::ai::StreamComplete {
                session_id: session_id.clone(),
                content: final_content.clone(),
            };
            let _ = app.emit("user-story-stream-complete", complete_data);
            
            log::info!("Streaming user story decomposition completed");
            Ok(final_content)
        }
        Err(e) => {
            // 发送错误事件
            let error_data = crate::ai::StreamError {
                session_id: session_id.clone(),
                error: e.to_string(),
            };
            let _ = app.emit("user-story-stream-error", error_data);
            
            log::error!("Streaming user story decomposition failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// 使用 AI 进行用户故事拆分
async fn decompose_with_ai(prd_content: &str, provider: &str, model: &str, api_key: Option<&str>) -> Result<Vec<UserStory>, String> {
    use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message};
    use crate::prompts::user_story_decomposition::generate_user_story_decomposition_prompt;
    use chrono::Utc;
    
    // 获取 API Key - 支持多种环境变量名
    let api_key = api_key
        .map(|k| k.to_string())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
        .or_else(|| std::env::var("MOONSHOT_API_KEY").ok())
        .or_else(|| std::env::var("ZHIPU_API_KEY").ok())
        .or_else(|| std::env::var("KIMI_API_KEY").ok())
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .ok_or_else(|| {
            "未提供 API Key，请在参数中传入或设置以下任一环境变量：\n\
             - OPENAI_API_KEY\n\
             - ANTHROPIC_API_KEY\n\
             - MOONSHOT_API_KEY (Kimi)\n\
             - ZHIPU_API_KEY (GLM)\n\
             - KIMI_API_KEY\n\
             - GLM_API_KEY"
                .to_string()
        })?;
    
    // 生成提示词
    let prompt = generate_user_story_decomposition_prompt(prd_content);
    
    log::info!("Calling AI service for user story decomposition...");
    log::debug!("Prompt length: {} characters", prompt.len());
    
    // 根据 provider 字符串创建对应的 AI Provider
    let provider_type = match provider {
        "openai" => AIProviderType::OpenAI,
        "anthropic" => AIProviderType::Anthropic,
        "kimi" => AIProviderType::Kimi,
        "glm" => AIProviderType::GLM,
        "minimax" => AIProviderType::MiniMax,
        _ => {
            return Err(format!("不支持的 AI 提供商：{}", provider));
        }
    };
    
    // 创建 AI Provider
    let ai_provider = AIProvider::new(provider_type, api_key);
    
    // 构建聊天请求
    let chat_request = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一位经验丰富的敏捷开发专家和产品经理。请严格按照要求的 JSON 格式输出用户故事列表，不要添加任何额外的解释或说明。".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(4096),
        stream: false,
    };
    
    // 调用 AI 服务
    let response = ai_provider.chat(chat_request).await
        .map_err(|e| format!("AI 服务调用失败：{}", e.message))?;
    
    log::info!("AI response received, length: {} characters", response.content.len());
    
    // 检查响应是否有效
    let trimmed_content = response.content.trim();
    if trimmed_content.is_empty() {
        return Err("AI 返回了空响应".to_string());
    }
    
    // 检测异常响应模式
    if let Some(error_msg) = detect_abnormal_response(trimmed_content) {
        log::error!("检测到异常AI响应: {}", error_msg);
        log::error!("响应预览(前500字符): {}", 
                    if trimmed_content.len() > 500 { &trimmed_content[..500] } else { trimmed_content });
        return Err(format!(
            "AI 返回了无效响应：{}\n\n\
             响应预览：{}\n\n\
             建议解决方案：\n\
             1. 检查 PRD 内容是否过长（建议精简到3000字以内）\n\
             2. 尝试切换到其他 AI 提供商（如 OpenAI GPT-4）\n\
             3. 检查 API Key 是否有足够配额且未过期\n\
             4. 确认使用的模型支持此任务（Kimi for Coding 可能不适合）\n\
             5. 简化 PRD 内容，只保留核心功能需求",
            error_msg,
            if trimmed_content.len() > 200 { &trimmed_content[..200] } else { trimmed_content }
        ));
    }
    
    log::debug!("AI response preview (first 300 chars): {}", 
                if response.content.len() > 300 { 
                    &response.content[..300] 
                } else { 
                    &response.content 
                });
    
    // 解析 AI 响应中的 JSON
    let user_stories = parse_ai_response_to_user_stories(&response.content)?;
    
    // 补充时间戳和状态
    let now = Utc::now().to_rfc3339();
    let mut stories_with_metadata: Vec<UserStory> = user_stories.into_iter().map(|mut story| {
        story.created_at = now.clone();
        story.updated_at = now.clone();
        if story.status.is_empty() {
            story.status = "draft".to_string();
        }
        story
    }).collect();
    
    // 确保故事编号连续
    for (index, story) in stories_with_metadata.iter_mut().enumerate() {
        story.story_number = format!("US-{:03}", index + 1);
        story.id = format!("us-{:03}", index + 1);
    }
    
    Ok(stories_with_metadata)
}

/// 解析 AI 响应为用户故事列表
fn parse_ai_response_to_user_stories(response: &str) -> Result<Vec<UserStory>, String> {
    log::info!("Parsing AI response to user stories...");
    
    // 尝试解析 Markdown 表格格式
    match parse_markdown_table_to_user_stories(response) {
        Ok(stories) => {
            if !stories.is_empty() {
                log::info!("Successfully parsed {} user stories from Markdown table", stories.len());
                return Ok(stories);
            }
        }
        Err(e) => {
            log::warn!("Markdown table parsing failed: {}", e);
        }
    }
    
    // 如果表格解析失败,尝试 JSON 格式(向后兼容)
    log::info!("Attempting JSON format as fallback...");
    use serde_json::Value;
    
    match extract_json_array(response) {
        Ok(json_str) => {
            let parsed: Value = serde_json::from_str(&json_str)
                .map_err(|e| format!("JSON 解析失败：{}", e))?;
            
            let stories_array = parsed.as_array()
                .ok_or_else(|| "AI 响应不是有效的 JSON 数组".to_string())?;
            
            let mut user_stories = Vec::new();
            for (index, story_value) in stories_array.iter().enumerate() {
                match parse_single_user_story(story_value, index) {
                    Ok(story) => user_stories.push(story),
                    Err(e) => {
                        log::warn!("跳过无效的用户故事 #{}: {}", index + 1, e);
                    }
                }
            }
            
            if !user_stories.is_empty() {
                log::info!("Successfully parsed {} user stories from JSON", user_stories.len());
                return Ok(user_stories);
            }
        }
        Err(e) => {
            log::warn!("JSON extraction failed: {}", e);
        }
    }
    
    // 所有策略都失败
    Err(format!(
        "无法解析 AI 响应。期望 Markdown 表格或 JSON 格式。\n\n\
         AI 响应预览（前300字符）：\n{}",
        if response.len() > 300 { &response[..300] } else { response }
    ))
}

/// 解析 Markdown 表格格式为用户故事列表
fn parse_markdown_table_to_user_stories(response: &str) -> Result<Vec<UserStory>, String> {
    // 查找 Markdown 表格
    let lines: Vec<&str> = response.lines().collect();
    
    // 寻找表格分隔行(包含 |---| 或 |-|-| 的行)
    let mut table_start = None;
    for (i, line) in lines.iter().enumerate() {
        if line.contains('|') && line.contains("---") {
            table_start = Some(i);
            break;
        }
    }
    
    let table_start = table_start.ok_or("未找到 Markdown 表格")?;
    
    // 提取表头和数据行
    if table_start == 0 {
        return Err("表格缺少表头".to_string());
    }
    
    let header_line = lines[table_start - 1];
    let data_lines: Vec<&str> = lines[table_start + 1..].iter()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with('|') && !trimmed.contains("---") && trimmed.len() > 2
        })
        .cloned()
        .collect();
    
    if data_lines.is_empty() {
        return Err("表格没有数据行".to_string());
    }
    
    log::info!("Found Markdown table with {} data rows", data_lines.len());
    
    // 解析表头
    let headers = parse_table_row(header_line);
    log::debug!("Table headers: {:?}", headers);
    
    // 解析每一行数据
    let mut user_stories = Vec::new();
    for (index, data_line) in data_lines.iter().enumerate() {
        let cells = parse_table_row(data_line);
        
        if cells.len() != headers.len() {
            log::warn!("跳过行 #{}: 列数不匹配 (期望 {}, 实际 {})", index + 1, headers.len(), cells.len());
            continue;
        }
        
        // 将表格行转换为 UserStory
        match convert_table_row_to_story(&headers, &cells, index) {
            Ok(story) => {
                log::debug!("Successfully parsed story #{}: {}", index + 1, story.title);
                user_stories.push(story);
            },
            Err(e) => {
                log::warn!("转换行 #{} 失败: {}", index + 1, e);
            }
        }
    }
    
    if user_stories.is_empty() {
        return Err("从表格中未能解析出任何有效的用户故事".to_string());
    }
    
    Ok(user_stories)
}

/// 解析表格行为单元格数组
fn parse_table_row(line: &str) -> Vec<String> {
    line.split('|')
        .skip(1)  // 跳过第一个空单元格
        .map(|cell| cell.trim().to_string())
        .filter(|cell| !cell.is_empty())
        .collect()
}

/// 将表格行转换为 UserStory
fn convert_table_row_to_story(headers: &[String], cells: &[String], index: usize) -> Result<UserStory, String> {
    // 创建字段映射(不区分大小写)
    let field_map: std::collections::HashMap<String, String> = headers.iter()
        .zip(cells.iter())
        .map(|(h, c)| (h.to_lowercase().trim().to_string(), c.clone()))
        .collect();
    
    // 提取字段值,提供默认值
    let story_number = field_map.get("序号")
        .or_else(|| field_map.get("story_number"))
        .or_else(|| field_map.get("编号"))
        .cloned()
        .unwrap_or_else(|| format!("US-{:03}", index + 1));
    
    let title = field_map.get("标题")
        .or_else(|| field_map.get("title"))
        .cloned()
        .unwrap_or_else(|| format!("用户故事 #{}", index + 1));
    
    let role = field_map.get("角色")
        .or_else(|| field_map.get("role"))
        .cloned()
        .unwrap_or_else(|| "用户".to_string());
    
    let feature = field_map.get("功能")
        .or_else(|| field_map.get("feature"))
        .cloned()
        .unwrap_or_default();
    
    let benefit = field_map.get("价值")
        .or_else(|| field_map.get("benefit"))
        .cloned()
        .unwrap_or_default();
    
    let priority = field_map.get("优先级")
        .or_else(|| field_map.get("priority"))
        .cloned()
        .unwrap_or_else(|| "P1".to_string());
    
    let story_points_str = field_map.get("故事点")
        .or_else(|| field_map.get("story_points"))
        .cloned()
        .unwrap_or_else(|| "5".to_string());
    
    let story_points = story_points_str.parse::<u32>().unwrap_or(5);
    
    let acceptance_criteria_str = field_map.get("验收标准")
        .or_else(|| field_map.get("acceptance_criteria"))
        .cloned()
        .unwrap_or_default();
    
    // 分号分隔的验收标准
    let acceptance_criteria: Vec<String> = acceptance_criteria_str
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let feature_module = field_map.get("模块")
        .or_else(|| field_map.get("feature_module"))
        .or_else(|| field_map.get("module"))
        .cloned()
        .unwrap_or_else(|| "通用".to_string());
    
    let labels_str = field_map.get("标签")
        .or_else(|| field_map.get("labels"))
        .cloned()
        .unwrap_or_default();
    
    // 逗号分隔的标签
    let labels: Vec<String> = labels_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let dependencies_str = field_map.get("依赖")
        .or_else(|| field_map.get("dependencies"))
        .cloned()
        .unwrap_or_else(|| "无".to_string());
    
    // 解析依赖关系
    let dependencies = if dependencies_str == "无" || dependencies_str.is_empty() {
        None
    } else {
        let deps: Vec<String> = dependencies_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty() && s != "无")
            .collect();
        if deps.is_empty() { None } else { Some(deps) }
    };
    
    // 构建描述
    let description = format!("作为{},我想要{},以便{}", role, feature, benefit);
    
    Ok(UserStory {
        id: story_number.to_lowercase().replace("us-", "us-"),
        story_number,
        title,
        role,
        feature,
        benefit,
        description,
        acceptance_criteria,
        priority: validate_priority(&priority),
        status: "draft".to_string(),
        story_points: Some(story_points),
        dependencies,
        feature_module: Some(feature_module),
        labels,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    })
}

/// 验证并规范化优先级
fn validate_priority(priority: &str) -> String {
    match priority.to_uppercase().as_str() {
        "P0" => "P0".to_string(),
        "P1" => "P1".to_string(),
        "P2" => "P2".to_string(),
        "P3" => "P3".to_string(),
        _ => "P1".to_string(),  // 默认 P1
    }
}

/// 检测异常的AI响应模式
fn detect_abnormal_response(content: &str) -> Option<String> {
    // 检查1: 空响应
    if content.is_empty() {
        return Some("响应为空".to_string());
    }
    
    // 检查2: 统计唯一字符数
    let unique_chars: std::collections::HashSet<char> = content.chars().collect();
    
    // 如果唯一字符极少（<=3个），可能是异常响应
    if unique_chars.len() <= 3 {
        let chars_vec: Vec<char> = unique_chars.iter().cloned().collect();
        
        // 全下划线
        if chars_vec.contains(&'_') && chars_vec.len() == 1 {
            return Some("响应只包含下划线字符，AI可能遇到错误或限制".to_string());
        }
        
        // 全横线
        if chars_vec.contains(&'-') && chars_vec.len() == 1 {
            return Some("响应只包含横线字符，AI可能遇到错误或限制".to_string());
        }
        
        // 全空格
        if chars_vec.contains(&' ') && chars_vec.len() == 1 {
            return Some("响应只包含空格".to_string());
        }
        
        // 重复的Base64-like模式 (如 7A7A7A...)
        if is_repetitive_pattern(content, 2) {
            return Some("响应包含重复的编码模式，可能是Base64数据损坏或API错误".to_string());
        }
    }
    
    // 检查3: 检测Base64编码特征（大量字母数字混合，无正常文本结构）
    if looks_like_corrupted_base64(content) {
        return Some("响应看起来像损坏的Base64编码数据，API可能返回了二进制内容".to_string());
    }
    
    // 检查4: 检测HTML错误页面
    if content.starts_with("<!DOCTYPE") || content.starts_with("<html") {
        return Some("响应是HTML页面，可能是API认证失败或服务不可用".to_string());
    }
    
    // 检查5: 检测JSON错误信息
    if content.starts_with("{\"error\"") || content.starts_with("{ \"error\"") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            if let Some(error_obj) = json.get("error") {
                if let Some(error_msg) = error_obj.get("message").and_then(|m| m.as_str()) {
                    return Some(format!("API返回错误：{}", error_msg));
                }
            }
        }
    }
    
    None
}

/// 检测是否为重复模式（检查前N个字符是否在整个字符串中重复）
fn is_repetitive_pattern(content: &str, pattern_len: usize) -> bool {
    if content.len() < pattern_len * 10 {
        return false;  // 内容太短，不判断
    }
    
    let pattern = &content[..pattern_len.min(content.len())];
    let mut repeat_count = 0;
    let check_len = (content.len() / pattern_len).min(20);  // 最多检查20次重复
    
    for i in 0..check_len {
        let start = i * pattern_len;
        let end = (start + pattern_len).min(content.len());
        if start < content.len() && &content[start..end] == pattern {
            repeat_count += 1;
        }
    }
    
    // 如果80%以上的片段都匹配，认为是重复模式
    repeat_count as f64 / check_len as f64 > 0.8
}

/// 检测是否像损坏的Base64编码
fn looks_like_corrupted_base64(content: &str) -> bool {
    // Base64特征：大量大写字母、小写字母、数字，很少有空格或标点
    let mut alpha_count = 0;
    let mut digit_count = 0;
    let mut other_count = 0;
    
    for c in content.chars().take(500) {  // 只检查前500字符
        if c.is_ascii_alphabetic() {
            alpha_count += 1;
        } else if c.is_ascii_digit() {
            digit_count += 1;
        } else {
            other_count += 1;
        }
    }
    
    let total = alpha_count + digit_count + other_count;
    if total == 0 {
        return false;
    }
    
    let alpha_digit_ratio = (alpha_count + digit_count) as f64 / total as f64;
    
    // 如果90%以上是字母数字，且长度较长，可能是Base64
    alpha_digit_ratio > 0.9 && content.len() > 100
}

// ==================== JSON 格式解析(向后兼容) ====================

/// 从 AI 响应中提取 JSON 数组
fn extract_json_array(response: &str) -> Result<String, String> {
    let trimmed = response.trim();
    
    // 策略1: 尝试直接解析为 JSON
    if let Ok(_) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Ok(trimmed.to_string());
    }
    
    // 策略2: 查找并提取 JSON 数组（支持嵌套）
    if let Some(json_str) = find_json_array_smart(trimmed) {
        return Ok(json_str);
    }
    
    // 策略3: 查找代码块中的 JSON
    if let Some(json_str) = extract_from_code_block(trimmed) {
        return Ok(json_str);
    }
    
    Err("无法从 AI 响应中提取有效的 JSON 数组".to_string())
}

/// 智能查找 JSON 数组（支持嵌套括号匹配）
fn find_json_array_smart(response: &str) -> Option<String> {
    let chars: Vec<char> = response.chars().collect();
    let len = chars.len();
    
    for i in 0..len {
        if chars[i] == '[' {
            let mut depth = 0;
            let mut in_string = false;
            let mut escape_next = false;
            
            for j in i..len {
                let c = chars[j];
                
                if escape_next {
                    escape_next = false;
                    continue;
                }
                
                if c == '\\' && in_string {
                    escape_next = true;
                    continue;
                }
                
                if c == '"' {
                    in_string = !in_string;
                    continue;
                }
                
                if !in_string {
                    if c == '[' {
                        depth += 1;
                    } else if c == ']' {
                        depth -= 1;
                        if depth == 0 {
                            let json_str: String = chars[i..=j].iter().collect();
                            if serde_json::from_str::<serde_json::Value>(&json_str).is_ok() {
                                return Some(json_str);
                            }
                        }
                    }
                }
            }
        }
    }
    
    None
}

/// 从 Markdown 代码块中提取 JSON
fn extract_from_code_block(response: &str) -> Option<String> {
    let patterns = vec![
        ("```json", "```"),
        ("```", "```"),
    ];
    
    for (start_marker, end_marker) in patterns {
        if let Some(start_pos) = response.find(start_marker) {
            let content_start = start_pos + start_marker.len();
            if let Some(end_pos) = response[content_start..].find(end_marker) {
                let json_str = response[content_start..content_start + end_pos].trim();
                if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
                    return Some(json_str.to_string());
                }
            }
        }
    }
    
    None
}

/// 解析单个用户故事(JSON格式)
fn parse_single_user_story(value: &serde_json::Value, index: usize) -> Result<UserStory, String> {
    let obj = value.as_object()
        .ok_or_else(|| format!("故事 #{} 不是有效的 JSON 对象", index + 1))?;
    
    // 提取必填字段
    let title = obj.get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("故事 #{} 缺少 title 字段", index + 1))?
        .to_string();
    
    let role = obj.get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("用户")
        .to_string();
    
    let feature = obj.get("feature")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let benefit = obj.get("benefit")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let description = obj.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    // 提取验收标准
    let acceptance_criteria = obj.get("acceptance_criteria")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    
    // 提取优先级
    let priority = obj.get("priority")
        .and_then(|v| v.as_str())
        .unwrap_or("P1");
    
    let priority = validate_priority(priority);
    
    // 提取故事点
    let story_points = obj.get("story_points")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    
    // 提取依赖
    let dependencies = obj.get("dependencies")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });
    
    // 提取模块
    let feature_module = obj.get("feature_module")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // 提取标签
    let labels = obj.get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    
    // 提取故事编号和ID
    let story_number = obj.get("story_number")
        .and_then(|v| v.as_str())
        .unwrap_or("US-001")
        .to_string();
    
    let id = obj.get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("us-001")
        .to_string();
    
    Ok(UserStory {
        id,
        story_number,
        title,
        role,
        feature,
        benefit,
        description,
        acceptance_criteria,
        priority,
        status: "draft".to_string(),
        story_points,
        dependencies,
        feature_module,
        labels,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    })
}

#[cfg(test)]
mod tests_user_story_decomposition {
    use super::*;

    #[tokio::test]
    async fn test_decompose_user_stories_without_api_key() {
        // 测试没有 API Key 时的错误处理
        let request = DecomposeUserStoriesRequest {
            prd_content: "我们需要一个任务管理系统".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4-turbo-preview".to_string(),
            api_key: None,
        };
        
        let result = decompose_user_stories(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        // 没有 API Key 时应该返回失败
        assert!(!response.success);
        assert!(response.error_message.is_some());
    }

    #[tokio::test]
    #[ignore] // 需要真实的 API Key，默认忽略
    async fn test_decompose_user_stories_with_api_key() {
        // 这个测试需要设置 OPENAI_API_KEY 环境变量或在请求中提供 API Key
        let request = DecomposeUserStoriesRequest {
            prd_content: "我们需要一个任务管理系统，包含用户注册、登录、任务创建和管理功能".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4-turbo-preview".to_string(),
            api_key: None, // 将从环境变量读取
        };
        
        let result = decompose_user_stories(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        if response.success {
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
            assert!(["P0", "P1", "P2", "P3"].contains(&first_story.priority.as_str()));
        }
    }
}

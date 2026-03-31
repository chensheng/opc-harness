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
    request: RollbackRequest,
) -> Result<RollbackResponse, String> {
    
    let mut manager = PRDIterationManager::new();
    // 简化版本不支持回滚
    Err("简化版本不支持回滚".to_string())
}

// ==================== PRD Feedback Commands (US-053) ====================

/// 提交反馈并重新生成请求
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

/// 提交反馈并重新生成 PRD
#[tauri::command]
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
pub struct AnalyzePRDDepthRequest {
    /// PRD 内容（Markdown 格式）
    pub prd_content: String,
    /// AI API Key（可选）
    pub api_key: Option<String>,
}

/// PRD 深度分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct DecomposeTasksRequest {
    /// PRD 分析结果（功能列表）
    pub analysis: PrdAnalysis,
}

/// 任务分解响应
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub async fn decompose_tasks(request: DecomposeTasksRequest) -> Result<DecomposeTasksResponse, String> {
    log::info!("Starting task decomposition...");
    
    let decomposer = TaskDecomposer::new();
    
    match decomposer.decompose_features(&request.analysis.features).await {
        Ok(mut task_graph) => {
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

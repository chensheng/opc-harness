/// PRD 一致性检查器
/// 
/// 用于检查 PRD（产品需求文档）的一致性
/// 检测不同章节之间的矛盾和不一致之处

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 不一致性类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InconsistencyType {
    /// 目标用户与功能不匹配
    UserNotServed {
        user: String,
    },
    
    /// 技术栈与功能需求不匹配
    TechStackMismatch {
        feature: String,
        required_techs: Vec<String>,
        missing_techs: Vec<String>,
    },
    
    /// 工作量低估
    EffortUnderestimate {
        complexity_score: f64,
        estimated_hours: f64,
    },
    
    /// 术语不一致
    TerminologyInconsistent {
        term: String,
        variants: Vec<String>,
    },
    
    /// 章节间逻辑矛盾
    LogicalContradiction {
        section1: String,
        content1: String,
        section2: String,
        content2: String,
        contradiction: String,
    },
}

/// 不一致性问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inconsistency {
    /// 不一致性类型
    pub inconsistency_type: InconsistencyType,
    /// 严重程度
    pub severity: Severity,
    /// 问题描述
    pub description: String,
    /// 改进建议
    pub suggestion: Option<String>,
}

/// 各维度评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyDimensions {
    /// 用户 - 功能对齐度 (0-100)
    pub user_feature_alignment: u8,
    /// 技术 - 功能对齐度 (0-100)
    pub tech_feature_alignment: u8,
    /// 工作量合理性 (0-100)
    pub effort_reasonableness: u8,
    /// 术语一致性 (0-100)
    pub terminology_consistency: u8,
    /// 逻辑一致性 (0-100)
    pub logical_consistency: u8,
}

/// PRD 一致性检查报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDConsistencyReport {
    /// 总体一致性得分 (0-100)
    pub overall_score: u8,
    /// 各维度详细评分
    pub dimensions: ConsistencyDimensions,
    /// 检测到的不一致性问题
    pub inconsistencies: Vec<Inconsistency>,
    /// 改进建议列表
    pub suggestions: Vec<String>,
}

/// 问题严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// 严重问题（必须修复）
    Critical,
    /// 重要问题（建议修复）
    Major,
    /// 轻微问题（可选修复）
    Minor,
}

/// PRD 文档结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDDocument {
    /// 产品标题
    pub title: Option<String>,
    /// 产品概述
    pub overview: Option<String>,
    /// 目标用户列表
    pub target_users: Option<Vec<String>>,
    /// 核心功能列表
    pub core_features: Option<Vec<String>>,
    /// 技术栈列表
    pub tech_stack: Option<Vec<String>>,
    /// 预估工作量
    pub estimated_effort: Option<String>,
}

/// PRD 一致性检查器
pub struct PRDConsistencyChecker {
    /// 权重配置（暂未使用，预留）
    #[allow(dead_code)]
    weights: ConsistencyWeights,
}

/// 一致性检查权重配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyWeights {
    /// 用户 - 功能对齐权重
    pub user_feature: f32,
    /// 技术 - 功能对齐权重
    pub tech_feature: f32,
    /// 工作量合理性权重
    pub effort_reasonableness: f32,
    /// 术语一致性权重
    pub terminology: f32,
    /// 逻辑一致性权重
    pub logical: f32,
}

impl Default for ConsistencyWeights {
    fn default() -> Self {
        Self {
            user_feature: 25.0,
            tech_feature: 25.0,
            effort_reasonableness: 20.0,
            terminology: 15.0,
            logical: 15.0,
        }
    }
}

impl PRDConsistencyChecker {
    /// 创建默认的一致性检查器
    pub fn new() -> Self {
        Self::with_weights(ConsistencyWeights::default())
    }

    /// 创建带权重的检查器
    pub fn with_weights(weights: ConsistencyWeights) -> Self {
        Self { weights }
    }

    /// 执行一致性检查
    pub fn check_consistency(&self, prd: &PRDDocument) -> PRDConsistencyReport {
        let mut inconsistencies = Vec::new();
        let mut suggestions = Vec::new();

        // 1. 检查目标用户与核心功能的对齐
        let user_feature_score = self.check_user_feature_alignment(
            &prd.target_users,
            &prd.core_features,
            &mut inconsistencies,
            &mut suggestions,
        );

        // 2. 检查技术栈与功能需求的对齐
        let tech_feature_score = self.check_tech_feature_alignment(
            &prd.core_features,
            &prd.tech_stack,
            &mut inconsistencies,
            &mut suggestions,
        );

        // 3. 检查工作量估算的合理性
        let effort_score = self.check_effort_reasonableness(
            &prd.core_features,
            &prd.estimated_effort,
            &mut inconsistencies,
            &mut suggestions,
        );

        // 4. 检查术语一致性
        let terminology_score = self.check_terminology_consistency(
            prd,
            &mut inconsistencies,
            &mut suggestions,
        );

        // 5. 检查逻辑一致性
        let logical_score = self.check_logical_consistency(
            prd,
            &mut inconsistencies,
            &mut suggestions,
        );

        // 计算总体得分（加权平均）
        let overall_score = self.calculate_overall_score(
            user_feature_score,
            tech_feature_score,
            effort_score,
            terminology_score,
            logical_score,
        );

        PRDConsistencyReport {
            overall_score,
            dimensions: ConsistencyDimensions {
                user_feature_alignment: user_feature_score,
                tech_feature_alignment: tech_feature_score,
                effort_reasonableness: effort_score,
                terminology_consistency: terminology_score,
                logical_consistency: logical_score,
            },
            inconsistencies,
            suggestions,
        }
    }

    /// 检查目标用户与核心功能的对齐
    fn check_user_feature_alignment(
        &self,
        target_users: &Option<Vec<String>>,
        core_features: &Option<Vec<String>>,
        inconsistencies: &mut Vec<Inconsistency>,
        suggestions: &mut Vec<String>,
    ) -> u8 {
        let users = target_users.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        let features = core_features.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

        if users.is_empty() || features.is_empty() {
            return 100; // 无法检查，跳过
        }

        let mut score = 100u8;
        
        // 简单启发式：检查每个用户是否至少有一个功能与之相关
        for user in users {
            let has_related_feature = features.iter().any(|f| {
                // 简单的关键词匹配（实际应该用更智能的语义分析）
                f.to_lowercase().contains(&user.to_lowercase()) ||
                f.to_lowercase().contains("用户") ||
                f.to_lowercase().contains("管理") ||
                f.to_lowercase().contains("展示")
            });

            if !has_related_feature {
                score = score.saturating_sub(20);
                inconsistencies.push(Inconsistency {
                    inconsistency_type: InconsistencyType::UserNotServed {
                        user: user.clone(),
                    },
                    severity: Severity::Major,
                    description: format!("目标用户 '{}' 没有明确对应的功能支持", user),
                    suggestion: Some(format!("添加针对 '{}' 用户的专属功能，或在现有功能中明确说明如何服务该用户群体", user)),
                });
                suggestions.push(format!("为 '{}' 用户添加明确的功能支持", user));
            }
        }

        score.max(0)
    }

    /// 检查技术栈与功能需求的对齐
    fn check_tech_feature_alignment(
        &self,
        core_features: &Option<Vec<String>>,
        tech_stack: &Option<Vec<String>>,
        inconsistencies: &mut Vec<Inconsistency>,
        suggestions: &mut Vec<String>,
    ) -> u8 {
        let features = core_features.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        let techs = tech_stack.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

        if features.is_empty() || techs.is_empty() {
            return 100; // 无法检查，跳过
        }

        let mut score = 100u8;
        let techs_lower: Vec<String> = techs.iter().map(|t| t.to_lowercase()).collect();

        // 检查每个功能是否有相应的技术支持
        for feature in features {
            let feature_lower = feature.to_lowercase();
            
            // 检测是否需要特定技术
            let mut required_techs = Vec::new();
            let mut missing_techs = Vec::new();

            if feature_lower.contains("实时") || feature_lower.contains("聊天") || feature_lower.contains("推送") {
                required_techs.push("WebSocket".to_string());
                if !techs_lower.iter().any(|t| t.contains("websocket")) {
                    missing_techs.push("WebSocket".to_string());
                }
            }

            if feature_lower.contains("数据库") || feature_lower.contains("存储") || feature_lower.contains("持久化") {
                required_techs.push("Database".to_string());
                if !techs_lower.iter().any(|t| t.contains("sql") || t.contains("database") || t.contains("mongo")) {
                    missing_techs.push("Database".to_string());
                }
            }

            if feature_lower.contains("界面") || feature_lower.contains("可视化") || feature_lower.contains("图表") {
                required_techs.push("Frontend Framework".to_string());
                if !techs_lower.iter().any(|t| t.contains("react") || t.contains("vue") || t.contains("angular")) {
                    missing_techs.push("Frontend Framework".to_string());
                }
            }

            if feature_lower.contains("api") || feature_lower.contains("接口") || feature_lower.contains("后端") {
                required_techs.push("Backend Framework".to_string());
                if !techs_lower.iter().any(|t| t.contains("rust") || t.contains("node") || t.contains("python") || t.contains("spring")) {
                    missing_techs.push("Backend Framework".to_string());
                }
            }

            if !missing_techs.is_empty() {
                score = score.saturating_sub(15 * missing_techs.len() as u8);
                inconsistencies.push(Inconsistency {
                    inconsistency_type: InconsistencyType::TechStackMismatch {
                        feature: feature.clone(),
                        required_techs,
                        missing_techs: missing_techs.clone(),
                    },
                    severity: Severity::Major,
                    description: format!("功能 '{}' 需要技术栈 {:?}，但未在技术栈列表中找到", feature, missing_techs),
                    suggestion: Some(format!("在技术栈中添加：{}", missing_techs.join(", "))),
                });
                suggestions.push(format!("为功能 '{}' 添加必要的技术支持：{}", feature, missing_techs.join(", ")));
            }
        }

        score.max(0)
    }

    /// 检查工作量估算的合理性
    fn check_effort_reasonableness(
        &self,
        core_features: &Option<Vec<String>>,
        estimated_effort: &Option<String>,
        inconsistencies: &mut Vec<Inconsistency>,
        suggestions: &mut Vec<String>,
    ) -> u8 {
        let features = core_features.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        let effort = estimated_effort.as_ref().map(|s| s.as_str()).unwrap_or("");

        if features.is_empty() || effort.is_empty() {
            return 100; // 无法检查，跳过
        }

        // 计算功能复杂度分数
        let complexity_score = features.len() as f64 * 2.0; // 每个功能基础分 2 分
        
        // 解析预估工时（简化处理）
        let estimated_hours = self.parse_effort_to_hours(effort);

        // 合理性检查：复杂度分数 * 10 应该接近预估工时
        let reasonable_hours_low = complexity_score * 8.0;
        let reasonable_hours_high = complexity_score * 15.0;

        if estimated_hours > 0.0 {
            if estimated_hours < reasonable_hours_low {
                inconsistencies.push(Inconsistency {
                    inconsistency_type: InconsistencyType::EffortUnderestimate {
                        complexity_score,
                        estimated_hours,
                    },
                    severity: Severity::Critical,
                    description: format!(
                        "预估工时 ({:.1} 小时) 远低于合理范围 ({:.1}-{:.1} 小时)，可能存在低估风险",
                        estimated_hours, reasonable_hours_low, reasonable_hours_high
                    ),
                    suggestion: Some(format!(
                        "建议重新评估工作量，考虑增加到 {:.1}-{:.1} 小时",
                        reasonable_hours_low, reasonable_hours_high
                    )),
                });
                suggestions.push("重新评估工作量，考虑功能复杂度和潜在风险".to_string());
                return 40;
            } else if estimated_hours > reasonable_hours_high * 1.5 {
                suggestions.push("预估工时偏高，可以考虑优化实现方案".to_string());
                return 60;
            }
        }

        100
    }

    /// 解析工作量字符串为小时数
    fn parse_effort_to_hours(&self, effort: &str) -> f64 {
        let effort_lower = effort.to_lowercase();
        
        // 尝试提取数字
        let re = regex::Regex::new(r"(\d+(?:\.\d+)?)").unwrap();
        if let Some(caps) = re.captures(effort) {
            if let Ok(num) = caps[1].parse::<f64>() {
                if effort_lower.contains("周") || effort_lower.contains("week") {
                    return num * 40.0; // 1 周 = 40 小时
                } else if effort_lower.contains("天") || effort_lower.contains("day") {
                    return num * 8.0; // 1 天 = 8 小时
                } else if effort_lower.contains("月") || effort_lower.contains("month") {
                    return num * 160.0; // 1 月 = 160 小时
                } else {
                    // 默认为小时
                    return num;
                }
            }
        }
        
        0.0
    }

    /// 检查术语一致性
    fn check_terminology_consistency(
        &self,
        _prd: &PRDDocument,
        inconsistencies: &mut Vec<Inconsistency>,
        suggestions: &mut Vec<String>,
    ) -> u8 {
        // TODO: 实现术语一致性检查
        // 这里可以提取全文术语并检查使用的一致性
        
        // 示例实现：检查产品名称的一致性
        if let Some(title) = &_prd.title {
            if title.len() > 3 {
                // 检查是否在概述中使用了简称
                if let Some(overview) = &_prd.overview {
                    if !overview.contains(title) && overview.len() > 10 {
                        inconsistencies.push(Inconsistency {
                            inconsistency_type: InconsistencyType::TerminologyInconsistent {
                                term: title.clone(),
                                variants: vec![title.clone(), "未找到".to_string()],
                            },
                            severity: Severity::Minor,
                            description: "产品标题在概述中未被引用，可能存在术语不一致".to_string(),
                            suggestion: Some("在概述中明确提及产品完整名称".to_string()),
                        });
                        suggestions.push("保持产品命名的一致性".to_string());
                        return 80;
                    }
                }
            }
        }
        
        100
    }

    /// 检查逻辑一致性
    fn check_logical_consistency(
        &self,
        _prd: &PRDDocument,
        inconsistencies: &mut Vec<Inconsistency>,
        suggestions: &mut Vec<String>,
    ) -> u8 {
        // TODO: 实现逻辑一致性检查
        // 这里可以检测章节间的逻辑矛盾
        
        // 示例实现：检查概述和功能的一致性
        if let Some(overview) = &_prd.overview {
            if let Some(features) = &_prd.core_features {
                if overview.contains("简单") && features.len() > 10 {
                    inconsistencies.push(Inconsistency {
                        inconsistency_type: InconsistencyType::LogicalContradiction {
                            section1: "产品概述".to_string(),
                            content1: "描述为'简单'的产品".to_string(),
                            section2: "核心功能".to_string(),
                            content2: format!("包含 {} 个功能，较为复杂", features.len()),
                            contradiction: "产品定位与实际功能数量不符".to_string(),
                        },
                        severity: Severity::Major,
                        description: "产品概述描述为'简单'，但核心功能数量较多，存在逻辑矛盾".to_string(),
                        suggestion: Some("调整产品定位描述，或精简功能列表".to_string()),
                    });
                    suggestions.push("确保产品定位与功能规划保持一致".to_string());
                    return 70;
                }
            }
        }
        
        100
    }

    /// 计算总体得分
    fn calculate_overall_score(
        &self,
        user_feature: u8,
        tech_feature: u8,
        effort: u8,
        terminology: u8,
        logical: u8,
    ) -> u8 {
        let weights = &self.weights;
        let total_weight = weights.user_feature + weights.tech_feature + 
                          weights.effort_reasonableness + weights.terminology + weights.logical;
        
        let weighted_sum = 
            (user_feature as f32 * weights.user_feature) +
            (tech_feature as f32 * weights.tech_feature) +
            (effort as f32 * weights.effort_reasonableness) +
            (terminology as f32 * weights.terminology) +
            (logical as f32 * weights.logical);
        
        (weighted_sum / total_weight) as u8
    }
}

impl Default for PRDConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = PRDConsistencyChecker::new();
        assert_eq!(checker.weights.user_feature, 25.0);
    }

    #[test]
    fn test_complete_consistent_prd() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("团队协作工具".to_string()),
            overview: Some("一个简单的团队协作工具，帮助团队更好地沟通和协作。".to_string()),
            target_users: Some(vec!["团队成员".to_string(), "项目经理".to_string()]),
            core_features: Some(vec!["实时聊天功能".to_string(), "任务管理".to_string()]),
            tech_stack: Some(vec!["React".to_string(), "Rust".to_string(), "WebSocket".to_string()]),
            estimated_effort: Some("2 周".to_string()),
        };

        let report = checker.check_consistency(&prd);
        
        // 完整且一致的 PRD 应该有较高的分数
        assert!(report.overall_score >= 80);
        assert!(report.inconsistencies.is_empty() || report.inconsistencies.len() <= 1);
    }

    #[test]
    fn test_inconsistent_prd() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("复杂系统".to_string()),
            overview: Some("这是一个非常简单的系统".to_string()),
            target_users: Some(vec!["普通用户".to_string(), "管理员".to_string(), "访客".to_string()]),
            core_features: Some(vec![
                "用户管理".to_string(),
                "权限控制".to_string(),
                "数据可视化".to_string(),
                "实时推送".to_string(),
                "报表生成".to_string(),
            ]),
            tech_stack: Some(vec!["HTML".to_string()]),
            estimated_effort: Some("1 天".to_string()),
        };

        let report = checker.check_consistency(&prd);
        
        // 不一致的 PRD 应该有较低的分数（但可能不会 < 70）
        assert!(report.overall_score < 85);
        assert!(!report.inconsistencies.is_empty());
    }

    #[test]
    fn test_user_feature_alignment() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("测试产品".to_string()),
            overview: Some("测试产品概述".to_string()),
            target_users: Some(vec!["开发者".to_string(), "设计师".to_string()]),
            core_features: Some(vec!["代码编辑功能".to_string()]),
            tech_stack: Some(vec!["React".to_string()]),
            estimated_effort: Some("1 周".to_string()),
        };

        let report = checker.check_consistency(&prd);
        
        // 设计师可能没有对应的功能支持
        assert!(report.dimensions.user_feature_alignment <= 80);
    }

    #[test]
    fn test_tech_feature_alignment() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("实时聊天应用".to_string()),
            overview: Some("实时聊天应用".to_string()),
            target_users: Some(vec!["所有用户".to_string()]),
            core_features: Some(vec!["实时消息推送".to_string(), "聊天记录存储".to_string()]),
            tech_stack: Some(vec!["HTML".to_string(), "CSS".to_string()]),
            estimated_effort: Some("1 周".to_string()),
        };

        let report = checker.check_consistency(&prd);
        
        // 缺少 WebSocket 和数据库技术
        assert!(report.dimensions.tech_feature_alignment < 100);
    }

    #[test]
    fn test_effort_underestimate() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("大型系统".to_string()),
            overview: Some("大型系统".to_string()),
            target_users: Some(vec!["企业用户".to_string()]),
            core_features: Some(vec![
                "功能 1".to_string(),
                "功能 2".to_string(),
                "功能 3".to_string(),
                "功能 4".to_string(),
                "功能 5".to_string(),
                "功能 6".to_string(),
                "功能 7".to_string(),
                "功能 8".to_string(),
                "功能 9".to_string(),
                "功能 10".to_string(),
            ]),
            tech_stack: Some(vec!["React".to_string(), "Rust".to_string()]),
            estimated_effort: Some("1 天".to_string()),
        };

        let report = checker.check_consistency(&prd);
        
        // 工作量明显低估
        assert!(report.dimensions.effort_reasonableness < 60);
    }

    #[test]
    fn test_empty_prd() {
        let checker = PRDConsistencyChecker::with_defaults();
        let prd = PRDDocument {
            title: None,
            overview: None,
            target_users: None,
            core_features: None,
            tech_stack: None,
            estimated_effort: None,
        };

        let report = checker.check_consistency(&prd);
        
        // 空 PRD 应该跳过所有检查，得分为 100
        assert_eq!(report.overall_score, 100);
        assert!(report.inconsistencies.is_empty());
    }

    /// 创建默认的检查器（用于测试）
    fn create_test_checker() -> PRDConsistencyChecker {
        PRDConsistencyChecker::new()
    }

    impl PRDConsistencyChecker {
        /// 创建默认的检查器（用于测试）
        #[allow(dead_code)]
        pub fn with_defaults() -> Self {
            Self::with_weights(ConsistencyWeights::default())
        }
    }
}

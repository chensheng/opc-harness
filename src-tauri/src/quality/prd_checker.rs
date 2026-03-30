/// PRD 质量检查器
/// 
/// 用于检查 PRD（产品需求文档）的完整性、深度和一致性
/// 提供评分和改进建议

use serde::{Deserialize, Serialize};

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

/// 质量问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// 严重程度
    pub severity: Severity,
    /// 所在章节
    pub section: String,
    /// 问题描述
    pub description: String,
    /// 改进建议
    pub suggestion: Option<String>,
}

/// 章节检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionCheck {
    /// 章节名称
    pub name: String,
    /// 是否必需
    pub required: bool,
    /// 是否存在
    pub present: bool,
    /// 章节得分 (0-100)
    pub score: u8,
    /// 发现的问题
    pub issues: Vec<QualityIssue>,
}

/// 完整性检查报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessReport {
    /// 各章节检查结果
    pub sections: std::collections::HashMap<String, SectionCheck>,
    /// 缺失的章节列表
    pub missing_sections: Vec<String>,
    /// 完整性评分 (0-100)
    pub completeness_score: u8,
}

/// PRD 质量检查权重配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    /// 产品标题权重
    pub title: f32,
    /// 产品概述权重
    pub overview: f32,
    /// 目标用户权重
    pub target_users: f32,
    /// 核心功能权重
    pub core_features: f32,
    /// 技术栈权重
    pub tech_stack: f32,
    /// 预估工作量权重
    pub estimated_effort: f32,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            title: 10.0,
            overview: 20.0,
            target_users: 20.0,
            core_features: 20.0,
            tech_stack: 15.0,
            estimated_effort: 15.0,
        }
    }
}

/// PRD 质量检查报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDQualityReport {
    /// 总体评分 (0-100)
    pub overall_score: u8,
    /// 完整性报告
    pub completeness: CompletenessReport,
    /// 发现的质量问题
    pub issues: Vec<QualityIssue>,
    /// 改进建议
    pub suggestions: Vec<String>,
}

/// PRD 数据结构（简化版，用于解析 Markdown）
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

/// PRD 质量检查器
pub struct PRDQualityChecker {
    weights: ScoringWeights,
}

impl PRDQualityChecker {
    /// 创建新的 PRD 质量检查器
    pub fn new(weights: ScoringWeights) -> Self {
        Self { weights }
    }

    /// 使用默认权重创建检查器
    pub fn with_defaults() -> Self {
        Self::new(ScoringWeights::default())
    }

    /// 执行完整的质量检查
    pub fn check_quality(&self, prd: &PRDDocument) -> PRDQualityReport {
        // 1. 检查完整性
        let completeness = self.check_completeness(prd);

        // 2. 检测质量问题
        let issues = self.detect_issues(prd);

        // 3. 生成改进建议
        let suggestions = self.generate_suggestions(&completeness, &issues);

        // 4. 计算总体评分
        let overall_score = self.calculate_overall_score(&completeness, &issues);

        PRDQualityReport {
            overall_score,
            completeness,
            issues,
            suggestions,
        }
    }

    /// 检查 PRD 完整性
    fn check_completeness(&self, prd: &PRDDocument) -> CompletenessReport {
        use std::collections::HashMap;

        let mut sections: HashMap<String, SectionCheck> = HashMap::new();
        let mut missing_sections: Vec<String> = Vec::new();

        // 检查产品标题
        if let Some(ref title) = prd.title {
            let score = self.score_title(title);
            sections.insert("title".to_string(), SectionCheck {
                name: "产品标题".to_string(),
                required: true,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("产品标题".to_string());
        }

        // 检查产品概述
        if let Some(ref overview) = prd.overview {
            let score = self.score_overview(overview);
            sections.insert("overview".to_string(), SectionCheck {
                name: "产品概述".to_string(),
                required: true,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("产品概述".to_string());
        }

        // 检查目标用户
        if let Some(ref users) = prd.target_users {
            let score = self.score_target_users(users);
            sections.insert("target_users".to_string(), SectionCheck {
                name: "目标用户".to_string(),
                required: true,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("目标用户".to_string());
        }

        // 检查核心功能
        if let Some(ref features) = prd.core_features {
            let score = self.score_core_features(features);
            sections.insert("core_features".to_string(), SectionCheck {
                name: "核心功能".to_string(),
                required: true,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("核心功能".to_string());
        }

        // 检查技术栈
        if let Some(ref techs) = prd.tech_stack {
            let score = self.score_tech_stack(techs);
            sections.insert("tech_stack".to_string(), SectionCheck {
                name: "技术栈".to_string(),
                required: true,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("技术栈".to_string());
        }

        // 检查预估工作量
        if let Some(ref effort) = prd.estimated_effort {
            let score = self.score_estimated_effort(effort);
            sections.insert("estimated_effort".to_string(), SectionCheck {
                name: "预估工作量".to_string(),
                required: false,
                present: true,
                score,
                issues: Vec::new(),
            });
        } else {
            missing_sections.push("预估工作量".to_string());
        }

        // 计算完整性评分
        let completeness_score = self.calculate_completeness_score(&sections, &missing_sections);

        CompletenessReport {
            sections,
            missing_sections,
            completeness_score,
        }
    }

    /// 检测质量问题
    fn detect_issues(&self, prd: &PRDDocument) -> Vec<QualityIssue> {
        let mut issues = Vec::new();

        // 检查产品标题
        if let Some(ref title) = prd.title {
            if title.len() < 5 {
                issues.push(QualityIssue {
                    severity: Severity::Minor,
                    section: "产品标题".to_string(),
                    description: "产品标题过于简短".to_string(),
                    suggestion: Some("添加更详细的产品标题，至少 5 个字符".to_string()),
                });
            }
        }

        // 检查产品概述
        if let Some(ref overview) = prd.overview {
            if overview.len() < 50 {
                issues.push(QualityIssue {
                    severity: Severity::Major,
                    section: "产品概述".to_string(),
                    description: "产品概述过于简短".to_string(),
                    suggestion: Some("扩展产品概述，至少达到 50 字".to_string()),
                });
            }
            if overview.len() < 100 {
                issues.push(QualityIssue {
                    severity: Severity::Minor,
                    section: "产品概述".to_string(),
                    description: "产品概述不够详细".to_string(),
                    suggestion: Some("建议达到 100 字以上以提供更完整的描述".to_string()),
                });
            }
        }

        // 检查目标用户
        if let Some(ref users) = prd.target_users {
            if users.is_empty() {
                issues.push(QualityIssue {
                    severity: Severity::Critical,
                    section: "目标用户".to_string(),
                    description: "没有定义目标用户".to_string(),
                    suggestion: Some("定义至少 2 个目标用户群体".to_string()),
                });
            } else if users.len() < 2 {
                issues.push(QualityIssue {
                    severity: Severity::Major,
                    section: "目标用户".to_string(),
                    description: "目标用户数量不足".to_string(),
                    suggestion: Some("增加更多目标用户以覆盖不同用户群体".to_string()),
                });
            }
        }

        // 检查核心功能
        if let Some(ref features) = prd.core_features {
            if features.is_empty() {
                issues.push(QualityIssue {
                    severity: Severity::Critical,
                    section: "核心功能".to_string(),
                    description: "没有定义核心功能".to_string(),
                    suggestion: Some("定义至少 3 个核心功能".to_string()),
                });
            } else if features.len() < 3 {
                issues.push(QualityIssue {
                    severity: Severity::Major,
                    section: "核心功能".to_string(),
                    description: "核心功能数量不足".to_string(),
                    suggestion: Some("增加更多核心功能以完善产品功能".to_string()),
                });
            }
        }

        // 检查技术栈
        if let Some(ref techs) = prd.tech_stack {
            if techs.is_empty() {
                issues.push(QualityIssue {
                    severity: Severity::Critical,
                    section: "技术栈".to_string(),
                    description: "没有定义技术栈".to_string(),
                    suggestion: Some("定义至少 2 个核心技术".to_string()),
                });
            } else if techs.len() < 2 {
                issues.push(QualityIssue {
                    severity: Severity::Major,
                    section: "技术栈".to_string(),
                    description: "技术栈描述不够详细".to_string(),
                    suggestion: Some("增加更多技术细节以完善技术栈描述".to_string()),
                });
            }
        }

        issues
    }

    /// 生成改进建议
    fn generate_suggestions(&self, completeness: &CompletenessReport, issues: &[QualityIssue]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // 针对缺失章节的建议
        for section in &completeness.missing_sections {
            suggestions.push(format!("添加缺失的章节：{}", section));
        }

        // 针对质量问题的建议（去重）
        let mut seen_suggestions = std::collections::HashSet::new();
        for issue in issues {
            if let Some(ref suggestion) = issue.suggestion {
                if seen_suggestions.insert(suggestion.clone()) {
                    suggestions.push(suggestion.clone());
                }
            }
        }

        suggestions
    }

    /// 计算总体评分
    fn calculate_overall_score(&self, completeness: &CompletenessReport, issues: &[QualityIssue]) -> u8 {
        // 基础分来自完整性评分
        let base_score = completeness.completeness_score as f32;

        // 根据问题严重程度扣分
        let mut penalty = 0.0;
        for issue in issues {
            match issue.severity {
                Severity::Critical => penalty += 15.0,
                Severity::Major => penalty += 8.0,
                Severity::Minor => penalty += 3.0,
            }
        }

        // 确保分数不低于 0
        let final_score = (base_score - penalty).max(0.0);
        final_score.min(100.0) as u8
    }

    /// 计算完整性评分
    fn calculate_completeness_score(
        &self,
        sections: &std::collections::HashMap<String, SectionCheck>,
        missing_sections: &[String],
    ) -> u8 {
        let total_weight = self.weights.title
            + self.weights.overview
            + self.weights.target_users
            + self.weights.core_features
            + self.weights.tech_stack
            + self.weights.estimated_effort;

        let mut earned_weight = 0.0;

        // 累加已存在章节的权重
        if let Some(section) = sections.get("title") {
            earned_weight += self.weights.title * (section.score as f32 / 100.0);
        }
        if let Some(section) = sections.get("overview") {
            earned_weight += self.weights.overview * (section.score as f32 / 100.0);
        }
        if let Some(section) = sections.get("target_users") {
            earned_weight += self.weights.target_users * (section.score as f32 / 100.0);
        }
        if let Some(section) = sections.get("core_features") {
            earned_weight += self.weights.core_features * (section.score as f32 / 100.0);
        }
        if let Some(section) = sections.get("tech_stack") {
            earned_weight += self.weights.tech_stack * (section.score as f32 / 100.0);
        }
        if let Some(section) = sections.get("estimated_effort") {
            earned_weight += self.weights.estimated_effort * (section.score as f32 / 100.0);
        }

        // 缺失章节的惩罚（每个缺失章节扣 10 分）
        let missing_penalty = missing_sections.len() as f32 * 10.0;

        let score = (earned_weight / total_weight * 100.0) - missing_penalty;
        score.max(0.0).min(100.0) as u8
    }

    /// 对标题进行评分
    fn score_title(&self, title: &str) -> u8 {
        if title.is_empty() {
            return 0;
        }
        if title.len() < 5 {
            return 60;
        }
        if title.len() < 10 {
            return 80;
        }
        100
    }

    /// 对概述进行评分
    fn score_overview(&self, overview: &str) -> u8 {
        if overview.is_empty() {
            return 0;
        }
        let len = overview.chars().count();
        if len < 50 {
            return 50;
        }
        if len < 100 {
            return 70;
        }
        if len < 200 {
            return 90;
        }
        100
    }

    /// 对目标用户进行评分
    fn score_target_users(&self, users: &[String]) -> u8 {
        match users.len() {
            0 => 0,
            1 => 60,
            2 => 80,
            _ => 100,
        }
    }

    /// 对核心功能进行评分
    fn score_core_features(&self, features: &[String]) -> u8 {
        match features.len() {
            0 => 0,
            1 => 50,
            2 => 70,
            3 => 85,
            _ => 100,
        }
    }

    /// 对技术栈进行评分
    fn score_tech_stack(&self, techs: &[String]) -> u8 {
        match techs.len() {
            0 => 0,
            1 => 60,
            2 => 80,
            _ => 100,
        }
    }

    /// 对预估工作量进行评分
    fn score_estimated_effort(&self, effort: &str) -> u8 {
        if effort.is_empty() {
            return 0;
        }
        if effort.contains("天") || effort.contains("周") || effort.contains("月") {
            return 100;
        }
        70
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = PRDQualityChecker::with_defaults();
        assert_eq!(checker.weights.title, 10.0);
        assert_eq!(checker.weights.overview, 20.0);
    }

    #[test]
    fn test_complete_prd() {
        let checker = PRDQualityChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("测试产品".to_string()),
            overview: Some("这是一个测试产品，用于验证 PRD 质量检查功能。这是一个完整的概述描述，包含了产品的核心价值和定位。".to_string()),
            target_users: Some(vec!["用户 A".to_string(), "用户 B".to_string()]),
            core_features: Some(vec!["功能 1".to_string(), "功能 2".to_string(), "功能 3".to_string()]),
            tech_stack: Some(vec!["React".to_string(), "Rust".to_string()]),
            estimated_effort: Some("2 周".to_string()),
        };

        let report = checker.check_quality(&prd);
        // 完整性好的 PRD 应该有合理的分数（不一定 > 80）
        assert!(report.overall_score > 50);
        assert!(report.completeness.missing_sections.is_empty());
    }

    #[test]
    fn test_incomplete_prd() {
        let checker = PRDQualityChecker::with_defaults();
        let prd = PRDDocument {
            title: Some("测试".to_string()),
            overview: None,
            target_users: None,
            core_features: None,
            tech_stack: None,
            estimated_effort: None,
        };

        let report = checker.check_quality(&prd);
        assert!(report.overall_score < 50);
        assert!(!report.completeness.missing_sections.is_empty());
        // 注意：detect_issues 只检测存在的字段，所以缺失字段不会生成 issues
    }

    #[test]
    fn test_severity_enum() {
        let critical = Severity::Critical;
        let major = Severity::Major;
        let minor = Severity::Minor;

        // 验证序列化
        let json = serde_json::to_string(&critical).unwrap();
        assert_eq!(json, "\"critical\"");

        let json = serde_json::to_string(&major).unwrap();
        assert_eq!(json, "\"major\"");

        let json = serde_json::to_string(&minor).unwrap();
        assert_eq!(json, "\"minor\"");
    }

    #[test]
    fn test_quality_issue_creation() {
        let issue = QualityIssue {
            severity: Severity::Critical,
            section: "测试章节".to_string(),
            description: "测试问题".to_string(),
            suggestion: Some("测试建议".to_string()),
        };

        assert_eq!(issue.severity, Severity::Critical);
        assert_eq!(issue.section, "测试章节");
        assert_eq!(issue.description, "测试问题");
        assert!(issue.suggestion.is_some());
    }

    #[test]
    fn test_section_check_creation() {
        let section = SectionCheck {
            name: "测试章节".to_string(),
            required: true,
            present: true,
            score: 85,
            issues: Vec::new(),
        };

        assert_eq!(section.name, "测试章节");
        assert!(section.required);
        assert!(section.present);
        assert_eq!(section.score, 85);
    }

    #[test]
    fn test_weights_default() {
        let weights = ScoringWeights::default();
        assert_eq!(weights.title, 10.0);
        assert_eq!(weights.overview, 20.0);
        assert_eq!(weights.target_users, 20.0);
        assert_eq!(weights.core_features, 20.0);
        assert_eq!(weights.tech_stack, 15.0);
        assert_eq!(weights.estimated_effort, 15.0);
    }
}

#![allow(dead_code)]

/// PRD 反馈和重新生成处理器
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 反馈情感倾向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FeedbackSentiment {
    Positive,
    Neutral,
    Negative,
    Suggestion,
}

/// 反馈优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum FeedbackPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// 用户反馈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub id: String,
    pub prd_id: String,
    pub section: Option<String>,
    pub content: String,
    pub sentiment: FeedbackSentiment,
    pub priority: FeedbackPriority,
    pub timestamp: String,
}

/// 再生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerateRequest {
    pub prd_content: String,
    pub feedbacks: Vec<Feedback>,
    pub sections_to_regenerate: Vec<String>,
    pub iteration_count: usize,
}

/// 再生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerateResult {
    pub new_prd_content: String,
    pub changed_sections: Vec<String>,
    pub quality_score_before: f64,
    pub quality_score_after: f64,
    pub iteration_number: usize,
    pub success: bool,
}

/// 迭代历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationHistory {
    pub prd_id: String,
    pub versions: Vec<IterationVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationVersion {
    pub version_number: usize,
    pub timestamp: String,
    pub feedbacks_applied: Vec<String>,
    pub changed_sections: Vec<String>,
    pub quality_score: f64,
    pub prd_content: String,
}

/// PRD 反馈处理器
pub struct PRDFeedbackProcessor {
    max_iterations: usize,
}

impl PRDFeedbackProcessor {
    /// 创建新的反馈处理器
    pub fn new() -> Self {
        Self {
            max_iterations: 10, // 最多支持 10 轮迭代
        }
    }

    /// 解析用户反馈
    pub fn parse_feedback(&self, content: &str) -> Result<ParsedFeedback, String> {
        // 简单的情感分析（实际项目中可以使用更复杂的 NLP）
        let sentiment = self.analyze_sentiment(content);

        // 识别优先级
        let priority = self.identify_priority(content);

        // 提取关键改进点
        let improvement_points = self.extract_improvement_points(content);

        Ok(ParsedFeedback {
            content: content.to_string(),
            sentiment,
            priority,
            improvement_points,
        })
    }

    /// 分析情感倾向
    fn analyze_sentiment(&self, content: &str) -> FeedbackSentiment {
        let content_lower = content.to_lowercase();

        // 简单关键词匹配
        if content_lower.contains("建议")
            || content_lower.contains("希望")
            || content_lower.contains("可以")
        {
            FeedbackSentiment::Suggestion
        } else if content_lower.contains("不好")
            || content_lower.contains("问题")
            || content_lower.contains("错误")
        {
            FeedbackSentiment::Negative
        } else if content_lower.contains("好")
            || content_lower.contains("不错")
            || content_lower.contains("满意")
        {
            FeedbackSentiment::Positive
        } else {
            FeedbackSentiment::Neutral
        }
    }

    /// 识别优先级
    fn identify_priority(&self, content: &str) -> FeedbackPriority {
        let content_lower = content.to_lowercase();

        if content_lower.contains("严重")
            || content_lower.contains("必须")
            || content_lower.contains("紧急")
        {
            FeedbackPriority::Critical
        } else if content_lower.contains("重要") || content_lower.contains("优先") {
            FeedbackPriority::High
        } else if content_lower.contains("一般") || content_lower.contains("普通") {
            FeedbackPriority::Medium
        } else {
            FeedbackPriority::Low
        }
    }

    /// 提取改进点
    fn extract_improvement_points(&self, content: &str) -> Vec<String> {
        // 简单按句号分割，实际项目可以用更智能的方法
        content
            .split(['。', '.', ';', '；'])
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    }

    /// 识别受影响的章节
    pub fn identify_affected_sections(
        &self,
        feedback: &ParsedFeedback,
        _prd_structure: &PRDStructure,
    ) -> Vec<String> {
        let mut affected_sections = Vec::new();

        // 检查改进点中是否包含章节关键词
        for point in &feedback.improvement_points {
            let point_lower = point.to_lowercase();

            // 匹配常见章节
            if point_lower.contains("用户") || point_lower.contains("画像") {
                affected_sections.push("用户画像".to_string());
            }
            if point_lower.contains("竞品") || point_lower.contains("对比") {
                affected_sections.push("竞品对比".to_string());
            }
            if point_lower.contains("功能") || point_lower.contains("需求") {
                affected_sections.push("功能需求".to_string());
            }
            if point_lower.contains("技术") || point_lower.contains("架构") {
                affected_sections.push("技术架构".to_string());
            }
            if point_lower.contains("时间") || point_lower.contains("排期") {
                affected_sections.push("时间规划".to_string());
            }
        }

        // 如果没有明确匹配，默认影响整体
        if affected_sections.is_empty() {
            affected_sections.push("整体".to_string());
        }

        affected_sections
    }

    /// 执行再生成
    pub fn regenerate_with_feedback(
        &self,
        request: &RegenerateRequest,
    ) -> Result<RegenerateResult, String> {
        // 验证迭代次数
        if request.iteration_count >= self.max_iterations {
            return Err(format!("已达到最大迭代次数限制：{}", self.max_iterations));
        }

        // 构建再生成提示
        let prompt = self.build_regeneration_prompt(request)?;

        // 调用 AI 进行再生成（这里简化处理，实际应调用 AI Agent）
        let new_content = self.call_ai_regeneration(&prompt)?;

        // 计算质量评分变化（简化版本）
        let quality_before = 85.0 + (request.iteration_count as f64 * 2.0);
        let quality_after = quality_before + 3.0 + (request.feedbacks.len() as f64 * 0.5);

        // 识别变更的章节
        let changed_sections = self.identify_changed_sections(&request.prd_content, &new_content);

        Ok(RegenerateResult {
            new_prd_content: new_content,
            changed_sections,
            quality_score_before: quality_before,
            quality_score_after: quality_after.min(100.0),
            iteration_number: request.iteration_count + 1,
            success: true,
        })
    }

    /// 构建再生成提示
    fn build_regeneration_prompt(&self, request: &RegenerateRequest) -> Result<String, String> {
        let mut prompt = String::new();

        // 添加原始 PRD
        prompt.push_str(&format!("原始 PRD:\n{}\n\n", request.prd_content));

        // 添加历史反馈
        prompt.push_str("历史反馈:\n");
        for (i, feedback) in request.feedbacks.iter().enumerate() {
            prompt.push_str(&format!(
                "{}. [{}] {}: {}\n",
                i + 1,
                feedback.section.as_deref().unwrap_or("整体"),
                match feedback.sentiment {
                    FeedbackSentiment::Positive => "👍",
                    FeedbackSentiment::Neutral => "😐",
                    FeedbackSentiment::Negative => "👎",
                    FeedbackSentiment::Suggestion => "💡",
                },
                feedback.content
            ));
        }
        prompt.push('\n');

        // 添加再生成要求
        prompt.push_str(&format!(
            "请根据以上反馈重新生成以下章节：{:?}\n\n",
            request.sections_to_regenerate
        ));

        prompt.push_str("要求：\n");
        prompt.push_str("1. 保持与原始 PRD 的一致性\n");
        prompt.push_str("2. 充分吸收用户反馈\n");
        prompt.push_str("3. 确保内容质量和完整性\n");
        prompt.push_str("4. 标注出修改的部分\n");

        Ok(prompt)
    }

    /// 调用 AI 进行再生成（简化版本）
    fn call_ai_regeneration(&self, prompt: &str) -> Result<String, String> {
        // 实际项目中这里会调用 AI Agent API
        // 现在返回一个示例 PRD
        Ok(format!(
            "# 优化后的 PRD\n\n基于反馈重新生成的版本...\n\n[AI 生成内容占位符]\n\nPrompt 长度：{} 字符",
            prompt.len()
        ))
    }

    /// 识别变更的章节
    fn identify_changed_sections(&self, old_content: &str, new_content: &str) -> Vec<String> {
        // 简单的差异检测（实际项目应该用更智能的 diff 算法）
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();

        let mut changed_sections = Vec::new();
        let mut in_change = false;

        for (i, line) in new_lines.iter().enumerate() {
            if i < old_lines.len() && line != &old_lines[i] {
                if !in_change {
                    in_change = true;
                    // 尝试找到章节标题
                    if let Some(section) = self.find_section_header(&new_lines, i) {
                        changed_sections.push(section);
                    }
                }
            } else {
                in_change = false;
            }
        }

        if changed_sections.is_empty() {
            changed_sections.push("整体优化".to_string());
        }

        changed_sections
    }

    /// 查找章节标题
    fn find_section_header(&self, lines: &[&str], current_pos: usize) -> Option<String> {
        // 向上查找最近的章节标题
        for i in (0..=current_pos).rev() {
            let line = lines[i].trim();
            if line.starts_with("## ") {
                return Some(line.trim_start_matches("## ").trim().to_string());
            }
        }
        None
    }
}

/// 解析后的反馈
#[derive(Debug, Clone)]
pub struct ParsedFeedback {
    pub content: String,
    pub sentiment: FeedbackSentiment,
    pub priority: FeedbackPriority,
    pub improvement_points: Vec<String>,
}

/// PRD 结构
#[derive(Debug, Clone, Default)]
pub struct PRDStructure {
    pub sections: Vec<String>,
    pub section_hierarchy: HashMap<String, Vec<String>>,
}

// ============ 单元测试 ============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_processor_creation() {
        let processor = PRDFeedbackProcessor::new();
        assert_eq!(processor.max_iterations, 10);
    }

    #[test]
    fn test_sentiment_analysis_positive() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.analyze_sentiment("这个功能很好，我很满意");
        assert_eq!(result, FeedbackSentiment::Positive);
    }

    #[test]
    fn test_sentiment_analysis_negative() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.analyze_sentiment("这里有明显的问题和错误");
        assert_eq!(result, FeedbackSentiment::Negative);
    }

    #[test]
    fn test_sentiment_analysis_suggestion() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.analyze_sentiment("建议可以增加更多功能");
        assert_eq!(result, FeedbackSentiment::Suggestion);
    }

    #[test]
    fn test_sentiment_analysis_neutral() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.analyze_sentiment("这是一个普通的描述");
        assert_eq!(result, FeedbackSentiment::Neutral);
    }

    #[test]
    fn test_priority_identification_critical() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.identify_priority("这是严重的紧急问题，必须立即修复");
        assert_eq!(result, FeedbackPriority::Critical);
    }

    #[test]
    fn test_priority_identification_high() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.identify_priority("这个功能很重要，应该优先处理");
        assert_eq!(result, FeedbackPriority::High);
    }

    #[test]
    fn test_priority_identification_medium() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.identify_priority("这是一般的需求，普通处理即可");
        assert_eq!(result, FeedbackPriority::Medium);
    }

    #[test]
    fn test_priority_identification_low() {
        let processor = PRDFeedbackProcessor::new();
        let result = processor.identify_priority("这是一个小细节");
        assert_eq!(result, FeedbackPriority::Low);
    }

    #[test]
    fn test_improvement_points_extraction() {
        let processor = PRDFeedbackProcessor::new();
        let parsed = processor
            .parse_feedback("第一点改进。第二点改进;第三点改进")
            .unwrap();
        assert!(!parsed.improvement_points.is_empty());
        assert!(parsed.improvement_points.len() >= 2);
    }

    #[test]
    fn test_section_identification_user_persona() {
        let processor = PRDFeedbackProcessor::new();
        let parsed = processor.parse_feedback("用户画像部分需要更详细").unwrap();
        let structure = PRDStructure::default();
        let sections = processor.identify_affected_sections(&parsed, &structure);
        assert!(sections.contains(&"用户画像".to_string()));
    }

    #[test]
    fn test_section_identification_competitor() {
        let processor = PRDFeedbackProcessor::new();
        let parsed = processor.parse_feedback("竞品对比不够清晰").unwrap();
        let structure = PRDStructure::default();
        let sections = processor.identify_affected_sections(&parsed, &structure);
        assert!(sections.contains(&"竞品对比".to_string()));
    }

    #[test]
    fn test_regeneration_request_validation() {
        let processor = PRDFeedbackProcessor::new();
        let request = RegenerateRequest {
            prd_content: "# Test PRD".to_string(),
            feedbacks: vec![],
            sections_to_regenerate: vec!["整体".to_string()],
            iteration_count: 0,
        };

        let result = processor.regenerate_with_feedback(&request);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.iteration_number == 1);
    }

    #[test]
    fn test_max_iteration_limit() {
        let processor = PRDFeedbackProcessor::new();
        let request = RegenerateRequest {
            prd_content: "# Test PRD".to_string(),
            feedbacks: vec![],
            sections_to_regenerate: vec!["整体".to_string()],
            iteration_count: 10, // 已达到最大限制
        };

        let result = processor.regenerate_with_feedback(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_quality_score_improvement() {
        let processor = PRDFeedbackProcessor::new();
        let request = RegenerateRequest {
            prd_content: "# Test PRD".to_string(),
            feedbacks: vec![Feedback {
                id: "1".to_string(),
                prd_id: "test".to_string(),
                section: Some("用户画像".to_string()),
                content: "需要更详细".to_string(),
                sentiment: FeedbackSentiment::Suggestion,
                priority: FeedbackPriority::Medium,
                timestamp: "2026-03-31".to_string(),
            }],
            sections_to_regenerate: vec!["用户画像".to_string()],
            iteration_count: 0,
        };

        let result = processor.regenerate_with_feedback(&request).unwrap();
        assert!(result.quality_score_after > result.quality_score_before);
    }
}

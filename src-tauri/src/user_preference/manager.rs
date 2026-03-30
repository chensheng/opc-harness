/// 用户偏好管理器（简化版）
/// 
/// 用于学习和应用用户的 PRD 偏好
/// 支持偏好收集、分析和应用

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 用户偏好数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    /// 偏好 ID
    pub id: String,
    /// 用户 ID（简化为固定值）
    pub user_id: String,
    /// 偏好类型
    pub preference_type: String,
    /// 偏好数据
    pub preference_data: serde_json::Value,
    /// 置信度分数 (0-1)
    pub confidence_score: f64,
}

/// 偏好模型
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PreferenceModel {
    /// 偏好的章节顺序
    pub preferred_structure: Vec<String>,
    /// 偏好的技术栈
    pub preferred_tech_stack: Vec<String>,
    /// 功能复杂度偏好 (0-1)
    pub preferred_feature_complexity: f64,
    /// 详细程度偏好 (0-1)
    pub preferred_detail_level: f64,
    /// 常见修改模式
    pub common_modifications: Vec<Modification>,
    /// 反馈关键词
    pub feedback_keywords: Vec<String>,
}

/// 修改模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modification {
    /// 修改类型
    pub modification_type: String,
    /// 修改内容
    pub content: String,
    /// 出现次数
    pub count: usize,
}

/// 反馈历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// 反馈内容
    pub content: String,
    /// 时间戳
    pub timestamp: i64,
    /// 反馈类型
    pub feedback_type: String,
}

/// 用户偏好管理器
pub struct UserPreferenceManager {
    /// 当前偏好模型
    model: PreferenceModel,
    /// 反馈历史
    feedback_history: Vec<Feedback>,
}

impl Default for UserPreferenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPreferenceManager {
    /// 创建新的偏好管理器
    pub fn new() -> Self {
        Self {
            model: PreferenceModel::default(),
            feedback_history: Vec::new(),
        }
    }

    /// 加载用户偏好
    pub fn load_preferences(&self) -> Result<PreferenceModel, String> {
        // 简化实现：返回当前模型
        Ok(self.model.clone())
    }

    /// 保存用户偏好
    pub fn save_preferences(&mut self, model: &PreferenceModel) -> Result<(), String> {
        self.model = model.clone();
        Ok(())
    }

    /// 从反馈中分析偏好
    pub fn analyze_from_feedback(&mut self, feedback_history: &[Feedback]) -> PreferenceModel {
        self.feedback_history = feedback_history.to_vec();
        
        let mut model = PreferenceModel::default();
        
        // 统计反馈关键词
        let mut keyword_counts: HashMap<String, usize> = HashMap::new();
        
        for feedback in feedback_history {
            let content_lower = feedback.content.to_lowercase();
            
            // 检测复杂度偏好
            if content_lower.contains("添加") || content_lower.contains("增加") || content_lower.contains("更多") {
                model.preferred_feature_complexity += 0.1;
                increment_keyword_count(&mut keyword_counts, "添加");
            }
            
            if content_lower.contains("简化") || content_lower.contains("减少") || content_lower.contains("精简") {
                model.preferred_feature_complexity -= 0.1;
                increment_keyword_count(&mut keyword_counts, "简化");
            }
            
            // 检测详细程度偏好
            if content_lower.contains("详细") || content_lower.contains("具体") || content_lower.contains("展开") {
                model.preferred_detail_level += 0.1;
                increment_keyword_count(&mut keyword_counts, "详细");
            }
            
            if content_lower.contains("简洁") || content_lower.contains("简单") || content_lower.contains("概括") {
                model.preferred_detail_level -= 0.1;
                increment_keyword_count(&mut keyword_counts, "简洁");
            }
            
            // 检测技术栈偏好
            if content_lower.contains("react") {
                model.preferred_tech_stack.push("React".to_string());
                increment_keyword_count(&mut keyword_counts, "React");
            }
            
            if content_lower.contains("rust") {
                model.preferred_tech_stack.push("Rust".to_string());
                increment_keyword_count(&mut keyword_counts, "Rust");
            }
            
            if content_lower.contains("python") {
                model.preferred_tech_stack.push("Python".to_string());
                increment_keyword_count(&mut keyword_counts, "Python");
            }
        }
        
        // 归一化到 0-1 范围
        model.preferred_feature_complexity = model.preferred_feature_complexity.clamp(0.0, 1.0);
        model.preferred_detail_level = model.preferred_detail_level.clamp(0.0, 1.0);
        
        // 去重技术栈
        model.preferred_tech_stack.sort();
        model.preferred_tech_stack.dedup();
        
        // 提取高频关键词
        model.feedback_keywords = keyword_counts.into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(keyword, _)| keyword)
            .collect();
        
        self.model = model.clone();
        model
    }

    /// 应用偏好到 PRD
    pub fn apply_preferences(&self, prd_json: &str) -> Result<String, String> {
        // 解析 PRD
        let mut prd: serde_json::Value = serde_json::from_str(prd_json)
            .map_err(|e| format!("解析 PRD 失败：{}", e))?;
        
        // 应用技术栈偏好
        if !self.model.preferred_tech_stack.is_empty() {
            if let Some(tech_stack) = prd.get_mut("techStack") {
                if tech_stack.is_array() {
                    *tech_stack = serde_json::to_value(&self.model.preferred_tech_stack).unwrap();
                }
            }
        }
        
        // 应用复杂度偏好（简化实现：调整功能数量）
        if let Some(features) = prd.get_mut("coreFeatures") {
            if let Some(feature_array) = features.as_array_mut() {
                let target_count = match self.model.preferred_feature_complexity {
                    x if x < 0.3 => 3,
                    x if x < 0.6 => 5,
                    _ => 8,
                };
                
                while feature_array.len() < target_count {
                    feature_array.push(serde_json::Value::String("基于偏好生成的功能".to_string()));
                }
            }
        }
        
        // 序列化回 JSON
        serde_json::to_string(&prd).map_err(|e| format!("序列化 PRD 失败：{}", e))
    }

    /// 获取当前偏好模型
    pub fn get_model(&self) -> &PreferenceModel {
        &self.model
    }
}

/// 辅助函数：增加关键词计数
fn increment_keyword_count(counts: &mut HashMap<String, usize>, keyword: &str) {
    *counts.entry(keyword.to_string()).or_insert(0) += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UserPreferenceManager::new();
        assert_eq!(manager.feedback_history.len(), 0);
        assert_eq!(manager.model.preferred_feature_complexity, 0.0);
    }

    #[test]
    fn test_analyze_addition_preference() {
        let mut manager = UserPreferenceManager::new();
        
        let feedbacks = vec![
            Feedback {
                content: "请添加更多功能".to_string(),
                timestamp: 1000,
                feedback_type: "modification".to_string(),
            },
            Feedback {
                content: "需要增加用户管理".to_string(),
                timestamp: 1001,
                feedback_type: "modification".to_string(),
            },
        ];

        let model = manager.analyze_from_feedback(&feedbacks);
        
        assert!(model.preferred_feature_complexity > 0.0);
        assert!(model.feedback_keywords.contains(&"添加".to_string()));
    }

    #[test]
    fn test_analyze_simplification_preference() {
        let mut manager = UserPreferenceManager::new();
        
        let feedbacks = vec![
            Feedback {
                content: "简化功能，太复杂了".to_string(),
                timestamp: 1000,
                feedback_type: "modification".to_string(),
            },
            Feedback {
                content: "减少不必要的功能".to_string(),
                timestamp: 1001,
                feedback_type: "modification".to_string(),
            },
        ];

        let model = manager.analyze_from_feedback(&feedbacks);
        
        assert!(model.preferred_feature_complexity < 0.5);
        assert!(model.feedback_keywords.contains(&"简化".to_string()));
    }

    #[test]
    fn test_analyze_tech_stack_preference() {
        let mut manager = UserPreferenceManager::new();
        
        let feedbacks = vec![
            Feedback {
                content: "使用 React 和 Rust 实现".to_string(),
                timestamp: 1000,
                feedback_type: "modification".to_string(),
            },
            Feedback {
                content: "前端用 React，后端用 Python".to_string(),
                timestamp: 1001,
                feedback_type: "modification".to_string(),
            },
        ];

        let model = manager.analyze_from_feedback(&feedbacks);
        
        assert!(model.preferred_tech_stack.contains(&"React".to_string()));
        assert!(model.preferred_tech_stack.contains(&"Rust".to_string()));
        assert!(model.preferred_tech_stack.contains(&"Python".to_string()));
    }

    #[test]
    fn test_apply_preferences() {
        let mut manager = UserPreferenceManager::new();
        
        // 设置偏好
        manager.model.preferred_tech_stack = vec!["React".to_string(), "Rust".to_string()];
        manager.model.preferred_feature_complexity = 0.8;
        
        let prd_json = r#"{"title": "测试", "coreFeatures": ["功能 1"], "techStack": ["Old"]}"#;
        
        let result = manager.apply_preferences(prd_json).unwrap();
        
        // 验证技术栈被替换
        assert!(result.contains("React"));
        assert!(result.contains("Rust"));
        
        // 验证功能数量增加
        let result_value: serde_json::Value = serde_json::from_str(&result).unwrap();
        let features = result_value.get("coreFeatures").unwrap().as_array().unwrap();
        assert!(features.len() >= 5);
    }

    #[test]
    fn test_save_and_load_preferences() {
        let mut manager = UserPreferenceManager::new();
        
        let model = PreferenceModel {
            preferred_feature_complexity: 0.7,
            preferred_detail_level: 0.6,
            preferred_tech_stack: vec!["React".to_string()],
            ..Default::default()
        };

        manager.save_preferences(&model).unwrap();
        
        let loaded = manager.load_preferences().unwrap();
        assert_eq!(loaded.preferred_feature_complexity, 0.7);
    }
}

/// PRD 迭代管理器（简化版）
///
/// 用于管理 PRD 的迭代优化流程
/// 支持版本管理、反馈整合和差异对比
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PRD 版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDVersion {
    /// 版本 ID
    pub version_id: String,
    /// 时间戳
    pub timestamp: i64,
    /// PRD 内容（JSON 字符串）
    pub prd_json: String,
    /// 迭代轮数
    pub iteration_number: u8,
    /// 用户反馈
    pub feedback: Option<String>,
}

/// 版本差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDDiff {
    /// 新增的功能
    pub added_features: Vec<String>,
    /// 移除的功能
    pub removed_features: Vec<String>,
    /// 修改的字段数量
    pub modified_fields_count: usize,
}

/// 迭代历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationHistory {
    /// 当前版本 ID
    pub current_version_id: String,
    /// 所有版本
    pub versions: Vec<PRDVersion>,
}

/// 迭代请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationRequest {
    /// 当前 PRD 内容（JSON）
    pub current_prd_json: String,
    /// 用户反馈
    pub user_feedback: String,
    /// 质量报告摘要
    pub quality_summary: Option<String>,
}

/// 迭代响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationResponse {
    /// 新版本 ID
    pub new_version_id: String,
    /// 优化后的 PRD（JSON）
    pub optimized_prd_json: String,
    /// 版本差异
    pub diff: PRDDiff,
    /// 迭代轮数
    pub iteration_number: u8,
}

/// 创建初始版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInitialVersionRequest {
    /// PRD 内容（JSON）
    pub prd_json: String,
}

/// 创建初始版本响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInitialVersionResponse {
    /// 版本 ID
    pub version_id: String,
}

/// 获取历史请求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GetIterationHistoryRequest {}

/// 获取历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GetIterationHistoryResponse {
    /// 迭代历史
    pub history: IterationHistory,
}

/// 回滚请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRequest {
    /// 版本 ID
    pub version_id: String,
}

/// 回滚响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResponse {
    /// 回滚后的版本
    pub version: PRDVersion,
}

/// PRD 迭代管理器
pub struct PRDIterationManager {
    /// 迭代历史
    history: IterationHistory,
}

impl Default for PRDIterationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PRDIterationManager {
    /// 创建新的迭代管理器
    pub fn new() -> Self {
        Self {
            history: IterationHistory {
                current_version_id: String::new(),
                versions: Vec::new(),
            },
        }
    }

    /// 创建初始版本
    pub fn create_initial_version(&mut self, prd_json: &str) -> String {
        let version_id = Uuid::new_v4().to_string();

        let version = PRDVersion {
            version_id: version_id.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            prd_json: prd_json.to_string(),
            iteration_number: 0,
            feedback: None,
        };

        self.history.versions.push(version);
        self.history.current_version_id = version_id.clone();

        version_id
    }

    /// 执行迭代优化
    pub fn iterate_with_feedback(
        &mut self,
        request: &IterationRequest,
    ) -> Result<IterationResponse, String> {
        // 1. 解析当前 PRD
        let current_prd: serde_json::Value = serde_json::from_str(&request.current_prd_json)
            .map_err(|e| format!("解析 PRD 失败：{}", e))?;

        // 2. 基于反馈生成优化版本（简化实现：模拟 AI 优化）
        let optimized_prd = self.simulate_optimization(&current_prd, &request.user_feedback);

        // 3. 计算版本差异
        let diff = self.calculate_diff(&current_prd, &optimized_prd);

        // 4. 创建新版本
        let new_version_id = Uuid::new_v4().to_string();
        let new_iteration_number = self.history.versions.len() as u8;

        let optimized_prd_json =
            serde_json::to_string(&optimized_prd).map_err(|e| format!("序列化 PRD 失败：{}", e))?;

        let version = PRDVersion {
            version_id: new_version_id.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            prd_json: optimized_prd_json.clone(),
            iteration_number: new_iteration_number,
            feedback: Some(request.user_feedback.clone()),
        };

        self.history.versions.push(version);
        self.history.current_version_id = new_version_id.clone();

        Ok(IterationResponse {
            new_version_id,
            optimized_prd_json,
            diff,
            iteration_number: new_iteration_number,
        })
    }

    /// 获取迭代历史
    #[allow(dead_code)]
    pub fn get_history(&self) -> &IterationHistory {
        &self.history
    }

    /// 计算版本差异
    pub fn calculate_diff(
        &self,
        old_prd: &serde_json::Value,
        new_prd: &serde_json::Value,
    ) -> PRDDiff {
        let old_features = old_prd
            .get("coreFeatures")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        let new_features = new_prd
            .get("coreFeatures")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        let added_features = new_features
            .iter()
            .filter(|f| !old_features.contains(f))
            .map(|s| s.to_string())
            .collect();

        let removed_features = old_features
            .iter()
            .filter(|f| !new_features.contains(f))
            .map(|s| s.to_string())
            .collect();

        // 简单计算修改的字段数量
        let mut modified_count = 0;
        let fields = [
            "title",
            "overview",
            "targetUsers",
            "techStack",
            "estimatedEffort",
        ];
        for field in fields {
            if old_prd.get(field) != new_prd.get(field) {
                modified_count += 1;
            }
        }

        PRDDiff {
            added_features,
            removed_features,
            modified_fields_count: modified_count,
        }
    }

    /// 模拟 AI 优化（简化实现）
    fn simulate_optimization(
        &self,
        current_prd: &serde_json::Value,
        feedback: &str,
    ) -> serde_json::Value {
        // 实际项目中应该调用 AI API 进行优化
        // 这里只是简单复制当前 PRD
        let mut optimized = current_prd.clone();

        // 如果反馈中提到"添加"，就在功能列表中添加一项
        if feedback.contains("添加") || feedback.contains("增加") {
            if let Some(features) = optimized.get_mut("coreFeatures") {
                if let Some(arr) = features.as_array_mut() {
                    arr.push(serde_json::Value::String(
                        "基于用户反馈新增的功能".to_string(),
                    ));
                }
            }
        }

        optimized
    }

    /// 回滚到指定版本
    #[allow(dead_code)]
    pub fn rollback_to_version(&mut self, version_id: &str) -> Result<&PRDVersion, String> {
        let version = self
            .history
            .versions
            .iter()
            .find(|v| v.version_id == version_id)
            .ok_or_else(|| format!("版本 {} 不存在", version_id))?;

        self.history.current_version_id = version_id.to_string();
        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_manager_creation() {
        let manager = PRDIterationManager::new();
        assert_eq!(manager.history.versions.len(), 0);
    }

    #[test]
    fn test_create_initial_version() {
        let mut manager = PRDIterationManager::new();
        let prd_json = r#"{"title": "测试产品"}"#;

        let version_id = manager.create_initial_version(prd_json);

        assert!(!version_id.is_empty());
        assert_eq!(manager.history.versions.len(), 1);
        assert_eq!(manager.history.current_version_id, version_id);
    }

    #[test]
    fn test_iterate_with_feedback() {
        let mut manager = PRDIterationManager::new();

        // 创建初始版本
        let initial_prd = json!({
            "title": "测试产品",
            "coreFeatures": ["功能 1", "功能 2"]
        });
        let initial_json = serde_json::to_string(&initial_prd).unwrap();
        manager.create_initial_version(&initial_json);

        // 执行迭代
        let request = IterationRequest {
            current_prd_json: initial_json,
            user_feedback: "请添加更多功能".to_string(),
            quality_summary: None,
        };

        let response = manager.iterate_with_feedback(&request).unwrap();

        assert!(!response.new_version_id.is_empty());
        assert_eq!(response.iteration_number, 1);
        assert!(response.diff.added_features.len() > 0);
        assert_eq!(manager.history.versions.len(), 2);
    }

    #[test]
    fn test_calculate_diff() {
        let manager = PRDIterationManager::new();

        let old_prd = json!({
            "title": "旧产品",
            "coreFeatures": ["功能 1", "功能 2"]
        });

        let new_prd = json!({
            "title": "新产品",
            "coreFeatures": ["功能 1", "功能 3"]
        });

        let diff = manager.calculate_diff(&old_prd, &new_prd);

        assert!(diff.added_features.contains(&"功能 3".to_string()));
        assert!(diff.removed_features.contains(&"功能 2".to_string()));
        assert_eq!(diff.modified_fields_count, 1); // title 改变了
    }

    #[test]
    fn test_rollback() {
        let mut manager = PRDIterationManager::new();

        // 创建两个版本
        let v1_id = manager.create_initial_version(r#"{"title": "V1"}"#);
        manager
            .iterate_with_feedback(&IterationRequest {
                current_prd_json: r#"{"title": "V1"}"#.to_string(),
                user_feedback: "优化".to_string(),
                quality_summary: None,
            })
            .unwrap();

        // 回滚到 V1 - 使用作用域限制借用生命周期
        let rolled_back_prd_json = {
            let rolled_back = manager.rollback_to_version(&v1_id).unwrap();
            rolled_back.prd_json.clone()
        };

        assert_eq!(manager.history.current_version_id, v1_id);
        assert!(rolled_back_prd_json.contains("V1"));
    }

    #[test]
    fn test_multiple_iterations() {
        let mut manager = PRDIterationManager::new();

        // 创建初始版本
        let initial_prd = json!({
            "title": "初始版本",
            "coreFeatures": ["功能 1"]
        });
        let initial_json = serde_json::to_string(&initial_prd).unwrap();
        manager.create_initial_version(&initial_json);

        // 执行 3 轮迭代
        for i in 1..=3 {
            let request = IterationRequest {
                current_prd_json: initial_json.clone(),
                user_feedback: format!("第 {} 轮优化", i),
                quality_summary: None,
            };

            let response = manager.iterate_with_feedback(&request).unwrap();
            assert_eq!(response.iteration_number, i as u8);
        }

        // 验证总共有 4 个版本（1 个初始 + 3 个迭代）
        assert_eq!(manager.history.versions.len(), 4);
    }
}

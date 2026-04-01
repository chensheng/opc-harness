//! PRD 文档解析器
//! 
//! VC-006: 实现 PRD 文档解析器
//! 负责从 PRD 文档中提取关键信息并转化为结构化数据

use crate::ai::AIConfig;

// ============================================================================
// 提示词模板
// ============================================================================

/// PRD 解析提示词模板
pub const PRD_PARSING_PROMPT: &str = r#"你是一位经验丰富的技术产品经理，擅长从 PRD 文档中提取关键信息并转化为可执行的技术任务。

## 任务
请分析以下 PRD 文档，提取关键信息并生成结构化的解析结果。

## PRD 文档内容
{prd_content}

## 提取要求
请按照以下 JSON 格式输出解析结果：

```json
{{
  "product_name": "产品名称",
  "product_description": "产品描述（100-200 字）",
  "target_users": ["用户群体 1", "用户群体 2"],
  "core_features": ["核心功能 1", "核心功能 2"],
  "non_functional_requirements": ["性能要求", "安全要求"],
  "suggested_tech_stack": ["技术栈 1", "技术栈 2"],
  "confidence_score": 0.95
}}
```

## 字段说明
- **product_name**: 从 PRD 中提取的产品名称
- **product_description**: 产品的核心价值主张
- **target_users**: 目标用户群体列表
- **core_features**: 核心功能特性列表（3-5 个）
- **non_functional_requirements**: 非功能性需求（性能、安全、可用性等）
- **suggested_tech_stack**: 推荐的技术栈
- **confidence_score**: 解析置信度（0.0-1.0），基于 PRD 的完整性和清晰度

## 注意事项
1. 保持客观准确，忠实于原始 PRD 内容
2. 如果某些信息在 PRD 中未明确提及，可以合理推断但需降低置信度
3. 技术栈建议应考虑现代、主流的技术选择
4. 输出必须是有效的 JSON 格式
"#;

/// 任务分解提示词模板
pub const TASK_DECOMPOSITION_PROMPT: &str = r#"你是一位资深的项目经理，擅长将复杂的产品需求分解为可执行的开发任务。

## 任务
请根据以下产品需求和功能特性，将其分解为独立的开发任务（Issues）。

## 产品信息
- **产品名称**: {product_name}
- **产品描述**: {product_description}

## 核心功能列表
{core_features}

## 技术要求
{tech_stack}

## 分解要求
请将每个核心功能分解为一个或多个 Issues，每个 Issue 应包含：
1. **清晰的标题**: 简洁描述任务内容
2. **详细描述**: 包含验收标准
3. **优先级**: P0（必须）、P1（应该）、P2（可以）
4. **预估工时**: 以小时为单位
5. **依赖关系**: 如果有前置任务，请指明

## 输出格式
请以 JSON 数组格式输出 Issues 列表：

```json
[
  {{
    "issue_id": "ISSUE-001",
    "title": "任务标题",
    "description": "详细任务描述",
    "priority": "high",
    "estimated_hours": 4.0,
    "status": "pending",
    "assignee": null,
    "labels": ["feature", "backend"]
  }}
]
```

## 优先级映射
- P0 → "high"
- P1 → "medium"  
- P2 → "low"

## 注意事项
1. 每个 Issue 应该是独立可执行的
2. 预估工时应考虑开发、测试和修复时间
3. 识别关键路径和依赖关系
4. 优先保证核心功能（P0）的实现
"#;

// ============================================================================
// 核心数据结构
// ============================================================================

/// PRD 解析结果
#[derive(Debug, Clone)]
pub struct PRDResult {
    /// 产品名称
    pub product_name: String,
    /// 产品描述
    pub product_description: String,
    /// 目标用户群体
    pub target_users: Vec<String>,
    /// 核心功能列表
    pub core_features: Vec<String>,
    /// 非功能性需求
    pub non_functional_requirements: Vec<String>,
    /// 推荐技术栈
    pub suggested_tech_stack: Vec<String>,
    /// 解析置信度
    pub confidence_score: f32,
    /// 识别出的 Issues（任务分解后填充）
    pub identified_issues: Vec<crate::agent::messages::Issue>,
}

// ============================================================================
// PRD 解析器实现
// ============================================================================

/// PRD 解析器配置
#[derive(Debug, Clone)]
pub struct PRDParserConfig {
    /// AI 服务配置
    pub ai_config: AIConfig,
    /// 是否使用流式输出
    pub use_streaming: bool,
}

/// PRD 解析器
pub struct PRDParser {
    #[allow(dead_code)]
    config: PRDParserConfig,
}

impl PRDParser {
    /// 创建新的 PRD 解析器
    pub fn new(config: PRDParserConfig) -> Self {
        Self { config }
    }

    /// 解析 PRD 文档
    pub async fn parse_prd(&self, prd_content: &str) -> Result<PRDResult, String> {
        // 1. 构建提示词
        let prompt = PRD_PARSING_PROMPT.replace("{prd_content}", prd_content);
        
        // 2. 调用 AI 服务进行解析（TODO: 实际调用）
        let ai_response = self.call_ai_service(&prompt).await?;
        
        // 3. 解析 AI 返回的 JSON 结果
        let parsed_data = self.parse_ai_response(&ai_response)?;
        
        // 4. 转换为 PRDResult
        let result = self.convert_to_prd_result(parsed_data)?;
        
        Ok(result)
    }

    /// 任务分解：将 PRD 分解为 Issues
    pub async fn decompose_tasks(
        &self,
        product_name: &str,
        product_description: &str,
        core_features: &[String],
        tech_stack: &[String],
    ) -> Result<Vec<crate::agent::messages::Issue>, String> {
        // 1. 构建提示词
        let mut prompt = TASK_DECOMPOSITION_PROMPT
            .replace("{product_name}", product_name);
        prompt = prompt
            .replace("{product_description}", product_description);
        
        let features_json = serde_json::to_string(core_features)
            .map_err(|e| format!("序列化功能列表失败：{}", e))?;
        prompt = prompt.replace("{core_features}", &features_json);
        
        let tech_json = serde_json::to_string(tech_stack)
            .map_err(|e| format!("序列化技术栈失败：{}", e))?;
        prompt = prompt.replace("{tech_stack}", &tech_json);
        
        // 2. 调用 AI 服务
        let ai_response = self.call_ai_service(&prompt).await?;
        
        // 3. 解析 Issues
        let issues = self.parse_issues_response(&ai_response)?;
        
        Ok(issues)
    }

    /// 调用 AI 服务（占位符）
    async fn call_ai_service(&self, _prompt: &str) -> Result<String, String> {
        // TODO: 实际调用 AI 服务
        log::info!("调用 AI 服务进行 PRD 解析...");
        
        // 模拟 AI 响应延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // 返回 Mock 数据用于测试
        Ok("{\"status\":\"mock\",\"message\":\"AI service not yet integrated\"}".to_string())
    }

    /// 解析 AI 响应
    fn parse_ai_response(&self, response: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(response)
            .map_err(|e| format!("解析 AI 响应失败：{}", e))
    }

    /// 转换为 PRDResult
    fn convert_to_prd_result(&self, data: serde_json::Value) -> Result<PRDResult, String> {
        let product_name = data["product_name"]
            .as_str()
            .unwrap_or("未命名产品")
            .to_string();
        
        let product_description = data["product_description"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        let target_users = data["target_users"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        let core_features = data["core_features"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        let non_functional_requirements = data["non_functional_requirements"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        let suggested_tech_stack = data["suggested_tech_stack"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        let confidence_score = data["confidence_score"]
            .as_f64()
            .unwrap_or(0.5) as f32;
        
        Ok(PRDResult {
            product_name,
            product_description,
            target_users,
            core_features,
            non_functional_requirements,
            suggested_tech_stack,
            confidence_score,
            identified_issues: Vec::new(),
        })
    }

    /// 解析 Issues 响应
    fn parse_issues_response(&self, response: &str) -> Result<Vec<crate::agent::messages::Issue>, String> {
        use crate::agent::messages::{Issue, Priority};
        
        let issues: Vec<serde_json::Value> = serde_json::from_str(response)
            .map_err(|e| format!("解析 Issues 失败：{}", e))?;
        
        let mut result = Vec::new();
        
        for issue_data in issues.iter() {
            let title = issue_data["title"]
                .as_str()
                .unwrap_or("未命名任务")
                .to_string();
            
            let description = issue_data["description"]
                .as_str()
                .unwrap_or("")
                .to_string();
            
            let priority_str = issue_data["priority"]
                .as_str()
                .unwrap_or("medium");
            
            let priority = match priority_str {
                "high" => Priority::High,
                "low" => Priority::Low,
                _ => Priority::Medium,
            };
            
            let estimated_hours = issue_data["estimated_hours"]
                .as_f64()
                .map(|h| h as f32);
            
            let issue = Issue::new(title, description, priority)
                .with_estimated_hours(estimated_hours.unwrap_or(4.0));
            
            result.push(issue);
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::messages::Priority;

    #[test]
    fn test_prd_parser_creation() {
        let config = PRDParserConfig {
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "sk-test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            use_streaming: false,
        };
        
        let _parser = PRDParser::new(config);
        
        // 验证解析器创建成功
        assert!(true);
    }

    #[test]
    fn test_prd_result_structure() {
        let result = PRDResult {
            product_name: "智能营销平台".to_string(),
            product_description: "基于 AI 的自动化营销系统".to_string(),
            target_users: vec!["营销人员".to_string(), "产品经理".to_string()],
            core_features: vec![
                "PRD 自动生成".to_string(),
                "代码自动编写".to_string(),
            ],
            non_functional_requirements: vec![
                "响应时间 < 2s".to_string(),
                "支持 1000+ 并发".to_string(),
            ],
            suggested_tech_stack: vec![
                "React".to_string(),
                "Rust".to_string(),
                "Tauri".to_string(),
            ],
            confidence_score: 0.95,
            identified_issues: Vec::new(),
        };
        
        assert_eq!(result.product_name, "智能营销平台");
        assert_eq!(result.target_users.len(), 2);
        assert_eq!(result.core_features.len(), 2);
        assert_eq!(result.suggested_tech_stack.len(), 3);
        assert_eq!(result.confidence_score, 0.95);
    }

    #[test]
    fn test_parse_ai_response_success() {
        let parser = PRDParser::new(PRDParserConfig {
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            use_streaming: false,
        });
        
        let mock_response = r#"{
            "product_name": "Test Product",
            "product_description": "A test product",
            "target_users": ["User1", "User2"],
            "core_features": ["Feature1", "Feature2"],
            "non_functional_requirements": ["Fast", "Secure"],
            "suggested_tech_stack": ["React", "Node.js"],
            "confidence_score": 0.9
        }"#;
        
        let result = parser.parse_ai_response(mock_response);
        
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["product_name"], "Test Product");
        assert_eq!(value["confidence_score"], 0.9);
    }

    #[test]
    fn test_parse_ai_response_invalid_json() {
        let parser = PRDParser::new(PRDParserConfig {
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            use_streaming: false,
        });
        
        let invalid_response = "This is not valid JSON";
        let result = parser.parse_ai_response(invalid_response);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("解析 AI 响应失败"));
    }

    #[test]
    fn test_parse_issues_response() {
        let parser = PRDParser::new(PRDParserConfig {
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            use_streaming: false,
        });
        
        let mock_issues = r#"[
            {
                "issue_id": "ISSUE-001",
                "title": "实现用户登录",
                "description": "JWT 认证",
                "priority": "high",
                "estimated_hours": 4.0
            },
            {
                "issue_id": "ISSUE-002",
                "title": "实现数据看板",
                "description": "数据可视化",
                "priority": "medium",
                "estimated_hours": 8.0
            }
        ]"#;
        
        let result = parser.parse_issues_response(mock_issues);
        
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].title, "实现用户登录");
        assert_eq!(issues[0].priority, Priority::High);
        assert_eq!(issues[1].estimated_hours, Some(8.0));
    }

    #[test]
    fn test_convert_to_prd_result() {
        let parser = PRDParser::new(PRDParserConfig {
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            use_streaming: false,
        });
        
        let mock_data = serde_json::json!({
            "product_name": "Test Platform",
            "product_description": "AI-powered platform",
            "target_users": ["Developers", "Designers"],
            "core_features": ["Auto-generate", "Deploy"],
            "non_functional_requirements": ["Scalable"],
            "suggested_tech_stack": ["React", "Rust"],
            "confidence_score": 0.85
        });
        
        let result = parser.convert_to_prd_result(mock_data);
        
        assert!(result.is_ok());
        let prd_result = result.unwrap();
        assert_eq!(prd_result.product_name, "Test Platform");
        assert_eq!(prd_result.target_users.len(), 2);
        assert_eq!(prd_result.core_features.len(), 2);
        assert_eq!(prd_result.confidence_score, 0.85);
    }

    #[test]
    fn test_prompt_templates_available() {
        // 验证提示词模板已定义
        assert!(!PRD_PARSING_PROMPT.is_empty());
        assert!(!TASK_DECOMPOSITION_PROMPT.is_empty());
        
        // 验证模板包含关键字段
        assert!(PRD_PARSING_PROMPT.contains("product_name"));
        assert!(PRD_PARSING_PROMPT.contains("confidence_score"));
        assert!(TASK_DECOMPOSITION_PROMPT.contains("issue_id"));
        assert!(TASK_DECOMPOSITION_PROMPT.contains("estimated_hours"));
    }
}

//! Initializer Agent 实现
//! 
//! 负责 PRD 文档解析、环境检查、Git 仓库初始化和任务分解

use serde::{Deserialize, Serialize};
use crate::agent::messages::Issue;
use crate::agent::prd_parser::{PRDParser, PRDParserConfig};

/// Initializer Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializerAgentConfig {
    /// Agent ID
    pub agent_id: String,
    /// 项目路径
    pub project_path: String,
    /// AI 服务配置
    pub ai_config: crate::ai::AIConfig,
    /// PRD 文档路径
    pub prd_file_path: Option<String>,
    /// PRD 内容（如果直接传入）
    pub prd_content: Option<String>,
}

/// PRD 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDParseResult {
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
    /// 技术栈建议
    pub suggested_tech_stack: Vec<String>,
    /// 识别出的 Issue 列表
    pub identified_issues: Vec<Issue>,
    /// 解析置信度 (0.0-1.0)
    pub confidence_score: f32,
}

impl PRDParseResult {
    /// 创建新的解析结果
    pub fn new(product_name: String, product_description: String) -> Self {
        Self {
            product_name,
            product_description,
            target_users: Vec::new(),
            core_features: Vec::new(),
            non_functional_requirements: Vec::new(),
            suggested_tech_stack: Vec::new(),
            identified_issues: Vec::new(),
            confidence_score: 0.0,
        }
    }

    /// 设置目标用户
    pub fn with_target_users(mut self, users: Vec<String>) -> Self {
        self.target_users = users;
        self
    }

    /// 设置核心功能
    pub fn with_core_features(mut self, features: Vec<String>) -> Self {
        self.core_features = features;
        self
    }

    /// 设置技术栈
    pub fn with_tech_stack(mut self, stack: Vec<String>) -> Self {
        self.suggested_tech_stack = stack;
        self
    }

    /// 设置识别的 Issues
    pub fn with_issues(mut self, issues: Vec<Issue>) -> Self {
        self.identified_issues = issues;
        self
    }

    /// 设置置信度
    pub fn with_confidence(mut self, score: f32) -> Self {
        self.confidence_score = score.clamp(0.0, 1.0);
        self
    }
}

/// 环境检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentCheckResult {
    /// 是否通过检查
    pub passed: bool,
    /// Git 是否已安装
    pub git_installed: bool,
    /// Git 版本
    pub git_version: Option<String>,
    /// Node.js 是否已安装
    pub node_installed: bool,
    /// Node.js 版本
    pub node_version: Option<String>,
    /// Rust/Cargo 是否已安装
    pub cargo_installed: bool,
    /// Cargo 版本
    pub cargo_version: Option<String>,
    /// 项目目录是否存在
    pub project_dir_exists: bool,
    /// 错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
}

impl EnvironmentCheckResult {
    /// 创建成功的检查结果
    pub fn success() -> Self {
        Self {
            passed: true,
            git_installed: true,
            git_version: None,
            node_installed: true,
            node_version: None,
            cargo_installed: true,
            cargo_version: None,
            project_dir_exists: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// 创建失败的结果
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            passed: false,
            git_installed: false,
            git_version: None,
            node_installed: false,
            node_version: None,
            cargo_installed: false,
            cargo_version: None,
            project_dir_exists: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// 添加 Git 版本信息
    pub fn with_git_version(mut self, version: String) -> Self {
        self.git_installed = true;
        self.git_version = Some(version);
        self
    }

    /// 添加 Node.js 版本信息
    pub fn with_node_version(mut self, version: String) -> Self {
        self.node_installed = true;
        self.node_version = Some(version);
        self
    }

    /// 添加 Cargo 版本信息
    pub fn with_cargo_version(mut self, version: String) -> Self {
        self.cargo_installed = true;
        self.cargo_version = Some(version);
        self
    }

    /// 添加警告
    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// 任务分解结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDecompositionResult {
    /// 是否成功分解
    pub success: bool,
    /// 分解出的 Issue 列表
    pub issues: Vec<Issue>,
    /// 预估总工作量 (小时)
    pub total_estimated_hours: f32,
    /// 建议的开发顺序
    pub suggested_order: Vec<String>,
    /// 依赖关系图
    pub dependencies: Vec<(String, String)>, // (from_issue_id, to_issue_id)
    /// 风险提示
    pub risks: Vec<String>,
}

impl TaskDecompositionResult {
    /// 创建新的分解结果
    pub fn new(issues: Vec<Issue>) -> Self {
        let total_hours: f32 = issues.iter()
            .filter_map(|i| i.estimated_hours)
            .sum();
        
        Self {
            success: true,
            issues,
            total_estimated_hours: total_hours,
            suggested_order: Vec::new(),
            dependencies: Vec::new(),
            risks: Vec::new(),
        }
    }

    /// 设置开发顺序
    pub fn with_suggested_order(mut self, order: Vec<String>) -> Self {
        self.suggested_order = order;
        self
    }

    /// 设置依赖关系
    pub fn with_dependencies(mut self, deps: Vec<(String, String)>) -> Self {
        self.dependencies = deps;
        self
    }

    /// 添加风险提示
    pub fn add_risk(mut self, risk: String) -> Self {
        self.risks.push(risk);
        self
    }
}

/// Initializer Agent 执行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InitializerStatus {
    /// 等待开始
    Pending,
    /// 正在解析 PRD
    ParsingPRD,
    /// 正在检查环境
    CheckingEnvironment,
    /// 正在初始化 Git
    InitializingGit,
    /// 正在分解任务
    DecomposingTasks,
    /// 等待 HITL 审查
    WaitingForHITL,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

/// Initializer Agent 完整结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializerResult {
    /// 是否成功完成
    pub success: bool,
    /// PRD 解析结果
    pub prd_result: Option<PRDParseResult>,
    /// 环境检查结果
    pub env_check: Option<EnvironmentCheckResult>,
    /// Git 初始化结果
    pub git_init_result: Option<bool>,
    /// 任务分解结果
    pub task_decomposition: Option<TaskDecompositionResult>,
    /// HITL 检查点 ID
    pub checkpoint_id: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

impl InitializerResult {
    /// 创建成功的结果
    pub fn success(
        prd_result: PRDParseResult,
        env_check: EnvironmentCheckResult,
        task_decomposition: TaskDecompositionResult,
    ) -> Self {
        Self {
            success: true,
            prd_result: Some(prd_result),
            env_check: Some(env_check),
            git_init_result: Some(true),
            task_decomposition: Some(task_decomposition),
            checkpoint_id: None,
            error: None,
        }
    }

    /// 创建失败的结果
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            prd_result: None,
            env_check: None,
            git_init_result: None,
            task_decomposition: None,
            checkpoint_id: None,
            error: Some(error),
        }
    }
}

/// Initializer Agent 结构体
#[derive(Debug, Clone)]
pub struct InitializerAgent {
    /// 配置信息
    pub config: InitializerAgentConfig,
    /// 当前状态
    pub status: InitializerStatus,
    /// 会话 ID
    pub session_id: String,
}

impl InitializerAgent {
    /// 创建新的 Initializer Agent
    pub fn new(config: InitializerAgentConfig) -> Self {
        Self {
            config,
            status: InitializerStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// 解析 PRD 文档
    /// 
    /// VC-006: 实现 PRD 文档解析器
    pub async fn parse_prd(&mut self) -> Result<PRDParseResult, String> {
        self.status = InitializerStatus::ParsingPRD;
        
        // 1. 获取 PRD 内容
        let prd_content = self.get_prd_content()?;
        
        // 2. 创建 PRD 解析器
        let parser_config = PRDParserConfig {
            ai_config: self.config.ai_config.clone(),
            use_streaming: false,
        };
        let parser = PRDParser::new(parser_config);
        
        // 3. 执行 PRD 解析
        let prd_result = parser.parse_prd(&prd_content).await?;
        
        // 4. 转换为 PRDParseResult
        let parse_result = PRDParseResult::new(
            prd_result.product_name,
            prd_result.product_description,
        )
        .with_target_users(prd_result.target_users)
        .with_core_features(prd_result.core_features)
        .with_tech_stack(prd_result.suggested_tech_stack)
        .with_confidence(prd_result.confidence_score);
        
        self.status = InitializerStatus::CheckingEnvironment;
        
        Ok(parse_result)
    }

    /// 获取 PRD 内容（从文件或参数）
    fn get_prd_content(&self) -> Result<String, String> {
        // 优先使用传入的 PRD 内容
        if let Some(content) = &self.config.prd_content {
            return Ok(content.clone());
        }
        
        // 否则从文件读取
        if let Some(file_path) = &self.config.prd_file_path {
            use std::fs;
            fs::read_to_string(file_path)
                .map_err(|e| format!("读取 PRD 文件失败：{}", e))
        } else {
            Err("未提供 PRD 内容或文件路径".to_string())
        }
    }

    /// 检查环境（占位符，待后续实现）
    pub async fn check_environment(&self) -> Result<EnvironmentCheckResult, String> {
        // TODO: 实现环境检查逻辑
        Err("Not implemented yet".to_string())
    }

    /// 初始化 Git 仓库（占位符，待后续实现）
    pub async fn initialize_git(&self) -> Result<bool, String> {
        // TODO: 实现 Git 初始化逻辑
        Err("Not implemented yet".to_string())
    }

    /// 分解任务为 Issues
    /// 
    /// VC-006: 基于 PRD 解析结果进行任务分解
    pub async fn decompose_tasks(
        &self,
        prd_result: &PRDParseResult,
    ) -> Result<TaskDecompositionResult, String> {
        // 1. 创建 PRD 解析器
        let parser_config = PRDParserConfig {
            ai_config: self.config.ai_config.clone(),
            use_streaming: false,
        };
        let parser = PRDParser::new(parser_config);
        
        // 2. 调用任务分解
        let issues = parser.decompose_tasks(
            &prd_result.product_name,
            &prd_result.product_description,
            &prd_result.core_features,
            &prd_result.suggested_tech_stack,
        ).await?;
        
        // 3. 创建 TaskDecompositionResult
        let result = TaskDecompositionResult::new(issues);
        
        Ok(result)
    }

    /// 执行完整的初始化流程（占位符，待后续实现）
    pub async fn run_initialization(&mut self) -> Result<InitializerResult, String> {
        // TODO: 实现完整初始化流程
        // 1. 解析 PRD
        // 2. 检查环境
        // 3. 初始化 Git
        // 4. 分解任务
        // 5. 触发 CP-002 检查点
        Err("Not implemented yet".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::messages::Priority;

    #[test]
    fn test_prd_parse_result() {
        let mut result = PRDParseResult::new(
            "智能营销平台".to_string(),
            "基于 AI 的自动化营销系统".to_string(),
        );
        
        assert_eq!(result.product_name, "智能营销平台");
        assert_eq!(result.confidence_score, 0.0);
        
        result = result
            .with_target_users(vec!["营销人员".to_string(), "产品经理".to_string()])
            .with_core_features(vec![
                "PRD 自动生成".to_string(),
                "代码自动编写".to_string(),
            ])
            .with_tech_stack(vec!["React".to_string(), "Rust".to_string(), "Tauri".to_string()])
            .with_confidence(0.95);
        
        assert_eq!(result.target_users.len(), 2);
        assert_eq!(result.core_features.len(), 2);
        assert_eq!(result.suggested_tech_stack.len(), 3);
        assert_eq!(result.confidence_score, 0.95);
    }

    #[test]
    fn test_environment_check_result() {
        let result = EnvironmentCheckResult::success()
            .with_git_version("2.40.0".to_string())
            .with_node_version("v20.10.0".to_string())
            .with_cargo_version("1.75.0".to_string())
            .add_warning("npm 版本较旧，建议升级".to_string());
        
        assert!(result.passed);
        assert_eq!(result.git_version, Some("2.40.0".to_string()));
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_task_decomposition_result() {
        let issue1 = Issue::new(
            "实现用户登录".to_string(),
            "JWT 认证".to_string(),
            Priority::High,
        ).with_estimated_hours(4.0);
        
        let issue2 = Issue::new(
            "实现数据看板".to_string(),
            "数据可视化".to_string(),
            Priority::Medium,
        ).with_estimated_hours(8.0);
        
        let result = TaskDecompositionResult::new(vec![issue1.clone(), issue2.clone()])
            .with_suggested_order(vec![issue1.issue_id.clone(), issue2.issue_id.clone()])
            .with_dependencies(vec![(issue2.issue_id.clone(), issue1.issue_id.clone())])
            .add_risk("时间紧张，建议优先完成核心功能".to_string());
        
        assert!(result.success);
        assert_eq!(result.issues.len(), 2);
        assert_eq!(result.total_estimated_hours, 12.0);
        assert_eq!(result.suggested_order.len(), 2);
        assert_eq!(result.dependencies.len(), 1);
        assert_eq!(result.risks.len(), 1);
    }

    #[test]
    fn test_initializer_result_creation() {
        let prd_result = PRDParseResult::new("Test".to_string(), "Desc".to_string());
        let env_check = EnvironmentCheckResult::success();
        let task_result = TaskDecompositionResult::new(vec![]);
        
        let result = InitializerResult::success(prd_result, env_check, task_result);
        
        assert!(result.success);
        assert!(result.prd_result.is_some());
        assert!(result.env_check.is_some());
        assert!(result.task_decomposition.is_some());
        assert!(result.error.is_none());
    }

    #[test]
    fn test_initializer_agent_creation() {
        use crate::ai::AIConfig;

        let config = InitializerAgentConfig {
            agent_id: "agent-init-001".to_string(),
            project_path: "/path/to/project".to_string(),
            ai_config: AIConfig {
                provider: "openai".to_string(),
                api_key: "sk-test".to_string(),
                model: "gpt-4".to_string(),
                base_url: None,
            },
            prd_file_path: Some("/path/to/prd.md".to_string()),
            prd_content: None,
        };
        
        let agent = InitializerAgent::new(config);
        
        assert_eq!(agent.config.agent_id, "agent-init-001");
        assert_eq!(agent.status, InitializerStatus::Pending);
        assert!(!agent.session_id.is_empty());
    }
}
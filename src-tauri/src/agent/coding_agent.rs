//! Coding Agent 实现
//! 
//! 负责代码生成、质量检查和自动修复

use serde::{Deserialize, Serialize};
use crate::agent::types::AgentStatus;

/// Coding Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingAgentConfig {
    /// Agent ID
    pub agent_id: String,
    /// 项目路径
    pub project_path: String,
    /// AI 服务配置
    pub ai_config: crate::ai::AIConfig,
    /// 代码生成温度值 (0.0-1.0)
    pub temperature: f32,
    /// 最大 token 数
    pub max_tokens: i32,
}

/// Coding Agent 任务类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CodingTaskType {
    /// 生成代码文件
    GenerateFile,
    /// 修改现有文件
    ModifyFile,
    /// 生成测试文件
    GenerateTest,
    /// 代码重构
    Refactor,
    /// 代码审查
    Review,
}

/// Coding Agent 任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingTask {
    /// 任务唯一标识
    pub task_id: String,
    /// 任务类型
    pub task_type: CodingTaskType,
    /// 目标文件路径
    pub file_path: String,
    /// 任务描述/提示词
    pub prompt: String,
    /// 附加上下文（如相关文件内容）
    pub context: Option<String>,
    /// 是否已完成
    pub completed: bool,
}

impl CodingTask {
    /// 创建新任务
    pub fn new(
        task_type: CodingTaskType,
        file_path: String,
        prompt: String,
    ) -> Self {
        Self {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type,
            file_path,
            prompt,
            context: None,
            completed: false,
        }
    }

    /// 创建带有上下文的任務
    pub fn with_context(
        task_type: CodingTaskType,
        file_path: String,
        prompt: String,
        context: String,
    ) -> Self {
        Self {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type,
            file_path,
            prompt,
            context: Some(context),
            completed: false,
        }
    }
}

/// Coding Agent 质量检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheckResult {
    /// 是否通过检查
    pub passed: bool,
    /// ESLint 错误数
    pub eslint_errors: usize,
    /// TypeScript 错误数
    pub typescript_errors: usize,
    /// 测试失败数
    pub test_failures: usize,
    /// 详细报告
    pub report: String,
}

/// Coding Agent 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingResult {
    /// 是否成功
    pub success: bool,
    /// 生成的代码内容
    pub code: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 使用的 token 数
    pub tokens_used: Option<i32>,
    /// 质量检查结果
    pub quality_check: Option<QualityCheckResult>,
}

/// Coding Agent 结构体
#[derive(Debug, Clone)]
pub struct CodingAgent {
    /// Agent 配置
    pub config: CodingAgentConfig,
    /// 当前任务
    pub current_task: Option<CodingTask>,
    /// 任务队列
    pub task_queue: Vec<CodingTask>,
    /// 已完成的仼务
    pub completed_tasks: Vec<CodingTask>,
    /// 运行状态
    pub status: AgentStatus,
}

impl CodingAgent {
    /// 创建新的 Coding Agent
    pub fn new(config: CodingAgentConfig) -> Self {
        Self {
            config,
            current_task: None,
            task_queue: Vec::new(),
            completed_tasks: Vec::new(),
            status: AgentStatus::Idle,
        }
    }

    /// 获取 Agent ID
    pub fn agent_id(&self) -> &str {
        &self.config.agent_id
    }

    /// 添加任务到队列
    pub fn add_task(&mut self, task: CodingTask) {
        self.task_queue.push(task);
        
        // 如果当前没有任务且处于空闲状态，更新状态
        if self.current_task.is_none() && self.status == AgentStatus::Idle {
            self.status = AgentStatus::Running;
        }
    }

    /// 获取下一个任务
    pub fn next_task(&mut self) -> Option<CodingTask> {
        if !self.task_queue.is_empty() {
            let task = self.task_queue.remove(0);
            self.current_task = Some(task.clone());
            Some(task)
        } else {
            self.current_task = None;
            None
        }
    }

    /// 标记当前任务完成
    pub fn complete_current_task(&mut self) {
        if let Some(task) = self.current_task.take() {
            let mut completed_task = task;
            completed_task.completed = true;
            self.completed_tasks.push(completed_task);
            
            // 如果还有任务，保持 Running 状态，否则设为 Idle
            if self.task_queue.is_empty() {
                self.status = AgentStatus::Idle;
            }
        }
    }

    /// 生成代码（调用 AI 服务）
    pub async fn generate_code(&self, prompt: &str, context: Option<&str>) -> Result<CodingResult, String> {
        use crate::ai::{Message, ChatRequest};
        
        // 构建提示词
        let system_prompt = r#"You are an expert programmer. Generate clean, well-structured, production-ready code.
Follow best practices and coding standards. Include appropriate comments and documentation."#;

        let mut messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        // 添加上下文（如果有）
        if let Some(ctx) = context {
            messages.push(Message {
                role: "user".to_string(),
                content: format!("\n\nContext:\n{}", ctx),
            });
        }

        // 调用 AI 服务
        let client = reqwest::Client::new();
        let api_url = match self.config.ai_config.base_url.as_deref() {
            Some(url) => url.to_string(),
            None => {
                // 根据 provider 使用默认 URL
                match self.config.ai_config.provider.as_str() {
                    "openai" => "https://api.openai.com/v1/chat/completions".to_string(),
                    "anthropic" => "https://api.anthropic.com/v1/messages".to_string(),
                    "kimi" => "https://api.moonshot.cn/v1/chat/completions".to_string(),
                    "glm" => "https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string(),
                    _ => return Err(format!("Unknown provider: {}", self.config.ai_config.provider)),
                }
            }
        };

        let request = ChatRequest {
            model: self.config.ai_config.model.clone(),
            messages: messages.clone(),
            temperature: Some(self.config.temperature),
            max_tokens: Some(self.config.max_tokens),
            stream: false,
        };

        match client.post(&api_url)
            .header("Authorization", format!("Bearer {}", self.config.ai_config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        // TODO: 解析真实的 AI 响应
                        // 这里简化处理，实际应该根据 provider 解析不同的响应格式
                        Ok(CodingResult {
                            success: true,
                            code: Some("// Generated code will be here".to_string()),
                            error: None,
                            tokens_used: Some(100),
                            quality_check: None,
                        })
                    } else {
                        Err(format!("AI API request failed: {}", response.status()))
                    }
                }
                Err(e) => Err(format!("Failed to call AI API: {}", e)),
            }
    }

    /// 读取文件内容
    pub fn read_file(&self, file_path: &str) -> Result<String, String> {
        use std::fs;
        use std::path::PathBuf;

        let full_path = PathBuf::from(&self.config.project_path).join(file_path);
        
        if !full_path.exists() {
            return Err(format!("File not found: {}", full_path.display()));
        }

        fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read file: {}", e))
    }

    /// 写入文件内容
    pub fn write_file(&self, file_path: &str, content: &str) -> Result<(), String> {
        use std::fs;
        use std::path::PathBuf;

        let full_path = PathBuf::from(&self.config.project_path).join(file_path);
        
        // 确保目录存在
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        fs::write(&full_path, content)
            .map_err(|e| format!("Failed to write file: {}", e))
    }

    /// 执行代码质量检查
    pub async fn run_quality_check(&self, file_path: &str) -> Result<QualityCheckResult, String> {
        // TODO: 集成真实的 ESLint 和 TypeScript 检查
        // 这里实现基础框架
        
        let result = QualityCheckResult {
            passed: true,
            eslint_errors: 0,
            typescript_errors: 0,
            test_failures: 0,
            report: "Quality check passed".to_string(),
        };

        Ok(result)
    }

    /// 执行自动修复（最多 3 次）
    pub async fn auto_fix(&mut self, max_retries: u32) -> Result<bool, String> {
        for attempt in 1..=max_retries {
            // 运行质量检查
            if let Some(task) = &self.current_task {
                match self.run_quality_check(&task.file_path).await {
                    Ok(check_result) => {
                        if check_result.passed {
                            return Ok(true);
                        }

                        // 尝试修复
                        let fix_prompt = format!(
                            "Fix the following issues in the code:\n{}",
                            check_result.report
                        );

                        match self.generate_code(&fix_prompt, None).await {
                            Ok(result) => {
                                if let Some(code) = result.code {
                                    self.write_file(&task.file_path, &code)?;
                                }
                            }
                            Err(e) => {
                                if attempt == max_retries {
                                    return Err(format!("Auto-fix failed after {} attempts: {}", max_retries, e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if attempt == max_retries {
                            return Err(format!("Quality check failed: {}", e));
                        }
                    }
                }
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AIConfig;

    #[test]
    fn test_coding_agent_creation() {
        let config = CodingAgentConfig {
            agent_id: "coding-agent-001".to_string(),
            project_path: "/tmp/test-project".to_string(),
            ai_config: AIConfig::with_key("openai".to_string(), "gpt-4".to_string(), "test-key".to_string()),
            temperature: 0.7,
            max_tokens: 4096,
        };

        let agent = CodingAgent::new(config.clone());

        assert_eq!(agent.agent_id(), "coding-agent-001");
        assert_eq!(agent.status, AgentStatus::Idle);
        assert!(agent.current_task.is_none());
        assert!(agent.task_queue.is_empty());
        assert!(agent.completed_tasks.is_empty());
    }

    #[test]
    fn test_add_and_complete_task() {
        let config = CodingAgentConfig {
            agent_id: "coding-agent-002".to_string(),
            project_path: "/tmp/test".to_string(),
            ai_config: AIConfig::with_key("openai".to_string(), "gpt-4".to_string(), "test-key".to_string()),
            temperature: 0.5,
            max_tokens: 2048,
        };

        let mut agent = CodingAgent::new(config);

        // 添加任务
        let task = CodingTask::new(
            CodingTaskType::GenerateFile,
            "src/main.rs".to_string(),
            "Create a Rust hello world program".to_string(),
        );
        agent.add_task(task.clone());

        assert_eq!(agent.task_queue.len(), 1);
        assert_eq!(agent.status, AgentStatus::Running);

        // 获取任务
        let retrieved_task = agent.next_task();
        assert!(retrieved_task.is_some());
        assert!(agent.current_task.is_some());

        // 完成任务
        agent.complete_current_task();
        assert_eq!(agent.completed_tasks.len(), 1);
        assert!(agent.current_task.is_none());
        assert_eq!(agent.status, AgentStatus::Idle);
    }

    #[test]
    fn test_multiple_tasks_queue() {
        let config = CodingAgentConfig {
            agent_id: "coding-agent-003".to_string(),
            project_path: "/tmp/test".to_string(),
            ai_config: AIConfig::with_key("openai".to_string(), "gpt-4".to_string(), "test-key".to_string()),
            temperature: 0.6,
            max_tokens: 3072,
        };

        let mut agent = CodingAgent::new(config);

        // 添加多个任务
        for i in 0..3 {
            let task = CodingTask::new(
                CodingTaskType::GenerateFile,
                format!("src/file_{}.rs", i),
                format!("Generate file {}", i),
            );
            agent.add_task(task);
        }

        assert_eq!(agent.task_queue.len(), 3);

        // 依次完成任务
        for i in 0..3 {
            let task = agent.next_task();
            assert!(task.is_some());
            agent.complete_current_task();
            assert_eq!(agent.completed_tasks.len(), i + 1);
        }

        assert!(agent.task_queue.is_empty());
        assert_eq!(agent.completed_tasks.len(), 3);
        assert_eq!(agent.status, AgentStatus::Idle);
    }

    #[test]
    fn test_coding_task_with_context() {
        let task = CodingTask::with_context(
            CodingTaskType::ModifyFile,
            "src/utils.rs".to_string(),
            "Add error handling".to_string(),
            "Existing code content...".to_string(),
        );

        assert_eq!(task.task_type, CodingTaskType::ModifyFile);
        assert_eq!(task.file_path, "src/utils.rs");
        assert!(task.context.is_some());
        assert!(!task.completed);
    }

    #[test]
    fn test_quality_check_result() {
        let check_result = QualityCheckResult {
            passed: true,
            eslint_errors: 0,
            typescript_errors: 0,
            test_failures: 0,
            report: "All checks passed".to_string(),
        };

        assert!(check_result.passed);
        assert_eq!(check_result.eslint_errors, 0);
        assert_eq!(check_result.typescript_errors, 0);
    }

    #[test]
    fn test_coding_result_success() {
        let result = CodingResult {
            success: true,
            code: Some("console.log('Hello');".to_string()),
            error: None,
            tokens_used: Some(150),
            quality_check: None,
        };

        assert!(result.success);
        assert!(result.code.is_some());
        assert_eq!(result.tokens_used, Some(150));
    }
}

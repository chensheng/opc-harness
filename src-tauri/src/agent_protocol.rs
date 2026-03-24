//! Agent 通信协议定义
//! 
//! 本模块定义了 Agent 与守护进程、前端之间的通信协议
//! 支持 Stdio 管道通信和 WebSocket 实时推送

use serde::{Deserialize, Serialize};

/// Agent 生命周期阶段
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentPhase {
    /// 初始化阶段：环境检查、任务分解
    Initializer,
    /// 编码阶段：代码生成、测试编写
    Coding,
    /// MR 创建阶段：汇总提交、创建合并请求
    MRCreation,
}

/// Agent 运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// 空闲状态，等待任务
    Idle,
    /// 正在执行任务
    Running,
    /// 已暂停
    Paused,
    /// 任务完成
    Completed,
    /// 任务失败，包含错误信息
    Failed(String),
}

/// Agent 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 唯一标识
    pub agent_id: String,
    /// Agent 类型："initializer" | "coding" | "mr_creation"
    #[serde(rename = "type")]
    pub agent_type: String,
    /// 当前阶段
    pub phase: AgentPhase,
    /// 当前状态
    pub status: AgentStatus,
    /// 项目路径
    pub project_path: String,
    /// 会话 ID
    pub session_id: String,
}

/// Agent 请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    /// 请求唯一标识
    pub request_id: String,
    /// 发送请求的 Agent ID
    pub agent_id: String,
    /// 动作类型
    pub action: String,
    /// 请求载荷
    pub payload: serde_json::Value,
}

impl AgentRequest {
    /// 创建新的请求
    pub fn new(agent_id: String, action: String, payload: serde_json::Value) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            agent_id,
            action,
            payload,
        }
    }
}

/// Agent 响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    /// 响应唯一标识
    pub response_id: String,
    /// 对应的请求 ID
    pub request_id: String,
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

impl AgentResponse {
    /// 创建成功响应
    pub fn success(request_id: String, data: Option<serde_json::Value>) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: true,
            data,
            error: None,
        }
    }

    /// 创建失败响应
    pub fn error(request_id: String, error_msg: String) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: false,
            data: None,
            error: Some(error_msg),
        }
    }
}

/// 消息类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 日志消息
    Log,
    /// 状态更新
    Status,
    /// 进度更新
    Progress,
    /// 错误消息
    Error,
    /// 心跳消息
    Heartbeat,
}

/// Agent 消息 (用于实时推送)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// 消息唯一标识
    pub message_id: String,
    /// 时间戳 (Unix timestamp)
    pub timestamp: i64,
    /// 消息来源："agent" | "daemon" | "frontend"
    pub source: String,
    /// 消息类型
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// 消息内容
    pub content: String,
    /// 附加元数据
    pub metadata: Option<serde_json::Value>,
}

impl AgentMessage {
    /// 创建日志消息
    pub fn log(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Log,
            content,
            metadata: None,
        }
    }

    /// 创建进度消息
    pub fn progress(source: String, content: String, progress: f32) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Progress,
            content,
            metadata: Some(serde_json::json!({ "progress": progress })),
        }
    }

    /// 创建状态消息
    pub fn status(source: String, content: String, status: AgentStatus) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Status,
            content,
            metadata: Some(serde_json::to_value(&status).unwrap_or_default()),
        }
    }

    /// 创建错误消息
    pub fn error(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Error,
            content,
            metadata: None,
        }
    }
}

// ========== Coding Agent 实现 ==========

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
        use crate::ai::{AIConfig, Message, ChatRequest};
        
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

// ========== Branch Manager 实现 ==========

/// 分支类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BranchType {
    /// 功能分支（feature/xxx）
    Feature,
    /// 修复分支（fix/xxx）
    Fix,
    /// 发布分支（release/v1.0.0）
    Release,
    /// 热修复分支（hotfix/xxx）
    Hotfix,
}

impl std::fmt::Display for BranchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchType::Feature => write!(f, "feature"),
            BranchType::Fix => write!(f, "fix"),
            BranchType::Release => write!(f, "release"),
            BranchType::Hotfix => write!(f, "hotfix"),
        }
    }
}

/// 分支信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// 分支名称
    pub name: String,
    /// 分支类型
    pub branch_type: BranchType,
    /// 是否当前分支
    pub is_current: bool,
    /// 最后一次提交哈希
    pub last_commit_hash: Option<String>,
    /// 最后一次提交消息
    pub last_commit_message: Option<String>,
    /// 创建时间戳
    pub created_at: Option<i64>,
}

/// 分支管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchManagerConfig {
    /// 项目路径
    pub project_path: String,
    /// 默认基础分支（main/develop）
    pub default_base_branch: String,
    /// 分支命名前缀（可选，如 "issue-"）
    pub name_prefix: Option<String>,
}

/// 分支操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchOperationResult {
    /// 是否成功
    pub success: bool,
    /// 分支名称
    pub branch_name: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 详细消息
    pub message: Option<String>,
}

/// 分支管理器结构体
#[derive(Debug, Clone)]
pub struct BranchManager {
    /// 配置信息
    pub config: BranchManagerConfig,
    /// 当前分支名称
    pub current_branch: Option<String>,
    /// 已创建的分支列表
    pub created_branches: Vec<String>,
}

impl BranchManager {
    /// 创建新的分支管理器
    pub fn new(config: BranchManagerConfig) -> Self {
        Self {
            config,
            current_branch: None,
            created_branches: Vec::new(),
        }
    }

    /// 生成规范的功能分支名称
    pub fn generate_branch_name(
        &self,
        branch_type: BranchType,
        description: &str,
        issue_id: Option<&str>,
    ) -> String {
        // 清理描述文本：转换为小写，替换空格为连字符，移除特殊字符
        let clean_desc = description
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        // 构建分支名称
        let mut parts = Vec::new();
        
        // 添加类型前缀
        parts.push(branch_type.to_string());
        
        // 添加 Issue ID（如果有）
        if let Some(id) = issue_id {
            parts.push(id.to_string());
        }
        
        // 添加自定义前缀（如果有）
        if let Some(prefix) = &self.config.name_prefix {
            parts.push(prefix.clone());
        }
        
        // 添加描述
        parts.push(clean_desc);

        parts.join("/")
    }

    /// 验证分支名称是否符合规范
    pub fn validate_branch_name(&self, branch_name: &str) -> Result<bool, String> {
        // 检查基本格式
        if branch_name.is_empty() {
            return Err("Branch name cannot be empty".to_string());
        }

        // 检查长度
        if branch_name.len() > 255 {
            return Err("Branch name is too long (max 255 characters)".to_string());
        }

        // 检查非法字符
        let invalid_chars = ['~', '^', ':', '\\', '?', '*', '[', ' ', '\t', '\n', '\r'];
        for ch in invalid_chars.iter() {
            if branch_name.contains(*ch) {
                return Err(format!("Branch name contains invalid character: '{}'", ch));
            }
        }

        // 检查是否以分隔符开头或结尾
        if branch_name.starts_with('/') || branch_name.ends_with('/') {
            return Err("Branch name cannot start or end with '/'".to_string());
        }

        // 检查连续的分隔符
        if branch_name.contains("//") {
            return Err("Branch name cannot contain consecutive slashes".to_string());
        }

        // 检查是否包含有效的类型前缀
        let valid_types = ["feature", "fix", "release", "hotfix"];
        let first_part = branch_name.split('/').next().unwrap_or("");
        if !valid_types.contains(&first_part) {
            return Err(format!(
                "Branch name must start with a valid type ({:?}), got: '{}'",
                valid_types, first_part
            ));
        }

        Ok(true)
    }

    /// 创建功能分支
    pub async fn create_feature_branch(
        &mut self,
        description: &str,
        issue_id: Option<&str>,
        base_branch: Option<&str>,
    ) -> Result<BranchOperationResult, String> {
        let branch_name = self.generate_branch_name(BranchType::Feature, description, issue_id);
        
        // 验证分支名称
        self.validate_branch_name(&branch_name)?;

        // 执行 Git 命令创建分支
        match self.execute_git_command(&["checkout", "-b", &branch_name]).await {
            Ok(_) => {
                self.current_branch = Some(branch_name.clone());
                self.created_branches.push(branch_name.clone());
                
                Ok(BranchOperationResult {
                    success: true,
                    branch_name: Some(branch_name),
                    error: None,
                    message: Some("Feature branch created and checked out successfully".to_string()),
                })
            }
            Err(e) => Err(format!("Failed to create feature branch: {}", e)),
        }
    }

    /// 切换到指定分支
    pub async fn checkout_branch(&mut self, branch_name: &str) -> Result<BranchOperationResult, String> {
        match self.execute_git_command(&["checkout", branch_name]).await {
            Ok(_) => {
                self.current_branch = Some(branch_name.to_string());
                
                Ok(BranchOperationResult {
                    success: true,
                    branch_name: Some(branch_name.to_string()),
                    error: None,
                    message: Some(format!("Switched to branch '{}'", branch_name)),
                })
            }
            Err(e) => Err(format!("Failed to checkout branch '{}': {}", branch_name, e)),
        }
    }

    /// 切换回基础分支
    pub async fn checkout_base_branch(&mut self) -> Result<BranchOperationResult, String> {
        let base_branch = self.config.default_base_branch.clone();
        self.checkout_branch(&base_branch).await
    }

    /// 获取当前分支信息
    pub async fn get_current_branch(&self) -> Result<Option<String>, String> {
        match self.execute_git_command(&["rev-parse", "--abbrev-ref", "HEAD"]).await {
            Ok(output) => {
                let branch = output.trim().to_string();
                Ok(if branch.is_empty() { None } else { Some(branch) })
            }
            Err(_) => Ok(None),
        }
    }

    /// 获取所有本地分支列表
    pub async fn get_local_branches(&self) -> Result<Vec<BranchInfo>, String> {
        let output = self.execute_git_command(&["branch", "--format=%(refname:short)%09%H%09%s"]).await?;
        
        let branches: Vec<BranchInfo> = output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let name = parts[0].trim_start_matches('*').trim().to_string();
                    let is_current = parts[0].starts_with('*');
                    
                    Some(BranchInfo {
                        name,
                        branch_type: self.detect_branch_type(parts[0]),
                        is_current,
                        last_commit_hash: Some(parts[1].to_string()),
                        last_commit_message: Some(parts[2].to_string()),
                        created_at: None,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(branches)
    }

    /// 删除指定分支
    pub async fn delete_branch(&mut self, branch_name: &str, force: bool) -> Result<BranchOperationResult, String> {
        let args = if force {
            vec!["branch", "-D", branch_name]
        } else {
            vec!["branch", "-d", branch_name]
        };

        match self.execute_git_command(&args).await {
            Ok(_) => {
                self.created_branches.retain(|name| name != branch_name);
                
                Ok(BranchOperationResult {
                    success: true,
                    branch_name: Some(branch_name.to_string()),
                    error: None,
                    message: Some(format!("Branch '{}' deleted successfully", branch_name)),
                })
            }
            Err(e) => Err(format!("Failed to delete branch '{}': {}", branch_name, e)),
        }
    }

    /// 重命名分支
    pub async fn rename_branch(
        &mut self,
        old_name: &str,
        new_name: &str,
    ) -> Result<BranchOperationResult, String> {
        // 验证新名称
        self.validate_branch_name(new_name)?;

        match self.execute_git_command(&["branch", "-m", old_name, new_name]).await {
            Ok(_) => {
                // 更新记录
                if let Some(current) = &self.current_branch {
                    if current == old_name {
                        self.current_branch = Some(new_name.to_string());
                    }
                }
                
                if let Some(pos) = self.created_branches.iter().position(|n| n == old_name) {
                    self.created_branches[pos] = new_name.to_string();
                }
                
                Ok(BranchOperationResult {
                    success: true,
                    branch_name: Some(new_name.to_string()),
                    error: None,
                    message: Some(format!("Branch renamed from '{}' to '{}'", old_name, new_name)),
                })
            }
            Err(e) => Err(format!("Failed to rename branch: {}", e)),
        }
    }

    /// 检测分支类型
    fn detect_branch_type(&self, branch_name: &str) -> BranchType {
        let first_part = branch_name.split('/').next().unwrap_or("").to_lowercase();
        
        match first_part.as_str() {
            "feature" => BranchType::Feature,
            "fix" => BranchType::Fix,
            "release" => BranchType::Release,
            "hotfix" => BranchType::Hotfix,
            _ => BranchType::Feature, // 默认为 Feature
        }
    }

    /// 执行 Git 命令的辅助方法
    async fn execute_git_command(&self, args: &[&str]) -> Result<String, String> {
        use tokio::process::Command;
        use std::path::PathBuf;

        let git_path = PathBuf::from(&self.config.project_path);
        
        let output = Command::new("git")
            .current_dir(&git_path)
            .args(args)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// 检查 Git 仓库是否存在
    pub async fn is_git_repo(&self) -> bool {
        self.execute_git_command(&["rev-parse", "--git-dir"]).await.is_ok()
    }

    /// 获取最近的提交历史
    pub async fn get_recent_commits(&self, count: usize) -> Result<Vec<(String, String)>, String> {
        let output = self.execute_git_command(&[
            "log",
            &format!("-{}", count),
            "--format=%H\t%s",
        ]).await?;

        let commits: Vec<(String, String)> = output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(commits)
    }
}

// ========== Daemon 相关类型定义 ==========

/// 守护进程运行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaemonStatus {
    Starting,      // 启动中
    Running,       // 运行中
    Paused,        // 已暂停
    Stopping,      // 停止中
    Stopped,       // 已停止
    Failed(String), // 失败 (含错误信息)
}

/// 守护进程配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub session_id: String,           // 会话 ID
    pub project_path: String,         // 项目路径
    pub log_level: String,            // 日志级别：debug/info/warn/error
    pub max_concurrent_agents: usize, // 最大并发 Agent 数
    pub workspace_dir: String,        // 工作目录
}

/// Agent 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProcessInfo {
    pub agent_id: String,             // Agent 唯一标识
    pub agent_type: String,           // Agent 类型：initializer/coding/mr_creation
    pub pid: Option<u32>,             // 进程 ID
    pub status: AgentStatus,          // 运行状态
    pub started_at: i64,              // 启动时间戳
    pub resource_usage: ResourceUsage, // 资源使用情况
}

/// 资源使用情况
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_percent: f32,    // CPU 使用率 (%)
    pub memory_mb: usize,    // 内存使用量 (MB)
    pub disk_io_read: u64,   // 磁盘读取 (bytes)
    pub disk_io_write: u64,  // 磁盘写入 (bytes)
    pub network_rx: u64,     // 网络接收 (bytes)
    pub network_tx: u64,     // 网络发送 (bytes)
}

/// 守护进程状态快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonSnapshot {
    pub daemon_id: String,              // 守护进程 ID
    pub status: DaemonStatus,           // 运行状态
    pub config: DaemonConfig,           // 配置信息
    pub active_agents: Vec<AgentProcessInfo>, // 活跃的 Agent 列表
    pub completed_tasks: Vec<String>,   // 已完成的任务列表
    pub pending_tasks: Vec<String>,     // 待处理的任务列表
    pub start_time: i64,                // 启动时间戳
    pub last_update: i64,               // 最后更新时间戳
    pub system_info: SystemInfo,        // 系统信息
}

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,                     // 操作系统
    pub arch: String,                   // 架构
    pub total_memory: u64,              // 总内存 (MB)
    pub available_memory: u64,          // 可用内存 (MB)
    pub cpu_cores: usize,               // CPU 核心数
    pub rust_version: String,           // Rust 版本
}

/// 守护进程命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {
    Start { config: DaemonConfig },     // 启动守护进程
    Stop { graceful: bool },            // 停止守护进程
    Pause,                               // 暂停
    Resume,                              // 恢复
    SpawnAgent { agent_type: String },  // 生成新的 Agent
    KillAgent { agent_id: String },     // 终止指定 Agent
    GetStatus,                           // 获取状态
    GetSnapshot,                         // 获取快照
}

/// 守护进程事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonEvent {
    Started,                             // 已启动
    Stopped,                             // 已停止
    AgentSpawned { agent_id: String },  // Agent 已生成
    AgentCompleted { agent_id: String }, // Agent 已完成
    AgentFailed { agent_id: String, error: String }, // Agent 失败
    ResourceWarning { message: String }, // 资源警告
    Error { message: String },           // 错误事件
}

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum WebSocketMessage {
    /// 客户端连接
    Connect {
        session_id: String,
    },
    /// 客户端断开
    Disconnect {
        session_id: String,
    },
    /// 普通消息
    Message {
        data: serde_json::Value,
    },
    /// 心跳消息
    Heartbeat {
        timestamp: i64,
    },
    /// 订阅 Agent 消息
    Subscribe {
        agent_id: String,
    },
    /// 取消订阅
    Unsubscribe {
        agent_id: String,
    },
}

/// 守护进程管理器
pub struct DaemonManager {
    daemon_id: String,
    status: DaemonStatus,
    config: Option<DaemonConfig>,
    agents: std::collections::HashMap<String, AgentProcessInfo>,
    completed_tasks: Vec<String>,
    pending_tasks: Vec<String>,
    start_time: i64,
    // ========== VC-013: 并发控制相关字段 ==========
    running_count: usize,                    // 当前运行中的 Agent 数量
    max_concurrent: usize,                   // 最大并发数 (从 config 同步)
    agent_queue: Vec<String>,                // 等待中的 Agent ID 队列
    running_agents: std::collections::HashSet<String>, // 正在运行的 Agent ID 集合
}

impl DaemonManager {
    /// 创建新的守护进程管理器
    pub fn new() -> Self {
        Self {
            daemon_id: uuid::Uuid::new_v4().to_string(),
            status: DaemonStatus::Stopped,
            config: None,
            agents: std::collections::HashMap::new(),
            completed_tasks: Vec::new(),
            pending_tasks: Vec::new(),
            start_time: 0,
            // ========== VC-013: 并发控制初始化 ==========
            running_count: 0,
            max_concurrent: 5, // 默认值，会在 start 时被 config 覆盖
            agent_queue: Vec::new(),
            running_agents: std::collections::HashSet::new(),
        }
    }

    /// 启动守护进程
    pub fn start(&mut self, config: DaemonConfig) -> Result<(), String> {
        if self.status == DaemonStatus::Running {
            return Err("Daemon is already running".to_string());
        }

        // ========== VC-013: 初始化并发控制配置 ==========
        self.max_concurrent = config.max_concurrent_agents;
        
        self.config = Some(config);
        self.status = DaemonStatus::Starting;
        self.start_time = chrono::Utc::now().timestamp();
        
        // TODO: 初始化资源监控、日志系统等
        self.status = DaemonStatus::Running;
        
        Ok(())
    }

    /// 停止守护进程
    pub fn stop(&mut self, graceful: bool) -> Result<(), String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        self.status = DaemonStatus::Stopping;
        
        // TODO: 停止所有 Agent 进程
        if graceful {
            // 优雅停止：等待当前任务完成
            for agent in self.agents.values_mut() {
                if agent.status == AgentStatus::Running {
                    agent.status = AgentStatus::Paused;
                }
            }
        } else {
            // 强制停止：立即终止所有 Agent
            self.agents.clear();
        }
        
        self.status = DaemonStatus::Stopped;
        self.pending_tasks.clear();
        
        Ok(())
    }

    /// 暂停守护进程
    pub fn pause(&mut self) -> Result<(), String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        for agent in self.agents.values_mut() {
            if agent.status == AgentStatus::Running {
                agent.status = AgentStatus::Paused;
            }
        }
        
        self.status = DaemonStatus::Paused;
        Ok(())
    }

    /// 恢复守护进程
    pub fn resume(&mut self) -> Result<(), String> {
        if self.status != DaemonStatus::Paused {
            return Err("Daemon is not paused".to_string());
        }

        for agent in self.agents.values_mut() {
            if agent.status == AgentStatus::Paused {
                agent.status = AgentStatus::Running;
            }
        }
        
        self.status = DaemonStatus::Running;
        Ok(())
    }

    /// 生成新的 Agent 进程
    pub fn spawn_agent(&mut self, agent_type: &str) -> Result<String, String> {
        if self.status != DaemonStatus::Running {
            return Err("Daemon is not running".to_string());
        }

        let agent_id = format!("{}-{}", agent_type, uuid::Uuid::new_v4());
        
        let agent_info = AgentProcessInfo {
            agent_id: agent_id.clone(),
            agent_type: agent_type.to_string(),
            pid: None, // TODO: 实际启动进程后设置
            status: AgentStatus::Idle,
            started_at: chrono::Utc::now().timestamp(),
            resource_usage: ResourceUsage::default(),
        };

        self.agents.insert(agent_id.clone(), agent_info);
        self.pending_tasks.push(agent_id.clone());
        
        Ok(agent_id)
    }

    /// 终止指定 Agent
    pub fn kill_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // TODO: 实际终止进程
        self.agents.remove(agent_id);
        self.pending_tasks.retain(|id| id != agent_id);
        
        Ok(())
    }

    /// 获取守护进程状态
    pub fn get_status(&self) -> DaemonStatus {
        self.status.clone()
    }

    /// 获取守护进程快照
    pub fn get_snapshot(&self) -> DaemonSnapshot {
        let system_info = self.get_system_info();
        
        DaemonSnapshot {
            daemon_id: self.daemon_id.clone(),
            status: self.status.clone(),
            config: self.config.clone().unwrap_or_else(|| DaemonConfig {
                session_id: String::new(),
                project_path: String::new(),
                log_level: "info".to_string(),
                max_concurrent_agents: 5,
                workspace_dir: String::new(),
            }),
            active_agents: self.agents.values().cloned().collect(),
            completed_tasks: self.completed_tasks.clone(),
            pending_tasks: self.pending_tasks.clone(),
            start_time: self.start_time,
            last_update: chrono::Utc::now().timestamp(),
            system_info,
        }
    }

    /// 获取系统信息
    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            total_memory: self.get_total_memory(),
            available_memory: self.get_available_memory(),
            cpu_cores: num_cpus::get(),
            rust_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 获取总内存 (MB)
    fn get_total_memory(&self) -> u64 {
        // TODO: 使用 sysinfo crate 获取真实值
        16384 // 默认 16GB
    }

    /// 获取可用内存 (MB)
    fn get_available_memory(&self) -> u64 {
        // TODO: 使用 sysinfo crate 获取真实值
        8192 // 默认 8GB
    }

    /// 更新资源使用情况
    pub fn update_resource_usage(&mut self) {
        for agent in self.agents.values_mut() {
            // TODO: 使用 sysinfo crate 获取真实资源使用情况
            agent.resource_usage = ResourceUsage {
                cpu_percent: 0.0,
                memory_mb: 0,
                disk_io_read: 0,
                disk_io_write: 0,
                network_rx: 0,
                network_tx: 0,
            };
        }
    }

    /// 标记任务完成
    pub fn mark_task_completed(&mut self, task_id: &str) {
        self.pending_tasks.retain(|id| id != task_id);
        self.completed_tasks.push(task_id.to_string());
    }

    // ========== VC-013: 并发控制核心方法 ==========

    /// 检查是否可以启动新的 Agent
    pub fn can_spawn_agent(&self) -> bool {
        self.running_count < self.max_concurrent
    }

    /// 获取可用的并发槽位数
    pub fn available_slots(&self) -> usize {
        if self.running_count >= self.max_concurrent {
            0
        } else {
            self.max_concurrent - self.running_count
        }
    }

    /// 尝试启动 Agent（受并发限制）
    /// 返回 true 表示可以立即启动，false 表示需要排队
    pub fn try_start_agent(&mut self, agent_id: &str) -> bool {
        // 检查是否已经在运行
        if self.running_agents.contains(agent_id) {
            return true;
        }

        // 检查是否有可用槽位
        if self.can_spawn_agent() {
            self.running_agents.insert(agent_id.to_string());
            self.running_count += 1;
            
            // 更新 Agent 状态为 Running
            if let Some(agent) = self.agents.get_mut(agent_id) {
                agent.status = AgentStatus::Running;
            }
            
            true
        } else {
            // 加入等待队列
            if !self.agent_queue.contains(&agent_id.to_string()) {
                self.agent_queue.push(agent_id.to_string());
                
                // 更新 Agent 状态为 Idle (等待中)
                if let Some(agent) = self.agents.get_mut(agent_id) {
                    agent.status = AgentStatus::Idle;
                }
            }
            false
        }
    }

    /// 停止 Agent 并释放槽位
    pub fn stop_agent(&mut self, agent_id: &str) -> Result<(), String> {
        if !self.agents.contains_key(agent_id) {
            return Err(format!("Agent {} not found", agent_id));
        }

        // 从运行集合中移除
        if self.running_agents.remove(agent_id) {
            self.running_count = self.running_count.saturating_sub(1);
        }

        // 更新 Agent 状态
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Completed;
        }

        // 从等待队列移除
        self.agent_queue.retain(|id| id != agent_id);

        // ========== VC-013: 调度队列中的下一个 Agent ==========
        self.schedule_next_agent();

        Ok(())
    }

    /// 调度队列中的下一个 Agent
    fn schedule_next_agent(&mut self) {
        // 检查是否有可用槽位和等待中的 Agent
        while self.can_spawn_agent() && !self.agent_queue.is_empty() {
            if let Some(next_agent_id) = self.agent_queue.first().cloned() {
                self.agent_queue.remove(0);
                
                // 启动该 Agent
                self.running_agents.insert(next_agent_id.clone());
                self.running_count += 1;
                
                // 更新 Agent 状态
                if let Some(agent) = self.agents.get_mut(&next_agent_id) {
                    agent.status = AgentStatus::Running;
                }
                
                // TODO: 实际启动 Agent 进程
            } else {
                break;
            }
        }
    }

    /// 获取当前并发统计信息
    pub fn get_concurrency_stats(&self) -> ConcurrencyStats {
        ConcurrencyStats {
            running_count: self.running_count,
            max_concurrent: self.max_concurrent,
            queued_count: self.agent_queue.len(),
            available_slots: self.available_slots(),
            utilization: if self.max_concurrent > 0 {
                (self.running_count as f32 / self.max_concurrent as f32) * 100.0
            } else {
                0.0
            },
        }
    }

    /// 动态调整最大并发数
    pub fn adjust_max_concurrent(&mut self, new_limit: usize) -> Result<(), String> {
        if new_limit == 0 {
            return Err("max_concurrent must be greater than 0".to_string());
        }

        let _old_limit = self.max_concurrent;
        self.max_concurrent = new_limit;

        // 如果新限制小于当前运行数，需要暂停部分 Agent
        if new_limit < self.running_count {
            // 暂停多余的 Agent（按启动时间排序，暂停最新的）
            let mut running_vec: Vec<_> = self.running_agents.iter().cloned().collect();
            running_vec.sort_by(|a, b| {
                let a_time = self.agents.get(a).map(|ag| ag.started_at).unwrap_or(0);
                let b_time = self.agents.get(b).map(|ag| ag.started_at).unwrap_or(0);
                b_time.cmp(&a_time) // 降序，最新的先暂停
            });

            let excess = self.running_count - new_limit;
            for agent_id in running_vec.into_iter().take(excess) {
                if let Some(agent) = self.agents.get_mut(&agent_id) {
                    if agent.status == AgentStatus::Running {
                        agent.status = AgentStatus::Paused;
                    }
                }
                self.running_agents.remove(&agent_id);
                self.agent_queue.push(agent_id);
            }
            
            self.running_count = new_limit;
        } else {
            // 如果新限制更大，尝试启动更多 Agent
            self.schedule_next_agent();
        }

        Ok(())
    }

    /// 获取所有等待中的 Agent
    pub fn get_queued_agents(&self) -> Vec<&String> {
        self.agent_queue.iter().collect()
    }

    /// 获取所有运行中的 Agent
    pub fn get_running_agents(&self) -> Vec<&String> {
        self.running_agents.iter().collect()
    }
}

/// 并发统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyStats {
    pub running_count: usize,      // 当前运行数
    pub max_concurrent: usize,     // 最大并发数
    pub queued_count: usize,       // 等待队列长度
    pub available_slots: usize,    // 可用槽位数
    pub utilization: f32,          // 并发利用率 (%)
}

/// Stdio 管道命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioCommand {
    /// 命令 ID
    pub command_id: String,
    /// 命令类型
    #[serde(rename = "command")]
    pub cmd_type: String,
    /// 命令参数
    pub args: Vec<String>,
    /// 工作目录
    pub cwd: Option<String>,
    /// 环境变量
    pub env: Option<HashMap<String, String>>,
}

impl StdioCommand {
    /// 创建新命令
    pub fn new(cmd_type: String, args: Vec<String>) -> Self {
        Self {
            command_id: uuid::Uuid::new_v4().to_string(),
            cmd_type,
            args,
            cwd: None,
            env: None,
        }
    }
}

/// Stdio 输出行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioOutput {
    /// 标准输出
    pub stdout: Option<String>,
    /// 标准错误
    pub stderr: Option<String>,
    /// 退出码
    pub exit_code: Option<i32>,
    /// 时间戳
    pub timestamp: i64,
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_request_creation() {
        let request = AgentRequest::new(
            "agent-001".to_string(),
            "initialize".to_string(),
            serde_json::json!({"project": "test"}),
        );
        
        assert!(!request.request_id.is_empty());
        assert_eq!(request.agent_id, "agent-001");
        assert_eq!(request.action, "initialize");
    }

    #[test]
    fn test_agent_response_success() {
        let response = AgentResponse::success(
            "req-001".to_string(),
            Some(serde_json::json!({"result": "ok"})),
        );
        
        assert!(response.success);
        assert!(response.error.is_none());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_agent_response_error() {
        let response = AgentResponse::error(
            "req-001".to_string(),
            "Something went wrong".to_string(),
        );
        
        assert!(!response.success);
        assert!(response.error.is_some());
        assert!(response.data.is_none());
    }

    #[test]
    fn test_agent_message_log() {
        let msg = AgentMessage::log("agent-001".to_string(), "Starting initialization...".to_string());
        
        assert_eq!(msg.message_type, MessageType::Log);
        assert_eq!(msg.source, "agent-001");
        assert_eq!(msg.content, "Starting initialization...");
    }

    #[test]
    fn test_agent_message_progress() {
        let msg = AgentMessage::progress("agent-001".to_string(), "Processing...".to_string(), 0.5);
        
        assert_eq!(msg.message_type, MessageType::Progress);
        assert!(msg.metadata.is_some());
    }

    #[test]
    fn test_stdio_command_creation() {
        let cmd = StdioCommand::new("git".to_string(), vec!["init".to_string()]);
        
        assert!(!cmd.command_id.is_empty());
        assert_eq!(cmd.cmd_type, "git");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_websocket_message_serialize() {
        let msg = WebSocketMessage::Connect {
            session_id: "session-001".to_string(),
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"connect\""));
        assert!(json.contains("session-001"));
    }

    // ========== Daemon 相关测试 ==========

    #[test]
    fn test_daemon_manager_creation() {
        let manager = DaemonManager::new();
        
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
        assert!(manager.agents.is_empty());
        assert!(manager.completed_tasks.is_empty());
        assert!(manager.pending_tasks.is_empty());
        // ========== VC-013: 验证并发控制字段初始化 ==========
        assert_eq!(manager.running_count, 0);
        assert_eq!(manager.max_concurrent, 5); // 默认值
        assert!(manager.agent_queue.is_empty());
        assert!(manager.running_agents.is_empty());
    }

    #[test]
    fn test_daemon_manager_start() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "debug".to_string(),
            max_concurrent_agents: 3,
            workspace_dir: "/tmp".to_string(),
        };
        
        let result = manager.start(config);
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_manager_stop() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let result = manager.stop(true);
        
        assert!(result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Stopped);
    }

    #[test]
    fn test_daemon_manager_spawn_agent() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        let agent_id = manager.spawn_agent("initializer");
        
        assert!(agent_id.is_ok());
        assert!(!agent_id.as_ref().unwrap().is_empty());
        assert_eq!(manager.agents.len(), 1);
    }

    #[test]
    fn test_daemon_manager_pause_resume() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 暂停
        let pause_result = manager.pause();
        assert!(pause_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Paused);
        
        // 恢复
        let resume_result = manager.resume();
        assert!(resume_result.is_ok());
        assert_eq!(manager.get_status(), DaemonStatus::Running);
    }

    #[test]
    fn test_daemon_snapshot() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 5,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        manager.spawn_agent("coding").unwrap();
        
        let snapshot = manager.get_snapshot();
        
        assert_eq!(snapshot.status, DaemonStatus::Running);
        assert_eq!(snapshot.active_agents.len(), 1);
        assert!(!snapshot.daemon_id.is_empty());
        assert!(snapshot.last_update > 0);
    }

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        
        assert_eq!(usage.cpu_percent, 0.0);
        assert_eq!(usage.memory_mb, 0);
        assert_eq!(usage.disk_io_read, 0);
        assert_eq!(usage.disk_io_write, 0);
    }

    #[test]
    fn test_system_info_creation() {
        let mut manager = DaemonManager::new();
        let snapshot = manager.get_snapshot();
        
        assert!(!snapshot.system_info.os.is_empty());
        assert!(!snapshot.system_info.arch.is_empty());
        assert!(snapshot.system_info.total_memory > 0);
        assert!(snapshot.system_info.cpu_cores > 0);
    }

    // ========== VC-013: 并发控制测试 ==========

    #[test]
    fn test_concurrency_config_initialization() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 0);
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_can_spawn_agent_when_slots_available() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 初始时有可用槽位
        assert!(manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 2);
    }

    #[test]
    fn test_cannot_spawn_agent_when_slots_full() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，占满槽位
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        // 此时不能再启动新 Agent
        assert!(!manager.can_spawn_agent());
        assert_eq!(manager.available_slots(), 0);
    }

    #[test]
    fn test_agent_queuing_when_concurrent_limit_reached() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        // 第 3 个 Agent 需要排队
        let agent3 = manager.spawn_agent("coding").unwrap();
        let should_start = manager.try_start_agent(&agent3);
        
        assert!(!should_start); // 不能立即启动
        assert_eq!(manager.agent_queue.len(), 1);
        assert!(manager.agent_queue.contains(&agent3));
    }

    #[test]
    fn test_auto_schedule_next_agent_on_completion() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，第 3 个排队
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3); // 会进入队列
        
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 1);
        
        // 完成 agent1，agent3 应该自动启动
        manager.stop_agent(&agent1).unwrap();
        
        assert_eq!(manager.running_count, 2); // 保持 2 个运行
        assert!(manager.agent_queue.is_empty()); // 队列清空
    }

    #[test]
    fn test_concurrency_stats() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        
        let stats = manager.get_concurrency_stats();
        
        assert_eq!(stats.running_count, 2);
        assert_eq!(stats.max_concurrent, 4);
        assert_eq!(stats.queued_count, 0);
        assert_eq!(stats.available_slots, 2);
        assert!((stats.utilization - 50.0).abs() < 0.1); // 50% 利用率
    }

    #[test]
    fn test_adjust_max_concurrent_increase() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 2 个 Agent，第 3 个排队
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        // 提高并发限制
        manager.adjust_max_concurrent(4).unwrap();
        
        assert_eq!(manager.max_concurrent, 4);
        assert_eq!(manager.running_count, 3); // agent3 应该自动启动
        assert!(manager.agent_queue.is_empty());
    }

    #[test]
    fn test_adjust_max_concurrent_decrease() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 4,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        // 启动 4 个 Agent
        for _ in 0..4 {
            let agent_id = manager.spawn_agent("coding").unwrap();
            manager.try_start_agent(&agent_id);
        }
        
        assert_eq!(manager.running_count, 4);
        
        // 降低并发限制到 2
        manager.adjust_max_concurrent(2).unwrap();
        
        assert_eq!(manager.max_concurrent, 2);
        assert_eq!(manager.running_count, 2);
        assert_eq!(manager.agent_queue.len(), 2); // 2 个被暂停的进入队列
    }

    #[test]
    fn test_get_running_and_queued_agents() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        manager.start(config).unwrap();
        
        let agent1 = manager.spawn_agent("coding").unwrap();
        let agent2 = manager.spawn_agent("coding").unwrap();
        let agent3 = manager.spawn_agent("coding").unwrap();
        
        manager.try_start_agent(&agent1);
        manager.try_start_agent(&agent2);
        manager.try_start_agent(&agent3);
        
        let running = manager.get_running_agents();
        let queued = manager.get_queued_agents();
        
        assert_eq!(running.len(), 2);
        assert_eq!(queued.len(), 1);
    }

    #[test]
    fn test_adjust_max_concurrent_zero_error() {
        let mut manager = DaemonManager::new();
        let config = DaemonConfig {
            session_id: "session-001".to_string(),
            project_path: "/tmp/test".to_string(),
            log_level: "info".to_string(),
            max_concurrent_agents: 2,
            workspace_dir: "/tmp".to_string(),
        };
        
        // 启动守护进程
        let result = manager.start(config);
        assert!(result.is_ok());
        
        // 验证不能设置为 0
        let adjust_result = manager.adjust_max_concurrent(0);
        assert!(adjust_result.is_err());
        assert_eq!(adjust_result.unwrap_err(), "max_concurrent must be greater than 0");
    }

    // ========== VC-012: Coding Agent 测试 ==========

    #[test]
    fn test_coding_agent_creation() {
        use crate::ai::AIConfig;
        
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
        use crate::ai::AIConfig;
        
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
        use crate::ai::AIConfig;
        
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

    // ========== VC-014: Branch Manager 测试 ==========

    #[test]
    fn test_branch_manager_creation() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test-project".to_string(),
            default_base_branch: "main".to_string(),
            name_prefix: Some("issue-".to_string()),
        };

        let manager = BranchManager::new(config.clone());

        assert_eq!(manager.config.project_path, "/tmp/test-project");
        assert_eq!(manager.config.default_base_branch, "main");
        assert_eq!(manager.config.name_prefix, Some("issue-".to_string()));
        assert!(manager.current_branch.is_none());
        assert!(manager.created_branches.is_empty());
    }

    #[test]
    fn test_generate_branch_name_feature() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test".to_string(),
            default_base_branch: "main".to_string(),
            name_prefix: None,
        };

        let manager = BranchManager::new(config);

        // 测试基本功能分支名称
        let name = manager.generate_branch_name(BranchType::Feature, "Add User Login", None);
        println!("Generated branch name 1: {}", name);
        assert!(name.starts_with("feature/"));
        assert!(name.contains("add-user-login"));

        // 测试带 Issue ID 的分支名称（无自定义前缀）
        let name_with_issue = manager.generate_branch_name(
            BranchType::Feature,
            "Implement Dashboard",
            Some("PROJ-123"),
        );
        println!("Generated branch name 2: {}", name_with_issue);
        // 格式应该是：feature/PROJ-123/implement-dashboard (Issue ID 保持原样)
        assert!(name_with_issue.starts_with("feature/PROJ-123/"));
        assert!(name_with_issue.contains("implement-dashboard"));
    }

    #[test]
    fn test_generate_branch_name_with_prefix() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test".to_string(),
            default_base_branch: "develop".to_string(),
            name_prefix: Some("issue-".to_string()),
        };

        let manager = BranchManager::new(config);

        let name = manager.generate_branch_name(
            BranchType::Feature,
            "Add Payment Gateway",
            Some("PAY-456"),
        );
        
        // 格式应该是：feature/PAY-456/issue-/add-payment-gateway
        assert!(name.starts_with("feature/PAY-456/"));
        assert!(name.contains("issue-"));
        assert!(name.contains("add-payment-gateway"));
    }

    #[test]
    fn test_validate_branch_name_valid() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test".to_string(),
            default_base_branch: "main".to_string(),
            name_prefix: None,
        };

        let manager = BranchManager::new(config);

        // 测试有效的分支名称
        assert!(manager.validate_branch_name("feature/add-login").is_ok());
        assert!(manager.validate_branch_name("fix/bug-fix-123").is_ok());
        assert!(manager.validate_branch_name("release/v1.0.0").is_ok());
        assert!(manager.validate_branch_name("hotfix/critical-fix").is_ok());
    }

    #[test]
    fn test_validate_branch_name_invalid() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test".to_string(),
            default_base_branch: "main".to_string(),
            name_prefix: None,
        };

        let manager = BranchManager::new(config);

        // 测试无效的分支名称
        assert!(manager.validate_branch_name("").is_err());
        assert!(manager.validate_branch_name(&"a".repeat(256)).is_err());
        assert!(manager.validate_branch_name("feature/name with space").is_err());
        assert!(manager.validate_branch_name("feature/name~tilde").is_err());
        assert!(manager.validate_branch_name("/feature/no-leading-slash").is_err());
        assert!(manager.validate_branch_name("feature/no-trailing-slash/").is_err());
        assert!(manager.validate_branch_name("invalid/type/name").is_err());
    }

    #[test]
    fn test_detect_branch_type() {
        let config = BranchManagerConfig {
            project_path: "/tmp/test".to_string(),
            default_base_branch: "main".to_string(),
            name_prefix: None,
        };

        let manager = BranchManager::new(config);

        // 使用反射或私有方法测试比较困难，这里通过 generate_branch_name 间接测试
        assert!(manager.generate_branch_name(BranchType::Feature, "test", None).starts_with("feature/"));
        assert!(manager.generate_branch_name(BranchType::Fix, "test", None).starts_with("fix/"));
        assert!(manager.generate_branch_name(BranchType::Release, "test", None).starts_with("release/"));
        assert!(manager.generate_branch_name(BranchType::Hotfix, "test", None).starts_with("hotfix/"));
    }

    #[test]
    fn test_branch_operation_result() {
        let success_result = BranchOperationResult {
            success: true,
            branch_name: Some("feature/test".to_string()),
            error: None,
            message: Some("Success".to_string()),
        };

        assert!(success_result.success);
        assert_eq!(success_result.branch_name, Some("feature/test".to_string()));
        assert!(success_result.error.is_none());

        let error_result = BranchOperationResult {
            success: false,
            branch_name: None,
            error: Some("Git command failed".to_string()),
            message: None,
        };

        assert!(!error_result.success);
        assert!(error_result.error.is_some());
    }

    #[test]
    fn test_branch_info_structure() {
        let branch_info = BranchInfo {
            name: "feature/test-feature".to_string(),
            branch_type: BranchType::Feature,
            is_current: true,
            last_commit_hash: Some("abc123".to_string()),
            last_commit_message: Some("Add new feature".to_string()),
            created_at: Some(1234567890),
        };

        assert_eq!(branch_info.name, "feature/test-feature");
        assert_eq!(branch_info.branch_type, BranchType::Feature);
        assert!(branch_info.is_current);
        assert_eq!(branch_info.last_commit_hash, Some("abc123".to_string()));
    }

}


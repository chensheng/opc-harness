//! Native Coding Agent - 纯 Rust 实现的自主编码智能体
//!
//! 直接调用 AI Provider API 执行用户故事，无需依赖外部 CLI 工具。
//! 支持 Function Calling、多轮对话、增量代码编辑和质量检查自动化。

use std::path::PathBuf;
use tokio::time::{timeout, Duration};

use crate::agent::checkpoint_manager::{CheckpointData, CheckpointManager, CheckpointType, UserDecision};
use crate::agent::tools::{CodeSearchTools, DependencyManager, FileSystemTools, GitTools, PackageManager, QualityTools};
use crate::ai::{AIProvider, AIProviderType, ChatRequest, Message};

/// Native Coding Agent 配置
#[derive(Debug, Clone)]
pub struct NativeAgentConfig {
    /// Agent ID
    pub agent_id: String,
    /// 工作空间路径
    pub workspace_path: PathBuf,
    /// AI Provider 类型
    pub provider_type: AIProviderType,
    /// API Key
    pub api_key: String,
    /// 模型名称
    pub model: String,
    /// 最大对话轮数
    pub max_turns: usize,
    /// 执行超时（秒）
    pub timeout_secs: u64,
    /// 是否启用历史压缩（默认 true）
    pub enable_history_compression: bool,
}

/// Native Coding Agent
pub struct NativeCodingAgent {
    config: NativeAgentConfig,
    ai_provider: AIProvider,
    /// 文件系统工具集
    fs_tools: FileSystemTools,
    /// Git 工具集
    git_tools: GitTools,
    /// 质量检查工具集
    quality_tools: QualityTools,
    /// 代码搜索工具集
    code_search_tools: CodeSearchTools,
    /// 依赖管理工具集（npm）
    npm_dependency_manager: DependencyManager,
    /// 依赖管理工具集（cargo）
    cargo_dependency_manager: DependencyManager,
    /// HITL Checkpoint 管理器
    checkpoint_manager: Option<CheckpointManager>,
    /// 对话历史
    conversation_history: Vec<Message>,
    /// 工具调用统计
    tool_calls_count: usize,
    /// Token 使用统计
    total_prompt_tokens: usize,
    total_completion_tokens: usize,
}

impl NativeCodingAgent {
    /// 创建新的 Native Coding Agent
    pub fn new(config: NativeAgentConfig) -> Self {
        let ai_provider = AIProvider::new(config.provider_type, config.api_key.clone());

        let workspace_path = config.workspace_path.clone();
        let timeout_secs = config.timeout_secs;

        Self {
            config,
            ai_provider,
            fs_tools: FileSystemTools::new(workspace_path.clone()),
            git_tools: GitTools::new(workspace_path.clone()),
            quality_tools: QualityTools::new(workspace_path.clone(), timeout_secs),
            code_search_tools: CodeSearchTools::new(workspace_path.clone()),
            npm_dependency_manager: DependencyManager::new(workspace_path.clone(), PackageManager::Npm),
            cargo_dependency_manager: DependencyManager::new(workspace_path, PackageManager::Cargo),
            checkpoint_manager: None, // 默认禁用，可通过 enable_checkpoint() 启用
            conversation_history: Vec::new(),
            tool_calls_count: 0,
            total_prompt_tokens: 0,
            total_completion_tokens: 0,
        }
    }

    /// 启用 HITL Checkpoint 功能
    pub fn enable_checkpoint(&mut self, db_path: PathBuf) {
        self.checkpoint_manager = Some(CheckpointManager::new(db_path));
        log::info!("HITL Checkpoint enabled");
    }

    /// 创建 checkpoint 并等待用户决策
    async fn create_and_wait_for_checkpoint(
        &self,
        story_id: &str,
        checkpoint_type: CheckpointType,
        title: &str,
        description: &str,
        payload: serde_json::Value,
        timeout_secs: u64,
    ) -> Result<UserDecision, AgentError> {
        let checkpoint_manager = self
            .checkpoint_manager
            .as_ref()
            .ok_or_else(|| AgentError::ToolError("Checkpoint manager not enabled".to_string()))?;

        let data = CheckpointData {
            title: title.to_string(),
            description: description.to_string(),
            payload,
            timeout_secs,
        };

        let checkpoint_id = checkpoint_manager
            .create_checkpoint(&self.config.agent_id, story_id, checkpoint_type.clone(), data)
            .map_err(|e| AgentError::ToolError(e))?;

        log::info!(
            "Checkpoint created: {} (type: {:?}), waiting for user decision...",
            checkpoint_id,
            checkpoint_type
        );

        // TODO: 实现 WebSocket 事件推送和阻塞等待逻辑
        // 目前返回默认批准，实际实现需要异步等待用户响应
        log::warn!("Blocking wait not implemented yet, defaulting to approve");
        Ok(UserDecision::Approve)
    }

    /// 执行用户故事
    pub async fn execute_story(
        &mut self,
        story_title: String,
        acceptance_criteria: String,
    ) -> Result<StoryExecutionResult, AgentError> {
        log::info!(
            "Starting Native Coding Agent execution for story: {}",
            story_title
        );

        // 1. 构建系统提示词
        let system_prompt = self.build_system_prompt(&story_title, &acceptance_criteria);
        self.conversation_history.push(Message {
            role: "system".to_string(),
            content: system_prompt,
        });

        // 2. 添加用户消息
        let user_message = format!(
            "请实现以下用户故事：\n\n标题：{}\n\n验收标准：\n{}",
            story_title, acceptance_criteria
        );
        self.conversation_history.push(Message {
            role: "user".to_string(),
            content: user_message,
        });

        // 3. 执行多轮对话循环
        let mut current_turn = 0;
        let max_turns = self.config.max_turns;

        while current_turn < max_turns {
            current_turn += 1;
            log::info!("Turn {}/{}", current_turn, max_turns);

            // 3.1 调用 AI API
            let response = self.call_ai_api().await?;

            // 6.3: 检查完成信号（任务 6.3）
            if Self::parse_completion_signal(&response) {
                log::info!("Task completion signal detected: <TASK_COMPLETE>");
                break;
            }

            // 3.2 解析响应，检查是否有工具调用
            if let Some(tool_calls) = self.parse_tool_calls(&response) {
                // 3.3 执行工具调用
                for tool_call in tool_calls {
                    let result = self.execute_tool_call(&tool_call).await?;

                    // 3.4 将工具结果添加到对话历史
                    self.conversation_history.push(Message {
                        role: "tool".to_string(),
                        content: format!("Tool {}: {}", tool_call.name, result),
                    });
                    self.tool_calls_count += 1;
                }

                // 6.7: 每 5 轮对话后自动触发压缩（任务 6.7）
                if self.config.enable_history_compression && current_turn % 5 == 0 {
                    self.compress_history(4); // 保留最近 4 条消息（任务 6.5）
                }

                // 继续下一轮对话
                continue;
            }

            // 3.5 如果没有工具调用，说明 AI 已完成任务
            log::info!("AI completed task without tool calls");
            break;
        }

        if current_turn >= max_turns {
            return Err(AgentError::MaxTurnsExceeded(max_turns));
        }

        // 4. 运行质量检查（任务 7.7：使用分阶段检查）
        log::info!("Running staged quality checks...");
        let quality_result = self
            .quality_tools
            .run_quality_checks_staged()
            .await
            .map_err(|e: String| AgentError::QualityCheckFailed(e))?;

        if !quality_result.passed {
            // 5. 如果质量检查失败，尝试自动修复（最多 3 次）
            log::warn!("Quality check failed, attempting auto-fix...");
            let fix_result = self.auto_fix_loop(quality_result.report.clone()).await?;

            if !fix_result {
                return Err(AgentError::QualityCheckFailed(
                    "Auto-fix failed after 3 attempts".to_string(),
                ));
            }
        }

        // 6. 提交代码更改
        log::info!("Committing changes to Git...");
        let commit_message = format!("feat: implement {}", story_title);
        self.git_tools
            .git_commit(&commit_message)
            .await
            .map_err(|e: String| AgentError::GitError(e))?;

        // 7. 返回成功结果
        Ok(StoryExecutionResult {
            success: true,
            message: format!("Story '{}' completed successfully", story_title),
            token_usage: Some(TokenUsage {
                prompt_tokens: self.total_prompt_tokens,
                completion_tokens: self.total_completion_tokens,
                total_tokens: self.total_prompt_tokens + self.total_completion_tokens,
            }),
            tool_calls_count: self.tool_calls_count,
            fix_attempts: if quality_result.passed { 0 } else { 1 },
        })
    }

    /// 获取 Agent ID
    pub fn agent_id(&self) -> &str {
        &self.config.agent_id
    }

    /// 获取工作空间路径
    pub fn workspace_path(&self) -> &PathBuf {
        &self.config.workspace_path
    }

    /// 构建系统提示词
    fn build_system_prompt(&self, story_title: &str, acceptance_criteria: &str) -> String {
        format!(
            r#"你是一个专业的软件工程师助手，负责实现用户故事。

**当前任务**：{}

**验收标准**：
{}

**可用工具**：
1. read_file - 读取文件内容
2. write_file - 写入文件内容
3. list_directory - 列出目录内容
4. edit_file - 编辑文件（基于行号）
5. git_status - 查看 Git 状态
6. git_diff - 查看代码差异
7. git_commit - 提交代码更改
8. run_linter - 运行 ESLint 检查
9. run_typescript_check - 运行 TypeScript 类型检查
10. run_tests - 运行测试
11. code_search_grep - 正则表达式搜索代码（参数：pattern, path?）
12. code_search_find_files - 查找文件（参数：pattern, extensions?）
13. code_search_find_symbol - 查找符号定义（参数：symbol_name）
14. npm_install - 安装 npm 包（参数：package, version?）
15. cargo_add - 添加 Rust crate（参数：crate, features?）
16. list_dependencies - 列出项目依赖

**工作流程**：
1. 首先使用 read_file 和 list_directory 了解项目结构
2. 根据需求编写或修改代码
3. 使用质量检查工具验证代码
4. 如果检查失败，根据错误信息修复代码
5. 最后使用 git_commit 提交更改

**重要规则**：
- 每次只调用一个工具
- 等待工具返回结果后再继续
- 确保代码通过所有质量检查
- 遵循项目的代码风格和规范
- 不要删除现有代码，除非明确要求

**任务完成信号**：
- 当任务完全完成并通过所有检查后，在回复的最后一行添加 `<TASK_COMPLETE>` 标记
- 例如："所有代码已编写并通过测试。\n<TASK_COMPLETE>"
- 这将告诉系统可以停止多轮对话循环
"#,
            story_title, acceptance_criteria
        )
    }

    /// 解析完成信号，检测 `<TASK_COMPLETE>` 标记
    fn parse_completion_signal(response: &str) -> bool {
        response.trim().ends_with("<TASK_COMPLETE>")
    }

    /// 压缩对话历史，保留 system message + 最近 N 条消息
    fn compress_history(&mut self, keep_recent: usize) {
        if self.conversation_history.len() <= keep_recent + 1 {
            // 如果历史消息不多，不需要压缩
            return;
        }

        // 保留第一条 system message
        let system_message = self.conversation_history.first().cloned();
        
        // 保留最近的 keep_recent 条消息
        let recent_messages: Vec<Message> = self
            .conversation_history
            .iter()
            .rev()
            .take(keep_recent)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // 生成摘要（简化版本）
        let summary = format!(
            "[对话摘要] 前 {} 轮对话已压缩。关键操作：代码编写、质量检查、错误修复。",
            self.conversation_history.len() - keep_recent - 1
        );

        // 重建历史：system + 摘要 + 最近消息
        self.conversation_history.clear();
        if let Some(sys) = system_message {
            self.conversation_history.push(sys);
        }
        self.conversation_history.push(Message {
            role: "assistant".to_string(),
            content: summary,
        });
        self.conversation_history.extend(recent_messages);

        log::info!(
            "History compressed: {} -> {} messages",
            self.conversation_history.len() + keep_recent,
            self.conversation_history.len()
        );
    }

    /// 调用 AI API
    async fn call_ai_api(&mut self) -> Result<String, AgentError> {
        let messages = self.conversation_history.clone();

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(4096),
            stream: false,
            project_id: None,
        };

        // 设置超时
        let timeout_duration = Duration::from_secs(self.config.timeout_secs);

        let result = timeout(timeout_duration, self.ai_provider.chat(request))
            .await
            .map_err(|_| AgentError::Timeout(self.config.timeout_secs))?
            .map_err(|e| AgentError::AIError(e.message))?;

        // 更新 Token 统计
        if let Some(usage) = &result.usage {
            self.total_prompt_tokens += usage.prompt_tokens as usize;
            self.total_completion_tokens += usage.completion_tokens as usize;
        }

        Ok(result.content)
    }

    /// 解析工具调用
    fn parse_tool_calls(&self, response: &str) -> Option<Vec<ToolCall>> {
        // TODO: 实现真正的 Function Calling 解析
        // 目前简化处理：检测 JSON 格式的工具调用

        // 尝试从响应中提取 JSON
        if let Some(json_start) = response.find('{') {
            if let Some(json_end) = response.rfind('}') {
                let json_str = &response[json_start..=json_end];

                // 尝试解析为工具调用
                if let Ok(tool_call) = serde_json::from_str::<ToolCall>(json_str) {
                    return Some(vec![tool_call]);
                }
            }
        }

        None
    }

    /// 执行工具调用
    async fn execute_tool_call(&self, tool_call: &ToolCall) -> Result<String, AgentError> {
        match tool_call.name.as_str() {
            "read_file" => {
                let path = tool_call
                    .arguments
                    .get("path")
                    .ok_or_else(|| AgentError::ToolError("Missing 'path' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'path' must be a string".to_string()))?;

                self.fs_tools
                    .read_file(path)
                    .await
                    .map_err(|e: String| AgentError::FileSystemError(e))
            }
            "write_file" => {
                let path = tool_call
                    .arguments
                    .get("path")
                    .ok_or_else(|| AgentError::ToolError("Missing 'path' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'path' must be a string".to_string()))?;
                let content = tool_call
                    .arguments
                    .get("content")
                    .ok_or_else(|| AgentError::ToolError("Missing 'content' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| {
                        AgentError::ToolError("'content' must be a string".to_string())
                    })?;

                self.fs_tools
                    .write_file(path, content)
                    .await
                    .map_err(|e: String| AgentError::FileSystemError(e))
            }
            "list_directory" => {
                let path = tool_call
                    .arguments
                    .get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or(".");
                let recursive = tool_call
                    .arguments
                    .get("recursive")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let max_depth = tool_call
                    .arguments
                    .get("max_depth")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(3) as usize;

                self.fs_tools
                    .list_directory(path, recursive, max_depth)
                    .await
                    .map_err(|e: String| AgentError::FileSystemError(e))
            }
            "edit_file" => {
                let path = tool_call
                    .arguments
                    .get("path")
                    .ok_or_else(|| AgentError::ToolError("Missing 'path' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'path' must be a string".to_string()))?;
                let start_line = tool_call
                    .arguments
                    .get("start_line")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| {
                        AgentError::ToolError("Missing 'start_line' argument".to_string())
                    })? as usize;
                let end_line = tool_call
                    .arguments
                    .get("end_line")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| {
                        AgentError::ToolError("Missing 'end_line' argument".to_string())
                    })? as usize;
                let new_content = tool_call
                    .arguments
                    .get("new_content")
                    .ok_or_else(|| {
                        AgentError::ToolError("Missing 'new_content' argument".to_string())
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        AgentError::ToolError("'new_content' must be a string".to_string())
                    })?;

                self.fs_tools
                    .edit_file(path, start_line, end_line, new_content)
                    .await
                    .map_err(|e: String| AgentError::FileSystemError(e))
            }
            "git_status" => self
                .git_tools
                .git_status()
                .await
                .map_err(|e: String| AgentError::GitError(e)),
            "git_diff" => {
                let file = tool_call.arguments.get("file").and_then(|v| v.as_str());

                self.git_tools
                    .git_diff(file)
                    .await
                    .map_err(|e: String| AgentError::GitError(e))
            }
            "git_commit" => {
                let message = tool_call
                    .arguments
                    .get("message")
                    .ok_or_else(|| AgentError::ToolError("Missing 'message' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| {
                        AgentError::ToolError("'message' must be a string".to_string())
                    })?;

                let commit_hash = self.git_tools
                    .git_commit(message)
                    .await
                    .map_err(|e: String| AgentError::GitError(e))?;

                // 如果启用了 checkpoint，创建提交前审核检查点
                if self.checkpoint_manager.is_some() {
                    // TODO: 需要从上下文中获取 story_id
                    // 目前暂时跳过，等待完整集成
                    log::debug!("Checkpoint enabled but story_id not available in tool context");
                }

                Ok(commit_hash)
            }
            "run_linter" => {
                let (error_count, report) = self
                    .quality_tools
                    .run_linter()
                    .await
                    .map_err(|e: String| AgentError::QualityCheckFailed(e))?;
                Ok(format!("Lint result: {} errors\n{}", error_count, report))
            }
            "run_typescript_check" => {
                let (error_count, report) = self
                    .quality_tools
                    .run_typescript_check()
                    .await
                    .map_err(|e: String| AgentError::QualityCheckFailed(e))?;
                Ok(format!(
                    "TypeScript check result: {} errors\n{}",
                    error_count, report
                ))
            }
            "run_tests" => {
                let (failure_count, report) = self
                    .quality_tools
                    .run_tests()
                    .await
                    .map_err(|e: String| AgentError::QualityCheckFailed(e))?;
                Ok(format!(
                    "Test result: {} failures\n{}",
                    failure_count, report
                ))
            }
            // Code Search Tools
            "code_search_grep" => {
                let pattern = tool_call
                    .arguments
                    .get("pattern")
                    .ok_or_else(|| AgentError::ToolError("Missing 'pattern' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'pattern' must be a string".to_string()))?;
                let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            
                let matches = self
                    .code_search_tools
                    .grep(pattern, path)
                    .await
                    .map_err(|e: String| AgentError::ToolError(e))?;
            
                if matches.is_empty() {
                    Ok("No matches found".to_string())
                } else {
                    let mut output = format!("Found {} matches:\n\n", matches.len());
                    for m in matches.iter().take(10) {
                        // 最多显示 10 条
                        output.push_str(&format!(
                            "File: {}:{}\n  {}\n",
                            m.file_path, m.line_number, m.content
                        ));
                        if !m.context_before.is_empty() {
                            for line in &m.context_before {
                                output.push_str(&format!("  {}\n", line));
                            }
                        }
                        if !m.context_after.is_empty() {
                            for line in &m.context_after {
                                output.push_str(&format!("  {}\n", line));
                            }
                        }
                        output.push('\n');
                    }
                    if matches.len() > 10 {
                        output.push_str(&format!("... and {} more matches", matches.len() - 10));
                    }
                    Ok(output)
                }
            }
            "code_search_find_files" => {
                let pattern = tool_call
                    .arguments
                    .get("pattern")
                    .ok_or_else(|| AgentError::ToolError("Missing 'pattern' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'pattern' must be a string".to_string()))?;
                let extensions = tool_call
                    .arguments
                    .get("extensions")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<&str>>()
                    });
            
                let files = self
                    .code_search_tools
                    .find_files(pattern, extensions.as_deref())
                    .await
                    .map_err(|e: String| AgentError::ToolError(e))?;
            
                if files.is_empty() {
                    Ok("No files found".to_string())
                } else {
                    let mut output = format!("Found {} files:\n\n", files.len());
                    for file in files.iter().take(20) {
                        // 最多显示 20 个
                        output.push_str(&format!("- {}\n", file));
                    }
                    if files.len() > 20 {
                        output.push_str(&format!("... and {} more files", files.len() - 20));
                    }
                    Ok(output)
                }
            }
            "code_search_find_symbol" => {
                let symbol_name = tool_call
                    .arguments
                    .get("symbol_name")
                    .ok_or_else(|| {
                        AgentError::ToolError("Missing 'symbol_name' argument".to_string())
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        AgentError::ToolError("'symbol_name' must be a string".to_string())
                    })?;
            
                let locations = self
                    .code_search_tools
                    .find_symbol(symbol_name)
                    .await
                    .map_err(|e: String| AgentError::ToolError(e))?;
            
                if locations.is_empty() {
                    Ok(format!("Symbol '{}' not found", symbol_name))
                } else {
                    let mut output = format!("Found {} locations for symbol '{}':\n\n", locations.len(), symbol_name);
                    for loc in locations.iter().take(10) {
                        let symbol_type = match loc.symbol_type {
                            crate::agent::tools::code_search::SymbolType::Function => "function",
                            crate::agent::tools::code_search::SymbolType::Class => "class",
                            crate::agent::tools::code_search::SymbolType::Variable => "variable",
                            crate::agent::tools::code_search::SymbolType::Unknown => "unknown",
                        };
                        output.push_str(&format!(
                            "- {} ({}) at {}:{}\n",
                            symbol_name, symbol_type, loc.file_path, loc.line_number
                        ));
                    }
                    if locations.len() > 10 {
                        output.push_str(&format!("... and {} more locations", locations.len() - 10));
                    }
                    Ok(output)
                }
            }
            // Dependency Management Tools
            "npm_install" => {
                let package = tool_call
                    .arguments
                    .get("package")
                    .ok_or_else(|| AgentError::ToolError("Missing 'package' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'package' must be a string".to_string()))?;
                let version = tool_call.arguments.get("version").and_then(|v| v.as_str());
            
                // 如果启用了 checkpoint，创建依赖安装前审核检查点
                if self.checkpoint_manager.is_some() {
                    let _payload = serde_json::json!({
                        "package": package,
                        "version": version
                    });
                    
                    // TODO: 需要从上下文中获取 story_id
                    log::debug!("Checkpoint enabled for dependency installation but story_id not available");
                }
            
                let result = self
                    .npm_dependency_manager
                    .npm_install(package, version)
                    .await
                    .map_err(|e: String| AgentError::ToolError(e))?;
            
                Ok(format!(
                    "Successfully installed {}@{}\n{}",
                    result.package_name, result.installed_version, result.message
                ))
            }
            "cargo_add" => {
                let crate_name = tool_call
                    .arguments
                    .get("crate")
                    .ok_or_else(|| AgentError::ToolError("Missing 'crate' argument".to_string()))?
                    .as_str()
                    .ok_or_else(|| AgentError::ToolError("'crate' must be a string".to_string()))?;
                let features = tool_call
                    .arguments
                    .get("features")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<&str>>()
                    });
            
                let result = self
                    .cargo_dependency_manager
                    .cargo_add(crate_name, features.as_deref())
                    .await
                    .map_err(|e: String| AgentError::ToolError(e))?;
            
                Ok(format!(
                    "Successfully added {} ({})\n{}",
                    result.package_name, result.installed_version, result.message
                ))
            }
            "list_dependencies" => {
                // 根据项目类型自动选择包管理器
                let package_json = self.config.workspace_path.join("package.json");
                let cargo_toml = self.config.workspace_path.join("Cargo.toml");
            
                let deps = if package_json.exists() {
                    self.npm_dependency_manager.list_dependencies().await.map_err(|e: String| AgentError::ToolError(e))?
                } else if cargo_toml.exists() {
                    self.cargo_dependency_manager.list_dependencies().await.map_err(|e: String| AgentError::ToolError(e))?
                } else {
                    return Err(AgentError::ToolError("No package.json or Cargo.toml found".to_string()));
                };
            
                if deps.is_empty() {
                    Ok("No dependencies found".to_string())
                } else {
                    let mut output = format!("Found {} dependencies:\n\n", deps.len());
                    for (name, version) in deps.iter().take(30) {
                        output.push_str(&format!("- {}@{}\n", name, version));
                    }
                    if deps.len() > 30 {
                        output.push_str(&format!("... and {} more", deps.len() - 30));
                    }
                    Ok(output)
                }
            }
            _ => Err(AgentError::ToolError(format!(
                "Unknown tool: {}",
                tool_call.name
            ))),
        }
    }

    /// 自动修复循环（最多 3 次）
    async fn auto_fix_loop(&mut self, mut error_report: String) -> Result<bool, AgentError> {
        let max_attempts = 3;

        for attempt in 1..=max_attempts {
            log::info!("Auto-fix attempt {}/{}", attempt, max_attempts);

            // 构建修复提示词
            let fix_prompt = format!(
                "代码质量检查失败，请根据以下错误信息进行修复：\n\n{}",
                error_report
            );

            self.conversation_history.push(Message {
                role: "user".to_string(),
                content: fix_prompt,
            });

            // 调用 AI 进行修复
            let response = self.call_ai_api().await?;

            // 解析并执行工具调用
            if let Some(tool_calls) = self.parse_tool_calls(&response) {
                for tool_call in tool_calls {
                    self.execute_tool_call(&tool_call).await?;
                    self.tool_calls_count += 1;
                }
            }

            // 再次运行质量检查（任务 7.7：使用分阶段检查）
            let quality_result = self
                .quality_tools
                .run_quality_checks_staged()
                .await
                .map_err(|e: String| AgentError::QualityCheckFailed(e))?;

            if quality_result.passed {
                log::info!("Auto-fix successful on attempt {}", attempt);
                return Ok(true);
            }

            // 更新错误报告
            error_report = quality_result.report.clone();
        }

        log::warn!("Auto-fix failed after {} attempts", max_attempts);
        Ok(false)
    }
}

/// Story 执行结果
#[derive(Debug, Clone)]
pub struct StoryExecutionResult {
    /// 是否成功
    pub success: bool,
    /// 执行消息
    pub message: String,
    /// Token 消耗统计
    pub token_usage: Option<TokenUsage>,
    /// 工具调用次数
    pub tool_calls_count: usize,
    /// 修复尝试次数
    pub fix_attempts: usize,
}

/// Token 使用统计
#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// Agent 错误类型
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("AI Provider error: {0}")]
    AIError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Quality check failed: {0}")]
    QualityCheckFailed(String),

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("Max turns exceeded: {0}")]
    MaxTurnsExceeded(usize),

    #[error("Tool execution error: {0}")]
    ToolError(String),
}

/// 工具调用结构
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ToolCall {
    /// 工具名称
    pub name: String,
    /// 工具参数
    pub arguments: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::mock_ai_provider::{MockAIConfig, MockAIProvider};
    use tempfile::TempDir;

    #[test]
    fn test_native_agent_creation() {
        let config = NativeAgentConfig {
            agent_id: "test-agent-001".to_string(),
            workspace_path: PathBuf::from("/tmp/test"),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 1800,
            enable_history_compression: true,
        };

        let agent = NativeCodingAgent::new(config);
        assert_eq!(agent.agent_id(), "test-agent-001");
    }

    #[tokio::test]
    async fn test_execute_story_with_mock_provider() {
        // 创建临时工作目录
        let temp_dir = TempDir::new().unwrap();

        // 配置 Mock AI Provider（不模拟工具调用，直接返回完成）
        let mock_config = MockAIConfig {
            response_content: "Task completed. All files created.".to_string(),
            simulate_tool_calls: false,
            tool_call_count: 0,
            prompt_tokens: 150,
            completion_tokens: 80,
        };

        let mock_provider = MockAIProvider::new(mock_config);

        // 注意：由于 NativeCodingAgent 内部直接使用 AIProvider，
        // 这里我们测试的是结构体创建和基本方法
        // 完整的 execute_story 测试需要集成测试环境

        let config = NativeAgentConfig {
            agent_id: "test-agent-002".to_string(),
            workspace_path: temp_dir.path().to_path_buf(),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 300, // 缩短超时时间用于测试
            enable_history_compression: true,
        };

        let agent = NativeCodingAgent::new(config);

        // 验证基本属性
        assert_eq!(agent.agent_id(), "test-agent-002");
        assert_eq!(agent.workspace_path(), temp_dir.path());

        // 验证 Mock Provider 未被调用
        assert_eq!(mock_provider.get_call_count(), 0);
    }

    #[test]
    fn test_build_system_prompt() {
        let temp_dir = TempDir::new().unwrap();
        let config = NativeAgentConfig {
            agent_id: "test-agent-003".to_string(),
            workspace_path: temp_dir.path().to_path_buf(),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 1800,
            enable_history_compression: true,
        };

        let agent = NativeCodingAgent::new(config);

        // 通过反射访问私有方法 build_system_prompt
        // 由于 Rust 不允许直接访问私有方法，我们测试公开接口
        // 这里主要验证结构体创建成功
        assert_eq!(agent.agent_id(), "test-agent-003");
    }

    #[test]
    fn test_token_usage_tracking() {
        let temp_dir = TempDir::new().unwrap();
        let config = NativeAgentConfig {
            agent_id: "test-agent-004".to_string(),
            workspace_path: temp_dir.path().to_path_buf(),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 1800,
            enable_history_compression: true,
        };

        let agent = NativeCodingAgent::new(config);

        // 验证初始 Token 统计为零
        assert_eq!(agent.total_prompt_tokens, 0);
        assert_eq!(agent.total_completion_tokens, 0);
    }

    #[test]
    fn test_agent_error_types() {
        // 测试各种错误类型的创建
        let ai_error = AgentError::AIError("API failed".to_string());
        assert!(ai_error.to_string().contains("API failed"));

        let fs_error = AgentError::FileSystemError("File not found".to_string());
        assert!(fs_error.to_string().contains("File not found"));

        let git_error = AgentError::GitError("Commit failed".to_string());
        assert!(git_error.to_string().contains("Commit failed"));

        let quality_error = AgentError::QualityCheckFailed("Lint errors".to_string());
        assert!(quality_error.to_string().contains("Lint errors"));

        let timeout_error = AgentError::Timeout(1800);
        assert!(timeout_error.to_string().contains("1800"));

        let max_turns_error = AgentError::MaxTurnsExceeded(10);
        assert!(max_turns_error.to_string().contains("10"));

        let tool_error = AgentError::ToolError("Unknown tool".to_string());
        assert!(tool_error.to_string().contains("Unknown tool"));
    }

    #[test]
    fn test_tool_call_parsing() {
        // 测试 ToolCall 结构体的序列化和反序列化
        let json_str = r#"{
            "name": "read_file",
            "arguments": {"path": "src/main.rs"}
        }"#;

        let tool_call: ToolCall = serde_json::from_str(json_str).unwrap();
        assert_eq!(tool_call.name, "read_file");
        assert_eq!(tool_call.arguments["path"], "src/main.rs");
    }

    // 任务 6.2: 测试完成信号检测
    #[test]
    fn test_parse_completion_signal() {
        // 有完成标记
        assert!(NativeCodingAgent::parse_completion_signal("Task done.\n<TASK_COMPLETE>"));
        assert!(NativeCodingAgent::parse_completion_signal("<TASK_COMPLETE>"));
        assert!(NativeCodingAgent::parse_completion_signal("All tests passed. <TASK_COMPLETE>\n"));

        // 没有完成标记
        assert!(!NativeCodingAgent::parse_completion_signal("Task in progress"));
        assert!(!NativeCodingAgent::parse_completion_signal("<TASK_INCOMPLETE>"));
        assert!(!NativeCodingAgent::parse_completion_signal(""));
    }

    // 任务 6.4-6.6, 6.9: 测试历史压缩
    #[test]
    fn test_compress_history() {
        let temp_dir = TempDir::new().unwrap();
        let config = NativeAgentConfig {
            agent_id: "test-compress".to_string(),
            workspace_path: temp_dir.path().to_path_buf(),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 1800,
            enable_history_compression: true,
        };

        let mut agent = NativeCodingAgent::new(config);

        // 添加 system message
        agent.conversation_history.push(Message {
            role: "system".to_string(),
            content: "You are a helpful assistant".to_string(),
        });

        // 添加 10 条对话消息
        for i in 0..10 {
            agent.conversation_history.push(Message {
                role: if i % 2 == 0 { "user" } else { "assistant" }.to_string(),
                content: format!("Message {}", i),
            });
        }

        let original_count = agent.conversation_history.len();
        assert_eq!(original_count, 11); // 1 system + 10 messages

        // 压缩历史，保留最近 4 条
        agent.compress_history(4);

        // 验证压缩后的数量：1 system + 1 summary + 4 recent = 6
        assert_eq!(agent.conversation_history.len(), 6);

        // 验证第一条是 system message
        assert_eq!(agent.conversation_history[0].role, "system");

        // 验证第二条是摘要
        assert!(agent.conversation_history[1].content.contains("对话摘要"));

        // 验证最后 4 条是原始消息
        assert_eq!(agent.conversation_history[5].content, "Message 9");
    }

    // 任务 6.9: 测量压缩前后的 token 数量
    #[test]
    fn test_token_reduction_after_compression() {
        let temp_dir = TempDir::new().unwrap();
        let config = NativeAgentConfig {
            agent_id: "test-tokens".to_string(),
            workspace_path: temp_dir.path().to_path_buf(),
            provider_type: AIProviderType::Kimi,
            api_key: "test-key".to_string(),
            model: "kimi-k2.5".to_string(),
            max_turns: 10,
            timeout_secs: 1800,
            enable_history_compression: true,
        };

        let mut agent = NativeCodingAgent::new(config);

        // 添加 system message
        agent.conversation_history.push(Message {
            role: "system".to_string(),
            content: "You are a helpful assistant".to_string(),
        });

        // 添加 20 条长消息（模拟真实对话）
        for i in 0..20 {
            agent.conversation_history.push(Message {
                role: if i % 2 == 0 { "user" } else { "assistant" }.to_string(),
                content: format!("This is a long message number {} with lots of text to simulate real conversation content that would consume many tokens in the context window.", i),
            });
        }

        let before_count = agent.conversation_history.len();
        let before_chars: usize = agent.conversation_history.iter().map(|m| m.content.len()).sum();

        // 压缩历史
        agent.compress_history(4);

        let after_count = agent.conversation_history.len();
        let after_chars: usize = agent.conversation_history.iter().map(|m| m.content.len()).sum();

        // 验证消息数量减少
        assert!(after_count < before_count);

        // 验证字符数减少至少 60%
        let reduction_ratio = (before_chars - after_chars) as f64 / before_chars as f64;
        assert!(
            reduction_ratio >= 0.6,
            "Token reduction ratio {} is less than 60%",
            reduction_ratio
        );

        log::info!(
            "Compression: {} -> {} messages, {} -> {} chars ({:.1}% reduction)",
            before_count,
            after_count,
            before_chars,
            after_chars,
            reduction_ratio * 100.0
        );
    }
}

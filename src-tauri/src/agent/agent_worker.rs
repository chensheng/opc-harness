//! Agent Worker - 完全去中心化的智能体
//!
//! 每个 Agent Worker 是一个独立的执行单元，拥有完整的 Agent Loop 逻辑：
//! - 定时查询数据库获取活跃 Sprint 和待处理的 User Stories
//! - 使用乐观锁竞争领取 Story
//! - 创建 Worktree 并启动 AI CLI 执行任务
//! - 完成后自动更新 Story 状态

use crate::agent::ai_cli_interaction::AICLIMessage;
use crate::agent::daemon::DaemonManager;
use crate::agent::daemon_types::StoryContext;
use crate::agent::retry_engine::{BackoffConfig, RetryDecision, RetryEngine};
use crate::agent::websocket_manager::WebSocketManager;
use crate::agent::worktree_manager::WorktreeManager;
use crate::db;
use crate::models::{UserStory, UserStoryRetryHistory};
use chrono::Utc;
use log;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::RwLock;

/// Agent Worker 配置
#[derive(Debug, Clone)]
pub struct AgentWorkerConfig {
    /// Worker 唯一标识
    pub worker_id: String,
    /// 项目 ID
    pub project_id: String,
    /// 查询间隔（秒）
    pub check_interval_secs: u64,
    /// 最大并发 Agent 数（预留，当前每个 Worker 只处理一个 Story）
    pub max_concurrent: usize,
    /// Tauri AppHandle（用于直接发送 WebSocket 事件）
    pub app_handle: Option<AppHandle>,
    /// Story 锁定超时时间（分钟），默认 30
    pub lock_timeout_minutes: u64,
}

impl Default for AgentWorkerConfig {
    fn default() -> Self {
        Self {
            worker_id: format!("worker-{}", uuid::Uuid::new_v4()),
            project_id: String::new(),
            check_interval_secs: 30, // 每 30 秒检查一次
            max_concurrent: 1,
            app_handle: None,
            lock_timeout_minutes: 30, // 默认 30 分钟超时
        }
    }
}

/// Agent Worker - 完全去中心化的智能体
pub struct AgentWorker {
    config: AgentWorkerConfig,
    daemon_manager: Arc<RwLock<DaemonManager>>,
    websocket_manager: Option<Arc<RwLock<WebSocketManager>>>,
    worktree_manager: Option<Arc<WorktreeManager>>,
    is_running: bool,
    current_story_id: Option<String>,
    /// 日志节流：记录每个 session 最后发送日志的时间戳（毫秒）
    last_log_timestamps: Arc<RwLock<std::collections::HashMap<String, u64>>>,
}

impl AgentWorker {
    /// 创建新的 Agent Worker
    pub fn new(config: AgentWorkerConfig, daemon_manager: Arc<RwLock<DaemonManager>>) -> Self {
        log::info!(
            "[AgentWorker:{}] Created with config: {:?}",
            config.worker_id,
            config
        );

        Self {
            config,
            daemon_manager,
            websocket_manager: None,
            worktree_manager: None,
            is_running: false,
            current_story_id: None,
            last_log_timestamps: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 设置 WebSocket Manager
    pub fn set_websocket_manager(&mut self, websocket_manager: Arc<RwLock<WebSocketManager>>) {
        self.websocket_manager = Some(websocket_manager);
        log::info!(
            "[AgentWorker:{}] WebSocket manager attached",
            self.config.worker_id
        );
    }

    /// 设置 Worktree Manager
    pub fn set_worktree_manager(&mut self, project_path: &str) {
        self.worktree_manager = Some(Arc::new(WorktreeManager::new(project_path)));
        log::info!(
            "[AgentWorker:{}] Worktree manager initialized for path: {}",
            self.config.worker_id,
            project_path
        );
    }

    /// 启动 Worker（开始独立运行 Agent Loop）
    pub async fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err(format!(
                "Agent Worker {} is already running",
                self.config.worker_id
            ));
        }

        self.is_running = true;
        log::info!(
            "[AgentWorker:{}] 🚀 Starting independent agent loop",
            self.config.worker_id
        );

        let worker_id = self.config.worker_id.clone();
        let project_id = self.config.project_id.clone();
        let check_interval = self.config.check_interval_secs;
        let lock_timeout_minutes = self.config.lock_timeout_minutes;
        let daemon_manager = self.daemon_manager.clone();
        let websocket_manager = self.websocket_manager.clone();
        let worktree_manager = self.worktree_manager.clone();
        let app_handle = self.config.app_handle.clone();
        let last_log_timestamps = self.last_log_timestamps.clone();

        // 在后台启动独立的 Agent Loop
        tokio::spawn(async move {
            log::info!(
                "[AgentWorker:{}] Independent loop started (interval: {}s)",
                worker_id,
                check_interval
            );

            loop {
                // 执行单次循环
                match Self::execute_cycle(
                    &worker_id,
                    &project_id,
                    &daemon_manager,
                    &websocket_manager,
                    &worktree_manager,
                    &app_handle,
                    &last_log_timestamps,
                    lock_timeout_minutes,
                )
                .await
                {
                    Ok(story_count) => {
                        if story_count > 0 {
                            log::info!(
                                "[AgentWorker:{}] ✅ Cycle completed. Started {} agent(s)",
                                worker_id,
                                story_count
                            );
                        } else {
                            log::debug!(
                                "[AgentWorker:{}] ⏸️  No pending stories in this cycle",
                                worker_id
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("[AgentWorker:{}] ❌ Cycle failed: {}", worker_id, e);
                    }
                }

                // 等待下一个周期
                tokio::time::sleep(Duration::from_secs(check_interval)).await;
            }
        });

        // 启动重试调度器
        if let Some(ws_manager) = &self.websocket_manager {
            let scheduler_config = crate::agent::retry_engine::SchedulerConfig::default();
            let mut scheduler = crate::agent::retry_engine::RetryScheduler::new(scheduler_config);
            let project_id_clone = self.config.project_id.clone();
            let ws_manager_clone = ws_manager.clone();

            tokio::spawn(async move {
                scheduler.run(project_id_clone, ws_manager_clone).await;
            });

            log::info!(
                "[AgentWorker:{}] 🔄 RetryScheduler started in background",
                self.config.worker_id
            );
        } else {
            log::warn!(
                "[AgentWorker:{}] WebSocket manager not available, RetryScheduler not started",
                self.config.worker_id
            );
        }

        Ok(())
    }

    /// 执行单次循环（查询数据库 → 乐观锁 → 启动 Agent）
    async fn execute_cycle(
        worker_id: &str,
        project_id: &str,
        daemon_manager: &Arc<RwLock<DaemonManager>>,
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        worktree_manager: &Option<Arc<WorktreeManager>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
        lock_timeout_minutes: u64,
    ) -> Result<usize, String> {
        log::info!(
            "[AgentWorker:{}] 🔄 Starting execution cycle for project: {}",
            worker_id,
            project_id
        );

        // Step 1: 查询活跃 Sprint（修复：添加项目隔离）
        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        log::info!(
            "[AgentWorker:{}] 🔄 Starting execution cycle for project: {}",
            worker_id,
            project_id
        );

        // ✅ 关键修复：传入 project_id 实现项目隔离
        let active_sprint = match db::get_active_sprint(&conn, project_id)
            .map_err(|e| format!("Failed to query active sprint: {}", e))?
        {
            Some(sprint) => sprint,
            None => {
                log::warn!(
                    "[AgentWorker:{}] ⚠️ No active Sprint found for project: {}",
                    worker_id,
                    project_id
                );

                return Ok(0);
            }
        };

        log::info!(
            "[AgentWorker:{}] ✅ Found active Sprint: {} ({})",
            worker_id,
            active_sprint.name,
            active_sprint.id
        );

        // Step 2: 加载待执行的 User Stories
        log::info!(
            "[AgentWorker:{}] 📋 Querying pending stories for Sprint: {}",
            worker_id,
            active_sprint.name
        );

        let pending_stories =
            db::get_pending_stories_by_sprint(&conn, &active_sprint.id, project_id)
                .map_err(|e| format!("Failed to query pending stories: {}", e))?;

        if pending_stories.is_empty() {
            log::info!(
                "[AgentWorker:{}] No pending stories in Sprint {}",
                worker_id,
                active_sprint.name
            );

            return Ok(0);
        }

        log::info!(
            "[AgentWorker:{}] Found {} pending story(s)",
            worker_id,
            pending_stories.len()
        );

        // Step 3: 尝试锁定第一个可用的 Story（乐观锁）
        let mut started_count = 0;

        // ✅ 修复：直接使用 worker_id 作为 agent_id（worker_id 就是创建智能体时保存在数据库中的 agent_id）
        let agent_id = worker_id.to_string();

        log::info!(
            "[AgentWorker:{}] Using database agent_id for this cycle: {}",
            worker_id,
            agent_id
        );

        for story in pending_stories.iter() {
            // 乐观锁：将 locked_by 字段设置为 agent_id
            match db::lock_user_story(&conn, &story.id, &agent_id, lock_timeout_minutes) {
                Ok(locked) => {
                    if !locked {
                        log::info!(
                            "[AgentWorker:{}] ⚠️ Story {} already locked by another agent, skipping",
                            worker_id,
                            story.story_number
                        );
                        continue;
                    }

                    log::info!(
                        "[AgentWorker:{}] 🔒 Locked story {}: {}",
                        worker_id,
                        story.story_number,
                        story.title
                    );

                    // Step 4: 启动 Coding Agent
                    match Self::start_coding_agent(
                        &agent_id,
                        story,
                        project_id,
                        daemon_manager,
                        websocket_manager,
                        worktree_manager,
                        app_handle,
                        last_log_timestamps,
                    )
                    .await
                    {
                        Ok(_) => {
                            started_count += 1;
                            log::info!(
                                "[AgentWorker:{}] 🚀 Started agent {} for story {}",
                                worker_id,
                                agent_id,
                                story.story_number
                            );

                            // 每个 Worker 每次循环只处理一个 Story
                            break;
                        }
                        Err(e) => {
                            log::error!(
                                "[AgentWorker:{}] ❌ Failed to start agent for story {}: {}",
                                worker_id,
                                story.story_number,
                                e
                            );

                            // 解锁 Story，允许其他 Agent 重试
                            if let Err(unlock_err) = db::unlock_user_story(&conn, &story.id) {
                                log::warn!(
                                    "[AgentWorker:{}] Failed to unlock story {}: {}",
                                    worker_id,
                                    story.story_number,
                                    unlock_err
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "[AgentWorker:{}] Failed to lock story {}: {}",
                        worker_id,
                        story.story_number,
                        e
                    );
                }
            }
        }

        Ok(started_count)
    }

    /// 启动 Coding Agent（复用现有逻辑）
    async fn start_coding_agent(
        agent_id: &str,
        story: &UserStory,
        project_id: &str,
        _daemon_manager: &Arc<RwLock<DaemonManager>>,
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        worktree_manager: &Option<Arc<WorktreeManager>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
    ) -> Result<(), String> {
        // 使用 Native Coding Agent
        log::info!("[AgentWorker:{}] 🚀 Using Native Coding Agent", agent_id);

        // 发送实时日志：Native Agent 启动
        Self::send_ws_log_to_both_for_coding(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            agent_id,
            project_id,
            "progress",
            "🚀 启动 Native Coding Agent...",
            Some("AgentWorker"),
        )
        .await;

        Self::execute_native_agent(
            agent_id,
            story,
            project_id,
            websocket_manager,
            worktree_manager,
            app_handle,
            last_log_timestamps,
        )
        .await
    }

    /// 执行 Native Coding Agent
    async fn execute_native_agent(
        agent_id: &str,
        story: &UserStory,
        project_id: &str,
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        worktree_manager: &Option<Arc<WorktreeManager>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
    ) -> Result<(), String> {
        use crate::agent::native_coding_agent::{NativeAgentConfig, NativeCodingAgent};
        use crate::agent::worktree_lifecycle::WorktreeLifecycleManager;
        use crate::ai::AIProviderType;

        log::info!(
            "[AgentWorker:{}] Starting Native Coding Agent for story {}",
            agent_id,
            story.story_number
        );

        // 1. 从数据库获取 Story 上下文
        let story_context = Self::get_story_context(&story.id)?;

        // 2. 创建 Worktree
        let (worktree_path, branch_name) = if let Some(ref wt_manager) = worktree_manager {
            let branch_name = format!("story-{}", story.story_number);

            Self::send_ws_log_to_both_for_coding(
                websocket_manager,
                app_handle,
                last_log_timestamps,
                agent_id,
                project_id,
                "progress",
                &format!("🌿 创建工作树分支: {}", branch_name),
                Some("AgentWorker"),
            )
            .await;

            match wt_manager
                .create_worktree(agent_id, &story.id, &branch_name)
                .await
            {
                Ok(path) => {
                    log::info!("[AgentWorker:{}] Worktree created at: {}", agent_id, path);

                    Self::send_ws_log_to_both_for_coding(
                        websocket_manager,
                        app_handle,
                        last_log_timestamps,
                        agent_id,
                        project_id,
                        "success",
                        &format!("✅ 工作树创建成功: {}", path),
                        Some("AgentWorker"),
                    )
                    .await;
                    (path, branch_name)
                }
                Err(e) => {
                    log::error!(
                        "[AgentWorker:{}] Failed to create worktree: {}, falling back to project root",
                        agent_id,
                        e
                    );

                    Self::send_ws_log_to_both_for_coding(
                        websocket_manager,
                        app_handle,
                        last_log_timestamps,
                        agent_id,
                        project_id,
                        "error",
                        &format!("❌ 工作树创建失败: {}", e),
                        Some("AgentWorker"),
                    )
                    .await;

                    let workspaces_root = crate::utils::paths::get_workspaces_dir();
                    (workspaces_root.to_string_lossy().to_string(), branch_name)
                }
            }
        } else {
            log::warn!(
                "[AgentWorker:{}] Worktree manager not initialized, using project root",
                agent_id
            );
            let workspaces_root = crate::utils::paths::get_workspaces_dir();
            (workspaces_root.to_string_lossy().to_string(), String::new())
        };

        // 3. 配置 Native Agent
        let api_key = std::env::var("VITE_AI_API_KEY").unwrap_or_else(|_| "test-key".to_string());
        let model = std::env::var("VITE_AI_MODEL").unwrap_or_else(|_| "kimi-k2.5".to_string());
        let provider_type_str =
            std::env::var("VITE_AI_PROVIDER").unwrap_or_else(|_| "kimi".to_string());

        let provider_type = match provider_type_str.as_str() {
            "openai" => AIProviderType::OpenAI,
            "anthropic" => AIProviderType::Anthropic,
            "kimi" => AIProviderType::Kimi,
            "glm" => AIProviderType::GLM,
            "minimax" => AIProviderType::MiniMax,
            _ => AIProviderType::Kimi,
        };

        let config = NativeAgentConfig {
            agent_id: agent_id.to_string(),
            workspace_path: std::path::PathBuf::from(&worktree_path),
            provider_type,
            api_key,
            model,
            max_turns: 10,
            timeout_secs: 1800, // 30 分钟
        };

        let mut native_agent = NativeCodingAgent::new(config);

        // 4. 发送实时日志：开始执行
        Self::send_ws_log_to_both_for_coding(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            agent_id,
            project_id,
            "progress",
            &format!("🤖 Native Agent 开始执行故事: {}", story.title),
            Some("AgentWorker"),
        )
        .await;

        // 5. 执行用户故事
        let title = story_context.title.unwrap_or_else(|| story.title.clone());
        let acceptance_criteria = story_context
            .acceptance_criteria
            .unwrap_or_else(|| story.acceptance_criteria.clone());

        match native_agent.execute_story(title, acceptance_criteria).await {
            Ok(result) => {
                log::info!(
                    "[AgentWorker:{}] ✅ Native Agent completed: {}",
                    agent_id,
                    result.message
                );

                // 发送完成日志
                Self::send_ws_log_to_both_for_coding(
                    websocket_manager,
                    app_handle,
                    last_log_timestamps,
                    agent_id,
                    project_id,
                    "success",
                    &format!(
                        "✅ 任务完成: {}\nToken 消耗: {} prompt + {} completion = {} total",
                        result.message,
                        result
                            .token_usage
                            .as_ref()
                            .map(|u| u.prompt_tokens)
                            .unwrap_or(0),
                        result
                            .token_usage
                            .as_ref()
                            .map(|u| u.completion_tokens)
                            .unwrap_or(0),
                        result
                            .token_usage
                            .as_ref()
                            .map(|u| u.total_tokens)
                            .unwrap_or(0)
                    ),
                    Some("AgentWorker"),
                )
                .await;

                // 更新 Story 状态为 completed
                if let Err(e) = Self::update_story_status_to_completed(&story.id).await {
                    log::error!(
                        "[AgentWorker:{}] Failed to update story status: {}",
                        agent_id,
                        e
                    );
                }

                // 清理 Worktree（任务 5.5-5.6）
                if !branch_name.is_empty() && worktree_manager.is_some() {
                    let lifecycle_manager = WorktreeLifecycleManager::new(
                        std::path::PathBuf::from(&worktree_path),
                        false, // preserve_branches 默认为 false
                    );

                    match lifecycle_manager
                        .cleanup_with_retry(
                            agent_id,
                            &story.id,
                            "success",
                            &worktree_path,
                            &branch_name,
                            3, // 最多重试 3 次
                        )
                        .await
                    {
                        Ok(cleanup_result) => {
                            log::info!(
                                "[AgentWorker:{}] Worktree cleanup result: {}",
                                agent_id,
                                cleanup_result.message
                            );
                        }
                        Err(e) => {
                            log::warn!(
                                "[AgentWorker:{}] Worktree cleanup failed (non-critical): {}",
                                agent_id,
                                e
                            );
                        }
                    }
                }

                Ok(())
            }
            Err(e) => {
                log::error!("[AgentWorker:{}] ❌ Native Agent failed: {}", agent_id, e);

                // 发送错误日志
                Self::send_ws_log_to_both_for_coding(
                    websocket_manager,
                    app_handle,
                    last_log_timestamps,
                    agent_id,
                    project_id,
                    "error",
                    &format!("❌ Native Agent 执行失败: {}", e),
                    Some("AgentWorker"),
                )
                .await;

                // 更新 Story 状态为 failed
                if let Err(update_err) = Self::update_story_status_to_failed(
                    &story.id,
                    &format!("Native Agent error: {}", e),
                )
                .await
                {
                    log::error!(
                        "[AgentWorker:{}] Failed to update story status: {}",
                        agent_id,
                        update_err
                    );
                }

                // 清理 Worktree（任务 5.5-5.6）- 即使失败也要清理
                if !branch_name.is_empty() && worktree_manager.is_some() {
                    let lifecycle_manager = WorktreeLifecycleManager::new(
                        std::path::PathBuf::from(&worktree_path),
                        false, // preserve_branches 默认为 false
                    );

                    match lifecycle_manager
                        .cleanup_with_retry(
                            agent_id,
                            &story.id,
                            "failed",
                            &worktree_path,
                            &branch_name,
                            3, // 最多重试 3 次
                        )
                        .await
                    {
                        Ok(cleanup_result) => {
                            log::info!(
                                "[AgentWorker:{}] Worktree cleanup result: {}",
                                agent_id,
                                cleanup_result.message
                            );
                        }
                        Err(e) => {
                            log::warn!(
                                "[AgentWorker:{}] Worktree cleanup failed (non-critical): {}",
                                agent_id,
                                e
                            );
                        }
                    }
                }

                Err(format!("Native Agent execution failed: {}", e))
            }
        }
    }

    /// 执行 CLI-based Coding Agent（原有逻辑）
    async fn execute_cli_agent(
        agent_id: &str,
        story: &UserStory,
        project_id: &str,
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        worktree_manager: &Option<Arc<WorktreeManager>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
    ) -> Result<(), String> {
        use crate::agent::daemon_types::AICLIConfig;
        use std::process::Stdio;
        use tokio::process::Command as TokioCommand;
        use tokio::sync::mpsc;

        // Clone agent_id to String for use in spawned tasks
        let agent_id_owned = agent_id.to_string();
        // let session_id = format!("agent-{}", agent_id); // 未使用 - TODO: 未来用于会话跟踪

        log::info!(
            "[AgentWorker:{}] Starting coding agent for story {}",
            agent_id,
            story.story_number
        );

        // 从数据库获取 Story 上下文
        let story_context = Self::get_story_context(&story.id)?;

        // 创建 Worktree
        let worktree_path = if let Some(ref wt_manager) = worktree_manager {
            let branch_name = format!("story-{}", story.story_number);

            // 📤 发送实时日志：创建 Worktree
            Self::send_ws_log_to_both_for_coding(
                websocket_manager,
                app_handle,
                last_log_timestamps,
                agent_id,
                project_id,
                "progress",
                &format!("🌿 创建工作树分支: {}", branch_name),
                Some("AgentWorker"),
            )
            .await;

            match wt_manager
                .create_worktree(agent_id, &story.id, &branch_name)
                .await
            {
                Ok(path) => {
                    log::info!("[AgentWorker:{}] Worktree created at: {}", agent_id, path);

                    // 📤 发送实时日志：Worktree 创建成功
                    Self::send_ws_log_to_both_for_coding(
                        websocket_manager,
                        app_handle,
                        last_log_timestamps,
                        agent_id,
                        project_id,
                        "success",
                        &format!("✅ 工作树创建成功: {}", path),
                        Some("AgentWorker"),
                    )
                    .await;
                    path
                }
                Err(e) => {
                    log::error!(
                        "[AgentWorker:{}] Failed to create worktree: {}, falling back to project root",
                        agent_id,
                        e
                    );

                    // 📤 发送实时日志：Worktree 创建失败
                    Self::send_ws_log_to_both_for_coding(
                        websocket_manager,
                        app_handle,
                        last_log_timestamps,
                        agent_id,
                        project_id,
                        "error",
                        &format!("❌ 工作树创建失败: {}", e),
                        Some("AgentWorker"),
                    )
                    .await;

                    // 回退到项目根目录
                    let workspaces_root = crate::utils::paths::get_workspaces_dir();
                    workspaces_root.to_string_lossy().to_string()
                }
            }
        } else {
            log::warn!(
                "[AgentWorker:{}] Worktree manager not initialized, using project root",
                agent_id
            );
            let workspaces_root = crate::utils::paths::get_workspaces_dir();
            workspaces_root.to_string_lossy().to_string()
        };

        // 构建 AICLIConfig
        let ai_config = AICLIConfig {
            command: "kimi".to_string(),
            working_dir: worktree_path.clone(),
            story_id: Some(story.id.clone()),
            story_title: story_context.title,
            acceptance_criteria: story_context.acceptance_criteria,
            agent_type: "coding".to_string(),
            extra_args: vec![],
        };

        // 构建 CLI 参数
        let args = ai_config.build_args();

        log::info!(
            "[AgentWorker:{}] Building CLI command for worktree with full context",
            agent_id
        );

        // 创建消息通道
        let (message_tx, mut message_rx) = mpsc::channel::<AICLIMessage>(100);

        // 启动 AI CLI 进程
        let child = TokioCommand::new(&ai_config.command)
            .args(&args)
            .current_dir(&worktree_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn AI CLI process: {}", e))?;

        log::info!(
            "[AgentWorker:{}] AI CLI process spawned with PID: {:?}",
            agent_id,
            child.id()
        );

        // 📤 发送实时日志：AI CLI 已启动
        Self::send_ws_log_to_both_for_coding(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            agent_id,
            project_id,
            "success",
            &format!("✅ AI 编码助手已启动 (PID: {:?})", child.id()),
            Some("AgentWorker"),
        )
        .await;

        // 创建交互管理器
        use crate::agent::ai_cli_interaction::AICLIInteraction;
        let interaction = AICLIInteraction::new(child, agent_id.to_string(), message_tx);

        // 📤 发送实时日志：开始监听 AI 输出
        Self::send_ws_log_to_both_for_coding(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            agent_id,
            project_id,
            "progress",
            "👂 开始监听 AI 输出...",
            Some("AgentWorker"),
        )
        .await;

        // 启动监听任务（带超时）
        let agent_id_for_listener = agent_id.to_string();
        // let session_id_for_listener = session_id.clone(); // 未使用 - TODO: 未来用于 WebSocket 会话管理
        let ws_manager_for_listener = websocket_manager.clone();
        let app_handle_for_listener = app_handle.clone();
        let last_log_timestamps_for_listener = last_log_timestamps.clone();
        let project_id_for_listener = project_id.to_string();
        tokio::spawn(async move {
            if let Err(e) = interaction.start_listening_with_timeout(1800).await {
                log::error!(
                    "[AgentWorker:{}] Listener task failed: {}",
                    agent_id_for_listener,
                    e
                );

                // 📤 发送实时日志：监听失败
                Self::send_ws_log_to_both_for_coding(
                    &ws_manager_for_listener,
                    &app_handle_for_listener,
                    &last_log_timestamps_for_listener,
                    &agent_id_for_listener,
                    &project_id_for_listener,
                    "error",
                    &format!("❌ AI 输出监听失败: {}", e),
                    Some("AgentWorker"),
                )
                .await;
            } else {
                // 📤 发送实时日志：监听完成
                Self::send_ws_log_to_both_for_coding(
                    &ws_manager_for_listener,
                    &app_handle_for_listener,
                    &last_log_timestamps_for_listener,
                    &agent_id_for_listener,
                    &project_id_for_listener,
                    "info",
                    "✅ AI 输出监听完成",
                    Some("AgentWorker"),
                )
                .await;
            }
        });

        // 处理 AI 输出的消息
        let story_id_for_commit = story.id.clone();
        let worktree_path_for_git = worktree_path.clone();
        let agent_id_for_output = agent_id_owned.clone();
        // let session_id_for_messages = session_id.clone(); // 未使用 - TODO: 未来用于消息会话关联
        let ws_manager_for_messages = websocket_manager.clone();
        let app_handle_for_messages = app_handle.clone();
        let last_log_timestamps_for_messages = last_log_timestamps.clone();
        let project_id_for_messages = project_id.to_string();

        tokio::spawn(async move {
            while let Some(message) = message_rx.recv().await {
                match message {
                    AICLIMessage::Stdout(line) => {
                        log::debug!("[AgentWorker:{}] AI Output: {}", agent_id_for_output, line);

                        // 📤 发送实时日志：AI 思考过程（过滤关键词）
                        if line.contains("思考") || line.contains("分析") || line.contains("计划")
                        {
                            Self::send_ws_log_to_both_for_coding(
                                &ws_manager_for_messages,
                                &app_handle_for_messages,
                                &last_log_timestamps_for_messages,
                                &agent_id_for_output,
                                &project_id_for_messages,
                                "log",
                                &format!("💭 {}", line),
                                Some("AgentWorker"),
                            )
                            .await;
                        }
                    }

                    AICLIMessage::Stderr(line) => {
                        log::warn!("[AgentWorker:{}] AI Error: {}", agent_id_for_output, line);

                        // 📤 发送实时日志：AI 错误
                        Self::send_ws_log_to_both_for_coding(
                            &ws_manager_for_messages,
                            &app_handle_for_messages,
                            &last_log_timestamps_for_messages,
                            &agent_id_for_output,
                            &project_id_for_messages,
                            "error",
                            &format!("⚠️ {}", line),
                            Some("AgentWorker"),
                        )
                        .await;
                    }

                    AICLIMessage::GeneratedCode { file_path, content } => {
                        if let Err(e) =
                            Self::write_generated_code(&worktree_path_for_git, &file_path, &content)
                                .await
                        {
                            log::error!(
                                "[AgentWorker:{}] Failed to write generated code: {}",
                                agent_id_for_output,
                                e
                            );

                            // 📤 发送实时日志：代码写入失败
                            Self::send_ws_log_to_both_for_coding(
                                &ws_manager_for_messages,
                                &app_handle_for_messages,
                                &last_log_timestamps_for_messages,
                                &agent_id_for_output,
                                &project_id_for_messages,
                                "error",
                                &format!("❌ 代码写入失败: {} - {}", file_path, e),
                                Some("AgentWorker"),
                            )
                            .await;
                        } else {
                            log::info!(
                                "[AgentWorker:{}] ✓ Wrote generated code to: {}",
                                agent_id_for_output,
                                file_path
                            );

                            // 📤 发送实时日志：代码生成成功
                            Self::send_ws_log_to_both_for_coding(
                                &ws_manager_for_messages,
                                &app_handle_for_messages,
                                &last_log_timestamps_for_messages,
                                &agent_id_for_output,
                                &project_id_for_messages,
                                "success",
                                &format!("✅ 生成代码并写入文件: {}", file_path),
                                Some("AgentWorker"),
                            )
                            .await;
                        }
                    }

                    AICLIMessage::TaskCompleted { success, summary } => {
                        log::info!(
                            "[AgentWorker:{}] Task completed: {} - {}",
                            agent_id_for_output,
                            if success { "SUCCESS" } else { "FAILED" },
                            summary
                        );

                        // 📤 发送实时日志：任务完成状态
                        Self::send_ws_log_to_both_for_coding(
                            &ws_manager_for_messages,
                            &app_handle_for_messages,
                            &last_log_timestamps_for_messages,
                            &agent_id_for_output,
                            &project_id_for_messages,
                            if success { "success" } else { "error" },
                            &format!(
                                "{} 任务完成: {}",
                                if success { "✅" } else { "❌" },
                                summary
                            ),
                            Some("AgentWorker"),
                        )
                        .await;

                        let story_id_for_update = story_id_for_commit.clone();
                        let worktree_path_for_git_clone = worktree_path_for_git.clone();
                        let agent_id_for_spawn = agent_id_owned.clone();
                        // let session_id_for_git = session_id_for_messages.clone(); // 未使用 - TODO: 未来用于 Git 操作会话跟踪
                        let ws_manager_for_git = ws_manager_for_messages.clone();
                        let app_handle_for_git = app_handle_for_messages.clone();
                        let last_log_timestamps_for_git = last_log_timestamps_for_messages.clone();
                        let project_id_for_git = project_id_for_messages.clone();

                        // 在后台处理 Git 操作和状态更新
                        tokio::spawn(async move {
                            if success {
                                // 📤 发送实时日志：开始 Git 提交
                                Self::send_ws_log_to_both_for_coding(
                                    &ws_manager_for_git,
                                    &app_handle_for_git,
                                    &last_log_timestamps_for_git,
                                    &agent_id_for_spawn,
                                    &project_id_for_git,
                                    "progress",
                                    "📦 开始提交代码到 Git...",
                                    Some("AgentWorker"),
                                )
                                .await;

                                // Git commit & push
                                match Self::commit_and_push_changes(
                                    &worktree_path_for_git_clone,
                                    &story_id_for_update,
                                )
                                .await
                                {
                                    Ok(commit_msg) => {
                                        log::info!(
                                            "[AgentWorker:{}] Successfully committed and pushed changes: {}",
                                            agent_id_for_spawn,
                                            commit_msg
                                        );

                                        // 📤 发送实时日志：Git 提交成功
                                        Self::send_ws_log_to_both_for_coding(
                                            &ws_manager_for_git,
                                            &app_handle_for_git,
                                            &last_log_timestamps_for_git,
                                            &agent_id_for_spawn,
                                            &project_id_for_git,
                                            "success",
                                            &format!("✅ Git 提交成功: {}", commit_msg),
                                            Some("AgentWorker"),
                                        )
                                        .await;

                                        // Git 成功后更新 Story 为 completed
                                        if let Err(e) = Self::update_story_status_to_completed(
                                            &story_id_for_update,
                                        )
                                        .await
                                        {
                                            log::error!(
                                                "[AgentWorker:{}] Failed to update story status to completed: {}",
                                                agent_id_for_spawn,
                                                e
                                            );

                                            // 📤 发送实时日志：更新故事状态失败
                                            Self::send_ws_log_to_both_for_coding(
                                                &ws_manager_for_git,
                                                &app_handle_for_git,
                                                &last_log_timestamps_for_git,
                                                &agent_id_for_spawn,
                                                &project_id_for_git,
                                                "error",
                                                &format!("❌ 更新故事状态失败: {}", e),
                                                Some("AgentWorker"),
                                            )
                                            .await;
                                        } else {
                                            log::info!(
                                                "[AgentWorker:{}] Successfully updated story status to completed",
                                                agent_id_for_spawn
                                            );

                                            // 📤 发送实时日志：用户故事已完成
                                            Self::send_ws_log_to_both_for_coding(
                                                &ws_manager_for_git,
                                                &app_handle_for_git,
                                                &last_log_timestamps_for_git,
                                                &agent_id_for_spawn,
                                                &project_id_for_git,
                                                "success",
                                                "🎉 用户故事已完成！",
                                                Some("AgentWorker"),
                                            )
                                            .await;
                                        }
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "[AgentWorker:{}] Failed to commit and push changes: {}",
                                            agent_id_for_spawn,
                                            e
                                        );

                                        // 📤 发送实时日志：Git 操作失败
                                        Self::send_ws_log_to_both_for_coding(
                                            &ws_manager_for_git,
                                            &app_handle_for_git,
                                            &last_log_timestamps_for_git,
                                            &agent_id_for_spawn,
                                            &project_id_for_git,
                                            "error",
                                            &format!("❌ Git 操作失败: {}", e),
                                            Some("AgentWorker"),
                                        )
                                        .await;
                                    }
                                }
                            } else {
                                // 任务失败直接标记为 failed
                                if let Err(e) = Self::update_story_status_to_failed(
                                    &story_id_for_update,
                                    &summary,
                                )
                                .await
                                {
                                    log::error!(
                                        "[AgentWorker:{}] Failed to update story status to failed: {}",
                                        agent_id_for_spawn,
                                        e
                                    );
                                } else {
                                    log::info!(
                                        "[AgentWorker:{}] Updated stories status to failed",
                                        agent_id_for_spawn
                                    );
                                }
                            }
                        });
                    }
                }
            }
        });

        Ok(())
    }

    /// 从数据库获取 Story 上下文
    fn get_story_context(story_id: &str) -> Result<StoryContext, String> {
        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        match db::get_user_story_by_id(&conn, story_id)
            .map_err(|e| format!("Failed to query user story: {}", e))?
        {
            Some(user_story) => {
                let title = if user_story.title.is_empty() {
                    None
                } else {
                    Some(user_story.title.clone())
                };

                let acceptance_criteria = if user_story.acceptance_criteria.is_empty() {
                    None
                } else {
                    Some(user_story.acceptance_criteria.clone())
                };

                log::info!(
                    "[AgentWorker] Retrieved story context for {}: title={:?}, criteria_len={}",
                    story_id,
                    title,
                    acceptance_criteria.as_ref().map(|s| s.len()).unwrap_or(0)
                );

                Ok(StoryContext {
                    title,
                    acceptance_criteria,
                })
            }
            None => {
                log::warn!(
                    "[AgentWorker] Story {} not found in database, using empty context",
                    story_id
                );
                Ok(StoryContext {
                    title: None,
                    acceptance_criteria: None,
                })
            }
        }
    }

    /// 写入生成的代码到文件
    async fn write_generated_code(
        worktree_path: &str,
        file_path: &str,
        content: &str,
    ) -> Result<(), String> {
        use tokio::fs;

        let full_path = Path::new(worktree_path).join(file_path);

        // 确保父目录存在
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // 写入文件
        fs::write(&full_path, content)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    /// Git Commit & Push
    async fn commit_and_push_changes(
        worktree_path: &str,
        story_id: &str,
    ) -> Result<String, String> {
        use tokio::process::Command as TokioCommand;

        log::info!(
            "[GitOps] Starting commit and push for worktree: {}",
            worktree_path
        );

        // 1. 检查是否有变更
        let status_output = TokioCommand::new("git")
            .args(["status", "--porcelain"])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("git status failed: {}", e))?;

        let status_stdout = String::from_utf8_lossy(&status_output.stdout);

        if status_stdout.trim().is_empty() {
            log::info!("[GitOps] No changes to commit");
            return Ok("No changes to commit".to_string());
        }

        log::info!("[GitOps] Detected changes:\n{}", status_stdout);

        // 2. 添加所有变更
        let add_output = TokioCommand::new("git")
            .args(["add", "."])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("git add failed: {}", e))?;

        if !add_output.status.success() {
            let stderr = String::from_utf8_lossy(&add_output.stderr);
            return Err(format!("git add failed: {}", stderr));
        }

        log::info!("[GitOps] Successfully staged all changes");

        // 3. 提交变更
        let commit_message = format!("Auto-generated code for story {}", story_id);
        let commit_output = TokioCommand::new("git")
            .args(["commit", "-m", &commit_message])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("git commit failed: {}", e))?;

        if !commit_output.status.success() {
            let stderr = String::from_utf8_lossy(&commit_output.stderr);
            return Err(format!("git commit failed: {}", stderr));
        }

        let commit_stdout = String::from_utf8_lossy(&commit_output.stdout);
        log::info!("[GitOps] Commit successful: {}", commit_stdout);

        // 4. 推送到远程分支
        let branch_name = format!("story-{}", story_id);

        // 先尝试推送
        let push_output = TokioCommand::new("git")
            .args(["push", "-u", "origin", &branch_name])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("git push failed: {}", e))?;

        if push_output.status.success() {
            log::info!("[GitOps] Successfully pushed to branch: {}", branch_name);
            return Ok(commit_message);
        }

        // 推送失败，尝试创建分支后重试
        log::warn!(
            "[GitOps] Push failed (possibly branch doesn't exist): {}",
            String::from_utf8_lossy(&push_output.stderr)
        );

        let create_branch_output = TokioCommand::new("git")
            .args(["checkout", "-b", &branch_name])
            .current_dir(worktree_path)
            .output()
            .await;

        if let Ok(output) = create_branch_output {
            if output.status.success() {
                log::info!("[GitOps] Branch created: {}", branch_name);
            } else {
                log::warn!(
                    "[GitOps] Branch creation failed (may already exist): {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        // 重试推送
        let retry_push_output = TokioCommand::new("git")
            .args(["push", "-u", "origin", &branch_name])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("git push retry failed: {}", e))?;

        if retry_push_output.status.success() {
            log::info!(
                "[GitOps] Successfully pushed to branch after retry: {}",
                branch_name
            );
            Ok(commit_message)
        } else {
            let stderr = String::from_utf8_lossy(&retry_push_output.stderr);
            Err(format!("git push failed after retry: {}", stderr))
        }
    }

    /// 更新 Story 状态为 completed
    async fn update_story_status_to_completed(story_id: &str) -> Result<(), String> {
        log::info!(
            "[StoryStatus] Updating story {} status to completed",
            story_id
        );

        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        match db::complete_user_story(&conn, story_id) {
            Ok(updated_count) => {
                if updated_count > 0 {
                    log::info!(
                        "[StoryStatus] Successfully updated {} story(s) to completed",
                        updated_count
                    );
                    Ok(())
                } else {
                    log::warn!("[StoryStatus] No story found with id: {}", story_id);
                    Err(format!("Story {} not found", story_id))
                }
            }
            Err(e) => {
                log::error!("[StoryStatus] Failed to update story status: {}", e);
                Err(format!("Database error: {}", e))
            }
        }
    }

    /// 更新 Story 状态为 failed（集成重试引擎）
    async fn update_story_status_to_failed(story_id: &str, reason: &str) -> Result<(), String> {
        log::info!(
            "[StoryStatus] Updating story {} status to failed: {}",
            story_id,
            reason
        );

        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // 1. 获取当前 Story 信息
        let story = match db::get_user_story_by_id(&conn, story_id) {
            Ok(Some(s)) => s,
            Ok(None) => {
                log::warn!("[StoryStatus] No story found with id: {}", story_id);
                return Err(format!("Story {} not found", story_id));
            }
            Err(e) => {
                log::error!("[StoryStatus] Failed to query story: {}", e);
                return Err(format!("Database error: {}", e));
            }
        };

        // 2. 创建重试引擎实例
        let backoff_config = BackoffConfig::default();
        let max_retries = story.max_retries as u32;
        let retry_engine = RetryEngine::new(max_retries, backoff_config);

        // 3. 使用重试引擎做决策
        let current_retry_count = story.retry_count as u32;
        let decision = retry_engine.should_retry(current_retry_count, reason);

        match decision {
            RetryDecision::Retry { next_retry_at } => {
                log::info!(
                    "[StoryStatus] Decision: RETRY story {}. Next retry at: {}",
                    story_id,
                    next_retry_at
                );

                // 3.1 创建重试历史记录
                let now = Utc::now().to_rfc3339();
                let retry_history = UserStoryRetryHistory {
                    id: format!("retry_{}_{}", story_id, now.replace(':', "-")),
                    user_story_id: story_id.to_string(),
                    retry_number: (current_retry_count + 1) as i32,
                    triggered_at: now.clone(),
                    error_message: Some(reason.to_string()),
                    error_type: Some("temporary".to_string()), // 简化：临时错误才重试
                    decision: "retry".to_string(),
                    next_retry_at: Some(next_retry_at.clone()),
                    completed_at: None,
                    result: Some("pending".to_string()),
                    created_at: now.clone(),
                };

                if let Err(e) = db::create_retry_history_record(&conn, &retry_history) {
                    log::error!("[StoryStatus] Failed to create retry history: {}", e);
                }

                // 3.2 更新 Story 状态为 scheduled_retry
                let updated = conn
                    .execute(
                        "UPDATE user_stories 
                     SET status = 'scheduled_retry',
                         next_retry_at = ?1,
                         retry_count = retry_count + 1,
                         updated_at = ?2
                     WHERE id = ?3",
                        rusqlite::params![next_retry_at, now, story_id],
                    )
                    .map_err(|e| format!("Database error: {}", e))?;

                if updated > 0 {
                    log::info!(
                        "[StoryStatus] Story {} scheduled for retry #{} at {}",
                        story_id,
                        current_retry_count + 1,
                        next_retry_at
                    );
                }

                Ok(())
            }
            RetryDecision::Abort {
                reason: abort_reason,
            } => {
                log::info!(
                    "[StoryStatus] Decision: ABORT retry for story {}: {}",
                    story_id,
                    abort_reason
                );

                // 4.1 创建重试历史记录（标记为终止）
                let now = Utc::now().to_rfc3339();
                let retry_history = UserStoryRetryHistory {
                    id: format!("retry_{}_{}", story_id, now.replace(':', "-")),
                    user_story_id: story_id.to_string(),
                    retry_number: (current_retry_count + 1) as i32,
                    triggered_at: now.clone(),
                    error_message: Some(reason.to_string()),
                    error_type: Some("permanent".to_string()),
                    decision: "abort".to_string(),
                    next_retry_at: None,
                    completed_at: Some(now.clone()),
                    result: Some("failed".to_string()),
                    created_at: now.clone(),
                };

                if let Err(e) = db::create_retry_history_record(&conn, &retry_history) {
                    log::error!("[StoryStatus] Failed to create retry history: {}", e);
                }

                // 4.2 更新 Story 状态为 permanently_failed
                match db::fail_user_story(&conn, story_id, &abort_reason) {
                    Ok(updated_count) => {
                        if updated_count > 0 {
                            log::info!(
                                "[StoryStatus] Story {} marked as permanently failed",
                                story_id
                            );
                            Ok(())
                        } else {
                            log::warn!("[StoryStatus] No story found with id: {}", story_id);
                            Err(format!("Story {} not found", story_id))
                        }
                    }
                    Err(e) => {
                        log::error!("[StoryStatus] Failed to update story status: {}", e);
                        Err(format!("Database error: {}", e))
                    }
                }
            }
        }
    }

    /// 停止 Worker
    pub async fn stop(&mut self) -> Result<(), String> {
        if !self.is_running {
            return Err(format!(
                "Agent Worker {} is not running",
                self.config.worker_id
            ));
        }

        self.is_running = false;
        log::info!(
            "[AgentWorker:{}] 🛑 Stopping independent agent loop",
            self.config.worker_id
        );

        // RetryScheduler 会在收到 ctrl_c 信号时自动优雅停止
        // 它会等待所有活跃的重试任务完成后再退出
        log::info!(
            "[AgentWorker:{}] ℹ️  RetryScheduler will gracefully shutdown on signal",
            self.config.worker_id
        );

        Ok(())
    }

    /// 检查 Worker 是否正在运行
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// 获取当前处理的 Story ID
    pub fn current_story_id(&self) -> Option<&str> {
        self.current_story_id.as_deref()
    }

    /// 发送日志到 WebSocket（带节流机制）
    async fn send_ws_log(
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
        session_id: &str,
        level: &str,
        message: &str,
        source: Option<&str>,
    ) {
        let source_tag = source.unwrap_or("AgentWorker");

        // 1. 始终输出到控制台日志（符合用户详细日志偏好）
        match level {
            "error" => {
                log::error!("[{}] [{}] ❌ {}", source_tag, session_id, message);
            }
            "warning" => {
                log::warn!("[{}] [{}] ⚠️ {}", source_tag, session_id, message);
            }
            "success" => {
                log::info!("[{}] [{}] ✅ {}", source_tag, session_id, message);
            }
            "progress" => {
                log::info!("[{}] [{}] 📊 {}", source_tag, session_id, message);
            }
            "info" => {
                log::info!("[{}] [{}] ℹ️ {}", source_tag, session_id, message);
            }
            _ => {
                log::debug!("[{}] [{}] {}", source_tag, session_id, message);
            }
        }

        // 2. 节流检查：仅对 debug 级别日志进行节流，关键业务日志不节流
        // 策略：
        // - debug: 严格节流（200ms），防止高频调试信息刷屏
        // - info/success/warning/error: 不节流，确保所有关键决策日志100%显示
        let should_throttle = if level == "debug" {
            let throttle_interval = 200; // debug 日志节流间隔
            let now = chrono::Utc::now().timestamp_millis() as u64;
            let mut timestamps = last_log_timestamps.write().await;

            if let Some(&last_ts) = timestamps.get(session_id) {
                if now - last_ts < throttle_interval {
                    log::debug!(
                        "[{}] [{}] ⏱️ Debug日志节流：距离上次发送不足 {}ms",
                        source_tag,
                        session_id,
                        throttle_interval
                    );
                    true // 需要节流
                } else {
                    timestamps.insert(session_id.to_string(), now);
                    false // 允许发送
                }
            } else {
                timestamps.insert(session_id.to_string(), now);
                false // 首次发送
            }
        } else {
            // 关键业务日志不节流，直接允许发送
            false
        };

        if should_throttle {
            return;
        }

        // 3. 优先使用 AppHandle 直接发送（更高效）
        if let Some(handle) = app_handle {
            use tauri::Emitter;
            let event_name = format!("ws:{}", session_id);
            let ws_message = crate::agent::websocket_manager::WsMessage::log(
                session_id.to_string(),
                level,
                message,
                source,
            );

            match handle.emit(&event_name, &ws_message) {
                Ok(_) => {
                    log::debug!("[{}] [{}] ✅ Sent via AppHandle", source_tag, session_id);
                }
                Err(e) => {
                    log::warn!(
                        "[{}] [{}] ❌ AppHandle emit failed: {}",
                        source_tag,
                        session_id,
                        e
                    );
                    // 回退到 WebSocket Manager
                }
            }
            return;
        }

        // 4. 回退方案：使用 WebSocket Manager
        if let Some(ws_manager) = websocket_manager {
            if let Ok(manager) = ws_manager.try_read() {
                let session_id_string = session_id.to_string();
                match manager
                    .send_log(&session_id_string, level, message, source)
                    .await
                {
                    Ok(_) => {
                        log::debug!(
                            "[{}] [{}] ✅ Sent via WebSocket Manager",
                            source_tag,
                            session_id
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "[{}] [{}] ❌ WebSocket send failed: {}",
                            source_tag,
                            session_id,
                            e
                        );
                    }
                }
            } else {
                log::warn!(
                    "[{}] [{}] ⚠️ Cannot acquire WebSocket Manager lock",
                    source_tag,
                    session_id
                );
            }
        } else {
            log::debug!(
                "[{}] [{}] ℹ️ WebSocket Manager unavailable",
                source_tag,
                session_id
            );
        }
    }

    /// 发送日志到 WebSocket（仅发送到智能体粒度）
    async fn send_ws_log_to_both(
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
        worker_id: &str,
        _project_id: &str, // 保留参数以保持接口兼容，但不再使用
        level: &str,
        message: &str,
        source: Option<&str>,
    ) {
        // 仅发送到 agent-{worker_id}，不再发送到 project-{project_id}
        let agent_session_id = format!("agent-{}", worker_id);
        Self::send_ws_log(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            &agent_session_id,
            level,
            message,
            source,
        )
        .await;
    }

    /// 发送日志到 WebSocket（用于 start_coding_agent，仅发送到智能体粒度）
    async fn send_ws_log_to_both_for_coding(
        websocket_manager: &Option<Arc<RwLock<WebSocketManager>>>,
        app_handle: &Option<AppHandle>,
        last_log_timestamps: &Arc<RwLock<std::collections::HashMap<String, u64>>>,
        agent_id: &str,
        _project_id: &str, // 保留参数以保持接口兼容，但不再使用
        level: &str,
        message: &str,
        source: Option<&str>,
    ) {
        // 仅发送到 agent-{agent_id}，不再发送到 project-{project_id}
        let agent_session_id = format!("agent-{}", agent_id);
        Self::send_ws_log(
            websocket_manager,
            app_handle,
            last_log_timestamps,
            &agent_session_id,
            level,
            message,
            source,
        )
        .await;
    }
}

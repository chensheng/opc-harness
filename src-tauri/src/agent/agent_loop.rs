//! Agent Loop - 自动化执行引擎
//! 
//! 实现从 Sprint 中自动选择用户故事并分配给 Coding Agent 执行的完整流程

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use crate::db::{self, database};
use crate::agent::daemon::DaemonManager;
use crate::agent::worktree_manager::WorktreeManager;
use crate::agent::ai_cli_interaction::AICLIMessage;
use log;
use std::path::PathBuf;

/// Agent Loop 管理器
pub struct AgentLoop {
    /// Daemon 管理器（用于启动 Agent 进程）
    daemon_manager: Arc<RwLock<DaemonManager>>,
    /// Worktree 管理器（用于创建独立工作树）
    pub worktree_manager: Option<Arc<WorktreeManager>>,
    /// 是否正在运行
    pub is_running: bool,
}

impl AgentLoop {
    /// 创建新的 Agent Loop 实例
    pub fn new(daemon_manager: Arc<RwLock<DaemonManager>>) -> Self {
        Self {
            daemon_manager,
            worktree_manager: None,
            is_running: false,
        }
    }

    /// 设置 Worktree 管理器
    pub fn set_worktree_manager(&mut self, project_path: &str) {
        self.worktree_manager = Some(Arc::new(WorktreeManager::new(project_path)));
        log::info!("[AgentLoop] Worktree manager initialized for project: {}", project_path);
    }

    /// 启动 Agent Loop（单次执行）
    pub async fn execute_once(&mut self, project_id: &str) -> Result<usize, String> {
        let cycle_start = std::time::Instant::now();
        
        log::info!("");
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║           Agent Loop - Execution Cycle Start             ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!("[AgentLoop] 🔄 Starting execution cycle for project: {}", project_id);
        log::info!("[AgentLoop] ⏱️  Timestamp: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        
        // 获取数据库连接
        let db_start = std::time::Instant::now();
        let conn = database::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        log::debug!("[AgentLoop] ✓ Database connection acquired in {:?}", db_start.elapsed());
        
        // Step 1: 获取当前活跃的 Sprint
        log::info!("[AgentLoop] 🔍 Step 1: Checking for active Sprint...");
        let sprint_check_start = std::time::Instant::now();
        
        let active_sprint = db::get_active_sprint(&conn)
            .map_err(|e| format!("Failed to get active sprint: {}", e))?;
        
        if active_sprint.is_none() {
            log::warn!("[AgentLoop] ⚠️ No active Sprint found for current time");
            log::info!("[AgentLoop] 💡 Hint: Create a Sprint with status='active' and valid date range");
            log::info!("[AgentLoop] ⏱️  Cycle completed in {:?} (no active sprint)", cycle_start.elapsed());
            return Ok(0);
        }
        
        let sprint = active_sprint.unwrap();
        log::info!("[AgentLoop] ✓ Found active Sprint: '{}' (ID: {})", sprint.name, sprint.id);
        log::info!("[AgentLoop]   📅 Date range: {} to {}", 
                   sprint.start_date.format("%Y-%m-%d"), 
                   sprint.end_date.format("%Y-%m-%d"));
        log::debug!("[AgentLoop] ⏱️  Sprint check completed in {:?}", sprint_check_start.elapsed());
        
        // Step 2: 获取该 Sprint 下待执行的用户故事
        log::info!("[AgentLoop] 🔍 Step 2: Loading pending user stories from Sprint...");
        let stories_load_start = std::time::Instant::now();
        
        let pending_stories = db::get_pending_stories_by_sprint(&conn, &sprint.id)
            .map_err(|e| format!("Failed to get pending stories: {}", e))?;
        
        if pending_stories.is_empty() {
            log::info!("[AgentLoop] ℹ️ No pending stories found in Sprint {}", sprint.id);
            log::info!("[AgentLoop] 💡 Hint: Add user stories with status='draft/refined/approved' to this Sprint");
            log::info!("[AgentLoop] ⏱️  Cycle completed in {:?} (no pending stories)", cycle_start.elapsed());
            return Ok(0);
        }
        
        log::info!("[AgentLoop] ✓ Found {} pending story/ies to process", pending_stories.len());
        for (i, story) in pending_stories.iter().enumerate() {
            log::info!("[AgentLoop]   Story #{}: {} - {} (Priority: {}, Status: {})", 
                       i + 1, story.story_number, story.title, story.priority, story.status);
        }
        log::debug!("[AgentLoop] ⏱️  Stories loaded in {:?}", stories_load_start.elapsed());
        
        // Step 3: 为每个故事尝试锁定并启动 Agent
        log::info!("[AgentLoop] 🚀 Step 3: Attempting to lock and start Agents for each story...");
        let agent_start_start = std::time::Instant::now();
        
        let mut started_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;
        let agent_id_prefix = format!("coding-{}", Utc::now().timestamp());
        
        for (index, story) in pending_stories.iter().enumerate() {
            let story_start = std::time::Instant::now();
            
            // 生成唯一的 Agent ID
            let agent_id = format!("{}-{}", agent_id_prefix, index);
            
            log::info!("[AgentLoop]   ┌─ Processing story {}: {}...", story.story_number, story.title);
            log::debug!("[AgentLoop]   │  Agent ID: {}", agent_id);
            
            // 尝试锁定故事（乐观锁）
            log::debug!("[AgentLoop]   │  Attempting to acquire lock...");
            let locked = db::lock_user_story(&conn, &story.id, &agent_id)
                .map_err(|e| format!("Failed to lock story {}: {}", story.id, e))?;
            
            if !locked {
                log::warn!("[AgentLoop]   │  ⚠️ Story {} already locked or in progress, skipping", story.story_number);
                skipped_count += 1;
                log::debug!("[AgentLoop]   └─ Skipped in {:?}", story_start.elapsed());
                continue;
            }
            
            log::info!("[AgentLoop]   │  🔒 Successfully locked story {}: {} (Priority: {})", 
                       story.story_number, story.title, story.priority);
            
            // Step 4: 启动 Coding Agent
            log::info!("[AgentLoop]   │  🚀 Step 4: Starting Coding Agent for story {}...", story.story_number);
            match self.start_coding_agent(&agent_id, &story, project_id).await {
                Ok(_) => {
                    started_count += 1;
                    log::info!("[AgentLoop]   │  ✓ Successfully started Coding Agent #{} for story {}", 
                               started_count, story.story_number);
                    log::info!("[AgentLoop]   │  📝 Agent will work in isolated Worktree environment");
                    log::debug!("[AgentLoop]   └─ Agent started in {:?}", story_start.elapsed());
                }
                Err(e) => {
                    failed_count += 1;
                    log::error!("[AgentLoop]   │  ✗ Failed to start Agent for story {}: {}", 
                               story.story_number, e);
                    
                    // 标记故事为失败状态
                    log::debug!("[AgentLoop]   │  Marking story as failed in database...");
                    if let Err(db_err) = db::fail_user_story(&conn, &story.id, &e) {
                        log::error!("[AgentLoop]   │  Failed to update story status: {}", db_err);
                    } else {
                        log::debug!("[AgentLoop]   │  ✓ Story marked as failed");
                    }
                    log::debug!("[AgentLoop]   └─ Failed in {:?}", story_start.elapsed());
                }
            }
            
            // 短暂延迟，避免同时启动过多 Agent
            if index < pending_stories.len() - 1 {
                log::debug!("[AgentLoop]   │  Waiting 500ms before next story...");
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
        
        let total_elapsed = cycle_start.elapsed();
        let agent_phase_elapsed = agent_start_start.elapsed();
        
        log::info!("");
        log::info!("╔═══════════════════════════════════════════════════════════╗");
        log::info!("║           Agent Loop - Execution Cycle Complete          ║");
        log::info!("╚═══════════════════════════════════════════════════════════╝");
        log::info!("[AgentLoop] ✅ Execution cycle completed!");
        log::info!("[AgentLoop] 📊 Summary:");
        log::info!("[AgentLoop]   • Total pending stories: {}", pending_stories.len());
        log::info!("[AgentLoop]   • Successfully started: {} ✓", started_count);
        log::info!("[AgentLoop]   • Failed to start: {} ✗", failed_count);
        log::info!("[AgentLoop]   • Skipped (already locked): {} ⊘", skipped_count);
        log::info!("[AgentLoop]   • Remaining pending: {}", pending_stories.len() - started_count);
        log::info!("[AgentLoop] ⏱️  Performance:");
        log::info!("[AgentLoop]   • Total cycle time: {:?}", total_elapsed);
        log::info!("[AgentLoop]   • Agent startup phase: {:?}", agent_phase_elapsed);
        log::info!("[AgentLoop]   • Average per story: {:?}", agent_phase_elapsed / pending_stories.len() as u32);
        
        Ok(started_count)
    }

    /// 启动 Coding Agent 进程 (在 Worktree 中执行)
    async fn start_coding_agent(&self, agent_id: &str, story: &crate::models::UserStory, project_id: &str) -> Result<(), String> {
        let agent_start = std::time::Instant::now();
        
        log::info!("[AgentLoop] Starting coding agent for story: {}", story.title);
        log::debug!("[AgentLoop]   Agent ID: {}", agent_id);
        log::debug!("[AgentLoop]   Story ID: {}", story.id);
        log::debug!("[AgentLoop]   Story Number: {}", story.story_number);
        log::debug!("[AgentLoop]   Project ID: {}", project_id);
        
        // 获取 Daemon Manager 的写锁
        log::debug!("[AgentLoop]   Acquiring Daemon Manager lock...");
        let daemon_lock_start = std::time::Instant::now();
        let mut daemon = self.daemon_manager.write().await;
        log::debug!("[AgentLoop]   ✓ Daemon Manager lock acquired in {:?}", daemon_lock_start.elapsed());
        
        // 构造项目路径
        let workspaces_root = crate::utils::paths::get_workspaces_dir();
        let project_path = workspaces_root.join(project_id);
        
        log::debug!("[AgentLoop]   Project path: {:?}", project_path);
        
        if !project_path.exists() {
            return Err(format!("Project path does not exist: {:?}", project_path));
        }
        
        let project_path_str = project_path.to_string_lossy().to_string();
        
        // 如果配置了 Worktree 管理器,先创建 Worktree 并在其中启动 Agent
        if let Some(ref wt_manager) = self.worktree_manager {
            log::info!("[AgentLoop] Creating worktree for agent {} and story {}", agent_id, story.id);
            
            let worktree_start = std::time::Instant::now();
            
            // 生成分支名称 (基于 Story ID)
            let branch_name = format!("story-{}", story.story_number);
            log::debug!("[AgentLoop]   Branch name: {}", branch_name);
            
            // 克隆 wt_manager 以便在异步闭包中使用
            let wt_manager_clone = wt_manager.clone();
            let agent_id_clone = agent_id.to_string();
            let story_id_clone = story.id.clone();
            
            match wt_manager.create_worktree(agent_id, &story.id, &branch_name).await {
                Ok(worktree_path) => {
                    log::info!("[AgentLoop] ✓ Worktree created at: {}", worktree_path);
                    log::debug!("[AgentLoop]   Worktree creation time: {:?}", worktree_start.elapsed());
                    
                    // 创建消息通道用于接收 AI CLI 输出
                    let (message_tx, mut message_rx) = tokio::sync::mpsc::channel::<AICLIMessage>(100);
                    log::debug!("[AgentLoop]   Message channel created (capacity: 100)");
                    
                    // 在后台启动消息处理任务
                    let agent_id_for_log = agent_id_clone.clone();
                    let worktree_path_for_write = worktree_path.clone();
                    let worktree_path_for_git = worktree_path.clone();
                    let story_id_for_commit = story_id_clone.clone();
                    
                    log::debug!("[AgentLoop]   Spawning message handler task...");
                    let message_handler_start = std::time::Instant::now();
                    
                    tokio::spawn(async move {
                        log::debug!("[AgentLoop:{}] Message handler started", agent_id_for_log);
                        let mut message_count = 0;
                        
                        while let Some(message) = message_rx.recv().await {
                            message_count += 1;
                            
                            match message {
                                AICLIMessage::Stdout(line) => {
                                    log::debug!("[AgentLoop:{}] AI Output [msg #{}]: {}", agent_id_for_log, message_count, line);
                                }
                                AICLIMessage::Stderr(line) => {
                                    log::warn!("[AgentLoop:{}] AI Error [msg #{}]: {}", agent_id_for_log, message_count, line);
                                }
                                AICLIMessage::GeneratedCode { file_path, content } => {
                                    log::info!("[AgentLoop:{}] 📝 Generated code for file: {}", agent_id_for_log, file_path);
                                    log::debug!("[AgentLoop:{}]   Content size: {} bytes", agent_id_for_log, content.len());
                                    
                                    // 将生成的代码写入 Worktree 中的文件
                                    let write_start = std::time::Instant::now();
                                    if let Err(e) = Self::write_generated_code(&worktree_path_for_write, &file_path, &content).await {
                                        log::error!("[AgentLoop:{}] Failed to write generated code to {}: {}", 
                                            agent_id_for_log, file_path, e);
                                    } else {
                                        log::info!("[AgentLoop:{}] ✓ Successfully wrote generated code to: {} (in {:?})", 
                                            agent_id_for_log, file_path, write_start.elapsed());
                                    }
                                }
                                AICLIMessage::TaskCompleted { success, summary } => {
                                    let task_duration = message_handler_start.elapsed();
                                    log::info!("[AgentLoop:{}] 🏁 Task completed: {} - {}", 
                                               agent_id_for_log, 
                                               if success { "SUCCESS" } else { "FAILED" }, 
                                               summary);
                                    log::info!("[AgentLoop:{}] ⏱️  Total execution time: {:?}", agent_id_for_log, task_duration);
                                    log::info!("[AgentLoop:{}] 📨 Total messages processed: {}", agent_id_for_log, message_count);
                                    
                                    // 克隆必要的值用于后台任务
                                    let story_id_for_update = story_id_for_commit.clone();
                                    let agent_id_for_status = agent_id_for_log.clone();
                                    let worktree_path_for_git_clone = worktree_path_for_git.clone();
                                    
                                    // 启动后台任务处理 Git 操作和 Story 状态更新
                                    tokio::spawn(async move {
                                        if success {
                                            // 1. 执行 Git commit & push
                                            log::info!("[AgentLoop:{}] 🔄 Starting Git operations...", agent_id_for_status);
                                            let git_start = std::time::Instant::now();
                                            
                                            match Self::commit_and_push_changes(&worktree_path_for_git_clone, &story_id_for_update).await {
                                                Ok(commit_msg) => {
                                                    log::info!("[AgentLoop:{}] ✓ Successfully committed and pushed changes: {}", 
                                                        agent_id_for_status, commit_msg);
                                                    log::info!("[AgentLoop:{}] ⏱️  Git operations completed in {:?}", 
                                                        agent_id_for_status, git_start.elapsed());
                                                    
                                                    // 2. Git 成功后更新 Story 状态为 completed
                                                    log::info!("[AgentLoop:{}] 📊 Updating story status to 'completed'...", agent_id_for_status);
                                                    let db_update_start = std::time::Instant::now();
                                                    
                                                    match Self::update_story_status_to_completed(&story_id_for_update).await {
                                                        Ok(_) => {
                                                            log::info!("[AgentLoop:{}] ✓ Successfully updated story status to completed (in {:?})", 
                                                                agent_id_for_status, db_update_start.elapsed());
                                                        }
                                                        Err(e) => {
                                                            log::error!("[AgentLoop:{}] ✗ Failed to update story status to completed: {}", 
                                                                agent_id_for_status, e);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    log::error!("[AgentLoop:{}] ✗ Failed to commit and push changes: {}", 
                                                        agent_id_for_status, e);
                                                    log::debug!("[AgentLoop:{}] ⏱️  Git operations failed after {:?}", 
                                                        agent_id_for_status, git_start.elapsed());
                                                    
                                                    // Git 失败也标记 Story 为 failed
                                                    log::info!("[AgentLoop:{}] Marking story as failed due to Git error...", agent_id_for_status);
                                                    match Self::update_story_status_to_failed(&story_id_for_update, &format!("Git operation failed: {}", e)).await {
                                                        Ok(_) => {
                                                            log::info!("[AgentLoop:{}] Updated story status to failed due to Git error", 
                                                                agent_id_for_status);
                                                        }
                                                        Err(e2) => {
                                                            log::error!("[AgentLoop:{}] Failed to update story status to failed: {}", 
                                                                agent_id_for_status, e2);
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            // 任务失败,更新 Story 状态为 failed
                                            log::info!("[AgentLoop:{}] Task failed, updating story status...", agent_id_for_status);
                                            match Self::update_story_status_to_failed(&story_id_for_update, &summary).await {
                                                Ok(_) => {
                                                    log::info!("[AgentLoop:{}] ✓ Updated story status to failed", 
                                                        agent_id_for_status);
                                                }
                                                Err(e) => {
                                                    log::error!("[AgentLoop:{}] ✗ Failed to update story status to failed: {}", 
                                                        agent_id_for_status, e);
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        
                        log::debug!("[AgentLoop:{}] Message handler exited", agent_id_for_log);
                    });
                    
                    log::debug!("[AgentLoop]   ✓ Message handler spawned in {:?}", message_handler_start.elapsed());

                    // 在 Worktree 中启动 Agent (带 STDIO 监控)
                    log::info!("[AgentLoop]   🚀 Spawning AI CLI process in worktree...");
                    let spawn_start = std::time::Instant::now();
                    
                    match daemon.spawn_agent_with_stdio_monitoring("coding", &worktree_path, &story_id_clone, message_tx).await {
                        Ok(spawned_agent_id) => {
                            let spawn_elapsed = spawn_start.elapsed();
                            log::info!("[AgentLoop]   ✓ Successfully spawned coding agent in worktree with STDIO monitoring: {}", spawned_agent_id);
                            log::info!("[AgentLoop]   ⏱️  Agent spawn time: {:?}", spawn_elapsed);
                            log::info!("[AgentLoop]   ⏱️  Total setup time: {:?}", agent_start.elapsed());
                            
                            // TODO: 将 Agent ID 与 Story ID 关联，便于后续追踪
                            // 可以考虑在数据库中添加 agent_id 字段到 user_stories 表
                            
                            return Ok(());
                        }
                        Err(e) => {
                            log::error!("[AgentLoop]   ✗ Failed to spawn agent in worktree: {}. Cleaning up worktree.", e);
                            log::debug!("[AgentLoop]   ⏱️  Failed after {:?}", agent_start.elapsed());
                            
                            // 清理失败的 Worktree
                            log::debug!("[AgentLoop]   Attempting to clean up failed worktree...");
                            if let Err(cleanup_err) = wt_manager_clone.remove_worktree(&agent_id_clone).await {
                                log::warn!("[AgentLoop]   Failed to cleanup worktree: {}", cleanup_err);
                            } else {
                                log::debug!("[AgentLoop]   ✓ Worktree cleaned up successfully");
                            }
                            
                            return Err(format!("Failed to spawn agent in worktree: {}", e));
                        }
                    }
                }
                Err(e) => {
                    log::error!("[AgentLoop] Failed to create worktree: {}. Falling back to project root.", e);
                    // 继续执行,使用项目根目录
                }
            }
        }
        
        // 回退方案: 在项目根目录启动 Agent (原有逻辑)
        match daemon.spawn_agent("coding", &project_path_str) {
            Ok(spawned_agent_id) => {
                log::info!("[AgentLoop] Successfully spawned coding agent in project root: {}", spawned_agent_id);
                
                // TODO: 将 Agent ID 与 Story ID 关联，便于后续追踪
                
                Ok(())
            }
            Err(e) => {
                Err(format!("Failed to spawn coding agent: {}", e))
            }
        }
    }

    /// 清理已完成 Agent 的 Worktree
    pub async fn cleanup_completed_worktrees(&self, completed_agent_ids: &[String]) -> Result<usize, String> {
        if let Some(ref wt_manager) = self.worktree_manager {
            let mut cleaned_count = 0;
            
            for agent_id in completed_agent_ids {
                log::info!("[AgentLoop] Cleaning up worktree for completed agent: {}", agent_id);
                
                match wt_manager.remove_worktree(agent_id).await {
                    Ok(_) => {
                        cleaned_count += 1;
                        log::info!("[AgentLoop] Successfully removed worktree for agent {}", agent_id);
                    }
                    Err(e) => {
                        log::warn!("[AgentLoop] Failed to remove worktree for agent {}: {}", agent_id, e);
                    }
                }
            }
            
            if cleaned_count > 0 {
                log::info!("[AgentLoop] Cleaned up {} worktrees for completed agents", cleaned_count);
            }
            
            Ok(cleaned_count)
        } else {
            Err("Worktree manager not initialized".to_string())
        }
    }

    /// 启动持续运行的 Agent Loop（定时执行）
    pub async fn start_continuous(&mut self, project_id: &str, interval_secs: u64) {
        self.is_running = true;
        log::info!("[AgentLoop] Starting continuous loop with {}s interval", interval_secs);
        
        while self.is_running {
            match self.execute_once(project_id).await {
                Ok(count) => {
                    log::info!("[AgentLoop] Cycle completed. Started {} agents", count);
                }
                Err(e) => {
                    log::error!("[AgentLoop] Cycle failed: {}", e);
                }
            }
            
            // 等待下一个周期
            tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs)).await;
        }
        
        log::info!("[AgentLoop] Continuous loop stopped");
    }

    /// 停止 Agent Loop
    pub fn stop(&mut self) {
        self.is_running = false;
        log::info!("[AgentLoop] Stop signal received");
    }

    /// 将生成的代码写入 Worktree 中的文件
    async fn write_generated_code(worktree_path: &str, file_path: &str, content: &str) -> Result<(), String> {
        use tokio::fs;
        
        // 构建完整的文件路径
        let full_path = PathBuf::from(worktree_path).join(file_path);
        
        // 确保父目录存在
        if let Some(parent_dir) = full_path.parent() {
            fs::create_dir_all(parent_dir).await
                .map_err(|e| format!("Failed to create directory {:?}: {}", parent_dir, e))?;
        }
        
        // 写入文件内容
        fs::write(&full_path, content.as_bytes()).await
            .map_err(|e| format!("Failed to write file {:?}: {}", full_path, e))?;
        
        log::debug!("[CodeWriter] Successfully wrote {} bytes to {:?}", content.len(), full_path);
        
        Ok(())
    }

    /// 执行 Git commit & push 操作
    async fn commit_and_push_changes(worktree_path: &str, story_id: &str) -> Result<String, String> {
        use tokio::process::Command as TokioCommand;
        
        log::info!("[GitOps] ═══════════════════════════════════════════════");
        log::info!("[GitOps] Starting commit and push for worktree: {}", worktree_path);
        log::info!("[GitOps] Story ID: {}", story_id);
        
        let git_start = std::time::Instant::now();
        
        // 1. 检查是否有变更
        log::debug!("[GitOps] Step 1/5: Checking for changes (git status)...");
        let step1_start = std::time::Instant::now();
        
        let status_output = TokioCommand::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git status: {}", e))?;
        
        let status_stdout = String::from_utf8_lossy(&status_output.stdout);
        
        if status_stdout.trim().is_empty() {
            log::warn!("[GitOps] ⚠️ No changes detected in worktree: {}", worktree_path);
            log::debug!("[GitOps] ⏱️  Git check completed in {:?}", step1_start.elapsed());
            return Ok("No changes to commit".to_string());
        }
        
        let changed_files_count = status_stdout.lines().count();
        log::info!("[GitOps] ✓ Detected {} changed file(s)", changed_files_count);
        log::debug!("[GitOps] Changed files:\n{}", status_stdout);
        log::debug!("[GitOps] ⏱️  Status check completed in {:?}", step1_start.elapsed());
        
        // 2. 添加所有变更文件
        log::debug!("[GitOps] Step 2/5: Staging all changes (git add)...");
        let step2_start = std::time::Instant::now();
        
        let add_output = TokioCommand::new("git")
            .args(&["add", "."])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git add: {}", e))?;
        
        if !add_output.status.success() {
            let stderr = String::from_utf8_lossy(&add_output.stderr);
            return Err(format!("git add failed: {}", stderr));
        }
        
        log::info!("[GitOps] ✓ Successfully staged all changes");
        log::debug!("[GitOps] ⏱️  Git add completed in {:?}", step2_start.elapsed());
        
        // 3. 生成 commit message (基于 Story ID)
        let commit_message = format!("Auto-generated code for story {}", story_id);
        log::debug!("[GitOps] Commit message: {}", commit_message);
        
        // 4. 提交变更
        log::debug!("[GitOps] Step 3/5: Creating commit (git commit)...");
        let step3_start = std::time::Instant::now();
        
        let commit_output = TokioCommand::new("git")
            .args(&["commit", "-m", &commit_message])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git commit: {}", e))?;

        if !commit_output.status.success() {
            let stderr = String::from_utf8_lossy(&commit_output.stderr);
            return Err(format!("git commit failed: {}", stderr));
        }
        
        let commit_stdout = String::from_utf8_lossy(&commit_output.stdout);
        log::info!("[GitOps] ✓ Commit successful");
        log::debug!("[GitOps] Commit output: {}", commit_stdout);
        log::debug!("[GitOps] ⏱️  Git commit completed in {:?}", step3_start.elapsed());
        
        // 5. 推送到远程分支 (story-{story_number})
        // 注意: 这里假设分支已经存在,如果不存在需要先创建
        let branch_name = format!("story-{}", story_id);
        
        log::debug!("[GitOps] Step 4/5: Pushing to remote branch '{}' (git push)...", branch_name);
        let step4_start = std::time::Instant::now();
        
        let push_output = TokioCommand::new("git")
            .args(&["push", "-u", "origin", &branch_name])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git push: {}", e))?;
        
        if !push_output.status.success() {
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            log::warn!("[GitOps] ⚠️ Initial push failed: {}", stderr);
            log::info!("[GitOps] Attempting to create branch and retry...");
            
            // 尝试创建分支并重试
            log::debug!("[GitOps] Step 4b: Creating local branch '{}' (git checkout -b)...", branch_name);
            let checkout_output = TokioCommand::new("git")
                .args(&["checkout", "-b", &branch_name])
                .current_dir(worktree_path)
                .output()
                .await
                .map_err(|e| format!("Failed to create branch: {}", e))?;
            
            if !checkout_output.status.success() {
                let stderr = String::from_utf8_lossy(&checkout_output.stderr);
                return Err(format!("git checkout -b failed: {}", stderr));
            }
            
            log::info!("[GitOps] ✓ Local branch created");
            
            // 重试 push
            log::debug!("[GitOps] Step 4c: Retrying push after branch creation...");
            let retry_push_output = TokioCommand::new("git")
                .args(&["push", "-u", "origin", &branch_name])
                .current_dir(worktree_path)
                .output()
                .await
                .map_err(|e| format!("Failed to execute git push (retry): {}", e))?;
            
            if !retry_push_output.status.success() {
                let stderr = String::from_utf8_lossy(&retry_push_output.stderr);
                return Err(format!("git push (retry) failed: {}", stderr));
            }
            
            log::info!("[GitOps] ✓ Retry push successful");
            log::debug!("[GitOps] ⏱️  Branch creation + retry push completed in {:?}", step4_start.elapsed());
        } else {
            log::info!("[GitOps] ✓ Push successful");
            log::debug!("[GitOps] ⏱️  Git push completed in {:?}", step4_start.elapsed());
        }
        
        let total_git_time = git_start.elapsed();
        log::info!("[GitOps] ═══════════════════════════════════════════════");
        log::info!("[GitOps] ✅ All Git operations completed successfully!");
        log::info!("[GitOps] 📊 Summary:");
        log::info!("[GitOps]   • Changed files: {}", changed_files_count);
        log::info!("[GitOps]   • Branch: {}", branch_name);
        log::info!("[GitOps]   • Commit message: {}", commit_message);
        log::info!("[GitOps] ⏱️  Total Git time: {:?}", total_git_time);
        
        Ok(commit_message)
    }

    /// 更新 Story 状态为 completed
    async fn update_story_status_to_completed(story_id: &str) -> Result<(), String> {
        use crate::db;
        
        log::info!("[StoryStatus] 📊 Updating story {} status to 'completed'", story_id);
        let db_start = std::time::Instant::now();
        
        // 获取数据库连接
        log::debug!("[StoryStatus]   Acquiring database connection...");
        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        log::debug!("[StoryStatus]   ✓ Database connection acquired");
        
        // 调用 complete_user_story 方法
        log::debug!("[StoryStatus]   Executing UPDATE query...");
        match db::complete_user_story(&conn, story_id) {
            Ok(updated_count) => {
                if updated_count > 0 {
                    log::info!("[StoryStatus] ✓ Successfully updated {} story(s) to completed (in {:?})", 
                               updated_count, db_start.elapsed());
                    Ok(())
                } else {
                    log::warn!("[StoryStatus] ⚠️ No story found with id: {}", story_id);
                    Err(format!("Story {} not found", story_id))
                }
            }
            Err(e) => {
                log::error!("[StoryStatus] ✗ Failed to update story status: {}", e);
                Err(format!("Database error: {}", e))
            }
        }
    }

    /// 更新 Story 状态为 failed
    async fn update_story_status_to_failed(story_id: &str, reason: &str) -> Result<(), String> {
        use crate::db;
        
        log::info!("[StoryStatus] 📊 Updating story {} status to 'failed'", story_id);
        log::debug!("[StoryStatus]   Failure reason: {}", reason);
        let db_start = std::time::Instant::now();
        
        // 获取数据库连接
        log::debug!("[StoryStatus]   Acquiring database connection...");
        let conn = db::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        log::debug!("[StoryStatus]   ✓ Database connection acquired");
        
        // 调用 fail_user_story 方法
        log::debug!("[StoryStatus]   Executing UPDATE query...");
        match db::fail_user_story(&conn, story_id, reason) {
            Ok(updated_count) => {
                if updated_count > 0 {
                    log::info!("[StoryStatus] ✓ Successfully updated {} story(s) to failed (in {:?})", 
                               updated_count, db_start.elapsed());
                    Ok(())
                } else {
                    log::warn!("[StoryStatus] ⚠️ No story found with id: {}", story_id);
                    Err(format!("Story {} not found", story_id))
                }
            }
            Err(e) => {
                log::error!("[StoryStatus] ✗ Failed to update story status: {}", e);
                Err(format!("Database error: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_loop_creation() {
        let daemon_manager = Arc::new(RwLock::new(DaemonManager::new()));
        let mut loop_mgr = AgentLoop::new(daemon_manager);
        
        // 验证可以创建 Agent Loop 实例
        assert!(!loop_mgr.is_running);
    }
}

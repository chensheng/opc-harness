//! Agent Loop - 自动化执行引擎
//! 
//! 实现从 Sprint 中自动选择用户故事并分配给 Coding Agent 执行的完整流程

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use crate::db::{self, database};
use crate::agent::{DaemonManager, WorktreeManager};
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
        log::info!("[AgentLoop] Starting execution for project: {}", project_id);
        
        // 获取数据库连接
        let conn = database::get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // Step 1: 获取当前活跃的 Sprint
        let active_sprint = db::get_active_sprint(&conn)
            .map_err(|e| format!("Failed to get active sprint: {}", e))?;
        
        if active_sprint.is_none() {
            log::warn!("[AgentLoop] No active sprint found for current time");
            return Ok(0);
        }
        
        let sprint = active_sprint.unwrap();
        log::info!("[AgentLoop] Found active sprint: {} ({} stories)", 
                   sprint.name, sprint.id);
        
        // Step 2: 获取该 Sprint 下待执行的用户故事
        let pending_stories = db::get_pending_stories_by_sprint(&conn, &sprint.id)
            .map_err(|e| format!("Failed to get pending stories: {}", e))?;
        
        if pending_stories.is_empty() {
            log::info!("[AgentLoop] No pending stories in sprint {}", sprint.id);
            return Ok(0);
        }
        
        log::info!("[AgentLoop] Found {} pending stories to process", pending_stories.len());
        
        // Step 3: 为每个故事尝试锁定并启动 Agent
        let mut started_count = 0;
        let agent_id_prefix = format!("coding-{}", Utc::now().timestamp());
        
        for (index, story) in pending_stories.iter().enumerate() {
            // 生成唯一的 Agent ID
            let agent_id = format!("{}-{}", agent_id_prefix, index);
            
            // 尝试锁定故事（乐观锁）
            let locked = db::lock_user_story(&conn, &story.id, &agent_id)
                .map_err(|e| format!("Failed to lock story {}: {}", story.id, e))?;
            
            if !locked {
                log::warn!("[AgentLoop] Story {} already locked or in progress, skipping", story.id);
                continue;
            }
            
            log::info!("[AgentLoop] Locked story {}: {} (Priority: {})", 
                       story.story_number, story.title, story.priority);
            
            // Step 4: 启动 Coding Agent
            match self.start_coding_agent(&agent_id, &story, project_id).await {
                Ok(_) => {
                    started_count += 1;
                    log::info!("[AgentLoop] ✓ Started coding agent for story {}", story.story_number);
                }
                Err(e) => {
                    log::error!("[AgentLoop] ✗ Failed to start agent for story {}: {}", 
                               story.story_number, e);
                    
                    // 标记故事为失败状态
                    let _ = db::fail_user_story(&conn, &story.id, &e);
                }
            }
            
            // 短暂延迟，避免同时启动过多 Agent
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        log::info!("[AgentLoop] Execution completed. Started {} agents", started_count);
        Ok(started_count)
    }

    /// 启动 Coding Agent 进程 (在 Worktree 中执行)
    async fn start_coding_agent(&self, agent_id: &str, story: &crate::models::UserStory, project_id: &str) -> Result<(), String> {
        log::info!("[AgentLoop] Starting coding agent for story: {}", story.title);
        
        // 获取 Daemon Manager 的写锁
        let mut daemon = self.daemon_manager.write().await;
        
        // 构造项目路径
        let workspaces_root = crate::utils::paths::get_workspaces_dir();
        let project_path = workspaces_root.join(project_id);
        
        if !project_path.exists() {
            return Err(format!("Project path does not exist: {:?}", project_path));
        }
        
        let project_path_str = project_path.to_string_lossy().to_string();
        
        // 如果配置了 Worktree 管理器,先创建 Worktree 并在其中启动 Agent
        if let Some(ref wt_manager) = self.worktree_manager {
            log::info!("[AgentLoop] Creating worktree for agent {} and story {}", agent_id, story.id);
            
            // 生成分支名称 (基于 Story ID)
            let branch_name = format!("story-{}", story.story_number);
            
            // 克隆 wt_manager 以便在异步闭包中使用
            let wt_manager_clone = wt_manager.clone();
            let agent_id_clone = agent_id.to_string();
            let story_id_clone = story.id.clone();
            
            match wt_manager.create_worktree(agent_id, &story.id, &branch_name).await {
                Ok(worktree_path) => {
                    log::info!("[AgentLoop] Worktree created at: {}", worktree_path);
                    
                    // 创建消息通道用于接收 AI CLI 输出
                    let (message_tx, mut message_rx) = tokio::sync::mpsc::channel::<AICLIMessage>(100);
                    
                    // 在后台启动消息处理任务
                    let agent_id_for_log = agent_id_clone.clone();
                    let worktree_path_for_write = worktree_path.clone();
                    let worktree_path_for_git = worktree_path.clone();
                    let story_id_for_commit = story_id_clone.clone();
                    
                    tokio::spawn(async move {
                        while let Some(message) = message_rx.recv().await {
                            match message {
                                AICLIMessage::Stdout(line) => {
                                    log::debug!("[AgentLoop:{}] AI Output: {}", agent_id_for_log, line);
                                }
                                AICLIMessage::Stderr(line) => {
                                    log::warn!("[AgentLoop:{}] AI Error: {}", agent_id_for_log, line);
                                }
                                AICLIMessage::GeneratedCode { file_path, content } => {
                                    log::info!("[AgentLoop:{}] Generated code for file: {}", agent_id_for_log, file_path);
                                    
                                    // 将生成的代码写入 Worktree 中的文件
                                    if let Err(e) = Self::write_generated_code(&worktree_path_for_write, &file_path, &content).await {
                                        log::error!("[AgentLoop:{}] Failed to write generated code to {}: {}", 
                                            agent_id_for_log, file_path, e);
                                    } else {
                                        log::info!("[AgentLoop:{}] Successfully wrote generated code to: {}", 
                                            agent_id_for_log, file_path);
                                    }
                                }
                                AICLIMessage::TaskCompleted { success, summary } => {
                                    log::info!("[AgentLoop:{}] Task completed: {} - {}", agent_id_for_log, if success { "SUCCESS" } else { "FAILED" }, summary);
                                    
                                    // 如果任务成功,执行 Git commit & push
                                    if success {
                                        let worktree_path_for_git_clone = worktree_path_for_git.clone();
                                        let story_id_for_commit_clone = story_id_for_commit.clone();
                                        let agent_id_for_git = agent_id_for_log.clone();
                                        
                                        tokio::spawn(async move {
                                            match Self::commit_and_push_changes(&worktree_path_for_git_clone, &story_id_for_commit_clone).await {
                                                Ok(commit_msg) => {
                                                    log::info!("[AgentLoop:{}] Successfully committed and pushed changes: {}", 
                                                        agent_id_for_git, commit_msg);
                                                }
                                                Err(e) => {
                                                    log::error!("[AgentLoop:{}] Failed to commit and push changes: {}", 
                                                        agent_id_for_git, e);
                                                }
                                            }
                                        });
                                    }
                                    
                                    // TODO: 更新 Story 状态
                                }
                            }
                        }
                    });

                    // 在 Worktree 中启动 Agent (带 STDIO 监控)
                    match daemon.spawn_agent_with_stdio_monitoring("coding", &worktree_path, &story_id_clone, message_tx).await {
                        Ok(spawned_agent_id) => {
                            log::info!("[AgentLoop] Successfully spawned coding agent in worktree with STDIO monitoring: {}", spawned_agent_id);
                            
                            // TODO: 将 Agent ID 与 Story ID 关联，便于后续追踪
                            // 可以考虑在数据库中添加 agent_id 字段到 user_stories 表
                            
                            return Ok(());
                        }
                        Err(e) => {
                            log::error!("[AgentLoop] Failed to spawn agent in worktree: {}. Cleaning up worktree.", e);
                            
                            // 清理失败的 Worktree
                            if let Err(cleanup_err) = wt_manager_clone.remove_worktree(&agent_id_clone).await {
                                log::warn!("[AgentLoop] Failed to cleanup worktree: {}", cleanup_err);
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
        
        log::info!("[GitOps] Starting commit and push for worktree: {}", worktree_path);
        
        // 1. 检查是否有变更
        let status_output = TokioCommand::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git status: {}", e))?;
        
        let status_stdout = String::from_utf8_lossy(&status_output.stdout);
        
        if status_stdout.trim().is_empty() {
            log::warn!("[GitOps] No changes detected in worktree: {}", worktree_path);
            return Ok("No changes to commit".to_string());
        }
        
        log::info!("[GitOps] Detected changes:\n{}", status_stdout);
        
        // 2. 添加所有变更文件
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
        
        log::info!("[GitOps] Successfully staged all changes");
        
        // 3. 生成 commit message (基于 Story ID)
        let commit_message = format!("Auto-generated code for story {}", story_id);
        
        // 4. 提交变更
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
        log::info!("[GitOps] Commit successful: {}", commit_stdout);
        
        // 5. 推送到远程分支 (story-{story_number})
        // 注意: 这里假设分支已经存在,如果不存在需要先创建
        let branch_name = format!("story-{}", story_id);
        
        let push_output = TokioCommand::new("git")
            .args(&["push", "-u", "origin", &branch_name])
            .current_dir(worktree_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git push: {}", e))?;
        
        if !push_output.status.success() {
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            // Push 失败可能是分支不存在,尝试先创建分支
            log::warn!("[GitOps] Push failed (possibly branch doesn't exist): {}", stderr);
            
            // 尝试创建并推送分支
            let create_branch_output = TokioCommand::new("git")
                .args(&["checkout", "-b", &branch_name])
                .current_dir(worktree_path)
                .output()
                .await
                .map_err(|e| format!("Failed to create branch: {}", e))?;
            
            if !create_branch_output.status.success() {
                let stderr = String::from_utf8_lossy(&create_branch_output.stderr);
                log::warn!("[GitOps] Branch creation failed (may already exist): {}", stderr);
            }
            
            // 再次尝试推送
            let retry_push_output = TokioCommand::new("git")
                .args(&["push", "-u", "origin", &branch_name])
                .current_dir(worktree_path)
                .output()
                .await
                .map_err(|e| format!("Failed to execute git push (retry): {}", e))?;
            
            if !retry_push_output.status.success() {
                let stderr = String::from_utf8_lossy(&retry_push_output.stderr);
                return Err(format!("git push failed after retry: {}", stderr));
            }
            
            log::info!("[GitOps] Successfully created and pushed branch: {}", branch_name);
        } else {
            log::info!("[GitOps] Successfully pushed to branch: {}", branch_name);
        }
        
        Ok(commit_message)
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

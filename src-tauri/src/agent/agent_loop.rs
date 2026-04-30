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
                                    // TODO: 将生成的代码写入文件
                                }
                                AICLIMessage::TaskCompleted { success, summary } => {
                                    log::info!("[AgentLoop:{}] Task completed: {} - {}", agent_id_for_log, if success { "SUCCESS" } else { "FAILED" }, summary);
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

//! Worktree Lifecycle Manager
//!
//! 负责在 Story 执行完成后自动清理 worktree，防止磁盘空间泄漏。

use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

/// Worktree 清理结果
#[derive(Debug, Clone)]
pub struct CleanupResult {
    pub success: bool,
    pub worktree_path: Option<String>,
    pub branch_name: Option<String>,
    pub error: Option<String>,
    pub message: String,
}

/// Worktree 生命周期管理器
pub struct WorktreeLifecycleManager {
    project_root: PathBuf,
    preserve_branches: bool,
}

impl WorktreeLifecycleManager {
    /// 创建新的 WorktreeLifecycleManager
    pub fn new(project_root: PathBuf, preserve_branches: bool) -> Self {
        Self {
            project_root,
            preserve_branches,
        }
    }

    /// 在 Story 完成后清理 worktree
    ///
    /// # Arguments
    /// * `agent_id` - Agent ID
    /// * `story_id` - Story ID
    /// * `outcome` - 执行结果 ("success" | "failed")
    /// * `worktree_path` - Worktree 路径
    /// * `branch_name` - 分支名称
    ///
    /// # Returns
    /// 清理结果
    pub async fn cleanup_after_story(
        &self,
        agent_id: &str,
        story_id: &str,
        outcome: &str,
        worktree_path: &str,
        branch_name: &str,
    ) -> Result<CleanupResult, String> {
        log::info!(
            "Starting worktree cleanup for story {} (agent: {}, outcome: {})",
            story_id,
            agent_id,
            outcome
        );

        let worktree_path_buf = PathBuf::from(worktree_path);

        // 如果启用了保留分支，先创建永久分支
        if self.preserve_branches {
            match self.create_permanent_branch(branch_name, &worktree_path_buf).await {
                Ok(msg) => log::info!("Branch preserved: {}", msg),
                Err(e) => {
                    log::warn!("Failed to preserve branch: {}. Continuing with cleanup.", e);
                }
            }
        }

        // 删除 worktree 目录
        match self.remove_worktree_directory(&worktree_path_buf).await {
            Ok(_) => log::info!("Worktree directory removed: {}", worktree_path),
            Err(e) => {
                log::error!("Failed to remove worktree directory: {}", e);
                return Ok(CleanupResult {
                    success: false,
                    worktree_path: Some(worktree_path.to_string()),
                    branch_name: Some(branch_name.to_string()),
                    error: Some(format!("Failed to remove directory: {}", e)),
                    message: format!("Directory removal failed: {}", e),
                });
            }
        }

        // 移除 Git worktree 引用
        match self.remove_git_worktree_ref(branch_name).await {
            Ok(_) => log::info!("Git worktree reference removed: {}", branch_name),
            Err(e) => {
                log::warn!("Failed to remove git worktree reference: {}", e);
                // 不返回错误，因为目录已经删除
            }
        }

        log::info!(
            "Worktree cleanup completed successfully for story {}",
            story_id
        );

        Ok(CleanupResult {
            success: true,
            worktree_path: Some(worktree_path.to_string()),
            branch_name: Some(branch_name.to_string()),
            error: None,
            message: format!(
                "Worktree cleaned up successfully (outcome: {})",
                outcome
            ),
        })
    }

    /// 删除 worktree 目录
    async fn remove_worktree_directory(&self, path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            log::warn!("Worktree directory does not exist: {:?}", path);
            return Ok(());
        }

        fs::remove_dir_all(path)
            .await
            .map_err(|e| format!("Failed to remove directory {:?}: {}", path, e))?;

        Ok(())
    }

    /// 移除 Git worktree 引用
    async fn remove_git_worktree_ref(&self, branch_name: &str) -> Result<(), String> {
        // 使用 git worktree remove 命令
        let output = Command::new("git")
            .current_dir(&self.project_root)
            .args(&["worktree", "remove", branch_name])
            .output()
            .await
            .map_err(|e| format!("Failed to execute git worktree remove: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // 如果分支已经不存在，忽略错误
            if stderr.contains("not a working tree") || stderr.contains("does not exist") {
                log::debug!("Worktree reference already removed: {}", branch_name);
                return Ok(());
            }
            return Err(format!(
                "git worktree remove failed: {}",
                stderr.trim()
            ));
        }

        Ok(())
    }

    /// 创建永久分支（在清理前保留工作成果）
    async fn create_permanent_branch(&self, branch_name: &str, worktree_path: &PathBuf) -> Result<String, String> {
        let permanent_branch = format!("preserved/{}", branch_name);

        log::info!(
            "Creating permanent branch '{}' from worktree '{}'",
            permanent_branch,
            branch_name
        );

        // 在 worktree 中创建新分支
        let output = Command::new("git")
            .current_dir(worktree_path)
            .args(&["checkout", "-b", &permanent_branch])
            .output()
            .await
            .map_err(|e| format!("Failed to create permanent branch: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to create branch: {}", stderr.trim()));
        }

        Ok(format!("Created permanent branch: {}", permanent_branch))
    }

    /// 重试清理（最多 3 次）
    pub async fn cleanup_with_retry(
        &self,
        agent_id: &str,
        story_id: &str,
        outcome: &str,
        worktree_path: &str,
        branch_name: &str,
        max_retries: usize,
    ) -> Result<CleanupResult, String> {
        let mut last_error = None;

        for attempt in 1..=max_retries {
            log::info!("Cleanup attempt {}/{}", attempt, max_retries);

            match self
                .cleanup_after_story(agent_id, story_id, outcome, worktree_path, branch_name)
                .await
            {
                Ok(result) if result.success => {
                    return Ok(result);
                }
                Ok(result) => {
                    last_error = result.error.clone();
                    log::warn!("Cleanup attempt {} failed: {:?}", attempt, last_error);
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    log::warn!("Cleanup attempt {} failed with error: {}", attempt, e);
                }
            }

            // 等待一段时间后重试（指数退避）
            if attempt < max_retries {
                let delay_secs = 2u64.pow(attempt as u32); // 2, 4, 8 秒
                log::info!("Retrying in {} seconds...", delay_secs);
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
            }
        }

        Err(format!(
            "Cleanup failed after {} attempts. Last error: {:?}",
            max_retries, last_error
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_remove_worktree_directory() {
        let temp_dir = TempDir::new().unwrap();
        let worktree_path = temp_dir.path().join("test_worktree");
        fs::create_dir_all(&worktree_path).await.unwrap();

        let manager = WorktreeLifecycleManager::new(temp_dir.path().to_path_buf(), false);
        let result = manager.remove_worktree_directory(&worktree_path).await;

        assert!(result.is_ok());
        assert!(!worktree_path.exists());
    }

    #[tokio::test]
    async fn test_remove_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_path = temp_dir.path().join("nonexistent");

        let manager = WorktreeLifecycleManager::new(temp_dir.path().to_path_buf(), false);
        let result = manager.remove_worktree_directory(&nonexistent_path).await;

        assert!(result.is_ok()); // 应该成功（目录不存在时直接返回）
    }

    #[test]
    fn test_create_manager() {
        let temp_dir = TempDir::new().unwrap();
        let manager = WorktreeLifecycleManager::new(temp_dir.path().to_path_buf(), false);

        assert_eq!(manager.project_root, temp_dir.path());
        assert!(!manager.preserve_branches);
    }

    #[test]
    fn test_create_manager_with_preserve() {
        let temp_dir = TempDir::new().unwrap();
        let manager = WorktreeLifecycleManager::new(temp_dir.path().to_path_buf(), true);

        assert!(manager.preserve_branches);
    }
}

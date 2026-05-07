//! Worktree Manager 实现
//! 
//! 负责为每个 Agent 创建和管理独立的 Git Worktree 环境

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Worktree 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeInfo {
    /// Worktree ID (通常是 agent_id)
    pub id: String,
    /// Worktree 路径
    pub path: String,
    /// 关联的分支名称
    pub branch: String,
    /// 关联的故事 ID
    pub story_id: Option<String>,
    /// 创建时间戳
    pub created_at: i64,
    /// 是否为孤立 Worktree (对应的 Agent 已不存在)
    pub is_orphaned: bool,
}

/// Worktree 操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeOperationResult {
    /// 是否成功
    pub success: bool,
    /// Worktree 路径
    pub worktree_path: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 详细消息
    pub message: Option<String>,
}

/// Worktree 管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeManagerConfig {
    /// 项目根路径
    pub project_path: String,
    /// Worktrees 目录名称 (默认: .worktrees)
    pub worktrees_dir_name: String,
    /// 最大磁盘使用量 (字节, 默认: 10GB)
    pub max_disk_usage_bytes: u64,
}

impl Default for WorktreeManagerConfig {
    fn default() -> Self {
        Self {
            project_path: ".".to_string(),
            worktrees_dir_name: ".worktrees".to_string(),
            max_disk_usage_bytes: 10 * 1024 * 1024 * 1024, // 10GB
        }
    }
}

/// Worktree 管理器结构体
#[derive(Debug, Clone)]
pub struct WorktreeManager {
    /// 配置信息
    config: WorktreeManagerConfig,
    /// Worktrees 根目录路径
    worktrees_root: PathBuf,
}

impl WorktreeManager {
    /// 创建新的 Worktree 管理器
    pub fn new(project_path: &str) -> Self {
        let config = WorktreeManagerConfig {
            project_path: project_path.to_string(),
            ..Default::default()
        };
        
        let worktrees_root = Path::new(&config.project_path)
            .join(&config.worktrees_dir_name);
        
        Self {
            config,
            worktrees_root,
        }
    }

    /// 创建带自定义配置的 Worktree 管理器
    pub fn with_config(config: WorktreeManagerConfig) -> Self {
        let worktrees_root = Path::new(&config.project_path)
            .join(&config.worktrees_dir_name);
        
        Self {
            config,
            worktrees_root,
        }
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &WorktreeManagerConfig {
        &self.config
    }

    /// 初始化 Worktrees 目录
    async fn ensure_worktrees_dir(&self) -> Result<(), String> {
        if !self.worktrees_root.exists() {
            tokio::fs::create_dir_all(&self.worktrees_root)
                .await
                .map_err(|e| format!("Failed to create worktrees directory: {}", e))?;
            
            log::info!("[WorktreeManager] Created worktrees directory: {:?}", self.worktrees_root);
        }
        Ok(())
    }

    /// 生成 Worktree 路径
    fn generate_worktree_path(&self, agent_id: &str) -> PathBuf {
        // 清理 agent_id,确保路径安全
        let safe_agent_id = agent_id
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>();
        
        self.worktrees_root.join(format!("agent-{}", safe_agent_id))
    }

    /// 检查磁盘空间是否足够
    async fn check_disk_space(&self, required_bytes: u64) -> Result<(), String> {
        let current_usage = self.get_disk_usage().await?;
        
        if current_usage + required_bytes > self.config.max_disk_usage_bytes {
            return Err(format!(
                "Insufficient disk space. Current usage: {} MB, Limit: {} MB, Required: {} MB",
                current_usage / 1024 / 1024,
                self.config.max_disk_usage_bytes / 1024 / 1024,
                required_bytes / 1024 / 1024
            ));
        }
        
        Ok(())
    }

    /// 获取当前磁盘使用量 (字节)
    pub async fn get_disk_usage(&self) -> Result<u64, String> {
        if !self.worktrees_root.exists() {
            return Ok(0);
        }
        
        let mut total_size: u64 = 0;
        
        // 递归计算目录大小
        let mut entries = tokio::fs::read_dir(&self.worktrees_root)
            .await
            .map_err(|e| format!("Failed to read worktrees directory: {}", e))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Failed to read entry: {}", e))? {
            let path = entry.path();
            if path.is_dir() {
                total_size += self.calculate_dir_size(&path).await?;
            }
        }
        
        Ok(total_size)
    }

    /// 递归计算目录大小
    async fn calculate_dir_size(&self, dir_path: &Path) -> Result<u64, String> {
        let mut size: u64 = 0;
        
        let mut entries = tokio::fs::read_dir(dir_path)
            .await
            .map_err(|e| format!("Failed to read directory {:?}: {}", dir_path, e))?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Failed to read entry: {}", e))? {
            let path = entry.path();
            let metadata = tokio::fs::metadata(&path)
                .await
                .map_err(|e| format!("Failed to get metadata for {:?}: {}", path, e))?;
            
            if metadata.is_dir() {
                // 使用 Box::pin 避免递归异步函数的栈溢出
                let sub_size = Box::pin(self.calculate_dir_size(&path)).await?;
                size += sub_size;
            } else {
                size += metadata.len();
            }
        }
        
        Ok(size)
    }

    /// 创建 Worktree
    /// 
    /// # Arguments
    /// * `agent_id` - Agent ID,用于命名 Worktree
    /// * `story_id` - 关联的用户故事 ID
    /// * `branch_name` - 要检出的分支名称
    /// 
    /// # Returns
    /// Worktree 的绝对路径
    pub async fn create_worktree(
        &self,
        agent_id: &str,
        _story_id: &str,
        branch_name: &str,
    ) -> Result<String, String> {
        // 1. 确保 worktrees 目录存在
        self.ensure_worktrees_dir().await?;
        
        // 2. 检查磁盘空间
        self.check_disk_space(500 * 1024 * 1024).await?; // 预估需要 500MB
        
        // 3. 验证 Git 仓库已初始化（应该已由项目创建/打开流程处理）
        self.validate_git_repository().await?;
        
        // 4. 生成 Worktree 路径
        let worktree_path = self.generate_worktree_path(agent_id);
        
        // 5. 检查 Worktree 是否已存在
        if worktree_path.exists() {
            return Err(format!(
                "Worktree already exists for agent {}: {:?}",
                agent_id, worktree_path
            ));
        }
        
        // 6. 执行 git worktree add 命令
        let worktree_path_str = worktree_path.to_string_lossy().to_string();
        let project_path = &self.config.project_path;
        
        log::info!(
            "[WorktreeManager] Creating worktree for agent {} at {} from branch {}",
            agent_id,
            worktree_path_str,
            branch_name
        );
        
        // 构建 git worktree add 命令
        // git worktree add <path> <branch>
        let output = tokio::process::Command::new("git")
            .current_dir(project_path)
            .args(&["worktree", "add", &worktree_path_str, branch_name])
            .output()
            .await
            .map_err(|e| format!("Failed to execute git worktree add: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "Git worktree add failed: {}",
                stderr
            ));
        }
        
        log::info!("[WorktreeManager] Successfully created worktree at {}", worktree_path_str);
        
        Ok(worktree_path_str)
    }

    /// 验证 Git 仓库已初始化
    /// 
    /// 注意：Git 应该在项目创建/打开时已初始化，这里只做验证
    async fn validate_git_repository(&self) -> Result<(), String> {
        let project_path = &self.config.project_path;
        let git_dir = Path::new(project_path).join(".git");
        
        // 检查 .git 目录是否存在
        if git_dir.exists() {
            log::debug!("[WorktreeManager] ✓ Git repository verified at {}", project_path);
            return Ok(());
        }
        
        // Git 未初始化，返回错误（正常情况下不应该到达这里）
        log::error!("[WorktreeManager] ✗ Git repository not found at {}", project_path);
        log::error!("[WorktreeManager]   Git should have been initialized during project creation/opening");
        log::error!("[WorktreeManager]   Please initialize Git manually or recreate the project");
        
        Err(format!(
            "Git repository not initialized at {}. \
             Git should be initialized automatically when creating or opening a project.",
            project_path
        ))
    }

    /// 删除 Worktree
    /// 
    /// # Arguments
    /// * `agent_id` - Agent ID
    pub async fn remove_worktree(&self, agent_id: &str) -> Result<(), String> {
        let worktree_path = self.generate_worktree_path(agent_id);
        
        if !worktree_path.exists() {
            log::warn!("[WorktreeManager] Worktree does not exist for agent {}: {:?}", agent_id, worktree_path);
            return Ok(());
        }
        
        let worktree_path_str = worktree_path.to_string_lossy().to_string();
        let project_path = &self.config.project_path;
        
        log::info!("[WorktreeManager] Removing worktree for agent {} at {}", agent_id, worktree_path_str);
        
        // 执行 git worktree remove 命令
        let output = tokio::process::Command::new("git")
            .current_dir(project_path)
            .args(&["worktree", "remove", "--force", &worktree_path_str])
            .output()
            .await
            .map_err(|e| format!("Failed to execute git worktree remove: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // 如果 git worktree remove 失败,尝试直接删除目录
            log::warn!("[WorktreeManager] Git worktree remove failed, trying direct deletion: {}", stderr);
            
            tokio::fs::remove_dir_all(&worktree_path)
                .await
                .map_err(|e| format!("Failed to delete worktree directory: {}", e))?;
        }
        
        log::info!("[WorktreeManager] Successfully removed worktree for agent {}", agent_id);
        
        Ok(())
    }

    /// 列出所有 Worktrees
    pub async fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>, String> {
        if !self.worktrees_root.exists() {
            return Ok(Vec::new());
        }
        
        let mut worktrees = Vec::new();
        let project_path = &self.config.project_path;
        
        // 执行 git worktree list 命令
        let output = tokio::process::Command::new("git")
            .current_dir(project_path)
            .args(&["worktree", "list", "--porcelain"])
            .output()
            .await
            .map_err(|e| format!("Failed to execute git worktree list: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git worktree list failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // 解析 git worktree list 输出
        let mut current_worktree: Option<WorktreeInfo> = None;
        
        for line in stdout.lines() {
            if line.starts_with("worktree ") {
                // 保存前一个 worktree
                if let Some(wt) = current_worktree.take() {
                    worktrees.push(wt);
                }
                
                let path = line.trim_start_matches("worktree ").to_string();
                
                // 提取 agent_id 从路径
                let agent_id = if let Some(file_name) = Path::new(&path).file_name() {
                    file_name.to_string_lossy()
                        .trim_start_matches("agent-")
                        .to_string()
                } else {
                    String::new()
                };
                
                current_worktree = Some(WorktreeInfo {
                    id: agent_id,
                    path,
                    branch: String::new(),
                    story_id: None,
                    created_at: chrono::Utc::now().timestamp(),
                    is_orphaned: false, // TODO: 需要与活跃的 Agent 列表对比
                });
            } else if line.starts_with("HEAD ") {
                // HEAD 行包含分支信息
                if let Some(ref mut wt) = current_worktree {
                    let head_ref = line.trim_start_matches("HEAD ").trim();
                    if head_ref.starts_with("refs/heads/") {
                        wt.branch = head_ref.trim_start_matches("refs/heads/").to_string();
                    }
                }
            }
        }
        
        // 添加最后一个 worktree
        if let Some(wt) = current_worktree {
            worktrees.push(wt);
        }
        
        Ok(worktrees)
    }

    /// 清理孤立的 Worktrees (对应的 Agent 已不存在)
    /// 
    /// # Returns
    /// 清理的 Worktree 数量
    pub async fn cleanup_orphaned_worktrees(&self) -> Result<usize, String> {
        let worktrees = self.list_worktrees().await?;
        let mut cleaned_count = 0;
        
        for wt in worktrees {
            if wt.is_orphaned {
                log::info!("[WorktreeManager] Cleaning up orphaned worktree: {}", wt.id);
                
                match self.remove_worktree(&wt.id).await {
                    Ok(_) => cleaned_count += 1,
                    Err(e) => log::error!("[WorktreeManager] Failed to remove orphaned worktree {}: {}", wt.id, e),
                }
            }
        }
        
        if cleaned_count > 0 {
            log::info!("[WorktreeManager] Cleaned up {} orphaned worktrees", cleaned_count);
        }
        
        Ok(cleaned_count)
    }

    /// 检查 Worktree 是否存在
    pub async fn worktree_exists(&self, agent_id: &str) -> Result<bool, String> {
        let worktree_path = self.generate_worktree_path(agent_id);
        Ok(worktree_path.exists())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_worktree_manager_creation() {
        let manager = WorktreeManager::new("/tmp/test-project");
        
        assert_eq!(manager.config.project_path, "/tmp/test-project");
        assert_eq!(manager.config.worktrees_dir_name, ".worktrees");
        assert_eq!(manager.worktrees_root, PathBuf::from("/tmp/test-project/.worktrees"));
    }
    
    #[test]
    fn test_generate_worktree_path() {
        let manager = WorktreeManager::new("/tmp/test-project");
        
        let path = manager.generate_worktree_path("agent-123");
        // Use contains to handle platform-specific path separators
        assert!(path.to_string_lossy().contains(".worktrees"));
        assert!(path.to_string_lossy().contains("agent-agent-123"));
        
        let path = manager.generate_worktree_path("agent@#$%");
        assert!(path.to_string_lossy().contains(".worktrees"));
        // agent@#$% -> agent____ (4 special chars replaced with _)
        assert!(path.to_string_lossy().contains("agent-agent____"));
    }
}

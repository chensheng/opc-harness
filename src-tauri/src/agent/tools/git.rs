//! Git Tools for Native Coding Agent
//!
//! 提供 Git 版本控制操作工具。

use std::path::PathBuf;
use tokio::process::Command;

/// Git 工具集
pub struct GitTools {
    repo_path: PathBuf,
}

impl GitTools {
    /// 创建新的 Git 工具集
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    /// 执行 Git 命令
    async fn execute_git(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(args)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git command failed: {}", stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// 获取 Git 状态
    pub async fn git_status(&self) -> Result<String, String> {
        self.execute_git(&["status", "--porcelain"]).await
    }

    /// 获取文件差异
    pub async fn git_diff(&self, file: Option<&str>) -> Result<String, String> {
        let mut args = vec!["diff"];

        if let Some(f) = file {
            args.push("--");
            args.push(f);
        } else {
            args.push("HEAD");
        }

        self.execute_git(&args).await
    }

    /// 提交代码变更
    pub async fn git_commit(&self, message: &str) -> Result<String, String> {
        // 先添加所有变更
        self.execute_git(&["add", "."]).await?;

        // 检查是否有变更
        let status = self.git_status().await?;
        if status.trim().is_empty() {
            return Err("No changes to commit".to_string());
        }

        // 提交
        self.execute_git(&["commit", "-m", message]).await?;

        // 获取最新的 commit hash
        let hash = self.execute_git(&["rev-parse", "HEAD"]).await?;

        Ok(format!("Committed successfully: {}", hash.trim()))
    }

    /// 创建 Worktree
    pub async fn create_worktree(&self, agent_id: &str) -> Result<String, String> {
        let branch_name = format!("agent-{}", agent_id);
        let worktree_path = self.repo_path.join(".worktrees").join(agent_id);

        // 检查工作树是否已存在
        if worktree_path.exists() {
            return Err("Worktree already exists".to_string());
        }

        // 创建父目录
        tokio::fs::create_dir_all(self.repo_path.join(".worktrees"))
            .await
            .map_err(|e| format!("Failed to create worktrees directory: {}", e))?;

        // 创建新分支
        self.execute_git(&["branch", &branch_name]).await.ok();

        // 创建 worktree
        self.execute_git(&[
            "worktree",
            "add",
            worktree_path.to_str().unwrap(),
            &branch_name,
        ])
        .await?;

        Ok(worktree_path.to_string_lossy().to_string())
    }

    /// 清理 Worktree
    pub async fn cleanup_worktree(&self, agent_id: &str) -> Result<String, String> {
        let branch_name = format!("agent-{}", agent_id);
        let worktree_path = self.repo_path.join(".worktrees").join(agent_id);

        // 删除 worktree
        if worktree_path.exists() {
            self.execute_git(&["worktree", "remove", worktree_path.to_str().unwrap()])
                .await
                .ok();

            // 删除目录
            tokio::fs::remove_dir_all(&worktree_path).await.ok();
        }

        // 删除分支
        self.execute_git(&["branch", "-D", &branch_name]).await.ok();

        Ok(format!("Worktree cleaned up: {}", agent_id))
    }

    /// 创建新分支
    pub async fn create_branch(&self, branch_name: &str) -> Result<String, String> {
        self.execute_git(&["checkout", "-b", branch_name]).await?;
        Ok(branch_name.to_string())
    }

    /// 列出所有分支
    pub async fn list_branches(&self) -> Result<String, String> {
        self.execute_git(&["branch", "-a"]).await
    }

    /// 获取当前分支
    pub async fn current_branch(&self) -> Result<String, String> {
        self.execute_git(&["branch", "--show-current"]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as StdCommand;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_git_status() {
        let temp_dir = TempDir::new().unwrap();

        // 初始化 Git 仓库
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["init"])
            .output()
            .unwrap();

        let tools = GitTools::new(temp_dir.path().to_path_buf());
        let status = tools.git_status().await.unwrap();

        assert_eq!(status.trim(), "");
    }

    #[tokio::test]
    async fn test_git_commit_no_changes() {
        let temp_dir = TempDir::new().unwrap();

        // 初始化 Git 仓库
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["init"])
            .output()
            .unwrap();

        let tools = GitTools::new(temp_dir.path().to_path_buf());
        let result = tools.git_commit("Test commit").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No changes to commit"));
    }

    #[tokio::test]
    async fn test_git_diff_no_changes() {
        let temp_dir = TempDir::new().unwrap();

        // 初始化 Git 仓库
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["init"])
            .output()
            .unwrap();

        // 创建初始提交（避免 HEAD 不存在的问题）
        tokio::fs::write(temp_dir.path().join("README.md"), "# Test")
            .await
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["add", "."])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.email", "test@example.com"])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.name", "Test User"])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["commit", "-m", "Initial commit"])
            .output()
            .unwrap();

        let tools = GitTools::new(temp_dir.path().to_path_buf());
        let diff = tools.git_diff(None).await.unwrap();

        assert_eq!(diff.trim(), "");
    }

    #[tokio::test]
    async fn test_git_branch_operations() {
        let temp_dir = TempDir::new().unwrap();

        // 初始化 Git 仓库
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["init"])
            .output()
            .unwrap();

        // 配置用户信息（Git 提交必需）
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.email", "test@example.com"])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.name", "Test User"])
            .output()
            .unwrap();

        // 创建初始文件并提交（Git 需要至少一个提交才能创建分支）
        tokio::fs::write(temp_dir.path().join("README.md"), "# Test")
            .await
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["add", "."])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["commit", "-m", "Initial commit"])
            .output()
            .unwrap();

        let tools = GitTools::new(temp_dir.path().to_path_buf());

        // 测试创建分支
        let result = tools.create_branch("test-branch").await;
        assert!(result.is_ok());

        // 测试列出分支
        let branches = tools.list_branches().await.unwrap();
        assert!(branches.contains("test-branch"));

        // 测试当前分支
        let current = tools.current_branch().await.unwrap();
        assert_eq!(current.trim(), "test-branch");
    }

    #[tokio::test]
    async fn test_git_worktree_creation() {
        let temp_dir = TempDir::new().unwrap();

        // 初始化 Git 仓库
        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["init"])
            .output()
            .unwrap();

        // 创建初始提交（worktree 需要至少一个提交）
        tokio::fs::write(temp_dir.path().join("README.md"), "# Test")
            .await
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["add", "."])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.email", "test@example.com"])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["config", "user.name", "Test User"])
            .output()
            .unwrap();

        StdCommand::new("git")
            .current_dir(temp_dir.path())
            .args(&["commit", "-m", "Initial commit"])
            .output()
            .unwrap();

        let tools = GitTools::new(temp_dir.path().to_path_buf());

        // 测试创建 worktree
        let result = tools.create_worktree("test-agent").await;
        assert!(result.is_ok());

        let worktree_path = result.unwrap();
        assert!(std::path::Path::new(&worktree_path).exists());

        // 测试清理 worktree
        let cleanup_result = tools.cleanup_worktree("test-agent").await;
        assert!(cleanup_result.is_ok());
    }
}

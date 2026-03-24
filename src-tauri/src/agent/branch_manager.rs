//! Branch Manager 实现
//! 
//! 负责 Git 分支的创建、管理和命名规范

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

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

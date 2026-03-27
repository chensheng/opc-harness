//! Git Commit Assistant 实现
//! 
//! 负责分析代码变更，使用 AI 生成规范的 Git 提交信息

use serde::{Deserialize, Serialize};
use std::process::Command;

/// 提交类型（Conventional Commits）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommitType {
    /// 新功能 (Feature)
    Feat,
    /// Bug 修复 (Fix)
    Fix,
    /// 文档更新 (Documentation)
    Docs,
    /// 代码格式 (Style)
    Style,
    /// 重构 (Refactor)
    Refactor,
    /// 性能优化 (Performance)
    Perf,
    /// 测试 (Test)
    Test,
    /// 构建/工具 (Chore)
    Chore,
}

impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitType::Feat => write!(f, "feat"),
            CommitType::Fix => write!(f, "fix"),
            CommitType::Docs => write!(f, "docs"),
            CommitType::Style => write!(f, "style"),
            CommitType::Refactor => write!(f, "refactor"),
            CommitType::Perf => write!(f, "perf"),
            CommitType::Test => write!(f, "test"),
            CommitType::Chore => write!(f, "chore"),
        }
    }
}

impl std::str::FromStr for CommitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "feat" | "feature" => Ok(CommitType::Feat),
            "fix" | "bugfix" => Ok(CommitType::Fix),
            "docs" | "doc" => Ok(CommitType::Docs),
            "style" => Ok(CommitType::Style),
            "refactor" | "ref" => Ok(CommitType::Refactor),
            "perf" | "performance" => Ok(CommitType::Perf),
            "test" => Ok(CommitType::Test),
            "chore" => Ok(CommitType::Chore),
            _ => Err(format!("Invalid commit type: {}", s)),
        }
    }
}

/// 文件变更类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileChangeType {
    /// 新增文件
    Added,
    /// 修改文件
    Modified,
    /// 删除文件
    Deleted,
    /// 重命名文件
    Renamed,
}

impl std::fmt::Display for FileChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileChangeType::Added => write!(f, "A"),
            FileChangeType::Modified => write!(f, "M"),
            FileChangeType::Deleted => write!(f, "D"),
            FileChangeType::Renamed => write!(f, "R"),
        }
    }
}

/// 变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeInfo {
    /// 文件路径
    pub file_path: String,
    /// 新增行数
    pub additions: usize,
    /// 删除行数
    pub deletions: usize,
    /// 变更类型
    pub change_type: FileChangeType,
    /// 变更摘要（前几行 diff）
    pub diff_summary: Option<String>,
}

/// 提交信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessage {
    /// 提交类型
    pub commit_type: CommitType,
    /// 作用域（可选）
    pub scope: Option<String>,
    /// 简短摘要（≤50 字符）
    pub summary: String,
    /// 详细描述
    pub description: String,
    /// 破坏性变更列表
    pub breaking_changes: Vec<String>,
    /// 变更文件列表
    pub changed_files: Vec<String>,
    /// 完整的格式化提交信息
    pub formatted: String,
}

impl CommitMessage {
    /// 创建新的提交信息
    pub fn new(
        commit_type: CommitType,
        summary: String,
        description: String,
        changed_files: Vec<String>,
    ) -> Self {
        let scope = None;
        let breaking_changes = vec![];
        
        // 格式化：type(scope): summary\n\ndescription\n\nBREAKING CHANGE: ...\n\nChanged files: ...
        let formatted = Self::format_commit_message(&commit_type, &scope, &summary, &description, &breaking_changes, &changed_files);

        Self {
            commit_type,
            scope,
            summary,
            description,
            breaking_changes,
            changed_files,
            formatted,
        }
    }

    /// 创建带作用域的提交信息
    pub fn with_scope(
        commit_type: CommitType,
        scope: String,
        summary: String,
        description: String,
        changed_files: Vec<String>,
    ) -> Self {
        let breaking_changes = vec![];
        let formatted = Self::format_commit_message(&commit_type, &Some(scope.clone()), &summary, &description, &breaking_changes, &changed_files);

        Self {
            commit_type,
            scope: Some(scope),
            summary,
            description,
            breaking_changes,
            changed_files,
            formatted,
        }
    }

    /// 格式化提交信息
    fn format_commit_message(
        commit_type: &CommitType,
        scope: &Option<String>,
        summary: &str,
        description: &str,
        breaking_changes: &[String],
        changed_files: &[String],
    ) -> String {
        let mut formatted = if let Some(ref s) = scope {
            format!("{}({}): {}", commit_type, s, summary)
        } else {
            format!("{}: {}", commit_type, summary)
        };

        if !description.is_empty() {
            formatted.push_str("\n\n");
            formatted.push_str(description);
        }

        if !breaking_changes.is_empty() {
            formatted.push_str("\n\n");
            formatted.push_str("BREAKING CHANGES:\n");
            for change in breaking_changes {
                formatted.push_str(&format!("- {}\n", change));
            }
        }

        if !changed_files.is_empty() {
            formatted.push_str("\n\n");
            formatted.push_str("Changed files:\n");
            for file in changed_files {
                formatted.push_str(&format!("- {}\n", file));
            }
        }

        formatted
    }

    /// 更新作用域并重新生成 formatted
    pub fn set_scope(&mut self, scope: Option<String>) {
        self.scope = scope;
        self.formatted = Self::format_commit_message(
            &self.commit_type,
            &self.scope,
            &self.summary,
            &self.description,
            &self.breaking_changes,
            &self.changed_files,
        );
    }
}

/// Git Commit Assistant 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommitAssistantConfig {
    /// 项目路径
    pub project_path: String,
    /// 是否使用 AI 生成
    pub use_ai: bool,
    /// 是否包含文件列表
    pub include_file_list: bool,
    /// 摘要最大长度
    pub max_summary_length: usize,
    /// 是否符合 Conventional Commits 规范
    pub conventional_commit: bool,
}

impl Default for GitCommitAssistantConfig {
    fn default() -> Self {
        Self {
            project_path: ".".to_string(),
            use_ai: true,
            include_file_list: true,
            max_summary_length: 50,
            conventional_commit: true,
        }
    }
}

/// Git Commit Assistant 状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommitStatus {
    /// 等待开始
    Pending,
    /// 分析变更中
    AnalyzingChanges,
    /// 分类变更中
    CategorizingChanges,
    /// 生成提交信息中
    GeneratingMessage,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

impl std::fmt::Display for CommitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitStatus::Pending => write!(f, "等待开始"),
            CommitStatus::AnalyzingChanges => write!(f, "分析变更中"),
            CommitStatus::CategorizingChanges => write!(f, "分类变更中"),
            CommitStatus::GeneratingMessage => write!(f, "生成提交信息中"),
            CommitStatus::Completed => write!(f, "已完成"),
            CommitStatus::Failed(reason) => write!(f, "失败：{}", reason),
        }
    }
}

/// Git Commit Assistant
#[derive(Debug, Clone)]
pub struct GitCommitAssistant {
    /// 配置信息
    pub config: GitCommitAssistantConfig,
    /// 当前状态
    pub status: CommitStatus,
    /// 会话 ID
    pub session_id: String,
}

impl GitCommitAssistant {
    /// 创建新的 Git Commit Assistant
    pub fn new(config: GitCommitAssistantConfig) -> Self {
        Self {
            config,
            status: CommitStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// 执行完整的提交信息生成流程
    pub async fn generate_commit_message(&mut self) -> Result<CommitMessage, String> {
        log::info!("开始执行 Git Commit Assistant - Session: {}", self.session_id);

        // 1. 分析变更
        self.status = CommitStatus::AnalyzingChanges;
        log::info!("步骤 1/3: 分析代码变更");
        
        let changes = self.analyze_changes()?;
        
        if changes.is_empty() {
            log::warn!("没有检测到变更");
            self.status = CommitStatus::Failed("No changes detected".to_string());
            return Err("No changes to commit".to_string());
        }

        log::info!("检测到 {} 个文件变更", changes.len());

        // 2. 分类变更
        self.status = CommitStatus::CategorizingChanges;
        log::info!("步骤 2/3: 分类变更类型");
        
        let commit_type = self.categorize_changes(&changes);
        log::info!("识别变更类型：{:?}", commit_type);

        // 3. 生成提交信息
        self.status = CommitStatus::GeneratingMessage;
        log::info!("步骤 3/3: 生成提交信息");
        
        let message = self.create_commit_message(commit_type, &changes)?;
        
        // 完成
        self.status = CommitStatus::Completed;
        log::info!("提交信息生成完成：{}", message.formatted.lines().next().unwrap_or(""));
        
        Ok(message)
    }

    /// 分析代码变更
    fn analyze_changes(&self) -> Result<Vec<ChangeInfo>, String> {
        // 执行 git diff --cached --stat
        let output = Command::new("git")
            .args(&["diff", "--cached", "--stat"])
            .current_dir(&self.config.project_path)
            .output()
            .map_err(|e| format!("Failed to execute git diff: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git diff failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut changes = Vec::new();

        // 解析 git diff --stat 输出
        // 格式： src/file.rs | 10 +++++++---
        for line in stdout.lines() {
            if line.trim().is_empty() || line.contains("file changed") || line.contains("insertion") || line.contains("deletion") {
                continue;
            }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                let file_path = parts[0].trim().to_string();
                let stats = parts.get(1).unwrap_or(&"").trim();
                
                // 解析新增和删除行数
                let (additions, deletions) = self.parse_diff_stats(stats);
                
                // 确定变更类型
                let change_type = if additions > 0 && deletions == 0 && file_path.starts_with('/') {
                    FileChangeType::Added
                } else if deletions > 0 && additions == 0 {
                    FileChangeType::Deleted
                } else {
                    FileChangeType::Modified
                };

                changes.push(ChangeInfo {
                    file_path,
                    additions,
                    deletions,
                    change_type,
                    diff_summary: None,
                });
            }
        }

        Ok(changes)
    }

    /// 解析 diff 统计行
    fn parse_diff_stats(&self, stats: &str) -> (usize, usize) {
        // 从 "10 +++++++---" 中提取 + 和 - 的数量
        let plus_count = stats.chars().filter(|&c| c == '+').count();
        let minus_count = stats.chars().filter(|&c| c == '-').count();
        
        // 如果数字存在则使用数字，否则使用符号计数作为估计
        let numbers: Vec<usize> = stats
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .ok()
            .into_iter()
            .collect();

        if let Some(total) = numbers.first() {
            // 如果有总数字，按比例分配
            if plus_count + minus_count > 0 {
                let total_symbols = plus_count + minus_count;
                let additions = (*total * plus_count) / total_symbols;
                let deletions = (*total * minus_count) / total_symbols;
                (additions, deletions)
            } else {
                (*total, 0)
            }
        } else {
            // 使用符号计数作为估计
            (plus_count, minus_count)
        }
    }

    /// 分类变更类型
    fn categorize_changes(&self, changes: &[ChangeInfo]) -> CommitType {
        // 基于变更内容推断提交类型
        let mut has_test = false;
        let mut has_doc = false;
        let mut has_fix = false;
        let mut has_feature = false;
        let mut has_style = false;

        for change in changes {
            let file_lower = change.file_path.to_lowercase();
            
            // 检测测试文件
            if file_lower.contains("test") || file_lower.contains("spec") {
                has_test = true;
            }
            
            // 检测文档文件
            if file_lower.ends_with(".md") || file_lower.ends_with(".txt") {
                has_doc = true;
            }
            
            // 检测样式文件
            if file_lower.ends_with(".css") || file_lower.ends_with(".scss") || file_lower.ends_with(".less") {
                has_style = true;
            }
            
            // 检测修复相关关键字
            if change.file_path.contains("fix") || change.diff_summary.as_ref().map_or(false, |s| s.contains("fix")) {
                has_fix = true;
            }
            
            // 检测新功能相关关键字
            if change.file_path.contains("feature") || change.diff_summary.as_ref().map_or(false, |s| s.contains("add") || s.contains("new")) {
                has_feature = true;
            }
        }

        // 优先级判断
        if has_test && changes.iter().all(|c| c.file_path.contains("test") || c.file_path.contains("spec")) {
            CommitType::Test
        } else if has_doc {
            CommitType::Docs
        } else if has_style {
            CommitType::Style
        } else if has_fix {
            CommitType::Fix
        } else if has_feature {
            CommitType::Feat
        } else {
            // 默认视为重构
            CommitType::Refactor
        }
    }

    /// 创建提交信息
    fn create_commit_message(&self, commit_type: CommitType, changes: &[ChangeInfo]) -> Result<CommitMessage, String> {
        // 生成摘要
        let summary = self.generate_summary(commit_type.clone(), changes)?;
        
        // 生成描述
        let description = self.generate_description(changes);
        
        // 收集变更文件列表
        let changed_files: Vec<String> = if self.config.include_file_list {
            changes.iter().map(|c| c.file_path.clone()).collect()
        } else {
            vec![]
        };

        // 创建提交信息
        let message = CommitMessage::new(
            commit_type,
            summary,
            description,
            changed_files,
        );

        Ok(message)
    }

    /// 生成提交摘要
    fn generate_summary(&self, commit_type: CommitType, changes: &[ChangeInfo]) -> Result<String, String> {
        // 基于变更类型和文件生成简洁摘要
        let action = match commit_type {
            CommitType::Feat => "Add",
            CommitType::Fix => "Fix",
            CommitType::Docs => "Update docs",
            CommitType::Style => "Format",
            CommitType::Refactor => "Refactor",
            CommitType::Perf => "Optimize",
            CommitType::Test => "Add tests",
            CommitType::Chore => "Update",
        };

        // 提取主要变更的文件名
        let main_changes: Vec<&str> = changes
            .iter()
            .take(3)
            .map(|c| {
                c.file_path
                    .split('/')
                    .last()
                    .unwrap_or(&c.file_path)
                    .split('.')
                    .next()
                    .unwrap_or("")
            })
            .filter(|s| !s.is_empty())
            .collect();

        let target = if main_changes.is_empty() {
            "code"
        } else {
            &main_changes.join(", ")
        };

        let summary = format!("{} {}", action, target);
        
        // 确保不超过最大长度
        if summary.len() <= self.config.max_summary_length {
            Ok(summary)
        } else {
            Ok(summary[..self.config.max_summary_length].to_string())
        }
    }

    /// 生成详细描述
    fn generate_description(&self, changes: &[ChangeInfo]) -> String {
        let mut descriptions = Vec::new();

        for change in changes {
            let change_desc = match change.change_type {
                FileChangeType::Added => format!("Add new file: {}", change.file_path),
                FileChangeType::Deleted => format!("Remove file: {}", change.file_path),
                FileChangeType::Modified => {
                    if change.additions > 0 && change.deletions > 0 {
                        format!("Modify {}: +{} -{}", change.file_path, change.additions, change.deletions)
                    } else if change.additions > 0 {
                        format!("Add lines to {}: +{}", change.file_path, change.additions)
                    } else {
                        format!("Remove lines from {}: -{}", change.file_path, change.deletions)
                    }
                }
                FileChangeType::Renamed => format!("Rename file: {}", change.file_path),
            };
            descriptions.push(change_desc);
        }

        descriptions.join("\n")
    }

    /// 验证是否符合 Conventional Commits 规范
    pub fn validate_conventional_commit(&self, message: &str) -> Result<bool, String> {
        // Conventional Commits 格式：type(scope): description
        // type 必须是 feat, fix, docs, style, refactor, perf, test, chore 之一
        
        let pattern = regex::Regex::new(r"^(feat|fix|docs|style|refactor|perf|test|chore)(\([a-z0-9-]+\))?: .{1,72}$")
            .map_err(|e| format!("Invalid regex: {}", e))?;

        if pattern.is_match(message) {
            Ok(true)
        } else {
            Err("Message does not follow Conventional Commits format".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_commit_type_display() {
        assert_eq!(format!("{}", CommitType::Feat), "feat");
        assert_eq!(format!("{}", CommitType::Fix), "fix");
        assert_eq!(format!("{}", CommitType::Docs), "docs");
        assert_eq!(format!("{}", CommitType::Refactor), "refactor");
    }

    #[test]
    fn test_commit_type_from_str() {
        assert_eq!(CommitType::from_str("feat").unwrap(), CommitType::Feat);
        assert_eq!(CommitType::from_str("fix").unwrap(), CommitType::Fix);
        assert_eq!(CommitType::from_str("docs").unwrap(), CommitType::Docs);
        assert_eq!(CommitType::from_str("refactor").unwrap(), CommitType::Refactor);
        assert!(CommitType::from_str("invalid").is_err());
    }

    #[test]
    fn test_file_change_type_display() {
        assert_eq!(format!("{}", FileChangeType::Added), "A");
        assert_eq!(format!("{}", FileChangeType::Modified), "M");
        assert_eq!(format!("{}", FileChangeType::Deleted), "D");
    }

    #[test]
    fn test_change_info_creation() {
        let change = ChangeInfo {
            file_path: "src/main.rs".to_string(),
            additions: 10,
            deletions: 5,
            change_type: FileChangeType::Modified,
            diff_summary: None,
        };

        assert_eq!(change.file_path, "src/main.rs");
        assert_eq!(change.additions, 10);
        assert_eq!(change.deletions, 5);
    }

    #[test]
    fn test_commit_message_creation() {
        let message = CommitMessage::new(
            CommitType::Feat,
            "add new feature".to_string(),
            "This is a detailed description".to_string(),
            vec!["src/feature.rs".to_string()],
        );

        assert_eq!(message.commit_type, CommitType::Feat);
        assert_eq!(message.summary, "add new feature");
        assert!(message.formatted.starts_with("feat: add new feature"));
        assert!(message.formatted.contains("This is a detailed description"));
    }

    #[test]
    fn test_commit_message_with_scope() {
        let mut message = CommitMessage::new(
            CommitType::Fix,
            "resolve bug".to_string(),
            "".to_string(),
            vec![],
        );
        message.set_scope(Some("api".to_string()));

        assert_eq!(message.scope, Some("api".to_string()));
        assert!(message.formatted.starts_with("fix(api): resolve bug"));
    }

    #[test]
    fn test_git_commit_assistant_creation() {
        let config = GitCommitAssistantConfig {
            project_path: "/tmp/test".to_string(),
            use_ai: false,
            include_file_list: true,
            max_summary_length: 50,
            conventional_commit: true,
        };

        let assistant = GitCommitAssistant::new(config);

        assert_eq!(assistant.config.project_path, "/tmp/test");
        assert_eq!(assistant.status, CommitStatus::Pending);
        assert!(!assistant.session_id.is_empty());
    }

    #[test]
    fn test_default_config() {
        let config = GitCommitAssistantConfig::default();

        assert_eq!(config.project_path, ".");
        assert!(config.use_ai);
        assert!(config.include_file_list);
        assert_eq!(config.max_summary_length, 50);
        assert!(config.conventional_commit);
    }

    #[test]
    fn test_commit_status_display() {
        assert_eq!(format!("{}", CommitStatus::Pending), "等待开始");
        assert_eq!(format!("{}", CommitStatus::AnalyzingChanges), "分析变更中");
        assert_eq!(format!("{}", CommitStatus::Completed), "已完成");
    }

    #[test]
    fn test_diff_stats_parsing() {
        let assistant = GitCommitAssistant::new(GitCommitAssistantConfig::default());
        
        // 测试简单的 + 和 - 计数
        let (adds, dels) = assistant.parse_diff_stats("10 +++++++---");
        assert!(adds > 0);
        assert!(dels > 0);
        
        // 测试只有 +
        let (adds2, dels2) = assistant.parse_diff_stats("5 +++++");
        assert_eq!(adds2, 5);
        assert_eq!(dels2, 0);
    }

    #[test]
    fn test_change_categorization() {
        let assistant = GitCommitAssistant::new(GitCommitAssistantConfig::default());
        
        // 测试测试文件识别
        let test_changes = vec![
            ChangeInfo {
                file_path: "src/test.rs".to_string(),
                additions: 10,
                deletions: 0,
                change_type: FileChangeType::Added,
                diff_summary: None,
            }
        ];
        
        let commit_type = assistant.categorize_changes(&test_changes);
        assert_eq!(commit_type, CommitType::Test);
        
        // 测试文档文件识别
        let doc_changes = vec![
            ChangeInfo {
                file_path: "README.md".to_string(),
                additions: 5,
                deletions: 0,
                change_type: FileChangeType::Added,
                diff_summary: None,
            }
        ];
        
        let commit_type2 = assistant.categorize_changes(&doc_changes);
        assert_eq!(commit_type2, CommitType::Docs);
    }

    #[test]
    fn test_summary_generation() {
        let assistant = GitCommitAssistant::new(GitCommitAssistantConfig::default());
        
        let changes = vec![
            ChangeInfo {
                file_path: "src/feature.rs".to_string(),
                additions: 10,
                deletions: 0,
                change_type: FileChangeType::Added,
                diff_summary: None,
            }
        ];
        
        let summary = assistant.generate_summary(CommitType::Feat, &changes).unwrap();
        assert!(summary.starts_with("Add"));
        assert!(summary.len() <= 50);
    }

    #[test]
    fn test_description_generation() {
        let assistant = GitCommitAssistant::new(GitCommitAssistantConfig::default());
        
        let changes = vec![
            ChangeInfo {
                file_path: "src/main.rs".to_string(),
                additions: 10,
                deletions: 5,
                change_type: FileChangeType::Modified,
                diff_summary: None,
            }
        ];
        
        let description = assistant.generate_description(&changes);
        assert!(description.contains("Modify"));
        assert!(description.contains("+10"));
        assert!(description.contains("-5"));
    }

    #[test]
    fn test_conventional_commit_validation() {
        let assistant = GitCommitAssistant::new(GitCommitAssistantConfig::default());
        
        // 测试有效的提交信息
        assert!(assistant.validate_conventional_commit("feat: add new feature").is_ok());
        assert!(assistant.validate_conventional_commit("fix(api): resolve bug").is_ok());
        assert!(assistant.validate_conventional_commit("docs(readme): update installation guide").is_ok());
        
        // 测试无效的提交信息
        assert!(assistant.validate_conventional_commit("invalid message").is_err());
        assert!(assistant.validate_conventional_commit("added new stuff").is_err());
    }

    #[test]
    fn test_commit_message_formatting() {
        let message = CommitMessage::new(
            CommitType::Feat,
            "user authentication".to_string(),
            "Implement user login and registration".to_string(),
            vec!["src/auth.rs".to_string(), "tests/auth_test.rs".to_string()],
        );

        assert!(message.formatted.starts_with("feat: user authentication"));
        assert!(message.formatted.contains("Implement user login and registration"));
        assert!(message.formatted.contains("src/auth.rs"));
        assert!(message.formatted.contains("tests/auth_test.rs"));
    }

    #[test]
    fn test_empty_changes_detection() {
        // 这个测试需要实际的 git 仓库，这里只测试数据结构
        let changes: Vec<ChangeInfo> = vec![];
        assert!(changes.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let config = GitCommitAssistantConfig {
            project_path: "/tmp".to_string(),
            use_ai: false,
            include_file_list: false,
            max_summary_length: 72,
            conventional_commit: false,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: GitCommitAssistantConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.project_path, "/tmp");
        assert!(!deserialized.use_ai);
        assert!(!deserialized.include_file_list);
        assert_eq!(deserialized.max_summary_length, 72);
        assert!(!deserialized.conventional_commit);
    }

    #[test]
    fn test_commit_type_enum_coverage() {
        let types = vec![
            CommitType::Feat,
            CommitType::Fix,
            CommitType::Docs,
            CommitType::Style,
            CommitType::Refactor,
            CommitType::Perf,
            CommitType::Test,
            CommitType::Chore,
        ];

        for t in types {
            let display = format!("{}", t);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_file_change_type_enum_coverage() {
        let types = vec![
            FileChangeType::Added,
            FileChangeType::Modified,
            FileChangeType::Deleted,
            FileChangeType::Renamed,
        ];

        for t in types {
            let display = format!("{}", t);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_commit_status_enum_coverage() {
        let statuses = vec![
            CommitStatus::Pending,
            CommitStatus::AnalyzingChanges,
            CommitStatus::CategorizingChanges,
            CommitStatus::GeneratingMessage,
            CommitStatus::Completed,
            CommitStatus::Failed("test".to_string()),
        ];

        for s in statuses {
            let display = format!("{}", s);
            assert!(!display.is_empty());
        }
    }
}

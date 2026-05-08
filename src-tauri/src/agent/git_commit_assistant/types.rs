//! Git Commit Assistant 类型定义
//!
//! 包含提交类型、变更类型、变更信息、提交信息等核心数据结构

use serde::{Deserialize, Serialize};

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
        let formatted = Self::format_commit_message(
            &commit_type,
            &scope,
            &summary,
            &description,
            &breaking_changes,
            &changed_files,
        );

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
        let formatted = Self::format_commit_message(
            &commit_type,
            &Some(scope.clone()),
            &summary,
            &description,
            &breaking_changes,
            &changed_files,
        );

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

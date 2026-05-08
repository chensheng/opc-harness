//! Git Commit Assistant 提交信息生成器
//!
//! 负责生成提交摘要、详细描述和完整的提交信息

use super::config::GitCommitAssistantConfig;
use super::types::{ChangeInfo, CommitMessage, CommitType, FileChangeType};

/// 提交信息生成器
#[derive(Clone, Debug)]
pub struct MessageGenerator {
    config: GitCommitAssistantConfig,
}

impl MessageGenerator {
    /// 创建新的消息生成器
    pub fn new(config: GitCommitAssistantConfig) -> Self {
        Self { config }
    }

    /// 创建提交信息
    pub fn create_commit_message(
        &self,
        commit_type: CommitType,
        changes: &[ChangeInfo],
    ) -> Result<CommitMessage, String> {
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
        let message = CommitMessage::new(commit_type, summary, description, changed_files);

        Ok(message)
    }

    /// 生成提交摘要
    pub fn generate_summary(
        &self,
        commit_type: CommitType,
        changes: &[ChangeInfo],
    ) -> Result<String, String> {
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
                    .next_back()
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
    pub fn generate_description(&self, changes: &[ChangeInfo]) -> String {
        let mut descriptions = Vec::new();

        for change in changes {
            let change_desc = match change.change_type {
                FileChangeType::Added => format!("Add new file: {}", change.file_path),
                FileChangeType::Deleted => format!("Remove file: {}", change.file_path),
                FileChangeType::Modified => {
                    if change.additions > 0 && change.deletions > 0 {
                        format!(
                            "Modify {}: +{} -{}",
                            change.file_path, change.additions, change.deletions
                        )
                    } else if change.additions > 0 {
                        format!("Add lines to {}: +{}", change.file_path, change.additions)
                    } else {
                        format!(
                            "Remove lines from {}: -{}",
                            change.file_path, change.deletions
                        )
                    }
                }
                FileChangeType::Renamed => format!("Rename file: {}", change.file_path),
            };
            descriptions.push(change_desc);
        }

        descriptions.join("\n")
    }
}

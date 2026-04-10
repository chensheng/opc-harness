//! Git Commit Assistant 状态管理

use serde::{Deserialize, Serialize};

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

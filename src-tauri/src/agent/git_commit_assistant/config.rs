//! Git Commit Assistant 配置

use serde::{Deserialize, Serialize};

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

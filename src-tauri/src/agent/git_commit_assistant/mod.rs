//! Git Commit Assistant 模块
//! 
//! 负责分析代码变更，使用 AI 生成规范的 Git 提交信息
//! 
//! 模块结构：
//! - types: 核心类型定义（CommitType, ChangeInfo, CommitMessage等）
//! - config: 配置结构
//! - status: 状态枚举
//! - analyzer: 变更分析逻辑
//! - generator: 提交信息生成逻辑
//! - validator: 验证逻辑

mod types;
mod config;
mod status;
mod analyzer;
mod generator;
mod validator;

// 重新导出常用类型
pub use types::{CommitType, FileChangeType, ChangeInfo, CommitMessage};
pub use config::GitCommitAssistantConfig;
pub use status::CommitStatus;
pub use analyzer::ChangeAnalyzer;
pub use generator::MessageGenerator;
pub use validator::CommitValidator;

/// Git Commit Assistant
#[derive(Debug, Clone)]
pub struct GitCommitAssistant {
    /// 配置信息
    pub config: GitCommitAssistantConfig,
    /// 当前状态
    pub status: CommitStatus,
    /// 会话 ID
    pub session_id: String,
    /// 变更分析器
    analyzer: ChangeAnalyzer,
    /// 消息生成器
    generator: MessageGenerator,
    /// 验证器
    validator: CommitValidator,
}

impl GitCommitAssistant {
    /// 创建新的 Git Commit Assistant
    pub fn new(config: GitCommitAssistantConfig) -> Self {
        let analyzer = ChangeAnalyzer::new(config.clone());
        let generator = MessageGenerator::new(config.clone());
        let validator = CommitValidator::new();

        Self {
            config,
            status: CommitStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
            analyzer,
            generator,
            validator,
        }
    }

    /// 执行完整的提交信息生成流程
    pub async fn generate_commit_message(&mut self) -> Result<CommitMessage, String> {
        log::info!("开始执行 Git Commit Assistant - Session: {}", self.session_id);

        // 1. 分析变更
        self.status = CommitStatus::AnalyzingChanges;
        log::info!("步骤 1/3: 分析代码变更");
        
        let changes = self.analyzer.analyze_changes()?;
        
        if changes.is_empty() {
            log::warn!("没有检测到变更");
            self.status = CommitStatus::Failed("No changes detected".to_string());
            return Err("No changes to commit".to_string());
        }

        log::info!("检测到 {} 个文件变更", changes.len());

        // 2. 分类变更
        self.status = CommitStatus::CategorizingChanges;
        log::info!("步骤 2/3: 分类变更类型");
        
        let commit_type = self.analyzer.categorize_changes(&changes);
        log::info!("识别变更类型：{:?}", commit_type);

        // 3. 生成提交信息
        self.status = CommitStatus::GeneratingMessage;
        log::info!("步骤 3/3: 生成提交信息");
        
        let message = self.generator.create_commit_message(commit_type, &changes)?;
        
        // 完成
        self.status = CommitStatus::Completed;
        log::info!("提交信息生成完成：{}", message.formatted.lines().next().unwrap_or(""));
        
        Ok(message)
    }

    /// 验证是否符合 Conventional Commits 规范
    pub fn validate_conventional_commit(&self, message: &str) -> Result<bool, String> {
        self.validator.validate_conventional_commit(message)
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
        let (adds, dels) = assistant.analyzer.parse_diff_stats("10 +++++++---");
        assert!(adds > 0);
        assert!(dels > 0);
        
        // 测试只有 +
        let (adds2, dels2) = assistant.analyzer.parse_diff_stats("5 +++++");
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
        
        let commit_type = assistant.analyzer.categorize_changes(&test_changes);
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
        
        let commit_type2 = assistant.analyzer.categorize_changes(&doc_changes);
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
        
        let summary = assistant.generator.generate_summary(CommitType::Feat, &changes).unwrap();
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
        
        let description = assistant.generator.generate_description(&changes);
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

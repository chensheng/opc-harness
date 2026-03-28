//! MR Description Generator 实现
//! 
//! 负责分析合并后的代码变更，生成结构化的 Merge Request 描述文档

use serde::{Deserialize, Serialize};
use crate::agent::code_change_tracker::{ChangeStatistics, ChangeType};
use std::path::{Path, PathBuf};
use tokio::process::Command;

/// MR 描述信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRDescription {
    /// MR 标题
    pub title: String,
    /// MR 描述正文（Markdown 格式）
    pub description: String,
    /// 已实现的 Issue 列表
    pub implemented_issues: Vec<String>,
    /// 变更的文件列表
    pub changed_files: Vec<String>,
    /// 变更统计信息
    pub statistics: ChangeStatistics,
    /// 测试结果摘要
    pub test_results: Option<TestSummary>,
    /// 风险等级评估
    pub risk_assessment: RiskLevel,
    /// 推荐的审查者列表
    pub recommended_reviewers: Vec<String>,
    /// 生成时间
    pub generated_at: String,
}

/// 测试结果摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// 总测试数
    pub total_tests: u32,
    /// 通过数
    pub passed: u32,
    /// 失败数
    pub failed: u32,
    /// 跳过数
    pub skipped: u32,
    /// 代码覆盖率（百分比）
    pub coverage: f64,
    /// 测试报告（文本格式）
    pub test_report: String,
}

/// 风险等级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// 低风险：文档/样式变更
    Low,
    /// 中风险：功能增强/重构
    Medium,
    /// 高风险：核心逻辑变更
    High,
    /// 临界风险：破坏性变更
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "低"),
            RiskLevel::Medium => write!(f, "中"),
            RiskLevel::High => write!(f, "高"),
            RiskLevel::Critical => write!(f, "临界"),
        }
    }
}

/// MR Description Generator 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRDescriptionConfig {
    /// 项目路径
    pub project_path: String,
    /// 目标分支名称
    pub target_branch: String,
    /// 功能分支列表
    pub feature_branches: Vec<String>,
    /// 是否包含测试结果
    pub include_test_results: bool,
    /// 是否推荐审查者
    pub recommend_reviewers: bool,
}

/// MR Description Generator
#[derive(Debug, Clone)]
pub struct MRDescriptionGenerator {
    config: MRDescriptionConfig,
    workspace_root: PathBuf,
}

impl MRDescriptionGenerator {
    /// 创建新的生成器
    pub fn new(project_path: PathBuf) -> Result<Self, String> {
        if !project_path.exists() {
            return Err(format!("Project path does not exist: {:?}", project_path));
        }

        Ok(Self {
            config: MRDescriptionConfig {
                project_path: project_path.to_string_lossy().to_string(),
                target_branch: "main".to_string(),
                feature_branches: vec![],
                include_test_results: true,
                recommend_reviewers: true,
            },
            workspace_root: project_path,
        })
    }

    /// 创建带配置的生成器
    pub fn with_config(config: MRDescriptionConfig) -> Result<Self, String> {
        let workspace_root = PathBuf::from(&config.project_path);
        if !workspace_root.exists() {
            return Err(format!("Project path does not exist: {}", config.project_path));
        }

        Ok(Self {
            config,
            workspace_root,
        })
    }

    /// 生成 MR 描述
    pub async fn generate_description(
        &self,
        feature_branches: &[String],
        target_branch: &str,
    ) -> Result<MRDescription, String> {
        log::info!("Generating MR description for branches: {:?}", feature_branches);

        // 1. 获取所有分支的合并变更
        let changes = self.get_merged_changes(feature_branches, target_branch).await?;
        
        // 2. 计算变更统计
        let statistics = self.calculate_statistics(&changes);
        
        // 3. 提取 Issue 信息
        let issues = self.extract_issues_from_branches(feature_branches).await?;
        
        // 4. 收集测试结果
        let test_results = if self.config.include_test_results {
            self.collect_test_results().await.ok()
        } else {
            None
        };
        
        // 5. 评估风险等级
        let risk_level = self.assess_risk(&changes, &statistics);
        
        // 6. 推荐审查者
        let reviewers = if self.config.recommend_reviewers {
            self.recommend_reviewers(&changes)
        } else {
            vec![]
        };

        // 7. 生成变更文件列表
        let changed_files: Vec<String> = changes.iter()
            .map(|c| c.file_path.clone())
            .collect();

        // 8. 生成 MR 标题
        let title = self.generate_title(&issues, feature_branches.len());

        // 9. 生成 Markdown 描述
        let mr = MRDescription {
            title: title.clone(),
            description: self.format_markdown(&MRDescription {
                title,
                description: String::new(),
                implemented_issues: issues.clone(),
                changed_files: changed_files.clone(),
                statistics: statistics.clone(),
                test_results: test_results.clone(),
                risk_assessment: risk_level.clone(),
                recommended_reviewers: reviewers.clone(),
                generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            }),
            implemented_issues: issues,
            changed_files,
            statistics,
            test_results,
            risk_assessment: risk_level,
            recommended_reviewers: reviewers,
            generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        Ok(mr)
    }

    /// 获取合并后的变更信息
    async fn get_merged_changes(
        &self,
        feature_branches: &[String],
        target_branch: &str,
    ) -> Result<Vec<FileChangeInfo>, String> {
        let mut changes = Vec::new();

        // 对每个功能分支执行 git diff
        for branch in feature_branches {
            let output = Command::new("git")
                .args(["diff", "--numstat", &format!("{}...{}", target_branch, branch)])
                .current_dir(&self.workspace_root)
                .output()
                .await
                .map_err(|e| format!("Failed to execute git diff: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Git diff failed: {}", stderr));
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            changes.extend(self.parse_git_numstat(&stdout)?);
        }

        Ok(changes)
    }

    /// 解析 git diff --numstat 输出
    fn parse_git_numstat(&self, output: &str) -> Result<Vec<FileChangeInfo>, String> {
        let mut changes = Vec::new();

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                log::warn!("Invalid numstat line: {}", line);
                continue;
            }

            let additions = parts[0].parse::<usize>().unwrap_or(0);
            let deletions = parts[1].parse::<usize>().unwrap_or(0);
            let file_path = parts[2].to_string();

            // 处理二进制文件（- 表示二进制）
            let change_type = if additions == 0 && deletions == 0 {
                FileChangeType::Binary
            } else if additions > 0 && deletions == 0 {
                FileChangeType::Added
            } else if additions == 0 && deletions > 0 {
                FileChangeType::Deleted
            } else {
                FileChangeType::Modified
            };

            changes.push(FileChangeInfo {
                file_path,
                additions,
                deletions,
                change_type,
            });
        }

        Ok(changes)
    }

    /// 从分支名称提取 Issue 信息
    async fn extract_issues_from_branches(
        &self,
        feature_branches: &[String],
    ) -> Result<Vec<String>, String> {
        let mut issues = Vec::new();

        for branch in feature_branches {
            // 尝试从分支名提取 issue ID（如：feature/VC-034-description）
            if let Some(issue_id) = self.extract_issue_id(branch) {
                if !issues.contains(&issue_id) {
                    issues.push(issue_id);
                }
            }
        }

        Ok(issues)
    }

    /// 从分支名提取 Issue ID
    fn extract_issue_id(&self, branch_name: &str) -> Option<String> {
        // 支持多种分支命名格式：
        // - feature/VC-034-description
        // - VC-034-description
        // - feat/VC-034
        
        let parts: Vec<&str> = branch_name.split('/').collect();
        
        for part in parts {
            // 匹配 VC-XXX 或 INFRA-XXX 等模式
            if part.contains('-') {
                let subparts: Vec<&str> = part.split('-').collect();
                if subparts.len() >= 2 && subparts[0].chars().all(|c| c.is_alphabetic()) && subparts[1].chars().all(|c| c.is_numeric()) {
                    return Some(format!("{}-{}", subparts[0], subparts[1]));
                }
            }
        }

        None
    }

    /// 收集测试结果
    async fn collect_test_results(&self) -> Result<TestSummary, String> {
        // 运行 cargo test --json 获取测试结果
        let output = Command::new("cargo")
            .args(&["test", "--json"])
            .current_dir(&self.workspace_root)
            .output()
            .await
            .map_err(|e| format!("Failed to run cargo test: {}", e))?;

        // 解析 JSON 输出（简化版本）
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // 简单统计（实际应该解析 JSON）
        let total = stdout.matches("\"event\": \"ok\"").count() as u32;
        let failed = stdout.matches("\"event\": \"failed\"").count() as u32;
        
        Ok(TestSummary {
            total_tests: total + failed,
            passed: total,
            failed,
            skipped: 0,
            coverage: 0.0, // 需要额外的覆盖率工具
            test_report: stdout.to_string(),
        })
    }

    /// 评估风险等级
    fn assess_risk(&self, changes: &[FileChangeInfo], stats: &ChangeStatistics) -> RiskLevel {
        // 风险评估逻辑：
        // 1. 检查是否有核心文件变更（src/agent/, src/main.rs 等）
        // 2. 检查变更规模（>500 行变更为高风险）
        // 3. 检查是否有破坏性变更（删除文件 > 新增文件）

        let has_core_changes = changes.iter().any(|c| {
            c.file_path.contains("src/agent/") ||
            c.file_path.contains("src/main.rs") ||
            c.file_path.contains("src/commands/")
        });

        let large_change = stats.total_additions > 500 || stats.total_deletions > 300;
        let destructive_change = stats.total_deletions > stats.total_additions * 2;

        if has_core_changes && large_change {
            RiskLevel::Critical
        } else if has_core_changes || large_change {
            RiskLevel::High
        } else if destructive_change {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// 推荐审查者
    fn recommend_reviewers(&self, changes: &[FileChangeInfo]) -> Vec<String> {
        // 简化的推荐逻辑：
        // 1. 根据变更的文件类型推荐对应的负责人
        // 2. 如果变更涉及多个模块，推荐多个审查者
        
        let mut reviewers = Vec::new();

        // 检查是否有 Agent 相关变更
        if changes.iter().any(|c| c.file_path.contains("agent/")) {
            reviewers.push("backend-lead".to_string());
        }

        // 检查是否有前端变更
        if changes.iter().any(|c| c.file_path.contains("src/components/") || c.file_path.contains("src/pages/")) {
            reviewers.push("frontend-lead".to_string());
        }

        // 检查是否有架构变更
        if changes.iter().any(|c| c.file_path.contains("src-tauri/src/main.rs") || c.file_path.contains("architecture")) {
            reviewers.push("tech-lead".to_string());
        }

        // 如果没有特定推荐，返回默认审查者
        if reviewers.is_empty() {
            reviewers.push("team-lead".to_string());
        }

        reviewers
    }

    /// 生成 MR 标题
    fn generate_title(&self, issues: &[String], branch_count: usize) -> String {
        if issues.is_empty() {
            format!("Merge {} feature branches", branch_count)
        } else if issues.len() == 1 {
            format!("Implement {}", issues[0])
        } else {
            format!("Implement multiple features ({})", issues.join(", "))
        }
    }

    /// 格式化为 Markdown
    fn format_markdown(&self, mr: &MRDescription) -> String {
        let mut md = String::new();

        // 标题
        md.push_str(&format!("# {}\n\n", mr.title));

        // 📋 变更概述
        md.push_str("## 📋 变更概述\n\n");
        if !mr.implemented_issues.is_empty() {
            md.push_str(&format!("**实现的 Issues**: {}\n\n", mr.implemented_issues.join(", ")));
        }
        md.push_str(&format!("**风险等级**: {}\n", mr.risk_assessment));
        md.push_str(&format!("**生成时间**: {}\n\n", mr.generated_at));

        // 📊 变更统计
        md.push_str("## 📊 变更统计\n\n");
        md.push_str(&format!("- 总变更文件数：{}\n", mr.statistics.total_files_changed));
        md.push_str(&format!("- 总新增行数：{}\n", mr.statistics.total_additions));
        md.push_str(&format!("- 总删除行数：{}\n", mr.statistics.total_deletions));
        md.push_str(&format!("- 净变更：{}\n\n", mr.statistics.net_change));

        // 🧪 测试结果
        if let Some(ref test) = mr.test_results {
            md.push_str("## 🧪 测试结果\n\n");
            md.push_str(&format!("- 总测试数：{}\n", test.total_tests));
            md.push_str(&format!("- ✅ 通过：{}\n", test.passed));
            md.push_str(&format!("- ❌ 失败：{}\n", test.failed));
            if test.skipped > 0 {
                md.push_str(&format!("- ⏭️ 跳过：{}\n", test.skipped));
            }
            md.push_str(&format!("- 代码覆盖率：{:.1}%\n\n", test.coverage));
            
            if test.failed > 0 {
                md.push_str("**⚠️ 警告**: 有测试失败，请查看测试报告。\n\n");
            }
        }

        // 📝 变更文件列表
        md.push_str("## 📝 变更文件列表\n\n");
        md.push_str("``diff\n");
        for file in &mr.changed_files {
            md.push_str(&format!("+ {}\n", file));
        }
        md.push_str("```\n\n");

        // 👥 推荐审查者
        md.push_str("## 👥 推荐审查者\n\n");
        for reviewer in &mr.recommended_reviewers {
            md.push_str(&format!("- @{}\n", reviewer));
        }
        md.push('\n');

        // ⚠️ 风险提示
        md.push_str("## ⚠️ 风险提示\n\n");
        match mr.risk_assessment {
            RiskLevel::Low => {
                md.push_str("✅ **低风险**: 本次变更主要是文档或样式调整，影响范围较小。\n");
            }
            RiskLevel::Medium => {
                md.push_str("⚠️ **中风险**: 本次变更涉及功能增强或重构，建议进行详细审查。\n");
            }
            RiskLevel::High => {
                md.push_str("🔴 **高风险**: 本次变更涉及核心逻辑修改，需要仔细审查并进行充分测试。\n");
            }
            RiskLevel::Critical => {
                md.push_str("🚨 **临界风险**: 本次变更可能包含破坏性修改，强烈建议进行全面审查和回归测试。\n");
            }
        }

        md
    }

    /// 计算变更统计
    fn calculate_statistics(&self, changes: &[FileChangeInfo]) -> ChangeStatistics {
        let total_files = changes.len() as u32;
        let total_adds: usize = changes.iter().map(|c| c.additions).sum();
        let total_dels: usize = changes.iter().map(|c| c.deletions).sum();

        ChangeStatistics {
            total_files_changed: total_files,
            total_additions: total_adds as u32,
            total_deletions: total_dels as u32,
            net_change: (total_adds as i32) - (total_dels as i32),
            files_by_type: std::collections::HashMap::new(),
        }
    }
}

/// 文件变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeInfo {
    /// 文件路径
    pub file_path: String,
    /// 新增行数
    pub additions: usize,
    /// 删除行数
    pub deletions: usize,
    /// 变更类型
    pub change_type: FileChangeType,
}

/// 文件变更类型（扩展版）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileChangeType {
    /// 新增
    Added,
    /// 修改
    Modified,
    /// 删除
    Deleted,
    /// 二进制文件
    Binary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir);
        assert!(generator.is_ok());
    }

    #[tokio::test]
    async fn test_extract_issue_id() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        assert_eq!(generator.extract_issue_id("feature/VC-034-description"), Some("VC-034".to_string()));
        assert_eq!(generator.extract_issue_id("VC-035-mr-generator"), Some("VC-035".to_string()));
        assert_eq!(generator.extract_issue_id("feat/INFRA-001-update"), Some("INFRA-001".to_string()));
        assert_eq!(generator.extract_issue_id("invalid-branch"), None);
    }

    #[test]
    fn test_parse_git_numstat() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let output = "10\t5\tsrc/file1.rs\n0\t0\tsrc/image.png\n15\t0\tsrc/file2.rs\n";
        let changes = generator.parse_git_numstat(output).unwrap();

        assert_eq!(changes.len(), 3);
        assert_eq!(changes[0].file_path, "src/file1.rs");
        assert_eq!(changes[0].additions, 10);
        assert_eq!(changes[0].deletions, 5);
        assert_eq!(changes[1].change_type, FileChangeType::Binary);
        assert_eq!(changes[2].change_type, FileChangeType::Added);
    }

    #[test]
    fn test_risk_assessment_low() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let changes = vec![
            FileChangeInfo {
                file_path: "docs/readme.md".to_string(),
                additions: 5,
                deletions: 2,
                change_type: FileChangeType::Modified,
            },
        ];

        let stats = generator.calculate_statistics(&changes);
        let risk = generator.assess_risk(&changes, &stats);
        assert_eq!(risk, RiskLevel::Low);
    }

    #[test]
    fn test_risk_assessment_high() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let changes = vec![
            FileChangeInfo {
                file_path: "src-tauri/src/agent/coding_agent.rs".to_string(),
                additions: 600,
                deletions: 100,
                change_type: FileChangeType::Modified,
            },
        ];

        let stats = generator.calculate_statistics(&changes);
        let risk = generator.assess_risk(&changes, &stats);
        // 核心文件 + 大规模变更 = Critical
        assert_eq!(risk, RiskLevel::Critical);
    }

    #[test]
    fn test_recommend_reviewers() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let changes = vec![
            FileChangeInfo {
                file_path: "src-tauri/src/agent/new_agent.rs".to_string(),
                additions: 100,
                deletions: 0,
                change_type: FileChangeType::Added,
            },
        ];

        let reviewers = generator.recommend_reviewers(&changes);
        assert!(reviewers.contains(&"backend-lead".to_string()));
    }

    #[test]
    fn test_generate_title() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        assert_eq!(generator.generate_title(&vec!["VC-034".to_string()], 1), "Implement VC-034");
        assert_eq!(generator.generate_title(&vec!["VC-034".to_string(), "VC-035".to_string()], 2), "Implement multiple features (VC-034, VC-035)");
        assert_eq!(generator.generate_title(&vec![], 3), "Merge 3 feature branches");
    }

    #[test]
    fn test_format_markdown() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let mr = MRDescription {
            title: "Test MR".to_string(),
            description: String::new(),
            implemented_issues: vec!["VC-034".to_string()],
            changed_files: vec!["src/file.rs".to_string()],
            statistics: ChangeStatistics {
                total_files_changed: 1,
                total_additions: 10,
                total_deletions: 5,
                net_change: 5,
                files_by_type: std::collections::HashMap::new(),
            },
            test_results: None,
            risk_assessment: RiskLevel::Low,
            recommended_reviewers: vec!["backend-lead".to_string()],
            generated_at: "2026-03-28 13:00:00".to_string(),
        };

        let markdown = generator.format_markdown(&mr);
        assert!(markdown.contains("# Test MR"));
        assert!(markdown.contains("VC-034"));
        assert!(markdown.contains("低风险"));
    }

    #[test]
    fn test_calculate_statistics() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let changes = vec![
            FileChangeInfo {
                file_path: "file1.rs".to_string(),
                additions: 10,
                deletions: 5,
                change_type: FileChangeType::Modified,
            },
            FileChangeInfo {
                file_path: "file2.rs".to_string(),
                additions: 20,
                deletions: 0,
                change_type: FileChangeType::Added,
            },
        ];

        let stats = generator.calculate_statistics(&changes);
        assert_eq!(stats.total_files_changed, 2);
        assert_eq!(stats.total_additions, 30);
        assert_eq!(stats.total_deletions, 5);
        assert_eq!(stats.net_change, 25);
    }

    #[test]
    fn test_with_config() {
        let temp_dir = std::env::temp_dir();
        let config = MRDescriptionConfig {
            project_path: temp_dir.to_string_lossy().to_string(),
            target_branch: "main".to_string(),
            feature_branches: vec!["feature/test".to_string()],
            include_test_results: false,
            recommend_reviewers: false,
        };

        let generator = MRDescriptionGenerator::with_config(config);
        assert!(generator.is_ok());
    }

    #[tokio::test]
    async fn test_empty_branches() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let result = generator.generate_description(&vec![], "main").await;
        assert!(result.is_ok());
        
        let mr = result.unwrap();
        assert_eq!(mr.implemented_issues.len(), 0);
        assert!(mr.title.contains("Merge 0 feature branches"));
    }

    #[test]
    fn test_binary_file_detection() {
        let temp_dir = std::env::temp_dir();
        let generator = MRDescriptionGenerator::new(temp_dir).unwrap();

        let output = "0\t0\timage.png\n10\t5\tfile.rs\n";
        let changes = generator.parse_git_numstat(output).unwrap();

        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].change_type, FileChangeType::Binary);
        assert_eq!(changes[1].change_type, FileChangeType::Modified);
    }
}

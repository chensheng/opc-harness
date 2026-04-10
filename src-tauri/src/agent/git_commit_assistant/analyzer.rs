//! Git Commit Assistant 变更分析器
//! 
//! 负责分析代码变更、解析 diff 统计、分类变更类型

use std::process::Command;
use super::types::{ChangeInfo, FileChangeType, CommitType};
use super::config::GitCommitAssistantConfig;

/// 变更分析器
#[derive(Clone, Debug)]
pub struct ChangeAnalyzer {
    config: GitCommitAssistantConfig,
}

impl ChangeAnalyzer {
    /// 创建新的变更分析器
    pub fn new(config: GitCommitAssistantConfig) -> Self {
        Self { config }
    }

    /// 分析代码变更
    pub fn analyze_changes(&self) -> Result<Vec<ChangeInfo>, String> {
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
    pub fn parse_diff_stats(&self, stats: &str) -> (usize, usize) {
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
    pub fn categorize_changes(&self, changes: &[ChangeInfo]) -> CommitType {
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
}

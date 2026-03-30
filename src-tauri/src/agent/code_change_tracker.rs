//! Code Change Tracker Agent 实现
//! 
//! 负责检测工作区的文件变更，生成结构化的变更摘要和影响分析

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 变更类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    /// 新增文件
    Added,
    /// 修改文件
    Modified,
    /// 删除文件
    Deleted,
    /// 重命名文件
    Renamed,
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeType::Added => write!(f, "A"),
            ChangeType::Modified => write!(f, "M"),
            ChangeType::Deleted => write!(f, "D"),
            ChangeType::Renamed => write!(f, "R"),
        }
    }
}

impl std::str::FromStr for ChangeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "added" => Ok(ChangeType::Added),
            "M" | "modified" => Ok(ChangeType::Modified),
            "D" | "deleted" => Ok(ChangeType::Deleted),
            "R" | "renamed" => Ok(ChangeType::Renamed),
            _ => Err(format!("Invalid change type: {}", s)),
        }
    }
}

/// 单个文件的变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// 文件路径
    pub file_path: String,
    /// 变更类型
    pub change_type: ChangeType,
    /// 新增行数
    pub additions: u32,
    /// 删除行数
    pub deletions: u32,
    /// Git diff 输出
    pub diff: String,
    /// 受影响的依赖文件
    pub impacted_files: Vec<String>,
}

/// 变更统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeStatistics {
    /// 总变更文件数
    pub total_files_changed: u32,
    /// 总新增行数
    pub total_additions: u32,
    /// 总删除行数
    pub total_deletions: u32,
    /// 净变更（新增 - 删除）
    pub net_change: i32,
    /// 按变更类型分类的文件数
    pub files_by_type: HashMap<String, u32>,
}

/// 变更摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSummary {
    /// 统计信息
    pub statistics: ChangeStatistics,
    /// 所有文件变更
    pub changes: Vec<FileChange>,
    /// 所有受影响的文件
    pub impacted_files: Vec<String>,
    /// 生成时间
    pub generated_at: String,
}

/// Code Change Tracker Agent
pub struct CodeChangeTracker {
    workspace_root: PathBuf,
}

impl CodeChangeTracker {
    /// 创建新的变更追踪器
    pub fn new(workspace_root: PathBuf) -> Result<Self, String> {
        if !workspace_root.exists() {
            return Err(format!("Workspace does not exist: {:?}", workspace_root));
        }
        
        Ok(Self { workspace_root })
    }

    /// 检测工作区的所有变更
    pub async fn detect_changes(&self) -> Result<Vec<FileChange>, String> {
        log::info!("Detecting workspace changes...");
        
        // 获取 git status 输出
        let status_output = self.run_git_status()?;
        
        // 解析变更文件列表
        let changed_files = self.parse_git_status(&status_output)?;
        
        // 为每个文件获取详细的 diff 信息
        let mut changes = Vec::new();
        for (file_path, change_type) in changed_files {
            match self.get_file_diff(&file_path).await {
                Ok((additions, deletions, diff)) => {
                    let mut file_change = FileChange {
                        file_path,
                        change_type,
                        additions,
                        deletions,
                        diff,
                        impacted_files: vec![],
                    };
                    
                    // 分析依赖文件
                    file_change.impacted_files = self.analyze_imports(&file_change).await?;
                    changes.push(file_change);
                }
                Err(e) => {
                    log::warn!("Failed to get diff for {}: {}", file_path, e);
                }
            }
        }
        
        log::info!("Detected {} file changes", changes.len());
        Ok(changes)
    }

    /// 获取单个文件的 diff
    pub async fn get_file_diff(&self, file_path: &str) -> Result<(u32, u32, String), String> {
        let full_path = self.workspace_root.join(file_path);
        
        if !full_path.exists() {
            // 文件已删除
            return Ok((0, 0, format!("File deleted: {}", file_path)));
        }
        
        // 运行 git diff
        let output = Command::new("git")
            .args(["diff", "HEAD", "--", file_path])
            .current_dir(&self.workspace_root)
            .output()
            .map_err(|e| format!("Failed to run git diff: {}", e))?;
        
        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 计算新增和删除行数
        let additions = diff.lines().filter(|line| line.starts_with('+') && !line.starts_with("+++")).count() as u32;
        let deletions = diff.lines().filter(|line| line.starts_with('-') && !line.starts_with("---")).count() as u32;
        
        Ok((additions, deletions, diff))
    }

    /// 分析变更影响（识别依赖文件）
    pub async fn analyze_impact(&self, changes: &[FileChange]) -> Result<Vec<String>, String> {
        let mut impacted = Vec::new();
        
        for change in changes {
            // 添加直接依赖
            for file in &change.impacted_files {
                if !impacted.contains(file) {
                    impacted.push(file.clone());
                }
            }
        }
        
        Ok(impacted)
    }

    /// 生成变更摘要
    pub async fn generate_summary(&self) -> Result<ChangeSummary, String> {
        let changes = self.detect_changes().await?;
        let statistics = self.calculate_statistics(&changes);
        let impacted_files = self.analyze_impact(&changes).await?;
        let generated_at = chrono::Utc::now().to_rfc3339();
        
        Ok(ChangeSummary {
            statistics,
            changes,
            impacted_files,
            generated_at,
        })
    }

    /// 计算变更统计
    pub fn calculate_statistics(&self, changes: &[FileChange]) -> ChangeStatistics {
        let total_files_changed = changes.len() as u32;
        let total_additions: u32 = changes.iter().map(|c| c.additions).sum();
        let total_deletions: u32 = changes.iter().map(|c| c.deletions).sum();
        let net_change = total_additions as i32 - total_deletions as i32;
        
        let mut files_by_type: HashMap<String, u32> = HashMap::new();
        for change in changes {
            let type_key = change.change_type.to_string();
            *files_by_type.entry(type_key).or_insert(0) += 1;
        }
        
        ChangeStatistics {
            total_files_changed,
            total_additions,
            total_deletions,
            net_change,
            files_by_type,
        }
    }

    /// 运行 git status 命令
    fn run_git_status(&self) -> Result<String, String> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&self.workspace_root)
            .output()
            .map_err(|e| format!("Failed to run git status: {}", e))?;
        
        if !output.status.success() {
            return Err("Not a git repository or git command failed".to_string());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// 解析 git status 输出
    fn parse_git_status(&self, status: &str) -> Result<Vec<(String, ChangeType)>, String> {
        let mut changes = Vec::new();
        
        for line in status.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            // git status --porcelain 格式：XY filename
            let chars: Vec<char> = line.chars().collect();
            if chars.len() < 4 {
                continue;
            }
            
            let x = chars[0];
            let y = chars[1];
            let file_path = line[3..].to_string();
            
            // 根据状态字符判断变更类型
            let change_type = match (x, y) {
                ('A', _) | (_, 'A') => ChangeType::Added,
                ('D', _) | (_, 'D') => ChangeType::Deleted,
                ('M', _) | (_, 'M') | ('R', _) | (_, 'R') => ChangeType::Modified,
                ('R', 'R') => ChangeType::Renamed,
                ('?', _) => ChangeType::Added,  // Untracked files
                _ => ChangeType::Modified,
            };
            
            changes.push((file_path, change_type));
        }
        
        Ok(changes)
    }

    /// 分析文件的 import/require 语句，识别依赖
    async fn analyze_imports(&self, change: &FileChange) -> Result<Vec<String>, String> {
        let file_path = self.workspace_root.join(&change.file_path);
        
        if !file_path.exists() {
            return Ok(vec![]);
        }
        
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let imports = match self.get_file_extension(&change.file_path) {
            Some("ts") | Some("tsx") | Some("js") | Some("jsx") => {
                self.extract_typescript_imports(&content)
            }
            Some("rs") => {
                self.extract_rust_imports(&content)
            }
            _ => vec![],
        };
        
        Ok(imports)
    }

    /// 提取 TypeScript/JavaScript import 语句
    fn extract_typescript_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // 匹配 import ... from '...'
            if line.starts_with("import ") && line.contains(" from ") {
                if let Some(path) = self.extract_import_path(line) {
                    imports.push(path);
                }
            }
            // 匹配 require('...')
            else if line.contains("require(") {
                if let Some(path) = self.extract_require_path(line) {
                    imports.push(path);
                }
            }
        }
        
        imports
    }

    /// 提取 Rust use/mod 语句
    fn extract_rust_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // 匹配 use crate::...
            if line.starts_with("use ") {
                if let Some(path) = self.extract_use_path(line) {
                    imports.push(path);
                }
            }
            // 匹配 mod ...
            else if line.starts_with("mod ") {
                if let Some(module_name) = line.strip_prefix("mod ") {
                    let module_name = module_name.trim_end_matches(';');
                    imports.push(format!("{}.rs", module_name.trim()));
                }
            }
        }
        
        imports
    }

    /// 从 import 语句提取路径
    fn extract_import_path(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split(" from ").collect();
        if parts.len() != 2 {
            return None;
        }
        
        let path_part = parts[1].trim();
        // 移除引号和分号
        let path = path_part.trim_matches(|c| c == '\'' || c == '"' || c == ';');
        
        // 只关心相对路径
        if path.starts_with("./") || path.starts_with("../") || path.starts_with("@/") {
            Some(path.to_string())
        } else {
            None
        }
    }

    /// 从 require 语句提取路径
    fn extract_require_path(&self, line: &str) -> Option<String> {
        let start = line.find("require(")? + 8;
        let end = line[start..].find(')')? + start;
        let path = line[start..end].trim_matches(|c| c == '\'' || c == '"');
        
        if path.starts_with("./") || path.starts_with("../") || path.starts_with("@/") {
            Some(path.to_string())
        } else {
            None
        }
    }

    /// 从 use 语句提取路径
    fn extract_use_path(&self, line: &str) -> Option<String> {
        let without_use = line.strip_prefix("use ")?;
        let path = without_use.trim_end_matches(';').trim();
        Some(path.to_string())
    }

    /// 获取文件扩展名
    fn get_file_extension<'a>(&self, file_path: &'a str) -> Option<&'a str> {
        Path::new(file_path).extension().and_then(|ext| ext.to_str())
    }
}

// VC-034: Tauri Commands
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    /// 创建临时的 Git 仓库用于测试
    fn setup_git_repo() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path().to_path_buf();
        
        // 初始化 git 仓库
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to init git repo");
        
        // 配置 git 用户
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to config git");
        
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to config git");
        
        (temp_dir, repo_path)
    }

    /// 创建测试文件
    fn create_test_file(dir: &Path, path: &str, content: &str) -> PathBuf {
        let file_path = dir.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    /// 提交文件到 git
    fn commit_file(repo_path: &Path, file_path: &Path, message: &str) {
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo_path)
            .output()
            .expect("Failed to add file");
        
        Command::new("git")
            .args(["commit", "-m", message])
            .current_dir(repo_path)
            .output()
            .expect("Failed to commit");
    }

    #[test]
    fn test_tracker_creation() {
        let (_temp_dir, repo_path) = setup_git_repo();
        let tracker = CodeChangeTracker::new(repo_path.clone());
        assert!(tracker.is_ok());
    }

    #[test]
    fn test_tracker_nonexistent_directory() {
        let non_existent_path = PathBuf::from("/non/existent/path");
        let tracker = CodeChangeTracker::new(non_existent_path);
        assert!(tracker.is_err());
    }

    #[test]
    fn test_detect_added_files() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 先创建一个初始提交
        create_test_file(&repo_path, "README.md", "# Test");
        commit_file(&repo_path, &repo_path.join("README.md"), "Initial commit");
        
        // 创建新文件
        create_test_file(&repo_path, "new_file.ts", "export const test = 1;");
        
        let tracker = CodeChangeTracker::new(repo_path.clone()).unwrap();
        let changes = tokio_test::block_on(tracker.detect_changes()).unwrap();
        
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| c.file_path == "new_file.ts"));
    }

    #[test]
    fn test_detect_modified_files() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 创建并提交文件
        let file_path = create_test_file(&repo_path, "test.ts", "export const a = 1;");
        commit_file(&repo_path, &file_path, "Add test file");
        
        // 修改文件
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"export const a = 1;\nexport const b = 2;").unwrap();
        
        let tracker = CodeChangeTracker::new(repo_path.clone()).unwrap();
        let changes = tokio_test::block_on(tracker.detect_changes()).unwrap();
        
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| c.file_path == "test.ts" && c.change_type == ChangeType::Modified));
    }

    #[test]
    fn test_detect_deleted_files() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 创建并提交文件
        let file_path = create_test_file(&repo_path, "to_delete.ts", "export const x = 1;");
        commit_file(&repo_path, &file_path, "Add file");
        
        // 删除文件
        fs::remove_file(&file_path).unwrap();
        
        let tracker = CodeChangeTracker::new(repo_path.clone()).unwrap();
        let changes = tokio_test::block_on(tracker.detect_changes()).unwrap();
        
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| c.file_path == "to_delete.ts" && c.change_type == ChangeType::Deleted));
    }

    #[test]
    fn test_parse_git_diff() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 创建并提交文件
        let file_path = create_test_file(&repo_path, "diff_test.ts", "line 1\nline 2\nline 3\n");
        commit_file(&repo_path, &file_path, "Add file");
        
        // 修改文件
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"line 1\nmodified line 2\nline 3\nline 4\n").unwrap();
        
        let tracker = CodeChangeTracker::new(repo_path.clone()).unwrap();
        let (additions, deletions, diff) = tokio_test::block_on(tracker.get_file_diff("diff_test.ts")).unwrap();
        
        assert!(additions >= 1);
        assert!(deletions >= 1);
        assert!(diff.contains("modified line 2"));
    }

    #[test]
    fn test_calculate_statistics() {
        let changes = vec![
            FileChange {
                file_path: "file1.ts".to_string(),
                change_type: ChangeType::Added,
                additions: 10,
                deletions: 0,
                diff: "".to_string(),
                impacted_files: vec![],
            },
            FileChange {
                file_path: "file2.ts".to_string(),
                change_type: ChangeType::Modified,
                additions: 5,
                deletions: 3,
                diff: "".to_string(),
                impacted_files: vec![],
            },
        ];
        
        let (_temp_dir, repo_path) = setup_git_repo();
        let tracker = CodeChangeTracker::new(repo_path).unwrap();
        let stats = tracker.calculate_statistics(&changes);
        
        assert_eq!(stats.total_files_changed, 2);
        assert_eq!(stats.total_additions, 15);
        assert_eq!(stats.total_deletions, 3);
        assert_eq!(stats.net_change, 12);
        assert_eq!(*stats.files_by_type.get("A").unwrap_or(&0), 1);
        assert_eq!(*stats.files_by_type.get("M").unwrap_or(&0), 1);
    }

    #[test]
    fn test_extract_typescript_imports() {
        let content = r#"
import { foo } from './utils';
import bar from '../helpers/bar';
const baz = require('./baz');
import React from 'react';
"#;
        
        let tracker = CodeChangeTracker::new(PathBuf::from(".")).unwrap();
        let imports = tracker.extract_typescript_imports(content);
        
        assert!(imports.contains(&"./utils".to_string()));
        assert!(imports.contains(&"../helpers/bar".to_string()));
        assert!(imports.contains(&"./baz".to_string()));
        assert!(!imports.contains(&"react".to_string())); // 外部包不收集
    }

    #[test]
    fn test_extract_rust_imports() {
        let content = r#"
use crate::utils::helper;
use crate::models::User;
mod database;
use std::collections::HashMap;
"#;
        
        let tracker = CodeChangeTracker::new(PathBuf::from(".")).unwrap();
        let imports = tracker.extract_rust_imports(content);
        
        assert!(imports.contains(&"crate::utils::helper".to_string()));
        assert!(imports.contains(&"crate::models::User".to_string()));
        assert!(imports.contains(&"database.rs".to_string()));
    }

    #[test]
    #[ignore = "Requires git repository setup"]
    fn test_generate_summary() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 创建初始提交
        create_test_file(&repo_path, "README.md", "# Test");
        commit_file(&repo_path, &repo_path.join("README.md"), "Initial commit");
        
        // 创建多个文件
        create_test_file(&repo_path, "file1.ts", "export const a = 1;");
        create_test_file(&repo_path, "file2.ts", "export const b = 2;");
        
        let tracker = CodeChangeTracker::new(repo_path).unwrap();
        let summary = tokio_test::block_on(tracker.generate_summary()).unwrap();
        
        assert!(summary.statistics.total_files_changed >= 2);
        assert_eq!(summary.changes.len(), summary.statistics.total_files_changed as usize);
        assert!(!summary.generated_at.is_empty());
    }

    #[test]
    fn test_empty_workspace() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        // 创建初始提交但不再修改
        create_test_file(&repo_path, "README.md", "# Test");
        commit_file(&repo_path, &repo_path.join("README.md"), "Initial commit");
        
        let tracker = CodeChangeTracker::new(repo_path).unwrap();
        let changes = tokio_test::block_on(tracker.detect_changes()).unwrap();
        
        assert!(changes.is_empty());
    }

    #[tokio::test]
    async fn test_analyze_impact() {
        let (_temp_dir, repo_path) = setup_git_repo();
        
        let changes = vec![
            FileChange {
                file_path: "utils.ts".to_string(),
                change_type: ChangeType::Modified,
                additions: 5,
                deletions: 2,
                diff: "".to_string(),
                impacted_files: vec!["helper.ts".to_string(), "index.ts".to_string()],
            },
        ];
        
        let tracker = CodeChangeTracker::new(repo_path).unwrap();
        let impacted = tracker.analyze_impact(&changes).await.unwrap();
        
        assert_eq!(impacted.len(), 2);
        assert!(impacted.contains(&"helper.ts".to_string()));
        assert!(impacted.contains(&"index.ts".to_string()));
    }

    #[test]
    fn test_change_type_from_str() {
        assert_eq!("A".parse::<ChangeType>().unwrap(), ChangeType::Added);
        assert_eq!("added".parse::<ChangeType>().unwrap(), ChangeType::Added);
        assert_eq!("M".parse::<ChangeType>().unwrap(), ChangeType::Modified);
        assert_eq!("D".parse::<ChangeType>().unwrap(), ChangeType::Deleted);
        assert_eq!("R".parse::<ChangeType>().unwrap(), ChangeType::Renamed);
        
        assert!("X".parse::<ChangeType>().is_err());
    }

    #[test]
    fn test_change_type_display() {
        assert_eq!(ChangeType::Added.to_string(), "A");
        assert_eq!(ChangeType::Modified.to_string(), "M");
        assert_eq!(ChangeType::Deleted.to_string(), "D");
        assert_eq!(ChangeType::Renamed.to_string(), "R");
    }
}

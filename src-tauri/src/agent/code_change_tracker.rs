//! Code Change Tracker Agent 实现
//! 
//! 负责检测工作区的文件变更，生成结构化的变更摘要和影响分析

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{PathBuf};
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
                    let file_change = FileChange {
                        file_path,
                        change_type,
                        additions,
                        deletions,
                        diff,
                        impacted_files: vec![],
                    };
                    
                    // 分析依赖文件
                    // TODO: 实现 analyze_imports 方法
                    // file_change.impacted_files = self.analyze_imports(&file_change).await?;
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
                ('?', _) => ChangeType::Added,  // Untracked files
                _ => ChangeType::Modified,
            };
            changes.push((file_path, change_type));
        }
        
        Ok(changes)
    }
}

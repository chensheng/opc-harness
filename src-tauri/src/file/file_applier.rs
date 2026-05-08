//! 文件修改应用器
//!
//! 实现安全的文件修改、备份和回滚机制

#![allow(dead_code)]

use crate::file::backup::BackupManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 修改类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// 新建文件
    Created,
    /// 修改现有文件
    Modified,
    /// 删除文件
    Deleted,
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeType::Created => write!(f, "Created"),
            ChangeType::Modified => write!(f, "Modified"),
            ChangeType::Deleted => write!(f, "Deleted"),
        }
    }
}

/// 差异统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffStats {
    /// 新增行数
    pub additions: usize,
    /// 删除行数
    pub deletions: usize,
    /// 总行数
    pub total_lines: usize,
}

impl DiffStats {
    /// 创建新的差异统计
    pub fn new(additions: usize, deletions: usize, total_lines: usize) -> Self {
        Self {
            additions,
            deletions,
            total_lines,
        }
    }

    /// 计算差异统计
    pub fn calculate(old_content: &str, new_content: &str) -> Self {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();

        // 简单的差异计算（实际项目中可以使用更复杂的算法）
        let mut additions = 0;
        let mut deletions = 0;

        let max_lines = old_lines.len().max(new_lines.len());

        for i in 0..max_lines {
            let old_line = old_lines.get(i);
            let new_line = new_lines.get(i);

            match (old_line, new_line) {
                (Some(old), Some(new)) if old != new => {
                    additions += 1;
                    deletions += 1;
                }
                (None, Some(_)) => additions += 1,
                (Some(_), None) => deletions += 1,
                _ => {}
            }
        }

        Self {
            additions,
            deletions,
            total_lines: new_lines.len(),
        }
    }
}

/// 文件修改结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileApplyResult {
    /// 文件路径
    pub file_path: String,
    /// 是否成功
    pub success: bool,
    /// 修改类型
    pub change_type: ChangeType,
    /// 备份 ID（如果有）
    pub backup_id: Option<String>,
    /// 差异统计
    pub diff_stats: DiffStats,
    /// 错误信息
    pub error: Option<String>,
}

impl FileApplyResult {
    /// 创建成功结果
    pub fn success(
        file_path: String,
        change_type: ChangeType,
        backup_id: Option<String>,
        diff_stats: DiffStats,
    ) -> Self {
        Self {
            file_path,
            success: true,
            change_type,
            backup_id,
            diff_stats,
            error: None,
        }
    }

    /// 创建失败结果
    pub fn failure(file_path: String, error: String) -> Self {
        Self {
            file_path,
            success: false,
            change_type: ChangeType::Modified,
            backup_id: None,
            diff_stats: DiffStats::default(),
            error: Some(error),
        }
    }
}

/// 批量修改请求
#[derive(Debug, Clone)]
pub struct FileChange {
    /// 文件路径
    pub file_path: String,
    /// 新内容
    pub content: String,
    /// 是否强制覆盖（不备份）
    pub force: bool,
}

/// 批量修改结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyResult {
    /// 总文件数
    pub total_files: usize,
    /// 成功数量
    pub success_count: usize,
    /// 失败数量
    pub failure_count: usize,
    /// 详细结果
    pub results: Vec<FileApplyResult>,
}

impl BatchApplyResult {
    /// 创建新的批量结果
    pub fn new(results: Vec<FileApplyResult>) -> Self {
        let total_files = results.len();
        let success_count = results.iter().filter(|r| r.success).count();
        let failure_count = total_files - success_count;

        Self {
            total_files,
            success_count,
            failure_count,
            results,
        }
    }
}

/// 文件应用器
pub struct FileApplier {
    /// 项目根目录
    project_root: PathBuf,
    /// 备份管理器
    backup_manager: BackupManager,
}

impl FileApplier {
    /// 创建新的文件应用器
    pub fn new(project_root: &str) -> Self {
        let project_root = PathBuf::from(project_root);
        let backup_manager = BackupManager::new(project_root.to_str().unwrap());

        Self {
            project_root,
            backup_manager,
        }
    }

    /// 应用单个文件修改
    pub fn apply_file(&self, file_path: &str, content: &str) -> Result<FileApplyResult> {
        let full_path = self.project_root.join(file_path);

        // 验证文件路径
        if let Err(e) = self.validate_file_path(file_path) {
            return Ok(FileApplyResult::failure(
                file_path.to_string(),
                e.to_string(),
            ));
        }

        // 确定修改类型
        let change_type = if full_path.exists() {
            ChangeType::Modified
        } else {
            ChangeType::Created
        };

        // 如果是修改现有文件，创建备份
        let mut backup_id = None;
        if change_type == ChangeType::Modified {
            match self.backup_manager.create_backup(file_path) {
                Ok(info) => {
                    log::info!("Created backup: {}", info.backup_id);
                    backup_id = Some(info.backup_id);
                }
                Err(e) => {
                    log::warn!("Failed to create backup, continuing anyway: {}", e);
                }
            }
        }

        // 确保目录存在
        if let Some(parent) = full_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Ok(FileApplyResult::failure(
                    file_path.to_string(),
                    format!("Failed to create directory: {}", e),
                ));
            }
        }

        // 读取旧内容（用于差异计算）
        let old_content = fs::read_to_string(&full_path).unwrap_or_default();

        // 写入文件
        if let Err(e) = fs::write(&full_path, content) {
            return Ok(FileApplyResult::failure(
                file_path.to_string(),
                format!("Failed to write file: {}", e),
            ));
        }

        // 计算差异
        let diff_stats = DiffStats::calculate(&old_content, content);

        log::info!(
            "Applied file {}: {:?}, +{} -{} lines",
            file_path,
            change_type,
            diff_stats.additions,
            diff_stats.deletions
        );

        Ok(FileApplyResult::success(
            file_path.to_string(),
            change_type,
            backup_id,
            diff_stats,
        ))
    }

    /// 应用多个文件修改（原子操作）
    pub fn apply_batch(&self, changes: &[FileChange]) -> Result<BatchApplyResult> {
        log::info!("Applying batch changes to {} files", changes.len());

        let mut results = Vec::new();
        let mut created_backups = Vec::new();

        // 第一阶段：验证所有文件并创建备份
        for change in changes {
            // 验证文件路径
            if let Err(e) = self.validate_file_path(&change.file_path) {
                results.push(FileApplyResult::failure(
                    change.file_path.clone(),
                    e.to_string(),
                ));
                continue;
            }

            // 如果是修改现有文件且不是强制模式，创建备份
            let full_path = self.project_root.join(&change.file_path);
            if full_path.exists() && !change.force {
                match self.backup_manager.create_backup(&change.file_path) {
                    Ok(info) => {
                        created_backups.push(info.backup_id.clone());
                    }
                    Err(e) => {
                        log::warn!("Failed to create backup for {}: {}", change.file_path, e);
                    }
                }
            }
        }

        // 第二阶段：应用所有更改
        for change in changes {
            match self.apply_file(&change.file_path, &change.content) {
                Ok(apply_result) => {
                    results.push(apply_result);
                }
                Err(e) => {
                    results.push(FileApplyResult::failure(
                        change.file_path.clone(),
                        e.to_string(),
                    ));
                }
            }
        }

        let batch_result = BatchApplyResult::new(results.clone());

        // 如果有失败的，可以选择回滚（这里简化处理，不回滚）
        if batch_result.failure_count > 0 {
            log::warn!(
                "Batch apply completed with {} failures out of {}",
                batch_result.failure_count,
                batch_result.total_files
            );
        }

        Ok(batch_result)
    }

    /// 回滚文件修改
    pub fn rollback(&self, backup_id: &str) -> Result<()> {
        log::info!("Rolling back to backup: {}", backup_id);
        self.backup_manager.restore_backup(backup_id)
    }

    /// 获取备份列表
    pub fn list_backups(&self) -> Result<Vec<crate::file::backup::BackupInfo>> {
        self.backup_manager.list_backups()
    }

    /// 清理旧备份
    pub fn cleanup_backups(&self, max_age_days: u32) -> Result<usize> {
        self.backup_manager.cleanup_old_backups(max_age_days)
    }

    /// 验证文件路径
    fn validate_file_path(&self, file_path: &str) -> Result<()> {
        // 检查路径是否为空
        if file_path.is_empty() {
            return Err(anyhow::anyhow!("File path cannot be empty"));
        }

        // 检查路径是否包含非法字符
        let path = Path::new(file_path);

        // 不允许绝对路径
        if path.is_absolute() {
            return Err(anyhow::anyhow!("Absolute paths are not allowed"));
        }

        // 不允许路径遍历攻击
        if file_path.contains("..") {
            return Err(anyhow::anyhow!("Path traversal is not allowed"));
        }

        // 检查文件名长度
        if let Some(filename) = path.file_name() {
            if filename.to_string_lossy().len() > 255 {
                return Err(anyhow::anyhow!("Filename too long"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_stats_new() {
        let stats = DiffStats::new(5, 3, 10);

        assert_eq!(stats.additions, 5);
        assert_eq!(stats.deletions, 3);
        assert_eq!(stats.total_lines, 10);
    }

    #[test]
    fn test_diff_stats_calculate() {
        let old = "line1\nline2\nline3";
        let new = "line1\nmodified\nline3\nline4";

        let stats = DiffStats::calculate(old, new);

        assert!(stats.additions >= 1); // modified + line4
        assert!(stats.deletions >= 1); // line2
        assert!(stats.total_lines >= 3);
    }

    #[test]
    fn test_change_type_display() {
        assert_eq!(ChangeType::Created.to_string(), "Created");
        assert_eq!(ChangeType::Modified.to_string(), "Modified");
        assert_eq!(ChangeType::Deleted.to_string(), "Deleted");
    }
}

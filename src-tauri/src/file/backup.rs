//! 文件备份管理
//! 
//! 提供文件备份、恢复和清理功能

#![allow(dead_code)]

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 备份信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// 备份 ID（时间戳 + 随机数）
    pub backup_id: String,
    /// 原始文件路径
    pub original_path: String,
    /// 备份文件路径
    pub backup_path: String,
    /// 备份时间戳
    pub timestamp: i64,
    /// 文件大小（字节）
    pub file_size: u64,
}

/// 备份管理器
pub struct BackupManager {
    /// 项目根目录
    project_root: PathBuf,
    /// 备份目录
    backup_dir: PathBuf,
}

impl BackupManager {
    /// 创建新的备份管理器
    pub fn new(project_root: &str) -> Self {
        let project_root = PathBuf::from(project_root);
        let backup_dir = project_root.join(".harness-backups");
        
        // 确保备份目录存在
        if let Err(e) = fs::create_dir_all(&backup_dir) {
            log::warn!("Failed to create backup directory: {}", e);
        }
        
        Self {
            project_root,
            backup_dir,
        }
    }
    
    /// 创建文件备份
    pub fn create_backup(&self, file_path: &str) -> Result<BackupInfo> {
        let full_path = self.project_root.join(file_path);
        
        // 检查文件是否存在
        if !full_path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {}", file_path));
        }
        
        // 生成备份 ID
        let timestamp = Utc::now();
        let backup_id = format!(
            "{}_{:x}",
            timestamp.format("%Y%m%d_%H%M%S"),
            uuid::Uuid::new_v4().as_u128() % 0x100000000 // 取低 32 位作为 8 个十六进制字符
        );
        
        // 构建备份文件路径
        let relative_path = Path::new(file_path);
        let backup_filename = format!("{}_{}", backup_id, relative_path.file_name().unwrap_or_default().to_string_lossy());
        let backup_path = self.backup_dir.join(&backup_filename);
        
        // 确保备份目录的父目录存在
        if let Some(parent) = backup_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 复制文件到备份目录
        fs::copy(&full_path, &backup_path)
            .with_context(|| format!("Failed to backup file: {}", file_path))?;
        
        // 获取文件大小
        let file_size = fs::metadata(&full_path)?.len();
        
        let backup_info = BackupInfo {
            backup_id,
            original_path: file_path.to_string(),
            backup_path: backup_path.to_string_lossy().to_string(),
            timestamp: timestamp.timestamp(),
            file_size,
        };
        
        log::info!("Created backup for {}: {:?}", file_path, backup_info.backup_id);
        
        Ok(backup_info)
    }
    
    /// 恢复备份
    pub fn restore_backup(&self, backup_id: &str) -> Result<()> {
        // 查找备份文件
        let backup_file = self.find_backup_file(backup_id)?;
        
        // 从备份文件名中提取原始文件名
        let backup_filename = backup_file.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid backup filename"))?;
        
        // 解析原始文件名（格式：backupId_originalFilename）
        let original_filename = backup_filename
            .splitn(2, '_')
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Cannot parse original filename from backup"))?;
        
        let original_path = self.project_root.join(original_filename);
        
        // 确保目标目录存在
        if let Some(parent) = original_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 恢复文件
        fs::copy(&backup_file, &original_path)
            .with_context(|| format!("Failed to restore backup: {}", backup_id))?;
        
        log::info!("Restored backup {} to {:?}", backup_id, original_path);
        
        Ok(())
    }
    
    /// 删除备份
    pub fn delete_backup(&self, backup_id: &str) -> Result<()> {
        let backup_file = self.find_backup_file(backup_id)?;
        fs::remove_file(&backup_file)?;
        log::info!("Deleted backup: {}", backup_id);
        Ok(())
    }
    
    /// 清理旧备份
    pub fn cleanup_old_backups(&self, max_age_days: u32) -> Result<usize> {
        let cutoff = Utc::now() - Duration::days(max_age_days as i64);
        let mut deleted_count = 0;
        
        // 遍历备份目录
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            // 检查备份时间
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    let modified_time: DateTime<Utc> = modified.into();
                    if modified_time < cutoff {
                        fs::remove_file(&path)?;
                        deleted_count += 1;
                    }
                }
            }
        }
        
        log::info!("Cleaned up {} old backups", deleted_count);
        
        Ok(deleted_count)
    }
    
    /// 获取所有备份
    pub fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();
        
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                // 解析备份文件名
                if let Some((backup_id, original_filename)) = filename.split_once('_') {
                    let metadata = fs::metadata(&path)?;
                    
                    backups.push(BackupInfo {
                        backup_id: backup_id.to_string(),
                        original_path: original_filename.to_string(),
                        backup_path: path.to_string_lossy().to_string(),
                        timestamp: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0),
                        file_size: metadata.len(),
                    });
                }
            }
        }
        
        // 按时间倒序排序
        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(backups)
    }
    
    /// 查找备份文件
    fn find_backup_file(&self, backup_id: &str) -> Result<PathBuf> {
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with(&format!("{}_", backup_id)) {
                    return Ok(path);
                }
            }
        }
        
        Err(anyhow::anyhow!("Backup not found: {}", backup_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_info_structure() {
        let info = BackupInfo {
            backup_id: "test_123".to_string(),
            original_path: "test.txt".to_string(),
            backup_path: "/backup/test.txt".to_string(),
            timestamp: 1234567890,
            file_size: 1024,
        };
        
        assert_eq!(info.backup_id, "test_123");
        assert_eq!(info.original_path, "test.txt");
        assert_eq!(info.file_size, 1024);
    }
}
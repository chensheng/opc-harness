//! 文件服务
//!
//! 提供文件系统操作

use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

/// 文件服务
pub struct FileService;

impl FileService {
    /// 创建新的文件服务
    pub fn new() -> Self {
        Self
    }

    /// 读取文件内容
    pub async fn read_file(&self, path: &Path) -> Result<String> {
        let content = fs::read_to_string(path).await?;
        Ok(content)
    }

    /// 写入文件
    pub async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        fs::write(path, content).await?;
        Ok(())
    }

    /// 创建目录
    pub async fn create_dir(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).await?;
        Ok(())
    }

    /// 读取目录内容
    pub async fn read_dir(&self, path: &Path) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let metadata = entry.metadata().await?;
            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_file() {
                FileType::File
            } else {
                FileType::Other
            };

            entries.push(FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path(),
                file_type,
                size: metadata.len(),
                modified: metadata.modified()?.into(),
            });
        }

        Ok(entries)
    }

    /// 删除文件
    pub async fn delete_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path).await?;
        Ok(())
    }

    /// 删除目录
    pub async fn delete_dir(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path).await?;
        Ok(())
    }

    /// 复制文件
    pub async fn copy_file(&self, from: &Path, to: &Path) -> Result<()> {
        fs::copy(from, to).await?;
        Ok(())
    }

    /// 移动文件
    pub async fn move_file(&self, from: &Path, to: &Path) -> Result<()> {
        fs::rename(from, to).await?;
        Ok(())
    }

    /// 检查文件是否存在
    pub async fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// 获取应用数据目录
    pub fn get_app_data_dir() -> Result<PathBuf> {
        let dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?
            .join("OPC-HARNESS");
        
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        
        Ok(dir)
    }

    /// 获取项目根目录
    pub fn get_projects_dir() -> Result<PathBuf> {
        let dir = Self::get_app_data_dir()?.join("projects");
        
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        
        Ok(dir)
    }
}

/// 文件条目
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: std::time::SystemTime,
}

/// 文件类型
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Other,
}

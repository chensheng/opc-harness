//! File System Tools for Native Coding Agent
//!
//! 提供安全的文件系统操作工具，限制访问范围在工作空间内。

use std::path::PathBuf;
use tokio::fs;

/// 文件系统工具集
pub struct FileSystemTools {
    workspace_root: PathBuf,
}

impl FileSystemTools {
    /// 创建新的文件系统工具集
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    /// 验证路径是否在工作空间内
    fn validate_path(&self, path: &str) -> Result<PathBuf, String> {
        let full_path = self.workspace_root.join(path);

        // 规范化 workspace_root
        let canonical_root = self
            .workspace_root
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize workspace root: {}", e))?;

        // 规范化路径，防止 ../ 等绕过
        let canonical = match full_path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // 如果文件不存在，检查父目录
                if let Some(parent) = full_path.parent() {
                    match parent.canonicalize() {
                        Ok(p) => p.join(full_path.file_name().unwrap_or_default()),
                        Err(_) => return Err(format!("Invalid path: {}", path)),
                    }
                } else {
                    return Err(format!("Invalid path: {}", path));
                }
            }
        };

        // 确保路径在工作空间内（使用规范化后的根路径进行比较）
        if !canonical.starts_with(&canonical_root) {
            return Err(format!("Access denied: {} is outside workspace", path));
        }

        Ok(canonical)
    }

    /// 读取文件内容
    pub async fn read_file(&self, path: &str) -> Result<String, String> {
        let full_path = self.validate_path(path)?;

        // 检查文件大小（最大 500KB）
        let metadata = fs::metadata(&full_path)
            .await
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;

        if metadata.len() > 500 * 1024 {
            // 读取前 500KB
            let mut file = fs::read(&full_path)
                .await
                .map_err(|e| format!("Failed to read file: {}", e))?;
            file.truncate(500 * 1024);
            let content = String::from_utf8_lossy(&file).to_string();
            return Ok(format!("{}\\n[File truncated to 500KB]", content));
        }

        let content = fs::read_to_string(&full_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        Ok(content)
    }

    /// 写入文件内容
    pub async fn write_file(&self, path: &str, content: &str) -> Result<String, String> {
        let full_path = self.validate_path(path)?;

        // 自动创建父目录
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // 备份原文件（如果存在）
        if full_path.exists() {
            let backup_path = self
                .workspace_root
                .join(".backup")
                .join(path.replace(['/', '\\'], "_"));
            if let Some(backup_parent) = backup_path.parent() {
                let _ = fs::create_dir_all(backup_parent).await;
                let _ = fs::copy(&full_path, &backup_path).await;
            }
        }

        fs::write(&full_path, content)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(format!("File written successfully: {}", path))
    }

    /// 列出目录内容（使用广度优先遍历，避免递归）
    pub async fn list_directory(
        &self,
        path: &str,
        recursive: bool,
        max_depth: usize,
    ) -> Result<String, String> {
        let full_path = self.validate_path(path)?;

        if !full_path.is_dir() {
            return Err(format!("Not a directory: {}", path));
        }

        let mut result = Vec::new();

        // 使用队列进行广度优先遍历
        let mut queue: Vec<(PathBuf, String, usize)> = vec![(full_path.clone(), "".to_string(), 0)];

        while let Some((current_dir, current_rel, depth)) = queue.pop() {
            if depth > max_depth {
                continue;
            }

            let mut entries = fs::read_dir(&current_dir)
                .await
                .map_err(|e| format!("Failed to read directory: {}", e))?;

            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| format!("Failed to read entry: {}", e))?
            {
                let name = entry.file_name().to_string_lossy().to_string();
                let entry_path = entry.path();
                let rel_path = if current_rel.is_empty() {
                    name.clone()
                } else {
                    format!("{}/{}", current_rel, name)
                };

                if entry_path.is_dir() {
                    result.push(format!("📁 {}", rel_path));
                    if recursive && depth < max_depth {
                        queue.push((entry_path, rel_path, depth + 1));
                    }
                } else {
                    let metadata = entry.metadata().await.ok();
                    let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                    result.push(format!("📄 {} ({} bytes)", rel_path, size));
                }
            }
        }

        Ok(result.join("\n"))
    }

    /// 编辑文件（基于行号）
    pub async fn edit_file(
        &self,
        path: &str,
        start_line: usize,
        end_line: usize,
        new_content: &str,
    ) -> Result<String, String> {
        let full_path = self.validate_path(path)?;

        let content = fs::read_to_string(&full_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let lines: Vec<&str> = content.lines().collect();

        if start_line > lines.len() || end_line > lines.len() || start_line > end_line {
            return Err(format!(
                "Invalid line range: {}-{} (file has {} lines)",
                start_line,
                end_line,
                lines.len()
            ));
        }

        let mut new_lines: Vec<String> = Vec::new();

        // 保留 start_line 之前的行
        new_lines.extend(
            lines[..start_line.saturating_sub(1)]
                .iter()
                .map(|s| s.to_string()),
        );

        // 插入新内容
        new_lines.push(new_content.to_string());

        // 保留 end_line 之后的行
        if end_line < lines.len() {
            new_lines.extend(lines[end_line..].iter().map(|s| s.to_string()));
        }

        let new_content_full = new_lines.join("\n");

        fs::write(&full_path, &new_content_full)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        let added_lines = new_content.lines().count();
        let removed_lines = end_line - start_line + 1;

        Ok(format!(
            "File edited: {} lines added, {} lines removed",
            added_lines, removed_lines
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_read_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "Hello, World!").await.unwrap();

        let tools = FileSystemTools::new(temp_dir.path().to_path_buf());
        let content = tools.read_file("test.txt").await.unwrap();

        assert_eq!(content, "Hello, World!");
    }

    #[tokio::test]
    async fn test_read_file_outside_workspace() {
        let temp_dir = TempDir::new().unwrap();
        let tools = FileSystemTools::new(temp_dir.path().to_path_buf());

        // 尝试访问工作空间外的文件（使用绝对路径）
        let outside_path = std::path::Path::new("/").join("tmp").join("outside.txt");
        let result = tools.read_file(&outside_path.to_string_lossy()).await;

        // 应该返回错误
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let tools = FileSystemTools::new(temp_dir.path().to_path_buf());

        let result = tools
            .write_file("new_file.txt", "Test content")
            .await
            .unwrap();

        assert!(result.contains("written successfully"));
        assert!(temp_dir.path().join("new_file.txt").exists());
    }

    #[tokio::test]
    async fn test_list_directory() {
        let temp_dir = TempDir::new().unwrap();

        // 创建测试文件结构
        fs::write(temp_dir.path().join("file1.txt"), "content1")
            .await
            .unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content2")
            .await
            .unwrap();
        fs::create_dir(temp_dir.path().join("subdir"))
            .await
            .unwrap();
        fs::write(temp_dir.path().join("subdir/file3.txt"), "content3")
            .await
            .unwrap();

        let tools = FileSystemTools::new(temp_dir.path().to_path_buf());
        let listing = tools.list_directory(".", true, 5).await.unwrap();

        assert!(listing.contains("file1.txt"));
        assert!(listing.contains("file2.txt"));
        assert!(listing.contains("subdir"));
    }
}

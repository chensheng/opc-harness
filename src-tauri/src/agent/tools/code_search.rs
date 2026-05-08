//! Code Search Tools for Native Coding Agent
//!
//! 提供安全的代码搜索能力，支持正则表达式搜索、文件查找和符号定位。

use std::path::PathBuf;
use tokio::fs;

/// 搜索结果项
#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub file_path: String,
    pub line_number: usize,
    pub content: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

/// 符号位置信息
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    pub file_path: String,
    pub line_number: usize,
    pub symbol_type: SymbolType,
}

/// 符号类型
#[derive(Debug, Clone)]
pub enum SymbolType {
    Function,
    Class,
    Variable,
    Unknown,
}

/// 代码搜索工具集
pub struct CodeSearchTools {
    workspace_root: PathBuf,
}

impl CodeSearchTools {
    /// 创建新的代码搜索工具集
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

        // 确保路径在工作空间内
        if !canonical.starts_with(&canonical_root) {
            return Err(format!("Access denied: {} is outside workspace", path));
        }

        Ok(canonical)
    }

    /// 正则表达式搜索（grep）
    ///
    /// # Arguments
    /// * `pattern` - 正则表达式模式
    /// * `path` - 可选的搜索路径（相对于 workspace），默认为整个 workspace
    ///
    /// # Returns
    /// 匹配结果列表，最多 50 条
    pub async fn grep(&self, pattern: &str, path: Option<&str>) -> Result<Vec<SearchMatch>, String> {
        let search_path = match path {
            Some(p) => self.validate_path(p)?,
            None => self.workspace_root.clone(),
        };

        log::info!("Searching for pattern '{}' in {:?}", pattern, search_path);

        // 使用简单的行级搜索实现
        // TODO: 可以集成 ripgrep 以获得更好的性能
        let mut results = Vec::new();
        self.search_directory_recursive(&search_path, pattern, &mut results, 50).await?;

        log::info!("Found {} matches", results.len());
        Ok(results)
    }

    /// 递归搜索目录
    async fn search_directory_recursive(
        &self,
        dir_path: &PathBuf,
        pattern: &str,
        results: &mut Vec<SearchMatch>,
        max_results: usize,
    ) -> Result<(), String> {
        if results.len() >= max_results {
            return Ok(());
        }

        let mut entries = fs::read_dir(dir_path)
            .await
            .map_err(|e| format!("Failed to read directory {:?}: {}", dir_path, e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            if results.len() >= max_results {
                break;
            }

            let path = entry.path();

            // 跳过隐藏文件和 node_modules 等
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "node_modules" || name == "target" {
                    continue;
                }
            }

            if path.is_dir() {
                // 递归搜索子目录
                Box::pin(self.search_directory_recursive(&path, pattern, results, max_results)).await?;
            } else if path.is_file() {
                // 只搜索文本文件
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if Self::is_text_file_extension(ext) {
                        if let Ok(matches) = self.search_file(&path, pattern).await {
                            results.extend(matches);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 搜索单个文件
    async fn search_file(&self, file_path: &PathBuf, pattern: &str) -> Result<Vec<SearchMatch>, String> {
        let content = fs::read_to_string(file_path)
            .await
            .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

        let lines: Vec<&str> = content.lines().collect();
        let mut matches = Vec::new();

        // 简单的字符串匹配（可以升级为正则表达式）
        for (idx, line) in lines.iter().enumerate() {
            if line.contains(pattern) {
                let line_number = idx + 1;

                // 获取上下文（前后各 2 行）
                let context_before: Vec<String> = lines
                    .iter()
                    .skip(idx.saturating_sub(2))
                    .take(2)
                    .map(|s| s.to_string())
                    .collect();

                let context_after: Vec<String> = lines
                    .iter()
                    .skip(idx + 1)
                    .take(2)
                    .map(|s| s.to_string())
                    .collect();

                let relative_path = file_path
                    .strip_prefix(&self.workspace_root)
                    .unwrap_or(file_path)
                    .to_string_lossy()
                    .to_string();

                matches.push(SearchMatch {
                    file_path: relative_path,
                    line_number,
                    content: line.to_string(),
                    context_before,
                    context_after,
                });
            }
        }

        Ok(matches)
    }

    /// 文件查找（支持 glob 模式）
    ///
    /// # Arguments
    /// * `pattern` - 文件名模式（支持 * 和 ? 通配符）
    /// * `extensions` - 可选的文件扩展名过滤（如 ["ts", "tsx"]）
    pub async fn find_files(
        &self,
        pattern: &str,
        extensions: Option<&[&str]>,
    ) -> Result<Vec<String>, String> {
        log::info!("Finding files matching pattern '{}' with extensions {:?}", pattern, extensions);

        let mut results = Vec::new();
        self.find_files_recursive(&self.workspace_root, pattern, extensions, &mut results)
            .await?;

        log::info!("Found {} files", results.len());
        Ok(results)
    }

    /// 递归查找文件
    async fn find_files_recursive(
        &self,
        dir_path: &PathBuf,
        pattern: &str,
        extensions: Option<&[&str]>,
        results: &mut Vec<String>,
    ) -> Result<(), String> {
        let mut entries = fs::read_dir(dir_path)
            .await
            .map_err(|e| format!("Failed to read directory {:?}: {}", dir_path, e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();

            // 跳过隐藏文件和特定目录
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "node_modules" || name == "target" {
                    continue;
                }
            }

            if path.is_dir() {
                Box::pin(self.find_files_recursive(&path, pattern, extensions, results)).await?;
            } else if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // 检查文件名是否匹配模式
                    if Self::matches_pattern(file_name, pattern) {
                        // 检查扩展名过滤
                        if let Some(exts) = extensions {
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                if !exts.contains(&ext) {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }

                        let relative_path = path
                            .strip_prefix(&self.workspace_root)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string();

                        results.push(relative_path);
                    }
                }
            }
        }

        Ok(())
    }

    /// 检查文件名是否匹配 glob 模式
    fn matches_pattern(file_name: &str, pattern: &str) -> bool {
        // 简单的 glob 匹配实现
        // 支持 * (任意字符) 和 ? (单个字符)
        if pattern == "*" {
            return true;
        }

        // 转换为正则表达式进行匹配
        let regex_pattern = pattern
            .replace('.', "\\.")
            .replace("*", ".*")
            .replace("?", ".");

        let regex_pattern = format!("^{}$", regex_pattern);

        regex::Regex::new(&regex_pattern)
            .map(|re| re.is_match(file_name))
            .unwrap_or(false)
    }

    /// 判断是否为文本文件扩展名
    fn is_text_file_extension(ext: &str) -> bool {
        matches!(
            ext.to_lowercase().as_str(),
            "rs" | "ts"
                | "tsx"
                | "js"
                | "jsx"
                | "json"
                | "toml"
                | "yaml"
                | "yml"
                | "md"
                | "txt"
                | "css"
                | "html"
                | "py"
                | "go"
                | "java"
                | "c"
                | "cpp"
                | "h"
        )
    }

    /// 符号查找（简化版本）
    ///
    /// # Arguments
    /// * `symbol_name` - 要查找的符号名称（函数、类、变量）
    ///
    /// # Note
    /// 这是一个简化实现，使用字符串匹配。生产环境建议使用 tree-sitter 或语言服务器协议。
    pub async fn find_symbol(&self, symbol_name: &str) -> Result<Vec<SymbolLocation>, String> {
        log::info!("Searching for symbol '{}'", symbol_name);

        // 搜索包含符号定义的模式
        let patterns = vec![
            format!("fn {}", symbol_name),
            format!("function {}", symbol_name),
            format!("class {}", symbol_name),
            format!("const {} =", symbol_name),
            format!("let {} =", symbol_name),
            format!("var {} =", symbol_name),
            format!("struct {}", symbol_name),
            format!("impl {}", symbol_name),
        ];

        let mut locations = Vec::new();

        for pattern in patterns {
            if let Ok(matches) = self.grep(&pattern, None).await {
                for m in matches {
                    // 推断符号类型
                    let symbol_type = if m.content.contains("fn ") || m.content.contains("function ") {
                        SymbolType::Function
                    } else if m.content.contains("class ") || m.content.contains("struct ") {
                        SymbolType::Class
                    } else if m.content.contains("const ") || m.content.contains("let ") || m.content.contains("var ") {
                        SymbolType::Variable
                    } else {
                        SymbolType::Unknown
                    };

                    locations.push(SymbolLocation {
                        file_path: m.file_path,
                        line_number: m.line_number,
                        symbol_type,
                    });
                }
            }
        }

        log::info!("Found {} symbol locations", locations.len());
        Ok(locations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_grep_basic() {
        let temp_dir = TempDir::new().unwrap();
        let tools = CodeSearchTools::new(temp_dir.path().to_path_buf());

        // 创建测试文件
        let test_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&test_file, "hello world\nfoo bar\nhello again\n").await.unwrap();

        let results = tools.grep("hello", None).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].content, "hello world");
        assert_eq!(results[1].content, "hello again");
    }

    #[tokio::test]
    async fn test_find_files() {
        let temp_dir = TempDir::new().unwrap();
        let tools = CodeSearchTools::new(temp_dir.path().to_path_buf());

        // 创建测试文件
        tokio::fs::write(temp_dir.path().join("file1.ts"), "").await.unwrap();
        tokio::fs::write(temp_dir.path().join("file2.ts"), "").await.unwrap();
        tokio::fs::write(temp_dir.path().join("file3.js"), "").await.unwrap();

        let results = tools.find_files("*.ts", None).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_path_security() {
        let temp_dir = TempDir::new().unwrap();
        let tools = CodeSearchTools::new(temp_dir.path().to_path_buf());

        // 尝试访问工作空间外的路径应该失败
        let result = tools.grep("test", Some("../../etc/passwd")).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outside workspace"));
    }
}

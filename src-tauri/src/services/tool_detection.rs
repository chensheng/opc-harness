//! Tool detection service
//!
//! 检测本地开发工具的安装状态

use crate::models::ToolInfo;
use std::process::Command;

/// 工具类别
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolCategory {
    /// 运行时环境
    Runtime,
    /// 版本控制
    VersionControl,
    /// 包管理器
    PackageManager,
    /// AI 编码工具
    AICoding,
    /// 编辑器
    Editor,
    /// 构建工具
    BuildTool,
}

/// 工具检测结果
#[derive(Debug, Clone)]
pub struct ToolDetectionResult {
    pub tool: ToolInfo,
    pub category: ToolCategory,
    pub is_required: bool,
    pub description: String,
}

/// 工具检测服务
pub struct ToolDetectionService;

impl ToolDetectionService {
    /// 创建新的工具检测服务
    pub fn new() -> Self {
        Self
    }

    /// 检测所有工具
    pub fn detect_all(&self) -> Vec<ToolInfo> {
        vec![
            self.detect_node(),
            self.detect_git(),
            self.detect_npm(),
            self.detect_pnpm(),
            self.detect_yarn(),
            self.detect_kimi(),
            self.detect_claude(),
            self.detect_codex(),
            self.detect_vscode(),
            self.detect_cursor(),
            self.detect_python(),
            self.detect_rust(),
            self.detect_docker(),
        ]
    }

    /// 检测所有工具（带类别信息）
    pub fn detect_all_detailed(&self) -> Vec<ToolDetectionResult> {
        vec![
            ToolDetectionResult {
                tool: self.detect_node(),
                category: ToolCategory::Runtime,
                is_required: true,
                description: "JavaScript 运行时环境".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_npm(),
                category: ToolCategory::PackageManager,
                is_required: true,
                description: "Node.js 包管理器".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_pnpm(),
                category: ToolCategory::PackageManager,
                is_required: false,
                description: "高性能 Node.js 包管理器".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_yarn(),
                category: ToolCategory::PackageManager,
                is_required: false,
                description: "Facebook 的 Node.js 包管理器".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_git(),
                category: ToolCategory::VersionControl,
                is_required: true,
                description: "分布式版本控制系统".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_vscode(),
                category: ToolCategory::Editor,
                is_required: false,
                description: "Visual Studio Code 编辑器".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_cursor(),
                category: ToolCategory::Editor,
                is_required: false,
                description: "AI 驱动的代码编辑器".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_kimi(),
                category: ToolCategory::AICoding,
                is_required: false,
                description: "Kimi AI 编程助手".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_claude(),
                category: ToolCategory::AICoding,
                is_required: false,
                description: "Claude Code AI 编程助手".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_codex(),
                category: ToolCategory::AICoding,
                is_required: false,
                description: "OpenAI Codex CLI".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_python(),
                category: ToolCategory::Runtime,
                is_required: false,
                description: "Python 编程语言".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_rust(),
                category: ToolCategory::Runtime,
                is_required: false,
                description: "Rust 编程语言".to_string(),
            },
            ToolDetectionResult {
                tool: self.detect_docker(),
                category: ToolCategory::BuildTool,
                is_required: false,
                description: "容器化平台".to_string(),
            },
        ]
    }

    /// 检测必需工具
    pub fn detect_required_tools(&self) -> Vec<ToolDetectionResult> {
        self.detect_all_detailed()
            .into_iter()
            .filter(|r| r.is_required)
            .collect()
    }

    /// 检测缺失的必需工具
    pub fn detect_missing_required(&self) -> Vec<String> {
        self.detect_required_tools()
            .into_iter()
            .filter(|r| !r.tool.installed)
            .map(|r| r.tool.name)
            .collect()
    }

    /// 检查所有必需工具是否已安装
    pub fn all_required_installed(&self) -> bool {
        self.detect_missing_required().is_empty()
    }

    /// 检测 Node.js
    fn detect_node(&self) -> ToolInfo {
        let mut info = self.detect_tool("Node.js", "node", &["--version"]);
        // 清理版本字符串中的 "v" 前缀
        if let Some(ref v) = info.version {
            info.version = Some(v.trim_start_matches('v').to_string());
        }
        info
    }

    /// 检测 Git
    fn detect_git(&self) -> ToolInfo {
        let mut info = self.detect_tool("Git", "git", &["--version"]);
        // 提取版本号，格式通常是 "git version 2.x.x"
        if let Some(ref v) = info.version {
            let parts: Vec<&str> = v.split_whitespace().collect();
            if parts.len() >= 3 {
                info.version = Some(parts[2].to_string());
            }
        }
        info
    }

    /// 检测 npm
    fn detect_npm(&self) -> ToolInfo {
        self.detect_tool("npm", "npm", &["--version"])
    }

    /// 检测 pnpm
    fn detect_pnpm(&self) -> ToolInfo {
        self.detect_tool("pnpm", "pnpm", &["--version"])
    }

    /// 检测 yarn
    fn detect_yarn(&self) -> ToolInfo {
        self.detect_tool("Yarn", "yarn", &["--version"])
    }

    /// 检测 Kimi CLI
    fn detect_kimi(&self) -> ToolInfo {
        self.detect_tool("Kimi CLI", "kimi", &["--version"])
    }

    /// 检测 Claude Code
    fn detect_claude(&self) -> ToolInfo {
        self.detect_tool("Claude Code", "claude", &["--version"])
    }

    /// 检测 Codex CLI
    fn detect_codex(&self) -> ToolInfo {
        self.detect_tool("Codex CLI", "codex", &["--version"])
    }

    /// 检测 VS Code
    fn detect_vscode(&self) -> ToolInfo {
        let mut info = self.detect_tool("VS Code", "code", &["--version"]);
        if !info.installed {
            // Try Windows path
            info = self.detect_tool_with_path(
                "VS Code",
                r"C:\Program Files\Microsoft VS Code\bin\code.cmd",
                &["--version"],
            );
        }
        info
    }

    /// 检测 Cursor
    fn detect_cursor(&self) -> ToolInfo {
        self.detect_tool("Cursor", "cursor", &["--version"])
    }

    /// 检测 Python
    fn detect_python(&self) -> ToolInfo {
        // 优先检测 python3，如果不存在则检测 python
        let info = self.detect_tool("Python", "python3", &["--version"]);
        if info.installed {
            return info;
        }
        self.detect_tool("Python", "python", &["--version"])
    }

    /// 检测 Rust
    fn detect_rust(&self) -> ToolInfo {
        let mut info = self.detect_tool("Rust", "rustc", &["--version"]);
        // 提取版本号，格式: "rustc 1.x.x (hash date)"
        if let Some(ref v) = info.version {
            let parts: Vec<&str> = v.split_whitespace().collect();
            if parts.len() >= 2 {
                info.version = Some(parts[1].to_string());
            }
        }
        info
    }

    /// 检测 Docker
    fn detect_docker(&self) -> ToolInfo {
        let mut info = self.detect_tool("Docker", "docker", &["--version"]);
        // 提取版本号，格式: "Docker version 24.x.x, build xxx"
        if let Some(ref v) = info.version {
            if let Some(start) = v.find("version ") {
                let version_part = &v[start + 8..];
                let end = version_part.find(',').unwrap_or(version_part.len());
                info.version = Some(version_part[..end].to_string());
            }
        }
        info
    }

    /// 通用工具检测
    fn detect_tool(&self, name: &str, cmd: &str, args: &[&str]) -> ToolInfo {
        match Command::new(cmd).args(args).output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .map(|s| s.trim().to_string());

                    ToolInfo {
                        name: name.to_string(),
                        installed: true,
                        version,
                        path: which::which(cmd).ok().map(|p| p.to_string_lossy().to_string()),
                    }
                } else {
                    ToolInfo {
                        name: name.to_string(),
                        installed: false,
                        version: None,
                        path: None,
                    }
                }
            }
            Err(_) => ToolInfo {
                name: name.to_string(),
                installed: false,
                version: None,
                path: None,
            },
        }
    }

    /// 使用特定路径检测工具
    fn detect_tool_with_path(&self, name: &str, path: &str, args: &[&str]) -> ToolInfo {
        match Command::new(path).args(args).output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .map(|s| s.trim().to_string());

                    ToolInfo {
                        name: name.to_string(),
                        installed: true,
                        version,
                        path: Some(path.to_string()),
                    }
                } else {
                    ToolInfo {
                        name: name.to_string(),
                        installed: false,
                        version: None,
                        path: None,
                    }
                }
            }
            Err(_) => ToolInfo {
                name: name.to_string(),
                installed: false,
                version: None,
                path: None,
            },
        }
    }

    /// 检查特定工具是否已安装
    pub fn is_installed(&self, tool: &str) -> bool {
        match tool {
            "node" => self.detect_node().installed,
            "git" => self.detect_git().installed,
            "npm" => self.detect_npm().installed,
            "pnpm" => self.detect_pnpm().installed,
            "yarn" => self.detect_yarn().installed,
            "kimi" => self.detect_kimi().installed,
            "claude" => self.detect_claude().installed,
            "codex" => self.detect_codex().installed,
            "vscode" => self.detect_vscode().installed,
            "cursor" => self.detect_cursor().installed,
            "python" => self.detect_python().installed,
            "rust" => self.detect_rust().installed,
            "docker" => self.detect_docker().installed,
            _ => false,
        }
    }

    /// 获取工具信息
    pub fn get_tool_info(&self, tool: &str) -> Option<ToolInfo> {
        match tool {
            "node" => Some(self.detect_node()),
            "git" => Some(self.detect_git()),
            "npm" => Some(self.detect_npm()),
            "pnpm" => Some(self.detect_pnpm()),
            "yarn" => Some(self.detect_yarn()),
            "kimi" => Some(self.detect_kimi()),
            "claude" => Some(self.detect_claude()),
            "codex" => Some(self.detect_codex()),
            "vscode" => Some(self.detect_vscode()),
            "cursor" => Some(self.detect_cursor()),
            "python" => Some(self.detect_python()),
            "rust" => Some(self.detect_rust()),
            "docker" => Some(self.detect_docker()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_all() {
        let service = ToolDetectionService::new();
        let tools = service.detect_all();
        assert!(!tools.is_empty());
    }

    #[test]
    fn test_is_installed() {
        let service = ToolDetectionService::new();
        // 至少检测一个不报错
        let _ = service.is_installed("node");
        let _ = service.is_installed("git");
    }

    #[test]
    fn test_detect_required() {
        let service = ToolDetectionService::new();
        let required = service.detect_required_tools();
        assert!(!required.is_empty());
        // 验证所有标记为 required 的都被返回
        assert!(required.iter().all(|r| r.is_required));
    }

    #[test]
    fn test_get_tool_info() {
        let service = ToolDetectionService::new();
        let info = service.get_tool_info("node");
        assert!(info.is_some());
        assert_eq!(info.unwrap().name, "Node.js");
    }
}

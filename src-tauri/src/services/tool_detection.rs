//! Tool detection service

use crate::models::ToolInfo;
use anyhow::Result;
use std::process::Command;

pub struct ToolDetectionService;

impl ToolDetectionService {
    pub fn new() -> Self {
        Self
    }

    /// Detect all tools
    pub fn detect_all(&self) -> Vec<ToolInfo> {
        vec![
            self.detect_node(),
            self.detect_git(),
            self.detect_npm(),
            self.detect_kimi(),
            self.detect_claude(),
            self.detect_codex(),
            self.detect_vscode(),
            self.detect_cursor(),
        ]
    }

    /// Detect Node.js
    fn detect_node(&self) -> ToolInfo {
        self.detect_tool("Node.js", "node", &["--version"])
    }

    /// Detect Git
    fn detect_git(&self) -> ToolInfo {
        self.detect_tool("Git", "git", &["--version"])
    }

    /// Detect npm
    fn detect_npm(&self) -> ToolInfo {
        self.detect_tool("npm", "npm", &["--version"])
    }

    /// Detect Kimi CLI
    fn detect_kimi(&self) -> ToolInfo {
        self.detect_tool("Kimi CLI", "kimi", &["--version"])
    }

    /// Detect Claude Code
    fn detect_claude(&self) -> ToolInfo {
        self.detect_tool("Claude Code", "claude", &["--version"])
    }

    /// Detect Codex CLI
    fn detect_codex(&self) -> ToolInfo {
        self.detect_tool("Codex CLI", "codex", &["--version"])
    }

    /// Detect VS Code
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

    /// Detect Cursor
    fn detect_cursor(&self) -> ToolInfo {
        self.detect_tool("Cursor", "cursor", &["--version"])
    }

    /// Generic tool detection
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

    /// Tool detection with specific path
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

    /// Check if a specific tool is installed
    pub fn is_installed(&self, tool: &str) -> bool {
        match tool {
            "node" => self.detect_node().installed,
            "git" => self.detect_git().installed,
            "npm" => self.detect_npm().installed,
            "kimi" => self.detect_kimi().installed,
            "claude" => self.detect_claude().installed,
            "codex" => self.detect_codex().installed,
            "vscode" => self.detect_vscode().installed,
            "cursor" => self.detect_cursor().installed,
            _ => false,
        }
    }
}

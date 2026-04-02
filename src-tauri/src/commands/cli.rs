use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub is_installed: bool,
    pub version: Option<String>,
    pub install_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectToolsResponse {
    pub tools: Vec<ToolStatus>,
}

/// 检测单个工具的版本
async fn detect_tool_version(command: &str, args: Vec<&str>) -> Option<String> {
    match Command::new(command).args(&args).output().await {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                // 清理版本号，只保留核心版本
                Some(version.trim().to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// 检测工具是否安装
async fn is_tool_installed(command: &str) -> bool {
    #[cfg(windows)]
    {
        // Windows 使用 where 命令
        Command::new("where")
            .arg(command)
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg(unix)]
    {
        // Unix-like 系统使用 which 命令
        Command::new("which")
            .arg(command)
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[tauri::command]
pub async fn detect_tools() -> Result<DetectToolsResponse, String> {
    let mut tools = Vec::new();

    // 1. 检测 Node.js
    let node_installed = is_tool_installed("node").await;
    let node_version = if node_installed {
        detect_tool_version("node", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Node.js".to_string(),
        is_installed: node_installed,
        version: node_version,
        install_url: Some("https://nodejs.org".to_string()),
    });

    // 2. 检测 npm
    let npm_installed = is_tool_installed("npm").await;
    let npm_version = if npm_installed {
        detect_tool_version("npm", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "npm".to_string(),
        is_installed: npm_installed,
        version: npm_version,
        install_url: Some("https://www.npmjs.com".to_string()),
    });

    // 3. 检测 pnpm (可选)
    let pnpm_installed = is_tool_installed("pnpm").await;
    let pnpm_version = if pnpm_installed {
        detect_tool_version("pnpm", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "pnpm".to_string(),
        is_installed: pnpm_installed,
        version: pnpm_version,
        install_url: Some("https://pnpm.io".to_string()),
    });

    // 4. 检测 Git
    let git_installed = is_tool_installed("git").await;
    let git_version = if git_installed {
        detect_tool_version("git", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Git".to_string(),
        is_installed: git_installed,
        version: git_version,
        install_url: Some("https://git-scm.com".to_string()),
    });

    // 5. 检测 Rust/Cargo
    let cargo_installed = is_tool_installed("cargo").await;
    let cargo_version = if cargo_installed {
        detect_tool_version("cargo", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Rust/Cargo".to_string(),
        is_installed: cargo_installed,
        version: cargo_version,
        install_url: Some("https://www.rust-lang.org".to_string()),
    });

    // 6. 检测 Kimi CLI (可选工具)
    let kimi_installed = is_tool_installed("kimi").await;
    let kimi_version = if kimi_installed {
        detect_tool_version("kimi", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Kimi CLI".to_string(),
        is_installed: kimi_installed,
        version: kimi_version,
        install_url: Some("https://www.moonshot.cn/docs/cli".to_string()),
    });

    // 7. 检测 Claude Code (可选工具)
    let claude_installed = is_tool_installed("claude").await;
    let claude_version = if claude_installed {
        detect_tool_version("claude", vec!["--version"]).await
    } else {
        None
    };

    tools.push(ToolStatus {
        name: "Claude Code".to_string(),
        is_installed: claude_installed,
        version: claude_version,
        install_url: Some(
            "https://docs.anthropic.com/en/docs/agents-and-tools/claude-code".to_string(),
        ),
    });

    Ok(DetectToolsResponse { tools })
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLITool {
    pub id: String,
    pub name: String,
    pub command: String,
    pub install_url: String,
    pub description: String,
    pub features: Vec<String>,
    pub protocol: String,
}

pub fn get_cli_tools() -> Vec<CLITool> {
    vec![
        CLITool {
            id: "kimi".to_string(),
            name: "Kimi CLI".to_string(),
            command: "kimi".to_string(),
            install_url: "https://www.moonshot.cn/docs/cli".to_string(),
            description: "月之暗面官方CLI工具，中文支持优秀".to_string(),
            features: vec!["code_generation".to_string(), "file_edit".to_string(), "shell_command".to_string()],
            protocol: "stdio".to_string(),
        },
        CLITool {
            id: "claude".to_string(),
            name: "Claude Code".to_string(),
            command: "claude".to_string(),
            install_url: "https://docs.anthropic.com/en/docs/agents-and-tools/claude-code".to_string(),
            description: "Anthropic官方CLI工具，英文场景强大".to_string(),
            features: vec!["code_generation".to_string(), "file_edit".to_string(), "shell_command".to_string(), "git_integration".to_string()],
            protocol: "stdio".to_string(),
        },
        CLITool {
            id: "codex".to_string(),
            name: "OpenAI Codex CLI".to_string(),
            command: "codex".to_string(),
            install_url: "https://github.com/openai/codex".to_string(),
            description: "OpenAI官方CLI工具，基于GPT-4o".to_string(),
            features: vec!["code_generation".to_string(), "file_edit".to_string(), "shell_command".to_string(), "mcp_support".to_string()],
            protocol: "mcp".to_string(),
        },
    ]
}

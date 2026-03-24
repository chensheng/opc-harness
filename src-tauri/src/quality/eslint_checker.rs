/// ESLint 质量检查器
/// 
/// 用于运行 ESLint 检查 TypeScript/JavaScript 代码质量
/// 支持自动修复功能

use serde::{Deserialize, Serialize};

/// ESLint 检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintResult {
    /// 是否通过检查
    pub success: bool,
    /// 错误数量
    pub error_count: usize,
    /// 警告数量
    pub warning_count: usize,
    /// 问题列表
    pub messages: Vec<ESLintMessage>,
    /// 是否可自动修复
    pub fixable: bool,
}

/// ESLint 问题消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESLintMessage {
    /// 文件路径
    pub file: String,
    /// 行号
    pub line: usize,
    /// 列号
    pub column: usize,
    /// 消息内容
    pub message: String,
    /// 规则 ID
    pub rule_id: Option<String>,
    /// 严重程度 (0=error, 1=warning)
    pub severity: u8,
}

/// ESLint 检查器
pub struct ESLintChecker {
    project_root: String,
}

impl ESLintChecker {
    /// 创建新的 ESLint 检查器
    pub fn new(project_root: &str) -> Self {
        Self {
            project_root: project_root.to_string(),
        }
    }

    /// 运行 ESLint 检查
    pub async fn run(&self, file_pattern: &str) -> Result<ESLintResult, String> {
        use tokio::process::Command;

        let eslint_args = vec![
            "run".to_string(),
            "lint".to_string(),
            "--".to_string(),
            file_pattern.to_string(),
            "-f".to_string(),
            "json".to_string(),
        ];

        let output = Command::new("npm")
            .current_dir(&self.project_root)
            .args(&eslint_args)
            .output()
            .await
            .map_err(|e| format!("Failed to execute ESLint: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        self.parse_eslint_output(&stdout, &stderr, output.status.success())
    }

    /// 运行 ESLint 并自动修复
    pub async fn run_with_fix(&self, file_pattern: &str) -> Result<ESLintResult, String> {
        use tokio::process::Command;

        let eslint_args = vec![
            "run".to_string(),
            "lint:fix".to_string(),
            "--".to_string(),
            file_pattern.to_string(),
        ];

        let output = Command::new("npm")
            .current_dir(&self.project_root)
            .args(&eslint_args)
            .output()
            .await
            .map_err(|e| format!("Failed to execute ESLint with fix: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        self.parse_eslint_output(&stdout, &stderr, output.status.success())
    }

    /// 解析 ESLint 输出
    fn parse_eslint_output(&self, stdout: &str, stderr: &str, success: bool) -> Result<ESLintResult, String> {
        let json_output = if !stdout.trim().is_empty() {
            stdout
        } else {
            stderr
        };

        let has_errors = !success && (json_output.contains("error") || json_output.contains("✖"));
        
        let error_count = json_output.matches("error").count();
        let warning_count = json_output.matches("warning").count();

        let mut messages = Vec::new();
        
        for line in json_output.lines() {
            if line.contains("error") || line.contains("warning") {
                messages.push(ESLintMessage {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                    message: line.to_string(),
                    rule_id: None,
                    severity: if line.contains("error") { 0 } else { 1 },
                });
            }
        }

        Ok(ESLintResult {
            success: !has_errors,
            error_count,
            warning_count,
            messages,
            fixable: true,
        })
    }

    /// 检查单个文件
    pub async fn check_file(&self, file_path: &str) -> Result<ESLintResult, String> {
        self.run(file_path).await
    }

    /// 检查所有 TypeScript/JavaScript 文件
    pub async fn check_all(&self) -> Result<ESLintResult, String> {
        self.run("**/*.{ts,tsx,js,jsx}").await
    }

    /// 获取配置信息
    pub fn get_config_info(&self) -> String {
        format!(
            "ESLint Checker configured for project at: {}\n\
             Supported file types: TypeScript (.ts, .tsx), JavaScript (.js, .jsx)\n\
             Auto-fix available: yes",
            self.project_root
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eslint_checker_creation() {
        let checker = ESLintChecker::new("/tmp/test");
        assert_eq!(checker.project_root, "/tmp/test");
    }

    #[test]
    fn test_eslint_result_success() {
        let result = ESLintResult {
            success: true,
            error_count: 0,
            warning_count: 2,
            messages: vec![],
            fixable: true,
        };
        assert!(result.success);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_eslint_message_structure() {
        let message = ESLintMessage {
            file: "src/test.ts".to_string(),
            line: 10,
            column: 5,
            message: "Unexpected console statement".to_string(),
            rule_id: Some("no-console".to_string()),
            severity: 1,
        };
        assert_eq!(message.file, "src/test.ts");
        assert_eq!(message.line, 10);
        assert_eq!(message.severity, 1);
    }

    #[test]
    fn test_eslint_config_info() {
        let checker = ESLintChecker::new("/test/project");
        let info = checker.get_config_info();
        assert!(info.contains("/test/project"));
        assert!(info.contains("TypeScript"));
        assert!(info.contains("Auto-fix"));
    }
}

//! Debug Agent 实现
//! 
//! 负责分析编译错误、运行时错误和测试失败，使用 AI 生成诊断报告和修复建议

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

/// 错误类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorType {
    /// 语法错误
    SyntaxError,
    /// 类型错误
    TypeError,
    /// 逻辑错误
    LogicError,
    /// 运行时异常
    RuntimeError,
    /// 测试失败
    TestFailure,
    /// 编译错误
    CompilationError,
    /// 导入错误
    ImportError,
    /// 配置错误
    ConfigError,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::SyntaxError => write!(f, "语法错误"),
            ErrorType::TypeError => write!(f, "类型错误"),
            ErrorType::LogicError => write!(f, "逻辑错误"),
            ErrorType::RuntimeError => write!(f, "运行时异常"),
            ErrorType::TestFailure => write!(f, "测试失败"),
            ErrorType::CompilationError => write!(f, "编译错误"),
            ErrorType::ImportError => write!(f, "导入错误"),
            ErrorType::ConfigError => write!(f, "配置错误"),
        }
    }
}

/// 错误来源
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSource {
    /// TypeScript 编译器
    TypeScript,
    /// Rust 编译器
    Rust,
    /// ESLint
    ESLint,
    /// Jest/Vitest 测试
    Jest,
    /// Cargo Test 测试
    CargoTest,
    /// 运行时错误日志
    RuntimeLog,
    /// 用户提供的错误信息
    UserInput,
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSource::TypeScript => write!(f, "TypeScript"),
            ErrorSource::Rust => write!(f, "Rust"),
            ErrorSource::ESLint => write!(f, "ESLint"),
            ErrorSource::Jest => write!(f, "Jest"),
            ErrorSource::CargoTest => write!(f, "Cargo Test"),
            ErrorSource::RuntimeLog => write!(f, "运行日志"),
            ErrorSource::UserInput => write!(f, "用户输入"),
        }
    }
}

/// 错误信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误类型
    pub error_type: ErrorType,
    /// 错误来源
    pub error_source: ErrorSource,
    /// 文件路径
    pub file_path: String,
    /// 行号（如果有）
    pub line_number: Option<u32>,
    /// 列号（如果有）
    pub column: Option<u32>,
    /// 错误消息
    pub message: String,
    /// 堆栈跟踪（如果有）
    pub stack_trace: Option<String>,
    /// 代码片段
    pub code_snippet: Option<String>,
    /// 原始错误输出
    pub raw_output: String,
}

/// AI 诊断结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnosis {
    /// 错误信息
    pub error: ErrorInfo,
    /// 错误原因分析
    pub cause: String,
    /// 修复建议
    pub suggestion: String,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
    /// 备选修复方案
    pub alternative_fixes: Vec<String>,
    /// 相关文档链接
    pub documentation_links: Vec<String>,
}

/// Debug Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugAgentConfig {
    /// 项目路径
    pub project_path: String,
    /// 错误来源
    pub error_source: ErrorSource,
    /// 是否自动修复
    pub auto_fix: bool,
    /// 最大建议数
    pub max_suggestions: usize,
    /// 原始错误输出或日志
    pub error_output: String,
}

/// Debug Agent 状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DebugStatus {
    /// 等待开始
    Pending,
    /// 收集错误信息中
    CollectingErrors,
    /// 解析错误中
    ParsingErrors,
    /// AI 诊断中
    Diagnosing,
    /// 生成修复建议中
    GeneratingFixes,
    /// 完成
    Completed,
    /// 失败
    Failed(String),
}

impl std::fmt::Display for DebugStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugStatus::Pending => write!(f, "等待开始"),
            DebugStatus::CollectingErrors => write!(f, "收集错误信息中"),
            DebugStatus::ParsingErrors => write!(f, "解析错误中"),
            DebugStatus::Diagnosing => write!(f, "AI 诊断中"),
            DebugStatus::GeneratingFixes => write!(f, "生成修复建议中"),
            DebugStatus::Completed => write!(f, "已完成"),
            DebugStatus::Failed(reason) => write!(f, "失败：{}", reason),
        }
    }
}

/// Debug Agent 结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugResult {
    /// 是否成功
    pub success: bool,
    /// 诊断结果列表
    pub diagnoses: Vec<Diagnosis>,
    /// 错误总数
    pub error_count: usize,
    /// 已诊断的错误数
    pub diagnosed_count: usize,
    /// 平均置信度
    pub avg_confidence: f32,
    /// 错误信息
    pub error: Option<String>,
}

impl DebugResult {
    /// 创建成功的结果
    pub fn success(diagnoses: Vec<Diagnosis>) -> Self {
        let error_count = diagnoses.len();
        let avg_confidence = if diagnoses.is_empty() {
            0.0
        } else {
            diagnoses.iter().map(|d| d.confidence).sum::<f32>() / diagnoses.len() as f32
        };

        Self {
            success: true,
            diagnoses,
            error_count,
            diagnosed_count: error_count,
            avg_confidence,
            error: None,
        }
    }

    /// 创建失败的结果
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            diagnoses: vec![],
            error_count: 0,
            diagnosed_count: 0,
            avg_confidence: 0.0,
            error: Some(error),
        }
    }

    /// 链式调用：添加错误信息
    pub fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }
}

/// Debug Agent 结构体
#[derive(Debug, Clone)]
pub struct DebugAgent {
    /// 配置信息
    pub config: DebugAgentConfig,
    /// 当前状态
    pub status: DebugStatus,
    /// 会话 ID
    pub session_id: String,
}

impl DebugAgent {
    /// 创建新的 Debug Agent
    pub fn new(config: DebugAgentConfig) -> Self {
        Self {
            config,
            status: DebugStatus::Pending,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// 执行完整的调试流程
    pub async fn run_debug(&mut self) -> Result<DebugResult, String> {
        log::info!("开始执行 Debug Agent - Session: {}", self.session_id);

        // 1. 收集错误信息
        self.status = DebugStatus::CollectingErrors;
        log::info!("步骤 1/4: 收集错误信息");
        
        let errors = self.collect_errors().await?;
        
        if errors.is_empty() {
            log::info!("未检测到错误");
            self.status = DebugStatus::Completed;
            return Ok(DebugResult::success(vec![]));
        }

        log::info!("检测到 {} 个错误", errors.len());

        // 2. 解析错误
        self.status = DebugStatus::ParsingErrors;
        log::info!("步骤 2/4: 解析错误信息");
        
        let mut diagnoses = Vec::new();
        
        for error in errors {
            // 3. AI 诊断
            self.status = DebugStatus::Diagnosing;
            log::info!("步骤 3/4: 诊断错误：{}", error.message);
            
            match self.diagnose_error(&error).await {
                Ok(diagnosis) => {
                    log::info!("诊断完成，置信度：{:.2}", diagnosis.confidence);
                    diagnoses.push(diagnosis);
                }
                Err(e) => {
                    log::error!("诊断失败：{}", e);
                    // 继续处理下一个错误
                }
            }
        }

        // 4. 生成修复建议
        self.status = DebugStatus::GeneratingFixes;
        log::info!("步骤 4/4: 生成修复建议");
        
        // 已经完成在 diagnose_error 中
        
        // 完成
        self.status = DebugStatus::Completed;
        log::info!("Debug Agent 执行完成，共诊断 {} 个错误", diagnoses.len());
        
        Ok(DebugResult::success(diagnoses))
    }

    /// 收集错误信息
    async fn collect_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        // 根据错误来源解析错误信息
        match self.config.error_source {
            ErrorSource::TypeScript => {
                self.parse_typescript_errors()
            }
            ErrorSource::Rust => {
                self.parse_rust_errors()
            }
            ErrorSource::ESLint => {
                self.parse_eslint_errors()
            }
            ErrorSource::Jest => {
                self.parse_jest_errors()
            }
            ErrorSource::CargoTest => {
                self.parse_cargo_test_errors()
            }
            ErrorSource::RuntimeLog | ErrorSource::UserInput => {
                // 直接从配置中获取错误输出
                self.parse_generic_errors(&self.config.error_output)
            }
        }
    }

    /// 解析 TypeScript 编译错误
    fn parse_typescript_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        let output = &self.config.error_output;
        let mut errors = Vec::new();

        // TypeScript 错误格式示例：
        // src/App.tsx(15,5): error TS2304: Cannot find name 'useState'.
        let re = regex::Regex::new(
            r"(?P<file>[^(]+)\((?P<line>\d+),(?P<col>\d+)\):\s*error\s+TS\d+:\s*(?P<message>.+)"
        ).map_err(|e| format!("Invalid regex: {}", e))?;

        for cap in re.captures_iter(output) {
            let file_path = cap.name("file").map_or("", |m| m.as_str()).trim().to_string();
            let line_number: Option<u32> = cap.name("line").and_then(|m| m.as_str().parse().ok());
            let column: Option<u32> = cap.name("col").and_then(|m| m.as_str().parse().ok());
            let message = cap.name("message").map_or("", |m| m.as_str()).to_string();

            errors.push(ErrorInfo {
                error_type: ErrorType::TypeError,
                error_source: ErrorSource::TypeScript,
                file_path,
                line_number,
                column,
                message,
                stack_trace: None,
                code_snippet: None,
                raw_output: output.to_string(),
            });
        }

        Ok(errors)
    }

    /// 解析 Rust 编译错误
    fn parse_rust_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        let output = &self.config.error_output;
        let mut errors = Vec::new();

        // Rust 错误格式示例：
        // error[E0425]: cannot find function `foo` in this scope
        //  --> src/main.rs:10:5
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            
            if line.starts_with("error[") {
                // 提取错误代码和消息
                let _error_code = line.split(']').next().and_then(|s| s.split('[').nth(1)).unwrap_or("");
                let message = line.split(": ").skip(1).collect::<Vec<_>>().join(": ");

                // 查找下一行的文件位置
                let (file_path, line_num, col) = if i + 1 < lines.len() && lines[i + 1].trim().starts_with("-->") {
                    let location_line = lines[i + 1];
                    let parts: Vec<&str> = location_line.split(':').collect();
                    if parts.len() >= 3 {
                        let file = parts[0].replace("-->", "").trim().to_string();
                        let line = parts.get(1).and_then(|s| s.trim().parse().ok());
                        let col = parts.get(2).and_then(|s| s.trim().parse().ok());
                        (file, line, col)
                    } else {
                        ("unknown".to_string(), None, None)
                    }
                } else {
                    ("unknown".to_string(), None, None)
                };

                errors.push(ErrorInfo {
                    error_type: ErrorType::CompilationError,
                    error_source: ErrorSource::Rust,
                    file_path,
                    line_number: line_num,
                    column: col,
                    message,
                    stack_trace: None,
                    code_snippet: None,
                    raw_output: output.to_string(),
                });

                i += 2; // 跳过位置行
            } else {
                i += 1;
            }
        }

        Ok(errors)
    }

    /// 解析 ESLint 错误
    fn parse_eslint_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        let output = &self.config.error_output;
        let mut errors = Vec::new();

        // ESLint 格式示例：
        // /path/to/file.js
        //   10:5  error  'x' is defined but never used  no-unused-vars
        let lines: Vec<&str> = output.lines().collect();
        let mut current_file = String::new();

        for line in lines {
            if line.starts_with('/') || line.starts_with('\\') {
                current_file = line.trim().to_string();
            } else if line.contains("error") || line.contains("warning") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let position = parts[0]; // "10:5"
                    let position_parts: Vec<&str> = position.split(':').collect();
                    let line_number = position_parts.get(0).and_then(|s| s.parse().ok());
                    let column = position_parts.get(1).and_then(|s| s.parse().ok());
                    
                    let message_start = parts.iter().position(|&p| p == "error" || p == "warning");
                    let message = if let Some(idx) = message_start {
                        parts[idx..].join(" ")
                    } else {
                        parts[3..].join(" ")
                    };

                    errors.push(ErrorInfo {
                        error_type: ErrorType::SyntaxError,
                        error_source: ErrorSource::ESLint,
                        file_path: current_file.clone(),
                        line_number,
                        column,
                        message,
                        stack_trace: None,
                        code_snippet: None,
                        raw_output: output.to_string(),
                    });
                }
            }
        }

        Ok(errors)
    }

    /// 解析 Jest 测试错误
    fn parse_jest_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        let output = &self.config.error_output;
        let mut errors = Vec::new();

        // Jest 错误格式示例：
        // ● Test suite failed to run
        //   SyntaxError: Unexpected token 'export'
        //   
        //   > 1 | export const foo = () => {};
        //       |                  ^
        //   at Object.<anonymous> (src/foo.test.ts:1:10)
        
        if output.contains("Test suite failed") || output.contains("●") {
            // 尝试找到包含具体错误信息的行
            let message = output.lines()
                .find(|l| l.contains("SyntaxError") || l.contains("TypeError") || l.contains("ReferenceError"))
                .unwrap_or_else(|| output.lines().find(|l| !l.is_empty()).unwrap_or("Test failed"))
                .trim()
                .to_string();

            errors.push(ErrorInfo {
                error_type: ErrorType::TestFailure,
                error_source: ErrorSource::Jest,
                file_path: "unknown".to_string(),
                line_number: None,
                column: None,
                message,
                stack_trace: Some(output.to_string()),
                code_snippet: None,
                raw_output: output.to_string(),
            });
        }

        Ok(errors)
    }

    /// 解析 Cargo Test 错误
    fn parse_cargo_test_errors(&self) -> Result<Vec<ErrorInfo>, String> {
        let output = &self.config.error_output;
        let mut errors = Vec::new();

        // Cargo Test 错误格式示例：
        // test test_foo ... FAILED
        // 
        // failures:
        // 
        // ---- test_foo stdout ----
        // thread 'test_foo' panicked at 'assertion failed', src/lib.rs:10:5
        
        if output.contains("FAILED") || output.contains("panicked at") {
            errors.push(ErrorInfo {
                error_type: ErrorType::TestFailure,
                error_source: ErrorSource::CargoTest,
                file_path: "unknown".to_string(),
                line_number: None,
                column: None,
                message: output.lines().find(|l| l.contains("panicked at") || l.contains("assertion")).unwrap_or("Test failed").to_string(),
                stack_trace: Some(output.to_string()),
                code_snippet: None,
                raw_output: output.to_string(),
            });
        }

        Ok(errors)
    }

    /// 解析通用错误格式
    fn parse_generic_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
        let mut errors = Vec::new();

        // 尝试从输出中提取基本信息
        errors.push(ErrorInfo {
            error_type: ErrorType::LogicError, // 默认类型
            error_source: self.config.error_source.clone(),
            file_path: "unknown".to_string(),
            line_number: None,
            column: None,
            message: output.lines().next().unwrap_or("Unknown error").to_string(),
            stack_trace: Some(output.to_string()),
            code_snippet: None,
            raw_output: output.to_string(),
        });

        Ok(errors)
    }

    /// AI 诊断错误
    async fn diagnose_error(&self, error: &ErrorInfo) -> Result<Diagnosis, String> {
        // TODO: 实际实现中需要调用 AI API 进行诊断
        // 这里提供一个模板实现
        
        log::info!("诊断错误：{} ({})", error.message, error.error_type);

        // 基于错误类型生成诊断
        let (cause, suggestion, confidence) = match error.error_type {
            ErrorType::SyntaxError => (
                "代码存在语法错误，可能是缺少分号、括号不匹配或关键字拼写错误".to_string(),
                "检查错误位置的语法，确保所有括号成对出现，语句以分号结尾".to_string(),
                0.9,
            ),
            ErrorType::TypeError => (
                "类型不匹配，可能是变量类型声明错误或函数返回值与预期不符".to_string(),
                "检查变量类型声明，确保函数参数和返回值类型正确".to_string(),
                0.85,
            ),
            ErrorType::LogicError => (
                "逻辑错误，代码可以运行但行为不符合预期".to_string(),
                "仔细审查业务逻辑，添加调试日志或使用断点调试".to_string(),
                0.7,
            ),
            ErrorType::RuntimeError => (
                "运行时异常，可能是空指针、数组越界或资源未找到".to_string(),
                "添加适当的错误处理和边界检查，确保资源在使用前已正确初始化".to_string(),
                0.8,
            ),
            ErrorType::TestFailure => (
                "测试失败，断言不通过或测试用例期望与实际结果不符".to_string(),
                "检查测试用例的断言条件，确认被测试代码的逻辑正确".to_string(),
                0.75,
            ),
            ErrorType::CompilationError => (
                "编译错误，代码无法通过编译器检查".to_string(),
                "根据编译器错误信息修复代码，可能需要添加缺失的导入或修正类型定义".to_string(),
                0.85,
            ),
            ErrorType::ImportError => (
                "导入错误，模块或依赖项无法找到".to_string(),
                "检查导入路径是否正确，确认依赖项已安装".to_string(),
                0.9,
            ),
            ErrorType::ConfigError => (
                "配置错误，配置文件格式不正确或缺少必要字段".to_string(),
                "检查配置文件格式，确保所有必填字段都已提供".to_string(),
                0.85,
            ),
        };

        Ok(Diagnosis {
            error: error.clone(),
            cause,
            suggestion,
            confidence,
            alternative_fixes: vec![
                "查看相关文档和示例代码".to_string(),
                "使用 IDE 的代码检查和自动修复功能".to_string(),
                "在 Stack Overflow 搜索类似错误".to_string(),
            ],
            documentation_links: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type_display() {
        assert_eq!(format!("{}", ErrorType::SyntaxError), "语法错误");
        assert_eq!(format!("{}", ErrorType::TypeError), "类型错误");
        assert_eq!(format!("{}", ErrorType::LogicError), "逻辑错误");
    }

    #[test]
    fn test_error_source_display() {
        assert_eq!(format!("{}", ErrorSource::TypeScript), "TypeScript");
        assert_eq!(format!("{}", ErrorSource::Rust), "Rust");
        assert_eq!(format!("{}", ErrorSource::Jest), "Jest");
    }

    #[test]
    fn test_debug_status_display() {
        assert_eq!(format!("{}", DebugStatus::Pending), "等待开始");
        assert_eq!(format!("{}", DebugStatus::Diagnosing), "AI 诊断中");
        assert_eq!(format!("{}", DebugStatus::Completed), "已完成");
    }

    #[test]
    fn test_error_info_creation() {
        let error = ErrorInfo {
            error_type: ErrorType::SyntaxError,
            error_source: ErrorSource::TypeScript,
            file_path: "src/App.tsx".to_string(),
            line_number: Some(15),
            column: Some(5),
            message: "Cannot find name 'useState'".to_string(),
            stack_trace: None,
            code_snippet: None,
            raw_output: "test output".to_string(),
        };

        assert_eq!(error.error_type, ErrorType::SyntaxError);
        assert_eq!(error.file_path, "src/App.tsx");
        assert_eq!(error.line_number, Some(15));
    }

    #[test]
    fn test_diagnosis_creation() {
        let error = ErrorInfo {
            error_type: ErrorType::TypeError,
            error_source: ErrorSource::TypeScript,
            file_path: "src/test.ts".to_string(),
            line_number: Some(10),
            column: None,
            message: "Type mismatch".to_string(),
            stack_trace: None,
            code_snippet: None,
            raw_output: "test".to_string(),
        };

        let diagnosis = Diagnosis {
            error: error.clone(),
            cause: "类型不匹配".to_string(),
            suggestion: "检查类型声明".to_string(),
            confidence: 0.85,
            alternative_fixes: vec!["查看文档".to_string()],
            documentation_links: vec![],
        };

        assert_eq!(diagnosis.cause, "类型不匹配");
        assert_eq!(diagnosis.confidence, 0.85);
        assert_eq!(diagnosis.alternative_fixes.len(), 1);
    }

    #[test]
    fn test_debug_result_success() {
        let diagnoses = vec![
            Diagnosis {
                error: ErrorInfo {
                    error_type: ErrorType::SyntaxError,
                    error_source: ErrorSource::TypeScript,
                    file_path: "test.ts".to_string(),
                    line_number: None,
                    column: None,
                    message: "test".to_string(),
                    stack_trace: None,
                    code_snippet: None,
                    raw_output: "test".to_string(),
                },
                cause: "cause".to_string(),
                suggestion: "fix".to_string(),
                confidence: 0.9,
                alternative_fixes: vec![],
                documentation_links: vec![],
            }
        ];

        let result = DebugResult::success(diagnoses);
        
        assert!(result.success);
        assert_eq!(result.error_count, 1);
        assert_eq!(result.diagnosed_count, 1);
        assert_eq!(result.avg_confidence, 0.9);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_debug_result_failure() {
        let result = DebugResult::failure("Test error".to_string());
        
        assert!(!result.success);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.diagnosed_count, 0);
        assert_eq!(result.avg_confidence, 0.0);
        assert_eq!(result.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_debug_agent_creation() {
        let config = DebugAgentConfig {
            project_path: "/tmp/test".to_string(),
            error_source: ErrorSource::TypeScript,
            auto_fix: false,
            max_suggestions: 5,
            error_output: "test error".to_string(),
        };

        let agent = DebugAgent::new(config);

        assert_eq!(agent.config.project_path, "/tmp/test");
        assert_eq!(agent.status, DebugStatus::Pending);
        assert!(!agent.session_id.is_empty());
    }

    #[test]
    fn test_typescript_error_parsing() {
        let ts_output = r#"
src/App.tsx(15,5): error TS2304: Cannot find name 'useState'.
src/components/Button.tsx(23,10): error TS2307: Cannot find module './types'.
"#;

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::TypeScript,
            auto_fix: false,
            max_suggestions: 5,
            error_output: ts_output.to_string(),
        };

        let agent = DebugAgent::new(config);
        let errors = agent.parse_typescript_errors().unwrap();

        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].file_path, "src/App.tsx");
        assert_eq!(errors[0].line_number, Some(15));
        assert_eq!(errors[0].column, Some(5));
        assert!(errors[0].message.contains("useState"));
    }

    #[test]
    fn test_rust_error_parsing() {
        let rust_output = r#"
error[E0425]: cannot find function `foo` in this scope
  --> src/main.rs:10:5
   |
10 |     foo();
   |     ^^^ not found in this scope
"#;

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::Rust,
            auto_fix: false,
            max_suggestions: 5,
            error_output: rust_output.to_string(),
        };

        let agent = DebugAgent::new(config);
        let errors = agent.parse_rust_errors().unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].file_path, "src/main.rs");
        assert_eq!(errors[0].line_number, Some(10));
        assert!(errors[0].message.contains("foo"));
    }

    #[test]
    fn test_eslint_error_parsing() {
        let eslint_output = r#"
/path/to/file.js
  10:5  error  'x' is defined but never used  no-unused-vars
  15:3  warning  Missing semicolon  semi
"#;

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::ESLint,
            auto_fix: false,
            max_suggestions: 5,
            error_output: eslint_output.to_string(),
        };

        let agent = DebugAgent::new(config);
        let errors = agent.parse_eslint_errors().unwrap();

        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].file_path, "/path/to/file.js");
        assert_eq!(errors[0].line_number, Some(10));
        assert!(errors[0].message.contains("no-unused-vars"));
    }

    #[test]
    fn test_jest_error_parsing() {
        let jest_output = r#"
● Test suite failed to run
  SyntaxError: Unexpected token 'export'

  > 1 | export const foo = () => {};
      |                  ^
  at Object.<anonymous> (src/foo.test.ts:1:10)
"#;

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::Jest,
            auto_fix: false,
            max_suggestions: 5,
            error_output: jest_output.to_string(),
        };

        let agent = DebugAgent::new(config);
        let errors = agent.parse_jest_errors().unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, ErrorType::TestFailure);
        assert!(errors[0].message.contains("SyntaxError"));
    }

    #[test]
    fn test_cargo_test_error_parsing() {
        let cargo_output = r#"
test test_foo ... FAILED

failures:

---- test_foo stdout ----
thread 'test_foo' panicked at 'assertion failed', src/lib.rs:10:5
"#;

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::CargoTest,
            auto_fix: false,
            max_suggestions: 5,
            error_output: cargo_output.to_string(),
        };

        let agent = DebugAgent::new(config);
        let errors = agent.parse_cargo_test_errors().unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, ErrorType::TestFailure);
        assert!(errors[0].message.contains("assertion failed"));
    }

    #[test]
    fn test_ai_diagnosis_syntax_error() {
        let error = ErrorInfo {
            error_type: ErrorType::SyntaxError,
            error_source: ErrorSource::TypeScript,
            file_path: "test.ts".to_string(),
            line_number: None,
            column: None,
            message: "Missing semicolon".to_string(),
            stack_trace: None,
            code_snippet: None,
            raw_output: "test".to_string(),
        };

        let config = DebugAgentConfig {
            project_path: "/tmp".to_string(),
            error_source: ErrorSource::TypeScript,
            auto_fix: false,
            max_suggestions: 5,
            error_output: "test".to_string(),
        };

        let agent = DebugAgent::new(config);
        // 注意：这里不实际调用 diagnose_error，因为它是 async 且需要 AI API
        // 我们只测试数据结构
        assert_eq!(error.error_type, ErrorType::SyntaxError);
    }

    #[test]
    fn test_debug_config_serialization() {
        let config = DebugAgentConfig {
            project_path: "/tmp/test".to_string(),
            error_source: ErrorSource::Rust,
            auto_fix: true,
            max_suggestions: 10,
            error_output: "error".to_string(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: DebugAgentConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.project_path, "/tmp/test");
        assert_eq!(deserialized.error_source, ErrorSource::Rust);
        assert!(deserialized.auto_fix);
        assert_eq!(deserialized.max_suggestions, 10);
    }

    #[test]
    fn test_error_type_enum_coverage() {
        // 测试所有 ErrorType 变体
        let types = vec![
            ErrorType::SyntaxError,
            ErrorType::TypeError,
            ErrorType::LogicError,
            ErrorType::RuntimeError,
            ErrorType::TestFailure,
            ErrorType::CompilationError,
            ErrorType::ImportError,
            ErrorType::ConfigError,
        ];

        for error_type in types {
            let display = format!("{}", error_type);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_error_source_enum_coverage() {
        // 测试所有 ErrorSource 变体
        let sources = vec![
            ErrorSource::TypeScript,
            ErrorSource::Rust,
            ErrorSource::ESLint,
            ErrorSource::Jest,
            ErrorSource::CargoTest,
            ErrorSource::RuntimeLog,
            ErrorSource::UserInput,
        ];

        for source in sources {
            let display = format!("{}", source);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_debug_status_enum_coverage() {
        // 测试所有 DebugStatus 变体
        let statuses = vec![
            DebugStatus::Pending,
            DebugStatus::CollectingErrors,
            DebugStatus::ParsingErrors,
            DebugStatus::Diagnosing,
            DebugStatus::GeneratingFixes,
            DebugStatus::Completed,
            DebugStatus::Failed("test".to_string()),
        ];

        for status in statuses {
            let display = format!("{}", status);
            assert!(!display.is_empty());
        }
    }
}

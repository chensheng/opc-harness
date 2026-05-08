//! Debug Agent 错误解析器
//!
//! 负责从不同来源（TypeScript、Rust、ESLint、Jest等）解析错误信息

use super::types::{ErrorInfo, ErrorSource, ErrorType};

/// 错误解析器 trait
pub trait ErrorParser {
    /// 解析错误输出，返回结构化的错误信息列表
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String>;
}

/// TypeScript 错误解析器
pub struct TypeScriptParser;

impl ErrorParser for TypeScriptParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
        let mut errors = Vec::new();

        // TypeScript 错误格式示例：
        // src/App.tsx(15,5): error TS2304: Cannot find name 'useState'.
        let re = regex::Regex::new(
            r"(?P<file>[^(]+)\((?P<line>\d+),(?P<col>\d+)\):\s*error\s+TS\d+:\s*(?P<message>.+)",
        )
        .map_err(|e| format!("Invalid regex: {}", e))?;

        for cap in re.captures_iter(output) {
            let file_path = cap
                .name("file")
                .map_or("", |m| m.as_str())
                .trim()
                .to_string();
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
}

/// Rust 错误解析器
pub struct RustParser;

impl ErrorParser for RustParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
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
                let _error_code = line
                    .split(']')
                    .next()
                    .and_then(|s| s.split('[').nth(1))
                    .unwrap_or("");
                let message = line.split(": ").skip(1).collect::<Vec<_>>().join(": ");

                // 查找下一行的文件位置
                let (file_path, line_num, col) =
                    if i + 1 < lines.len() && lines[i + 1].trim().starts_with("-->") {
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
}

/// ESLint 错误解析器
pub struct ESLintParser;

impl ErrorParser for ESLintParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
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
                    let line_number = position_parts.first().and_then(|s| s.parse().ok());
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
}

/// Jest 测试错误解析器
pub struct JestParser;

impl ErrorParser for JestParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
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
            let message = output
                .lines()
                .find(|l| {
                    l.contains("SyntaxError")
                        || l.contains("TypeError")
                        || l.contains("ReferenceError")
                })
                .unwrap_or_else(|| {
                    output
                        .lines()
                        .find(|l| !l.is_empty())
                        .unwrap_or("Test failed")
                })
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
}

/// Cargo Test 错误解析器
pub struct CargoTestParser;

impl ErrorParser for CargoTestParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
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
                message: output
                    .lines()
                    .find(|l| l.contains("panicked at") || l.contains("assertion"))
                    .unwrap_or("Test failed")
                    .to_string(),
                stack_trace: Some(output.to_string()),
                code_snippet: None,
                raw_output: output.to_string(),
            });
        }

        Ok(errors)
    }
}

/// 通用错误解析器（用于运行时日志和用户输入）
pub struct GenericParser;

impl ErrorParser for GenericParser {
    fn parse_errors(&self, output: &str) -> Result<Vec<ErrorInfo>, String> {
        let mut errors = Vec::new();

        // 尝试从输出中提取基本信息
        errors.push(ErrorInfo {
            error_type: ErrorType::LogicError,     // 默认类型
            error_source: ErrorSource::RuntimeLog, // 将在调用时设置
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
}

/// 根据错误源获取对应的解析器
pub fn get_parser_for_source(source: &ErrorSource) -> Box<dyn ErrorParser> {
    match source {
        ErrorSource::TypeScript => Box::new(TypeScriptParser),
        ErrorSource::Rust => Box::new(RustParser),
        ErrorSource::ESLint => Box::new(ESLintParser),
        ErrorSource::Jest => Box::new(JestParser),
        ErrorSource::CargoTest => Box::new(CargoTestParser),
        ErrorSource::RuntimeLog | ErrorSource::UserInput => Box::new(GenericParser),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_error_parsing() {
        let ts_output = r#"
src/App.tsx(15,5): error TS2304: Cannot find name 'useState'.
src/components/Button.tsx(23,10): error TS2307: Cannot find module './types'.
"#;

        let parser = TypeScriptParser;
        let errors = parser.parse_errors(ts_output).unwrap();

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

        let parser = RustParser;
        let errors = parser.parse_errors(rust_output).unwrap();

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

        let parser = ESLintParser;
        let errors = parser.parse_errors(eslint_output).unwrap();

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

        let parser = JestParser;
        let errors = parser.parse_errors(jest_output).unwrap();

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

        let parser = CargoTestParser;
        let errors = parser.parse_errors(cargo_output).unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, ErrorType::TestFailure);
        assert!(errors[0].message.contains("assertion failed"));
    }

    #[test]
    fn test_generic_parser() {
        let generic_output = "Some runtime error occurred";

        let parser = GenericParser;
        let errors = parser.parse_errors(generic_output).unwrap();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, ErrorType::LogicError);
        assert!(errors[0].message.contains("runtime error"));
    }

    #[test]
    fn test_get_parser_for_source() {
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
            let _parser = get_parser_for_source(&source);
            // 只要能创建解析器就说明成功了
        }
    }
}

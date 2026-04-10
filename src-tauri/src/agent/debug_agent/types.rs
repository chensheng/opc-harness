//! Debug Agent 类型定义
//! 
//! 包含所有错误类型、配置和结果相关的结构体和枚举

use serde::{Deserialize, Serialize};

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

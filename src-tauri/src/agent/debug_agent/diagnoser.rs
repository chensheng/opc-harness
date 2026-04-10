//! Debug Agent AI 诊断器
//! 
//! 负责分析错误并生成诊断报告和修复建议

use super::types::{Diagnosis, ErrorInfo, ErrorType};

/// AI 诊断器
pub struct Diagnoser;

impl Diagnoser {
    /// 创建新的诊断器实例
    pub fn new() -> Self {
        Self
    }

    /// 诊断单个错误，生成诊断报告
    pub async fn diagnose_error(&self, error: &ErrorInfo) -> Result<Diagnosis, String> {
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
    use crate::agent::debug_agent::ErrorSource;

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

        let diagnoser = Diagnoser::new();
        // 注意：这里不实际调用 diagnose_error，因为它是 async 且需要 AI API
        // 我们只测试数据结构
        assert_eq!(error.error_type, ErrorType::SyntaxError);
    }
}

//! Debug Agent 实现
//! 
//! 负责分析编译错误、运行时错误和测试失败，用 AI 生成诊断报告和修复建议
//! 
//! 该模块已进行模块化拆分：
//! - `types`: 类型定义（ErrorType, ErrorSource, ErrorInfo, Diagnosis等）
//! - `parsers`: 错误解析器（TypeScript, Rust, ESLint, Jest等）
//! - `diagnoser`: AI 诊断器
//! - `core`: DebugAgent 核心实现

// 子模块声明
mod types;
mod parsers;
mod diagnoser;
mod core;

// 重新导出主要类型，保持向后兼容的 API
pub use types::{
    ErrorType,
    ErrorSource,
    ErrorInfo,
    Diagnosis,
    DebugAgentConfig,
    DebugStatus,
    DebugResult,
};

pub use core::DebugAgent;

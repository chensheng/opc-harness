//! 提示词模板模块
//! 
//! 提供各类 AI 提示词模板，用于不同的代码生成场景

pub mod code_generation;

// 重新导出常用项
pub use code_generation::*;

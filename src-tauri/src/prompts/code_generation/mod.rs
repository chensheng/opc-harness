//! 代码生成提示词模板
//! 
//! VC-019: 为 Coding Agent 提供标准化的代码生成提示词库
//! 
//! 支持场景：
//! - TypeScript/React: 组件、Hooks、类型定义、测试、样式
//! - Rust: 模块、Trait 实现、错误处理、测试、API 端点
//! - 通用：重构、Bug 修复、文档、代码审查

#![allow(dead_code)]

// 子模块
pub mod code_generation_types;
pub mod code_generation_typescript_templates;
pub mod code_generation_rust_templates;
pub mod code_generation_general_templates;
pub mod code_generation_utils;
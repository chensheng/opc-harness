//! 代码生成提示词 - 类型定义
//! 
//! VC-019: 为 Coding Agent 提供标准化的代码生成提示词库的类型定义

#![allow(dead_code)]

/// 编程语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeLanguage {
    TypeScript,
    Rust,
    Python,
    JavaScript,
}

impl std::fmt::Display for CodeLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLanguage::TypeScript => write!(f, "TypeScript"),
            CodeLanguage::Rust => write!(f, "Rust"),
            CodeLanguage::Python => write!(f, "Python"),
            CodeLanguage::JavaScript => write!(f, "JavaScript"),
        }
    }
}

/// 代码场景
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeScenario {
    // TypeScript/React 场景
    ComponentGeneration,
    HookGeneration,
    TypeDefinition,
    TestGeneration,
    StyleGeneration,
    
    // Rust 场景
    ModuleGeneration,
    TraitImplementation,
    ErrorHandling,
    ApiEndpoint,
    
    // 通用场景
    Refactoring,
    BugFixing,
    Documentation,
    CodeReview,
}

impl std::fmt::Display for CodeScenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeScenario::ComponentGeneration => "Component Generation",
            CodeScenario::HookGeneration => "Hook Generation",
            CodeScenario::TypeDefinition => "Type Definition",
            CodeScenario::TestGeneration => "Test Generation",
            CodeScenario::StyleGeneration => "Style Generation",
            CodeScenario::ModuleGeneration => "Module Generation",
            CodeScenario::TraitImplementation => "Trait Implementation",
            CodeScenario::ErrorHandling => "Error Handling",
            CodeScenario::ApiEndpoint => "API Endpoint",
            CodeScenario::Refactoring => "Refactoring",
            CodeScenario::BugFixing => "Bug Fixing",
            CodeScenario::Documentation => "Documentation",
            CodeScenario::CodeReview => "Code Review",
        };
        write!(f, "{}", name)
    }
}

/// 代码生成提示词模板
pub struct CodeGenPrompt {
    /// 模板名称
    pub name: &'static str,
    /// 适用语言
    pub language: CodeLanguage,
    /// 场景类型
    pub scenario: CodeScenario,
    /// 模板内容
    pub template: &'static str,
    /// 变量列表
    pub variables: Vec<&'static str>,
}

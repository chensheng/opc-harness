//! 代码生成提示词 - 工具函数和模板管理
//! 
//! VC-019: 提供模板获取、渲染等辅助功能

use super::code_generation_types::{CodeLanguage, CodeScenario, CodeGenPrompt};
use super::code_generation_typescript_templates::*;
use super::code_generation_rust_templates::*;
use super::code_generation_general_templates::*;

/// 获取所有代码生成提示词模板
pub fn get_all_code_gen_prompts() -> Vec<CodeGenPrompt> {
    vec![
        // TypeScript/React 模板
        CodeGenPrompt {
            name: "Component Generation",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::ComponentGeneration,
            template: COMPONENT_GENERATION_PROMPT,
            variables: vec!["component_name", "description", "props_interface", "usage_scenario"],
        },
        CodeGenPrompt {
            name: "Hook Generation",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::HookGeneration,
            template: HOOK_GENERATION_PROMPT,
            variables: vec!["hook_name", "description", "input_params", "return_value", "usage_example"],
        },
        CodeGenPrompt {
            name: "Type Definition",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::TypeDefinition,
            template: TYPE_DEFINITION_PROMPT,
            variables: vec!["business_context", "data_structure"],
        },
        CodeGenPrompt {
            name: "Test Generation",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::TestGeneration,
            template: TEST_GENERATION_PROMPT,
            variables: vec!["language", "code_to_test", "test_framework"],
        },
        CodeGenPrompt {
            name: "Style Generation",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::StyleGeneration,
            template: STYLE_GENERATION_PROMPT,
            variables: vec!["component_type", "design_style", "color_theme", "responsive_requirements"],
        },
        
        // Rust 模板
        CodeGenPrompt {
            name: "Rust Module Generation",
            language: CodeLanguage::Rust,
            scenario: CodeScenario::ModuleGeneration,
            template: RUST_MODULE_GENERATION_PROMPT,
            variables: vec!["module_name", "description", "structs", "methods", "error_types"],
        },
        CodeGenPrompt {
            name: "Rust Trait Implementation",
            language: CodeLanguage::Rust,
            scenario: CodeScenario::TraitImplementation,
            template: RUST_TRAIT_IMPLEMENTATION_PROMPT,
            variables: vec!["type_name", "type_definition", "trait_name", "methods", "special_requirements"],
        },
        CodeGenPrompt {
            name: "Rust Error Handling",
            language: CodeLanguage::Rust,
            scenario: CodeScenario::ErrorHandling,
            template: RUST_ERROR_HANDLING_PROMPT,
            variables: vec!["business_context", "possible_errors"],
        },
        CodeGenPrompt {
            name: "Rust API Endpoint",
            language: CodeLanguage::Rust,
            scenario: CodeScenario::ApiEndpoint,
            template: RUST_API_ENDPOINT_PROMPT,
            variables: vec!["command_name", "description", "input_params", "return_type", "side_effects"],
        },
        
        // 通用模板
        CodeGenPrompt {
            name: "Refactoring",
            language: CodeLanguage::TypeScript, // 通用，默认 TypeScript
            scenario: CodeScenario::Refactoring,
            template: REFACTORING_PROMPT,
            variables: vec!["language", "original_code", "refactoring_goals"],
        },
        CodeGenPrompt {
            name: "Bug Fixing",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::BugFixing,
            template: BUG_FIXING_PROMPT,
            variables: vec!["bug_description", "reproduction_steps", "expected_behavior", "actual_behavior", "language", "related_code"],
        },
        CodeGenPrompt {
            name: "Documentation",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::Documentation,
            template: DOCUMENTATION_PROMPT,
            variables: vec!["language", "code_content", "doc_type"],
        },
        CodeGenPrompt {
            name: "Code Review",
            language: CodeLanguage::TypeScript,
            scenario: CodeScenario::CodeReview,
            template: CODE_REVIEW_PROMPT,
            variables: vec!["language", "code_content"],
        },
    ]
}

/// 根据语言和场景获取提示词模板
pub fn get_prompt_by_language_and_scenario(
    language: CodeLanguage,
    scenario: CodeScenario,
) -> Option<CodeGenPrompt> {
    get_all_code_gen_prompts()
        .into_iter()
        .find(|p| p.language == language && p.scenario == scenario)
}

/// 渲染提示词模板（替换变量）
pub fn render_prompt(template: &str, variables: &[(&str, &str)]) -> String {
    let mut rendered = template.to_string();
    for (key, value) in variables {
        rendered = rendered.replace(&format!("{{{}}}", key), value);
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_prompts_count() {
        let prompts = get_all_code_gen_prompts();
        assert_eq!(prompts.len(), 13);
    }

    #[test]
    fn test_component_generation_prompt() {
        let prompt = get_prompt_by_language_and_scenario(
            CodeLanguage::TypeScript,
            CodeScenario::ComponentGeneration,
        );
        
        assert!(prompt.is_some());
        let prompt = prompt.unwrap();
        assert_eq!(prompt.name, "Component Generation");
        assert!(prompt.template.contains("React 组件"));
        assert!(prompt.variables.contains(&"component_name"));
    }

    #[test]
    fn test_rust_module_generation_prompt() {
        let prompt = get_prompt_by_language_and_scenario(
            CodeLanguage::Rust,
            CodeScenario::ModuleGeneration,
        );
        
        assert!(prompt.is_some());
        let prompt = prompt.unwrap();
        assert_eq!(prompt.name, "Rust Module Generation");
        assert!(prompt.template.contains("Rust 模块"));
        assert!(prompt.variables.contains(&"module_name"));
    }

    #[test]
    fn test_render_prompt_with_variables() {
        let template = "Hello {name}, you are {age} years old";
        let variables = vec![
            ("name", "Alice"),
            ("age", "25"),
        ];
        
        let rendered = render_prompt(template, &variables);
        
        assert_eq!(rendered, "Hello Alice, you are 25 years old");
    }

    #[test]
    fn test_render_prompt_with_missing_variables() {
        let template = "Hello {name}, you are {age} years old";
        let variables = vec![
            ("name", "Bob"),
        ];
        
        let rendered = render_prompt(template, &variables);
        
        assert!(rendered.contains("Hello Bob"));
        assert!(rendered.contains("{age}")); // 未替换的变量保持原样
    }

    #[test]
    fn test_all_prompts_have_valid_templates() {
        let prompts = get_all_code_gen_prompts();
        
        for prompt in prompts {
            assert!(!prompt.template.is_empty(), "Template {} should not be empty", prompt.name);
            assert!(prompt.template.len() > 100, "Template {} seems too short", prompt.name);
            assert!(!prompt.variables.is_empty(), "Template {} should have variables", prompt.name);
        }
    }

    #[test]
    fn test_language_display() {
        assert_eq!(CodeLanguage::TypeScript.to_string(), "TypeScript");
        assert_eq!(CodeLanguage::Rust.to_string(), "Rust");
        assert_eq!(CodeLanguage::Python.to_string(), "Python");
        assert_eq!(CodeLanguage::JavaScript.to_string(), "JavaScript");
    }

    #[test]
    fn test_scenario_display() {
        assert_eq!(CodeScenario::ComponentGeneration.to_string(), "Component Generation");
        assert_eq!(CodeScenario::ModuleGeneration.to_string(), "Module Generation");
        assert_eq!(CodeScenario::TestGeneration.to_string(), "Test Generation");
    }

    #[test]
    fn test_prompt_exists_for_each_scenario() {
        let scenarios = vec![
            CodeScenario::ComponentGeneration,
            CodeScenario::HookGeneration,
            CodeScenario::TypeDefinition,
            CodeScenario::TestGeneration,
            CodeScenario::StyleGeneration,
            CodeScenario::ModuleGeneration,
            CodeScenario::TraitImplementation,
            CodeScenario::ErrorHandling,
            CodeScenario::ApiEndpoint,
            CodeScenario::Refactoring,
            CodeScenario::BugFixing,
            CodeScenario::Documentation,
            CodeScenario::CodeReview,
        ];
        
        for scenario in scenarios {
            let found = get_all_code_gen_prompts()
                .iter()
                .any(|p| p.scenario == scenario);
            assert!(found, "No prompt found for scenario: {:?}", scenario);
        }
    }

    #[test]
    fn test_typescript_prompts() {
        let ts_prompts = get_all_code_gen_prompts()
            .into_iter()
            .filter(|p| p.language == CodeLanguage::TypeScript)
            .collect::<Vec<_>>();
        
        // TypeScript 应该有 5 个专用模板 + 4 个通用模板 = 9 个
        assert!(ts_prompts.len() >= 9);
    }

    #[test]
    fn test_rust_prompts() {
        let rust_prompts = get_all_code_gen_prompts()
            .into_iter()
            .filter(|p| p.language == CodeLanguage::Rust)
            .collect::<Vec<_>>();
        
        // Rust 应该有 4 个专用模板
        assert_eq!(rust_prompts.len(), 4);
    }
}

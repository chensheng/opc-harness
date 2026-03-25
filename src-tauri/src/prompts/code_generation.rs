//! 代码生成提示词模板
//! 
//! VC-019: 为 Coding Agent 提供标准化的代码生成提示词库
//! 
//! 支持场景：
//! - TypeScript/React: 组件、Hooks、类型定义、测试、样式
//! - Rust: 模块、Trait 实现、错误处理、测试、API 端点
//! - 通用：重构、Bug 修复、文档、代码审查

// ========== 数据结构定义 ==========

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

// ========== TypeScript/React 提示词模板 ==========

/// React 组件生成提示词模板
pub const COMPONENT_GENERATION_PROMPT: &str = r#"你是一位经验丰富的前端工程师，擅长编写高质量、可复用的 React 组件。

## 任务
请根据以下要求生成一个 React 组件。

## 组件信息
- **组件名称**: {component_name}
- **组件描述**: {description}
- **Props 接口**: {props_interface}
- **使用场景**: {usage_scenario}

## 技术要求
1. 使用 TypeScript 严格模式
2. 使用 React Hooks (useState, useEffect, useCallback 等)
3. 使用 Tailwind CSS 进行样式设计
4. 遵循单一职责原则
5. 包含完整的 PropTypes 类型定义
6. 添加必要的代码注释

## 代码规范
- 使用函数式组件
- 使用解构赋值
- 使用箭头函数
- 避免直接使用 any 类型
- 适当的错误处理
- 性能优化（React.memo, useMemo 等）

## 输出格式
请直接输出组件代码，无需额外解释。代码应包含：
1. 必要的 import 语句
2. Props 类型定义（interface 或 type）
3. 组件主逻辑
4. 导出语句

## 示例结构
```tsx
import React, { useState, useCallback } from 'react';
import { IconName } from 'lucide-react';

interface ComponentNameProps {
  // Props 定义
}

export function ComponentName({ prop1, prop2 }: ComponentNameProps) {
  // 组件逻辑
  
  return (
    <div className="...">
      {/* 组件内容 */}
    </div>
  );
}
```
"#;

/// Custom Hook 生成提示词模板
pub const HOOK_GENERATION_PROMPT: &str = r#"你是一位资深的前端架构师，擅长设计和实现可复用的 Custom Hooks。

## 任务
请根据以下需求实现一个 Custom Hook。

## Hook 信息
- **Hook 名称**: use{hook_name}
- **功能描述**: {description}
- **输入参数**: {input_params}
- **返回值**: {return_value}
- **使用场景**: {usage_example}

## 技术要求
1. 使用 TypeScript 严格模式
2. 遵循 React Hooks 规则
3. 正确处理依赖数组
4. 清理副作用（useEffect cleanup）
5. 类型安全，避免 any
6. 考虑边界情况

## 最佳实践
- 单一的抽象层次
- 清晰的职责边界
- 可组合性设计
- 完善的错误处理
- 性能优化（useMemo, useCallback）

## 输出格式
```typescript
import { useState, useEffect, useCallback, useMemo } from 'react';

interface UseHookNameParams {
  // 参数类型定义
}

interface UseHookNameReturn {
  // 返回值类型定义
}

export function useHookName(
  params: UseHookNameParams
): UseHookNameReturn {
  // Hook 实现
  
  return {
    // 返回值
  };
}
```
"#;

/// TypeScript 类型定义生成提示词模板
pub const TYPE_DEFINITION_PROMPT: &str = r#"你是一位 TypeScript 专家，擅长设计优雅、类型安全的类型系统。

## 任务
请为以下业务场景设计 TypeScript 类型定义。

## 业务场景
{business_context}

## 数据类型
{data_structure}

## 设计要求
1. 使用 interface 定义对象类型
2. 使用 type 定义联合类型和工具类型
3. 使用泛型提高复用性
4. 使用 readonly 标记不可变属性
5. 使用 ? 标记可选属性
6. 避免使用 any，使用 unknown 代替

## 类型安全
- 严格的 null/undefined 检查
- 字面量类型代替字符串常量
- 枚举类型代替魔法数字
- 判别联合（Discriminated Unions）

## 输出格式
```typescript
// 核心类型定义
interface TypeName {
  id: string;
  name: string;
  description?: string;
  createdAt: Date;
  status: 'active' | 'inactive' | 'pending';
}

// 工具类型
type PartialTypeName = Partial<TypeName>;
type ReadOnlyTypeName = Readonly<TypeName>;

// 泛型类型
interface Response<T> {
  success: boolean;
  data: T;
  error?: string;
}

// 联合类型
type ActionType = 'create' | 'update' | 'delete';
```
"#;

/// 单元测试生成提示词模板
pub const TEST_GENERATION_PROMPT: &str = r#"你是一位测试专家，擅长编写全面、高质量的单元测试。

## 任务
请为以下代码编写单元测试。

## 被测代码
```{language}
{code_to_test}
```

## 测试框架
{test_framework}

## 测试要求
1. 覆盖所有公共 API
2. 测试正常路径和异常路径
3. 测试边界条件
4. 使用有意义的测试用例名称
5. 遵循 AAA 模式（Arrange-Act-Assert）
6. 测试应该是独立的、可重复的

## 测试覆盖
- [ ] 正常情况测试
- [ ] 异常情况测试
- [ ] 边界条件测试
- [ ] 空值/undefined 测试
- [ ] 并发/异步测试

## 最佳实践
- 每个测试只测试一个行为
- 使用 describe 组织相关测试
- 使用 beforeEach/afterEach 设置环境
- Mock 外部依赖
- 测试应该有意义的名称

## 输出格式
```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { functionToTest } from './module';

describe('functionToTest', () => {
  describe('正常情况', () => {
    it('应该返回预期结果', () => {
      // Arrange
      const input = 'value';
      
      // Act
      const result = functionToTest(input);
      
      // Assert
      expect(result).toEqual(expected);
    });
  });

  describe('异常情况', () => {
    it('应该抛出错误', () => {
      // Arrange
      const invalidInput = null;
      
      // Act & Assert
      expect(() => functionToTest(invalidInput)).toThrow();
    });
  });
});
```
"#;

/// Tailwind CSS 样式生成提示词模板
pub const STYLE_GENERATION_PROMPT: &str = r#"你是一位 UI/UX 设计师，擅长使用 Tailwind CSS 创建美观、响应式的界面。

## 任务
请为以下组件生成 Tailwind CSS 样式。

## 组件信息
- **组件类型**: {component_type}
- **设计风格**: {design_style}
- **颜色主题**: {color_theme}
- **响应式要求**: {responsive_requirements}

## 设计要求
1. 遵循移动优先原则
2. 使用语义化的类名
3. 保持一致的间距系统
4. 考虑无障碍访问（a11y）
5. 支持暗色模式
6. 性能优化（避免不必要的类）

## 响应式断点
- sm: 640px+
- md: 768px+
- lg: 1024px+
- xl: 1280px+
- 2xl: 1536px+

## 输出格式
```jsx
<div className="
  /* 基础样式 */
  flex items-center justify-center
  p-4 m-2
  bg-white dark:bg-gray-800
  
  /* 响应式 */
  sm:flex-row
  md:p-6
  lg:w-full
  
  /* 交互状态 */
  hover:bg-gray-100 dark:hover:bg-gray-700
  focus:outline-none focus:ring-2 focus:ring-blue-500
  
  /* 无障碍 */
  cursor-pointer
  transition-colors duration-200
">
  {/* 内容 */}
</div>
```
"#;

// ========== Rust 提示词模板 ==========

/// Rust 模块生成提示词模板
pub const RUST_MODULE_GENERATION_PROMPT: &str = r#"你是一位资深的 Rust 开发者，擅长编写安全、高效的 Rust 代码。

## 任务
请根据以下要求生成一个 Rust 模块。

## 模块信息
- **模块名称**: {module_name}
- **功能描述**: {description}
- **主要结构体**: {structs}
- **主要方法**: {methods}
- **错误类型**: {error_types}

## 技术要求
1. 遵循 Rust 惯用法（Idiomatic Rust）
2. 使用 Result<T, E> 处理错误
3. 实现必要的 Trait（Debug, Clone, Serialize, Deserialize 等）
4. 使用 serde 进行序列化/反序列化
5. 包含完整的文档注释（///）
6. 包含单元测试（#[cfg(test)]）

## 代码规范
- 使用 snake_case 命名变量和函数
- 使用 PascalCase 命名类型
- 使用 UPPER_SNAKE_CASE 命名常量
- 适当的可见性修饰符（pub/pub(crate)/private）
- 所有权和借用清晰

## 输出格式
```rust
//! 模块文档说明

use serde::{Deserialize, Serialize};

/// 结构体文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructName {
    /// 字段文档
    pub field: String,
}

impl StructName {
    /// 方法文档
    pub fn new(param: String) -> Self {
        Self {
            field: param,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // 测试代码
    }
}
```
"#;

/// Rust Trait 实现提示词模板
pub const RUST_TRAIT_IMPLEMENTATION_PROMPT: &str = r#"你是一位 Rust 专家，擅长设计和实现 Trait。

## 任务
请为以下类型实现指定的 Trait。

## 类型信息
- **类型名称**: {type_name}
- **类型定义**: {type_definition}

## Trait 要求
- **Trait 名称**: {trait_name}
- **需要实现的方法**: {methods}
- **特殊要求**: {special_requirements}

## 实现要求
1. 遵循 Trait 的设计意图
2. 正确处理边界情况
3. 实现相关的派生 Trait（Display, Debug, Error 等）
4. 使用适当的泛型约束
5. 考虑生命周期标注

## 输出格式
```rust
use std::fmt;

/// 自定义 Trait
pub trait MyTrait {
    fn required_method(&self) -> Result<(), MyError>;
    
    fn default_method(&self) {
        println!("Default implementation");
    }
}

impl MyTrait for MyType {
    fn required_method(&self) -> Result<(), MyError> {
        // 实现逻辑
        Ok(())
    }
}

// 实现 Display Trait
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyType: {}", self.field)
    }
}

// 实现 Debug Trait
impl fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyType")
            .field("field", &self.field)
            .finish()
    }
}
```
"#;

/// Rust 错误处理提示词模板
pub const RUST_ERROR_HANDLING_PROMPT: &str = r#"你是一位 Rust 错误处理专家，擅长设计优雅的错误处理机制。

## 任务
请为以下场景设计错误处理方案。

## 业务场景
{business_context}

## 可能的错误类型
{possible_errors}

## 设计要求
1. 使用 thiserror 或 anyhow 库
2. 定义清晰的错误类型枚举
3. 实现 std::error::Error Trait
4. 实现 Display 和 Debug Trait
5. 支持错误链（Error Chain）
6. 提供有用的错误信息

## 最佳实践
- 具体的错误类型而非泛型
- 错误应该包含上下文信息
- 区分可恢复和不可恢复错误
- 使用 Result<T, E> 而非 panic!
- 提供错误恢复建议

## 输出格式
```rust
use thiserror::Error;
use std::fmt;

/// 自定义错误类型
#[derive(Error, Debug)]
pub enum MyAppError {
    #[error("IO 错误：{0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误：{message}")]
    ParseError { message: String },
    
    #[error("验证失败：{field} - {reason}")]
    ValidationError {
        field: String,
        reason: String,
    },
    
    #[error("配置错误：{0}")]
    Config(String),
}

// 使用示例
pub fn my_function() -> Result<MyType, MyAppError> {
    // 业务逻辑
    Ok(value)
}
```
"#;

/// Rust API 端点生成提示词模板
pub const RUST_API_ENDPOINT_PROMPT: &str = r#"你是一位 Tauri 和 Rust API 开发专家，擅长设计和实现 Tauri Commands。

## 任务
请为以下功能实现 Tauri Command。

## 功能信息
- **Command 名称**: {command_name}
- **功能描述**: {description}
- **输入参数**: {input_params}
- **返回类型**: {return_type}
- **副作用**: {side_effects}

## 技术要求
1. 使用 #[tauri::command] 宏
2. 返回 Result<T, String> 或自定义错误
3. 使用 State 管理应用状态（如需要）
4. 适当的日志记录
5. 参数验证
6. 错误处理和用户友好的错误消息

## 安全考虑
- 输入验证
- 权限检查
- 资源限制
- 防止注入攻击

## 输出格式
```rust
use tauri::{State, command};
use serde::{Deserialize, Serialize};

/// Command 文档说明
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandParams {
    pub param1: String,
    pub param2: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub data: String,
}

/// Command 函数
#[command]
pub async fn my_command(
    params: CommandParams,
    state: State<'_, AppState>,
) -> Result<CommandResult, String> {
    log::info!("Executing my_command with params: {:?}", params);
    
    // 参数验证
    if params.param1.is_empty() {
        return Err("param1 cannot be empty".to_string());
    }
    
    // 业务逻辑
    
    Ok(CommandResult {
        success: true,
        data: "result".to_string(),
    })
}
```
"#;

// ========== 通用提示词模板 ==========

/// 代码重构提示词模板
pub const REFACTORING_PROMPT: &str = r#"你是一位代码重构专家，擅长改善代码质量而不改变外部行为。

## 任务
请对以下代码进行重构。

## 原始代码
```{language}
{original_code}
```

## 重构目标
{refactoring_goals}

## 重构原则
1. **保持行为不变**: 重构不改变代码的外部行为
2. **小步前进**: 每次只做小的改动
3. **频繁测试**: 每步重构后运行测试
4. **消除重复**: DRY 原则
5. **提高可读性**: 代码是写给人看的

## 改进方向
- [ ] 提取函数/方法
- [ ] 重命名提高语义
- [ ] 消除重复代码
- [ ] 简化条件逻辑
- [ ] 替换条件表达式
- [ ] 分解过大的类/函数
- [ ] 改进错误处理

## 输出格式
```{language}
// 重构后的代码
// 在代码注释中说明做了哪些改进
```

## 重构说明
列出所有改进点：
1. 改进 1
2. 改进 2
...
```
"#;

/// Bug 修复提示词模板
pub const BUG_FIXING_PROMPT: &str = r#"你是一位调试专家，擅长快速定位和修复 Bug。

## 任务
请分析并修复以下 Bug。

## Bug 描述
{bug_description}

## 重现步骤
{reproduction_steps}

## 预期行为
{expected_behavior}

## 实际行为
{actual_behavior}

## 相关代码
```{language}
{related_code}
```

## 调试流程
1. **理解问题**: 准确理解 Bug 的表现
2. **定位原因**: 分析可能的根本原因
3. **制定方案**: 设计修复策略
4. **实施修复**: 修改代码
5. **验证修复**: 确保 Bug 已修复且无回归

## 输出格式
### 问题分析
分析 Bug 的根本原因。

### 修复方案
描述修复策略。

### 修复代码
```{language}
// 修复后的代码
```

### 验证方法
如何验证修复是否成功。
```
"#;

/// 代码文档生成提示词模板
pub const DOCUMENTATION_PROMPT: &str = r#"你是一位技术文档专家，擅长编写清晰、完整的技术文档。

## 任务
请为以下代码生成文档。

## 代码内容
```{language}
{code_content}
```

## 文档类型
{doc_type}

## 文档要求
1. **准确性**: 文档必须准确反映代码行为
2. **完整性**: 覆盖所有重要方面
3. **可读性**: 清晰易懂
4. **结构化**: 良好的组织结构
5. **示例**: 提供使用示例

## 文档结构
- **概述**: 简要说明功能
- **参数说明**: 详细的参数描述
- **返回值**: 返回值说明
- **异常处理**: 可能的异常
- **使用示例**: 代码示例
- **注意事项**: 特别说明

## 输出格式
```markdown
## 函数/类名称

### 功能描述
简要描述功能。

### 参数
- `param1` (类型): 参数说明

### 返回值
返回类型：说明

### 异常
可能抛出的异常。

### 示例
```{language}
// 使用示例代码
```

### 注意事项
特别说明。
```
```
"#;

/// 代码审查提示词模板
pub const CODE_REVIEW_PROMPT: &str = r#"你是一位资深代码审查专家，擅长发现代码问题并提供改进建议。

## 任务
请对以下代码进行全面审查。

## 代码内容
```{language}
{code_content}
```

## 审查维度

### 1. 代码质量
- [ ] 代码清晰度
- [ ] 命名规范性
- [ ] 函数大小
- [ ] 代码重复

### 2. 正确性
- [ ] 逻辑错误
- [ ] 边界条件
- [ ] 异常处理
- [ ] 并发安全

### 3. 性能
- [ ] 时间复杂度
- [ ] 空间复杂度
- [ ] 内存泄漏
- [ ] 不必要的计算

### 4. 可维护性
- [ ] 模块化
- [ ] 可扩展性
- [ ] 测试覆盖
- [ ] 文档注释

### 5. 安全性
- [ ] 输入验证
- [ ] SQL 注入
- [ ] XSS 攻击
- [ ] 敏感数据

## 输出格式
### 总体评价
对代码的整体印象。

### 发现的问题
#### 🔴 严重问题
1. 问题描述 + 修复建议

#### 🟡 一般问题
1. 问题描述 + 改进建议

#### 🟢 建议改进
1. 优化建议

### 优点
列出代码的优点。

### 总结
总体建议。
```
"#;

// ========== 模板管理函数 ==========

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
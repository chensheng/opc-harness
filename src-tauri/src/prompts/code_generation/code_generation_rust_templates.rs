//! Rust 代码生成提示词模板
//! 
//! VC-019: Rust 模块、Trait实现、错误处理、API端点生成模板

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

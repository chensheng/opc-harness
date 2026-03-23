# Rust 后端开发规范 (Tauri v2)

> **适用范围**: `src-tauri/` 目录下所有 Rust 代码  
> **最后更新**: 2026-03-22

## 🎯 模块职责

### 分层架构
```
src-tauri/
├── src/
│   ├── main.rs          # 应用入口，初始化 Tauri 和插件
│   ├── commands/        # Tauri 命令层 - 仅做参数转发和错误处理
│   ├── services/        # 业务逻辑层 - 核心业务实现
│   ├── models/          # 数据模型层 - 结构体和序列化
│   ├── db/              # 数据库层 - SQLite 连接和 CRUD
│   ├── ai/              # AI 服务层 - AI厂商集成
│   └── utils/           # 工具函数层 - 纯函数，无业务依赖
└── Cargo.toml           # Rust 依赖配置
```

### 数据流规则
```rust
// ✅ 推荐：单向数据流
Frontend → Commands → Services → DB/AI
                ↑                       |
                └────── Response ───────┘

// ❌ 禁止：
- Commands 包含业务逻辑 - 必须委托给 Services
- Services 直接返回前端 - 必须通过 Commands
- 循环依赖：commands ↔ services
```

## 🛠️ 开发规范

### 命令实现
```rust
// ✅ 推荐：Commands 仅做参数转发
#[tauri::command]
pub async fn create_project(
    name: String,
    description: Option<String>,
) -> Result<Project, String> {
    // 参数验证
    if name.is_empty() {
        return Err("项目名称不能为空".to_string());
    }
    
    // 委托给 Service 层
    services::create_project(name, description)
        .await
        .map_err(|e| format!("创建失败：{}", e))
}

// ❌ 避免：在 Commands 中包含复杂业务逻辑
```

### 错误处理
```rust
// ✅ 推荐：使用 Result + 中文错误信息
pub async fn save_project(project: Project) -> Result<Project, AppError> {
    match validate_project(&project) {
        Ok(_) => Ok(db::save_project(project).await?),
        Err(e) => Err(AppError::ValidationError(e)),
    }
}

// 错误类型定义
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "数据库错误：{}", e),
            AppError::ValidationError(e) => write!(f, "验证失败：{}", e),
            AppError::NotFound(e) => write!(f, "未找到：{}", e),
        }
    }
}
```

### 数据序列化
```rust
// ✅ 推荐：使用 camelCase 与前端保持一致
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ❌ 避免：使用 snake_case 直接暴露给前端
// ❌ 避免：忘记添加 #[serde(rename_all = "camelCase")]
```

## 📁 文件组织

### 命名规范
- **模块文件**: snake_case (e.g., `mod.rs`, `project_service.rs`)
- **结构体**: PascalCase (e.g., `Project`, `AIConfig`)
- **函数**: snake_case (e.g., `create_project()`)
- **常量**: SCREAMING_SNAKE_CASE (e.g., `MAX_RETRIES`)

### 目录结构
```
commands/
├── mod.rs             # 命令导出
├── ai.rs              # AI 相关命令
├── cli.rs             # CLI 相关命令
├── database.rs        # 数据库 CRUD 命令
├── system.rs          # 系统相关命令
└── ...

services/
├── mod.rs             # 服务导出
└── ...

models/
├── mod.rs             # 模型导出
└── ...
```

## 🔒 架构约束

### 依赖方向
```rust
// ✅ 允许：
commands → services → models → db
commands → utils
services → ai

// ❌ 禁止：
services → commands      // 服务层不可依赖命令层
db → services            // 数据库层不可依赖服务层
models → commands        // 模型层不可依赖命令层
```

### 导入规范
```rust
// ✅ 推荐：使用绝对路径
use crate::models::Project;
use crate::services::project_service;

// ✅ 推荐：在模块内使用相对路径
use super::models::AIConfig;
use crate::utils::generate_uuid();

// ❌ 避免：过度使用 super::super::...
```

## 🧪 测试要求

### 单元测试
```rust
// ✅ 推荐：为业务逻辑编写测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_name("My Project").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name("a".repeat(101)).is_err());
    }
}
```

### 集成测试
```rust
// ✅ 推荐：测试完整流程
#[tokio::test]
async fn test_create_project_flow() {
    let project = create_project("Test".to_string(), None).await.unwrap();
    assert_eq!(project.name, "Test");
    assert!(!project.id.is_empty());
}
```

## 🚨 常见陷阱

### 陷阱 1: 在 Commands 中包含业务逻辑
```rust
// ❌ 错误
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 直接在命令中操作数据库
    let conn = get_connection()?;
    let project = Project { name, .. };
    conn.execute(...)  // 复杂的 SQL 逻辑
}

// ✅ 正确
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 仅做参数验证和错误处理
    services::create_project(name).await
}
```

### 陷阱 2: 忽略错误传播
```rust
// ❌ 错误：使用 unwrap()
let project = db::get_project(id).unwrap();

// ✅ 正确：使用 ? 或 map_err
let project = db::get_project(id)
    .await?
    .ok_or_else(|| "项目不存在".to_string())?;
```

### 陷阱 3: 忘记序列化配置
```rust
// ❌ 错误：前端收到 snake_case 字段
#[derive(Serialize)]
pub struct Project {
    pub created_at: String,  // 前端期望 createdAt
}

// ✅ 正确
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub created_at: String,  // 序列化为 createdAt
}
```

## 🔧 工具集成

### Cargo 检查
```bash
# 编译检查
cargo check

# 格式化检查
cargo fmt --check

# Clippy lint
cargo clippy -- -D warnings
```

### 格式化配置
```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
```

## 📖 参考资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Tauri v2 文档](https://v2.tauri.app/)
- [serde 文档](https://serde.rs/)
- [rusqlite 文档](https://docs.rs/rusqlite/)

---

**违反这些规则的后果**: 
- `npm run harness:check` 将报告错误
- CI/CD 流水线会失败
- Agent 会自动修复并重新提交

**需要帮助？** 
查看根目录 [`AGENTS.md`](../AGENTS.md) 获取更多导航信息。

# 架构约束规则

本文档定义了 OPC-HARNESS 项目的架构约束和工程规范，确保代码质量和系统可维护性。

## 📐 分层架构规则

### 层级定义

```
┌─────────────────────────────────────┐
│         Presentation Layer          │
│      (React Components + UI)        │
│           src/components/           │
└─────────────────────────────────────┘
                 ↓↑
┌─────────────────────────────────────┐
│         State Management            │
│       (Zustand Stores)              │
│            src/stores/              │
└─────────────────────────────────────┘
                 ↓↑
┌─────────────────────────────────────┐
│         Tauri Commands              │
│    (TypeScript → Rust Bridge)       │
│         invoke() calls              │
└─────────────────────────────────────┘
                 ↓↑
┌─────────────────────────────────────┐
│         Business Logic              │
│        (Rust Services)              │
│      src-tauri/src/services/        │
└─────────────────────────────────────┘
                 ↓↑
┌─────────────────────────────────────┐
│         Data Access                 │
│        (SQLite + Models)            │
│       src-tauri/src/db/             │
└─────────────────────────────────────┘
```

### 依赖规则

#### ✅ 允许的依赖方向
1. **Components → Stores**: UI 组件可以访问 Zustand stores
2. **Stores → Types**: 状态管理可以使用类型定义
3. **Frontend → Tauri Commands**: 通过 `invoke()` 调用后端
4. **Rust Services → Models**: 服务层使用数据模型
5. **Rust Services → DB**: 服务层访问数据库

#### ❌ 禁止的依赖模式
1. **循环依赖**: A → B → A
2. **跨层跳跃**: Components 直接调用 Rust 函数（必须通过 Tauri commands）
3. **反向依赖**: Stores 不能依赖 Components
4. **外部导入**: 不允许从 `node_modules` 外的路径导入

### 代码组织规范

#### TypeScript 文件结构
```typescript
// 1. 标准库导入
import React from 'react';
import { useState } from 'react';

// 2. 第三方库导入
import { create } from 'zustand';
import { Button } from '@/components/ui/button';

// 3. 项目内部导入（按相对路径排序）
import { useAppStore } from '@/stores/appStore';
import { Project } from '@/types';

// 4. 样式导入
import './Component.css';

// 5. 类型定义
interface Props {
  name: string;
}

// 6. 组件实现
export function Component({ name }: Props) {
  return <div>{name}</div>;
}
```

#### Rust 文件结构
```rust
// 1. 标准库导入
use std::sync::Arc;

// 2. 外部 crate 导入
use serde::{Deserialize, Serialize};
use tauri::State;

// 3. 项目内部模块导入
use crate::models::Project;
use crate::db::Database;

// 4. 错误处理
use anyhow::Result;

// 5. 公共类型定义
pub type SharedState = Arc<tokio::sync::Mutex<AppState>>;

// 6. 函数实现
#[tauri::command]
pub async fn get_projects() -> Result<Vec<Project>, String> {
    // 实现逻辑
}
```

## 🔒 安全约束

### API Key 管理
```rust
// ✅ 正确：使用系统Keychain 存储
use keyring::Entry;

let entry = Entry::new("opc-harness", "openai_key")?;
entry.set_password(&api_key)?;

// ❌ 错误：硬编码或明文存储
const API_KEY = "sk-xxx"; // 绝对禁止！
```

### 数据验证
```typescript
// ✅ 推荐：运行时验证
function validateProject(data: unknown): Project {
  if (!data || typeof data.name !== 'string') {
    throw new Error('无效的项目数据');
  }
  return data as Project;
}

// ❌ 避免：无条件检查
const project = data as Project; // 可能不安全
```

## ⚡ 性能约束

### 响应时间限制
| 操作类型 | 最大延迟 | 降级策略 |
|---------|---------|---------|
| UI 交互 | < 100ms | 显示加载状态 |
| Tauri 命令 | < 500ms | 超时错误提示 |
| AI API 调用 | < 30s | 流式响应 + 重试 |
| 数据库查询 | < 100ms | 添加索引优化 |

### 内存使用限制
```rust
// ✅ 推荐：流式处理大文件
async fn process_large_file(path: &str) -> Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::with_capacity(8192); // 限制缓冲区大小
    // ...
}

// ❌ 避免：一次性加载到内存
async fn bad_process_file(path: &str) -> Result<()> {
    let content = tokio::fs::read_to_string(path).await?; // 可能占用大量内存
    // ...
}
```

### 并发限制
```rust
// ✅ 推荐：使用信号量控制并发
use tokio::sync::Semaphore;
static SEMAPHORE: Semaphore = Semaphore::const_new(5);

async fn limited_operation() {
    let _permit = SEMAPHORE.acquire().await.unwrap();
    // 最多 5 个并发
}
```

## 📝 代码规范约束

### TypeScript 严格模式
```json
// tsconfig.json 必须包含
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true
  }
}
```

### Rust 代码风格
```rust
// ✅ 遵循 Rust 官方风格指南
pub fn calculate_total(items: &[Item]) -> f64 {
    items.iter().map(|item| item.price).sum()
}

// ❌ 避免：不规范的命名和格式
pub fn CALCULATE(p: &Vec<Item>) -> f64 { // 错误的命名风格
    let mut total = 0.0;
    for i in p { // 应该使用迭代器
        total += i.price;
    }
    total
}
```

### 错误信息规范
```rust
// ✅ 推荐：清晰的中文错误提示
Err("未配置 AI厂商，请先在设置页面配置 API Key".to_string())

// ❌ 避免：模糊的错误信息
Err("Invalid configuration".to_string()) // 用户不知道如何修复
```

## 🧪 测试约束

### 单元测试覆盖率
- **核心业务逻辑**: >= 80%
- **工具函数**: >= 90%
- **UI 组件**: >= 70%

### 集成测试要求
```rust
// 每个 Tauri command 应该有对应的集成测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_get_projects_command() {
        // 测试命令执行
    }
}
```

## 🔄 变更管理

### 破坏性变更流程
1. **标记弃用**: 使用 `#[deprecated]` 标注旧 API
2. **迁移文档**: 提供详细的迁移指南
3. **版本递增**: 遵循语义化版本控制
4. **向后兼容**: 至少保留一个版本的过渡期

### 依赖更新策略
```bash
# 主版本更新前必须评估影响
npm update --save  # 小版本更新
npm install pkg@latest --save  # 主版本更新需要审查
```

## 📊 质量指标

### 代码复杂度限制
- **圈复杂度**: <= 10
- **函数长度**: <= 50 行
- **文件长度**: <= 500 行
- **嵌套深度**: <= 4 层

### 技术债务管理
- 每个 `TODO` 注释必须包含日期和负责人
- `FIXME` 注释必须在 7 天内解决
- `HACK` 注释必须在下次重构时移除

## 🚨 违规处理

### 自动检测
```bash
# 架构健康检查将自动发现违规
npm run harness:check

# 输出示例
❌ 违规：循环依赖 detected
   src/components/A.tsx → src/stores/B.ts → src/components/A.tsx
   
💡 建议：将共享逻辑提取到 utils/
```

### 修复策略
1. **警告**: 首次违规发出警告
2. **阻止提交**: lint-staged 拦截不符合规范的代码
3. **CI 失败**: 持续集成流水线拒绝违规代码

---

**维护者**: OPC-HARNESS Team  
**最后更新**: 2026-03-22  
**版本**: 1.0.0

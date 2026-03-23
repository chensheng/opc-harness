# 架构约束规则

> **版本**: 1.0.0  
> **最后更新**: 2026-03-23  
> **适用范围**: OPC-HARNESS 项目所有代码  
> **执行方式**: ESLint + Clippy 自动检查

## 📋 概述

本文档定义了项目的架构约束规则，确保代码遵循分层架构和依赖方向。所有规则通过 ESLint 和 Cargo Clippy 自动执行。

### 核心原则

1. **单向依赖**: 上层可以依赖下层，下层不可依赖上层
2. **职责分离**: 每一层有明确的职责边界
3. **自动化执行**: 所有规则可通过工具自动检查

---

## 🖥️ 前端架构约束 (src/)

### FE-ARCH-001: 状态管理层不可直接导入 UI 组件

**级别**: ❌ 错误 (error)

**规则**: `stores/` 目录下的文件不能导入 `components/` 目录

**示例**:
```typescript
// ❌ 错误
// stores/projectStore.ts
import { ProjectCard } from '@/components/ProjectCard';

// ✅ 正确
// stores/projectStore.ts
// Store 保持纯净，不导入任何组件
```

**修复建议**: Store 层应该保持纯净，仅管理状态。如需使用组件，请通过 Hooks 层中转。

---

### FE-ARCH-002: Hooks 不可直接导入具体业务组件

**级别**: ❌ 错误 (error)

**规则**: `hooks/` 目录下的文件不能导入 `components/vibe-*/` 目录

**示例**:
```typescript
// ❌ 错误
// hooks/useAIStream.ts
import { VibeDesignForm } from '@/components/vibe-design/VibeDesignForm';

// ✅ 正确
// hooks/useAIStream.ts
// Hooks 应该是通用的，不依赖具体业务组件
```

**修复建议**: Hooks 应该是通用的，不应依赖具体业务组件。考虑将组件逻辑上移到父组件。

---

### FE-ARCH-003: 工具函数层不可依赖状态管理层

**级别**: ❌ 错误 (error)

**规则**: `lib/` 目录下的文件不能导入 `stores/` 目录

**示例**:
```typescript
// ❌ 错误
// lib/utils.ts
import { useAppStore } from '@/stores/appStore';

// ✅ 正确
// lib/utils.ts
// 工具函数通过参数传递所需数据，不依赖全局状态
export function formatDate(date: Date): string {
  return date.toISOString();
}
```

**修复建议**: 工具函数应该是纯函数，不依赖全局状态。考虑通过参数传递所需数据。

---

### FE-ARCH-004: 优先使用路径别名

**级别**: ⚠️ 警告 (warn)

**规则**: 相对路径深度不能超过 3 层

**示例**:
```typescript
// ❌ 避免：相对路径过深
import { Button } from '../../../components/ui/button';

// ✅ 推荐：使用路径别名
import { Button } from '@/components/ui/button';
```

**修复建议**: 使用路径别名可以提高可读性：`import { X } from '@/path/to/module'`

---

### FE-ARCH-005: 禁止直接调用 Tauri invoke()

**级别**: ❌ 错误 (error)

**规则**: `components/` 目录下的 `.tsx` 文件不能直接调用 `invoke()` 函数

**示例**:
```typescript
// ❌ 错误
function MyComponent() {
  const handleSave = async () => {
    await invoke('save_project', { data });
  };
}

// ✅ 正确
// stores/projectStore.ts
export const useProjectStore = create((set) => ({
  saveProject: async (data) => {
    await invoke('save_project', { data });
  }
}));

// components/MyComponent.tsx
function MyComponent() {
  const { saveProject } = useProjectStore();
  const handleSave = () => saveProject(data);
}
```

**修复建议**: 请通过 stores 或 hooks 封装 Tauri 调用：`useProjectStore().saveProject()`

---

## 🦀 后端架构约束 (src-tauri/)

### BE-ARCH-001: Commands 层不可包含复杂业务逻辑

**级别**: ❌ 错误 (error)

**规则**: 
- `commands/` 目录下的函数不能超过 30 行
- 不能直接调用 `db::` 或 `ai::` 命名空间

**示例**:
```rust
// ❌ 错误
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 直接在命令中操作数据库 - 超过 30 行
    let conn = get_connection()?;
    let project = Project { name, .. };
    conn.execute(...)?;  // 复杂的 SQL 逻辑
    // ... 更多业务逻辑
}

// ✅ 正确
#[tauri::command]
pub fn create_project(name: String) -> Result<Project, String> {
    // 仅做参数验证和错误处理
    services::create_project(name).await
}
```

**修复建议**: Commands 层仅做参数验证和错误处理，业务逻辑委托给 Services 层。

---

### BE-ARCH-002: Services 层不可依赖 Commands 层

**级别**: ❌ 错误 (error)

**规则**: `services/` 目录不能导入 `crate::commands`

**示例**:
```rust
// ❌ 错误
// services/project_service.rs
use crate::commands::get_project_by_id;

// ✅ 正确
// services/project_service.rs
// Services 层独立于 Commands 层
```

**修复建议**: 依赖方向应该是 Commands → Services，反向依赖会导致循环依赖。

---

### BE-ARCH-003: Database 层不可依赖 Services 层

**级别**: ❌ 错误 (error)

**规则**: `db/` 目录不能导入 `crate::services`

**示例**:
```rust
// ❌ 错误
// db/project_db.rs
use crate::services::validate_project;

// ✅ 正确
// db/project_db.rs
// DB 层仅提供 CRUD 操作，不依赖业务逻辑
```

**修复建议**: DB 层仅提供 CRUD 操作，业务逻辑在 Services 层实现。

---

### BE-ARCH-004: 序列化必须使用 camelCase

**级别**: ❌ 错误 (error)

**规则**: `models/` 目录下所有结构体必须包含 `#[serde(rename_all = "camelCase")]` 属性

**示例**:
```rust
// ❌ 错误
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub created_at: String,  // 前端期望 createdAt
}

// ✅ 正确
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub created_at: String,  // 序列化为 createdAt
}
```

**修复建议**: 添加 `#[serde(rename_all = "camelCase")]` 属性到结构体定义。

---

### BE-ARCH-005: 公共函数必须返回 Result 类型

**级别**: ❌ 错误 (error)

**规则**: `services/` 目录下所有公共函数必须返回 `Result<_, AppError>` 类型

**示例**:
```rust
// ❌ 错误
pub fn save_project(project: Project) -> Project {
    // 没有错误处理
}

// ✅ 正确
pub async fn save_project(project: Project) -> Result<Project, AppError> {
    validate_project(&project)?;
    Ok(db::save_project(project).await?)
}
```

**修复建议**: 使用 `Result<T, AppError>` 作为返回类型，提供清晰的错误信息。

---

## 🔧 执行方式

### ESLint (前端)

**配置文件**: `.eslintrc.cjs`

**运行命令**:
```bash
npm run lint                    # 检查
npm run lint:fix               # 自动修复
```

**插件**: `eslint-plugin-opc-harness`

---

### Cargo Clippy (后端)

**配置文件**: `src-tauri/clippy.toml`

**运行命令**:
```bash
cd src-tauri && cargo clippy -- -D warnings
```

---

### 自动修复

**运行命令**:
```bash
npm run harness:fix            # 自动修复代码规范问题
```

**空运行模式**:
```bash
npm run harness:fix:dry        # 查看将修复什么（不实际修改）
```

---

## 📊 违规处理

### 评分影响

| 违规类型 | 扣分 | 影响 |
|---------|------|------|
| Error 级别规则 | -10 分/条 | 架构检查失败 |
| Warn 级别规则 | -5 分/条 | 可接受但有改进空间 |

### 质量门禁

- **90-100 分**: 优秀 ✨ - 可以安全合并
- **70-89 分**: 良好 👍 - 有一些改进空间
- **<70 分**: 需要修复 ⚠️ - 不建议合并

---

## 🎓 最佳实践

### 依赖方向图

```
Frontend:
Component → Hook → Store → Command → Service → DB
     ↑                                        |
     └────────── State Update ←───────────────┘

Backend:
main.rs → Commands → Services → Models → DB
                          ↓
                        AI/Cli
```

### 常见陷阱

1. **循环依赖**: A 导入 B，B 又导入 A
   - **解决**: 提取共同依赖到独立模块

2. **跨层调用**: Component 直接调用 Service
   - **解决**: 通过 Store 或 Hook 中转

3. **业务逻辑泄露**: DB 层包含业务验证
   - **解决**: 移动到 Services 层

---

## 🔄 更新记录

### v1.0.0 (2026-03-23)
- ✅ 初始版本，基于 architecture-rules.json 转换
- ✅ 添加 5 条前端规则
- ✅ 添加 5 条后端规则
- ✅ 明确执行方式和评分标准

---

## 🔗 相关资源

- [AGENTS.md](../AGENTS.md) - AI Agent 导航地图
- [src/AGENTS.md](../../src/AGENTS.md) - 前端开发规范
- [src-tauri/AGENTS.md](../../src-tauri/AGENTS.md) - Rust 后端规范
- [best-practices.md](./best-practices.md) - 编码最佳实践

---

**维护者**: OPC-HARNESS Team  
**审查周期**: 季度 ⭐  
**下次审查日期**: 2026-06-23

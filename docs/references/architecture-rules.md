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
4. **测试覆盖**: 所有功能必须有单元测试 + E2E 测试覆盖

---

## 🖥️ 前端架构约束 (src/)

### FE-ARCH-001: 状态管理层不可直接导入 UI 组件

**级别**: ❌ 错误 (error)

**规则**: `stores/` 目录下的文件不能导入 `components/` 目录

**示例**:
``typescript
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
``typescript
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
``typescript
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
``typescript
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
``typescript
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
```
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
```
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
```
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
```
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
```
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

## 🧪 测试架构约束 (Testing)

### TEST-001: 所有功能必须有单元测试覆盖

**级别**: ❌ 错误 (error)

**规则**: 
- 每个新功能的代码文件必须包含对应的 `.test.ts` 或 `.test.tsx` 文件
- Rust 模块必须包含 `#[cfg(test)] mod tests` 测试模块
- 单元测试覆盖率目标 ≥70%

**示例**:
``typescript
// ✅ 正确 - 包含测试文件
src/hooks/useOpenAIProvider.ts
src/hooks/useOpenAIProvider.test.ts  // 必须存在

// ❌ 错误 - 缺少测试
src/hooks/useNewFeature.ts
// 没有 test.ts 文件！
```

**Rust 示例**:
``rust
// src/ai/mod.rs

// ... 实现代码 ...

#[cfg(test)]
mod tests {
    #[test]
    fn test_provider_creation() {
        let provider = OpenAIProvider::new("test-key".to_string());
        assert_eq!(provider.api_key(), "test-key");
    }
}
```

**修复建议**: 为每个新功能创建对应的测试文件，确保测试覆盖率≥70%。

**执行方式**: 
- TypeScript: Vitest (`npm run test:unit`)
- Rust: Cargo Test (`cd src-tauri && cargo test`)

---

### TEST-002: 核心流程必须有 E2E 测试覆盖

**级别**: ❌ 错误 (error)

**规则**: 
- 所有核心用户流程必须包含 E2E 测试用例
- E2E 测试必须覆盖：应用启动、核心页面导航、关键配置流程
- E2E 测试必须自动管理开发服务器生命周期

**示例**:
``typescript
// ✅ 正确 - E2E 测试覆盖核心流程
// e2e/app.spec.ts
describe('OPC-HARNESS Application', () => {
  it('should load the application successfully')
  it('should have valid HTML structure')
  it('should navigate to Settings page')
  it('should detect installed tools')
})

// ❌ 错误 - 缺少 E2E 测试
// 只有单元测试，没有端到端测试
```

**E2E 测试要求**:
1. **自动服务器管理**: 检测端口占用、自动启停开发服务器
2. **测试报告生成**: 自动生成 HTML 测试报告并保存
3. **优雅清理机制**: 测试结束后清理所有资源
4. **跨平台兼容**: 支持 Windows/Linux/macOS

**修复建议**: 在 `e2e/` 目录下创建 `.spec.ts` 文件，覆盖核心用户流程。

**执行方式**: 
```
npm run test:e2e              # 运行所有 E2E 测试
npx vitest run e2e           # 直接运行 Vitest E2E
```

---

### TEST-003: 测试必须先于功能完成

**级别**: ❌ 错误 (error)

**规则**: 
- 遵循测试先行 (TDD) 原则
- 功能代码提交前，相关测试必须已编写并通过
- 不允许有无测试的功能代码

**示例**:
``typescript
// ✅ 正确的工作流
// 1. 先写测试
describe('useNewFeature', () => {
  it('should initialize with default state', () => {
    // 测试代码
  })
})

// 2. 运行测试（失败）
npm run test:unit  // ❌ FAIL

// 3. 实现功能
export function useNewFeature() {
  // 实现代码
}

// 4. 再次运行测试（通过）
npm run test:unit  // ✅ PASS

// ❌ 错误的做法
// 先写功能代码，后补测试或不写测试
```

**修复建议**: 采用 TDD 工作流：Red (测试失败) → Green (测试通过) → Refactor (重构)

---

### TEST-004: E2E 测试必须独立运行

**级别**: ⚠️ 警告 (warn)

**规则**: 
- E2E 测试不应依赖外部服务（如真实 API）
- 必须使用 Mock 数据或测试专用环境
- 测试用例之间不能有状态依赖

**示例**:
``typescript
// ✅ 正确 - 使用 Mock，独立运行
it('should handle chat request', async () => {
  // 使用 Mock 数据，不依赖真实 API
  const mockResponse = { content: 'Mock response' }
  vi.spyOn(api, 'chat').mockResolvedValue(mockResponse)
  
  const result = await chat({ messages: [] })
  expect(result).toEqual(mockResponse)
})

// ❌ 错误 - 依赖真实 API
it('should call real OpenAI API', async () => {
  // 不应该调用真实的 API
  const result = await realOpenAICall()
})
```

**修复建议**: 使用 Mock 数据和 Stub 技术，确保测试可重复且快速。

---

### TEST-005: 测试覆盖率不达标禁止合并

**级别**: ❌ 错误 (error)

**规则**: 
- 单元测试覆盖率 <70% 的代码禁止合并到主分支
- E2E 测试必须覆盖所有 P0/P1 级功能
- 每次 PR 必须附带测试覆盖率报告

**覆盖率要求**:
```yaml
# vite.config.ts
coverage:
  thresholds:
    global:
      branches: 70
      functions: 70
      lines: 70
      statements: 70
```

**检查命令**:
```
# 运行测试并生成覆盖率报告
npm run test:unit -- --coverage

# 查看覆盖率报告
open coverage/index.html
```

**修复建议**: 若覆盖率不达标，需补充更多测试用例，特别是边界条件和错误处理场景。

---

## 🔧 执行方式

### ESLint (前端)

**配置文件**: `.eslintrc.cjs`

**运行命令**:
```
npm run lint                    # 检查
npm run lint:fix               # 自动修复
```

**插件**: `eslint-plugin-opc-harness`

---

### Cargo Clippy (后端)

**配置文件**: `src-tauri/clippy.toml`

**运行命令**:
```
cd src-tauri && cargo clippy -- -D warnings
```

---

### 自动修复

**运行命令**:
```
npm run harness:fix            # 自动修复代码规范问题
```

**空运行模式**:
```
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

Testing:
Unit Tests (70%+) → Integration Tests → E2E Tests (Core Flows)
```

### 常见陷阱

1. **循环依赖**: A 导入 B，B 又导入 A
   - **解决**: 提取共同依赖到独立模块

2. **跨层调用**: Component 直接调用 Service
   - **解决**: 通过 Store 或 Hook 中转

3. **业务逻辑泄露**: DB 层包含业务验证
   - **解决**: 移动到 Services 层

4. **缺少测试覆盖** 🔴: 功能代码无对应测试
   - **影响**: 回归 bug 风险高，质量无法保障
   - **解决**: 遵循 TDD 流程，先写测试再实现功能

5. **E2E 测试依赖外部服务** ⚠️: 测试不稳定且缓慢
   - **影响**: CI/CD 失败率高，开发效率低
   - **解决**: 使用 Mock 数据，确保测试独立性

6. **后补测试** ❌: 先写功能后补测试
   - **影响**: 测试覆盖率不足，代码质量差
   - **解决**: 强制执行 TDD，测试不通过不提交

---

## 🔄 更新记录

### v1.1.0 (2026-03-23) - 添加测试架构约束 🔥
- ✅ 新增 TEST-001: 所有功能必须有单元测试覆盖
- ✅ 新增 TEST-002: 核心流程必须有 E2E 测试覆盖
- ✅ 新增 TEST-003: 测试必须先于功能完成 (TDD)
- ✅ 新增 TEST-004: E2E 测试必须独立运行
- ✅ 新增 TEST-005: 测试覆盖率不达标禁止合并
- ✅ 更新最佳实践，添加测试相关陷阱和解决方案
- ✅ 明确单元测试覆盖率目标 ≥70%
- ✅ 明确 E2E 测试自动管理服务器生命周期要求

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
# Harness Engineering 开发流程

> **Harness Engineering**: "质量内建，非事后检查"  
> **适用范围**: OPC-HARNESS 项目所有开发任务  
> **最后更新**: 2026-03-25  
> **执行方式**: 强制遵循 ✅

---

## 📋 核心原则

1. **质量内建** - 质量是构建出来的，不是检查出来的
2. **测试先行** - 先写测试再实现功能 (TDD) 🔥
3. **持续验证** - 每次修改后自动运行检查和测试
4. **E2E 覆盖** - 核心流程必须有端到端测试验证 🔥
5. **架构约束** - 严格遵守分层架构和依赖规则
6. **计划驱动** - 执行计划引领开发全过程 📋

---

## 🚀 开发流程

```
graph LR
A[1.任务选择] --> B[2.创建执行计划] --> C[3.架构学习] --> D[4.测试设计] --> E[5.开发实施] --> F[6.质量验证] --> G[7.更新任务状态] --> H[8.归档执行计划] --> I[9.Git 提交]
```

---

### 阶段 1: 任务选择 (5%)

**查阅**: [`Sprint计划`](./sprint-plans/index.md)

**选择标准**:

- 每次只选1个任务
- 🔴 P0/P1 高优先级 - 关键路径任务
- 🎯 独立性强 - 不依赖其他未完成的任务
- ⏱️ 工作量适中 - 可快速完成并验证
- 💡 价值明确 - 为后续功能奠定基础

**示例**:

```
任务：US-001 - 定义 Agent 通信协议
理由:
1. P0 高优先级 - Vibe Coding 基础架构
2. 关键路径 - Initializer/Coding Agent 依赖
3. 技术成熟 - Rust 类型系统支持
4. 工作量适中 - 可快速完成并验证
```

---

### 阶段 2: 创建执行计划 (5%) ⭐

[`创建执行计划步骤`](./exec-plans/templates/how-to-create-exec-plan.md)

---

### 阶段 3: 架构学习 (5%)

**必读文档**:

1. [`架构约束规则`](./design-docs/architecture-rules.md) - FE/BE/TEST 规则
2. [`前端规范`](../../src/AGENTS.md) - React/TypeScript 最佳实践
3. [`后端规范`](../../src-tauri/AGENTS.md) - Rust 编码规范

**关键约束**:

#### 前端 (FE-ARCH)

- FE-ARCH-001: Store 不导入组件
- FE-ARCH-002: Hooks 不导入业务组件
- FE-ARCH-003: 工具函数不依赖 Store
- FE-ARCH-004: 使用 @/ 路径别名
- FE-ARCH-005: 通过 Hook 封装 invoke

#### 后端 (BE-ARCH)

- BE-ARCH-001: Commands 层不含复杂逻辑
- BE-ARCH-002: Services 层不依赖 Commands
- BE-ARCH-003: Database 层不依赖 Services
- BE-ARCH-004: 序列化使用 camelCase
- BE-ARCH-005: 公共函数返回 Result 类型

#### 测试 (TEST) 🔥

- TEST-001: 所有功能必须有单元测试覆盖 (≥70%)
- TEST-002: 核心流程必须有 E2E 测试覆盖
- TEST-003: 测试必须先于功能完成 (TDD)
- TEST-004: E2E 测试必须独立运行
- TEST-005: 测试覆盖率不达标禁止合并

---

### 阶段 4: 测试设计 (10%) 🔥

**单元测试设计**:

```typescript
// TypeScript 测试 (*.test.ts)
describe('useOpenAIProvider', () => {
  it('should initialize with correct state')
  it('should validate API key successfully')
  it('should handle chat request')
})
```

```rust
// Rust 测试 (#[cfg(test)])
#[cfg(test)]
mod tests {
    #[test]
    fn test_provider_creation() {
        let provider = OpenAIProvider::new("test-key".to_string());
        assert_eq!(provider.api_key(), "test-key");
    }
}
```

**E2E 测试设计** 🔥:

```typescript
// e2e/app.spec.ts
describe('OPC-HARNESS Application', () => {
  it('should load the application successfully')
  it('should have valid HTML structure')
  it('should load required assets')
  it('should respond on mobile viewport size')
  it('should have no critical console errors')
  it('API endpoints should be accessible')
})
```

**覆盖率目标**:

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

---

### 阶段 5: 开发实施 (45%)

**后端实现 (Rust)**:

```rust
// src-tauri/src/ai/mod.rs
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AppError> {
        log::info!("Sending chat request to OpenAI");
        // ... 实现
    }
}
```

**关键要求**:

- ✅ 完整的类型定义
- ✅ 错误处理机制 (`Result<T, AppError>`)
- ✅ 日志记录
- ✅ 无 `cargo check` 错误
- ✅ 单元测试覆盖

**前端实现 (TypeScript/React)**:

```typescript
// src/hooks/useOpenAIProvider.ts
export function useOpenAIProvider() {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const chat = useCallback(async (request: ChatRequest) => {
    setIsLoading(true)
    try {
      // ... 实现
    } catch (err) {
      setError(err.message)
    } finally {
      setIsLoading(false)
    }
  }, [])

  return { isLoading, error, chat }
}
```

**关键要求**:

- ✅ TypeScript 类型安全
- ✅ React Hooks 最佳实践
- ✅ 遵循架构约束 (FE-ARCH)
- ✅ 无 `any` 类型（或最小化使用）
- ✅ 单元测试覆盖

---

### 阶段 6: 质量验证 (20%)

**运行 Harness 健康检查**:

```bash
npm run harness:check
```

**检查项** (8 项):

1. ✅ TypeScript Type Checking
2. ✅ ESLint Code Quality
3. ✅ Prettier Formatting
4. ✅ Rust Compilation
5. ✅ Rust Unit Tests Check
6. ✅ TypeScript Unit Tests Check
7. ✅ Dependency Integrity - 依赖文件完整性
8. ✅ Directory Structure - 目录结构检查

**评分标准**:

- 🟢 **Excellent**: 100/100 - 所有检查通过
- 🟡 **Good**: 70-99 分 - 少量警告
- 🔴 **Needs Fix**: <70 分 - 需要立即修复

**问题修复循环**:

```bash
# 迭代直到 Health Score = 100/100
while [ $(npm run harness:check | grep "Health Score" | cut -d: -f2 | cut -d/ -f1) -lt 100 ]; do
  npm run harness:fix          # 自动修复格式问题
  npx tsc --noEmit             # 手动修复类型错误
  cd src-tauri && cargo check  # 检查 Rust 编译
done
```

---

### 阶段 7: 更新任务状态 (10%)

将Sprint计划中的任务状态更新为 "✅ 完成"

---

### 阶段 8: 归档执行计划 (5%)

[`归档执行计划步骤`](./exec-plans/templates/how-to-complete-exec-plan.md)

---

### 阶段 9: Git 提交 (5%)

> ✅ **Git 提交规范**: 确保版本控制的可追溯性和一致性

**提交前验证**:

```bash
# 确认所有更改已暂存
git status
```

**标准提交流程**:

```bash
git add .
git commit -m "✅ {TASK_ID}: {TASK_NAME} 完成

- 实现内容简述
- 质量指标：Health Score XX/100
- 测试覆盖：XX%
- E2E 测试：100% 通过
- 执行计划已归档"
```

**提交信息格式要求**:

- **标题行**: `✅ {TASK_ID}: {TASK_NAME} 完成`
- **正文**:
  - 第一行：实现内容简述（1-2 句话）
  - 第二行：质量指标数据
  - 第三行：测试覆盖率
  - 第四行：E2E 测试结果
  - 第五行：执行计划状态

---

## 🎯 关键成功要素

### 1. **测试先行 (Test-First)** 🔥

- 先写测试再实现功能
- 确保测试覆盖率≥70%
- E2E 测试必须覆盖核心流程
- 测试必须 100% 通过

### 2. **计划驱动 (Plan-Driven)** 📋

- **任务选择后立即创建执行计划**
- **及时记录阻塞问题**
- **完成后立即归档**
- 📖 参考：[`执行计划使用指南`](./exec-plans/index.md)

### 3. **持续验证 (Continuous Validation)**

- 每次修改后运行 `harness:check`
- 及时修复类型错误和格式化问题
- 保持 Health Score ≥95

### 4. **架构约束 (Architecture Constraints)**

- 严格遵守分层架构
- 遵循单向依赖原则
- 使用路径别名 (@/)
- **遵守测试架构约束 (TEST-001 ~ TEST-005)** 🔥

### 5. **文档驱动 (Documentation-Driven)**

- **更新Sprint计划**
- **执行计划全程记录** 📋
- **包含 Harness 合规性声明**
- 经验教训沉淀

### 6. **自动化优先 (Automation-First)**

- 使用 `npm run harness:fix` 自动修复
- 利用 Prettier 保持一致性
- 依赖 CI/CD 验证

### 7. **Git 规范 (Git Convention)** ✅

- 遵循标准提交信息格式
- 包含任务 ID 和质量指标
- 确保执行计划已归档
- 本地验证后推送

---

## 🔗 相关资源

### 工具命令

```bash
npm run harness:check      # 架构健康检查
npm run harness:fix        # 自动修复问题
npm run test:e2e          # 运行 E2E 测试 🔥
npm run format             # 格式化代码
npx tsc --noEmit          # TypeScript 类型检查
cd src-tauri; cargo check # Rust 编译检查
```

---

## 🎓 最佳实践

### ✅ DO (应该做的)

1. 开发前阅读架构规则和测试约束
2. 任务选择后立即创建执行计划
3. 先写测试再实现功能 (TDD)
4. 编写 E2E 测试覆盖核心流程
5. 频繁运行 `harness:check`
6. 使用自动化工具修复问题
7. 每日更新执行日志
8. 及时记录阻塞问题
9. 保持 Health Score ≥90
10. 确保 E2E 测试自动管理服务器
11. 完成后立即归档执行计划
12. **遵循 Git 提交规范独立归档** ✅

### ❌ DON'T (不应该做的)

1. 跳过单元测试
2. 跳过 E2E 测试
3. 跳过执行计划直接开发
4. 忽略 TypeScript 错误
5. 手动修改格式化后的代码
6. 不更新文档就标记完成
7. 违反架构约束 (如循环依赖)
8. 在 Health Score <90 时提交
9. E2E 测试依赖真实 API
10. 测试覆盖率不达标就提交
11. 忘记归档执行计划
12. 在执行计划中写套话
13. **Git 提交信息不规范** ❌

---

## 📈 质量门禁

| 指标             | 目标   | 实际       | 评级          |
| ---------------- | ------ | ---------- | ------------- |
| TypeScript 编译  | 通过   | ✅ 通过    | ⭐⭐⭐⭐⭐    |
| ESLint 检查      | 通过   | ✅ 通过    | ⭐⭐⭐⭐⭐    |
| Prettier 格式化  | 一致   | ✅ 一致    | ⭐⭐⭐⭐⭐    |
| Rust cargo check | 通过   | ✅ 通过    | ⭐⭐⭐⭐⭐    |
| 单元测试覆盖率   | ≥70%   | ✅ 95%     | ⭐⭐⭐⭐⭐    |
| E2E 测试通过     | 100%   | ✅ 100%    | ⭐⭐⭐⭐⭐ 🔥 |
| 架构约束         | 无违规 | ✅ 无违规  | ⭐⭐⭐⭐⭐    |
| Harness Score    | ≥90    | ✅ 100/100 | ⭐⭐⭐⭐⭐    |
| 执行计划完整     | 是     | ✅ 是      | ⭐⭐⭐⭐⭐    |
| **Git 提交规范** | 符合   | ✅ 符合    | ⭐⭐⭐⭐⭐    |

**综合评分**: ⭐⭐⭐⭐⭐ **Excellent**

---

## 🔄 持续改进循环

```
graph LR
    A[执行任务] --> B[创建执行计划]
    B --> C[开发实施]
    C --> D[质量验证]
    D --> E{Health Score ≥ 90?}
    E -->|否 | F[修复问题]
    F --> D
    E -->|是 | G[完成执行计划]
    G --> H[Git 提交]
    H --> I[复盘总结]
    I --> J[经验沉淀]
    J --> K[改进下一个任务]
    K --> A
```

**关键点**:

- 📋 执行计划贯穿全流程
- 🔥 质量验证是关键关卡
- ✅ **Git 提交独立归档确保版本规范**
- 🔄 复盘促进持续改进
- 📊 数据驱动决策

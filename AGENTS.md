# AI Agent 导航地图

> **Harness Engineering 核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)
> 
> **适用范围**: 本项目所有 AI Agent 和开发者  
> **最后更新**: 2026-03-23

## 🎯 快速定位

### 项目概述
**OPC-HARNESS** - AI 驱动的桌面应用 MVP
- **技术栈**: React 18 + TypeScript 5 + Tauri v2 + Rust + SQLite
- **核心功能**: Vibe Design → Vibe Coding → Vibe Marketing

### 📍 关键文档（按优先级）

#### 1️⃣ 必读入口 ⭐⭐⭐
- [`src/AGENTS.md`](./src/AGENTS.md) - 前端开发规范
- [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 后端规范
- **[📘 Harness Engineering 开发流程](./docs/references/harness-engineering-process.md)** - **标准 7 阶段开发流程** 🔥

#### 2️⃣ 架构与约束 ⭐⭐
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 系统架构设计
- [`docs/references/architecture-rules.md`](./docs/references/architecture-rules.md) - 架构约束规则 (含 FE-ARCH/BE-ARCH/TEST) ⭐
- **[📗 E2E 测试规范](./e2e/app.spec.ts)** - **端到端测试示例** 🔥

#### 3️⃣ 测试与验证 ⭐⭐⭐
- 单元测试：`npm run test:unit` - 运行所有单元测试（覆盖率≥70%）⭐
- E2E 测试：`npm run test:e2e` - E2E 测试（智能运行，自动管理服务器）🔥
- 架构检查：`npm run harness:check` - 架构健康检查（目标 100/100）⭐

#### 4️⃣ 最佳实践 ⭐
- [`docs/references/best-practices.md`](./docs/references/best-practices.md) - 编码最佳实践
- **[📙 规范更新总结](./docs/references/HARNESS_ENGINEERING_UPDATE_SUMMARY.md)** - **最新测试约束说明** 🔥

---

## 🏗️ Harness Engineering 三大支柱

### 1. 上下文工程 (Context Engineering)

**目的**: 帮助 AI Agent 快速理解项目背景和任务

```
渐进式披露层级:
AGENTS.md (导航) → 模块 AGENTS.md (规范) → docs/ (详细设计)
```

**关键文件**:
- 本文件 - 导航地图
- `docs/design-docs/` - 技术决策记录
- `docs/exec-plans/` - 执行计划和进度
- `docs/MAINTENANCE.md` - 文档维护清单 ⭐

### 2. 架构约束 (Architectural Constraints)

**目的**: 确保 AI 生成的代码符合项目规范

**强制执行方式**:
- ESLint + TypeScript - 前端代码规范
- cargo clippy - Rust 代码规范
- 自定义架构规则 - 防止循环依赖和架构漂移

**核心约束**:
``typescript
// ✅ 允许的数据流
Component → Hook → Store → Commands → Services → DB

// ❌ 禁止的依赖
Store → Component  // 状态层不可依赖 UI 层
Services → Commands // 服务层不可依赖命令层
DB → Services      // 数据库层不可依赖服务层
```

**测试架构约束** 🔥:
``typescript
// TEST-001: 所有功能必须有单元测试覆盖 (≥70%)
src/hooks/useFeature.ts
src/hooks/useFeature.test.ts  // 必须存在

// TEST-002: 核心流程必须有 E2E 测试覆盖
e2e/app.spec.ts  // 应用启动、导航、配置流程

// TEST-003: 测试必须先于功能完成 (TDD)
// 先写测试 → 实现功能 → 持续重构

// TEST-004: E2E 测试必须独立运行（使用 Mock）
// TEST-005: 测试覆盖率不达标禁止合并
```

### 3. 反馈回路 (Feedback Loops)

**目的**: 快速发现问题并持续改进

**自动化检查**:
```
# 提交前必跑
npm run harness:check          # 架构健康检查（6 项验证）

# 完整验证（包含文档和死代码）
npm run harness:check -- -All

# 自动修复格式问题
npm run harness:fix
```

**质量门禁**:
- ✅ TypeScript 编译通过
- ✅ ESLint 无错误（警告≤0）
- ✅ Prettier 格式化一致
- ✅ Rust cargo check 通过
- ✅ 单元测试覆盖率 ≥70%
- ✅ **E2E 测试 100% 通过** 🔥
- ✅ 架构约束无违规
- ✅ **Health Score ≥90（目标 100/100）**

---

## 🚀 快速开始

### 对于 AI Agent ⭐⭐⭐

**标准开发流程（7 阶段）** 🔥:

``mermaid
graph TD
    A[1. 任务选择] --> B[2. 架构学习]
    B --> C[3. 测试设计]
    C --> D[4. 开发实施]
    D --> E[5. 质量验证]
    E --> F[6. 文档更新]
    F --> G[7. 完成交付]
```

**详细步骤**:

#### 阶段 1: 任务选择
1. 查阅 [`docs/exec-plans/active/MVP版本规划.md`](./docs/exec-plans/active/MVP版本规划.md)
2. 选择 P0/P1 优先级任务
3. 确认任务独立性和验收标准

#### 阶段 2: 架构学习
1. 阅读 [`architecture-rules.md`](./docs/references/architecture-rules.md) - FE-ARCH/BE-ARCH/TEST约束
2. 理解分层架构和依赖方向
3. 明确技术实现方案

#### 阶段 3: 测试设计 🔥
1. **单元测试设计** - 覆盖率目标≥70%
   ```typescript
   // src/hooks/useFeature.test.ts
   describe('useFeature', () => {
     it('should initialize correctly')
     it('should handle success case')
     it('should handle error case')
   })
   ```

2. **E2E 测试设计** - 核心流程覆盖 🔥
   ```typescript
   // e2e/app.spec.ts
   describe('Application', () => {
     it('should load successfully')
     it('should navigate to Settings')
     it('should detect tools')
   })
   ```

#### 阶段 4: 开发实施
1. **后端 (Rust)** - 完整类型、错误处理、日志记录
   ```rust
   // src-tauri/src/feature/mod.rs
   pub struct FeatureService { /* ... */ }
   
   #[cfg(test)]
   mod tests { /* 单元测试 */ }
   ```

2. **前端 (TypeScript/React)** - 类型安全、Hooks 封装
   ```typescript
   // src/hooks/useFeature.ts
   export function useFeature() {
     // 通过 Hook 封装 invoke
   }
   ```

#### 阶段 5: 质量验证 ⭐⭐⭐
```bash
# 1. 运行单元测试
npm run test:unit              # 必须 100% 通过
npm run test:unit -- --coverage # 覆盖率≥70%

# 2. 运行 E2E 测试 🔥
npm run test:e2e              # 核心流程验证

# 3. 架构健康检查
npm run harness:check          # 目标：100/100

# 4. 循环修复直至 Excellent
while [ Health Score -lt 100 ]; do
  npm run harness:fix
  npm run test:unit
  npm run test:e2e
done
```

#### 阶段 6: 文档更新
1. 更新 [`MVP版本规划.md`](./docs/exec-plans/active/MVP版本规划.md) - 标记任务完成
2. 创建任务完成报告 - `task-completion-{TASK_ID}.md`
3. 添加 Harness 合规性声明

#### 阶段 7: 完成交付
**交付检查清单** ✅:
- [x] TypeScript 类型检查通过
- [x] ESLint 无错误
- [x] Prettier 格式化一致
- [x] Rust cargo check 通过
- [x] 单元测试 100% 通过（覆盖率≥70%）
- [x] **E2E 测试 100% 通过** 🔥
- [x] Health Score = 100/100
- [x] 架构约束无违规
- [x] 文档已更新

**Git 提交示例**:
```
feat: 实现 VD-010 OpenAI 适配器

- 添加 OpenAIProvider Rust 实现
- 创建 useOpenAIProvider Hook
- 编写单元测试 (Rust 4 个 + TS 5 个)
- 编写 E2E 测试 (6 个场景) 🔥
- 通过 Harness Engineering 验证 (100/100)
- 测试覆盖率：85%

Closes #VD-010
```

---

### 对于人类开发者

1. **理解架构** - 阅读 [`ARCHITECTURE.md`](./ARCHITECTURE.md)
2. **遵循规范** - 查看 `src/AGENTS.md` 或 `src-tauri/AGENTS.md`
3. **运行测试** - 使用 `npm run test:unit` 和 `npm run test:e2e`
4. **追踪进度** - 查看 `docs/exec-plans/active/`

---

## 📁 文档组织原则

### 渐进式披露

```
Level 1: AGENTS.md (本文件)     ← 导航地图，< 100 行
    ↓
Level 2: src/AGENTS.md          ← 模块规范，具体规则  
    ↓
Level 3: docs/*                 ← 详细设计，技术细节
```

### 关注点分离

- **技术方案** → `docs/design-docs/`
- **执行计划** → `docs/exec-plans/`
- **产品需求** → `docs/product-specs/`
- **参考资料** → `docs/references/`
- **自动生成** → `docs/generated/`

---

## 🔧 常用命令

### 日常开发 ⭐⭐⭐

```bash
# ========== 测试流程（按顺序执行）==========

# 1. 单元测试（必须 100% 通过）
npm run test:unit              # 运行所有单元测试
npm run test:unit -- --coverage # 生成覆盖率报告（目标≥70%）

# 2. E2E 测试（核心流程验证）🔥
npm run test:e2e              # 智能运行，自动管理服务器
npx vitest run e2e           # 直接运行 Vitest E2E

# 3. 架构健康检查（6 项全面验证）
npm run harness:check          # TypeScript+ESLint+Prettier+Rust+ 依赖 + 结构

# 4. 自动修复格式问题
npm run harness:fix            # 自动修复 ESLint 和 Prettier 问题

# ========== 完整验证流程 ==========

# 提交前完整检查（推荐）
npm run test:unit && \
npm run test:e2e && \
npm run harness:check

# 快速检查（仅核心项）
npm run harness:quick
```

### 定期维护

```bash
# 完整检查（包含文档和死代码）
npm run harness:check -- -All

# 清理临时文件和构建产物
npm run harness:gc

# 验证 Tauri 应用
npm run harness:verify:tauri

# 验证文档结构
npm run harness:validate:docs
```

### 开发调试

```bash
# 监听模式运行测试（文件变化自动重新运行）
npx vitest

# 带 UI 界面的测试
npx vitest --ui

# 只运行特定测试文件
npx vitest run src/hooks/*.test.ts
npx vitest run e2e/app.spec.ts

# TypeScript 类型检查
npx tsc --noEmit

# Rust 编译检查
cd src-tauri && cargo check
```

---

## 📊 质量门禁标准

| 检查项 | 满分 | 通过标准 | 说明 |
|--------|------|---------|------|
| TypeScript 类型检查 | 20 | 编译通过 | `tsc --noEmit` |
| ESLint 代码规范 | 15 | 无错误 | `npm run lint` |
| Prettier 格式化 | 10 | 格式统一 | `prettier --check` |
| Rust 编译检查 | 25 | cargo check 通过 | 无编译错误 |
| **单元测试覆盖率** 🔥 | 20 | **≥70%** | Vitest + Cargo Test |
| **E2E 测试** 🔥 | 10 | **100% 通过** | 核心流程覆盖 |
| 架构约束 | 10 | 无违规 | FE-ARCH/BE-ARCH/TEST |

**总分**: 110 分（含 E2E 测试额外 10 分）

**评分等级**:
- **90-110**: 优秀 ✨ - 可以安全合并
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议合并

**Health Score 计算**:
``bash
# 优秀 (Excellent): 100/100
✅ TypeScript 编译通过
✅ ESLint 无错误
✅ Prettier 格式化一致
✅ Rust cargo check 通过
✅ 依赖完整性
✅ 目录结构完整

# 额外加分项 🔥
✅ 单元测试覆盖率 ≥70% (+10 分)
✅ E2E 测试 100% 通过 (+10 分)
```

---

## 🎓 学习路径

### 新手入门（1-2 小时）⭐⭐⭐

#### 第 1 步：理解 Harness Engineering（30 分钟）
1. ✅ 阅读本文件 - 10 分钟
2. ✅ **[精读 Harness Engineering 开发流程](./docs/references/harness-engineering-process.md)** - 20 分钟 🔥
   - 7 阶段标准流程
   - TEST-001 ~ TEST-005 测试约束
   - E2E 测试要求

#### 第 2 步：掌握架构约束（30 分钟）
1. ✅ 浏览对应模块的 AGENTS.md - 20 分钟
   - [`src/AGENTS.md`](./src/AGENTS.md) - 前端规范
   - [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 规范
2. ✅ 运行 `npm run harness:check` 并理解输出 - 10 分钟

#### 第 3 步：实践测试流程（40 分钟）🔥
1. ✅ **编写单元测试** - 20 分钟
   ```bash
   npm run test:unit -- --coverage
   # 查看覆盖率报告
   open coverage/index.html
   ```

2. ✅ **编写 E2E 测试** - 20 分钟
   ```bash
   npm run test:e2e
   # 查看测试报告
   open docs/testing/e2e-reports/
   ```

#### 第 4 步：熟悉最佳实践（20 分钟）
- ✅ 阅读 [`best-practices.md`](./docs/references/best-practices.md) - 20 分钟
- ✅ 查看示例任务完成报告 - 10 分钟
  - [`task-completion-vd-010.md`](./docs/exec-plans/active/task-completion-vd-010.md)
  - [`task-completion-infra-011.md`](./docs/exec-plans/active/task-completion-infra-011.md)

---

### 进阶提升（1 天）⭐⭐

#### 深度学习
1. 📖 精读 [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 2 小时
2. 📝 学习所有架构决策记录 - 2 小时
3. 🔧 尝试自定义检查规则 - 2 小时
4. 📚 贡献新的最佳实践 - 2 小时

#### 实战演练
1. 💻 完成一个小功能开发（遵循 7 阶段流程）
2. 🧪 编写完整的单元测试 + E2E 测试
3. ✅ 通过 Harness Engineering 验证（100/100）
4. 📝 创建任务完成报告

---

### 专家精通（持续）⭐⭐⭐

#### 架构优化
- 参与架构规则制定和更新
- 设计和实现新的架构约束
- 优化 Harness Engineering 流程

#### 质量保障
- 建立测试覆盖率仪表板
- 集成 CI/CD 自动化验证
- 实施视觉回归测试和性能测试

---

## ❓ 常见问题

### Q: Harness Engineering 是什么？
A: 一套为 AI 协作优化的工程实践体系，通过构建受控环境让 AI 能够可靠地完成编码任务。核心理念是"质量内建，而非事后检查"。

### Q: 为什么需要这个？
A: 
- 🤖 AI 生成的代码质量参差不齐
- 📋 团队成员 coding style 不一致
- 🗂️ 项目结构容易混乱
- 📉 技术债务难以发现和管理
- 🔍 **缺少端到端验证，回归 bug 风险高**

### Q: 7 阶段开发流程是什么？🔥
A: 详见 **[Harness Engineering 开发流程](./docs/references/harness-engineering-process.md)**，包括：
1. 任务选择 → 2. 架构学习 → 3. 测试设计 → 4. 开发实施 → 5. 质量验证 → 6. 文档更新 → 7. 完成交付

### Q: E2E 测试的要求是什么？🔥
A: 
- **必需性**: 所有核心功能必须有 E2E 测试覆盖（TEST-002）
- **覆盖场景**: 应用启动、核心页面导航、关键配置流程、API 可达性
- **技术要求**: 
  - ✅ 自动管理开发服务器生命周期
  - ✅ 使用 Mock 数据（不依赖真实 API）
  - ✅ 生成 HTML 测试报告
  - ✅ 优雅清理资源
- **示例代码**: [`e2e/app.spec.ts`](./e2e/app.spec.ts)

### Q: 如何向 AI 提问？
A: 参考 [`best-practices.md`](./docs/references/best-practices.md) 中的"如何向 AI 提问"章节。

### Q: 可以自定义规则吗？
A: 可以！编辑 [`architecture-rules.md`](./docs/references/architecture-rules.md) 添加自定义规则。新增测试约束需遵循 TEST-001 ~ TEST-005。

### Q: Health Score 如何计算？
A: 基于 6 项核心检查（TypeScript、ESLint、Prettier、Rust、依赖、结构），每项满分约 16.67 分。额外加分项：单元测试覆盖率≥70% (+10 分)、E2E 测试 100% 通过 (+10 分)。目标分数：100/100。

### Q: 测试覆盖率不达标怎么办？
A: 
1. 运行 `npm run test:unit -- --coverage` 查看覆盖率报告
2. 补充边界条件和错误处理场景的测试
3. 确保每个公共函数都有对应的测试用例
4. 若覆盖率仍不达标，禁止合并代码（TEST-005）

---

## 🔗 相关资源

### 官方文档
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [Tauri v2 官方文档](https://v2.tauri.app/)
- [React 官方文档](https://react.dev/)
- [TypeScript 手册](https://www.typescriptlang.org/docs/)

### 工具链
- [ESLint - 代码规范检查](https://eslint.org/)
- [Prettier - 代码格式化](https://prettier.io/)
- [cargo - Rust 包管理](https://doc.rust-lang.org/cargo/)
- [Vitest - 单元测试框架](https://vitest.dev/)

---

**维护者**: OPC-HARNESS Team  
**版本**: 3.0.0 (新增 Harness Engineering 7 阶段流程和 E2E 测试要求) 🔥  
**最后更新**: 2026-03-23  
**变更说明**: 
- ✅ 新增 7 阶段标准开发流程
- ✅ 新增 TEST-001 ~ TEST-005 测试架构约束
- ✅ 新增 E2E 测试要求和示例
- ✅ 更新质量门禁标准（含单元测试+E2E 测试）
- ✅ 更新学习路径和常见问题

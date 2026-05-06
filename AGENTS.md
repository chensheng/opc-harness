# AI Agent 导航地图

> **Harness Engineering**: "人类掌舵，Agent 执行"  
> **适用范围**: OPC-HARNESS 项目所有 AI Agent 和开发者  
> **最后更新**: 2026-05-06

---

开发前必须先阅读 [`Harness Engineering 开发流程`](./docs/dev_workflow.md)，必须严格遵循该标准开发流程。

## 🎯 快速入口（按优先级）

### ⭐⭐⭐ 必读核心

- [`Harness Engineering 开发流程`](./docs/dev_workflow.md) - 标准开发流程
- [`Sprint 规划索引`](./docs/sprint-plans/index.md) - Sprint计划导航
- [`src/AGENTS.md`](./src/AGENTS.md) - 前端开发规范（React + TypeScript）
- [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 后端规范

### ⭐⭐ 架构与约束

- [`ARCHITECTURE.md`](./docs/design-docs/system-architecture.md) - 系统架构设计
- [`docs/design-docs/architecture-rules.md`](./docs/design-docs/architecture-rules.md) - 架构约束（FE/BE/TEST）🔥
- [`e2e/app.spec.ts`](./e2e/app.spec.ts) - E2E 测试示例 🔥

### ⭐ 测试与验证

```bash
npm run harness:check      # 架构健康检查（目标 100/100）
```

---

## 🏗️ 三大支柱

### 1. 上下文工程

**披露层级**: AGENTS.md → 模块 AGENTS.md → docs/

**关键文档**:

- `docs/design-docs/` - 技术决策记录 ([迁移指南](./docs/MIGRATION_GUIDE.md))
- `docs/product-specs/` - 产品规格 (参见 OpenSpec capabilities)
- `docs/exec-plans/` - 执行计划 (历史计划已归档到 OpenSpec)
- `docs/sprint-plans/` - Sprint计划
- `openspec/specs/` - OpenSpec Capabilities (新增能力规范)
- `openspec/changes/archive/` - 已归档的历史变更

### 2. 架构约束

**数据流规则**:

```typescript
// ✅ 允许
Component → Hook → Store → Commands → Services → DB

// ❌ 禁止
Store → Component    // 状态层不可依赖 UI 层
Services → Commands  // 服务层不可依赖命令层
DB → Services        // 数据库层不可依赖数据层
```

**代码规模约束** 🔥:

- CODE-001: 单个文件代码行数不得超过 500 行
- CODE-002: 超过 500 行的文件必须进行模块化拆分
- CODE-003: 拆分策略遵循 [`React组件模块化拆分规范`](./src/AGENTS.md) 或 [`Rust模块化拆分最佳实践`](./src-tauri/AGENTS.md)
- CODE-004: 拆分后需确保类型安全、测试覆盖和公共接口兼容性

**测试约束** 🔥:

- TEST-001: 所有功能必须单元测试覆盖（≥70%）
- TEST-002: 核心流程必须 E2E 测试覆盖
- TEST-003: 测试先行（TDD）
- TEST-004: E2E 测试独立（Mock 数据）
- TEST-005: 覆盖率不达标禁止合并

### 3. 反馈回路

**自动化检查**:

```bash
npm run harness:check          # 提交前必跑（完整验证）
npm run harness:fix            # 自动修复格式问题
```

**质量门禁**:

- ✅ TypeScript 编译通过
- ✅ ESLint 无错误
- ✅ Prettier 格式化一致
- ✅ Rust cargo check 通过
- ✅ 单元测试≥70% 🔥
- ✅ E2E 测试 100% 通过 🔥
- ✅ Health Score = 100/100

---

## 📁 文档结构

```
Level 1: AGENTS.md (本文件)     ← 导航地图
    ↓
Level 2: docs/*                 ← 详细设计
```

**目录组织**:

```
Level 1: AGENTS.md (本文件)     ← 导航地图
    ↓
Level 2: docs/                  ← 详细文档
    ├── dev_workflow.md         ← 开发流程 🔥
    ├── exec-plans/             ← 执行计划
    ├── product-specs/          ← 产品规格
    └── design-docs/            ← 设计文档
    └── sprint-plans/           ← Sprint计划
```

---

## 🔧 命令速查

### 日常开发

```bash
# 测试流程
npm run harness:check          # 架构检查（完整验证）
npm run harness:fix            # 自动修复
npm run test:e2e               # E2E 测试
```

### 提交前验证

```bash
# 完整验证（默认，包含文档和死代码检测）
npm run harness:check
```

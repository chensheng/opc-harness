# AI Agent 导航地图

> **Harness Engineering**: "人类掌舵，Agent 执行"  
> **适用范围**: OPC-HARNESS 项目所有 AI Agent 和开发者  
> **最后更新**: 2026-05-06
> **文档体系**: OpenSpec 标准化工作流 ✅

---

开发前必须先阅读 [`Harness Engineering 开发流程`](./openspec/specs/development-workflow/spec.md)，必须严格遵循该标准开发流程。

## 🎯 快速入口（按优先级）

### ⭐⭐⭐ 必读核心

- [`Harness Engineering 开发流程`](./openspec/specs/development-workflow/spec.md) - 标准开发流程
- [`Sprint 规划指南`](./openspec/specs/sprint-planning/spec.md) - Sprint计划导航
- [`src/AGENTS.md`](./src/AGENTS.md) - 前端开发规范（React + TypeScript）
- [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 后端规范

### ⭐⭐ 架构与约束

- [`系统架构设计`](./openspec/specs/design-documentation/spec.md) - 分层架构、数据流规则、代码规模约束
- [`e2e/app.spec.ts`](./e2e/app.spec.ts) - E2E 测试示例 🔥

### ⭐ 测试与验证

```bash
npm run harness:check      # 架构健康检查（目标 100/100）
```

---

## 🏗️ 三大支柱

### 1. 上下文工程

**披露层级**: AGENTS.md → 模块 AGENTS.md → OpenSpec Specs

**关键文档**:

- `openspec/specs/` - OpenSpec Capabilities (18 个能力规范)
- `openspec/changes/archive/` - 已归档的历史变更
- `openspec/changes/*/proposal.md` - 活跃变更的提案
- `openspec/changes/*/design.md` - 活跃变更的设计文档
- `openspec/changes/*/tasks.md` - 活跃变更的实施任务

**Capabilities 列表**:

| 类别 | Capability | 说明 |
|------|-----------|------|
| **Vibe 系列** | vibe-coding | AI 驱动的自主编码系统 |
| | vibe-design | AI 驱动的产品设计助手 |
| | vibe-marketing | 营销数据分析与内容生成 |
| **开发流程** | development-workflow | Harness Engineering 开发流程 |
| | execution-tracking | 执行计划与技术债务追踪 |
| | sprint-planning | Sprint 计划与归档机制 |
| **架构与设计** | design-documentation | 架构文档、ADR、设计决策 |
| | product-specification | 产品规格层级体系 |
| | data-storage | SQLite 数据库集成与 Repository 模式 |
| **编码规范** | coding-harness | 自主编码 Harness 规范 |
| | best-practices | 项目开发最佳实践 |
| **Agent 系统** | agent-initialization | Agent 初始化与环境配置 |
| | agent-observability | Agent 可观测性与监控 |
| | agent-tracing | Agent 追踪与日志 |
| | agent-alerting | Agent 警报与通知 |
| **项目管理** | git-repository-management | Git 仓库管理 |
| | project-creation | 项目创建与初始化 |

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
Level 2: openspec/              ← OpenSpec 工作流
    ├── specs/                  ← Capabilities (18 个)
    ├── changes/
    │   ├── archive/            ← 已归档的变更
    │   └── <active>/           ← 活跃变更 (proposal/design/tasks)
    └── config.yaml             ← OpenSpec 配置
```

**OpenSpec 工作流**:

```
1. /opsx:propose <change-name>  → 创建变更提案
2. /opsx:continue <change-name> → 完善 artifacts (design/specs/tasks)
3. /opsx:apply <change-name>    → 实施任务
4. /opsx:archive <change-name>  → 归档完成的变更
```

---

## 🔧 命令速查

### 日常开发

```bash
# OpenSpec 工作流
/opsx:propose <name>            # 创建新变更
/opsx:continue <name>           # 继续完善 artifacts
/opsx:apply <name>              # 实施变更任务
/opsx:archive <name>            # 归档完成的变更

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

---

## 📝 迁移说明

> **重要**: 本文档于 2026-05-06 完成从 `docs/` 到 OpenSpec 的迁移。
>
> **历史文档**: 所有旧的 `docs/` 目录内容已归档到 `openspec/changes/archive/`。
>
> **过渡期**: 如发现任何断裂的链接或缺失的文档,请报告并创建新的 OpenSpec change 进行修复。

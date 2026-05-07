# AI Agent 导航地图

> **Harness Engineering**: "人类掌舵，Agent 执行"  
> **适用范围**: OPC-HARNESS 项目所有 AI Agent 和开发者  
> **最后更新**: 2026-05-07
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

- `openspec/specs/` - 18 个能力规范
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

**测试约束** 🔥:

- TEST-001: 所有功能必须单元测试覆盖（≥70%）
- TEST-002: 核心流程必须 E2E 测试覆盖
- TEST-003: 测试先行（TDD）

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
- ✅ 单元测试≥70%
- ✅ E2E 测试 100% 通过
- ✅ Health Score = 100/100

---

## 📚 详细文档

如需了解更多信息，请参考以下文档：

### OpenSpec 工作流

- [`development-workflow spec`](./openspec/specs/development-workflow/spec.md) - OpenSpec 工作流详细说明、最佳实践和常见问题
- [`agent-initialization spec`](./openspec/specs/agent-initialization/spec.md) - Agent 初始化与环境配置

### 架构与设计

- [`design-documentation spec`](./openspec/specs/design-documentation/spec.md) - SDD 软件设计文档规范
- [`harness-sdd-integration spec`](./openspec/specs/harness-sdd-integration/spec.md) - Harness 工作流与 SDD 整合

### 其他能力规范

- 查看 `openspec/specs/` 目录获取全部 18 个能力规范

---

## 🔍 前端 Console Bridge

**功能**: 在开发模式下,前端 console 日志自动转发到后端 Rust 日志系统

**启用方式**:

- **开发模式**: 自动启用 (`npm run tauri:dev`)
- **生产模式**: 默认禁用,可通过环境变量 `VITE_ENABLE_CONSOLE_BRIDGE=true` 启用

**使用示例**:

```typescript
// 前端代码 - 所有 console 调用自动转发到后端
console.log('User logged in', { userId: 123 })
console.error('Failed to fetch data', error)
```

**特性**:

- ✅ 保留浏览器 DevTools Console 功能
- ✅ 支持所有 console 方法(log/info/warn/error/debug)
- ✅ 仅开发模式启用,无性能影响

---

## 🔧 命令速查

### OpenSpec 工作流

```bash
/opsx:propose <name>            # 创建新变更
/opsx:apply <name>              # 实施变更任务
/opsx:archive <name>            # 归档完成的变更
openspec status --change <name> # 查看变更状态
openspec list                   # 列出所有活跃变更
```

### 测试流程

```bash
npm run harness:check          # 架构检查（完整验证）
npm run harness:fix            # 自动修复
npm run test:e2e               # E2E 测试
```

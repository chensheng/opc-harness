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

## 🚀 OpenSpec 快速入门

OpenSpec 是本项目的核心工作流,通过结构化的变更管理确保每次修改都有完整的提案、设计和验证。

### 前置要求

确保已安装 OpenSpec CLI:

```bash
# 全局安装 OpenSpec
npm install -g @fission-ai/openspec

# 验证安装
openspec --version
```

### 第一个变更 (Step-by-Step)

让我们创建一个简单的文档更新作为第一个变更:

```bash
# 步骤 1: 创建变更提案
/opsx:propose update-readme-typo

# AI Agent 自动生成:
# - proposal.md (为什么需要这个变更)
# - design.md (如何实现)
# - specs/ (能力规范,如需要)
# - tasks.md (实施任务列表)

# 步骤 2: 查看生成的 artifacts
ls openspec/changes/update-readme-typo/

# 步骤 3: 实施任务
/opsx:apply update-readme-typo

# AI Agent 自动执行 tasks.md 中的任务
# 每完成一个任务自动标记为 [x]

# 步骤 4: 质量验证
# 系统自动创建:
# - quality-check.md (Health Score 等)
# - runtime-check.md (运行时验证)

# 步骤 5: 归档完成的变更
/opsx:archive update-readme-typo

# 变更被移动到:
# openspec/changes/archive/YYYY-MM-DD-update-readme-typo/
```

### 完整工作流概览

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐     ┌──────────┐
│  Propose    │────▶│   Design     │────▶│   Apply     │────▶│ Archive  │
│  创建提案   │     │  完善设计    │     │  实施任务   │     │  归档    │
└─────────────┘     └──────────────┘     └─────────────┘     └──────────┘
     │                     │                    │                   │
     ▼                     ▼                    ▼                   ▼
  proposal.md          design.md            执行 tasks         移至 archive/
  (Why & What)        (How)                (Code Changes)      (历史记录)
                       specs/               quality-check.md
                       (Requirements)       runtime-check.md
```

### 常用命令速查

```bash
# 创建新变更
/opsx:propose <change-name>

# 继续完善 artifacts (如果中途停止)
/opsx:continue <change-name>

# 实施变更任务
/opsx:apply <change-name>

# 查看变更状态
openspec status --change <change-name>

# 列出所有活跃变更
openspec list

# 归档完成的变更
/opsx:archive <change-name>

# 查看已归档的变更
ls openspec/changes/archive/
```

---

## 💡 OpenSpec 最佳实践

### 何时使用 `/opsx:propose` vs `/opsx:explore`

**使用 `/opsx:propose` 当**:

- ✅ 你有明确的需求和实现思路
- ✅ 变更范围清晰,可以定义具体的任务
- ✅ 需要完整的 artifacts (proposal/design/specs/tasks)

**使用 `/opsx:explore` 当**:

- 🔍 你还在探索问题,不确定最佳方案
- 🔍 需要 brainstorming 或技术调研
- 🔍 想先理清思路再创建正式变更

### 变更粒度建议

**小而频繁的变更** (推荐):

- ✅ 每个变更聚焦单一功能或修复
- ✅ 易于审查和测试
- ✅ 降低合并冲突风险
- ✅ 更容易回滚

**示例**:

```bash
# ✅ 好的粒度
/opsx:propose add-user-login-validation
/opsx:propose fix-database-connection-timeout
/opsx:propose update-readme-installation-steps

# ❌ 避免过大的变更
/opsx:propose refactor-entire-application  # 太大!
```

### Spec 编写技巧 (ADDED vs MODIFIED)

**使用 `ADDED Requirements` 当**:

- 新增功能或能力
- 之前不存在的需求

**使用 `MODIFIED Requirements` 当**:

- 修改现有需求的行为
- 扩展现有能力的范围

**示例**:

```markdown
## ADDED Requirements

### Requirement: 用户登录验证

系统 SHALL 在用户登录时验证邮箱格式和密码强度。

## MODIFIED Requirements

### Requirement: 密码策略

系统 SHALL 要求密码至少包含 8 个字符、1 个大写字母和 1 个数字。
(之前是 6 个字符,无特殊要求)
```

### 归档时机和注意事项

**何时归档**:

- ✅ 所有 tasks 已完成并标记为 `[x]`
- ✅ quality-check.md 和 runtime-check.md 已创建
- ✅ 代码已提交到 Git
- ✅ 变更通过审查和测试

**归档前检查**:

```bash
# 1. 确认所有任务完成
openspec status --change <name>

# 2. 检查是否有 delta specs 需要同步
ls openspec/changes/<name>/specs/

# 3. 验证质量检查
# 查看 quality-check.md 中的 Health Score

# 4. 归档
/opsx:archive <name>
```

**归档后**:

- 变更移至 `openspec/changes/archive/YYYY-MM-DD-<name>/`
- 可通过 Git history 追溯所有决策
- Specs 已同步到 `openspec/specs/` (如有)

---

## ❓ 常见问题

### Q: 如何开始一个新特性?

**A**: 使用以下步骤:

1. **评估需求**: 明确特性和目标
2. **创建变更**: `/opsx:propose add-new-feature`
3. **完善设计**: AI Agent 生成 proposal/design/specs/tasks
4. **实施任务**: `/opsx:apply add-new-feature`
5. **质量验证**: 检查 Health Score 和测试结果
6. **归档变更**: `/opsx:archive add-new-feature`

详细步骤参见 [🚀 OpenSpec 快速入门](#-openspec-快速入门)。

### Q: 我的变更需要 spec 吗?

**A**: 取决于变更类型:

**需要 specs 当**:

- ✅ 新增或修改系统能力 (capabilities)
- ✅ 改变用户可见的行为
- ✅ 影响多个模块的交互

**不需要 specs 当**:

- ❌ 纯文档更新 (如 README.md)
- ❌ Bug 修复 (不改变行为)
- ❌ 代码重构 (不改变外部接口)

**示例**:

```bash
# 需要 specs
/opsx:propose add-user-authentication      # 新增能力
/opsx:propose modify-password-policy       # 修改行为

# 不需要 specs
/opsx:propose fix-typo-in-readme           # 文档更新
/opsx:propose refactor-database-layer      # 内部重构
```

### Q: 如何处理冲突的变更?

**A**: OpenSpec 通过以下方式管理冲突:

1. **早期检测**: 在 propose 阶段识别潜在冲突
2. **变更隔离**: 每个变更独立开发,减少耦合
3. **顺序合并**: 先归档一个变更,再处理另一个
4. **手动解决**: 如有代码冲突,使用 Git merge 工具

**最佳实践**:

- 保持变更小而聚焦
- 频繁归档完成的变更
- 与其他开发者沟通大型变更
- 使用 `openspec list` 查看活跃变更

### Q: 归档后还能修改吗?

**A**: 可以,但需要创建新的变更:

1. **归档是不可逆的**: 已归档的变更不应直接修改
2. **创建新变更**: 如需修改,创建新的 change
3. **引用历史**: 在新 proposal 中引用归档的变更

**示例**:

```bash
# 发现归档的变更有 bug
/opsx:propose fix-bug-from-previous-change

# 在 proposal.md 中说明:
# "This change fixes a bug introduced in
#  archive/2026-05-06-add-user-auth"
```

### Q: 何时使用 explore mode?

**A**: 当你还不确定最佳方案时:

- 🔍 技术调研和可行性分析
- 🔍 Brainstorming 多种实现方案
- 🔍 探索复杂问题的解决方案
- 🔍 学习新技术或框架

**示例场景**:

```bash
# 不确定如何实现实时通知
/opsx:explore real-time-notification-options

# 探索不同的数据库方案
/opsx:explore database-selection-for-analytics
```

### Q: Health Score 是什么?为什么重要?

**A**: Health Score 是代码质量的综合评分 (0-100):

**组成部分**:

- TypeScript 类型检查
- ESLint 代码质量
- Prettier 格式化
- Rust 编译检查
- 单元测试覆盖率 (≥70%)
- E2E 测试通过率 (100%)

**重要性**:

- 🎯 目标: ≥80 (推荐 ≥90)
- 🚫 <80: 禁止合并到 main 分支
- ✅ 每次变更必须维护或提高 Health Score

**查看 Health Score**:

```bash
npm run harness:check
# 输出中包含 Overall Score
```

### Q: Harness 与 SDD 如何协同?

**A**: Harness Engineering 工作流与 SDD (Software Design Document) 实践互补:

- **Harness 工作流**: 关注开发执行和质量验证 (propose → design → apply → archive)
- **SDD 文档**: 记录长期稳定的系统架构和决策 (架构规则、ADR)
- **整合方式**:
  - 在 propose 阶段评估是否需要更新 SDD 或创建 ADR
  - 在 design 阶段参考 SDD 定义的架构规则
  - 重大架构变更时同步更新 SDD 和 OpenSpec specs
  - ADR 与 OpenSpec changes 双向引用,保持追溯性

**决策矩阵**:

| 变更类型 | SDD 更新 | ADR 创建 | Specs 更新 |
|---------|---------|---------|------------|
| 重大架构变更 | ✅ 必须 | ✅ 必须 | ✅ 必须 |
| 模块级调整 | ⚠️ 可选 | ✅ 建议 | ✅ 必须 |
| 功能实现 | ❌ 不需要 | ❌ 不需要 | ✅ 必须 |

详见 [`harness-sdd-integration spec`](./openspec/specs/harness-sdd-integration/spec.md)

---

## 📐 SDD 软件设计文档

SDD (Software Design Document) 记录系统的整体架构和设计决策,与 OpenSpec 工作流协同使用。

**核心要点**:
- **SDD vs OpenSpec design**: SDD 定义长期稳定的架构规则,OpenSpec design.md 描述具体变更的实现方案
- **ADR**: 重大架构决策必须创建 ADR (Architecture Decision Record),并在 "Related Changes" 中引用相关的 OpenSpec change
- **更新时机**: 重大架构变更时必须更新 SDD,功能实现时无需更新

**详细规范**: 参见 [`design-documentation spec`](./openspec/specs/design-documentation/spec.md) 和 [`harness-sdd-integration spec`](./openspec/specs/harness-sdd-integration/spec.md)

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

| 类别           | Capability                | 说明                                |
| -------------- | ------------------------- | ----------------------------------- |
| **Vibe 系列**  | vibe-coding               | AI 驱动的自主编码系统               |
|                | vibe-design               | AI 驱动的产品设计助手               |
|                | vibe-marketing            | 营销数据分析与内容生成              |
| **开发流程**   | development-workflow      | Harness Engineering 开发流程        |
|                | execution-tracking        | 执行计划与技术债务追踪              |
|                | sprint-planning           | Sprint 计划与归档机制               |
| **架构与设计** | design-documentation      | 架构文档、ADR、设计决策             |
|                | harness-sdd-integration   | Harness 工作流与 SDD 实践整合规范   |
|                | product-specification     | 产品规格层级体系                    |
|                | data-storage              | SQLite 数据库集成与 Repository 模式 |
| **编码规范**   | coding-harness            | 自主编码 Harness 规范               |
|                | best-practices            | 项目开发最佳实践                    |
| **Agent 系统** | agent-initialization      | Agent 初始化与环境配置              |
|                | agent-observability       | Agent 可观测性与监控                |
|                | agent-tracing             | Agent 追踪与日志                    |
|                | agent-alerting            | Agent 警报与通知                    |
| **项目管理**   | git-repository-management | Git 仓库管理                        |
|                | project-creation          | 项目创建与初始化                    |

### 2. 架构约束

**SDD 软件设计文档**: 系统架构、分层设计、数据流规则、ADR 等长期稳定的架构文档,详见 [`design-documentation spec`](./openspec/specs/design-documentation/spec.md)

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

## 🔍 前端 Console Bridge

**功能**: 在开发模式下,前端 console 日志自动转发到后端 Rust 日志系统

**启用方式**:
- **开发模式**: 自动启用 (`npm run tauri:dev`)
- **生产模式**: 默认禁用,可通过环境变量 `VITE_ENABLE_CONSOLE_BRIDGE=true` 启用

**使用示例**:
```typescript
// 前端代码 - 所有 console 调用自动转发到后端
console.log("User logged in", { userId: 123 });
console.error("Failed to fetch data", error);
console.warn("Deprecated API usage");
```

**后端日志输出**:
```
[Frontend] User logged in {"userId":123}
[Frontend] Failed to fetch data Error: ...
[Frontend] Deprecated API usage
```

**特性**:
- ✅ 保留浏览器 DevTools Console 功能
- ✅ 支持所有 console 方法(log/info/warn/error/debug)
- ✅ 安全序列化对象(处理循环引用)
- ✅ 异步发送,不阻塞 UI
- ✅ 仅开发模式启用,无性能影响

**注意事项**:
- ⚠️ 避免在循环中频繁输出大量日志(可能增加 IPC 开销)
- ⚠️ 大对象会被序列化,可能导致日志冗长
- ⚠️ 生产环境建议禁用(默认已禁用)

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

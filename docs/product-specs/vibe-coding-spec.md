# Vibe Coding 功能规格说明

> **文档版本**: v1.0  
> **创建日期**: 2026-03-24  
> **最后更新**: 2026-03-24  
> **状态**: 🔄 开发中 (28% 完成)  
> **优先级**: P0  
> **负责人**: 技术团队

---

## 📋 目录

1. [概述](#1-概述)
2. [核心架构](#2-核心架构)
3. [功能需求](#3-功能需求)
4. [技术规格](#4-技术规格)
5. [用户界面](#5-用户界面)
6. [数据流](#6-数据流)
7. [验收标准](#7-验收标准)
8. [风险与挑战](#8-风险与挑战)

---

## 1. 概述

### 1.1 产品定位

Vibe Coding 是 OPC-HARNESS 的核心模块，采用**AI 驱动的自主编码系统**，通过**多会话编排**和**HITL 检查点**机制，实现从 PRD 到可部署代码的全流程自动化。

**核心理念**: "人类掌舵，AI 执行" (Humans steer. AI execute.)

### 1.2 核心价值

| 为谁            | 解决什么问题           | 达成什么结果                 |
| --------------- | ---------------------- | ---------------------------- |
| 独立开发者      | 编码工作繁重、重复性高 | AI 自动完成 80%+编码工作     |
| 技术创业者      | 有想法但缺乏编码能力   | 自然语言描述即可生成完整产品 |
| 设计师/产品经理 | 有设计但无法实现       | 设计稿自动转化为可运行代码   |
| 内容创作者      | 需要快速发布工具       | 几小时内完成从想法到产品     |

### 1.3 关键指标

| 指标          | 定义                                | 目标值   | 当前值 |
| ------------- | ----------------------------------- | -------- | ------ |
| AI 编码成功率 | 成功完成编码任务的次数 / 总编码次数 | > 85%    | -      |
| 检查点通过率  | 自动批准的检查点数 / 总检查点数     | > 70%    | -      |
| 代码质量分数  | 通过质量门禁的会话数 / 总会话数     | > 95%    | -      |
| 平均完成时间  | 中等复杂度项目的完成时间            | < 4 小时 | -      |
| 部署成功率    | 一键部署成功的次数 / 总部署次数     | > 95%    | -      |

---

## 2. 核心架构

### 2.1 多会话编排架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Vibe Coding 架构                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  📋 PRD 文档                                                │
│       ↓                                                     │
│  ┌─────────────────┐                                       │
│  │ Initializer     │ 环境初始化、任务分解                   │
│  │ Agent           │ → 创建 Milestone                      │
│  │                 │ → 分解为 Issues                       │
│  └────────┬────────┘                                       │
│           ↓                                                 │
│  ┌─────────────────┐                                       │
│  │ Coding Agent 1  │ Issue #1: 用户认证                     │
│  │ Coding Agent 2  │ Issue #2: 登录页面                     │
│  │ Coding Agent 3  │ Issue #3: 数据库设计                   │
│  │ ...             │ (并行执行)                            │
│  └────────┬────────┘                                       │
│           ↓                                                 │
│  ┌─────────────────┐                                       │
│  │ MR Creation     │ 汇总所有提交，创建合并请求              │
│  │ Agent           │ → 生成 MR 描述                         │
│  │                 │ → 关联 Issues                         │
│  └─────────────────┘                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Agent 类型

| Agent 类型            | 职责                 | 关键能力                         | 预计耗时         |
| --------------------- | -------------------- | -------------------------------- | ---------------- |
| **Initializer Agent** | 环境初始化、任务分解 | PRD 解析、Git 初始化、Issue 创建 | 5-10 分钟        |
| **Coding Agent**      | 具体 Issue 实现      | 代码生成、测试编写、质量修复     | 15-30 分钟/Issue |
| **MR Creation Agent** | 合并请求创建         | 分支合并、回归测试、MR 生成      | 10-20 分钟       |

### 2.3 守护进程架构

```
┌─────────────────┐
│   Web 界面      │ ← 用户交互层
└────────┬────────┘
         │ HTTP/WebSocket
┌────────▼────────┐
│  守护进程       │ ← 核心编排层
│  (Daemon)       │   - 代理状态管理
│                 │   - 任务调度
│                 │   - 日志收集
└────────┬────────┘
         │
    ┌────┴────┬─────────┬─────────┐
    ↓         ↓         ↓         ↓
┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐
│Agent1│ │Agent2│ │Agent3│ │ ...  │
└──────┘ └──────┘ └──────┘ └──────┘
  AI 编码代理集群
```

**核心优势**:

- TUI/界面崩溃不影响运行中的代理
- 支持断点续传和状态恢复
- 日志文件持久化存储
- 支持远程监控和管理

---

## 3. 功能需求

### 3.1 多会话编排 (P0)

#### FR-VC-001: Initializer Agent

**功能描述**: 解析 PRD 文档，初始化环境，分解任务为可执行的 Issues

**详细需求**:

| 需求 ID     | 需求描述           | 验收标准                      | 优先级 | 状态      |
| ----------- | ------------------ | ----------------------------- | ------ | --------- |
| FR-VC-001-1 | PRD 文档解析       | 读取并理解 PRD 内容           | P0     | 📋 待开始 |
| FR-VC-001-2 | 环境检查           | 验证 Git、Node.js、Rust等工具 | P0     | 📋 待开始 |
| FR-VC-001-3 | Git 仓库初始化     | 创建或验证 Git 仓库           | P0     | 📋 待开始 |
| FR-VC-001-4 | 任务分解算法       | 将 PRD 分解为 10-20 个 Issues | P0     | 📋 待开始 |
| FR-VC-001-5 | Issue 优先级排序   | 根据依赖关系排序              | P0     | 📋 待开始 |
| FR-VC-001-6 | 触发 CP-002 检查点 | 人工审查任务分解结果          | P0     | 📋 待开始 |

**数据结构**:

```typescript
interface InitializerAgentConfig {
  prdPath: string
  workspaceRoot: string
  gitProvider: 'gitlab' | 'github' | 'local'
  maxIssues?: number // 默认：15
}

interface DecomposedIssue {
  iid: string
  title: string
  description: string
  priority: 'P0' | 'P1' | 'P2'
  dependencies?: string[]
  estimatedTime?: number // 分钟
  labels: string[]
}
```

#### FR-VC-002: Coding Agent 集群

**功能描述**: 多个 Coding Agent 并行执行不同的 Issues

**详细需求**:

| 需求 ID     | 需求描述        | 验收标准                  | 优先级 | 状态      |
| ----------- | --------------- | ------------------------- | ------ | --------- |
| FR-VC-002-1 | 单个 Issue 实现 | 完成一个 Issue 的所有要求 | P0     | ✅ 完成   |
| FR-VC-002-2 | 并发控制        | 支持 4+ Agents 同时运行   | P0     | ✅ 完成   |
| FR-VC-002-3 | 功能分支管理    | 自动创建 feature 分支     | P0     | ✅ 完成   |
| FR-VC-002-4 | 代码生成        | 根据 Issue 描述生成代码   | P0     | 📋 待开始 |
| FR-VC-002-5 | 测试生成        | 自动生成单元测试          | P1     | 📋 待开始 |
| FR-VC-002-6 | 触发检查点      | CP-005/CP-006 审查        | P0     | 📋 待开始 |

**并发策略**:

- 最大并发数：4 (可配置)
- 资源监控：CPU < 80%, 内存 < 2GB
- 冲突检测：避免修改同一文件

#### FR-VC-003: MR Creation Agent

**功能描述**: 汇总所有 Coding Agent 的提交，创建合并请求

**详细需求**:

| 需求 ID     | 需求描述           | 验收标准                     | 优先级 | 状态      |
| ----------- | ------------------ | ---------------------------- | ------ | --------- |
| FR-VC-003-1 | 分支合并           | 合并所有 feature 分支到 main | P0     | 📋 待开始 |
| FR-VC-003-2 | 回归测试           | 运行完整的 E2E 测试          | P0     | 📋 待开始 |
| FR-VC-003-3 | MR 描述生成        | 自动生成详细的 MR 描述       | P1     | 📋 待开始 |
| FR-VC-003-4 | 触发 CP-007/CP-008 | 最终审查                     | P0     | 📋 待开始 |

### 3.2 HITL 检查点机制 (P0)

#### FR-VC-004: 8 个关键检查点

| 检查点 ID  | 触发时机           | 审查内容                     | 预计耗时 | 自动接受条件                |
| ---------- | ------------------ | ---------------------------- | -------- | --------------------------- |
| **CP-001** | Initializer 开始前 | 项目目录验证、Git 仓库检查   | < 1min   | 信任模式开启                |
| **CP-002** | 任务分解后         | Issue 列表质量、优先级合理性 | 2-5min   | Issue 数量 < 20, 无 P0 依赖 |
| **CP-003** | Issue 丰富化后     | 上下文信息完整性             | 1-2min   | 上下文得分 > 80             |
| **CP-004** | 编码会话前         | 回归测试结果审查             | 1-3min   | 测试覆盖率 > 90%            |
| **CP-005** | 选择下一个 Issue   | Issue 优先级确认             | < 1min   | 优先级队列稳定              |
| **CP-006** | Issue 完成后       | 实现完整性、测试覆盖率       | 2-5min   | 测试全部通过                |
| **CP-007** | 所有 Issue 完成后  | 是否进入 MR 创建阶段         | 1-2min   | 无未完成 Issue              |
| **CP-008** | MR 创建后          | MR 描述、变更内容审查        | 3-10min  | 变更文件 < 50 个            |

**检查点 UI 组件**:

```tsx
interface CheckpointProps {
  checkpointId: string
  agentId: string
  data: any // 检查点相关数据
  onApprove: () => void
  onReject: (reason: string) => void
  onRequestChanges: (feedback: string) => void
}
```

**自动接受模式**:

- 用户可配置信任阈值
- 信任度 > 90% 的检查点自动接受
- 批量运行时可启用全自动模式

### 3.3 质量门禁系统 (P0)

#### FR-VC-005: 多层次质量保障

| 门禁 ID    | 检查项   | 命令示例                  | 失败处理          | 状态      |
| ---------- | -------- | ------------------------- | ----------------- | --------- |
| **QG-001** | 代码检查 | `eslint .` / `ruff check` | 最多 3 次修复尝试 | ✅ 完成   |
| **QG-002** | 类型检查 | `tsc` / `pyright`         | 最多 3 次修复尝试 | 📋 待开始 |
| **QG-003** | 单元测试 | `npm test` / `pytest`     | 最多 3 次修复尝试 | 📋 待开始 |
| **QG-004** | 回归测试 | E2E 测试验证              | 失败后暂停并通知  | 📋 待开始 |
| **QG-005** | E2E 测试 | Puppeteer 浏览器自动化    | 失败后暂停并通知  | 📋 待开始 |

**质量门禁接口**:

```rust
pub trait QualityGate {
    async fn check(&self, workspace: &Path) -> Result<QGResult>;
    async fn auto_fix(&self, workspace: &Path) -> Result<QGFixResult>;
    fn get_name(&self) -> &str;
    fn is_critical(&self) -> bool;
}

pub enum QGStatus {
    Passed,
    Failed { errors: Vec<String> },
    Fixed { changes: Vec<String> },
    Skipped { reason: String },
}
```

**失败恢复策略**:

1. 第一次失败：自动分析错误并尝试修复
2. 第二次失败：回滚最近提交，重新生成
3. 第三次失败：跳过该 Issue，记录警告
4. 超过限制：暂停并通知用户介入

### 3.4 实时日志与监控 (P0)

#### FR-VC-006: WebSocket 实时推送

**功能描述**: 通过 WebSocket 向界面实时推送 Agent 状态和日志

**详细需求**:

| 需求 ID     | 需求描述   | 验收标准                | 优先级 | 状态      |
| ----------- | ---------- | ----------------------- | ------ | --------- |
| FR-VC-006-1 | 日志推送   | 实时推送 Agent 输出日志 | P0     | ✅ 完成   |
| FR-VC-006-2 | 进度更新   | 推送任务完成百分比      | P0     | ✅ 完成   |
| FR-VC-006-3 | 状态变化   | Agent 状态变化通知      | P0     | ✅ 完成   |
| FR-VC-006-4 | 检查点通知 | 触发检查点时弹窗提醒    | P0     | 📋 待开始 |
| FR-VC-006-5 | 错误告警   | 检测到错误立即通知      | P0     | 📋 待开始 |

**消息类型**:

```typescript
enum WsMessageType {
  LOG = 'log', // Agent 日志输出
  PROGRESS = 'progress', // 进度更新
  STATUS = 'status', // 状态变化
  AGENT_RESPONSE = 'agent_response', // Agent 响应
  CHECKPOINT = 'checkpoint', // 检查点触发
  ERROR = 'error', // 错误通知
}
```

---

## 4. 技术规格

### 4.1 数据结构

#### Agent 核心数据

```typescript
// Agent 类型定义
enum AgentType {
  INITIALIZER = 'initializer',
  CODING = 'coding',
  MR_CREATION = 'mr_creation',
}

// Agent 状态
enum AgentState {
  IDLE = 'idle',
  RUNNING = 'running',
  PAUSED = 'paused',
  COMPLETED = 'completed',
  FAILED = 'failed',
}

// Agent 会话状态
interface AgentSessionState {
  sessionId: string
  agentId: string
  type: AgentType
  state: AgentState
  currentTask?: string
  progress: number // 0-100
  logs: LogEntry[]
  checkpoints: CheckpointRecord[]
  createdAt: number
  updatedAt: number
}

// Issue 数据结构
interface Issue {
  iid: string
  title: string
  description: string
  priority: 'P0' | 'P1' | 'P2'
  status: 'backlog' | 'in_progress' | 'review' | 'done'
  assignee?: string // Agent ID
  dependencies?: string[]
  labels: string[]
  createdAt: number
  closedAt?: number
}
```

#### 检查点数据

```typescript
// 检查点请求
interface CheckpointRequest {
  checkpointId: string
  agentId: string
  sessionId: string
  type: CheckpointType
  data: any
  timestamp: number
}

// 检查点类型
enum CheckpointType {
  PROJECT_VALIDATION = 'cp_001',
  TASK_DECOMPOSITION = 'cp_002',
  CONTEXT_ENRICHMENT = 'cp_003',
  REGRESSION_REVIEW = 'cp_004',
  ISSUE_SELECTION = 'cp_005',
  ISSUE_CLOSURE = 'cp_006',
  MR_DECISION = 'cp_007',
  MR_REVIEW = 'cp_008',
}

// 检查点响应
interface CheckpointResponse {
  requestId: string
  approved: boolean
  modifications?: string[]
  feedback?: string
  reviewedAt: number
}
```

### 4.2 通信协议

#### Stdio 管道通信

```rust
// Stdio 消息格式
pub struct StdioMessage {
    pub message_type: StdioMessageType,
    pub agent_id: String,
    pub session_id: String,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

pub enum StdioMessageType {
    Spawn,      // 启动 Agent
    Command,    // 发送命令
    Output,     // 接收输出
    Error,      // 错误通知
    Exit,       // Agent 退出
}
```

#### WebSocket 实时推送

```rust
// WebSocket 消息格式
pub struct WsMessage {
    pub message_type: WsMessageType,
    pub session_id: String,
    pub agent_id: Option<String>,
    pub data: serde_json::Value,
    pub timestamp: u64,
}
```

### 4.3 数据库设计

#### Agent 会话表

```sql
CREATE TABLE agent_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT UNIQUE NOT NULL,
    agent_id TEXT NOT NULL,
    agent_type TEXT NOT NULL,
    state TEXT NOT NULL,
    current_task TEXT,
    progress INTEGER DEFAULT 0,
    logs TEXT, -- JSON array
    checkpoints TEXT, -- JSON array
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_agent_sessions_state ON agent_sessions(state);
CREATE INDEX idx_agent_sessions_type ON agent_sessions(agent_type);
```

#### Issues 追踪表

```sql
CREATE TABLE issues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iid TEXT UNIQUE NOT NULL,
    session_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    priority TEXT NOT NULL,
    status TEXT NOT NULL,
    assignee TEXT,
    dependencies TEXT, -- JSON array
    labels TEXT, -- JSON array
    created_at INTEGER NOT NULL,
    closed_at INTEGER
);
```

---

## 5. 用户界面

### 5.1 Coding Workspace

```
┌─────────────────────────────────────────────────────────────┐
│ Vibe Coding - AI 自主编码中...              [暂停] [查看日志] │
├──────────────────┬──────────────────────────────────────────┤
│                  │                                          │
│ 📊 任务进度       │  📝 实时日志                             │
│                  │                                          │
│ 总任务：12        │  [Agent: initializer]                   │
│ 已完成：8         │  > 正在读取 PRD 文档...                  │
│ 进行中：2         │  > ✓ 创建 GitLab milestone...           │
│ 待开始：2         │  > ✓ Milestone #45 创建成功             │
│                  │                                          │
│ ──────────────── │  [Agent: coding-agent-1]                │
│                  │  > Issue #1: 用户认证系统                 │
│ 🔍 当前阶段：     │  >   - 创建 src/auth/目录结构            │
│ 编码实现          │  >   - 实现 JWT token 生成逻辑            │
│                  │  >   - 编写单元测试 (15 个测试用例)         │
│ ⏱️  预计剩余：     │  > ✓ 运行质量门禁...                     │
│ 1 小时 30 分钟     │  > ✓ ESLint 检查通过                     │
│                  │  > ✓ TypeScript 类型检查通过             │
│ 🎯 里程碑：      │  > ✓ Jest 测试全部通过 (15/15)         │
│ ▓▓▓▓▓▓▓▓░░ 67%   │  > ✓ 提交代码：feat: add user auth      │
│                  │                                          │
│ ──────────────── │  [HITL] 等待检查点审查：issue_closure    │
│                  │  > Issue #1 已完成，请审查实现结果        │
│ ⚠️  待审查检查点：│                                          │
│ 1 个             │  [审查代码] [批准继续] [要求修改]          │
│                  │                                          │
└──────────────────┴──────────────────────────────────────────┘
│ 快捷键：s=启动/停止 p=暂停 l=查看日志 r=审查检查点 ?=帮助     │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 检查点审查界面

```
┌─────────────────────────────────────────────────────────────┐
│ 🔍 检查点审查：Issue #1 完成审查 (CP-006)                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ 📋 Issue 信息                                               │
│ ├─ 标题：用户认证系统                                        │
│ ├─ 优先级：P0                                               │
│ ├─ 负责人：coding-agent-1                                   │
│ └─ 耗时：23 分钟                                             │
│                                                             │
│ ✅ 完成情况                                                 │
│ ├─ 代码生成：✓ 完成                                         │
│ ├─ 单元测试：✓ 15/15 通过                                   │
│ ├─ 质量门禁：✓ ESLint + TSC + Jest 全部通过                 │
│ └─ Git 提交：✓ feat: add user auth (#1)                     │
│                                                             │
│ 📊 代码统计                                                 │
│ ├─ 新增文件：5 个                                            │
│ ├─ 修改文件：2 个                                            │
│ ├─ 新增代码：423 行                                          │
│ ├─ 删除代码：12 行                                           │
│ └─ 测试覆盖：92.5%                                          │
│                                                             │
│ ⚠️  潜在问题                                                │
│ └─ 无                                                       │
│                                                             │
│ ─────────────────────────────────────────────────────────  │
│                                                             │
│ [👍 批准关闭]  [👎 要求修改]  [📝 查看详情]                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. 数据流

### 6.1 完整编码流程

```
PRD 文档
    ↓
[Initializer Agent]
    ├─ 解析 PRD
    ├─ 环境检查
    ├─ Git 初始化
    └─ 任务分解 → Issues
    ↓
[HITL CP-002] 任务分解审查
    ↓
[Coding Agent Loop]
    ├─ 选择 Issue (CP-005)
    ├─ 上下文丰富化 (CP-003)
    ├─ 代码实现
    ├─ 测试编写
    ├─ 质量门禁 (QG-001~QG-003)
    │   ├─ 失败 → 自动修复 (最多 3 次)
    │   └─ 成功 → 继续
    ├─ Git 提交
    ├─ HITL 审查 (CP-006)
    │   ├─ 批准 → 关闭 Issue
    │   └─ 拒绝 → 重新实现
    └─ 会话交接
    ↓
[MR Creation Agent]
    ├─ 合并所有分支
    ├─ 运行回归测试 (QG-004)
    ├─ 生成 MR 描述
    └─ HITL 审查 (CP-007/CP-008)
    ↓
完整可运行的代码 + 测试 + 部署配置
```

### 6.2 状态流转

```
IDLE → RUNNING → PAUSED → RUNNING → COMPLETED
                  ↓
                FAILED → RETRY → RUNNING
                  ↓
                ABORTED
```

---

## 7. 验收标准

### 7.1 MVP 核心验收

- [ ] **Initializer Agent 跑通**
  - PRD 解析正确
  - 任务分解合理 (10-20 个 Issues)
  - CP-002 检查点可用

- [ ] **单个 Coding Agent 完成 Issue**
  - 代码生成符合预期
  - 测试自动生成并运行
  - 质量门禁全部通过

- [ ] **HITL 检查点可用**
  - 至少实现 CP-001/CP-002/CP-006
  - 界面可审查和批准
  - 支持要求修改

- [ ] **质量门禁系统运行**
  - ESLint 检查通过
  - TypeScript 编译通过
  - Jest 测试运行正常

- [ ] **实时日志和监控**
  - WebSocket 推送正常
  - 日志延迟 < 100ms
  - 状态更新及时

### 7.2 技术指标

- [ ] TypeScript 编译零错误
- [ ] ESLint 零警告
- [ ] Rust `cargo check` 通过
- [ ] 单元测试覆盖率 > 70%
- [ ] 界面响应时间 < 300ms

### 7.3 性能指标

- [ ] 支持 4+ Agents 并发运行
- [ ] CPU 使用率 < 80%
- [ ] 内存使用 < 2GB
- [ ] 日志延迟 < 100ms
- [ ] 检查点响应 < 5s

---

## 8. 风险与挑战

### 8.1 已识别风险

| 风险                  | 概率 | 影响 | 缓解措施                          | 状态  |
| --------------------- | ---- | ---- | --------------------------------- | ----- |
| AI 生成代码质量不稳定 | 中   | 高   | HITL 检查点、质量门禁、多模型备用 | 🟡 中 |
| 多会话并发资源耗尽    | 低   | 中   | 并发限制、资源监控、自动降级      | 🟢 低 |
| 检查点大量被拒绝      | 中   | 高   | 收集反馈、调整提示词、切换模型    | 🟡 中 |
| 上下文预测失败        | 中   | 高   | Git 历史备份、人工检查点兜底      | 🟡 中 |
| 质量门禁失败率高      | 中   | 中   | 设置合理阈值、提供人工介入        | 🟡 中 |

### 8.2 技术挑战

1. **并发控制复杂度高**
   - 需要精确的资源管理和冲突检测
   - 解决方案：实现 AgentManager 统一调度

2. **上下文窗口限制**
   - AI 模型的上下文长度有限
   - 解决方案：分阶段处理、关键信息提取

3. **错误恢复困难**
   - AI 可能执行错误命令
   - 解决方案：Git 回滚机制、工作空间隔离

4. **提示词工程难度大**
   - 需要精心设计的提示词才能获得好的结果
   - 解决方案：持续迭代、A/B 测试、收集最佳实践

---

## 📚 参考资料

- [产品设计文档 §6.2](../references/产品设计.md#62-vibe-coding-核心机制)
- [Symphony产品设计文档](../references/symphony.md)
- [Autonomous Coding Harness](../references/autonomous-coding-harness.md)
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)

---

**维护者**: OPC-HARNESS Technical Team  
**最后更新**: 2026-03-24  
**下一步**: 完成 VC-006 ~ VC-011 (Initializer Agent)

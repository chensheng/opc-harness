# OPC-HARNESS MVP版本规划

> **文档版本**: v2.2  
> **最后更新**: 2026 年 3 月 23 日 00:30  
> **目标阶段**: MVP (2026-04-15)  
> **开发周期**: 6-7 周  
> **架构基础**: 混合架构（云端 AI + 本地自主 Agent）  
> **核心理念**: "人类掌舵，Agent 执行"（Humans steer. Agents execute.）  
> **当前状态**: 🟡 UI 层已完成（48%），Agent 核心功能待开发（重点难点）

### 📊 进度概览

```
总体进度：48% (39/81 任务完成)

✅ 已完成模块:
  - 基础设施：9/14 (64%)
  - Vibe Design: 24/26 (92%) - UI 完整，流式输出已实现，待接入真实 AI API
  - Vibe Marketing: 5/5 (100%) - UI 完整，待接入真实 AI API
  
📋 待开始模块:
  - Vibe Coding: 0/36 (0%) - UI 组件已存在，Agent 核心功能待开发
  - AI 适配器：0/5 (0%) - 待接入真实 AI API
```

---

## 开发任务总览

### 任务分布

```
┌─────────────────────────────────────────────────────────────┐
│  用户故事 1: Vibe Design - 产品构思                           │
│  ├─ 核心功能：想法→PRD→用户画像→竞品分析                     │
│  └─ 任务数：12 个                                            │
├─────────────────────────────────────────────────────────────┤
│  用户故事 2: Vibe Coding - AI 自主编码 (MVP 核心)              │
│  ├─ 核心功能：多会话编排+HITL 检查点 + 质量门禁               │
│  └─ 任务数：28 个                                            │
├─────────────────────────────────────────────────────────────┤
│  用户故事 3: Vibe Marketing - 增长运营 (MVP 简化版)            │
│  ├─ 核心功能：发布策略 + 营销文案生成                         │
│  └─ 任务数：5 个                                             │
├─────────────────────────────────────────────────────────────┤
│  基础设施与通用能力                                          │
│  └─ 任务数：15 个                                            │
├─────────────────────────────────────────────────────────────┤
│  总计：60 个任务       │
└─────────────────────────────────────────────────────────────┘
```

### 任务命名规范

| 前缀 | 含义 | 使用场景 |
|------|------|----------|
| **INFRA** | Infrastructure | 基础设施与通用能力 |
| **VD** | Vibe Design | Vibe Design - 产品构思 |
| **VC** | Vibe Coding | Vibe Coding - AI 自主编码 |
| **VM** | Vibe Marketing | Vibe Marketing - 增长运营 |

### 任务优先级分布

| 优先级 | 任务数量 | 说明               |
| ------ | -------- | ------------------ |
| **P0** | 48       | 核心功能，必须完成 |
| **P1** | 12       | 重要功能，建议完成 |

### 任务状态说明

| 状态   | 说明         |
| ------ | ------------ |
| 📋 待开始 | 尚未开始的任务 |
| 🔄 进行中 | 正在开发中的任务 |
| ✅ 已完成 | 已完成并通过验收的任务 |
| ⏸️ 已暂停 | 暂时搁置的任务 |
| ❌ 已取消 | 取消的任务 |

---

## 基础设施与通用能力 (Week 1-2) - 15 个任务

### 1.1 项目基础架构

| 任务 ID    | 任务名称                            | 优先级 | 估时 | 状态     | 验收标准                          |
| ---------- | ----------------------------------- | ------ | ---- | -------- | --------------------------------- |
| INFRA-001  | 初始化 Tauri v2 + React 项目          | P0     | 4h   | ✅ 已完成 | 项目能正常启动，热更新工作        |
| INFRA-002  | 创建项目初始化和目录结构            | P0     | 4h   | ✅ 已完成 | 能创建标准项目目录结构            |
| INFRA-003  | 实现 Git 仓库初始化功能             | P0     | 3h   | ✅ 已完成 | 能初始化 Git 并创建主分支         |
| INFRA-004  | 配置 ESLint + Prettier              | P0     | 2h   | ✅ 已完成 | 代码规范检查工具配置完整          |
| INFRA-005  | 配置 Zustand 状态管理               | P0     | 2h   | ✅ 已完成 | Store 能正常创建和使用            |

### 1.2 Rust 后端与数据库

| 任务 ID    | 任务名称                       | 优先级 | 估时 | 状态     | 验收标准                       |
| ---------- | ------------------------------ | ------ | ---- | -------- | ------------------------------ |
| INFRA-006  | 配置 Rust 项目结构和依赖         | P0     | 3h   | ✅ 已完成 | Cargo.toml 配置完整，能编译运行 |
| INFRA-007  | 创建基础 Tauri Commands 结构     | P0     | 4h   | ✅ 已完成 | 前后端 IPC 通信正常              |
| INFRA-008  | 集成 SQLite 数据库 (rusqlite)    | P0     | 4h   | ✅ 已完成 | 数据库连接正常，能执行 CRUD，开发环境验证通过 |
| INFRA-009  | 创建项目数据表结构              | P0     | 4h   | ✅ 已完成 | projects/ai_configs/cli_sessions 表 |
| INFRA-010  | 集成 OS 密钥存储 (keyring-rs)    | P0     | 4h   | 📋 待开始 | 能在各平台读写密钥               |

### 1.3 工具检测与环境准备

| 任务 ID    | 任务名称                           | 优先级 | 估时 | 状态     | 验收标准                    |
| ---------- | ---------------------------------- | ------ | ---- | -------- | --------------------------- |
| INFRA-011  | 实现本地工具检测命令               | P0     | 4h   | 📋 待开始 | 能检测 Node.js/Git 等是否安装 |
| INFRA-012  | 实现 Git 环境检测与初始化           | P0     | 4h   | 📋 待开始 | 能检测 Git 并初始化仓库       |

### 1.4 Agent 基础框架

| 任务 ID    | 任务名称                           | 优先级 | 估时 | 状态     | 验收标准                    |
| ---------- | ---------------------------------- | ------ | ---- | -------- | --------------------------- |
| INFRA-013  | 定义 Agent 通信协议 (Stdio/WebSocket) | P0     | 6h   | 📋 待开始 | 前后端通信接口定义完整      |
| INFRA-014  | 实现守护进程基础框架               | P0     | 8h   | 📋 待开始 | 能启动/停止后台进程         |

---

## 用户故事 1: Vibe Design - 产品构思 (12 个任务)

> **作为** 一名独立开发者，  
> **我希望** 通过自然语言描述我的想法，AI 能帮我完善产品构思，  
> **以便** 快速验证想法的可行性。

### 验收标准

- [ ] 能够用几句话描述产品想法
- [ ] AI 能生成产品需求文档 (PRD)
- [ ] AI 能生成用户画像和用户故事
- [ ] AI 能提供竞品分析和差异化建议

### 2.1 AI厂商配置与管理 (VD-001 ~ VD-008)

| 任务 ID  | 任务名称                       | 优先级 | 估时 | 状态     | 验收标准                     |
| -------- | ------------------------------ | ------ | ---- | -------- | ---------------------------- |
| VD-001   | 创建 AI厂商配置数据结构       | P0     | 2h   | ✅ 已完成 | TypeScript 类型定义完整，包含 AIConfig/AIProvider 等 |
| VD-002   | 实现 API密钥安全存储功能      | P0     | 4h   | ✅ 已完成 | Zustand持久化存储，待接入 OS Keychain |
| VD-003   | 创建 AI厂商配置界面           | P0     | 4h   | ✅ 已完成 | 支持 4 家厂商配置，界面美观完整 |
| VD-004   | 实现 API密钥验证功能          | P0     | 4h   | ✅ 已完成 | 前端模拟验证，后端 Rust 待接入真实 API |
| VD-005   | 支持OpenAI API 配置           | P0     | 2h   | ✅ 已完成 | GPT-4o/Claude/Kimi/GLM 模型列表配置 |
| VD-006   | 支持 Anthropic Claude 配置     | P0     | 2h   | ✅ 已完成 | Claude 3.5/3 系列模型配置 |
| VD-007   | 支持 Kimi API 配置             | P0     | 2h   | ✅ 已完成 | Kimi K1.5/K1 模型配置 |
| VD-008   | 支持 GLM API 配置              | P0     | 2h   | ✅ 已完成 | GLM-4/CodeGeeX 模型配置 |

### 2.2 AI 适配服务 (Rust) (VD-009 ~ VD-013)

| 任务 ID  | 任务名称                     | 优先级 | 估时 | 状态     | 验收标准                         |
| -------- | ---------------------------- | ------ | ---- | -------- | -------------------------------- |
| VD-009   | 创建 AI Provider Trait 定义  | P0     | 3h   | ✅ 已完成 | Rust AIProvider 结构已定义完整 |
| VD-010   | 实现 OpenAI 适配器           | P0     | 6h   | 📋 待开始 | 能调用 OpenAI API 并返回结果     |
| VD-011   | 实现 Kimi 适配器             | P0     | 6h   | 📋 待开始 | 能调用 Kimi API 并返回结果       |
| VD-012   | 实现 AI 服务管理器 (统一入口) | P0     | 4h   | 📋 待开始 | 支持切换厂商，故障转移           |
| VD-013   | 实现流式输出 (SSE) 支持      | P0     | 6h   | ✅ 已完成 | AI 响应能实时流式显示，打字机效果流畅 |

### 2.3 核心功能：PRD 与用户画像 (VD-014 ~ VD-020)

| 任务 ID  | 任务名称                    | 优先级 | 估时 | 状态     | 验收标准                     |
| -------- | --------------------------- | ------ | ---- | -------- | ---------------------------- |
| VD-014   | 创建想法输入界面            | P0     | 4h   | ✅ 已完成 | [`IdeaInput`](d:\workspace\opc-harness\src\components\vibe-design\IdeaInput.tsx) 组件完整，支持示例选择 |
| VD-015   | 创建 PRD 生成提示词模板     | P0     | 4h   | ✅ 已完成 | Rust后端Mock数据，待接入真实AI |
| VD-016   | 实现 PRD 生成 API           | P0     | 4h   | ✅ 已完成 | [`generate_prd`](d:\workspace\opc-harness\src-tauri\src\commands\ai.rs#L173) Tauri Command已实现 |
| VD-017   | 创建 PRD 展示组件           | P0     | 6h   | ✅ 已完成 | [`PRDDisplay`](d:\workspace\opc-harness\src\components\vibe-design\PRDDisplay.tsx) 支持Markdown导出 |
| VD-018   | 创建用户画像提示词模板      | P0     | 3h   | ✅ 已完成 | Rust后端Mock数据 |
| VD-019   | 实现用户画像生成 API        | P0     | 3h   | ✅ 已完成 | [`generate_user_personas`](d:\workspace\opc-harness\src-tauri\src\commands\ai.rs#L195) Tauri Command已实现 |
| VD-020   | 创建用户画像展示组件        | P0     | 4h   | ✅ 已完成 | [`UserPersonas`](d:\workspace\opc-harness\src\components\vibe-design\UserPersonas.tsx) 卡片式显示 |

### 2.4 竞品分析与流程整合 (VD-021 ~ VD-026)

| 任务 ID  | 任务名称               | 优先级 | 估时 | 状态     | 验收标准                     |
| -------- | ---------------------- | ------ | ---- | -------- | ---------------------------- |
| VD-021   | 创建竞品分析提示词模板 | P0     | 3h   | ✅ 已完成 | Rust后端Mock数据 |
| VD-022   | 实现竞品分析生成 API   | P0     | 3h   | ✅ 已完成 | [`generate_competitor_analysis`](d:\workspace\opc-harness\src-tauri\src\commands\ai.rs#L215) Tauri Command已实现 |
| VD-023   | 创建竞品分析展示组件   | P0     | 4h   | ✅ 已完成 | [`CompetitorAnalysis`](d:\workspace\opc-harness\src\components\vibe-design\CompetitorAnalysis.tsx) 对比表格展示 |
| VD-024   | 实现 Vibe Design 步骤向导 | P0     | 6h   | ✅ 已完成 | React Router路由完整，流程打通 |
| VD-025   | 实现项目状态管理       | P0     | 3h   | ✅ 已完成 | [`useProjectStore`](d:\workspace\opc-harness\src\stores\projectStore.ts) 支持状态流转 |
| VD-026   | 实现加载与进度显示     | P0     | 2h   | ✅ 已完成 | [`LoadingOverlay`](d:\workspace\opc-harness\src\components\common\LoadingOverlay.tsx) 全局加载状态 |


---

## 用户故事 2: Vibe Coding - AI 自主编码 (28 个任务)

> **作为** 一名设计师，  
> **我希望** 只需描述产品需求，AI 就能自动完成所有编码工作，  
> **以便** 无需任何编程知识就能获得可运行的产品。

### 核心验收标准 (MVP)

- [ ] **多会话编排**: Initializer Agent + 4+ Coding Agents + MR Creation Agent
- [ ] **HITL 检查点**: 8 个关键决策点的人工审查机制
- [ ] **质量门禁**: 代码检查 + 类型检查 + 单元测试
- [ ] **守护进程**: 后台管理 Agent 生命周期
- [ ] **Git 集成**: 自动分支、提交、合并
- [ ] **实时日志**: 编码过程实时查看和监控

### 📊 实现状态概览

| 模块 | 状态 | 说明 |
|------|------|------|
| **UI 界面** | ✅ 已完成 | [`CodingWorkspace`](d:\workspace\opc-harness\src\components\vibe-coding\CodingWorkspace.tsx) 组件完整 |
| **Agent 基础架构** | 📋 待开始 | Stdio/WebSocket通信、Agent Manager 均未实现 |
| **Initializer Agent** | 📋 待开始 | PRD 解析器、环境检查、任务分解算法均未实现 |
| **Coding Agent 集群** | 📋 待开始 | 并发控制、代码生成、测试生成均未实现 |
| **质量门禁系统** | 📋 待开始 | ESLint/TSC/Jest集成均未实现 |
| **HITL 检查点** | 📋 待开始 | 8 个检查点 UI 和逻辑均未实现 |
| **MR Creation Agent** | 📋 待开始 | 分支合并、MR 生成均未实现 |
| **监控与日志** | 📋 待开始 | 实时日志流、进度可视化均未实现 |

### 3.1 Agent 基础架构 (Week 2-3) (VC-001 ~ VC-005)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-001   | 定义 Agent 通信协议和数据结构    | P0     | 6h   | 📋 待开始 | AgentRequest/Response结构定义 |
| VC-002   | 实现 Stdio 管道通信层            | P0     | 6h   | 📋 待开始 | 能与子进程 stdin/stdout 通信 |
| VC-003   | 实现 WebSocket 实时推送层        | P0     | 6h   | 📋 待开始 | 前端能接收 Agent 实时日志     |
| VC-004   | 创建 Agent 管理器 (Manager)      | P0     | 8h   | 📋 待开始 | 能启动/停止/监控 Agent 状态   |
| VC-005   | 实现会话状态持久化              | P0     | 4h   | 📋 待开始 | 崩溃后能从快照恢复           |

**备注**: 当前 [`CodingWorkspace`](d:\workspace\opc-harness\src\components\vibe-coding\CodingWorkspace.tsx) 仅提供 UI 界面，Agent 核心功能待开发。

### 3.2 Initializer Agent (Week 3) (VC-006 ~ VC-011)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-006   | 实现 PRD 文档解析器              | P0     | 4h   | 📋 待开始 | 能提取技术栈和功能列表      |
| VC-007   | 实现环境检查逻辑                | P0     | 4h   | 📋 待开始 | 检测 Node.js/Git/依赖安装   |
| VC-008   | 实现 Git 仓库初始化              | P0     | 3h   | 📋 待开始 | 创建 main/develop 分支       |
| VC-009   | 实现任务分解算法 (PRD→Issues)   | P0     | 8h   | 📋 待开始 | 生成 10-20 个结构化 Issues    |
| VC-010   | 创建 GitLab Issues / JSON 追踪  | P0     | 4h   | 📋 待开始 | Issue 文件包含完整上下文     |
| VC-011   | 触发 CP-002 检查点               | P0     | 2h   | 📋 待开始 | 前端显示任务分解审查界面    |

### 3.3 Coding Agent 集群 (Week 4) (VC-012 ~ VC-017)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-012   | 实现单个 Coding Agent 逻辑       | P0     | 8h   | 📋 待开始 | 能独立完成一个 Issue         |
| VC-013   | 实现并发控制 (4+ Agents 同时运行) | P0     | 6h   | 📋 待开始 | 无资源冲突，稳定并发        |
| VC-014   | 实现功能分支管理                | P0     | 3h   | 📋 待开始 | 自动创建 feature/issue-{id} |
| VC-015   | 实现代码生成功能                | P0     | 8h   | 📋 待开始 | 根据 Issue 生成可运行代码     |
| VC-016   | 实现测试生成 (Jest/Pytest)      | P0     | 6h   | 📋 待开始 | 自动生成单元测试文件        |
| VC-017   | 触发 CP-005/CP-006 检查点        | P0     | 3h   | 📋 待开始 | Issue 选择和完成审查         |

### 3.4 质量门禁系统 (Week 4) (VC-018 ~ VC-022)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-018   | 实现 QG-001 代码检查 (ESLint)    | P0     | 4h   | 📋 待开始 | 运行 eslint 并解析结果       |
| VC-019   | 实现 QG-002 类型检查 (TypeScript) | P0     | 4h   | 📋 待开始 | 运行 tsc --noEmit           |
| VC-020   | 实现 QG-003 单元测试 (Jest)      | P0     | 4h   | 📋 待开始 | 运行 jest 并收集覆盖率       |
| VC-021   | 实现自动修复机制 (最多 3 次)      | P0     | 6h   | 📋 待开始 | 失败后自动尝试修复          |
| VC-022   | 实现 Git 回滚机制                 | P0     | 4h   | 📋 待开始 | 失败后恢复到安全检查点      |

### 3.5 HITL 检查点系统 (Week 4-5) (VC-023 ~ VC-028)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-023   | 实现 Checkpoint Manager         | P0     | 6h   | 📋 待开始 | 管理 8 个检查点状态          |
| VC-024   | 创建 CP-001 项目验证界面         | P0     | 3h   | 📋 待开始 | 显示项目目录和 Git 状态      |
| VC-025   | 创建 CP-002 任务分解审查界面     | P0     | 6h   | 📋 待开始 | 显示 Milestones 和 Issues 树  |
| VC-026   | 创建 CP-006 Issue 完成审查界面   | P0     | 6h   | 📋 待开始 | 显示代码 diff 和测试结果     |
| VC-027   | 创建 CP-008 最终 MR 审查界面      | P0     | 6h   | 📋 待开始 | 显示完整变更和 MR 描述       |
| VC-028   | 实现自动接受逻辑                | P0     | 4h   | 📋 待开始 | 基于信任度阈值自动批准      |

### 3.6 MR Creation Agent (Week 5) (VC-029 ~ VC-032)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-029   | 实现分支合并逻辑                | P0     | 4h   | 📋 待开始 | 合并 feature 到 develop 到 main |
| VC-030   | 实现回归测试运行器              | P0     | 4h   | 📋 待开始 | 运行已完成功能测试          |
| VC-031   | 实现 MR 描述生成器               | P0     | 4h   | 📋 待开始 | 生成包含 Issues/测试/MR 链接 |
| VC-032   | 触发 CP-007/CP-008 检查点        | P0     | 3h   | 📋 待开始 | MR 创建和最终审查           |

### 3.7 监控与日志 (Week 5) (VC-033 ~ VC-036)

| 任务 ID  | 任务名称                        | 优先级 | 估时 | 状态     | 验收标准                    |
| -------- | ------------------------------- | ------ | ---- | -------- | --------------------------- |
| VC-033   | 实现实时日志流                  | P0     | 6h   | 📋 待开始 | 终端式输出，支持滚动        |
| VC-034   | 创建进度可视化组件              | P0     | 6h   | 📋 待开始 | 显示任务进度条和统计        |
| VC-035   | 实现错误检测和通知              | P0     | 4h   | 📋 待开始 | 错误高亮并提示用户介入      |
| VC-036   | 创建 Agent 状态监控面板          | P0     | 6h   | 📋 待开始 | 显示各 Agent 状态和资源使用  |

---

## 用户故事 3: Vibe Marketing - 增长运营 (5 个任务)

> **作为** 一名内容创作者，  
> **我希望** AI 能帮我生成发布策略和推广文案，  
> **以便** 让产品获得更多用户。

### 验收标准 (MVP)

- [ ] AI 能生成产品发布策略大纲
- [ ] AI 能生成社交媒体推广文案
- [ ] 支持导出发布计划

### 4.1 发布策略与营销文案 (VM-001 ~ VM-005)

| 任务 ID  | 任务名称               | 优先级 | 估时 | 状态     | 验收标准                       |
| -------- | ---------------------- | ------ | ---- | -------- | ------------------------------ |
| VM-001   | 创建发布策略提示词模板 | P1     | 3h   | ✅ 已完成 | Rust后端Mock数据，待接入真实AI |
| VM-002   | 实现发布策略生成 API   | P1     | 3h   | ✅ 已完成 | [`generate_marketing_strategy`](d:\workspace\opc-harness\src-tauri\src\commands\ai.rs#L237) Tauri Command已实现 |
| VM-003   | 创建营销文案提示词模板 | P1     | 3h   | ✅ 已完成 | 支持 Twitter/小红书/Reddit 等平台 |
| VM-004   | 实现营销文案生成 API   | P1     | 3h   | ✅ 已完成 | [`generate_marketing_copy`](d:\workspace\opc-harness\src-tauri\src\commands\ai.rs#L257) Tauri Command已实现 |
| VM-005   | 创建营销文案展示组件   | P1     | 4h   | ✅ 已完成 | [`MarketingStrategy`](d:\workspace\opc-harness\src\components\vibe-marketing\MarketingStrategy.tsx) 支持一键复制 |

---

## 任务依赖关系图 (更新)

```
Week 1: 基础设施 ✅ (80% 完成)
├── INFRA-001 ~ INFRA-007 ✅ (已完成)
├── INFRA-008 ~ INFRA-012 📋 (待开始 - 数据库/工具检测)
└── INFRA-013 ~ INFRA-014 📋 (待开始 - Agent 基础框架)
    │
    ▼
Week 2: AI 配置 + Agent 通信 🔄 (AI 配置 UI 已完成)
├── VD-001 ~ VD-008 ✅ (AI 配置 UI 完成)
├── VD-009 ~ VD-013 📋 (AI 适配器待开发)
├── VC-001 ~ VC-005 📋 (Agent 通信协议待开发)
└── VC-006 ~ VC-011 📋 (Initializer Agent 待开发)
    │
    ▼
Week 3: Initializer + HITL 📋
├── VC-006 ~ VC-011 (Initializer 待开发)
├── VC-023 ~ VC-028 (HITL 检查点 UI 待开发)
└── VC-012 ~ VC-017 (Coding Agent 基础待开发)
    │
    ▼
Week 4: Coding Agents + 质量门禁 📋
├── VC-012 ~ VC-017 (Coding Agent 集群待开发)
├── VC-018 ~ VC-022 (质量门禁待开发)
└── VC-033 ~ VC-036 (监控与日志待开发)
    │
    ▼
Week 5: MR Creation + 整合 📋
├── VC-029 ~ VC-032 (MR Creation Agent 待开发)
├── VC-027 (CP-008 最终审查待开发)
└── VM-001 ~ VM-005 ✅ (Marketing UI 已完成)
    │
    ▼
Week 6: 测试与发布
├── 端到端测试
├── 性能优化
└── 文档完善与发布
```

---

## 任务 ID 索引表 (更新)

### 按模块分类

| 模块 | 任务 ID 范围 | 任务数 | 已完成 | 进行中 | 待开始 | 完成率 |
|------|-------------|--------|--------|--------|--------|--------|
| **INFRA** - 基础设施 | INFRA-001 ~ INFRA-014 | 14 | 9 | 0 | 5 | 64% |
| **VD** - Vibe Design | VD-001 ~ VD-026 | 26 | 24 | 1 | 1 | 92% |
| **VC** - Vibe Coding | VC-001 ~ VC-036 | 36 | 0 | 0 | 36 | 0% |
| **VM** - Vibe Marketing | VM-001 ~ VM-005 | 5 | 5 | 0 | 0 | 100% |
| **总计** | | **81** | **38** | **1** | **42** | **47%** |

**注意**: 
- UI 界面已完成的模块（Vibe Design、Vibe Marketing）后端 API 当前使用 Mock 数据
- Vibe Coding 的 UI 组件 [`CodingWorkspace`](d:\workspace\opc-harness\src\components\vibe-coding\CodingWorkspace.tsx) 已存在，但 Agent 核心功能待开发
- AI 适配器（VD-009~VD-013）需要接入真实 AI API

### Vibe Design 任务分组 (更新)

| 分组 | 任务 ID 范围 | 数量 | 状态 | 说明 |
|------|-------------|------|------|------|
| AI厂商配置 | VD-001 ~ VD-008 | 8 | ✅ 已完成 | API密钥管理与配置 UI，待接入 OS Keychain |
| AI适配服务 | VD-009 ~ VD-013 | 5 | 🔄 进行中 | Rust后端适配器 Trait 已定义，待实现具体厂商 |
| 核心功能 | VD-014 ~ VD-026 | 13 | ✅ 已完成 | PRD/画像/竞品生成 UI 完整，API 使用 Mock 数据 |

### Vibe Coding 任务分组 (更新)

| 分组 | 任务 ID 范围 | 数量 | 状态 | 说明 |
|------|-------------|------|------|------|
| Agent基础架构 | VC-001 ~ VC-005 | 5 | 📋 待开始 | 通信协议与管理均未实现 |
| Initializer Agent | VC-006 ~ VC-011 | 6 | 📋 待开始 | 环境初始化与任务分解均未实现 |
| Coding Agent集群 | VC-012 ~ VC-017 | 6 | 📋 待开始 | 并发编码 Agent 均未实现 |
| 质量门禁系统 | VC-018 ~ VC-022 | 5 | 📋 待开始 | 代码检查与回滚机制均未实现 |
| HITL检查点 | VC-023 ~ VC-028 | 6 | 📋 待开始 | 人工审查界面均未实现 |
| MR Creation | VC-029 ~ VC-032 | 4 | 📋 待开始 | 合并请求创建均未实现 |
| 监控与日志 | VC-033 ~ VC-036 | 4 | 📋 待开始 | 实时监控面板均未实现 |
| **UI 工作区** | - | 1 | ✅ 已完成 | [`CodingWorkspace`](d:\workspace\opc-harness\src\components\vibe-coding\CodingWorkspace.tsx) 组件完整，仅展示层 |

---

## 精简说明

### MVP核心聚焦 (更新)

**已完成的功能**:
1. ✅ **基础设施完整**（Tauri v2 + React 18 + TypeScript + Zustand）
2. ✅ **AI 配置 UI**（支持OpenAI/Claude/Kimi/GLM 四家厂商）
3. ✅ **Vibe Design 全流程 UI**（想法→PRD→画像→竞品，待接入真实 AI API）
4. ✅ **Vibe Coding 工作区 UI**（文件树 + 编辑器+CLI 控制台 + 预览，待实现 Agent 核心功能）
5. ✅ **Vibe Marketing UI**（策略 + 文案生成，待接入真实 AI API）
6. ✅ **Harness Engineering 体系**（架构约束、上下文工程、自动化脚本）
7. ✅ **SQLite 数据库集成**（完整的 CRUD 操作服务层）

**待完成的核心功能 (P0)**:
1. 📋 **Agent 基础架构**（Stdio/WebSocket通信、Agent Manager）
2. 📋 **Initializer Agent**（PRD 解析器、环境检查、任务分解算法）
3. 📋 **Coding Agent 集群**（并发控制、代码生成、测试生成）
4. 📋 **质量门禁系统**（ESLint/TSC/Jest集成、自动修复、Git 回滚）
5. 📋 **HITL 检查点**（8 个关键决策点的审查界面和逻辑）
6. 📋 **MR Creation Agent**（分支合并、回归测试、MR 描述生成）
7. 📋 **监控与日志**（实时日志流、进度可视化、错误检测）
8. 📋 **AI 适配器**（OpenAI/Claude/Kimi/GLM真实API调用）
9. 📋 **数据库集成**（SQLite CRUD 操作）
10. 📋 **OS Keychain 集成**（API密钥安全存储）

**可以延后的功能 (P1 或 v1.0)**:
1. ⏸️ 更多 AI厂商支持（v1.0 实现）
2. ⏸️ 高级编辑功能（v1.0 实现）
3. ⏸️ 外部编辑器集成（简化）
4. ⏸️ 复杂的项目管理（简化）
5. ⏸️ E2E 测试和回归测试（v1.0 实现）
6. ⏸️ 本地模型支持（v2.0 实现）

---

## 风险与应对 (更新)

| 风险              | 概率 | 影响 | 应对措施                       | 状态 |
| ----------------- | ---- | ---- | ------------------------------ | ---- |
| Agent 编排复杂度高 | 高   | 高   | 分阶段实现，先跑通单 Agent 流程  | 🟡 高 |
| HITL 检查点设计困难 | 中   | 高   | 参考 Symphony/Autonomous Harness | 🟡 中 |
| 质量门禁失败率高 | 中   | 中   | 设置合理阈值，提供人工介入    | 🟡 中 |
| AI API 接入延迟   | 中   | 高   | 优先完成 UI 和 Mock 数据，降低阻塞风险 | 🟢 低 |
| 开发周期延长     | 高   | 高   | 已调整预期为 6-7 周，聚焦核心功能 | 🟡 中 |
| AI 响应不可控     | 中   | 高   | Git 回滚机制 + 检查点兜底       | 🟡 中 |
| 并发资源耗尽     | 低   | 中   | 限制并发数，动态资源监控      | 🟢 低 |

**风险趋势**: 🟢 整体可控，Agent 核心功能是主要风险点

---

## 下一步行动 (更新)

### 立即开始（Week 1-2）
1. ✅ **完成数据库集成**（INFRA-008~INFRA-010）
   - [ ] 集成 SQLite 数据库 (rusqlite)
   - [ ] 创建项目数据表结构
   - [ ] 集成 OS 密钥存储 (keyring-rs)

2. ✅ **完成工具检测**（INFRA-011~INFRA-012）
   - [ ] 实现本地工具检测命令
   - [ ] 实现 Git 环境检测与初始化

3. 🔄 **启动 AI 适配器开发**（VD-009~VD-013）
   - [ ] 完善 Rust AI Provider Trait
   - [ ] 实现 OpenAI 适配器（优先）
   - [ ] 接入真实 AI API 测试

### 优先级最高（Week 2-3）
4. 🔴 **Agent 基础架构**（VC-001~VC-005）
   - [ ] 定义 Agent 通信协议 (Stdio/WebSocket)
   - [ ] 实现单个 Agent 的 Stdio 管道通信
   - [ ] 创建简单的 Agent Manager

5. 🔴 **Initializer Agent 原型**（VC-006~VC-011）
   - [ ] 实现 PRD 文档解析器（Mock 数据）
   - [ ] 实现任务分解算法（Mock Issues）
   - [ ] 触发 CP-002 检查点 UI

### 并行开发
- **Vibe Design**: 接入真实 AI API，替换 Mock 数据
- **HITL 检查点**: 设计 8 个检查点的 UI 框架
- **质量门禁**: 调研 ESLint/TSC/Jest的Rust集成方案

### 阶段性验证
- Week 2 末：✅ AI 配置能调用真实API
- Week 3 末：🎯 单个 Agent 能独立运行并完成简单任务
- Week 4 末：🎯 Initializer 能分解 PRD 为 Issues
- Week 5 末：🎯 Coding Agent 能完成 1-2 个简单 Issue
- Week 6 末：🎯 完整流程跑通（含 MR Creation）

### 最后整合（Week 7）
- 端到端测试
- 性能优化
- 文档完善与发布

**预计完成时间**: 2026-04-15  
**当前状态**: 🟡 基础设施和 UI 已完成（48%），Agent 核心功能待开发（重点难点）  
**关键路径**: Agent 基础架构 → Initializer Agent → Coding Agent → 质量门禁 → HITL → MR Creation

---

## 附录：关键术语表

| 术语 | 英文 | 定义 |
|------|------|------|
| **Initializer Agent** | Initializer Agent | 负责环境初始化、任务分解的 AI代理 |
| **Coding Agent** | Coding Agent | 负责具体 Issue 实现的 AI代理 |
| **MR Creation Agent** | MR Creation Agent | 负责汇总提交并创建合并请求的 AI代理 |
| **HITL** | Human-in-the-Loop | 人在回路，关键决策点的人工干预机制 |
| **多会话编排** | Multi-Session Orchestration | 连接多个 AI 会话实现长时间复杂任务的系统 |
| **质量门禁** | Quality Gates | 代码质量检查机制 (Lint/Type Check/Test) |
| **守护进程** | Daemon | 后台运行的进程管理 AI代理生命周期 |
| **检查点** | Checkpoint | 关键决策点的人工审查环节 |

# OPC-HARNESS 架构设计

> **文档版本**: v3.0  
> **最后更新**: 2026 年 3 月 22 日  
> **架构类型**: 混合架构 (云端AI + 本地自主 Agent)  
> **运行环境**: 用户本地计算机 + 云端AI 服务

---

## 1. 架构概述

### 1.1 架构定位

OPC-HARNESS 采用**混合架构 (Hybrid Architecture)** 设计，作为一款本地桌面应用运行在用户计算机上。核心创新在于**Vibe Coding 自主编码系统**——通过多会话编排 (Multi-Session Orchestration) 和人在回路 (Human-in-the-Loop) 机制，实现从 PRD 到可部署代码的全流程自动化。

### 1.2 核心原则

| 原则 | 说明 | 实现策略 |
|------|------|---------|
| **自主编码** | AI Agent 自主完成编码任务，人类仅关键决策点审查 | 多会话编排 + HITL 检查点 |
| **多云 AI 支持** | 支持多家 AI厂商，Agent自主选择最优模型 | 统一 AI 适配层 + 多厂商 API 对接 |
| **本地优先存储** | 项目数据、代码、日志存储在本地文件系统 | SQLite + 本地文件存储 + Git 版本控制 |
| **质量门禁** | 多层次质量保障确保代码符合生产标准 | 代码检查 + 类型检查 + 单元测试 |
| **密钥安全** | API密钥本地加密存储 | OS Keychain/Keystore |
| **透明可控** | AI 执行过程实时可见，关键决策需人工批准 | 实时日志 + 检查点审查机制 |

### 1.3 技术栈概览

```
前端：React + TypeScript + Tailwind CSS + Zustand + Monaco Editor
后端：Rust + Tauri v2 + tokio (异步运行时)
数据库：SQLite (元数据) + Git (代码版本控制)
AI 服务：OpenAI / Anthropic / Kimi / GLM / MiniMax
Agent 框架：自主编码 Agent 集群 (Initializer + Coding + MR Creation)
质量工具：Ruff/Pyright/eslint/tsc/pytest (可插拔)
```

### 1.4 架构演进路线

```
┌─────────────────────────────────────────────────────────────┐
│  MVP 阶段 (v1.0) - 当前版本                                 │
│  ✓ 支持 5+ 主流 AI厂商 API                                   │
│  ✓ AI 配置管理界面完成                                       │
│  ✓ Vibe Design 模块完成 (PRD/用户画像/竞品分析)              │
│  ⚠ Vibe Coding 核心架构设计完成                            │
│    ├─ 多会话编排架构 (Initializer + Coding Agents + MR)     │
│    ├─ HITL 检查点机制 (8 个关键审查点)                       │
│    ├─ 质量门禁系统 (代码检查/类型检查/单元测试)             │
│    ├─ 守护进程架构 (代理生命周期管理)                       │
│    └─ Git 集成 (自动分支/提交/MR)                           │
│  ✓ 本地 SQLite 数据存储                                     │
│  ✓ 基础工具链检测 (Node.js/Git)                             │
├─────────────────────────────────────────────────────────────┤
│  进阶阶段 (v2.0) - 规划中                                   │
│  ├─ 可插拔代码质量系统 (支持多语言工具链)                   │
│  ├─ 上下文丰富化 (Context7、SearXNG 集成)                    │
│  ├─ 会话交接机制优化                                        │
│  ├─ 错误恢复和重试策略改进                                  │
│  ├─ 支持本地模型 (Ollama) 作为备选                          │
│  ├─ 多设备同步 (可选)                                        │
│  ├─ 插件系统扩展                                             │
│  └─ 团队协作 (局域网 P2P)                                     │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                     🖥️ 桌面应用层 (Frontend)                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  React + TypeScript + Tailwind CSS                      │   │
│  │  ├─ UI 界面与用户交互                                     │   │
│  │  ├─ 状态管理 (Zustand)                                   │   │
│  │  ├─ 路由导航 (React Router)                              │   │
│  │  ├─ 代码编辑器 (Monaco Editor)                           │   │
│  │  └─ Vibe Coding 监控界面                                 │   │
│  └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                     🔧 主进程层 (Tauri v2 Main Process)         │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │ 文件系统插件  │ │ 对话框插件   │ │ Shell 插件    │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │ AI 服务模块    │ │ Agent 编排器   │ │ 数据存储服务  │          │
│  │              │ │ (守护进程)    │ │              │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │ HITL 检查点   │ │ 质量门禁     │ │ Git 管理器    │          │
│  │ 管理器        │ │ 系统         │ │              │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
├─────────────────────────────────────────────────────────────────┤
│                     🤖 云端AI 服务层 (Cloud AI)                   │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐         │
│  │  OpenAI  │ │ Anthropic│ │  Kimi    │ │  GLM     │         │
│  │  GPT-4o  │ │  Claude  │ │  K2.5    │ │  GLM-4   │         │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘         │
├─────────────────────────────────────────────────────────────────┤
│                     🛠️ 本地工具链层                              │
│  ┌──────────┐ ┌──────────┐ ┌─────────────────────────┐       │
│  │ Node.js  │ │   Git    │ │  质量工具 (Ruff/Pytest) │       │
│  └──────────┘ └──────────┘ └─────────────────────────┘       │
├─────────────────────────────────────────────────────────────────┤
│                     💾 本地数据存储层                            │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │  SQLite DB   │ │  项目文件    │ │  执行日志    │          │
│  │  (元数据)    │ │  (代码/资源) │ │  (Agent Logs)│          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
│  ┌──────────────────────────────────────────────────┐         │
│  │  Git 仓库 (版本控制 + 回滚保障)                    │         │
│  └──────────────────────────────────────────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 AI 服务架构

```
┌─────────────────────────────────────────────────────────────┐
│  AI 服务统一适配层 (AI Adapter Layer)                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  前端接口层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  • AIConfig (配置接口)                                  ││
│  │  • Message (消息接口)                                   ││
│  │  • AIProvider (厂商标识)                                ││
│  │  • ModelSelection (模型选择策略)                        ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  后端服务层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  • AIProviderType (厂商枚举)                            ││
│  │  • ChatRequest/Response (请求响应)                       ││
│  │  • AIProvider (服务实现)                                ││
│  │  • AgentChatService (Agent 专用通信)                     ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  命令接口层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  • validate_ai_key (验证密钥)                           ││
│  │  • chat (对话)                                          ││
│  │  • generate_prd (生成 PRD)                               ││
│  │  • generate_user_personas (生成用户画像)                ││
│  │  • generate_competitor_analysis (生成竞品分析)          ││
│  │  • spawn_agent (启动 AI 代理)                             ││
│  │  • agent_execute (执行编码任务)                         ││
│  │  • run_quality_gates (运行质量门禁)                     ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  已实现特性:                                                 │
│  ✓ 统一接口，支持 5 家 AI厂商                                 │
│  ✓ API Key 验证功能                                          │
│  ✓ 流式输出支持                                              │
│  ✓ 结构化数据生成 (PRD/用户画像/竞品分析)                    │
│  ⚠ Agent 自主编码支持 (开发中)                               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 Vibe Coding 核心架构

```
┌─────────────────────────────────────────────────────────────┐
│  Vibe Coding - AI 自主编码系统                                │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  多会话编排层:                                                │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Initializer Agent (初始化代理)                         ││
│  │  ├─ 读取 PRD 文档，理解项目需求                          ││
│  │  ├─ 环境检查和初始化 (Node.js/Git/依赖)                 ││
│  │  ├─ 任务分解：PRD → Milestones → Issues                ││
│  │  └─ 创建 GitLab Issues / 本地 JSON 追踪                  ││
│  └─────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Coding Agents (编码代理集群，4+ 并发)                    ││
│  │  ├─ Agent #1: Issue #1 - 用户认证系统                   ││
│  │  ├─ Agent #2: Issue #2 - 登录页面 UI                     ││
│  │  ├─ Agent #3: Issue #3 - 数据库设计                      ││
│  │  └─ ... (并行执行，独立 Git 分支)                        ││
│  └─────────────────────────────────────────────────────────┘│
│  ┌─────────────────────────────────────────────────────────┐│
│  │  MR Creation Agent (合并请求创建代理)                   ││
│  │  ├─ 汇总所有 Coding Agent 的提交                          ││
│  │  ├─ 生成 MR 描述和变更说明                               ││
│  │  └─ 创建 Merge Request                                  ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  HITL 检查点层 (8 个关键决策点):                               │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  CP-001: Initializer 开始前 - 项目验证                   ││
│  │  CP-002: 任务分解后 - Issue 质量审查                     ││
│  │  CP-003: Issue 丰富化后 - 上下文完整性检查              ││
│  │  CP-004: 编码会话前 - 回归测试结果审查                  ││
│  │  CP-005: 选择下一个 Issue - 优先级确认                  ││
│  │  CP-006: Issue 完成后 - 实现完整性审查                  ││
│  │  CP-007: 所有 Issue 完成后 - MR 创建审查                  ││
│  │  CP-008: MR 创建后 - 最终变更审查                       ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  质量门禁层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  QG-001: 代码检查 (Ruff / ESLint)                       ││
│  │  QG-002: 类型检查 (Pyright / TypeScript)                ││
│  │  QG-003: 单元测试 (Pytest / Jest)                       ││
│  │  QG-004: 回归测试 (已完成功能验证)                      ││
│  │  QG-005: E2E 测试 (Puppeteer 浏览器自动化)               ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  守护进程层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Daemon Process (后台管理进程)                          ││
│  │  ├─ Agent 生命周期管理 (启动/停止/恢复)                  ││
│  │  ├─ 任务调度和并发控制                                  ││
│  │  ├─ 日志收集和持久化                                    ││
│  │  ├─ 状态快照和断点续传                                  ││
│  │  └─ 错误检测和自动恢复                                  ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  Git 集成层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Git Manager (版本控制)                                 ││
│  │  ├─ 自动创建功能分支 (feature/issue-#1)                ││
│  │  ├─ 自动提交代码 (feat: add user auth)                 ││
│  │  ├─ Git 回滚机制 (失败时恢复到安全点)                    ││
│  │  └─ MR/PR 自动生成                                      ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 CLI工具集成架构

```
┌─────────────────────────────────────────────────────────────┐
│  CLI工具集成层 (CLI Tool Integration)                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  当前状态：框架已完成，具体工具待接入                        │
│                                                              │
│  会话管理层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  • CLISession (会话结构)                                ││
│  │  • SessionManager (会话管理器)                          ││
│  │  • 多会话并发支持                                       ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  命令接口层:                                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  • detect_tools (工具检测)                              ││
│  │  • create_cli_session (创建会话)                        ││
│  │  • send_cli_prompt (发送指令)                           ││
│  │  • read_cli_output (读取输出)                           ││
│  │  • stop_cli_session (停止会话)                          ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  规划支持的工具:                                             │
│  • Kimi CLI - 月之暗面官方工具                               │
│  • Claude Code - Anthropic官方工具                          │
│  • Codex CLI - OpenAI官方工具                               │
│  • OpenCode - 开源工具                                       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 技术架构

### 3.1 桌面应用框架

**已选择：Tauri v2** 

选择理由:
- ✅ 包体积小 (相比 Electron 小 10-20 倍)
- ✅ Rust 后端，性能优异
- ✅ 安全性好 (内存安全、系统权限控制)
- ✅ 跨平台支持 (Windows/macOS/Linux)
- ✅ 适合频繁调用本地 CLI工具的场景

### 3.2 完整技术栈

```
┌─────────────────────────────────────────────────────────────┐
│  前端层 (Frontend)                                          │
│  ├─ 框架：React + TypeScript                                │
│  ├─ 样式：Tailwind CSS + shadcn/ui (Radix UI)              │
│  ├─ 状态管理：Zustand + Immer                              │
│  ├─ 路由：React Router                                      │
│  ├─ 编辑器：Monaco Editor                                   │
│  ├─ Markdown 渲染：react-markdown                           │
│  └─ Vibe Coding 监控：实时日志终端 + 进度可视化             │
├─────────────────────────────────────────────────────────────┤
│  应用层 (Tauri v2)                                          │
│  ├─ 后端：Rust                                              │
│  ├─ IPC:Tauri Commands + Events                            │
│  ├─ HTTP 客户端：reqwest                                     │
│  ├─ 进程管理：tokio (异步运行时 + 多任务调度)               │
│  ├─ 数据库：rusqlite (SQLite)                               │
│  ├─ 密钥存储：keyring (OS Keychain)                         │
│  ├─ Git 操作：git2 (libgit2 bindings)                       │
│  ├─ 文件监控：notify (fs watch)                             │
│  └─ 工具库：uuid, chrono, futures, serde                   │
├─────────────────────────────────────────────────────────────┤
│  Agent 编排层                                                │
│  ├─ Initializer Agent: 环境初始化、任务分解                 │
│  ├─ Coding Agent Pool: 多并发编码代理集群                   │
│  ├─ MR Creation Agent: 合并请求自动生成                     │
│  ├─ HITL Checkpoint Manager: 检查点管理和人工审查           │
│  ├─ Quality Gate Runner: 质量门禁执行器                     │
│  └─ Daemon Process: 后台守护进程                            │
├─────────────────────────────────────────────────────────────┤
│  云端AI 服务对接                                            │
│  ├─ 协议：REST API + SSE (Server-Sent Events)              │
│  ├─ 认证：API Key / Bearer Token                           │
│  ├─ 格式：OpenAI API兼容格式                               │
│  ├─ 流式：text/event-stream                                │
│  └─ Agent Prompt: 自主编码专用提示词模板                    │
├─────────────────────────────────────────────────────────────┤
│  本地工具链                                                 │
│  ├─ 运行时：Node.js                                         │
│  ├─ 版本控制：Git                                           │
│  ├─ 代码检查：Ruff (Python) / ESLint (JS/TS)               │
│  ├─ 类型检查：Pyright (Python) / TypeScript                │
│  ├─ 单元测试：Pytest (Python) / Jest (JS/TS)               │
│  └─ E2E 测试：Puppeteer (浏览器自动化)                      │
├─────────────────────────────────────────────────────────────┤
│  数据存储                                                   │
│  ├─ 元数据：SQLite (opc-harness.db)                        │
│  ├─ 项目文件：本地文件系统 + Git 仓库                        │
│  ├─ 执行日志：~/.opc-harness/logs/*.log                    │
│  └─ 密钥：OS Keychain / Windows Credential                 │
└─────────────────────────────────────────────────────────────┘
```

### 3.3 依赖版本

#### 前端核心依赖
- React 18
- TypeScript 5
- Tailwind CSS 3
- Zustand 4
- React Router 6
- Monaco Editor
- Radix UI

#### Rust 后端依赖
- Tauri 2.0
- tokio (异步运行时)
- reqwest (HTTP 客户端)
- rusqlite (SQLite 驱动)
- keyring (密钥管理)
- serde/serde_json (序列化)

---

## 4. 核心模块设计

### 4.1 AI 适配服务

#### 4.1.1 接口设计

**前端接口:**
- AIConfig: AI厂商配置
- Message: 对话消息
- AIProvider: 厂商标识
- ChatRequest/Response: 请求响应结构
- ModelSelection: 模型选择策略 (根据任务类型自动选择)

**后端服务:**
- AIProviderType: 厂商类型枚举 (OpenAI/Anthropic/Kimi/GLM/MiniMax)
- AIProvider: AI 服务实现类
- ChatRequest/Response: 数据传输对象
- AgentChatService: Agent 专用通信服务 (支持长会话、上下文管理)

**Tauri 命令:**
- validate_ai_key: 验证 API Key 有效性
- chat: 发送对话请求
- generate_prd: 生成产品需求文档
- generate_user_personas: 生成用户画像
- generate_competitor_analysis: 生成竞品分析
- generate_marketing_strategy: 生成营销策略
- spawn_agent: 启动 AI编码代理
- agent_execute: 执行编码任务
- run_quality_gates: 运行质量门禁
- trigger_checkpoint: 触发 HITL 检查点

#### 4.1.2 厂商适配器模式

采用工厂模式实现多厂商适配:

```
AIProvider (抽象基类)
    │
    ├── OpenAIAdapter (GPT-4o, GPT-4o-mini, o1)
    ├── AnthropicAdapter (Claude 3.5, Claude 3)
    ├── KimiAdapter (Kimi K1.5, Kimi K1)
    ├── GLMAdapter (GLM-4, CodeGeeX)
    └── MiniMaxAdapter (abab6.5, abab6)
```

每个适配器实现:
- BaseURL 配置
- 认证头格式
- 请求体结构
- 响应解析逻辑
- Token 计数和成本计算
- 流式输出处理

#### 4.1.3 智能模型选择

根据任务类型自动推荐最优模型:

| 任务类型 | 推荐模型 | 备选模型 | 经济方案 |
|---------|---------|---------|---------|
| PRD 生成 | GPT-4o | Claude 3.5 | GPT-4o-mini |
| 代码生成 | Claude 3.5 | GPT-4o | GPT-4o-mini |
| 代码审查 | Claude 3.5 | GPT-4o | - |
| 测试生成 | GPT-4o | Claude 3.5 | GPT-4o-mini |
| 营销文案 | Claude 3.5 | GPT-4o | GLM-4-Air |
| 简单问答 | GPT-4o-mini | GLM-4-Air | GPT-4o-mini |

#### 4.1.4 通信流程

```
前端 → Tauri Command → AIProvider → HTTP Request → Cloud AI
                                                    ↓
前端 ← IPC Response ← AIProvider ← HTTP Response ← Cloud AI
```

### 4.2 Vibe Coding Agent 系统

#### 4.2.1 Initializer Agent (初始化代理)

**职责**: 环境初始化、任务分解、创建追踪工单

**工作流程**:
1. 读取 PRD 文档，理解项目需求和技术栈
2. 检查开发环境 (Node.js/Git/依赖安装)
3. 初始化 Git 仓库，创建初始提交
4. 将 PRD 分解为 Milestones (里程碑)
5. 将每个 Milestone 分解为具体的 Issues
6. 为每个 Issue 添加详细描述和验收标准
7. 创建 GitLab Issues 或本地 JSON 追踪文件
8. 触发 HITL 检查点 CP-002 (任务分解审查)

**输出**:
- Git 仓库初始化完成
- Milestone 列表 (3-5 个)
- Issue 列表 (10-20 个)
- 项目目录结构

**通信协议**:
```rust
struct InitializerRequest {
    project_id: String,
    prd: PRDDocument,
    tech_stack: Vec<String>,
}

struct InitializerResponse {
    milestones: Vec<Milestone>,
    issues: Vec<Issue>,
    git_branch: String,
    checkpoint_id: Option<String>, // CP-002
}
```

#### 4.2.2 Coding Agent (编码代理)

**职责**: 自主实现单个 Issue，包括代码编写、测试生成、质量验证

**工作流程**:
1. 从 Issue 池中选择一个 Issue (按优先级)
2. 触发 HITL 检查点 CP-005 (Issue 选择确认)
3. 创建功能分支：`feature/issue-{id}`
4. 分析 Issue 需求和上下文
5. 实现代码功能
6. 生成单元测试
7. 运行质量门禁 (QG-001 ~ QG-003)
8. 如果失败，自动修复 (最多 3 次)
9. Git提交：`feat: add {feature_name}`
10. 触发 HITL 检查点 CP-006 (Issue 完成审查)
11. 合并到主分支或等待 MR

**并发控制**:
- 支持 4+ 个 Coding Agent 同时运行
- 每个 Agent 独立的功能分支
- 避免文件冲突的协调机制
- 资源使用监控 (CPU/内存)

**通信协议**:
```rust
struct CodingAgentRequest {
    issue: Issue,
    context: ProjectContext,
    branch_name: String,
}

struct CodingAgentResponse {
    commits: Vec<GitCommit>,
    test_results: Vec<TestResult>,
    quality_gates: QualityGateStatus,
    checkpoint_id: Option<String>, // CP-006
}
```

#### 4.2.3 MR Creation Agent (合并请求创建代理)

**职责**: 汇总所有提交，生成合并请求

**工作流程**:
1. 确认所有 Issues 已完成并通过审查
2. 触发 HITL 检查点 CP-007 (MR 创建审查)
3. 将所有功能分支合并到 `develop` 分支
4. 运行回归测试 (QG-004)
5. 生成 MR 描述，包含：
   - 变更概述
   - 实现的 Issues 列表
   - 测试覆盖率报告
   - 质量门禁状态
6. 创建 Merge Request 到 `main` 分支
7. 触发 HITL 检查点 CP-008 (最终 MR 审查)

**通信协议**:
```rust
struct MRCreationRequest {
    project_id: String,
    completed_issues: Vec<Issue>,
    develop_branch: String,
    main_branch: String,
}

struct MRCreationResponse {
    mr_id: String,
    mr_url: String,
    description: String,
    checkpoint_id: Option<String>, // CP-008
}
```

### 4.3 HITL 检查点机制

#### 4.3.1 检查点设计原则

**人在回路 (Human-in-the-Loop)**: 在关键决策点设置人工审查环节，确保代码质量和方向正确。

**自动接受模式**: 
- 用户可配置信任度阈值
- 高信任度检查点自动通过
- 支持批量运行和成熟项目
- 用户可随时切换手动审查模式

#### 4.3.2 检查点列表

| 检查点 ID | 名称 | 触发时机 | 审查内容 | 预计耗时 | 自动接受条件 |
|----------|------|---------|---------|---------|-------------|
| **CP-001** | 项目验证 | Initializer 开始前 | 项目目录验证、Git 仓库检查 | < 1min | 总是自动 |
| **CP-002** | 任务分解审查 | Initializer 完成后 | Issue 列表质量、优先级合理性 | 2-5min | 信任度 > 80% |
| **CP-003** | 上下文丰富化 | Issue 丰富化后 | 上下文信息完整性 | 1-2min | 信任度 > 90% |
| **CP-004** | 回归测试审查 | 编码会话前 | 已完成功能测试结果 | 1-3min | 信任度 > 85% |
| **CP-005** | Issue 选择确认 | 选择下一个 Issue | Issue 优先级确认 | < 1min | 总是自动 |
| **CP-006** | Issue 完成审查 | Issue 完成后 | 实现完整性、测试覆盖率 | 2-5min | 信任度 > 75% |
| **CP-007** | MR 创建审查 | 所有 Issue 完成后 | 是否进入 MR 创建阶段 | 1-2min | 信任度 > 80% |
| **CP-008** | 最终 MR 审查 | MR 创建后 | MR 描述、变更内容审查 | 3-10min | 从不自动 |

#### 4.3.3 检查点数据结构

``rust
struct Checkpoint {
    id: String,              // CP-001, CP-002, ...
    name: String,            // "任务分解审查"
    description: String,     // 详细描述
    triggered_at: DateTime,  // 触发时间
    agent_id: String,        // 触发的 Agent
    status: CheckpointStatus, // pending/reviewed/approved/rejected
    
    // 审查内容
    review_items: Vec<ReviewItem>,
    
    // 用户操作
    user_action: Option<UserAction>, // approve/reject/modify
    feedback: Option<String>,        // 拒绝原因或修改建议
    
    // 自动接受配置
    auto_accept_enabled: bool,
    trust_threshold: f32,            // 0.0 - 1.0
}

enum CheckpointStatus {
    Pending,      // 等待审查
    Reviewed,     // 已审查但未决定
    Approved,     // 已批准继续
    Rejected,     // 已拒绝需修改
}

struct ReviewItem {
    item_type: String,      // "issue_list", "code_diff", "test_report"
    title: String,          // 标题
    content: String,        // Markdown 或 JSON
    severity: Severity,     // High/Medium/Low
}
```

#### 4.3.4 检查点 UI 交互

前端提供专门的检查点审查界面:

```
┌─────────────────────────────────────────────────────────────┐
│ 🔍 检查点审查：CP-002 - 任务分解审查                         │
├─────────────────────────────────────────────────────────────┤
│ 触发 Agent: Initializer                                     │
│ 触发时间：2026-03-22 14:30:00                               │
├─────────────────────────────────────────────────────────────┤
│ 审查内容:                                                   │
│                                                             │
│  ✓ Milestone #1: 用户认证系统                               │
│    ├─ Issue #1: JWT Token 生成与验证                        │
│    ├─ Issue #2: 用户注册页面                                │
│    └─ Issue #3: 登录页面                                   │
│                                                             │
│  ✓ Milestone #2: 项目管理                                  │
│    ├─ Issue #4: 项目看板组件                                │
│    └─ Issue #5: 任务拖拽功能                                │
│                                                             │
│  ⚠ 警告：Issue #6 的描述不够详细                            │
├─────────────────────────────────────────────────────────────┤
│ [✅ 批准继续]  [❌ 要求修改]  [💬 添加备注]                   │
└─────────────────────────────────────────────────────────────┘
```

### 4.4 质量门禁系统

#### 4.4.1 多层次质量保障

质量门禁系统在每次代码提交前自动执行，确保生成的代码符合生产标准。

```
┌─────────────────────────────────────────────────────────────┐
│  质量门禁流程 (Quality Gate Pipeline)                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  代码提交后 → QG-001 → QG-002 → QG-003 → QG-004 → QG-005   │
│       ↓              ↓         ↓         ↓         ↓        │
│     通过？通过？通过？通过？通过？通过？通过？通过？通过？通过？           │
│       ↓              ↓         ↓         ↓         ↓        │
│    [是] → 进入下一关                                         │
│    [否] → 自动修复 (最多 3 次) → 仍失败则回滚并通知             │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

#### 4.4.2 门禁详细配置

| 门禁 ID | 名称 | 检查项 | 命令示例 | 失败处理 | 预计耗时 |
|--------|------|-------|---------|---------|---------|
| **QG-001** | 代码检查 | Lint/Style | `ruff check .` / `eslint .` | 最多 3 次修复尝试 | 10-30s |
| **QG-002** | 类型检查 | Type Safety | `pyright` / `tsc --noEmit` | 最多 3 次修复尝试 | 20-60s |
| **QG-003** | 单元测试 | Unit Tests | `pytest` / `npm test` | 最多 3 次修复尝试 | 30-120s |
| **QG-004** | 回归测试 | Existing Features | `pytest tests/regression/` | 失败后暂停并通知 | 1-5min |
| **QG-005** | E2E 测试 | Browser Automation | `puppeteer e2e/*.test.js` | 失败后暂停并通知 | 2-10min |

#### 4.4.3 可插拔设计

质量工具支持多语言和多框架:

```rust
trait QualityGate {
    fn name(&self) -> &str;
    fn check(&self, project_path: &Path) -> Result<GateResult, Error>;
    fn auto_fix(&self) -> Option<Box<dyn QualityGate>>;
}

// Python 技术栈
struct PythonLintGate;      // Ruff
struct PythonTypeGate;      // Pyright
struct PythonTestGate;      // Pytest

// JavaScript/TypeScript技术栈
struct JSLintGate;          // ESLint
struct TypeScriptGate;      // tsc
struct JestTestGate;        // Jest

// Rust 技术栈
struct CargoClippyGate;     // Clippy
struct CargoTestGate;       // cargo test
```

#### 4.4.4 失败恢复策略

```
质量门禁失败
    ↓
分析失败原因
    ↓
[可自动修复？]
    ├─ 是 → 尝试自动修复 (最多 3 次)
    │        ↓
    │      重新运行门禁
    │        ↓
    │      [成功？] → 继续流程
    │        ↓
    │      [失败] → Git 回滚 + 通知用户
    │
    └─ 否 → Git 回滚到最近安全点
             ↓
           触发 HITL 检查点 (CP-006)
             ↓
           通知用户介入
```

### 4.5 守护进程架构

#### 4.5.1 设计目标

后台守护进程管理 AI代理生命周期，确保稳定性和可靠性：

- ✅ TUI/界面崩溃不影响运行中的代理
- ✅ 支持断点续传和状态恢复
- ✅ 日志文件持久化存储
- ✅ 支持远程监控和管理
- ✅ 资源使用监控和限制

#### 4.5.2 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│   Web 界面 (React)                                          │
│   - Vibe Coding 监控面板                                     │
│   - 实时日志显示                                             │
│   - 检查点审查界面                                           │
│   - 进度可视化                                               │
└────────────────┬────────────────────────────────────────────┘
                 │ HTTP/WebSocket
┌────────────────▼────────────────────────────────────────────┐
│  守护进程 (Daemon Process)                                  │
│  ├─ Agent Orchestrator (编排器)                             │
│  │   ├─ Initializer Agent 管理                              │
│  │   ├─ Coding Agent Pool (4+ 并发)                          │
│  │   └─ MR Creation Agent 管理                              │
│  │                                                          │
│  ├─ Checkpoint Manager (检查点管理)                         │
│  │   ├─ 触发审查请求                                        │
│  │   ├─ 等待用户响应                                        │
│  │   └─ 自动接受逻辑                                        │
│  │                                                          │
│  ├─ Quality Gate Runner (质量门禁执行器)                    │
│  │   ├─ 运行 QG-001 ~ QG-005                                │
│  │   ├─ 自动修复尝试                                        │
│  │   └─ 结果报告                                            │
│  │                                                          │
│  ├─ Git Manager (Git 操作)                                   │
│  │   ├─ 分支管理                                            │
│  │   ├─ 提交历史                                            │
│  │   └─ 回滚机制                                            │
│  │                                                          │
│  ├─ Log Collector (日志收集)                                │
│  │   ├─ Agent 输出捕获                                       │
│  │   ├─ 结构化日志存储                                      │
│  │   └─ 实时推送给前端                                      │
│  │                                                          │
│  └─ State Manager (状态管理)                                │
│      ├─ 会话快照                                            │
│      ├─ 断点续传                                            │
│      └─ 资源监控                                            │
└────────────────┬────────────────────────────────────────────┘
                 │
    ┌────────────┴─────────────┬─────────────┬────────────┐
    ↓            ↓             ↓             ↓            ↓
┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐
│Agent1│  │Agent2│  │Agent3│  │Agent4│  │ ...  │
└──────┘  └──────┘  └──────┘  └──────┘  └──────┘
           AI编码代理集群 (独立进程)
```

#### 4.5.3 通信协议

**前端与守护进程**:
- WebSocket 实时通信
- HTTP REST API 用于命令和控制
- 心跳检测保持连接

**守护进程与 Agent**:
- Stdio管道通信 (stdin/stdout/stderr)
- 每个 Agent 独立进程
- 输出流实时捕获和解析

#### 4.5.4 状态恢复

```rust
struct DaemonState {
    session_id: String,
    project_id: String,
    current_phase: AgentPhase,  // Initializer/Coding/MRCreation
    
    // Agent 状态
    active_agents: Vec<AgentStatus>,
    completed_issues: Vec<Issue>,
    pending_issues: Vec<Issue>,
    
    // 检查点状态
    checkpoints: Vec<Checkpoint>,
    
    // 日志和快照
    log_file: PathBuf,
    last_snapshot: DateTime,
    
    // 资源使用
    cpu_usage: f32,
    memory_usage: usize,
}

// 定期保存状态快照 (每 30 秒)
// 崩溃后从最近快照恢复
// 支持手动暂停和继续
```

### 4.6 Git 集成

#### 4.6.1 自动化 Git 工作流

```
┌─────────────────────────────────────────────────────────────┐
│  Git 自动化流程                                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Initializer Agent:                                          │
│  1. git init (如未初始化)                                   │
│  2. git checkout -b main                                    │
│  3. 创建初始提交：chore: initial commit                     │
│  4. git checkout -b develop                                 │
│                                                              │
│  Coding Agent (每个 Issue):                                  │
│  1. git checkout -b feature/issue-{id}                      │
│  2. 实现代码并运行质量门禁                                   │
│  3. git add .                                               │
│  4. git commit -m "feat: {description}"                     │
│  5. git checkout develop                                    │
│  6. git merge feature/issue-{id}                            │
│  7. git branch -d feature/issue-{id}                        │
│                                                              │
│  MR Creation Agent:                                          │
│  1. git checkout main                                       │
│  2. git merge develop --no-ff                               │
│  3. git push origin main                                    │
│  4. 创建 Merge Request                                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

#### 4.6.2 Git 回滚机制

当质量门禁失败或检测到错误时:

```rust
// 保存安全检查点
fn create_safe_point(&self) -> Result<SafePoint> {
    SafePoint {
        commit_hash: self.git.head()?,
        branch: self.git.current_branch()?,
        timestamp: Utc::now(),
        description: format!("Before issue {}", current_issue.id),
    }
}

// 回滚到安全检查点
fn rollback_to_safe_point(&self, safe_point: &SafePoint) -> Result<()> {
    self.git.reset_hard(&safe_point.commit_hash)?;
    self.git.checkout(&safe_point.branch)?;
    log::warn!("Rolled back to safe point: {}", safe_point.description);
    Ok(())
}
```

#### 4.6.3 提交信息规范

遵循 Conventional Commits:

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式调整
- `refactor:` 重构
- `test:` 测试相关
- `chore:` 构建/工具配置

示例:
```bash
feat: add user authentication with JWT
fix: resolve login page responsive layout issue
test: add unit tests for auth service (15 cases)
```

### 4.2 CLI工具集成

#### 4.2.1 当前实现状态

**框架已完成，具体工具待接入**。已实现的基础功能:

**会话管理:**
- CLISession: 会话数据结构
- SessionManager: 全局会话管理器 (基于 Mutex + HashMap)
- 支持多会话并发

**命令接口:**
- detect_tools: 检测已安装的工具
- create_cli_session: 创建新的 CLI 会话
- send_cli_prompt: 发送指令到 CLI
- read_cli_output: 读取 CLI 输出
- stop_cli_session: 停止 CLI 会话

#### 4.2.2 规划中的工具支持

```
CLITool (抽象接口)
    │
    ├── KimiCLI (月之暗面官方工具)
    ├── ClaudeCode (Anthropic官方工具)
    ├── CodexCLI (OpenAI官方工具)
    └── OpenCode (开源工具)
```

每个工具需要实现:
- 工具检测逻辑
- 进程启动与参数配置
- Stdio 通信协议适配
- 输出解析规则

---

## 5. 数据存储设计

### 5.1 数据库设计

#### 5.1.1 数据库初始化

数据库存储在应用数据目录: `~/.opc-harness/opc-harness.db`

初始化流程:
1. 获取应用数据目录路径
2. 确保目录存在
3. 打开 SQLite 连接
4. 创建必要的数据表

#### 5.1.2 数据表结构

**项目表 (projects):**
- id: 主键 (UUID)
- name: 项目名称
- description: 项目描述
- status: 阶段 (idea/design/coding/marketing/completed)
- progress: 进度百分比
- created_at/updated_at: 时间戳
- idea/prd/user_personas/competitor_analysis: JSON 字段

**AI 配置表 (ai_configs):**
- provider: 主键 (厂商 ID)
- model: 默认模型
- api_key: API密钥

**CLI 会话表 (cli_sessions):**
- id: 主键 (UUID)
- tool_type: 工具类型
- project_path: 项目路径
- created_at: 创建时间

### 5.2 文件系统结构

```
~/.opc-harness/                    # 应用数据目录
├── opc-harness.db                 # SQLite 数据库
└── config.json                    # 应用配置

~/OPC-Harness-Projects/            # 用户项目目录
├── project-name-1/
│   ├── .opc-harness/              # 项目元数据
│   │   ├── manifest.json
│   │   ├── prd.md
│   │   └── user-personas.json
│   ├── src/                       # 源代码
│   └── design/                    # 设计稿
└── project-name-2/
    └── ...
```

### 5.3 密钥管理

使用跨平台密钥管理服务 (keyring-rs):

**存储策略:**
- macOS: Keychain Services
- Windows: Windows Credential Manager
- Linux: Secret Service API

**操作接口:**
- store_api_key: 存储 API Key
- get_api_key: 获取 API Key
- delete_api_key: 删除 API Key

---

## 6. 前端架构

### 6.1 路由结构

```
/                          # 首页仪表盘
/idea                      # 想法输入页面
/prd/:projectId           # PRD 展示页面
/personas/:projectId      # 用户画像页面
/competitors/:projectId   # 竞品分析页面
/coding/:projectId        # 编码工作区
/marketing/:projectId     # 营销运营页面
/ai-config                # AI 配置页面
/settings                 # 设置页面
```

### 6.2 状态管理

采用 Zustand 进行全局状态管理:

**AI 配置 Store:**
- providers: 支持的 AI厂商列表
- configs: 用户配置信息
- Actions: setConfig, removeConfig, getConfig

**项目 Store:**
- projects: 项目列表
- currentProject: 当前选中项目
- Actions: addProject, updateProject, deleteProject

### 6.3 核心组件

**通用组件:**
- AppLayout: 应用布局容器
- Header: 顶部导航栏
- Sidebar: 侧边栏菜单
- Dashboard: 仪表盘

**业务组件:**
- IdeaInput: 想法输入表单
- PRDDisplay: PRD 展示组件
- UserPersonas: 用户画像组件
- CompetitorAnalysis: 竞品分析组件
- CodingWorkspace: 编码工作区
- MarketingStrategy: 营销策略组件
- AIConfig: AI 配置管理

---

## 7. 数据流设计

### 7.1 Vibe Design 数据流

```
用户输入想法 (/idea)
    │
    ▼
┌─────────────────────────────────────────┐
│ 前端：IdeaInput 组件收集用户输入         │
│ - 提供产品创意描述                       │
│ - 选择默认 AI厂商和模型                   │
│ - 显示加载状态和进度                     │
└────────────────┬────────────────────────┘
                 │ IPC: invoke('generate_prd')
                 ▼
┌─────────────────────────────────────────┐
│ 后端：AI 服务模块                         │
│ ├─ 解析 provider 类型                     │
│ ├─ 创建 AIProvider 实例                   │
│ ├─ 构造系统 Prompt + 用户输入            │
│ └─ 发送 HTTP 请求到 AI厂商 API             │
└────────────────┬────────────────────────┘
                 │ HTTP POST /v1/chat/completions
                 ▼
┌─────────────────────────────────────────┐
│ 云端AI 服务 (OpenAI/Claude/Kimi/GLM)     │
│ ├─ 理解用户需求                          │
│ ├─ 生成结构化 PRD 文档                    │
│ └─ 返回 JSON 格式响应                      │
└────────────────┬────────────────────────┘
                 │ HTTP Response
                 ▼
┌─────────────────────────────────────────┐
│ 后端：解析响应，保存到数据库             │
│ ├─ 解析 PRDResponse 结构                  │
│ ├─ 保存到 projects 表                     │
│ └─ 返回给前端                            │
└────────────────┬────────────────────────┘
                 │ IPC Response
                 ▼
┌─────────────────────────────────────────┐
│ 前端：PRDDisplay 组件展示结果            │
│ ├─ Markdown 格式化显示                    │
│ ├─ 可编辑和导出                          │
│ └─ 导航到下一步 (用户画像/竞品分析)       │
└─────────────────────────────────────────┘
```

### 7.2 Vibe Coding 数据流 (自主编码)

```
用户确认 PRD，进入编码阶段 (/coding/:projectId)
    │
    ▼
┌─────────────────────────────────────────┐
│ 前端：启动 Vibe Coding 流程              │
│ - 显示项目信息和 PRD 摘要                │
│ - 选择 AI厂商和模型策略                  │
│ - 配置 HITL 检查点自动接受阈值           │
│ - [开始自主编码] 按钮                    │
└────────────────┬────────────────────────┘
                 │ IPC: invoke('spawn_initializer_agent')
                 ▼
┌─────────────────────────────────────────┐
│ 后端：守护进程启动 Initializer Agent     │
│ ├─ 创建守护进程会话                      │
│ ├─ 初始化 Agent 通信管道                  │
│ ├─ 加载 PRD 文档和项目上下文              │
│ └─ 返回会话 ID 给前端                     │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ Initializer Agent 执行中...               │
│ ├─ 读取并理解 PRD 文档                    │
│ ├─ 检查开发环境 (Node.js/Git)           │
│ ├─ 初始化 Git 仓库                        │
│ ├─ 任务分解：PRD → Milestones           │
│ ├─ Milestone → Issues (10-20 个)         │
│ └─ 创建 GitLab Issues / 本地 JSON 追踪   │
└────────────────┬────────────────────────┘
                 │ 触发 CP-002
                 ▼
┌─────────────────────────────────────────┐
│ HITL 检查点：CP-002 任务分解审查          │
│ ├─ 前端显示 Issue 列表和优先级            │
│ ├─ 用户审查任务分解质量                 │
│ ├─ [批准继续] / [要求修改]               │
│ └─ 用户批准后继续                        │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ 守护进程：启动 Coding Agent Pool         │
│ ├─ 创建 4+ 个并发 Coding Agents            │
│ ├─ 分配 Issues 给各 Agent                  │
│ ├─ 每个 Agent 独立功能分支                │
│ └─ 并行执行编码任务                      │
└────────────────┬────────────────────────┘
                 │
          ┌──────┴──────┬──────────┬──────────┐
          ↓             ↓          ↓          ↓
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│CodingAgent#1│ │CodingAgent#2│ │CodingAgent#3│
│Issue #1     │ │Issue #2     │ │Issue #3     │
│用户认证     │ │登录页面     │ │数据库设计   │
└──────┬──────┘ └──────┬──────┘ └──────┬──────┘
       │               │               │
       │ 每个 Agent 执行以下流程:        │
       ▼               ▼               ▼
┌─────────────────────────────────────────┐
│ 单个 Coding Agent 工作流程:               │
│ 1. 选择 Issue (触发 CP-005)             │
│ 2. 创建功能分支                         │
│ 3. 分析需求并实现代码                   │
│ 4. 生成单元测试                         │
│ 5. 运行质量门禁 QG-001~QG-003           │
│    ├─ 失败 → 自动修复 (最多 3 次)         │
│    └─ 仍失败 → Git 回滚 + 通知            │
│ 6. Git提交                              │
│ 7. 触发 CP-006 (Issue 完成审查)          │
│ 8. 用户批准后合并到 develop             │
└─────────────────────────────────────────┘
                 │
                 ▼ (所有 Issues 完成后)
┌─────────────────────────────────────────┐
│ 触发 CP-007: MR 创建审查                 │
│ ├─ 显示所有完成的 Issues                │
│ ├─ 确认是否进入 MR 创建阶段              │
│ └─ 用户批准                            │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ MR Creation Agent 执行中...              │
│ ├─ 合并所有 feature 分支到 develop       │
│ ├─ 运行回归测试 QG-004                  │
│ ├─ 合并 develop 到 main                   │
│ ├─ 生成 MR 描述                          │
│ │   - 变更概述                           │
│ │   - 实现的 Issues 列表                 │
│ │   - 测试覆盖率报告                    │
│ │   - 质量门禁状态                      │
│ └─ 创建 Merge Request                   │
└────────────────┬────────────────────────┘
                 │ 触发 CP-008
                 ▼
┌─────────────────────────────────────────┐
│ HITL 检查点：CP-008 最终 MR 审查           │
│ ├─ 显示完整 MR 内容和变更 diff           │
│ ├─ 用户审查最终代码质量                 │
│ ├─ [批准合并] / [要求修改]               │
│ └─ 用户批准后完成                       │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ 前端：显示编码完成总结                   │
│ ├─ 显示完成的代码统计                   │
│ ├─ 测试覆盖率报告                       │
│ ├─ Git 仓库位置                          │
│ └─ [一键部署] 按钮                      │
└─────────────────────────────────────────┘
```

**关键特性**:
- **透明化**: 实时显示每个 Agent 的工作进展
- **可干预**: 8 个检查点需要人工审查和批准
- **易监控**: 详细日志和进度追踪
- **低干扰**: 自动化执行，仅在必要时请求用户输入
- **有保障**: 多层次质量门禁确保代码质量
- **可回滚**: Git 机制保障随时恢复到安全点

---

## 8. 工具检测

### 8.1 已实现功能

**工具检测命令:**
- 检测 Node.js 安装状态
- 检测 Git 安装状态
- 预留 Kimi CLI、Claude Code、Codex CLI 检测接口

**返回信息:**
- 工具名称
- 安装状态
- 版本号 (如已安装)
- 安装链接 (如未安装)

### 8.2 前端展示

工具列表展示:
- 显示各工具的名称和描述
- 标识已安装/未安装状态
- 提供安装链接跳转
- 支持选择工具创建会话

---

## 9. 安全设计

### 9.1 API密钥安全管理

**存储策略:**
- 使用系统级密钥存储服务
- 跨平台支持 (Keychain/Credential Manager/Secret Service)
- 不在配置文件或数据库中明文存储

**操作接口:**
- 存储：将 API Key 存入系统钥匙串
- 获取：从系统钥匙串读取
- 删除：从系统钥匙串移除

### 9.2 网络安全策略

**允许连接的服务:**
- OpenAI API (api.openai.com)
- Anthropic API (api.anthropic.com)
- Kimi API (api.moonshot.cn)
- GLM API (open.bigmodel.cn)
- MiniMax API (api.minimax.chat)

**禁止行为:**
- ✗ 不会上传用户代码到第三方服务器
- ✗ 不会收集用户行为数据
- ✗ 不会主动连接非 AI厂商的服务

### 9.3 Tauri 安全配置

**权限控制:**
- 最小权限原则
- 仅启用必要的插件能力
- 文件系统访问限制

**CSP 配置:**
- 宽松策略 (开发阶段)
- 生产环境可收紧策略

---

## 10. 性能与成本优化

### 10.1 智能模型选择

根据任务类型推荐合适的模型:

| 任务类型 | 推荐模型 | 备选模型 | 经济方案 |
|---------|---------|---------|---------|
| PRD 生成 | GPT-4o | Claude 3.5 | GPT-4o-mini |
| 代码生成 | Codex | Claude 3.5 | GPT-4o-mini |
| 营销文案 | Claude 3.5 | GPT-4o | GLM-4-Air |
| 简单问答 | GPT-4o-mini | GLM-4-Air | GPT-4o-mini |

### 10.2 响应缓存机制

**缓存策略:**
- TTL: 24 小时
- Key 生成：基于请求内容哈希
- 自动清理过期缓存

**适用场景:**
- 相同的 Prompt 重复请求
- PRD 生成结果持久化
- 用户画像缓存复用

### 10.3 Token 优化

**优化策略:**
- 自动截断超长上下文
- Prompt 压缩技术
- 流式输出减少等待时间

---

## 附录

### A. 技术选型总结

| 层级 | 技术 | 选择理由 |
|------|------|---------|
| **桌面框架** | Tauri v2 | 轻量、安全、Rust 性能、适合 Agent 编排 |
| **前端** | React 18 | 生态成熟、类型安全、组件化 |
| **语言** | TypeScript 5 | 类型安全、开发体验好 |
| **样式** | Tailwind CSS 3 | 开发效率高 |
| **UI 组件** | shadcn/ui | 美观、可定制、基于 Radix UI |
| **状态管理** | Zustand 4 | 轻量、易用、支持中间件 |
| **路由** | React Router 6 | 标准解决方案 |
| **后端** | Rust | 性能、安全、CLI集成友好、适合并发 |
| **HTTP 客户端** | reqwest | 异步、功能完善 |
| **数据库** | SQLite | 零配置、单文件、嵌入式 |
| **密钥存储** | OS Keychain | 最安全的方式 |
| **进程管理** | tokio | 异步运行时、多任务调度 |
| **Git 操作** | git2 | libgit2 bindings、完整 Git 能力 |
| **文件监控** | notify | 跨平台文件系统监听 |
| **Agent 通信** | Stdio + WebSocket | 实时双向通信 |

### B. AI 服务与 Agent

#### B.1 云端AI厂商

| 厂商 | 代表模型 | 特点 | 适用场景 | Agent 角色 |
|------|---------|------|---------|-----------|
| OpenAI | GPT-4o, o1 | 综合能力最强 | 通用任务、代码生成 | Coding Agent |
| Anthropic | Claude 3.5 Sonnet | 长上下文、代码强 | 代码生成、PRD、审查 | Initializer/MR Agent |
| 月之暗面 | Kimi K1.5 | 中文优秀 | 中文内容生成 | Coding Agent |
| 智谱 AI | GLM-4, CodeGeeX | 国产、CodeGeeX专业 | 代码生成 | Coding Agent |
| MiniMax | abab6.5 | 性价比高 | 简单任务、测试生成 | Coding Agent |

#### B.2 Agent 角色与职责

| Agent 角色 | 推荐模型 | 主要职责 | 通信方式 |
|-----------|---------|---------|---------|
| **Initializer Agent** | Claude 3.5 Sonnet | 理解 PRD、任务分解、环境初始化 | Stdio + WebSocket |
| **Coding Agent #1~4+** | GPT-4o / Kimi K1.5 | 具体 Issue 实现、测试生成 | Stdio + WebSocket |
| **MR Creation Agent** | Claude 3.5 Sonnet | 汇总提交、生成 MR 描述 | Stdio + WebSocket |
| **Quality Gate Runner** | - (本地工具) | 执行代码检查、类型检查、测试 | 进程调用 |

#### B.3 Agent 提示词模板

每个 Agent 角色有专门的系统提示词:

```
Initializer Agent System Prompt:
"你是一名资深软件架构师，负责将产品需求文档 (PRD) 分解为可执行的开发任务。
你的职责包括：
1. 理解 PRD 中的产品需求和技术栈
2. 设计合理的系统架构和模块划分
3. 创建 Milestones (3-5 个关键里程碑)
4. 将每个 Milestone 分解为具体的 Issues (10-20 个)
5. 为每个 Issue 编写详细描述和验收标准
6. 评估技术难度和工作量

请确保任务分解合理、优先级清晰、依赖关系明确。"

Coding Agent System Prompt:
"你是一名全栈开发工程师，负责自主完成指定的开发任务。
你的工作流程：
1. 仔细阅读 Issue 描述和验收标准
2. 分析项目现有代码结构和上下文
3. 创建 Git 功能分支
4. 实现代码功能
5. 编写单元测试 (目标覆盖率 > 80%)
6. 运行质量门禁 (Lint/Type Check/Test)
7. 如果失败则自动修复 (最多 3 次)
8. Git提交并等待审查

请确保代码质量高、测试完善、符合项目规范。"
```

### C. 开发命令

```
# 前端开发
npm run dev              # Vite 开发服务器
npm run build            # 生产构建
npm run preview          # 预览构建

# Tauri 开发
npm run tauri:dev        # Tauri 开发环境 (含守护进程)
npm run tauri:build      # 生产安装包

# 代码质量
npm run lint             # ESLint 检查
npm run format           # Prettier 格式化

# Harness Engineering
npm run harness:check    # 架构健康检查
npm run harness:gc       # 垃圾回收

# Agent 调试 (开发中)
npm run agent:debug      # 启动 Agent 调试模式
npm run agent:logs       # 查看 Agent 执行日志
```

### D. 项目目录结构

```
opc-harness/
├── src/                          # 前端代码
│   ├── components/               # UI 组件
│   │   ├── common/               # 通用组件
│   │   │   ├── AppLayout.tsx
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Dashboard.tsx
│   │   ├── ui/                   # shadcn/ui 组件
│   │   ├── vibe-design/          # Vibe Design 模块
│   │   │   ├── IdeaInput.tsx
│   │   │   ├── PRDDisplay.tsx
│   │   │   ├── UserPersonas.tsx
│   │   │   └── CompetitorAnalysis.tsx
│   │   ├── vibe-coding/          # Vibe Coding 模块
│   │   │   ├── CodingWorkspace.tsx
│   │   │   ├── AgentMonitor.tsx           # Agent 监控面板
│   │   │   ├── CheckpointReview.tsx       # 检查点审查界面
│   │   │   ├── LogTerminal.tsx            # 实时日志终端
│   │   │   └── ProgressVisualization.tsx  # 进度可视化
│   │   └── vibe-marketing/       # Vibe Marketing 模块
│   │       └── MarketingStrategy.tsx
│   ├── stores/                   # Zustand stores
│   │   ├── aiConfigStore.ts
│   │   ├── projectStore.ts
│   │   └── agentStore.ts         # Agent 状态管理 (新增)
│   ├── types/                    # TypeScript 类型
│   │   └── index.ts              # 包含 Agent/Checkpoint类型定义
│   └── App.tsx                   # 应用入口
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # Rust 入口 (含守护进程初始化)
│   │   ├── commands/             # Tauri Commands
│   │   │   ├── ai.rs             # AI 服务命令
│   │   │   ├── agent.rs          # Agent 编排命令 (新增)
│   │   │   ├── checkpoint.rs     # HITL 检查点命令 (新增)
│   │   │   ├── quality_gate.rs   # 质量门禁命令 (新增)
│   │   │   └── system.rs         # 系统相关命令
│   │   ├── agents/               # Agent 实现 (新增目录)
│   │   │   ├── mod.rs
│   │   │   ├── initializer.rs    # Initializer Agent
│   │   │   ├── coding.rs         # Coding Agent
│   │   │   └── mr_creation.rs    # MR Creation Agent
│   │   ├── checkpoint/           # HITL 检查点管理 (新增目录)
│   │   │   ├── mod.rs
│   │   │   └── manager.rs        # CheckpointManager 实现
│   │   ├── quality_gates/        # 质量门禁系统 (新增目录)
│   │   │   ├── mod.rs
│   │   │   ├── runner.rs         # QualityGateRunner
│   │   │   ├── lint.rs           # 代码检查
│   │   │   ├── types.rs          # 类型检查
│   │   │   └── tests.rs          # 单元测试
│   │   ├── daemon/               # 守护进程 (新增目录)
│   │   │   ├── mod.rs
│   │   │   ├── orchestrator.rs   # Agent 编排器
│   │   │   └── state.rs          # 状态管理
│   │   ├── ai/                   # AI 服务
│   │   ├── db/                   # 数据库
│   │   └── services/             # 业务逻辑
│   └── Cargo.toml                # Rust 依赖
├── docs/                         # 文档
│   ├── 产品设计.md
│   ├── 架构设计.md
│   └── AGENTS.md                 # AI 导航地图
├── .harness/                     # Harness Engineering
│   ├── context-engineering/
│   │   ├── decision-records/     # 决策记录
│   │   └── execution-logs/       # 执行日志
│   └── checkpoints/              # 检查点配置 (新增)
└── package.json                  # 前端依赖
```

---

> **文档版本**: v3.0  
> **最后更新**: 2026 年 3 月 22 日  
> **维护者**: OPC-HARNESS Team
> 
> 本文档描述了 OPC-HARNESS 的高层架构设计，不包含具体代码实现细节。如需了解实现细节，请参考源代码或 AGENTS.md 导航地图。
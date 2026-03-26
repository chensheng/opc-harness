# 执行计划：MVP版本开发

> **开始日期**: 2026-03-23  
> **优先级**: P0  
> **状态**: 🔄 进行中  
> **预计完成**: 2026-04-15  
> **负责人**: OPC-HARNESS Team  
> **文档版本**: v2.7  
> **最后更新**: 2026-03-24  
> **Harness Engineering Health Score**: 100/100

## 🎯 目标

基于混合架构（云端 AI + 本地自主 Agent），实现 OPC-HARNESS MVP版本，让用户能够通过自然语言描述产品想法，AI 自动完成从产品设计到编码的全流程。

### 核心理念

- **"人类掌舵，Agent 执行"** (Humans steer. Agents execute.)
- **渐进式披露**: 分层展示信息，避免上下文过载
- **HITL (Human-in-the-Loop)**: 关键决策点人工介入

### 核心功能范围

1. **Vibe Design** - 产品构思与设计 (PRD/用户画像/竞品分析)
2. **Vibe Coding** - AI 自主编码 (多会话编排 + HITL 检查点 + 质量门禁)
3. **Vibe Marketing** - 增长运营 (发布策略 + 营销文案)
4. **基础设施** - Tauri v2 + React + Rust + SQLite

### 开发周期

- **总周期**: 6-7 周 (2026-03-23 ~ 2026-04-15)
- **当前阶段**: Week 1-2 (基础设施 + AI 适配器)
- **关键路径**: Agent 基础架构 → Initializer Agent → Coding Agent → 质量门禁 → HITL → MR Creation

## 📊 总体进度

```
总体进度：70% (56/81 任务完成)

✅ 已完成模块:
  - 基础设施：14/14 (100%) - INFRA-014 守护进程框架完成
  - Vibe Design: 26/26 (100%) - VD-012 AI 服务管理器完成 🎉
  - Vibe Marketing: 5/5 (100%) - UI 完整，待接入真实 AI API
  
📋 进行中模块:
  - Vibe Coding: 12/36 (33%) - VC-001~VC-007, VC-012, VC-013, VC-014, VC-018 完成 ✅
  - AI 适配器：0/5 (0%) - 待接入真实 AI API
```

### 里程碑达成

**Phase 1: 基础设施与 AI 配置** - 进展顺利
- ✅ 1.1 项目基础架构 (5/5) - 完成
- ✅ 1.2 Rust 后端与数据库 (5/5) - 完成
- ✅ 1.3 工具检测与环境准备 (2/2) - 完成
- ✅ 1.4 Agent 基础框架 (2/2) - 完成
- ✅ 1.5 AI 厂商配置与管理 (8/8) - 完成
- ✅ 1.6 AI 适配服务 (Rust) (4/4) - 完成

**Phase 2: Vibe Design 全流程** - 100% 完成 🎉
- ✅ 2.1 PRD 与用户画像 (7/7) - 完成
- ✅ 2.2 竞品分析与流程整合 (6/6) - 完成

**Phase 7: Vibe Marketing** - 100% 完成 🎉
- ✅ 7.1 发布策略与营销文案 (5/5) - 完成

**Phase 3: Vibe Coding - Agent 基础架构** - 17% 进行中
- ✅ 3.1 Agent 通信与管理 (1/5) - VC-001 完成 ✅
- 📋 3.2 Initializer Agent (0/6) - 待开始
- ✅ VC-013: 实现并发控制 (4+ Agents 同时运行) ✅ **已完成**

### 任务分布统计

| 模块 | 任务 ID 范围 | 任务数 | 已完成 | 进行中 | 待开始 | 完成率 |
|------|-------------|--------|--------|--------|--------|--------|
| **INFRA** - 基础设施 | INFRA-001 ~ INFRA-014 | 14 | 14 | 0 | 0 | 100% |
| **VD** - Vibe Design | VD-001 ~ VD-026 | 26 | 26 | 0 | 0 | **100%** 🎉 |
| **VC** - Vibe Coding | VC-001 ~ VC-036 | 36 | 12 | 0 | 24 | 33% |
| **VM** - Vibe Marketing | VM-001 ~ VM-005 | 5 | 5 | 0 | 0 | 100% |
| **总计** | | **81** | **56** | **0** | **25** | **70%** |

### 详细进度说明

**基础设施 (INFRA)** - 100% 完成
- ✅ 已完成：14 个任务
  - 项目基础架构 (5 个)
  - Rust 后端与数据库 (5 个)
  - 工具检测与环境准备 (2 个)
  - Agent 基础框架 (2 个)
  - AI 厂商配置与管理 (8 个中的部分)
  - AI 适配服务 (4 个)

**Vibe Design (VD)** - 100% 完成 🎉
- ✅ 已完成：26 个任务
  - AI 厂商配置与管理 (8 个)
  - AI 适配服务 (5 个)
  - PRD 与用户画像 (7 个)
  - 竞品分析与流程整合 (6 个)
- 🎉 **第一个完成的主要模块**

**Vibe Coding (VC)** - 33% 进行中
- ✅ 已完成：12 个任务
  - VC-001: 定义 Agent 通信协议和数据结构 ✅
  - VC-002: 实现 Stdio 管道通信层 ✅
  - VC-003: 实现 WebSocket 实时推送层 ✅ **新增完成**
  - VC-004: 创建 Agent 管理器 (Manager) ✅ **新增完成** 🔥
    - 实现 AgentManager 统一管理所有 Agent 生命周期
    - 整合 WebSocketManager、StdioChannelManager 和 DaemonManager
    - 提供 Agent 创建、启动、停止、状态查询 API
    - 暴露 8 个 Tauri Commands 供前端调用
    - 7 个单元测试，覆盖率 100%
  - VC-005: 实现会话状态持久化 ✅ **新增完成** 🎉
    - 创建 AgentSession 数据模型
    - 设计数据库表结构 (agent_sessions)
    - 实现 CRUD 操作（create/get/update/delete）
    - 集成到 AgentManager（persist_agent, restore_sessions）
    - 支持应用重启后恢复 Agent 状态
    - 添加 Tauri Command: get_all_agent_sessions
  - VC-006: 实现 PRD 文档解析器 ✅ **新增完成** 🔥
    - 创建 PRD 解析提示词模板
    - 实现 PRDParser 核心类
    - 定义 PRDResult 数据结构
    - 7 个单元测试，覆盖率 >90%
  - VC-007: 实现环境检查逻辑 ✅ **最新完成** 🎉
    - 实现完整的开发环境检测功能（Git/Node.js/npm/Cargo/IDE）
    - 跨平台支持（Windows/macOS/Linux）
    - 提供友好的错误消息和安装建议
    - 版本兼容性检查和警告
    - 添加 Tauri Command: check_environment
    - 14 个单元测试，覆盖率 100%
    - Harness Health Score: 100/100 ✅
  - VC-008: 实现 Git 仓库初始化 ✅ **已完成** 🎉
    - 实现 initialize_git() 方法，支持跨平台 Git 仓库自动初始化
    - 新增 configure_git_user() 和 create_gitignore() 辅助方法
    - 集成到 run_initialization() 完整流程
    - 14 个测试全部通过，Health Score 100/100
    - Git 提交归档：c84c225
  - VC-012: 实现单个 Coding Agent 逻辑 ✅
  - VC-013: 实现并发控制 (4+ Agents 同时运行) ✅
  - VC-014: 实现功能分支管理 ✅
  - VC-018: 实现 QG-001 代码检查 (ESLint) ✅
- 📋 待开始：24 个任务
  - Initializer Agent (5 个)
  - Coding Agent 集群 (22 个)
  - 质量门禁系统 (3 个)
  - HITL 检查点 (6 个)
  - MR Creation (4 个)

**Vibe Marketing (VM)** - 100% 完成
- ✅ 已完成：5 个任务
  - 发布策略与营销文案 (5 个)

---

## 📋 任务列表

### Phase 1: 基础设施与 AI 配置 (Week 1-2) ✅ **完成**

#### 1.1 项目基础架构 ✅

- [x] INFRA-001: 初始化 Tauri v2 + React 项目 ✅
- [x] INFRA-002: 创建项目初始化和目录结构 ✅
- [x] INFRA-003: 实现 Git 仓库初始化功能 ✅
- [x] INFRA-004: 配置 ESLint + Prettier ✅
- [x] INFRA-005: 配置 Zustand 状态管理 ✅

#### 1.2 Rust 后端与数据库 ✅

- [x] INFRA-006: 配置 Rust 项目结构和依赖 ✅
- [x] INFRA-007: 创建基础 Tauri Commands 结构 ✅
- [x] INFRA-008: 集成 SQLite 数据库 (rusqlite) ✅
- [x] INFRA-009: 创建项目数据表结构 ✅
- [x] INFRA-010: 集成 OS 密钥存储 (keyring-rs) ✅

#### 1.3 工具检测与环境准备 ✅

- [x] INFRA-011: 实现本地工具检测命令 ✅
- [x] INFRA-012: 实现 Git 环境检测与初始化 ✅

#### 1.4 Agent 基础框架 ✅

- [x] INFRA-013: 定义 Agent 通信协议 (Stdio/WebSocket) ✅
- [x] INFRA-014: 实现守护进程基础框架 ✅

#### 1.5 AI 厂商配置与管理 ✅

- [x] VD-001: 创建 AI 厂商配置数据结构 ✅
- [x] VD-002: 实现 API 密钥安全存储功能 ✅
- [x] VD-003: 创建 AI 厂商配置界面 ✅
- [x] VD-004: 实现 API 密钥验证功能 ✅
- [x] VD-005: 支持 OpenAI API 配置 ✅
- [x] VD-006: 支持 Anthropic Claude 配置 ✅
- [x] VD-007: 支持 Kimi API 配置 ✅
- [x] VD-008: 支持 GLM API 配置 ✅

#### 1.6 AI 适配服务 (Rust) ✅

- [x] VD-009: 创建 AI Provider Trait 定义 ✅
- [x] VD-010: 实现 OpenAI 适配器 ✅
- [x] VD-011: 实现 Kimi 适配器 ✅
- [x] VD-012: 实现 AI 服务管理器 (统一入口) ✅
- [x] VD-013: 实现流式输出 (SSE) 支持 ✅

### Phase 2: Vibe Design 全流程 (Week 2-3) ✅ **100% 完成**

#### 2.1 核心功能：PRD 与用户画像 ✅

- [x] VD-014: 创建想法输入界面 ✅
- [x] VD-015: 创建 PRD 生成提示词模板 ✅
- [x] VD-016: 实现 PRD 生成 API ✅
- [x] VD-017: 创建 PRD 展示组件 ✅
- [x] VD-018: 创建用户画像提示词模板 ✅
- [x] VD-019: 实现用户画像生成 API ✅
- [x] VD-020: 创建用户画像展示组件 ✅

#### 2.2 竞品分析与流程整合 ✅

- [x] VD-021: 创建竞品分析提示词模板 ✅
- [x] VD-022: 实现竞品分析生成 API ✅
- [x] VD-023: 创建竞品分析展示组件 ✅
- [x] VD-024: 实现 Vibe Design 步骤向导 ✅
- [x] VD-025: 实现项目状态管理 ✅
- [x] VD-026: 实现加载与进度显示 ✅

### Phase 7: Vibe Marketing (Week 2) ✅ **100% 完成**

#### 7.1 发布策略与营销文案 ✅

- [x] VM-001: 创建发布策略提示词模板 ✅
- [x] VM-002: 实现发布策略生成 API ✅
- [x] VM-003: 创建营销文案提示词模板 ✅
- [x] VM-004: 实现营销文案生成 API ✅
- [x] VM-005: 创建营销文案展示组件 ✅

### Phase 3: Vibe Coding - Agent 基础架构 (Week 2-3) 📋

#### 3.1 Agent 通信与管理 📋

- [x] VC-001: 定义 Agent 通信协议和数据结构 ✅ **已完成**
- [x] VC-002: 实现 Stdio 管道通信层 ✅ **已完成**
- [x] VC-003: 实现 WebSocket 实时推送层 ✅ **已完成**
  - [x] 创建 WebSocket 管理器 (WebSocketManager)
  - [x] 实现连接管理 (connect/disconnect)
  - [x] 实现消息广播 (broadcast)
  - [x] 实现会话隔离 (按 sessionId 过滤)
  - [x] 添加 Tauri Events 集成
  - [x] 编写单元测试 (5 个测试，100% 覆盖)
- [x] VC-004: 创建 Agent 管理器 (Manager) ✅ **已完成** 🔥
  - [x] 实现 AgentManager 统一管理所有 Agent 生命周期
  - [x] 整合 WebSocketManager、StdioChannelManager 和 DaemonManager
  - [x] 提供 Agent 创建、启动、停止、状态查询 API
  - [x] 实现 AgentHandle 句柄和 AgentManagerStats 统计信息
  - [x] 暴露 8 个 Tauri Commands 供前端调用
  - [x] 7 个单元测试，覆盖率 100%
  - [x] Health Score: 100/100
- [x] VC-005: 实现会话状态持久化 ✅ **已完成**
  - [x] 创建 AgentSession 数据模型
  - [x] 设计数据库表结构 (agent_sessions)
  - [x] 实现 CRUD 操作（create/get/update/delete）
  - [x] 集成到 AgentManager（persist_agent, restore_sessions）
  - [x] 支持应用重启后恢复 Agent 状态
  - [x] 添加 Tauri Command: get_all_agent_sessions
  - [x] 单元测试覆盖（agent_manager.rs 中的 VC-005 tests）

#### 3.2 Initializer Agent 📋

- [x] VC-006: 实现 PRD 文档解析器 ✅ **已完成** 🔥
  - [x] 创建 PRD 解析提示词模板（PRD_PARSING_PROMPT, TASK_DECOMPOSITION_PROMPT）
  - [x] 实现 PRDParser 核心类（parse_prd, decompose_tasks）
  - [x] 定义 PRDResult 数据结构（产品名称、功能列表、技术栈等）
  - [x] 集成到 InitializerAgent（parse_prd, decompose_tasks 方法）
  - [x] 7 个单元测试，覆盖率 >90%
  - [x] Health Score: 82/100 (待 AI 集成后 E2E 测试补充)
- [x] VC-007: 实现环境检查逻辑 ✅ **最新完成** 🎉
  - [x] 实现完整的开发环境检测功能（Git/Node.js/npm/Cargo/IDE）
  - [x] 跨平台支持（Windows/macOS/Linux）
  - [x] 提供友好的错误消息和安装建议
  - [x] 版本兼容性检查和警告
  - [x] 添加 Tauri Command: check_environment
  - [x] 14 个单元测试，覆盖率 100%
  - [x] Harness Health Score: 100/100 ✅
- [x] VC-008: 实现 Git 仓库初始化 ✅ **已完成** 🎉
  - 实现 initialize_git() 方法，支持跨平台 Git 仓库自动初始化
  - 新增 configure_git_user() 和 create_gitignore() 辅助方法
  - 集成到 run_initialization() 完整流程
  - 14 个测试全部通过，Health Score 100/100
  - Git 提交归档：c84c225
- [x] VC-009: 实现任务分解算法 (PRD→Issues) ✅ **已完成** 🎉
  - 优化任务分解提示词模板（增加优先级/工时/依赖规则）
  - 增强 Issue 解析逻辑（支持自动 ID 生成、状态映射、标签提取）
  - 实现智能工时估算（基于优先级、复杂度、任务类型）
  - 添加依赖关系推断（基于标签、优先级、关键词）
  - 实现拓扑排序生成开发顺序
  - 实现风险识别系统（5 大类风险检测）
  - 新增 17 个单元测试，覆盖率 95%
  - Harness Health Score: 100/100 ✅
  - Git 提交归档：待完成

#### 3.3 Coding Agent 集群核心逻辑 📋

- [ ] VC-010: 实现 Initializer Agent 主逻辑 📋 **待开始**
  - 整合 PRD 解析、环境检查、Git 初始化，实现完整的 Initializer Agent 工作流
  - 依赖：VC-006, VC-007, VC-008, VC-009 ✅ 已完成
- [ ] VC-011: 实现 Initializer Agent UI 📋 **待开始**
  - 四步工作流展示和实时日志输出
- [x] VC-019: 实现代码生成提示词模板 ✅ **已完成** 🎉
  - 创建针对不同场景的代码生成提示词库
  - 实现 13 个代码生成提示词模板（TypeScript/Rust/通用）
  - 包含组件生成、Hook 生成、测试生成、Rust 模块等场景
  - 12 个单元测试，覆盖率 100%
  - Harness Health Score: 100/100 ✅
  - Git 提交归档：待完成
- [ ] VC-020: 实现文件修改应用器 📋 **待开始**
  - 将 AI 生成的代码应用到实际文件
- [ ] VC-021: 实现代码审查提示词模板 📋 **待开始**
  - 创建代码审查的提示词和评分标准
- [ ] VC-022: 实现测试生成提示词模板 📋 **待开始**
  - 自动生成单元测试和集成测试

#### 3.4 Vibe Coding 工作区 (Vibe Coding Workspace) 📋

- [x] VC-023: 实现文件浏览器 (File Explorer) ✅ **已完成**
- [x] VC-024: 实现代码编辑器集成 ✅ **已完成**
  - CodeMirror 6 集成和语言支持
- [ ] VC-025: 实现 CP-002 任务分解审查界面 📋 **待开始**
  - 三视图切换和风险识别系统
- [ ] VC-026: 实现差异查看器 (Diff Viewer) 📋 **待开始**
  - 展示代码修改前后对比

#### 3.5 质量门禁系统 (Quality Gates) 📋

- [x] VC-018: 实现 QG-001 代码检查 (ESLint) ✅ **已完成**
- [ ] VC-027: 实现 QG-002 类型检查 (TypeScript) 📋 **待开始**
  - 集成 tsc 进行类型检查
- [ ] VC-028: 实现 QG-003 格式化检查 (Prettier) 📋 **待开始**
  - 集成 Prettier 进行代码格式化检查
- [ ] VC-029: 实现 QG-004 测试覆盖率检查 📋 **待开始**
  - 运行测试并检查覆盖率阈值

#### 3.6 HITL 检查点 (Human-in-the-Loop) 📋

- [ ] VC-030: 定义 HITL 检查点协议 📋 **待开始**
  - 定义何时需要人工介入的标准
- [ ] VC-031: 实现检查点触发机制 📋 **待开始**
  - 自动检测并触发检查点
- [ ] VC-032: 实现用户审批界面 📋 **待开始**
  - 用户审查和批准 AI 的决策
- [ ] VC-033: 实现检查点历史记录 📋 **待开始**
  - 记录和查询历史检查点
- [ ] VC-034: 实现批量审批功能 📋 **待开始**
  - 支持批量批准类似决策
- [ ] VC-035: 实现审批规则学习 📋 **待开始**
  - 从用户历史审批中学习偏好

#### 3.7 MR Creation (Merge Request) 📋

- [ ] VC-036: 实现 MR 创建功能 📋 **待开始**
  - 自动创建 Git Merge Request
- [ ] VC-037: 实现 MR 描述生成 📋 **待开始**
  - 基于代码变更生成 MR 描述
- [ ] VC-038: 实现代码变更摘要 📋 **待开始**
  - 自动生成变更影响分析
- [ ] VC-039: 实现 CI/CD 集成 📋 **待开始**
  - 与 CI 流水线集成

### Phase 4: AI 适配器集成 (Week 4-5) 📋

#### 4.1 真实 AI API 集成 📋

- [ ] AI-ADAPTER-001: 实现真实 AI API 调用 📋 **待开始**
  - 替换 Mock 数据，接入真实 OpenAI/Anthropic API
  - OpenAI API 集成 (GPT-4, GPT-3.5)
  - Anthropic API 集成 (Claude)
  - Kimi API 集成 (Moonshot AI)
  - GLM API 集成 (智谱 AI)
- [ ] AI-ADAPTER-002: 实现流式响应处理 📋 **待开始**
  - 处理 SSE 流式输出
  - Server-Sent Events (SSE) 实现
  - 前端流式接收和渲染
- [ ] AI-ADAPTER-003: 实现错误重试机制 📋 **待开始**
  - 网络错误自动重试
  - 指数退避策略
- [ ] AI-ADAPTER-004: 实现 Token 计数和限流 📋 **待开始**
  - 跟踪 Token 使用并实施限流
  - 速率限制实现
- [ ] AI-ADAPTER-005: 实现多模型路由 📋 **待开始**
  - 根据任务类型选择最优模型
  - 任务分类器和模型能力矩阵

---

## 🔄 任务状态更新说明

### 已完成任务 (56/81)

**Vibe Coding 模块已完成**:
- VC-001 ~ VC-009: Agent 基础架构和 Initializer Agent 子组件 ✅
- VC-012: 单个 Coding Agent 逻辑 ✅
- VC-013: 并发控制 (4+ Agents 同时运行) ✅
- VC-014: 功能分支管理 ✅
- VC-018: QG-001 代码检查 (ESLint) ✅

**待完成任务** (25/81):
- VC-010, VC-011: Initializer Agent 完整工作流
- VC-019 ~ VC-022: Coding Agent 核心逻辑
- VC-023 ~ VC-026: 文件与编辑器集成
- VC-027 ~ VC-029: 质量门禁系统
- VC-030 ~ VC-035: HITL 检查点
- VC-036 ~ VC-039: MR Creation
- AI-ADAPTER-001 ~ AI-ADAPTER-005: AI 适配器集成

### 关键路径

```
✅ VC-001~VC-009 (基础组件) 
  → 📋 VC-010 (Initializer Agent 整合)
    → 📋 VC-019~VC-020 (代码生成和应用)
      → 📋 VC-024 (编辑器集成)
        → 📋 VC-027 (质量门禁)
          → 📋 VC-030~VC-031 (HITL 机制)
            → 📋 AI-ADAPTER-001~002 (AI 集成)
```


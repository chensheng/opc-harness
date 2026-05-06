# OPC-HARNESS

> AI 驱动的一人公司操作系统

OPC-HARNESS 是一个为独立创造者整合产品构思 (Vibe Design)、快速构建 (Vibe Coding)、增长运营 (Vibe Marketing) 三大模块的 AI 驱动桌面应用，实现从想法到产品运营的全流程闭环。

## ✨ 核心功能

### 🎨 Vibe Design - 产品构思

- 自然语言输入产品想法
- AI 生成产品需求文档 (PRD)
- 自动生成用户画像
- 竞品分析和差异化建议

### 💻 Vibe Coding - 快速构建

- 集成多种 AI 编码工具 CLI (Kimi/Claude/CodeFree)
- 代码编辑器和文件管理
- 实时预览功能
- 一键部署到 Vercel/Netlify

### 📈 Vibe Marketing - 增长运营

- AI 生成发布策略
- 多平台营销文案生成
- 发布时间线规划
- 推广渠道建议

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows: WebView2 Runtime + Visual Studio Build Tools

### 安装

```bash
# 克隆仓库
git clone https://github.com/chensheng/opc-harness.git
cd opc-harness

# 安装前端依赖
npm install

# 安装 Tauri CLI
cargo install tauri-cli
```

### 开发

```bash
# 启动开发服务器
npm run tauri:dev
```

### 构建

```bash
# 构建生产版本
npm run tauri:build
```

## 🔄 开发工作流

本项目采用 **OpenSpec** 实验性工作流管理所有变更,通过 Lingma 命令行工具实现结构化的开发流程。

### 安装 OpenSpec

OpenSpec 是一个独立的命令行工具,需要单独安装:

```bash
# 全局安装 OpenSpec CLI
npm install -g @fission-ai/openspec
```

验证安装:

```bash
# 检查 openspec 命令是否可用
openspec --version

# 查看当前项目的 OpenSpec 状态
openspec status
```

**注意**: 本项目已初始化 OpenSpec,包含 `openspec/` 目录和 `harness-quality` schema 配置。

### OpenSpec 核心命令

```bash
# 1. 创建变更提案
/opsx:propose <change-name>

# 2. 实施变更任务
/opsx:apply <change-name>

# 3. 归档完成的变更
/opsx:archive <change-name>
```

### 完整变更生命周期示例

```bash
# 步骤 1: 创建新特性提案
/opsx:propose add-user-authentication

# 系统自动生成:
# - proposal.md (为什么需要这个变更)
# - design.md (如何实现)
# - specs/ (能力规范)
# - tasks.md (实施任务列表)

# 步骤 2: 实施任务
/opsx:apply add-user-authentication

# AI Agent 自动执行 tasks.md 中的任务
# 每完成一个任务自动标记为 [x]

# 步骤 3: 质量验证
# 系统自动创建:
# - quality-check.md (Health Score 验证)
# - runtime-check.md (运行时验证)

# 步骤 4: 归档变更
/opsx:archive add-user-authentication

# 变更移动到 openspec/changes/archive/YYYY-MM-DD-add-user-authentication/
```

### 了解更多

- **[AGENTS.md](./AGENTS.md)** - 完整的 OpenSpec 导航和 18 个 capabilities 详解 ⭐
- **openspec/specs/** - 所有能力规范文档
- **openspec/changes/archive/** - 已归档的历史变更记录

## 🏗️ 技术架构

### 前端

- **框架**: React 18 + TypeScript 5
- **样式**: Tailwind CSS 3 + shadcn/ui
- **状态管理**: Zustand + Immer
- **路由**: React Router 6

### 后端 (Tauri v2)

- **语言**: Rust
- **数据库**: SQLite (rusqlite)
- **密钥存储**: OS Keychain (keyring-rs)
- **HTTP 客户端**: reqwest

### AI 集成

- **支持厂商**: OpenAI, Anthropic Claude, 月之暗面 Kimi, 智谱 GLM, CodeFree
- **CLI 工具**: Kimi CLI, Claude Code, CodeFree CLI
- **协议**: REST API + SSE (Server-Sent Events)

## 🤖 Harness Engineering

本项目实现了基于 **Harness Engineering** 理念的 AI 协作开发环境，让 AI Agent 能够更高效地完成编码任务。

### 核心理念

- **AI-Agent 协作** - 为 AI Agent 提供清晰的架构约束和上下文信息
- **OpenSpec 工作流** - 通过结构化的变更管理流程，确保每次变更都有完整的提案、设计和验证 ⭐
- **自动化验证** - 通过智能脚本自动检测代码质量和架构健康度
- **渐进式检查** - 从基础类型检查到完整架构验证，分层保障代码质量
- **文档驱动** - 防止“注释漂移”，确保代码与文档同步演进（通过 OpenSpec specs）

### 核心文档

- **[AGENTS.md](./AGENTS.md)** - AI Agent 导航地图（必读）⭐
- **openspec/specs/** - 18 个能力规范文档，定义系统的核心行为
- **openspec/changes/archive/** - 已归档的历史变更记录，可追溯所有决策

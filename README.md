# OPC-HARNESS

> AI 驱动的一人公司操作系统

OPC-HARNESS 是一个为独立创造者整合产品构思 (Vibe Design)、快速构建 (Vibe Coding)、增长运营 (Vibe Marketing) 三大模块的 AI 驱动桌面应用，实现从想法到产品运营的全流程闭环。

## 📁 数据存储

### 工作区目录结构

OPC-HARNESS 采用业界标准做法（参考 OpenClaw、Claude Code），在用户 home 目录下创建 `.opc-harness` 隐藏目录来存储所有应用数据：

```
~/.opc-harness/                    # 应用数据根目录
├── opc-harness.db                 # SQLite 数据库
├── config/                        # 配置文件
├── logs/                          # 日志文件
├── cache/                         # 缓存文件
├── sessions/                      # 会话数据
└── workspaces/                    # 项目工作区 ⭐
    ├── {project-uuid-1}/          # 项目1的代码目录（使用UUID命名）
    ├── {project-uuid-2}/          # 项目2的代码目录（使用UUID命名）
    └── ...
```

**项目工作区特性**：
- ✅ 自动创建：创建新项目时自动生成 `~/.opc-harness/workspaces/{project_id}` 目录（使用项目UUID作为目录名）
- ✅ 启动检查：应用启动时自动检查并修复缺失的工作区目录
- ✅ 唯一性保证：使用UUID确保目录名全局唯一，避免冲突
- ✅ 跨平台兼容：支持 Windows、macOS、Linux

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
- **自动化验证** - 通过智能脚本自动检测代码质量和架构健康度
- **渐进式检查** - 从基础类型检查到完整架构验证，分层保障代码质量
- **文档驱动** - 防止"注释漂移"，确保代码与文档同步演进

### 核心文档

- **[AGENTS.md](./AGENTS.md)** - AI Agent 导航地图（必读）⭐

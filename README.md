# OPC-HARNESS

> AI驱动的一人公司操作系统 - MVP版本

## 项目简介

OPC-HARNESS是一款桌面应用，帮助独立创造者通过三大模块实现从想法到产品的全流程闭环：

- **Vibe Design**: AI驱动的产品构思，生成PRD、用户画像、竞品分析
- **Vibe Coding**: 集成AI编码工具CLI（Kimi/Claude/Codex），快速构建产品
- **Vibe Marketing**: AI辅助的增长运营，生成发布策略和营销文案

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React 18 + TypeScript 5 + Tailwind CSS 3 |
| 状态管理 | Zustand 4 |
| 编辑器 | Monaco Editor |
| 桌面框架 | Tauri v2 (Rust) |
| 数据库 | SQLite (rusqlite) |
| 构建工具 | Vite 5 |

## 快速开始

### 环境要求

- **Node.js**: >= 18.0.0
- **Rust**: >= 1.70.0 ([安装指南](docs/Rust安装与启动指南.md))

### 启动开发服务器

```bash
# 1. 安装前端依赖
npm install

# 2. 启动 Tauri 开发服务器
npm run tauri:dev
```

> 首次编译需要 5-15 分钟，请耐心等待。

### 构建发布版本

```bash
npm run tauri:build
```

构建输出：
- Windows: `src-tauri/target/release/bundle/msi/`
- macOS: `src-tauri/target/release/bundle/dmg/`

### 使用 PowerShell 脚本（可选）

```powershell
# 检查环境
.\scripts\setup-and-run.ps1 -Check

# 启动开发服务器
.\scripts\setup-and-run.ps1 -Dev

# 构建发布版本
.\scripts\setup-and-run.ps1 -Build
```

## 项目结构

```
opc-harness/
├── src/                    # 前端源码 (React + TS)
│   ├── components/         # 组件
│   ├── stores/             # Zustand 状态管理
│   └── ...
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── models/         # 数据模型
│   │   └── services/       # 业务服务
│   └── ...
└── scripts/                # 工具脚本
```

## 功能模块

### Vibe Design
- [x] AI厂商配置管理（OpenAI/Claude/Kimi/GLM）
- [x] API密钥安全存储
- [x] 产品想法输入
- [x] PRD自动生成
- [x] 用户画像生成
- [x] 竞品分析

### Vibe Coding
- [x] CLI工具集成（Kimi/Claude/Codex）
- [x] 终端控制台界面
- [x] Monaco代码编辑器
- [x] 文件树浏览
- [x] 实时预览

### Vibe Marketing
- [x] 发布策略生成
- [x] 营销文案生成
- [x] 多平台文案支持

## 配置说明

### AI厂商API配置

| 厂商 | 密钥获取地址 |
|------|-------------|
| OpenAI | https://platform.openai.com/api-keys |
| Anthropic | https://console.anthropic.com/settings/keys |
| 月之暗面(Kimi) | https://platform.moonshot.cn/console/api-keys |
| 智谱AI(GLM) | https://open.bigmodel.cn/usercenter/apikeys |

### CLI工具安装（可选）

```bash
# Kimi CLI
npm install -g @moonshotai/kimi-cli

# Claude Code
npm install -g @anthropic-ai/claude-code

# Codex CLI
npm install -g @openai/codex
```

## 开发文档

- [MVP开发任务拆解](docs/MVP版本开发任务拆解.md)
- [Rust安装与启动指南](docs/Rust安装与启动指南.md)

## License

MIT License

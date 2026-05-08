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
- **Native Coding Agent**: 纯 Rust 实现的智能编码代理
  - 🔍 代码搜索工具：grep、find_files、find_symbol
  - 📦 依赖管理：npm install、cargo add
  - 👥 HITL Checkpoint：人工审核关键决策点
  - 🌿 Worktree 自动清理：防止磁盘空间泄漏
  - 💬 对话历史优化：自动压缩，节省 Token
  - ✅ 分阶段质量检查：lint → type-check → test

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

## 📚 了解更多

- **[AGENTS.md](./AGENTS.md)** - 完整的开发工作流、技术架构和 AI Agent 导航 ⭐
- **openspec/specs/** - OpenSpec 能力规范文档
- **openspec/changes/archive/** - 历史变更记录

# OPC-HARNESS

> AI 驱动的一人公司操作系统

OPC-HARNESS 是一个为独立创造者整合产品构思 (Vibe Design)、快速构建 (Vibe Coding)、增长运营 (Vibe Marketing) 三大模块的 AI 驱动桌面应用，实现从想法到产品的全流程闭环。

## ✨ 核心功能

### 🎨 Vibe Design - 产品构思
- 自然语言输入产品想法
- AI 生成产品需求文档 (PRD)
- 自动生成用户画像
- 竞品分析和差异化建议

### 💻 Vibe Coding - 快速构建
- 集成多种 AI编码工具 CLI (Kimi/Claude/Codex)
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
- **支持厂商**: OpenAI, Anthropic Claude, 月之暗面 Kimi, 智谱 GLM
- **协议**: REST API + SSE (Server-Sent Events)

## 🤖 Harness Engineering

本项目实现了基于 **Harness Engineering** 理念的 AI 协作开发环境，让 AI Agent 能够更高效地完成编码任务。

### 核心组件

- **[AGENTS.md](./AGENTS.md)** - AI Agent 导航地图（必读）
- **[.harness/](./.harness/)** - Harness Engineering 配置目录
  - **[README.md](./.harness/README.md)** - Harness 使用指南
  - **[constraints/](./.harness/constraints/)** - 架构约束规则
  - **[context-engineering/](./.harness/context-engineering/)** - 上下文工程数据
    - `decision-records/` - 架构决策记录 (ADRs)
    - `execution-logs/` - 执行日志模板
    - `knowledge-base/` - 知识库和最佳实践
  - **[scripts/](./scripts/)** - 自动化脚本和 CLI 浏览器验证

### 快速命令

```bash
# 架构健康检查（主入口）⭐
npm run harness:check       # 基础检查（TypeScript、ESLint、Prettier、Rust、依赖、目录）

# 扩展检查（可选）
npm run harness:check -- -DocCheck    # + 文档一致性检查
npm run harness:check -- -DeadCode    # + 死代码检测
npm run harness:check -- -All         # 运行所有检查（推荐提交前使用）

# 测试
npm run test:unit           # 单元测试 ⭐
npm run test:e2e            # E2E 测试（智能运行，自动管理服务器）⭐

# 覆盖率报告（按需使用）
npx vitest run --coverage

# 代码质量
npm run lint                # ESLint 检查
npm run lint:fix            # 自动修复
npm run format              # Prettier 格式化
npm run format:check        # 检查格式
```

### 📚 测试文档

- **[docs/testing/](./docs/testing/)** - 测试体系文档中心
  - [README.md](./docs/testing/README.md) - 5 分钟快速开始
  - [E2E-STRATEGY.md](./docs/testing/E2E-STRATEGY.md) - E2E 测试方案说明
  - [testing-full.md](./docs/testing/testing-full.md) - 完整测试指南
  - [testing-validation.md](./docs/testing/testing-validation.md) - 安装验证清单

### AI Agent 工作流程

1. **阅读导航**: AI Agent 首先阅读 [AGENTS.md](./AGENTS.md) 了解项目结构
2. **查看约束**: 参考 [architecture-rules.md](./.harness/constraints/architecture-rules.md) 确保符合规范
3. **学习最佳实践**: 查阅 [best-practices.md](./.harness/context-engineering/knowledge-base/best-practices.md)
4. **生成代码**: 基于上下文和约束生成代码
5. **验证质量**: 运行 `npm run harness:check` 验证代码质量
6. **记录决策**: 如有架构变更，编写 [ADR](./.harness/context-engineering/decision-records/)

详细使用指南请参考：[.harness/README.md](./.harness/README.md)

## 📁 项目结构

```
opc-harness/
├── AGENTS.md                 # AI Agent 导航地图
├── src/                      # 前端代码
│   ├── components/           # React 组件
│   │   ├── ui/              # shadcn/ui 组件
│   │   ├── vibe-design/     # Vibe Design 模块
│   │   ├── vibe-coding/     # Vibe Coding 模块
│   │   ├── vibe-marketing/  # Vibe Marketing 模块
│   │   └── common/          # 通用组件
│   ├── stores/              # Zustand 状态管理
│   ├── types/               # TypeScript 类型定义
│   └── lib/                 # 工具函数
├── src-tauri/               # Tauri 后端代码
│   └── src/
│       ├── commands/        # Tauri 命令
│       ├── ai/              # AI Provider 适配器
│       ├── cli/             # CLI工具集成
│       ├── db/              # 数据库模块
│       ├── services/        # 业务逻辑
│       └── models/          # 数据模型
├── .harness/                # Harness Engineering 配置
│   ├── constraints/         # 架构约束
│   ├── context-engineering/ # 上下文工程
│   └── scripts/             # 自动化脚本
└── package.json
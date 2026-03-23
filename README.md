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

### 核心理念

- **AI-Agent 协作** - 为 AI Agent 提供清晰的架构约束和上下文信息
- **自动化验证** - 通过智能脚本自动检测代码质量和架构健康度
- **渐进式检查** - 从基础类型检查到完整架构验证，分层保障代码质量
- **文档驱动** - 防止"注释漂移"，确保代码与文档同步演进

### 核心组件

- **[AGENTS.md](./AGENTS.md)** - AI Agent 导航地图（必读）⭐
- **[scripts/](./scripts/)** - Harness Engineering 自动化脚本
  - **[README.md](./scripts/README.md)** - 脚本使用说明
  - **harness-check.ps1** - 架构健康检查（主入口）
  - **harness-doc-check.ps1** - 文档一致性检查
  - **harness-dead-code.ps1** - 死代码检测
  - **harness-e2e.ps1** - E2E 测试运行器
  - **cli-browser-verify/** - CLI 浏览器验证工具
- **[docs/](./docs/)** - 文档和知识库中心
  - **[testing/](./docs/testing/)** - 测试体系文档
    - [README.md](./docs/testing/README.md) - 测试导航
    - [COMMANDS-REFERENCE.md](./docs/testing/COMMANDS-REFERENCE.md) - 命令参考
    - [HARNESS-COMMANDS.md](./docs/testing/HARNESS-COMMANDS.md) - Harness 命令说明
    - [HARNESS-STRUCTURE.md](./docs/testing/HARNESS-STRUCTURE.md) - 目录结构说明
  - **[references/](./docs/references/)** - 参考资料
    - [architecture-rules.json](./docs/references/architecture-rules.json) - 架构规则配置
    - [best-practices.md](./docs/references/best-practices.md) - 最佳实践

### 快速命令

#### Harness Engineering ⭐

```bash
# 架构健康检查（主入口）
npm run harness:check                    # 基础检查（6 项）
npm run harness:check -- -DocCheck       # + 文档一致性检查
npm run harness:check -- -DeadCode       # + 死代码检测
npm run harness:check -- -All            # 完整检查（推荐提交前使用）⭐
```

#### 测试套件

```bash
# 单元测试
npm run test:unit                        # 运行所有单元测试 ⭐

# E2E 测试
npm run test:e2e                         # E2E 测试（智能运行，自动管理服务器）⭐

# 按需使用（不常用）
npx vitest run --coverage                # 生成覆盖率报告 📊
npx vitest --ui                          # UI 界面 🔍
npx vitest                               # 监视模式（开发时用）💻
```

#### 代码质量

```bash
npm run lint                             # ESLint 检查
npm run lint:fix                         # 自动修复
npm run format                           # Prettier 格式化
npm run format:check                     # 检查格式
```

### 🎯 常用工作流

#### 日常开发
```bash
# 1. 运行单元测试
npm run test:unit

# 2. 代码修改后运行架构检查
npm run harness:check
```

#### 提交前验证 ⭐
```bash
# 方式一：分步执行
npm run test:unit
npm run test:e2e
npm run harness:check

# 方式二：一站式完整检查（推荐）
npm run harness:check -- -All && npm run test:unit && npm run test:e2e
```

#### 定期维护
```bash
# 每周运行一次完整检查
npm run harness:check -- -All

# 按需生成覆盖率报告
npx vitest run --coverage
```

### 📚 测试文档

- **[docs/testing/](./docs/testing/)** - 测试体系文档中心
  - [README.md](./docs/testing/README.md) - 5 分钟快速开始
  - [COMMANDS-REFERENCE.md](./docs/testing/COMMANDS-REFERENCE.md) - 完整命令参考
  - [HARNESS-COMMANDS.md](./docs/testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
  - [HARNESS-COMMANDS-UPDATE.md](./docs/testing/HARNESS-COMMANDS-UPDATE.md) - 命令更新说明
  - [HARNESS-STRUCTURE.md](./docs/testing/HARNESS-STRUCTURE.md) - 目录结构说明
  - [E2E-STRATEGY.md](./docs/testing/E2E-STRATEGY.md) - E2E 测试方案说明
  - [testing-full.md](./docs/testing/testing-full.md) - 完整测试指南
  - [testing-validation.md](./docs/testing/testing-validation.md) - 安装验证清单
  - [RUN-E2E-AUTO.md](./docs/testing/RUN-E2E-AUTO.md) - E2E 自动运行指南

### 📚 Harness Engineering 文档

- **[docs/references/](./docs/references/)** - 参考资料库
  - [architecture-rules.json](./docs/references/architecture-rules.json) - 架构规则配置
  - [best-practices.md](./docs/references/best-practices.md) - 最佳实践指南
  - [harness-user-guide.md](./docs/references/harness-user-guide.md) - Harness 使用指南
  - [harness-quickstart.md](./docs/references/harness-quickstart.md) - 快速入门
  - [index.md](./docs/references/index.md) - 文档索引

### AI Agent 工作流程

1. **阅读导航**: AI Agent 首先阅读 [AGENTS.md](./AGENTS.md) 了解项目结构 ⭐
2. **查看约束**: 参考 [architecture-rules.json](./docs/references/architecture-rules.json) 确保符合规范
3. **学习最佳实践**: 查阅 [best-practices.md](./docs/references/best-practices.md)
4. **生成代码**: 基于上下文和约束生成代码
5. **验证质量**: 运行 `npm run harness:check` 或 `npm run harness:check -- -All` 验证代码质量
6. **运行测试**: 执行 `npm run test:unit` 和 `npm run test:e2e` 确保功能正常
7. **记录决策**: 如有架构变更，更新相关文档

详细使用指南请参考：
- [scripts/README.md](./scripts/README.md) - 自动化脚本说明
- [docs/testing/README.md](./docs/testing/README.md) - 测试体系导航
- [docs/references/harness-user-guide.md](./docs/references/harness-user-guide.md) - Harness 使用指南

## 📁 项目结构

```
opc-harness/
├── AGENTS.md                 # AI Agent 导航地图
├── ARCHITECTURE.md           # 架构设计文档
├── IMPLEMENTATION.md         # 实现说明
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
├── scripts/                 # Harness Engineering 自动化脚本
│   ├── harness-check.ps1           # 架构健康检查
│   ├── harness-doc-check.ps1       # 文档一致性检查
│   ├── harness-dead-code.ps1       # 死代码检测
│   ├── harness-e2e.ps1             # E2E 测试运行器
│   └── cli-browser-verify/         # CLI 浏览器验证工具
├── docs/                    # 文档中心
│   └── testing/             # 测试体系文档
└── package.json
```


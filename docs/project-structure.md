# OPC-HARNESS 项目目录结构

## 概述

本项目采用 Tauri v2 + React + TypeScript + Rust 架构，遵循模块化和分层设计原则。

## 前端目录结构 (src/)

```
src/
├── components/              # React 组件
│   ├── ui/                 # shadcn/ui 基础组件
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── input.tsx
│   │   ├── dialog.tsx
│   │   └── ...
│   ├── vibe-design/        # Vibe Design 模块组件
│   ├── vibe-coding/        # Vibe Coding 模块组件
│   ├── vibe-marketing/     # Vibe Marketing 模块组件
│   └── common/             # 通用组件
│
├── hooks/                  # 自定义 React Hooks
│   └── useTheme.ts
│
├── lib/                    # 工具库
│   └── utils.ts           # cn() 等工具函数
│
├── services/               # API 服务层
│   └── api.ts             # Tauri 命令调用
│
├── stores/                 # Zustand 状态管理
│   ├── index.ts
│   ├── appStore.ts
│   ├── projectStore.ts
│   └── aiConfigStore.ts
│
├── styles/                 # 样式文件
│
├── types/                  # TypeScript 类型定义
│   └── index.ts
│
├── utils/                  # 工具函数
│   └── format.ts
│
├── App.tsx                 # 根组件
├── main.tsx               # 入口文件
└── index.css              # 全局样式
```

## 后端目录结构 (src-tauri/src/)

```
src-tauri/src/
├── commands/              # Tauri 命令处理器
│   ├── mod.rs
│   ├── project.rs
│   ├── ai.rs
│   ├── cli.rs
│   └── system.rs
│
├── models/                # 数据模型
│   └── mod.rs
│
├── services/              # 业务逻辑层
│   ├── mod.rs
│   ├── project_service.rs
│   ├── ai_service.rs
│   ├── cli_service.rs
│   └── tool_detection.rs
│
├── db/                    # 数据库模块
│   ├── mod.rs
│   └── migrations.rs
│
├── ai/                    # AI Provider 适配器
│   ├── mod.rs
│   ├── openai.rs
│   ├── anthropic.rs
│   ├── kimi.rs
│   └── glm.rs
│
├── cli/                   # CLI 工具集成
│   ├── mod.rs
│   ├── kimi.rs
│   ├── claude.rs
│   └── codex.rs
│
├── utils/                 # 工具函数
│   └── mod.rs
│
└── main.rs               # 程序入口
```

## 配置文件

### 前端配置

- `tsconfig.json` - TypeScript 配置
- `vite.config.ts` - Vite 配置
- `tailwind.config.js` - Tailwind CSS 配置
- `postcss.config.js` - PostCSS 配置
- `components.json` - shadcn/ui 配置
- `eslint.config.js` - ESLint 配置
- `.prettierrc` - Prettier 配置

### 后端配置

- `src-tauri/Cargo.toml` - Rust 依赖配置
- `src-tauri/tauri.conf.json` - Tauri 配置

### 开发工具配置

- `.husky/pre-commit` - Git pre-commit hook
- `.husky/commit-msg` - Git commit-msg hook
- `lint-staged` 配置在 `package.json` 中

## 模块说明

### Vibe Design (产品构思)

- 想法输入与 PRD 生成
- 用户画像生成
- 竞品分析

### Vibe Coding (快速构建)

- CLI 工具集成 (Kimi/Claude/Codex)
- 代码编辑器
- 本地预览

### Vibe Marketing (增长运营)

- 发布策略生成
- 营销文案生成

## 命名规范

- 组件文件: PascalCase (e.g., `Button.tsx`)
- 工具文件: camelCase (e.g., `format.ts`)
- 样式文件: kebab-case (e.g., `index.css`)
- Rust 模块: snake_case (e.g., `app_state.rs`)

## 导入路径别名

- `@/` - 指向 `src/` 目录
- 示例: `import { Button } from '@/components/ui/button'`

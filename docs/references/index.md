# 参考资料索引

> 本目录收集外部参考文档、最佳实践和技术资源
> 
> **用途**: 为开发者和 AI Agent 提供统一的技术参考和规范指南
> 
> **维护者**: OPC-HARNESS Team  
> **最后更新**: 2026-03-23  
> **状态**: 🟢 已优化

---

## 📑 快速导航

| 类别 | 核心文档 | 重要性 |
|------|---------|--------|
| **Harness Engineering** | [best-practices.md](./best-practices.md), [architecture-rules.md](./architecture-rules.md) | ⭐⭐⭐ |
| **产品规格** | [产品设计.md](./产品设计.md), [架构设计.md](./架构设计.md) | ⭐⭐⭐ |
| **技术栈** | [Tauri v2](https://v2.tauri.app/), [React 18](https://react.dev/) | ⭐⭐ |
| **代码规范** | [AGENTS.md](../../AGENTS.md), [src/AGENTS.md](../../src/AGENTS.md) | ⭐⭐⭐ |

---

## 📚 核心参考文档

### Harness Engineering ⭐

> Harness Engineering 是一套让 AI Agent 更好地协助你开发项目的工程实践体系
> 
> **核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)

| 文档 | 描述 | 适用场景 |
|------|------|---------|
| [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/) | OpenAI 官方最佳实践 | 了解 Harness 理念 |
| [best-practices.md](./best-practices.md) | 编码最佳实践 🔥 | 日常开发参考 |
| ~~[architecture-rules.md](./architecture-rules.md)~~ | ❌ 已移至 [`design-docs/architecture-rules.md`](../design-docs/architecture-rules.md) | - |
| ~~harness-user-guide.md~~ | ❌ 文件不存在 | - |
| ~~harness-quickstart.md~~ | ❌ 文件不存在 | - |
| ~~harness-engineering-process.md~~ | ❌ 文件不存在 | - |

**注意**: 部分文档链接已失效，建议补充或删除。**architecture-rules.md** 已移至 [`docs/design-docs/`](../design-docs/)。

### 产品与技术规格

| 文档 | 描述 | 版本 |
|------|------|------|
| [产品设计.md](./产品设计.md) | 完整产品需求文档 (PRD) | v1.1 |
| [架构设计.md](./架构设计.md) | 系统架构设计文档 | v3.0 |
| [symphony.md](./symphony.md) | Symphony 产品设计文档 | v2.0 |
| [autonomous-coding-harness.md](./autonomous-coding-harness.md) | Coding Harness PRD | v1.1 |

---

## 🛠️ 技术栈文档

### 前端技术栈

| 技术 | 官方文档 | 用途 | 项目路径 |
|------|---------|------|---------|
| **React 18** | [react.dev](https://react.dev/) | UI 框架 | `src/` |
| **TypeScript 5** | [typescriptlang.org](https://www.typescriptlang.org/docs/) | 类型系统 | 全局 |
| **Tailwind CSS** | [tailwindcss.com](https://tailwindcss.com/) | 样式框架 | 全局 |
| **Zustand** | [zustand-demo.pmnd.rs](https://zustand-demo.pmnd.rs/) | 状态管理 | `src/stores/` |
| **Vite** | [vitejs.dev](https://vitejs.dev/) | 构建工具 | 构建配置 |

### 后端技术栈

| 技术 | 官方文档 | 用途 | 项目路径 |
|------|---------|------|---------|
| **Rust** | [rust-lang.org](https://www.rust-lang.org/learn) | 后端语言 | `src-tauri/` |
| **Tauri v2** | [v2.tauri.app](https://v2.tauri.app/) | 桌面框架 | 全局 |
| **tokio** | [tokio.rs](https://tokio.rs/) | 异步运行时 | `src-tauri/` |
| **rusqlite** | [docs.rs/rusqlite](https://docs.rs/rusqlite/) | SQLite 绑定 | `src-tauri/db/` |
| **serde** | [serde.rs](https://serde.rs/) | 序列化库 | `src-tauri/` |

### UI 组件库

| 组件库 | 官方文档 | 用途 |
|-------|---------|------|
| **shadcn/ui** | [ui.shadcn.com](https://ui.shadcn.com/) | 基础组件库 |
| **Radix UI** | [radix-ui.com](https://www.radix-ui.com/) | 无头组件 |
| **Lucide Icons** | [lucide.dev](https://lucide.dev/) | 图标库 |

---

## 📖 项目规范

### 代码规范

| 文档 | 适用范围 | 重要性 |
|------|---------|--------|
| [AGENTS.md](../../AGENTS.md) | 全体开发者 & AI Agent | ⭐⭐⭐ |
| [src/AGENTS.md](../../src/AGENTS.md) | 前端开发者 | ⭐⭐⭐ |
| [src-tauri/AGENTS.md](../../src-tauri/AGENTS.md) | Rust 开发者 | ⭐⭐⭐ |
| [ARCHITECTURE.md](../../ARCHITECTURE.md) | 架构师 & Tech Lead | ⭐⭐⭐ |

### 架构约束

- **[architecture-rules.md](./architecture-rules.md)** - 前后端架构约束规则定义 ⭐
  - 前端规则：FE-ARCH-001 ~ FE-ARCH-005
  - 后端规则：BE-ARCH-001 ~ BE-ARCH-XXX
  - 执行方式：ESLint + Clippy 自动检查

### 测试策略

| 测试类型 | 命令 | 覆盖范围 |
|---------|------|---------|
| **单元测试** | `npm run test:unit` / `cargo test` | 所有功能模块 |
| **E2E 测试** | `npm run test:e2e` | 关键用户流程 |
| **架构检查** | `npm run harness:check` | 架构约束合规性 |
| **类型检查** | `npx tsc --noEmit` / `cargo check` | 类型安全 |

---

## 🔍 学习资源

### 入门教程

| 教程 | 难度 | 预计时间 |
|------|------|---------|
| [Tauri 入门教程](https://v2.tauri.app/start/) | ⭐⭐ | 2 小时 |
| [Rust 编程之旅](https://tour.rust-lang.org/) | ⭐⭐⭐ | 4 小时 |
| [React 逐步教程](https://react.dev/learn/tutorial-tic-tac-toe) | ⭐⭐ | 3 小时 |

### 视频课程

- [Tauri v2 完整教程](https://www.youtube.com/results?search_query=tauri+v2+tutorial)
- [Rust 系统设计](https://www.youtube.com/results?search_query=rust+system+design)

### 进阶阅读

- [Context Engineering 最佳实践](https://openai.com/index/context-engineering/)
- [Human-in-the-Loop AI 设计模式](https://arxiv.org/abs/2102.09347)

---

## 📝 文档维护指南

### 添加新文档

1. 将文档放入 `docs/references/` 目录
2. 更新本索引的相应分类
3. 在文档顶部添加元数据（版本、日期、作者）

### 文档更新规范

```
# 文档标题

> **版本**: v1.0.0
> **最后更新**: YYYY-MM-DD
> **适用范围**: XXX
> **状态**: 🟢 已优化 / 🟡 草稿 / 🔴 待更新
```

### 文档质量检查

- [ ] 链接是否有效
- [ ] 版本信息是否最新
- [ ] 是否有清晰的目录结构
- [ ] 是否包含必要的示例代码

---

## 🔗 相关资源

- **项目根目录**: [`../../`](../../)
- **源代码**: [`../../src/`](../../src/), [`../../src-tauri/`](../../src-tauri/)
- **执行计划**: [`../exec-plans/`](../exec-plans/)
- **设计文档**: [`../design-docs/`](../design-docs/)

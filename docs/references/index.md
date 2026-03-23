# 参考资料索引

> 本目录收集外部参考文档、最佳实践和技术资源

## 📚 核心参考

### Harness Engineering
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/) - OpenAI官方最佳实践
- [Harness Engineering 中文解读](https://mparticle.uc.cn/article.html?uc_param_str=frdnsnpfvecpntnwprdssskt) - 详细中文解析

### Harness 工具文档 ⭐️ NEW
- [`harness-user-guide.md`](./harness-user-guide.md) - Harness Engineering 完整使用指南
- [`harness-quickstart.md`](./harness-quickstart.md) - 30 秒快速入门 Harness

### 技术栈文档
- [Tauri v2 官方文档](https://v2.tauri.app/)
- [React 18 官方文档](https://react.dev/)
- [TypeScript 5 手册](https://www.typescriptlang.org/docs/)
- [Rust 编程语言](https://www.rust-lang.org/learn)
- [Zustand 状态管理](https://zustand-demo.pmnd.rs/)

## 🛠️ 工具和库

### UI 组件
- [shadcn/ui](https://ui.shadcn.com/) - 基础组件库
- [Radix UI](https://www.radix-ui.com/) - 无头组件
- [Lucide Icons](https://lucide.dev/) - 图标库

### Rust 生态
- [serde 序列化](https://serde.rs/)
- [rusqlite](https://docs.rs/rusqlite/)
- [tokio 异步运行时](https://tokio.rs/)
- [tauri-plugin-log](https://docs.rs/tauri-plugin-log/)

### 开发工具
- [Vite 构建工具](https://vitejs.dev/)
- [ESLint](https://eslint.org/)
- [Prettier](https://prettier.io/)
- [Cargo](https://doc.rust-lang.org/cargo/)

## 📖 最佳实践

### 代码规范
- [`../../src/AGENTS.md`](../../src/AGENTS.md) - 前端开发指南
- [`../../src-tauri/AGENTS.md`](../../src-tauri/AGENTS.md) - Rust 开发指南
- [`../design-docs/architecture-patterns.md`](../design-docs/architecture-patterns.md) - 架构模式

### 架构约束规则
- [`architecture-rules.json`](./architecture-rules.json) ⭐️ NEW - 前后端架构约束规则定义

### 测试策略
- 单元测试：`cargo test` / `vitest`
- 集成测试：Tauri E2E
- CLI 浏览器验证：`npm run harness:verify:cli`

### 性能优化
- 代码分割：Vite 动态导入
- 懒加载：React.lazy + Suspense
- Rust 优化：Release 模式编译

## 🔍 学习资源

### 教程
- [Tauri 入门教程](https://v2.tauri.app/start/)
- [Rust 编程之旅](https://tour.rust-lang.org/)
- [React 逐步教程](https://react.dev/learn/tutorial-tic-tac-toe)

### 视频课程
- [Tauri v2 完整教程](https://www.youtube.com/results?search_query=tauri+v2+tutorial)
- [Rust 系统设计](https://www.youtube.com/results?search_query=rust+system+design)

## 📝 团队知识

### 内部文档
- [`best-practices.md`](./best-practices.md) - 团队最佳实践集合
- [`common-pitfalls.md`](./common-pitfalls.md) - 常见陷阱和解决方案
- [`onboarding-guide.md`](./onboarding-guide.md) - 新成员入职指南

### 决策记录
- 查看：[`../design-docs/decision-records/`](../design-docs/decision-records/)

### 执行日志
- 查看：[`../exec-plans/completed/`](../exec-plans/completed/)

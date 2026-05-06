# 参考资料索引

> 本目录收集外部参考文档、最佳实践和技术资源
>
> **用途**: 为开发者和 AI Agent 提供统一的技术参考和规范指南
>
> **维护者**: OPC-HARNESS Team  
> **最后更新**: 2026-05-06  
> **状态**: 🟢 已优化

---

## 📑 快速导航

| 类别                    | 核心文档                                                                                     | 重要性 |
| ----------------------- | -------------------------------------------------------------------------------------------- | ------ |
| **Harness Engineering** | [best-practices.md](./best-practices.md), [architecture-rules.md](./architecture-rules.md)   | ⭐⭐⭐ |
| **OpenSpec 工作流**     | [openspec-harness-integration.md](./openspec-harness-integration.md)                         | ⭐⭐⭐ |
| **产品规格**            | [product-design.md](./product-design.md), [architecture-design.md](./architecture-design.md) | ⭐⭐⭐ |
| **技术栈**              | [Tauri v2](https://v2.tauri.app/), [React 18](https://react.dev/)                            | ⭐⭐   |
| **代码规范**            | [AGENTS.md](../../AGENTS.md), [src/AGENTS.md](../../src/AGENTS.md)                           | ⭐⭐⭐ |

---

## 📚 核心参考文档

### Harness Engineering ⭐

> Harness Engineering 是一套让 AI Agent 更好地协助你开发项目的工程实践体系
>
> **核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)

| 文档                                                                        | 描述                                                                                  | 适用场景          |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ----------------- |
| [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/) | OpenAI 官方最佳实践                                                                   | 了解 Harness 理念 |
| [best-practices.md](./best-practices.md)                                    | 编码最佳实践 🔥                                                                       | 日常开发参考      |
| ~~[architecture-rules.md](./architecture-rules.md)~~                        | ❌ 已移至 [`design-docs/architecture-rules.md`](../design-docs/architecture-rules.md) | -                 |
| ~~harness-user-guide.md~~                                                   | ❌ 文件不存在                                                                         | -                 |
| ~~harness-quickstart.md~~                                                   | ❌ 文件不存在                                                                         | -                 |
| ~~harness-engineering-process.md~~                                          | ❌ 文件不存在                                                                         | -                 |

**注意**: 部分文档链接已失效，建议补充或删除。**architecture-rules.md** 已移至 [`docs/design-docs/`](../design-docs/)。

### OpenSpec 工作流 ⭐

> OpenSpec 是 AI 驱动的结构化开发工作流，通过自定义 schema 集成 Harness Engineering 质量门禁

| 文档 | 描述 | 版本 |
|------|------|------|
| [openspec-harness-integration.md](./openspec-harness-integration.md) | OpenSpec + Harness:check 完整集成方案 | v2.0 |
| [openspec-harness-quality-schema.md](./openspec-harness-quality-schema.md) | harness-quality schema 完整使用指南 | v2.0 |
| [openspec-harness-quality-quickstart.md](./openspec-harness-quality-quickstart.md) | 快速参考指南 | v2.0 |
| [openspec-harness-quality-changelog.md](./openspec-harness-quality-changelog.md) | Schema 版本更新日志 | v2.0 |

### 产品与技术规格

| 文档                                                           | 描述                                                                                    | 版本 |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ---- |
| ~~[产品设计.md](./产品设计.md)~~                               | ❌ 已移至 [`product-specs/产品设计.md`](../product-specs/产品设计.md)                   | -    |
| ~~[架构设计.md](./架构设计.md)~~                               | ❌ 已移至 [`design-docs/system-architecture.md`](../design-docs/system-architecture.md) | -    |
| [symphony.md](./symphony.md)                                   | Symphony产品设计文档                                                                    | v2.0 |
| [autonomous-coding-harness.md](./autonomous-coding-harness.md) | Coding Harness PRD                                                                      | v1.1 |

## Why

当前 AGENTS.md 文件过长（559 行），包含大量 OpenSpec 工作流详细说明、最佳实践、常见问题等内容，导致：

1. **信息过载** - AI Agent 和开发者难以快速找到关键信息
2. **重复内容** - 许多细节已在 OpenSpec specs 中详细说明
3. **维护负担** - 过长的文档需要同步更新，增加维护成本
4. **导航困难** - 关键信息被淹没在大量细节中

通过精简 AGENTS.md，保留核心导航和快速入口，将详细内容引导至专门的 OpenSpec specs，可以提升文档的可读性和可维护性。

## What Changes

- **保留快速入口章节** - 按优先级列出的核心文档链接
- **保留三大支柱概述** - 上下文工程、架构约束、反馈回路的简要说明
- **删除 OpenSpec 快速入门详细教程** - 移至专门的 OpenSpec 文档或保持简洁引用
- **删除 OpenSpec 最佳实践长篇说明** - 移至 openspec/specs/development-workflow/spec.md
- **删除常见问题 FAQ 章节** - 移至专门的 FAQ 文档或简化为链接
- **精简三大支柱详细描述** - 保留核心要点，删除冗余解释
- **添加文档导航** - 在末尾添加"详细文档"链接，指向 OpenSpec specs

## Capabilities

### New Capabilities

<!-- 此变更不涉及新增能力 -->

### Modified Capabilities

- `agent-initialization`: 简化 AGENTS.md 中的 Agent 初始化指引，将详细内容迁移至专门文档
- `development-workflow`: 简化 README 和 AGENTS.md 中的工作流说明，将详细内容统一至 openspec/specs/development-workflow/spec.md

## Impact

**受影响的文件**:
- `AGENTS.md` - 从 559 行精简至约 150-200 行

**受影响的系统**:
- 无代码变更，纯文档优化
- AI Agent 和开发者通过 AGENTS.md 快速导航后，可通过链接跳转至详细文档

**依赖影响**:
- 无

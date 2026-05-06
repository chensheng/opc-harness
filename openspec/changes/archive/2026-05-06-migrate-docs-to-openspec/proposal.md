## Why

当前项目的文档分散在 `docs/` 目录下,采用传统的文档组织结构(design-docs、product-specs、exec-plans、sprint-plans 等),与 OpenSpec 工作流不兼容。这导致:

1. **文档与变更脱节** - 文档更新无法追踪到具体的变更提案
2. **缺乏版本控制** - 历史决策和演进过程难以追溯
3. **质量门禁缺失** - 文档变更没有自动化验证和质量检查
4. **工作流不一致** - 开发流程使用 OpenSpec,但文档管理仍用传统方式

通过将现有文档迁移到 OpenSpec 规范,可以实现文档即代码(Docs as Code),让文档管理与开发工作流统一,提升可维护性和可追溯性。

## What Changes

- **迁移设计文档** - 将 `docs/design-docs/` 下的架构设计、决策记录转换为 OpenSpec design artifacts 并归档
- **迁移产品规格** - 将 `docs/product-specs/` 下的产品规格转换为 OpenSpec specs artifacts 并建立 capability 体系
- **归档执行计划** - 将 `docs/exec-plans/completed/` 下的已完成执行计划移动到 `openspec/changes/archive/`
- **整合 Sprint 计划** - 将 `docs/sprint-plans/` 的内容整合到 OpenSpec workflow 或保留为项目管理文档
- **更新引用路径** - 修正所有文档中的交叉引用链接,确保迁移后链接有效
- **建立映射关系** - 创建迁移映射表,记录原文档位置与新 OpenSpec artifact 的对应关系

## Capabilities

### New Capabilities

- `document-management`: 定义文档管理体系,包括文档分类、命名规范、归档策略和引用规则
- `design-documentation`: 规范架构设计文档的结构,包含系统架构、架构约束、决策记录的标准化格式
- `product-specification`: 建立产品规格体系,定义 vibe-coding、vibe-design、vibe-marketing 等模块的能力边界
- `execution-tracking`: 执行计划跟踪机制,支持计划的创建、执行、完成和归档全流程
- `sprint-planning`: Sprint 规划管理,包括 sprint 定义、任务分解、进度跟踪和回顾机制

### Modified Capabilities

<!-- 暂无修改现有 capabilities -->

## Impact

**受影响的文档**:
- `docs/design-docs/*` → 归档到 `openspec/changes/archive/` 或保留为参考文档
- `docs/product-specs/*` → 转换为 OpenSpec specs 或保留为核心产品文档
- `docs/exec-plans/completed/*` → 移动到 `openspec/changes/archive/`
- `docs/sprint-plans/*` → 评估是否整合到 OpenSpec 或保留独立
- `docs/references/*` → 部分可能成为 OpenSpec schema 的补充材料

**受影响的系统**:
- OpenSpec workflow - 新增多个 archived changes
- 文档引用系统 - 需要更新所有内部链接
- AGENTS.md - 可能需要更新文档导航指引

**依赖影响**:
- 无代码依赖变化,纯文档迁移
- 需要确保所有外部引用(docs 链接)仍然有效

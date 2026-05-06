## Why

当前项目存在双轨文档体系(`docs/` 和 `openspec/`),导致:

1. **文档分散** - 同一类文档分布在两个位置,查找困难
2. **维护成本高** - 需要同时维护两套文档结构和索引
3. **规范不统一** - docs 使用传统文档格式,openspec 使用标准化 workflow
4. **历史遗留** - 之前的迁移只完成了部分工作(exec-plans 和 ADRs),仍有大量文档在 docs/

通过完全移除 docs 目录并统一使用 OpenSpec,可以实现:
- 单一文档来源(Single Source of Truth)
- 标准化的变更管理流程
- 更好的版本控制和可追溯性
- 简化项目结构,降低维护成本

## What Changes

### 文档迁移

**从 `docs/` 迁移到 OpenSpec**:

1. **设计文档** (`docs/design-docs/`)
   - `system-architecture.md` → 归档为 OpenSpec change 或保留为核心参考
   - `architecture-rules.md` → 整合到 `design-documentation` capability
   - `index.md` → 合并到 AGENTS.md

2. **产品规格** (`docs/product-specs/`)
   - `product-design.md` → 归档或整合到 `product-specification` capability
   - `vibe-coding-spec.md` → 创建新的 `vibe-coding` capability
   - `vibe-design-spec.md` → 创建新的 `vibe-design` capability
   - `vibe-marketing-spec.md` → 创建新的 `vibe-marketing` capability
   - `index.md` → 合并到 AGENTS.md

3. **执行计划模板** (`docs/exec-plans/templates/`)
   - `how-to-create-exec-plan.md` → 整合到 `execution-tracking` capability
   - `how-to-archive-exec-plan.md` → 整合到 `execution-tracking` capability
   - `how-to-track-tech-debt.md` → 整合到 `execution-tracking` capability
   - `index.md` → 合并到 AGENTS.md

4. **Sprint 计划** (`docs/sprint-plans/`)
   - `sprint-guide.md` → 整合到 `sprint-planning` capability
   - `sprint-1.md`, `sprint-2.md` → 归档为 historical sprints
   - `archive/2026-Q1-sprint-1-mvp.md` → 已归档,保持
   - `index.md` → 合并到 AGENTS.md

5. **参考文档** (`docs/references/`)
   - `autonomous-coding-harness.md` → 归档或创建 `coding-harness` capability
   - `best-practices.md` → 整合到 AGENTS.md 或创建 `best-practices` capability
   - `symphony.md` → 归档(大型参考文档)
   - OpenSpec 相关文档 → 已存在,可删除重复
   - `index.md` → 合并到 AGENTS.md

6. **其他文档**
   - `dev_workflow.md` → 整合到 AGENTS.md 或创建 `development-workflow` capability
   - `data-storage.md` → 创建 `data-storage` capability 或归档
   - `migration-guide.md` → 归档(已完成迁移的临时文档)
   - `MIGRATION_GUIDE.md` → 归档(上一次迁移的文档)

### 目录清理

**BREAKING**: 删除整个 `docs/` 目录及其所有内容

### 导航更新

- 更新根目录 `AGENTS.md`,将所有文档引用指向 OpenSpec
- 创建统一的文档索引导航

## Capabilities

### New Capabilities

- `vibe-coding`: Vibe Coding 模块的产品规格和功能定义
- `vibe-design`: Vibe Design 模块的产品规格和设计系统规范
- `vibe-marketing`: Vibe Marketing 模块的产品规格和营销分析功能
- `development-workflow`: 开发工作流程规范,包括 Harness Engineering 流程
- `data-storage`: 数据存储方案和数据库设计规范
- `coding-harness`: 自主编码 Harness 的详细规范和最佳实践
- `best-practices`: 项目开发最佳实践和编码规范

### Modified Capabilities

- `design-documentation`: 补充架构规则和系统设计相关内容
- `product-specification`: 整合产品设计和规格管理体系
- `execution-tracking`: 添加执行计划模板和技术债务追踪规范
- `sprint-planning`: 整合 Sprint 指南和规划流程

## Impact

**受影响的文件**:
- ❌ **删除**: 整个 `docs/` 目录 (29 个文件)
- ✏️ **修改**: `AGENTS.md` - 全面更新文档导航
- ➕ **新增**: 7 个新的 capability specs
- ✏️ **更新**: 4 个现有 capabilities (design-documentation, product-specification, execution-tracking, sprint-planning)

**受影响的系统**:
- 文档导航系统 - 所有文档引用需要更新
- 开发者工作流 - 需要适应纯 OpenSpec 文档体系
- IDE/编辑器配置 - 可能需要更新文档搜索路径

**Breaking Changes**:
- ⚠️ 所有指向 `docs/` 的外部链接将失效
- ⚠️ 需要更新 README.md 和其他入口文档中的链接
- ⚠️ 团队成员需要重新熟悉文档位置

**迁移策略**:
1. 先创建所有新的 capabilities
2. 更新 AGENTS.md 建立新的导航
3. 验证所有链接有效性
4. 最后删除 docs/ 目录
5. 创建迁移说明文档帮助过渡

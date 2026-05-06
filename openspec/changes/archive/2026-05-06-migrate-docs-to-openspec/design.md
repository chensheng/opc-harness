## Context

当前项目采用双层文档体系:
- **传统文档** (`docs/`): 包含设计文档、产品规格、执行计划、Sprint 计划等,采用目录分类组织
- **OpenSpec 工作流** (`openspec/`): 基于变更的文档管理,支持 proposal → specs → design → tasks → quality-check → runtime-check 完整流程

现有问题:
1. 两套体系并行,开发者需要维护两个位置
2. 历史文档无法追踪到具体变更
3. 缺乏统一的归档和版本管理机制
4. 文档质量没有自动化验证

本次迁移需要将 `docs/` 下的核心文档整合到 OpenSpec 体系中,实现文档即代码。

## Goals / Non-Goals

**Goals:**
- 将已完成的历史文档(执行计划、决策记录)归档到 `openspec/changes/archive/`
- 建立新的 capability 体系,覆盖文档管理、设计规范、产品规格、执行跟踪、Sprint 规划
- 确保所有文档引用路径在迁移后仍然有效
- 保留核心参考文档在 `docs/` 作为快速导航入口

**Non-Goals:**
- 不修改文档内容本身(仅调整组织结构)
- 不改变现有的 AGENTS.md 导航结构
- 不涉及代码层面的变更
- 不迁移临时性文档(如 generated/、optimizations/ 空目录)

## Decisions

### Decision 1: 历史文档归档策略

**选择**: 将已完成的执行计划和决策记录移动到 `openspec/changes/archive/YYYY-MM-DD-migrate-docs-to-openspec/`

**理由**:
- OpenSpec archive 目录专为已完成变更设计,格式为 `YYYY-MM-DD-<name>`
- 保持历史可追溯性,同时清理 `docs/` 目录
- 符合 OpenSpec 工作流的归档规范

**替代方案**:
- ❌ 保留在 `docs/exec-plans/completed/` - 与 OpenSpec 工作流不一致
- ❌ 删除旧文档 - 失去历史记录

### Decision 2: 核心产品规格保留策略

**选择**: `docs/product-specs/` 保留作为产品级规格索引,同时在 OpenSpec 中建立对应的 capabilities

**理由**:
- 产品规格是跨变更的全局规范,不适合放在单个 change 中
- OpenSpec specs 聚焦于变更引入的新能力,而非全局产品定义
- 保留 `docs/product-specs/` 作为权威产品文档,OpenSpec specs 引用它们

**替代方案**:
- ❌ 完全迁移到 OpenSpec specs - 产品规格不是变更级别的 artifact
- ❌ 删除 docs 版本 - 失去全局产品视图

### Decision 3: Sprint 计划处理方式

**选择**: 保留 `docs/sprint-plans/` 独立,不迁移到 OpenSpec

**理由**:
- Sprint 计划是项目管理 artifact,不属于技术变更范畴
- OpenSpec 聚焦于功能和架构变更,而非迭代管理
- 保留独立便于团队查看 sprint 历史和规划

**替代方案**:
- ❌ 迁移到 OpenSpec - Sprint 不是 technical change
- ❌ 删除 - 失去 sprint 历史记录

### Decision 4: 设计文档处理

**选择**: `docs/design-docs/system-architecture.md` 和 `architecture-rules.md` 保留为核心参考文档,决策记录归档

**理由**:
- 系统架构和架构约束是全局规范,被多个变更引用
- 决策记录(ADR)是历史决策,适合归档到 OpenSpec changes
- 保留核心文档在 `docs/` 便于快速访问

### Decision 5: 引用链接更新范围

**选择**: 仅更新 OpenSpec artifacts 内部的引用,不批量修改 `docs/` 下的所有文档

**理由**:
- 批量修改风险高,可能破坏外部引用
- `docs/` 保留的文档维持原有链接结构
- 新创建的 OpenSpec artifacts 使用正确的相对路径

## Risks / Trade-offs

**[Risk] 链接断裂** → **Mitigation**: 
- 保留 `docs/` 目录结构,仅移动归档文档
- 创建迁移映射表记录所有路径变化
- 提供搜索指引帮助找到迁移后的文档

**[Risk] 开发者混淆** → **Mitigation**:
- 更新 AGENTS.md 明确哪些文档在 `docs/`,哪些在 `openspec/`
- 在 `docs/` 保留索引文件指向 OpenSpec artifacts
- 提供清晰的文档导航指南

**[Risk] 归档格式不一致** → **Mitigation**:
- 遵循 OpenSpec archive 命名规范 `YYYY-MM-DD-<name>`
- 每个 archived change 包含完整的 proposal/design/specs/tasks
- 使用统一的归档日期(迁移执行日期)

**[Trade-off] 双轨制持续存在** → 接受短期混乱,换取渐进式迁移
- 不强制一次性迁移所有文档
- 允许 `docs/` 和 `openspec/` 并存
- 未来新变更统一使用 OpenSpec,旧文档逐步归档

## Migration Plan

### Phase 1: 准备阶段
1. 分析 `docs/` 目录结构,识别需要迁移的文档
2. 创建 OpenSpec change `migrate-docs-to-openspec`
3. 定义新的 capabilities 体系

### Phase 2: 归档历史文档
1. 移动 `docs/exec-plans/completed/*` → `openspec/changes/archive/`
2. 移动 `docs/design-docs/decision-records/*` → `openspec/changes/archive/`
3. 为每个归档项创建最小化的 OpenSpec artifacts (proposal + specs)

### Phase 3: 建立 Capabilities
1. 为 5 个新 capabilities 创建 spec 文件
2. 定义每个 capability 的需求和场景
3. 关联到现有的产品规格文档

### Phase 4: 更新引用
1. 修正 AGENTS.md 中的文档导航链接
2. 在 `docs/` 创建迁移说明文档
3. 验证所有关键链接有效性

### Phase 5: 验证与归档
1. 运行 `npm run harness:check` 确保质量门禁通过
2. 启动应用验证无运行时错误
3. 归档 `migrate-docs-to-openspec` change

## Open Questions

1. **是否需要对所有历史执行计划创建完整的 OpenSpec artifacts?**
   - 建议: 仅创建 proposal.md 摘要,保留原始文档作为附件
   
2. **`docs/references/` 下的 OpenSpec 相关文档如何处理?**
   - 建议: 保留作为 OpenSpec schema 的补充说明,不迁移

3. **未来的文档更新流程是什么?**
   - 建议: 新变更通过 OpenSpec workflow,定期归档到 `docs/` 作为参考

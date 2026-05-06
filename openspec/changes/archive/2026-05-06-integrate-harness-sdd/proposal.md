## Why

当前项目已经建立了 OpenSpec 工作流和 SDD 文档体系,但两者之间的整合还不够紧密。Harness Engineering 流程主要关注开发执行和质量验证,而 SDD 更侧重架构设计和决策记录。为了更好地发挥两者的优势,需要将 harness 工作流与 SDD 实践深度整合,形成统一的开发模式。

## What Changes

- **整合 harness 流程与 SDD 文档**:在 Harness Engineering 开发流程中明确 SDD 的使用时机和规范
- **建立 ADR 与 OpenSpec 变更的关联机制**:确保重要的架构决策通过 ADR 记录,并与相关的 OpenSpec change 关联
- **优化开发工作流**:在 propose/design/apply/archive 各阶段明确 SDD 相关任务
- **更新 AGENTS.md 导航**:简化文档结构,突出 harness+SDD 整合后的开发模式

## Capabilities

### New Capabilities
- `harness-sdd-integration`: 定义 harness 工作流与 SDD 实践的整合规范,包括何时创建 SDD、如何编写 ADR、如何与 OpenSpec artifacts 协同

### Modified Capabilities
- `development-workflow`: 修改开发流程规范,融入 SDD 实践要求
- `design-documentation`: 强化设计文档与 OpenSpec 的集成,明确 SDD 在变更管理中的角色

## Impact

**受影响的文件**:
- `openspec/specs/development-workflow/spec.md` - 更新开发流程规范
- `openspec/specs/design-documentation/spec.md` - 强化 SDD 与 OpenSpec 集成
- `AGENTS.md` - 更新导航地图,反映整合后的开发模式
- `openspec/changes/integrate-harness-sdd/design.md` - 详细设计方案
- `openspec/changes/integrate-harness-sdd/tasks.md` - 实施任务列表

**影响范围**:
- 所有开发者需要遵循新的整合开发模式
- OpenSpec 变更流程将包含 SDD 相关要求
- 架构决策记录(ADR)将成为标准实践

## Context

AGENTS.md 是项目的核心导航文档,为 AI Agent 和开发者提供快速入口和关键信息。该文档于 2026-05-06 完成了从旧版 `docs/` 目录到 OpenSpec 工作流的迁移。迁移完成后,文档末尾保留了一个"迁移说明"章节,用于过渡期间提示用户报告断裂链接或缺失文档。

当前状态:
- 所有主要文档引用已验证存在(development-workflow、sprint-planning、design-documentation、harness-sdd-integration specs)
- src/AGENTS.md 和 src-tauri/AGENTS.md 文件存在
- e2e/app.spec.ts 测试文件存在
- 迁移工作已完成超过合理过渡期

## Goals / Non-Goals

**Goals:**
- 清理 AGENTS.md 中过时的迁移说明内容
- 保持文档简洁,避免给新开发者造成困惑
- 确保文档只包含当前相关和有效的信息

**Non-Goals:**
- 不修改其他任何文档内容
- 不添加新的功能或能力
- 不改变文档结构或其他引用链接

## Decisions

**决策: 移除整个"迁移说明"章节**

理由:
1. 迁移工作已经完成,所有引用都已验证有效
2. 过渡期提示不再必要,可能造成混淆
3. 如有未来文档问题,应通过正常的 issue 追踪机制处理,而非在 AGENTS.md 中保留永久性提示

**替代方案考虑:**
- 保留但更新说明 → 拒绝:仍会占用空间且无实际价值
- 移动到其他地方 → 拒绝:迁移说明本质上是临时性的,无需长期保留

## Risks / Trade-offs

**[Risk] 未来发现断裂链接时无明确报告渠道** → Mitigation: 开发者可通过 Git issues 或团队沟通渠道报告,这是标准做法

**[Trade-off] 丢失迁移历史信息** → 可接受:Git history 已完整记录迁移过程,无需在文档中重复

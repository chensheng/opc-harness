## Context

当前项目已经建立了两个重要的工程实践体系:

1. **OpenSpec 工作流**:通过结构化的变更管理(proposal → design → specs → tasks → apply → archive)确保每次修改都有完整的文档和验证
2. **SDD 软件设计文档**:包含系统架构、分层设计、数据流规则、ADR 等标准软件工程实践

但目前这两个体系相对独立,缺乏明确的整合机制。开发者在实际工作中可能不清楚:
- 何时需要创建/更新 SDD 文档
- ADR 应该如何与 OpenSpec changes 关联
- harness 开发流程中如何融入 SDD 实践

## Goals / Non-Goals

**Goals:**
- 明确 harness 工作流各阶段与 SDD 实践的对应关系
- 建立 ADR 与 OpenSpec change 的关联机制
- 在 development-workflow spec 中融入 SDD 要求
- 简化 AGENTS.md 导航,突出整合后的开发模式
- 提供清晰的决策指南:何时创建 SDD、何时编写 ADR

**Non-Goals:**
- 不改变 OpenSpec 的核心工作流(propose/design/apply/archive)
- 不重构现有的 SDD 文档结构
- 不涉及代码层面的架构变更
- 不修改 harness:check 的质量检查逻辑

## Decisions

### Decision 1: 在 development-workflow spec 中增加 SDD 实践要求

**选择方案**:在现有的 Harness Engineering 开发流程中,明确各阶段与 SDD 的交互点

**理由**:
- 保持现有流程不变,只是增强指导
- 开发者无需学习全新流程,降低认知负担
- SDD 作为架构约束自然融入开发过程

**替代方案**:
- ❌ 创建独立的 "SDD 工作流" spec - 会导致流程分裂,增加复杂性
- ❌ 将 SDD 完全纳入 OpenSpec - SDD 是长期稳定的架构文档,不应随每个 change 频繁变动

**实施方式**:
在 `development-workflow/spec.md` 中新增 Requirements:
- **Requirement: SDD 文档维护** - 明确何时更新系统架构文档
- **Requirement: ADR 创建规范** - 定义需要记录 ADR 的场景
- **Scenario: 架构变更时的文档更新** - 描述如何处理重大架构调整

### Decision 2: 建立 ADR 与 OpenSpec Change 的双向引用机制

**选择方案**:在 ADR 中引用相关的 OpenSpec change,在 change 的 design.md 中引用相关的 ADR

**理由**:
- 保持追溯性:可以从架构决策追溯到具体实现
- 保持独立性:ADR 长期稳定,change 临时存在后归档
- 符合现有实践:design-documentation spec 已要求两者保持一致

**实施方式**:
- 在 ADR 模板中增加 "Related Changes" 章节,列出相关的 OpenSpec change 名称
- 在 change 的 design.md 中增加 "Related ADRs" 章节,引用受影响的架构决策
- 在 design-documentation spec 中明确要求这种双向引用

### Decision 3: 明确 SDD 更新时机

**选择方案**:定义三类场景及对应的文档更新策略

**理由**:
- 避免过度文档化:不是每个 change 都需要更新 SDD
- 避免文档滞后:重大变更时必须更新 SDD
- 提供清晰判断标准

**场景分类**:

| 场景 | SDD 更新 | ADR 创建 | OpenSpec Specs 更新 |
|------|---------|---------|-------------------|
| **重大架构变更** (如新增分层、改变数据流规则) | ✅ 必须 | ✅ 必须 | ✅ 必须 |
| **模块级设计调整** (如新增服务层、修改接口) | ⚠️ 可选 | ✅ 建议 | ✅ 必须 |
| **功能实现** (不改变架构) | ❌ 不需要 | ❌ 不需要 | ✅ 必须 |

**实施方式**:
在 design-documentation spec 中增加 Requirement: "SDD 更新决策矩阵",明确上述规则

### Decision 4: 简化 AGENTS.md 结构

**选择方案**:将 SDD 相关内容整合到现有章节,而非独立大段

**理由**:
- AGENTS.md 应该是快速导航,不是详细文档
- 详细内容应在对应的 spec 中
- 减少重复,保持一致性

**实施方式**:
- 保留 "📐 SDD 软件设计文档" 章节,但精简为关键要点和链接
- 在 "🏗️ 三大支柱" 的 "架构约束" 部分强调 SDD 的作用
- 在 "❓ 常见问题" 中增加 "Harness 与 SDD 如何协同?"

## Risks / Trade-offs

### Risk 1: 开发者可能混淆 SDD 和 OpenSpec design.md 的职责

**影响**:可能导致文档重复或遗漏

**缓解措施**:
- 在 AGENTS.md 中明确两者的区别(已有表格对比)
- 在 design.md template 中增加提示:"参考 SDD 定义的架构规则"
- 通过 harness:check 自动检测文档完整性

### Risk 2: ADR 数量增长过快,难以维护

**影响**:ADR 库变得庞大,查找困难

**缓解措施**:
- 严格遵循 ADR 创建标准(仅重大决策)
- 定期归档过时的 ADR (标记为 Deprecated)
- 在 design-documentation spec 中提供 ADR 索引

### Risk 3: 整合初期可能存在文档不一致

**影响**:短期内可能出现 SDD 与 specs 不同步

**缓解措施**:
- 在本次 change 中一次性更新所有相关文档
- 在 quality-check.md 中增加文档一致性检查项
- 通过 harness:check 自动化验证

## Migration Plan

**实施步骤**:
1. 创建新的 capabilities specs (harness-sdd-integration)
2. 更新现有 specs (development-workflow, design-documentation)
3. 更新 AGENTS.md 导航
4. 运行 harness:check 验证文档一致性
5. 归档 change 并通知团队

**回滚策略**:
- 由于主要是文档变更,回滚只需恢复相关文件
- Git history 可追溯所有变更
- 不影响现有代码功能

## Open Questions

无。本次变更聚焦文档和规范整合,所有决策已在上述章节明确。

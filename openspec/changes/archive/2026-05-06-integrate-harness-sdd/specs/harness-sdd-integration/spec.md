## ADDED Requirements

### Requirement: Harness 与 SDD 整合开发模式
系统 SHALL 提供统一的 harness+SDD 整合开发模式,明确 OpenSpec 工作流与软件设计文档实践的协同关系。

#### Scenario: 开发者开始新任务时查阅整合指南
- **WHEN** 开发者准备开始新的开发任务
- **THEN** 系统应在 AGENTS.md 中提供清晰的 harness+SDD 整合开发流程指引

#### Scenario: 判断是否需要更新 SDD 文档
- **WHEN** 开发者完成 OpenSpec change 的实施
- **THEN** 系统应根据变更类型(重大架构/模块级/功能实现)提供明确的 SDD 更新决策指南

### Requirement: ADR 与 OpenSpec Change 双向引用
系统 SHALL 支持 ADR (Architecture Decision Record) 与 OpenSpec changes 之间的双向引用机制,确保架构决策与具体实现的追溯性。

#### Scenario: 在 ADR 中引用相关变更
- **WHEN** 创建新的 ADR 记录重大架构决策
- **THEN** ADR 文档 SHALL 包含 "Related Changes" 章节,列出相关的 OpenSpec change 名称

#### Scenario: 在 Change Design 中引用相关 ADR
- **WHEN** 编写 OpenSpec change 的 design.md
- **THEN** 如果变更涉及架构决策,design.md SHALL 包含 "Related ADRs" 章节,引用受影响的架构决策

### Requirement: SDD 更新决策矩阵
系统 SHALL 提供明确的决策矩阵,指导开发者在不同场景下是否需要更新 SDD 文档、创建 ADR 或更新 OpenSpec specs。

#### Scenario: 重大架构变更时的文档要求
- **WHEN** 变更涉及新增分层、改变数据流规则等重大架构调整
- **THEN** 系统 SHALL 要求同时更新 SDD 文档、创建 ADR、并更新 OpenSpec specs

#### Scenario: 模块级设计调整时的文档要求
- **WHEN** 变更涉及新增服务层、修改接口等模块级调整
- **THEN** 系统 SHALL 建议创建 ADR,可选更新 SDD,但必须更新 OpenSpec specs

#### Scenario: 纯功能实现时的文档要求
- **WHEN** 变更仅实现新功能,不改变现有架构
- **THEN** 系统 SHALL 只要求更新 OpenSpec specs,无需更新 SDD 或创建 ADR

### Requirement: SDD 在 Harness 流程中的集成点
系统 SHALL 在 Harness Engineering 开发流程的各阶段(propose/design/apply/archive)明确 SDD 相关的任务和检查点。

#### Scenario: Propose 阶段评估 SDD 影响
- **WHEN** 开发者使用 `/opsx:propose` 创建新变更
- **THEN** proposal.md SHALL 包含 "Impact" 章节,说明是否需要更新 SDD 或创建 ADR

#### Scenario: Design 阶段参考 SDD 架构规则
- **WHEN** 开发者编写 design.md
- **THEN** design.md SHALL 引用相关的 SDD 架构规则,确保设计方案符合系统整体架构

#### Scenario: Archive 阶段验证文档一致性
- **WHEN** 开发者使用 `/opsx:archive` 归档完成的变更
- **THEN** 系统 SHALL 验证 SDD、ADR 和 OpenSpec specs 之间的一致性

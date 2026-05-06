## MODIFIED Requirements

### Requirement: Harness Engineering 开发流程
系统 SHALL 强制执行标准化的开发工作流程,包括任务选择、执行计划创建、架构学习、测试设计、开发实施、质量验证等阶段,并明确与 SDD 实践的集成点。

#### Scenario: 创建执行计划
- **WHEN** 开发者选择新任务
- **THEN** 系统引导创建详细的执行计划,包含步骤、验收标准和质量检查项

#### Scenario: 质量门禁验证
- **WHEN** 开发者完成编码
- **THEN** 系统自动运行 harness:check,确保 Health Score ≥ 80

#### Scenario: 评估 SDD 影响
- **WHEN** 开发者在 propose 阶段创建 proposal.md
- **THEN** proposal SHALL 评估变更对 SDD 文档的影响,明确是否需要更新架构文档或创建 ADR

### Requirement: SDD 文档维护
系统 SHALL 要求开发者在特定场景下更新 SDD (Software Design Document) 文档,确保架构文档与实现保持一致。

#### Scenario: 重大架构变更时更新 SDD
- **WHEN** 变更涉及新增分层、改变数据流规则、修改核心架构模式
- **THEN** 开发者 MUST 更新 `openspec/specs/design-documentation/spec.md` 中的相关章节

#### Scenario: 模块级调整时可选更新 SDD
- **WHEN** 变更涉及新增服务层、修改接口定义等模块级调整
- **THEN** 开发者 SHOULD 考虑更新 SDD 文档,但非强制要求

#### Scenario: 纯功能实现时不更新 SDD
- **WHEN** 变更仅实现新功能,不改变现有架构
- **THEN** 开发者无需更新 SDD 文档

### Requirement: ADR 创建规范
系统 SHALL 要求开发者在做出重要架构决策时创建 ADR (Architecture Decision Record),记录决策背景和 rationale。

#### Scenario: 重大技术选型时创建 ADR
- **WHEN** 团队决定采用新的技术栈、框架或工具链
- **THEN** 开发者 MUST 创建 ADR 记录选型理由、替代方案和预期影响

#### Scenario: 架构变更时创建 ADR
- **WHEN** 引入新的分层、改变依赖方向、修改数据流规则
- **THEN** 开发者 MUST 创建 ADR 记录架构决策

#### Scenario: ADR 引用相关 OpenSpec Changes
- **WHEN** 创建新的 ADR
- **THEN** ADR SHALL 包含 "Related Changes" 章节,列出相关的 OpenSpec change 名称

### Requirement: 测试先行 (TDD)
系统 MUST 强制测试先行的开发实践,要求先写测试再实现功能。

#### Scenario: 单元测试覆盖
- **WHEN** 实现新功能
- **THEN** 必须先编写单元测试,覆盖率 ≥ 70%

#### Scenario: E2E 测试验证
- **WHEN** 核心流程开发完成
- **THEN** 必须编写 E2E 测试验证端到端功能

### Requirement: 架构约束强制执行
系统 SHALL 通过自动化工具强制执行分层架构和依赖规则。

#### Scenario: 检测违规依赖
- **WHEN** 代码违反分层架构规则(如 Store 依赖 Component)
- **THEN** ESLint 报错并阻止提交

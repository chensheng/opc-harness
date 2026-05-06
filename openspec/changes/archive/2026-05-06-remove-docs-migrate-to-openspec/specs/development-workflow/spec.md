## ADDED Requirements

### Requirement: Harness Engineering 开发流程
系统 SHALL 强制执行标准化的开发工作流程,包括任务选择、执行计划创建、架构学习、测试设计、开发实施、质量验证等阶段。

#### Scenario: 创建执行计划
- **WHEN** 开发者选择新任务
- **THEN** 系统引导创建详细的执行计划,包含步骤、验收标准和质量检查项

#### Scenario: 质量门禁验证
- **WHEN** 开发者完成编码
- **THEN** 系统自动运行 harness:check,确保 Health Score ≥ 80

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

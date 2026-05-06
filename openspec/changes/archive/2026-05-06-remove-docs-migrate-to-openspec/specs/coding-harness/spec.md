## ADDED Requirements

### Requirement: 自主编码 Harness 规范
系统 SHALL 提供完整的自主编码 Harness 实现指南,包括 Agent 通信协议、任务分解策略和质量控制机制。

#### Scenario: Agent 间通信
- **WHEN** Initializer Agent 完成任务分解
- **THEN** 通过标准化协议将 Issues 分发给 Coding Agents

#### Scenario: 质量控制循环
- **WHEN** Coding Agent 生成代码
- **THEN** Harness 自动执行 lint、test、type-check,失败时触发修复循环

### Requirement: 最佳实践集成
系统 MUST 集成开发最佳实践,包括代码规范、设计模式、测试策略等。

#### Scenario: 代码规范检查
- **WHEN** 生成 TypeScript 代码
- **THEN** 遵循项目 ESLint 和 Prettier 配置

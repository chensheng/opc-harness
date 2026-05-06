## ADDED Requirements

### Requirement: 项目开发最佳实践
系统 SHALL 提供全面的开发最佳实践指南,涵盖代码质量、架构设计、测试策略等方面。

#### Scenario: 代码审查清单
- **WHEN** 开发者提交代码前
- **THEN** 系统提供代码审查清单,确保符合最佳实践

#### Scenario: 性能优化建议
- **WHEN** 检测到潜在性能问题
- **THEN** 系统提供具体的优化建议和示例代码

### Requirement: 编码规范
系统 MUST 强制执行统一的编码规范,包括命名约定、文件组织、注释标准等。

#### Scenario: 命名规范检查
- **WHEN** 变量或函数命名不符合规范
- **THEN** linter 报错并提供修正建议

## ADDED Requirements

### Requirement: Harness Engineering 开发流程
系统 SHALL 强制执行标准化的开发工作流程,包括任务选择、执行计划创建、架构学习、测试设计、开发实施、质量验证等阶段,并明确与 SDD 实践的集成点。

#### Scenario: 创建执行计划
- **WHEN** 开发者选择新任务
- **THEN** 系统引导创建详细的执行计划,包含步骤、验收标准和质量检查项

#### Scenario: Rust 编译无警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码无编译警告
- **THEN** Rust Compilation Check 项获得满分（12.5 分）
- **AND** 显示 "[PASS] Rust compilation check passed"

#### Scenario: Rust 编译有少量警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码有 1-6 个编译警告
- **THEN** Rust Compilation Check 项根据警告数量扣分（每个警告扣 2 分）
- **AND** 显示 "[WARN] Found N warnings (-M points)"
- **AND** 列出前 5 个警告的摘要信息

#### Scenario: Rust 编译有大量警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码有 7 个或更多编译警告
- **THEN** Rust Compilation Check 项得 0 分
- **AND** 显示 "[FAIL] Found N warnings (maximum penalty applied)"
- **AND** 提示开发者清理警告以提高评分

#### Scenario: 健康评分计算
- **WHEN** 所有 8 项检查完成
- **THEN** 系统计算总分时包含 Rust 警告扣分
- **AND** 如果总分 < 80，标记为 FAIL 并提示需要改进

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

### Requirement: README 文档结构

README.md SHALL 保持简洁，仅包含概述、核心功能、快速开始三个核心部分，其他详细信息应引导至 AGENTS.md 和 OpenSpec specs。

#### Scenario: 新用户查看 README

- **WHEN** 新用户首次访问项目仓库
- **THEN** 他们能在 30 秒内理解项目价值和快速上手步骤

#### Scenario: 用户需要详细技术信息

- **WHEN** 用户需要了解开发工作流或技术架构
- **THEN** README 提供清晰的链接指向 AGENTS.md 和 openspec/specs/

### Requirement: 文档导航

README.md SHALL 在末尾提供"了解更多"章节，包含到 AGENTS.md 和 OpenSpec 文档的链接。

#### Scenario: 用户探索更多文档

- **WHEN** 用户阅读完 README 的快速开始部分
- **THEN** 他们能看到明确的下一步文档指引

### Requirement: 文档导航简化

AGENTS.md SHALL 在末尾提供"详细文档"章节，包含到 OpenSpec specs 的链接，避免在 AGENTS.md 中重复详细说明。

#### Scenario: 用户探索详细文档

- **WHEN** 用户阅读完 AGENTS.md 的快速入口部分
- **THEN** 他们能看到明确的下一步文档指引，链接到相关的 OpenSpec specs

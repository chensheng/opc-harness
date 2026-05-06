## MODIFIED Requirements

### Requirement: 架构文档结构
架构设计文档 MUST 包含以下标准章节:
- **系统概述**: 高层架构图和核心组件说明
- **分层架构**: FE/BE/TEST 各层的职责和边界
- **数据流规则**: 允许和禁止的依赖方向
- **代码规模约束**: 文件大小限制、模块化要求
- **技术栈说明**: 使用的框架、库、工具链

#### Scenario: 查看系统架构
- **WHEN** 开发者阅读系统架构文档
- **THEN** 应看到完整的分层架构图、组件交互和数据流规则

### Requirement: 架构约束强制执行
系统 SHALL 通过自动化工具强制执行架构约束:
- ESLint 规则检查分层依赖 (`architecture-constraint.cjs`)
- 文件大小检查 (单文件 ≤ 500 行)
- 测试覆盖率检查 (≥ 70%)

#### Scenario: 违反依赖规则
- **WHEN** 开发者在 Store 层导入 Component
- **THEN** ESLint 应报错 "Store layer cannot depend on UI layer"

#### Scenario: 文件过大
- **WHEN** 单个 TypeScript 文件超过 500 行
- **THEN** 架构检查应警告并要求模块化拆分

### Requirement: 架构决策记录 (ADR)
每个重要架构决策 MUST 记录为 ADR,包含:
- **标题**: 决策简述
- **状态**: Proposed/Accepted/Deprecated
- **背景**: 为什么需要这个决策
- **决策**: 具体选择了什么方案
- **后果**: 带来的影响和权衡

#### Scenario: 记录新技术选型
- **WHEN** 团队决定采用 OpenSpec 工作流
- **THEN** 应创建 ADR 记录选型理由、替代方案和预期影响

### Requirement: 设计文档与 OpenSpec 集成
架构设计文档 SHALL 与 OpenSpec design artifacts 保持一致:
- OpenSpec change 的 `design.md` 应引用核心架构文档
- 重大架构变更应同时更新架构文档和 OpenSpec specs

#### Scenario: 架构演进追踪
- **WHEN** 查看某个 OpenSpec change 的 design.md
- **THEN** 应能看到该变更如何符合或扩展现有架构规范

### Requirement: 系统设计文档归档
系统 SHALL 支持将历史系统设计文档归档到 OpenSpec changes,保持可追溯性。

#### Scenario: 归档系统架构文档
- **WHEN** 系统架构发生重大变更
- **THEN** 旧版本归档到 `openspec/changes/archive/YYYY-MM-DD-<name>/`,新版本成为当前标准

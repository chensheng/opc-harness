## MODIFIED Requirements

### Requirement: Sprint 指南标准化
Sprint 指南 SHALL 定义:
- Sprint 长度和节奏 (如 2 周一个 sprint)
- 角色职责 (PO、Scrum Master、Developer)
- 仪式安排 (Planning、Daily Standup、Review、Retro)
- 工件要求 (Backlog、Sprint Backlog、Increment)

#### Scenario: 新成员学习 Sprint 流程
- **WHEN** 新开发者加入团队
- **THEN** 应阅读 sprint-planning capability 了解如何参与 sprint

### Requirement: Sprint 归档机制
已完成的 Sprint 计划 SHALL 移动到 archive,保留历史记录供回顾和参考。

#### Scenario: 归档 Sprint
- **WHEN** Sprint 结束并完成回顾
- **THEN** 将 Sprint 计划归档到 `openspec/changes/archive/YYYY-MM-DD-sprint-N/`

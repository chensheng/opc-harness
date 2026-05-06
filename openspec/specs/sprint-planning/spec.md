## ADDED Requirements

### Requirement: Sprint 计划结构
每个 Sprint 计划文档 MUST 包含:
- **Sprint 信息**: 编号、日期范围、目标
- **用户故事**: 本 sprint 要完成的用户故事列表
- **任务分解**: 每个故事拆分为具体任务
- **容量规划**: 团队可用工时和任务分配
- **完成定义**: Sprint 成功的验收标准

#### Scenario: 查看 Sprint 1 计划
- **WHEN** 团队成员访问 `docs/sprint-plans/sprint-1.md`
- **THEN** 应看到 sprint 目标、用户故事列表和任务分配

### Requirement: Sprint 指南标准化
Sprint 指南 (`docs/sprint-plans/sprint-guide.md`) SHALL 定义:
- Sprint 长度和节奏 (如 2 周一个 sprint)
- 角色职责 (PO、Scrum Master、Developer)
- 仪式安排 (Planning、Daily Standup、Review、Retro)
- 工件要求 (Backlog、Sprint Backlog、Increment)

#### Scenario: 新成员学习 Sprint 流程
- **WHEN** 新开发者加入团队
- **THEN** 应阅读 sprint-guide.md 了解如何参与 sprint

### Requirement: Sprint 归档机制
已完成的 Sprint 计划 SHALL 移动到 `docs/sprint-plans/archive/`:
- 保留历史记录供回顾和参考
- 归档时添加 retrospective 总结
- 标记关键成果和学习点

#### Scenario: 归档 Sprint 1
- **WHEN** Sprint 1 结束并完成回顾
- **THEN** 应将 `sprint-1.md` 移动到 `archive/sprint-1.md` 并添加 retrospective 章节

### Requirement: Sprint 与 OpenSpec Changes 关联
Sprint 计划 SHOULD 关联相关的 OpenSpec changes:
- 在 sprint 计划中列出本 sprint 要实施的 changes
- 在 OpenSpec change 的 proposal 中引用所属 sprint
- 通过双向链接实现可追溯性

#### Scenario: 追踪 Sprint 中的 Changes
- **WHEN** 查看 `docs/sprint-plans/sprint-2.md`
- **THEN** 应看到本 sprint 涉及的 OpenSpec changes 列表和状态

### Requirement: Sprint 进度跟踪
Sprint 计划 MUST 提供进度跟踪机制:
- 每日更新任务完成状态
- 可视化燃尽图或进度条
- 标识阻塞任务和风险

#### Scenario: 每日站会更新进度
- **WHEN** 开发者在站会上汇报进展
- **THEN** 应更新 sprint 计划中的任务状态和剩余工时

### Requirement: Sprint 回顾文档化
每个 Sprint 结束后 MUST 产出回顾文档:
- **做得好的**: 继续保持的实践
- **待改进的**: 需要调整的流程或技术
- **行动计划**: 下个 sprint 要尝试的改进
- 回顾文档附加到归档的 sprint 计划中

#### Scenario: Sprint 回顾会议
- **WHEN** Sprint 2 结束召开回顾会议
- **THEN** 应在 `sprint-2.md` 底部添加 retrospective 章节记录讨论结果

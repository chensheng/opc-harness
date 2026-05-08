## MODIFIED Requirements

### Requirement: 进度追踪与可视化

系统 MUST 提供多维度的进度追踪，包括用户故事的重试状态和历史信息。

**原有内容**:
- 总体进度百分比
- 每个 Milestone 的完成状态
- 每个 Issue 的详细状态(pending/in-progress/completed/failed)
- 实时统计(已完成文件数、测试通过率等)
- 每个用户故事的重试状态（scheduled_retry/permanently_failed/max_retries_exceeded）
- 重试次数和预计下次重试时间
- 重试历史记录访问入口

**新增内容**:
- Native Agent 执行状态监控（idle/running/paused/completed/failed）
- AI Provider 调用统计（Token 消耗、API 调用次数、平均响应时间）
- 工具调用日志（read_file/write_file/git_commit 等操作记录）
- 质量检查详情（Lint 错误数、Test 失败数、修复次数）

#### Scenario: 可视化进度仪表板
- **WHEN** 用户打开 Vibe Coding 界面
- **THEN** 显示进度条、Milestone 列表、Issues 状态表格，以及重试中的 Story 数量
- **AND** 显示 Native Agent 执行状态徽章

#### Scenario: 失败任务重试
- **WHEN** 某个 Issue 执行失败
- **THEN** 系统根据错误类型自动决定是否重试，或用户可以点击重试按钮手动触发

#### Scenario: 显示重试状态徽章
- **WHEN** 用户故事处于 scheduled_retry 状态
- **THEN** 在列表中显示黄色"等待重试"徽章，并显示预计下次重试时间

#### Scenario: 查看重试历史
- **WHEN** 用户点击用户故事的"查看重试历史"按钮
- **THEN** 系统弹出对话框，展示该 Story 的所有重试记录和时间线

#### Scenario: 查看 Native Agent 执行详情
- **WHEN** 用户点击正在执行的 Story
- **THEN** 系统显示实时日志流（AI 思考过程、工具调用、代码生成）
- **AND** 显示 Token 消耗统计
- **AND** 显示质量检查结果

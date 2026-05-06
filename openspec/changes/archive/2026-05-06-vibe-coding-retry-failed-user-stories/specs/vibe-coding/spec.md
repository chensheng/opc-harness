## MODIFIED Requirements

### Requirement: 进度追踪与可视化

系统 MUST 提供多维度的进度追踪，包括用户故事的重试状态和历史信息。

**原有内容**:
- 总体进度百分比
- 每个 Milestone 的完成状态
- 每个 Issue 的详细状态(pending/in-progress/completed/failed)
- 实时统计(已完成文件数、测试通过率等)

**新增内容**:
- 每个用户故事的重试状态（scheduled_retry/permanently_failed/max_retries_exceeded）
- 重试次数和预计下次重试时间
- 重试历史记录访问入口

#### Scenario: 可视化进度仪表板
- **WHEN** 用户打开 Vibe Coding 界面
- **THEN** 显示进度条、Milestone 列表、Issues 状态表格，以及重试中的 Story 数量

#### Scenario: 失败任务重试
- **WHEN** 某个 Issue 执行失败
- **THEN** 系统根据错误类型自动决定是否重试，或用户可以点击重试按钮手动触发

#### Scenario: 显示重试状态徽章
- **WHEN** 用户故事处于 scheduled_retry 状态
- **THEN** 在列表中显示黄色"等待重试"徽章，并显示预计下次重试时间

#### Scenario: 查看重试历史
- **WHEN** 用户点击用户故事的"查看重试历史"按钮
- **THEN** 系统弹出对话框，展示该 Story 的所有重试记录和时间线

## ADDED Requirements

### Requirement: 重试调度器定时扫描

RetryScheduler SHALL 每 30 秒自动扫描数据库中的待重试队列，查找状态为 `scheduled_retry` 且 `next_retry_at <= now()` 的用户故事。

#### Scenario: 扫描到待重试的 Story

- **WHEN** 调度器执行定时扫描
- **THEN** 系统查询数据库中所有满足条件的 Story，并按 `next_retry_at` 升序排序

#### Scenario: 未找到待重试的 Story

- **WHEN** 数据库中没有待重试的 Story
- **THEN** 调度器记录调试日志并等待下一个扫描周期

#### Scenario: 扫描间隔可配置

- **WHEN** 调度器初始化时
- **THEN** 系统使用 `SchedulerConfig.check_interval_seconds` 配置扫描间隔（默认 30 秒）

### Requirement: 并发控制机制

RetryScheduler SHALL 限制同时执行的重试任务数量，最多不超过 3 个，避免资源过载。

#### Scenario: 未达到并发上限时触发新重试

- **WHEN** 当前活跃重试数量 < 3 且有新的待重试 Story
- **THEN** 系统注册该 Story 并开始执行重试任务

#### Scenario: 达到并发上限时拒绝新重试

- **WHEN** 当前活跃重试数量 >= 3 且有新的待重试 Story
- **THEN** 系统跳过该 Story，记录警告日志，等待下一个扫描周期

#### Scenario: 完成重试后释放并发槽位

- **WHEN** 一个重试任务完成（成功或失败）
- **THEN** 系统从 `active_retries` 中移除该 Story，允许新的重试任务启动

### Requirement: 重试任务触发逻辑

RetryScheduler SHALL 在扫描到待重试 Story 后，更新其状态为 `in_progress`，创建重试历史记录，并调用 Agent Worker 执行任务。

#### Scenario: 触发单个 Story 的重试

- **WHEN** 调度器决定重试某个 Story
- **THEN** 系统执行以下步骤：
  1. 更新 Story 状态为 `in_progress`
  2. 创建重试历史记录（status='pending'）
  3. 调用 `execute_user_story` 方法启动 Agent
  4. 将 Story ID 和 Agent ID 注册到 `active_retries`

#### Scenario: 重试任务启动失败

- **WHEN** 调用 `execute_user_story` 失败
- **THEN** 系统记录错误日志，不更新 Story 状态，等待下一个扫描周期重新尝试

### Requirement: 调度器生命周期管理

RetryScheduler SHALL 在 Agent Worker 启动时自动启动，在 Agent Worker 关闭时优雅停止，等待所有活跃的重试任务完成后退出。

#### Scenario: Agent Worker 启动时初始化调度器

- **WHEN** Agent Worker 的 `start()` 方法被调用
- **THEN** 系统在后台线程中启动 RetryScheduler 的 `run()` 方法

#### Scenario: 接收到关闭信号时优雅停止

- **WHEN** 调度器接收到关闭信号（Ctrl+C 或程序退出）
- **THEN** 系统停止定时扫描循环，但继续等待活跃的重试任务完成

#### Scenario: 所有活跃任务完成后退出

- **WHEN** `active_retries` 为空且收到关闭信号
- **THEN** 调度器记录关闭日志并退出后台线程

### Requirement: 重试历史记录完善

RetryScheduler SHALL 在每次重试开始时创建历史记录，并在重试完成后更新结果（success/failed）。

#### Scenario: 重试开始时创建记录

- **WHEN** 调度器触发某个 Story 的重试
- **THEN** 系统创建 `UserStoryRetryHistory` 记录，包含：
  - `retry_number`: 当前重试次数
  - `triggered_at`: 触发时间
  - `result`: "pending"

#### Scenario: 重试成功后更新记录

- **WHEN** 重试任务执行成功
- **THEN** 系统更新对应的重试历史记录：
  - `completed_at`: 完成时间
  - `result`: "success"

#### Scenario: 重试失败后更新记录

- **WHEN** 重试任务执行失败
- **THEN** 系统更新对应的重试历史记录：
  - `completed_at`: 完成时间
  - `result`: "failed"
  - `error_message`: 失败原因

### Requirement: 实时状态推送

RetryScheduler SHALL 通过 WebSocket 实时推送重试任务的状态变更到前端，包括开始、成功、失败等事件。

#### Scenario: 重试开始时推送通知

- **WHEN** 调度器触发某个 Story 的重试
- **THEN** 系统通过 WebSocket 发送日志消息："🔄 开始重试 Story {story_number}（第 {retry_count} 次）"

#### Scenario: 重试成功时推送通知

- **WHEN** 重试任务执行成功
- **THEN** 系统通过 WebSocket 发送日志消息："✅ Story {story_number} 重试成功"

#### Scenario: 重试失败时推送通知

- **WHEN** 重试任务执行失败
- **THEN** 系统通过 WebSocket 发送日志消息："❌ Story {story_number} 重试失败：{error_message}"

### Requirement: 调度器健康检查

RetryScheduler SHALL 提供状态查询接口，返回当前活跃重试数量、最后扫描时间等监控信息。

#### Scenario: 查询调度器状态

- **WHEN** 监控系统调用 `get_status()` 方法
- **THEN** 系统返回 `SchedulerStatus` 对象，包含：
  - `is_running`: 是否正在运行
  - `active_retry_count`: 活跃重试数量
  - `active_retries`: 活跃重试列表（story_id -> agent_id）
  - `last_scan_at`: 最后扫描时间

#### Scenario: 监控面板展示调度器状态

- **WHEN** 用户在 AgentMonitor 面板查看调度器状态
- **THEN** 系统显示当前活跃重试数量和详细列表

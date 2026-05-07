## MODIFIED Requirements

### Requirement: Agent Worker 重试引擎集成

Agent Worker SHALL 在初始化时加载并配置重试引擎和重试调度器，使其能够自动扫描待重试队列并触发重试任务。

#### Scenario: Agent Worker 启动时初始化重试调度器

- **WHEN** Agent Worker 的 `start()` 方法被调用
- **THEN** 系统在后台线程中启动 RetryScheduler 的 `run()` 方法，传入 project_id 和 websocket_manager

#### Scenario: Agent Worker 优雅关闭时停止调度器

- **WHEN** Agent Worker 接收到关闭信号或调用 `stop()` 方法
- **THEN** 系统向调度器发送停止信号，等待所有活跃的重试任务完成后退出后台线程

#### Scenario: 调度器与 Agent Loop 并行运行

- **WHEN** Agent Worker 正常运行时
- **THEN** 系统的 Agent Loop（查询 pending Stories）和 RetryScheduler（查询 scheduled_retry Stories）同时在后台运行，互不干扰

## Why

当前 Vibe Coding 的重试调度器（RetryScheduler）核心框架已完成，但关键的定时扫描和任务触发逻辑尚未实现（tasks.md 中 5.4-5.6 未完成），且 Agent Worker 未集成调度器的启动/停止机制（tasks.md 中 6.5-6.7 未完成）。这导致失败的用户故事无法自动重试，降低了系统的自动化程度和可靠性。需要完整实现 RetryScheduler 的后台调度功能，使其能够自动扫描待重试队列并触发重试任务。

## What Changes

- **完整实现 RetryScheduler 定时扫描逻辑** - 每 30 秒扫描数据库中 `status='scheduled_retry'` 且 `next_retry_at <= now()` 的用户故事
- **实现并发控制机制** - 限制同时重试的 Story 数量（最多 3 个），避免资源过载
- **实现重试任务触发逻辑** - 调用 Agent Worker 执行重试任务，更新状态为 `in_progress`
- **实现调度器的启动和停止方法** - 在 Agent Worker 初始化时自动启动调度器，关闭时优雅停止
- **添加调度器健康检查** - 监控调度器运行状态，支持故障恢复
- **完善重试历史记录** - 记录每次重试的开始时间、结束时间和结果

## Capabilities

### New Capabilities
- `retry-scheduler`: 重试调度器的完整实现，包括定时扫描、并发控制、任务触发和生命周期管理

### Modified Capabilities
- `agent-initialization`: Agent Worker 初始化时需要启动重试调度器，关闭时需要优雅停止

## Impact

**受影响的代码**:
- `src-tauri/src/agent/retry_engine.rs` - 新增 RetryScheduler 的完整实现
- `src-tauri/src/agent/agent_worker.rs` - 集成调度器的启动/停止逻辑
- `src-tauri/src/db/user_story_repository.rs` - 可能需要新增查询待重试队列的方法

**受影响的系统**:
- Agent Worker 生命周期管理
- 数据库查询性能（定时扫描）
- WebSocket 实时日志推送（重试任务状态更新）

**依赖关系**:
- 依赖现有的 RetryEngine、ErrorClassifier、BackoffCalculator
- 依赖数据库表 `user_stories` 和 `user_story_retry_history`
- 依赖 WebSocketManager 用于实时状态推送

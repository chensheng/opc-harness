## 1. 数据库层增强

- [x] 1.1 在 `src-tauri/src/db/user_story_repository.rs` 中实现 `get_pending_retries()` 方法，查询待重试队列
- [x] 1.2 添加数据库索引优化：为 `user_stories` 表的 `next_retry_at` 字段创建索引
- [x] 1.3 实现 `update_retry_history_result()` 方法，用于更新重试历史记录的结果

## 2. RetryScheduler 核心实现

- [x] 2.1 在 `src-tauri/src/agent/retry_engine.rs` 中完善 `RetryScheduler::run()` 方法，实现定时扫描循环
- [x] 2.2 实现 `scan_and_trigger()` 方法，执行单次扫描并触发待重试任务
- [x] 2.3 实现优雅停止逻辑，使用 `tokio::select!` 监听关闭信号
- [x] 2.4 实现 `get_status()` 方法，返回调度器健康状态
- [ ] 2.5 编写 RetryScheduler 的单元测试，覆盖并发控制和生命周期管理

## 3. Agent Worker 集成

- [x] 3.1 修改 `AgentWorker::start()` 方法，在启动时初始化并运行 RetryScheduler
- [ ] 3.2 修改 `AgentWorker::stop()` 方法，向调度器发送停止信号
- [x] 3.3 确保调度器与现有 Agent Loop 并行运行，互不干扰
- [x] 3.4 添加日志记录，追踪调度器的启动、扫描和关闭过程

## 4. 重试任务触发逻辑

- [ ] 4.1 实现 `trigger_retry()` 方法，更新 Story 状态为 `in_progress`
- [ ] 4.2 在触发重试时创建初始的重试历史记录（result='pending'）
- [ ] 4.3 调用现有的 `execute_user_story()` 方法启动 Agent 执行任务
- [ ] 4.4 注册 Story 到 `active_retries` HashMap 进行并发控制
- [ ] 4.5 处理任务启动失败的情况，记录错误日志

## 5. 重试结果处理

- [ ] 5.1 在 `execute_user_story()` 完成后，根据结果更新重试历史记录
- [ ] 5.2 成功时更新 `result='success'` 和 `completed_at`
- [ ] 5.3 失败时更新 `result='failed'`、`completed_at` 和 `error_message`
- [ ] 5.4 从 `active_retries` 中移除已完成的 Story，释放并发槽位

## 6. WebSocket 实时推送

- [ ] 6.1 在重试开始时通过 WebSocket 发送通知："🔄 开始重试 Story {story_number}"
- [ ] 6.2 在重试成功时发送通知："✅ Story {story_number} 重试成功"
- [ ] 6.3 在重试失败时发送通知："❌ Story {story_number} 重试失败：{error}"
- [ ] 6.4 确保日志消息包含足够上下文（Story ID、重试次数等）

## 7. 监控与健康检查

- [ ] 7.1 实现 `SchedulerStatus` 结构体，包含运行状态、活跃重试数量等信息
- [ ] 7.2 在 `AgentMonitor` 面板中添加调度器状态展示组件
- [ ] 7.3 记录每次扫描的耗时和找到的 Story 数量，便于性能分析

## 8. 测试与验证

- [ ] 8.1 编写集成测试，验证完整的失败→调度→重试→成功流程
- [ ] 8.2 测试并发控制：同时有 5 个待重试 Story，验证最多只有 3 个同时执行
- [ ] 8.3 测试优雅停止：在重试任务执行中关闭 Worker，验证任务完成后再退出
- [ ] 8.4 手动测试：创建一个失败的 Story，等待自动重试，验证端到端流程
- [ ] 8.5 性能测试：模拟 100 个待重试 Story，验证扫描和触发的性能

## 9. 文档更新

- [ ] 9.1 更新 `AGENT_CREATION.md` 或相关文档，说明重试调度器的工作原理
- [ ] 9.2 在代码中添加必要的注释，解释关键逻辑和设计决策
- [ ] 9.3 更新 README 或用户文档，说明如何配置重试参数（如需要）

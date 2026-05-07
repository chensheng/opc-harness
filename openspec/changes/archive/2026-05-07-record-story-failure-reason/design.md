## Context

当前 `AgentWorker` 在执行用户故事时，通过 `RetryEngine` 对 AI CLI 进程的错误进行分类（临时/永久）。当判定为永久失败时，系统会调用 `db::fail_user_story` 更新状态，但仅记录了通用的错误消息，缺乏结构化的失败详情。

## Goals / Non-Goals

**Goals:**
1. 在数据库中持久化存储失败的详细原因（如 CLI 退出码、标准错误输出摘要）。
2. 确保前端能够实时或刷新后看到具体的失败反馈。
3. 保持向后兼容，不影响现有的成功路径逻辑。

**Non-Goals:**
1. 不实现复杂的错误自动修复功能。
2. 不改变现有的重试调度器（RetryScheduler）的核心算法。

## Decisions

### Decision 1: 数据库字段扩展
**选择：** 在 `user_stories` 表中增加 `failure_reason` (TEXT) 和 `last_error_timestamp` (DATETIME)。
**理由：** 
- `failure_reason` 用于存储人类可读的错误描述（例如：“Kimi CLI 不支持 --story-id 参数”）。
- 避免修改核心状态机，仅作为状态为 `failed` 时的补充信息。

### Decision 2: 错误捕获位置
**选择：** 在 `agent_worker.rs` 的 `execute_story` 函数末尾，根据 `RetryDecision` 的结果统一收集错误信息。
**理由：** 
- 此时已经完成了所有的重试尝试，可以确定最终的失败结论。
- 可以访问到完整的 `std::process::Output` 对象，提取 `stderr`。

### Decision 3: 前端展示方式
**选择：** 在 `AgentMonitor.tsx` 的智能体卡片中，如果状态为 `failed`，则展开显示红色的错误详情块。
**理由：** 
- 提供即时的视觉反馈，减少开发者排查问题的时间。

## Risks / Trade-offs

- **[Risk] 错误信息过长**: CLI 的 `stderr` 可能包含大量堆栈信息。
  - **Mitigation**: 在后端截断并只保留最后 500 字符或关键错误行。
- **[Risk] 数据库迁移**: 现有项目需要执行 SQL 迁移脚本。
  - **Mitigation**: 在应用启动时自动检测并执行 `ALTER TABLE`。

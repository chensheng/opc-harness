## Why

当前智能体在执行用户故事（User Story）失败时，虽然会在后端日志中记录错误原因（如 Kimi CLI 参数不匹配、进程退出状态等），但这些关键的失败信息并未持久化存储到数据库的用户故事记录中。这导致：
1. 开发者无法通过前端界面直接查看任务失败的具体原因。
2. 缺乏历史失败数据，难以进行后续的自动化重试策略优化或根因分析。
3. 故障排查效率低，需要手动翻阅复杂的后端日志文件。

## What Changes

- **增强错误分类与捕获**：在 `AgentWorker` 执行循环中，更精细地捕获 AI CLI 进程的退出状态和标准错误输出。
- **数据库模型扩展**：在 `user_stories` 表中增加 `failure_reason` 和 `error_details` 字段，用于存储结构化的失败信息。
- **状态更新逻辑修改**：当智能体判定任务为“永久失败”（Permanent Error）或达到最大重试次数时，将收集到的错误原因同步写入数据库。
- **前端展示集成**：在 Agent Monitor 或 User Story 详情面板中展示失败原因，提供清晰的故障反馈。

## Capabilities

### New Capabilities
- `story-failure-tracking`: 负责捕获、分类并持久化存储用户故事执行过程中的失败原因。

### Modified Capabilities
- `coding-harness`: 修改智能体执行循环的错误处理分支，确保在任务终止前调用失败记录逻辑。

## Impact

**受影响的代码：**
- `src-tauri/src/agent/agent_worker.rs` - 核心执行与错误捕获逻辑。
- `src-tauri/src/db/schema.rs` & `src-tauri/src/models/user_story.rs` - 数据库模型定义。
- `src-tauri/src/db/mod.rs` - 增加更新失败原因的数据库操作方法。
- `src/components/vibe-coding/AgentMonitor.tsx` - 前端展示失败详情。

**影响范围：**
- 所有通过去中心化 Worker 执行的用户故事。
- 现有的数据库需要进行迁移以支持新字段。

**API 变化：**
- 无对外 API 变化，属于内部数据流增强。

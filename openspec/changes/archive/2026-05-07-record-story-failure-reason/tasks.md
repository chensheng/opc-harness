## 1. 数据库迁移与模型更新

- [x] 1.1 在 `src-tauri/src/db/database.rs` 中为 `user_stories` 表添加 `failure_reason` (TEXT) 和 `last_error_timestamp` (DATETIME) 字段
- [x] 1.2 更新 `src-tauri/src/models/mod.rs` 中的 `UserStory` 结构体以包含新字段
- [x] 1.3 在 `src-tauri/src/db/database.rs` 中实现自动迁移逻辑，确保现有数据库能平滑升级

## 2. 后端错误捕获与持久化

- [x] 2.1 修改 `src-tauri/src/agent/agent_worker.rs`，在 `execute_story` 失败分支中提取 CLI 的 `stderr` 和退出码
- [x] 2.2 在 `src-tauri/src/db/sprint_repository.rs` 中增加 `update_story_failure_reason` 方法（已在 `fail_user_story` 中实现）
- [x] 2.3 集成错误分类逻辑，确保“永久失败”时调用新的数据库更新方法

## 3. 前端展示集成

- [x] 3.1 在 `src/components/vibe-coding/AgentMonitor.tsx` 中增强智能体卡片，当状态为 `failed` 时显示错误详情块
- [x] 3.2 确保前端从后端获取的用户故事数据中包含 `failure_reason` 字段并正确渲染

## 4. 验证与测试

- [x] 4.1 手动触发一个已知会失败的 Story（如使用错误的 Kimi CLI 参数），验证数据库中是否记录了原因
- [x] 4.2 检查前端界面是否能正确显示红色的错误提示信息

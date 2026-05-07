## 1. 清理 Commands 目录的未使用 Imports

- [x] 1.1 移除 `src-tauri/src/commands/observability.rs` 中未使用的 repository imports (AgentAlertsRepository, AgentLogsRepository, AgentTracesRepository)
- [x] 1.2 移除 `src-tauri/src/commands/quality/mod.rs` 中未使用的 `quality_check::*` re-export
- [x] 1.3 移除 `src-tauri/src/commands/retry.rs` 中未使用的 `tauri::State` import
- [x] 1.4 验证 commands/ 目录编译无警告

## 2. 清理 DB 和 Services 目录的未使用 Imports

- [x] 2.1 移除 `src-tauri/src/db/mod.rs` 中未使用的 retry 函数 exports (get_scheduled_retry_stories, update_user_story_next_retry_at)
- [x] 2.2 移除 `src-tauri/src/services/observability_service.rs` 中未使用的 repository imports
- [x] 2.3 移除其他文件中未使用的 rusqlite::Connection, Duration, sleep, Child, Stdio, AsyncWriteExt 等 imports (部分完成)
- [x] 2.4 验证 db/ 和 services/ 目录编译无警告

## 3. 修复 Agent Worker 中的未使用变量

- [x] 3.1 分析 `src-tauri/src/agent/agent_worker.rs` 中所有未使用的 `app_handle` 变量
- [x] 3.2 将预留但未使用的参数重命名为 `_app_handle` 前缀
- [x] 3.3 移除确实不需要的未使用变量
- [ ] 3.4 为计划未来使用的变量添加 TODO 注释
- [ ] 3.5 验证 agent/ 目录编译无警告

## 4. 修复 Unreachable Pattern 警告

- [x] 4.1 定位 unreachable pattern 警告的具体位置
- [x] 4.2 分析 match 表达式逻辑,确定是否需要该分支
- [x] 4.3 移除冗余的 match arms 或调整 pattern 顺序
- [ ] 4.4 验证逻辑正确性,确保不改变业务行为

## 5. 清理 Cargo.toml Manifest 警告

- [x] 5.1 检查 `src-tauri/Cargo.toml` 中的 `build` key 警告
- [x] 5.2 确定 build key 的正确位置或删除它
- [x] 5.3 验证 manifest 格式正确

## 6. 最终验证和测试

- [ ] 6.1 运行 `cargo check` 确认 0 warnings
- [ ] 6.2 运行 `cargo test` 确保所有测试通过
- [ ] 6.3 运行 `npm run harness:check` 验证健康评分提升
- [ ] 6.4 启动 Tauri 开发环境验证功能正常
- [ ] 6.5 提交所有变更到 Git

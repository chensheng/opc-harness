## 1. 工具集扩展 - 代码搜索

- [x] 1.1 创建 `src-tauri/src/agent/tools/code_search.rs`，实现 CodeSearchTools 结构体
- [x] 1.2 实现 `grep(pattern, path?)` 方法，支持正则表达式搜索并返回匹配结果
- [x] 1.3 实现 `find_files(pattern, extensions?)` 方法，支持 glob 模式文件查找
- [x] 1.4 实现 `find_symbol(symbol_name)` 方法，支持符号定位（可选：使用 tree-sitter）
- [x] 1.5 添加路径安全验证，防止访问工作空间外的文件
- [x] 1.6 在 `native_coding_agent.rs` 中集成 code_search 工具到 execute_tool_call
- [x] 1.7 更新系统提示词，告知 AI 可用的代码搜索工具
- [x] 1.8 编写单元测试：测试 grep、find_files、路径安全检查

## 2. 工具集扩展 - 依赖管理

- [x] 2.1 创建 `src-tauri/src/agent/tools/dependency_manager.rs`，实现 DependencyManager 结构体
- [x] 2.2 实现 `npm_install(package, version?)` 方法，支持 npm 包安装
- [x] 2.3 实现 `cargo_add(crate, features?)` 方法，支持 Rust crate 添加
- [x] 2.4 实现 `list_dependencies()` 方法，解析 package.json/Cargo.toml
- [x] 2.5 添加包名合法性验证（仅允许官方 registry）
- [x] 2.6 在 `native_coding_agent.rs` 中集成 dependency 工具
- [x] 2.7 更新系统提示词，告知 AI 可用的依赖管理工具
- [x] 2.8 编写单元测试：Mock npm/cargo 命令，测试包名验证逻辑

## 3. HITL Checkpoint 机制 - 后端实现

- [x] 3.1 创建数据库迁移文件 `migrations/005_create_agent_checkpoints.sql`
- [x] 3.2 定义 agent_checkpoints 表结构（id, agent_id, story_id, type, status, data, feedback, timestamps）
- [x] 3.3 创建 `src-tauri/src/agent/checkpoint_manager.rs`，实现 CheckpointManager
- [x] 3.4 实现 `create_checkpoint(type, data)` 方法，创建 checkpoint 记录
- [x] 3.5 实现 `resolve_checkpoint(id, decision, feedback)` 方法，处理用户决策
- [x] 3.6 实现 WebSocket 事件推送：checkpoint_created, checkpoint_resolved
- [x] 3.7 实现超时机制：30 分钟无响应自动标记为 timeout
- [x] 3.8 在 `native_coding_agent.rs` 中添加 checkpoint 触发点（代码生成后、依赖安装前）
- [x] 3.9 实现阻塞等待逻辑：wait_for_checkpoint_decision(checkpoint_id)
- [x] 3.10 实现回滚机制：当用户拒绝时撤销代码更改

## 4. HITL Checkpoint 机制 - 前端实现

- [ ] 4.1 创建 `src/components/vibe-coding/CheckpointApprovalDialog.tsx` 组件
- [ ] 4.2 集成 react-diff-viewer 显示代码差异
- [ ] 4.3 实现批准/拒绝按钮及反馈输入框
- [ ] 4.4 通过 WebSocket 监听 checkpoint_created 事件
- [ ] 4.5 实现决策提交：调用后端 API `/api/checkpoints/{id}/resolve`
- [ ] 4.6 添加批量审批功能："全部批准"按钮
- [ ] 4.7 显示待审批 checkpoints 数量徽章
- [ ] 4.8 处理超时情况：显示超时警告并提供重新触发选项

## 5. Worktree 自动清理

- [ ] 5.1 创建 `src-tauri/src/agent/worktree_lifecycle.rs`，实现 WorktreeLifecycleManager
- [ ] 5.2 实现 `cleanup_after_story(agent_id, story_id, outcome)` 方法
- [ ] 5.3 删除 worktree 目录（tokio::fs::remove_dir_all）
- [ ] 5.4 移除 Git worktree 引用（git worktree remove）
- [ ] 5.5 在 `agent_worker.rs` 的 execute_native_agent 结束时调用清理
- [ ] 5.6 确保成功和失败两种情况下都执行清理
- [ ] 5.7 实现清理失败重试机制：后台任务定期重试（最多 3 次）
- [ ] 5.8 添加配置选项 `preserve_worktree_branches`（默认 false）
- [ ] 5.9 如果启用保留分支，在清理前创建永久分支
- [ ] 5.10 编写集成测试：验证 Story 完成后 worktree 被删除

## 6. 对话历史优化

- [ ] 6.1 在 `native_coding_agent.rs` 的系统提示词中添加 `<TASK_COMPLETE>` 标记说明
- [ ] 6.2 实现 `parse_completion_signal(response)` 方法，检测完成标记
- [ ] 6.3 修改多轮循环逻辑：检测到完成信号后立即 break
- [ ] 6.4 实现 `compress_history()` 方法，压缩早期对话
- [ ] 6.5 保留 system message + 最近 4 条消息
- [ ] 6.6 生成对话摘要：总结 Turn 1-N 的关键操作和错误
- [ ] 6.7 每 5 轮对话后自动触发压缩
- [ ] 6.8 添加配置选项 `enable_history_compression`（默认 true）
- [ ] 6.9 测量压缩前后的 token 数量，验证减少至少 60%
- [ ] 6.10 编写单元测试：测试历史压缩逻辑和 token 计算

## 7. 质量检查改进

- [ ] 7.1 修改 `src-tauri/src/agent/tools/quality.rs`，添加分阶段检查方法
- [ ] 7.2 实现 `run_quality_checks_staged()` 方法，按顺序执行 lint → type-check → test
- [ ] 7.3 任一阶段失败时立即返回，不进行后续检查
- [ ] 7.4 返回详细的错误位置（文件路径 + 行号）
- [ ] 7.5 实现 `run_incremental_lint(modified_files)` 方法，只检查修改的文件
- [ ] 7.6 使用 ESLint `--cache` 选项或手动过滤文件列表
- [ ] 7.7 在 `native_coding_agent.rs` 中替换原有的 run_quality_checks 调用
- [ ] 7.8 测量增量检查的性能提升（目标：减少 50%+ 时间）
- [ ] 7.9 编写单元测试：测试分阶段检查和增量检查逻辑

## 8. 配置和向后兼容性

- [ ] 8.1 在 `NativeAgentConfig` 中添加新功能开关字段
- [ ] 8.2 所有新功能的默认值为 false（保持向后兼容）
- [ ] 8.3 更新配置文件示例（`.env.example`）添加新配置项说明
- [ ] 8.4 在 Settings UI 中添加新功能开关（可选，高级设置）
- [ ] 8.5 编写文档：说明每个配置项的作用和推荐值

## 9. 测试和验证

- [ ] 9.1 运行 `cargo test` 确保所有单元测试通过
- [ ] 9.2 运行 `npm run test` 确保前端测试通过
- [ ] 9.3 手动测试 HITL 流程：启动 Agent → 触发 checkpoint → 批准/拒绝 → 验证行为
- [ ] 9.4 手动测试 worktree 清理：执行 Story → 验证 worktree 被删除
- [ ] 9.5 性能测试：测量对话历史压缩的 token 节省效果
- [ ] 9.6 性能测试：测量增量质量检查的时间节省效果
- [ ] 9.7 运行 `npm run harness:check` 确保 Health Score = 100/100
- [ ] 9.8 启动 Tauri 开发环境，验证无运行时错误
- [ ] 9.9 端到端测试：完整执行一个 User Story，验证所有新功能协同工作

## 10. 文档和归档

- [ ] 10.1 更新 README.md，添加新功能说明
- [ ] 10.2 创建质量检查报告 quality-check.md
- [ ] 10.3 创建运行时验证报告 runtime-check.md
- [ ] 10.4 准备归档：同步 delta specs 到主 specs
- [ ] 10.5 归档变更到 openspec/changes/archive/

## 1. 数据库迁移

- [x] 1.1 在 `src-tauri/src/db/database.rs` 中添加 `user_story_retry_history` 表的创建逻辑
- [x] 1.2 在 `user_stories` 表中添加 `next_retry_at` 字段（TEXT, 可选）
- [x] 1.3 在 `user_stories` 表中添加 `max_retries` 字段（INTEGER, 默认值 3）
- [x] 1.4 编写数据库迁移脚本，确保向后兼容（检查字段是否存在）
- [x] 1.5 测试数据库迁移，验证新表和字段正确创建

## 2. 数据模型扩展

- [x] 2.1 在 `src-tauri/src/models/mod.rs` 中新增 `UserStoryRetryHistory` 结构体
- [x] 2.2 更新 `UserStory` 结构体，添加 `next_retry_at` 和 `max_retries` 字段
- [x] 2.3 实现 `UserStoryRetryHistory` 的 Entity trait（from_row 方法）
- [x] 2.4 在前端 `src/types/index.ts` 中新增 `UserStoryRetryHistory` 类型定义
- [x] 2.5 更新前端 `UserStory` 类型，添加重试相关可选字段

## 3. 数据库 Repository 层

- [x] 3.1 在 `src-tauri/src/db/sprint_repository.rs` 中新增 `create_retry_history_record` 函数
- [x] 3.2 新增 `update_retry_history_result` 函数，更新重试结果
- [x] 3.3 新增 `get_user_story_retry_history` 函数，查询指定 Story 的重试历史
- [x] 3.4 新增 `get_project_retry_statistics` 函数，获取项目级别的重试统计
- [x] 3.5 新增 `update_user_story_next_retry_at` 函数，更新下次重试时间
- [x] 3.6 新增 `get_scheduled_retry_stories` 函数，查询待重试的 Story 列表
- [ ] 3.7 为所有新增函数编写单元测试

## 4. 重试引擎核心实现

- [x] 4.1 创建 `src-tauri/src/agent/retry_engine.rs` 文件
- [x] 4.2 实现 `ErrorClassifier` 结构体和错误分类逻辑（基于正则表达式）
- [x] 4.3 实现 `BackoffStrategy` 结构体和指数退避算法
- [x] 4.4 实现 `RetryEngine` 结构体，包含 `should_retry` 决策方法
- [x] 4.5 实现 `RetryDecision` 枚举类型（Retry/Abort）
- [x] 4.6 编写重试引擎的单元测试，覆盖各种错误类型和边界情况

## 5. 重试调度器实现

- [x] 5.1 在 `src-tauri/src/agent/retry_engine.rs` 中实现 `RetryScheduler` 结构体
- [x] 5.2 实现定时检查逻辑（每 30 秒扫描待重试队列）
- [x] 5.3 实现并发控制，限制同时重试的 Story 数量（最多 3 个）
- [ ] 5.4 实现重试任务的触发逻辑，调用 Agent Worker 执行
- [ ] 5.5 实现调度器的启动和停止方法
- [ ] 5.6 编写调度器的集成测试

## 6. Agent Worker 集成

- [x] 6.1 修改 `src-tauri/src/agent/agent_worker.rs` 中的 `execute_user_story` 方法
- [x] 6.2 在执行失败时调用重试引擎的 `should_retry` 方法
- [x] 6.3 根据决策结果更新 Story 状态（scheduled_retry/permanently_failed）
- [x] 6.4 创建重试历史记录并保存到数据库
- [ ] 6.5 在 Agent Worker 初始化时启动重试调度器
- [ ] 6.6 在 Agent Worker 关闭时优雅停止调度器
- [ ] 6.7 测试完整的失败→重试→成功/失败流程

## 7. Tauri Commands 新增

- [x] 7.1 创建 `src-tauri/src/commands/retry.rs` 文件
- [x] 7.2 实现 `get_user_story_retry_history` Command
- [x] 7.3 实现 `update_user_story_retry_config` Command
- [x] 7.4 实现 `get_project_retry_statistics` Command
- [x] 7.5 在 `src-tauri/src/commands/mod.rs` 中注册新 Commands
- [x] 7.6 在 `src-tauri/src/main.rs` 或 `lib.rs` 中导出 Commands
- [ ] 7.7 编写 Commands 的集成测试

## 8. 前端状态管理扩展

- [x] 8.1 在 `src/stores/userStoryStore.ts` 中新增 `loadRetryHistory` action
- [x] 8.2 新增 `updateRetryConfig` action，更新项目的重试配置
- [ ] 8.3 新增 `getRetryStatistics` action，获取重试统计数据
- [x] 8.4 在 store 中添加 `retryHistoryByStory` 状态字段
- [x] 8.5 更新 `UserStory` 类型的映射逻辑，处理新增字段
- [ ] 8.6 编写 Zustand store 的单元测试

## 9. 前端 UI 组件开发

- [x] 9.1 创建 `src/components/vibe-coding/RetryHistoryTimeline.tsx` 组件
- [x] 9.2 实现重试历史时间线展示，支持成功/失败/待处理的视觉区分
- [x] 9.3 在 `UserStoryTable.tsx` 中添加"查看重试历史"按钮
- [x] 9.4 在用户故事卡片中显示重试次数徽章
- [ ] 9.5 显示预计下次重试时间（对于 scheduled_retry 状态的 Story）
- [x] 9.6 创建 `src/components/vibe-coding/RetryConfigPanel.tsx` 组件
- [ ] 9.7 在项目设置页面集成重试配置面板
- [ ] 9.8 实现配置表单，支持修改最大重试次数、延迟时间等参数

## 10. 前端 API 调用封装

- [x] 10.1 在 `src/hooks/useUserStoryDecomposition.ts` 或新建 hook 中封装重试相关 API
- [x] 10.2 实现 `useRetryHistory` hook，提供重试历史查询功能
- [x] 10.3 实现 `useRetryConfig` hook，提供重试配置的读取和更新
- [x] 10.4 处理 API 调用的加载状态和错误处理
- [ ] 10.5 编写 hooks 的单元测试

## 11. 错误处理和边界情况

- [ ] 11.1 处理数据库查询失败的情况，提供友好的错误提示
- [ ] 11.2 处理重试配置无效输入的情况，进行前端验证
- [ ] 11.3 处理并发重试冲突的情况，添加锁机制或乐观锁
- [ ] 11.4 处理 Story 在等待重试期间被手动删除的情况
- [ ] 11.5 处理系统重启后待重试任务的恢复逻辑

## 12. 测试与验证

- [ ] 12.1 编写端到端测试，模拟用户故事失败→自动重试→成功的完整流程
- [x] 12.2 测试临时错误（网络超时）的自动重试行为
- [x] 12.3 测试永久错误（代码错误）的不重试行为
- [x] 12.4 测试超过最大重试次数后的终止逻辑
- [x] 12.5 测试指数退避算法的正确性（验证延迟时间计算）
- [x] 12.6 测试前端 UI 正确显示重试历史和状态
- [ ] 12.7 测试重试配置的保存和加载功能
- [x] 12.8 运行 TypeScript 编译检查，确保没有类型错误
- [x] 12.9 运行 ESLint 和 Prettier 检查
- [x] 12.10 运行 Rust cargo check 和 cargo clippy

## 13. 文档更新

- [ ] 13.1 更新 `openspec/specs/vibe-coding/spec.md`，同步 delta specs
- [ ] 13.2 更新 `openspec/specs/agent-initialization/spec.md`，同步 delta specs
- [ ] 13.3 在用户指南中添加重试功能的使用说明
- [ ] 13.4 在 API 文档中记录新增的 Commands
- [ ] 13.5 更新架构文档，说明重试引擎的设计和数据流

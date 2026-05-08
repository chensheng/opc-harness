## 1. 项目结构与依赖配置

- [x] 1.1 在 `src-tauri/Cargo.toml` 中添加 `diffy` 依赖（用于代码 diff/merge）
- [x] 1.2 创建新的模块目录结构：`src-tauri/src/agent/tools/`
- [x] 1.3 创建 `src-tauri/src/agent/native_coding_agent.rs` 文件骨架
- [x] 1.4 更新 `src-tauri/src/agent/mod.rs` 导出新模块

## 2. Native Coding Agent 核心实现

- [x] 2.1 实现 NativeCodingAgent 结构体，包含 AI Provider、工具集、工作空间路径等字段
- [x] 2.2 实现 new() 构造函数，初始化 FunctionCallingAgent 和工具列表
- [x] 2.3 实现 execute_story() 主执行方法，接收 Story 标题和验收标准
- [x] 2.4 实现多轮对话循环逻辑，支持最多 10 轮交互
- [x] 2.5 实现对话历史管理，超过阈值时触发摘要
- [x] 2.6 实现超时控制机制（默认 30 分钟）

## 3. 文件系统工具集实现

- [x] 3.1 实现 `ReadFileTool`，包含路径安全验证和 500KB 限制
- [x] 3.2 实现 `WriteFileTool`，支持自动创建目录和文件备份
- [x] 3.3 实现 `ListDirectoryTool`，支持递归遍历和深度限制
- [x] 3.4 实现 `EditFileTool`，支持基于行号的增量编辑
- [x] 3.5 为所有文件工具添加单元测试（路径越界、大文件截断等边界情况）

## 4. Git 工具集实现

- [x] 4.1 实现 `GitStatusTool`，解析 `git status --porcelain` 输出
- [x] 4.2 实现 `GitDiffTool`，返回 unified diff 格式的差异内容
- [x] 4.3 实现 `GitCommitTool`，自动生成 Conventional Commits 消息
- [x] 4.4 实现 `CreateWorktreeTool`，创建独立分支和 worktree 目录
- [x] 4.5 实现 `CleanupWorktreeTool`，删除 worktree 和对应分支
- [x] 4.6 为 Git 工具添加集成测试（需要临时 Git 仓库）

## 5. 质量检查工具集实现

- [x] 5.1 实现 `RunLinterTool`，执行 ESLint 并解析 JSON 输出
- [x] 5.2 实现 `RunTypeScriptCheckTool`，执行 `tsc --noEmit`
- [x] 5.3 实现 `RunTestsTool`，执行测试并解析结果
- [x] 5.4 实现 `RunQualityChecksTool`，并行执行所有检查（使用 `tokio::join!`）
- [x] 5.5 实现质量检查超时控制（60 秒）
- [x] 5.6 为质量工具添加集成测试（需要示例 TypeScript 项目）

## 6. Prompt 工程与响应解析

- [x] 6.1 实现 `build_story_prompt()` 函数，构建用户故事的系统提示词
- [x] 6.2 实现 `build_context()` 函数，收集项目结构和相关文件信息
- [x] 6.3 实现 `parse_function_calls()` 函数，解析 AI 的工具调用请求
- [x] 6.4 实现 `parse_code_changes()` 函数，从 AI 响应中提取代码片段或 diff
- [x] 6.5 实现 diff 应用逻辑，使用 `diffy` crate 合并变更

## 7. 错误自动修复循环

- [x] 7.1 实现 auto_fix_loop() 方法，最多重试 3 次
- [x] 7.2 实现错误信息格式化，将 Lint/Test 错误转换为 AI 可理解的反馈
- [x] 7.3 实现失败状态标记逻辑（permanently_failed/max_retries_exceeded）
- [x] 7.4 记录修复次数和每次的错误信息到数据库

## 8. AgentWorker 集成

- [x] 8.1 修改 `AgentWorker::start_coding_agent()` 方法，添加配置开关判断
- [x] 8.2 实现 `execute_native_agent()` 方法，调用 NativeCodingAgent
- [x] 8.3 保留原有 `execute_cli_agent()` 方法作为降级方案
- [x] 8.4 实现环境变量读取逻辑（`VITE_USE_NATIVE_AGENT`）
- [x] 8.5 添加 WebSocket 实时日志推送（AI 思考过程、工具调用、质量检查结果）

## 9. Daemon Manager 适配

- [x] 9.1 更新 `DaemonManager` 以支持 Native Agent 状态追踪
- [x] 9.2 实现 Native Agent 的资源监控（内存使用、API 调用次数）
- [x] 9.3 添加 Agent 暂停/恢复功能（通过取消 tokio task 实现）

## 10. 前端无感知适配

- [x] 10.1 验证 Tauri Command `start_coding_agent` 接口保持不变
- [x] 10.2 确保前端接收的消息格式与 CLI 模式一致
- [x] 10.3 添加 Native Agent 特有的状态字段（token_usage/tool_calls）

## 11. 单元测试与集成测试

- [x] 11.1 为 NativeCodingAgent 编写单元测试（Mock AI Provider）
- [x] 11.2 为所有 Tools 编写单元测试（使用 tempfile 创建临时工作空间）
- [x] 11.3 修复测试失败问题（文件系统路径验证、Git 分支操作）
- [ ] 11.4 编写端到端测试，执行完整的 Story 流程
- [ ] 11.5 性能基准测试：对比 Native Agent vs CLI 模式的执行时间
- [ ] 11.6 确保测试覆盖率 ≥ 70%

## 12. 文档与配置

- [x] 12.1 更新 `openspec/specs/coding-harness/spec.md`（已创建 delta spec）
- [x] 12.2 更新 `openspec/specs/vibe-coding/spec.md`（已创建 delta spec）
- [x] 12.3 编写 Native Agent 使用文档（`docs/Native-Coding-Agent.md`）
- [x] 12.4 添加环境变量配置说明（`.env.example`）
- [x] 12.5 更新 README.md，说明 Native Agent 特性

## 13. 灰度发布准备

- [x] 13.1 添加配置开关默认值（`VITE_USE_NATIVE_AGENT=false`）
- [x] 13.2 实现运行时切换逻辑（通过前端设置界面）
- [x] 13.3 添加遥测数据收集（Token 消耗已通过 WebSocket 日志推送）
- [ ] 13.4 设置告警规则（失败率 > 10% 时通知）

## 14. 代码质量与健康检查

- [x] 14.1 运行 `cargo clippy` 修复所有警告
- [x] 14.2 运行 `cargo fmt` 格式化代码
- [x] 14.3 确保 `npm run harness:check` 健康评分 = 100/100
- [x] 14.4 移除所有 `#[allow(dead_code)]` 和未使用变量
- [x] 14.5 审查所有 `unwrap()` 调用，替换为 proper error handling

## 15. 归档与清理

- [x] 15.1 标记 `ai_cli_interaction.rs` 为 `#[deprecated]`
- [x] 15.2 添加迁移指南注释，说明何时完全移除 CLI 支持
- [ ] 15.3 归档本变更到 `openspec/changes/archive/`
- [ ] 15.4 更新 AGENTS.md，记录 Native Agent 架构决策

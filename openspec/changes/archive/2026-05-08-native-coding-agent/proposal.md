## Why

当前 Vibe Coding 智能体依赖外部 CLI 工具（Kimi CLI、Claude Code CLI）执行编码任务，存在以下核心问题：

1. **厂商锁定**：每个 CLI 工具的参数格式完全不同，难以统一抽象
2. **环境依赖**：用户必须先安装第三方 CLI 工具，增加使用门槛
3. **调试困难**：STDIO 通信难以追踪，错误信息不透明，Kimi CLI 的 `--story-id` 参数不被支持导致任务失败
4. **版本碎片**：CLI 工具版本升级可能破坏兼容性，维护成本高

通过自研 Native Coding Agent，直接使用 AI Provider API（OpenAI/Anthropic/Kimi/GLM），可以彻底摆脱对外部 CLI 的依赖，实现完全可控的编码智能体架构。

## What Changes

- **移除 CLI 依赖**：不再启动 Kimi/Claude CLI 子进程，改为直接调用 AI API
- **新增 Native Agent 核心**：实现基于 Function Calling 的自主编码智能体
- **新增文件系统工具集**：提供 read_file、write_file、edit_file、list_directory 等安全文件操作工具
- **新增 Git 集成工具**：提供 git_status、git_diff、git_commit、git_worktree 等版本控制工具
- **新增代码质量工具**：集成 ESLint、TypeScript Check、测试运行器等质量检查能力
- **重构 AgentWorker**：将 CLI 启动逻辑替换为 Native Agent 执行逻辑
- **保留向后兼容**：通过配置开关支持 CLI 模式作为降级方案

## Capabilities

### New Capabilities

- `native-coding-agent`: 纯 Rust 实现的自主编码智能体核心，支持多轮对话、工具调用、增量代码编辑
- `agent-file-system-tools`: 安全的文件系统操作工具集，支持工作空间沙箱隔离
- `agent-git-tools`: Git 版本控制工具集，支持分支管理、差异对比、提交操作
- `agent-quality-tools`: 代码质量检查工具集，支持 Linter、Type Check、Test Runner

### Modified Capabilities

- `coding-harness`: 修改 Worktree 隔离环境管理要求，从 CLI 驱动改为 Native Agent 驱动
- `vibe-coding`: 修改进度追踪与可视化要求，增加 Native Agent 执行状态监控

## Impact

**受影响的代码模块**:
- `src-tauri/src/agent/agent_worker.rs` - 核心执行逻辑重构
- `src-tauri/src/agent/daemon_core.rs` - Daemon 管理器适配
- `src-tauri/src/agent/ai_cli_interaction.rs` - 标记为废弃（保留作为降级方案）

**新增代码模块**:
- `src-tauri/src/agent/native_coding_agent.rs` - Native Agent 核心实现
- `src-tauri/src/agent/tools/filesystem.rs` - 文件系统工具
- `src-tauri/src/agent/tools/git.rs` - Git 工具
- `src-tauri/src/agent/tools/quality.rs` - 质量检查工具
- `src-tauri/src/agent/prompt_engine.rs` - Prompt 工程模块
- `src-tauri/src/agent/response_parser.rs` - AI 响应解析器

**API 变更**:
- Tauri Command `start_coding_agent` 的参数和返回值保持不变（前端无感知）
- 内部实现从 CLI 进程管理切换为 Native Agent 执行

**依赖变更**:
- 新增 `tokio` (已有)
- 新增 `async-trait` (已有)
- 新增 `diffy` (用于代码 diff/merge)
- 移除对 `kimi`、`claude` CLI 二进制文件的运行时依赖

**BREAKING**: 无（保持向后兼容，通过配置开关渐进式迁移）

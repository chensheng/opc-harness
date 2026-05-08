# Spec: Agent Code Search Capability

## Requirement: 代码搜索工具集

Native Coding Agent SHALL 提供代码搜索能力，使 AI 能够主动探索和理解代码库结构。Agent MUST 支持正则表达式搜索、文件查找和符号定位三种搜索模式。

### Scenario: 正则表达式搜索

- **WHEN** AI 需要查找符合特定模式的代码
- **THEN** Agent 调用 `code_search_grep(pattern, path?)` 工具
- **AND** 返回匹配的行号、内容和上下文（前后各 2 行）
- **AND** 限制单次搜索结果不超过 50 条匹配项

### Scenario: 文件查找

- **WHEN** AI 需要查找特定类型的文件
- **THEN** Agent 调用 `code_search_find_files(pattern, extensions?)` 工具
- **AND** 支持 glob 模式匹配（如 `*.ts`, `src/**/*.tsx`）
- **AND** 可指定文件扩展名过滤

### Scenario: 符号定位

- **WHEN** AI 需要查找函数、类或变量的定义位置
- **THEN** Agent 调用 `code_search_find_symbol(symbol_name)` 工具
- **AND** 返回符号的定义位置（文件路径 + 行号）
- **AND** 如果存在多个定义，返回所有位置供 AI 选择

### Scenario: 路径安全验证

- **WHEN** AI 尝试搜索工作空间外的路径
- **THEN** Agent 拒绝执行并返回错误："Access denied: path is outside workspace"
- **AND** 记录安全警告日志

---

# Spec: Agent Dependency Management Capability

## Requirement: 依赖管理工具集

Native Coding Agent SHALL 提供依赖管理能力，使 AI 能够安装和管理项目依赖。Agent MUST 支持 npm 和 cargo 两种包管理器，并确保安全性。

### Scenario: 安装 npm 包

- **WHEN** AI 需要添加前端依赖
- **THEN** Agent 调用 `dependency_npm_install(package, version?)` 工具
- **AND** 验证包名合法性（仅允许字母、数字、连字符、下划线）
- **AND** 执行 `npm install <package>@<version>` 或 `npm install <package>`（无版本时安装最新）
- **AND** 返回安装的版本号和建议的 import 语句

### Scenario: 添加 Rust crate

- **WHEN** AI 需要添加后端依赖
- **THEN** Agent 调用 `dependency_cargo_add(crate, features?)` 工具
- **AND** 验证 crate 名称合法性
- **AND** 执行 `cargo add <crate> --features <features>`（如有特性）
- **AND** 返回添加的 crate 版本和 Cargo.toml 变更

### Scenario: 列出当前依赖

- **WHEN** AI 需要了解项目现有依赖
- **THEN** Agent 调用 `dependency_list()` 工具
- **AND** 解析 package.json 或 Cargo.toml
- **AND** 返回依赖列表（名称 + 版本）

### Scenario: 依赖安装安全检查

- **WHEN** AI 尝试安装可疑包（如包含脚本的包）
- **THEN** Agent 发出警告并请求用户确认（通过 HITL checkpoint）
- **AND** 记录审计日志到数据库

---

# Spec: Agent HITL Checkpoint Capability

## Requirement: Human-in-the-Loop 检查点机制

Native Coding Agent SHALL 在关键决策点暂停执行，等待用户审核和批准。System MUST 提供 checkpoint 创建、审批和恢复的完整流程。

### Scenario: 代码审查检查点

- **WHEN** AI 完成代码生成并准备提交
- **AND** `enable_hitl` 配置为 true
- **THEN** Agent 调用 checkpoint_manager.create_checkpoint("code_review", diff_data)
- **AND** 通过 WebSocket 发送事件到前端
- **AND** 阻塞执行，等待用户决策
- **AND** 超时时间：30 分钟（可配置）

### Scenario: 用户批准更改

- **WHEN** 用户在前端点击"批准"按钮
- **THEN** 前端发送决策到后端 `/api/checkpoints/{id}/resolve`
- **AND** CheckpointManager 更新状态为 "approved"
- **AND** 恢复 Agent 执行
- **AND** 记录用户反馈（如有）

### Scenario: 用户拒绝更改

- **WHEN** 用户在前端点击"拒绝"按钮并提供反馈
- **THEN** CheckpointManager 更新状态为 "rejected"
- **AND** Agent 回滚当前更改
- **AND** 将用户反馈添加到对话历史
- **AND** Agent 尝试重新生成或终止执行

### Scenario: Checkpoint 超时

- **WHEN** 用户在 30 分钟内未响应
- **THEN** CheckpointManager 自动标记为 "timeout"
- **AND** Agent 根据配置决定继续或终止
- **AND** 默认行为：终止执行并记录超时日志

### Scenario: 批量审批

- **WHEN** 用户有多个待审批的 checkpoints
- **THEN** 前端提供"全部批准"按钮
- **AND** 后端批量更新所有 pending checkpoints 为 "approved"
- **AND** 恢复所有阻塞的 Agents

---

# Spec: Agent Worktree Lifecycle Capability

## Requirement: Worktree 生命周期管理

System SHALL 自动管理 Git worktrees 的创建和清理，防止磁盘空间泄漏。Worktree MUST 在 Story 完成后（无论成功或失败）立即清理。

### Scenario: Story 成功后清理

- **WHEN** NativeCodingAgent 成功执行 Story
- **THEN** AgentWorker 调用 worktree_lifecycle.cleanup_after_story(agent_id, story_id, Success)
- **AND** 删除 worktree 目录（`.worktrees/agent-{id}`）
- **AND** 移除 Git worktree 引用（`git worktree remove`）
- **AND** 记录清理日志："Cleaned up worktree for agent {id} (story {story_id})"

### Scenario: Story 失败后清理

- **WHEN** NativeCodingAgent 执行 Story 失败
- **THEN** AgentWorker 仍然调用 cleanup_after_story(agent_id, story_id, Failure)
- **AND** 即使执行失败也删除 worktree
- **AND** 记录警告日志："Cleaned up worktree after failure for agent {id}"

### Scenario: 清理失败处理

- **WHEN** worktree 清理操作失败（如权限问题）
- **THEN** AgentWorker 记录错误日志但不中断主流程
- **AND** 将失败的 worktree 路径加入清理队列
- **AND** 后台任务定期重试清理（最多 3 次）

### Scenario: 保留永久分支（可选）

- **WHEN** 配置 `preserve_worktree_branches = true`
- **THEN** 在清理前将 worktree 分支合并到主分支或创建永久分支
- **AND** 分支命名：`story-{story_number}-completed-{timestamp}`
- **AND** 默认配置：false（不保留）

---

# Spec: Modified - Coding Harness Quality Checks

## Requirement: 分阶段质量检查（MODIFIED）

Native Coding Agent SHALL 按顺序执行分阶段质量检查，而非一次性运行所有检查。System MUST 在任一阶段失败时立即停止并提供精确的错误位置。

### Scenario: Lint 阶段检查

- **WHEN** Agent 完成代码生成
- **THEN** 首先运行 ESLint/Prettier 检查
- **AND** 如果失败，返回具体的文件和行号
- **AND** 不进行后续的 type-check 和 test

### Scenario: Type Check 阶段检查

- **WHEN** Lint 检查通过
- **THEN** 运行 TypeScript/Rust 编译检查
- **AND** 如果失败，返回编译错误详情
- **AND** 不进行后续的 test

### Scenario: Test 阶段检查

- **WHEN** Type check 通过
- **THEN** 运行单元测试和集成测试
- **AND** 返回测试覆盖率报告
- **AND** 如果有失败的测试，提供详细的断言错误信息

### Scenario: 增量检查优化

- **WHEN** 只有部分文件被修改
- **THEN** 使用 `--cache` 选项或手动过滤只检查修改的文件
- **AND** 显著减少检查时间（目标：减少 50%+）

---

# Spec: Modified - Agent Initialization Conversation History

## Requirement: 对话历史优化（MODIFIED）

Native Coding Agent SHALL 实现对话历史压缩和任务完成信号检测，以减少 token 成本并提高执行效率。

### Scenario: 任务完成信号检测

- **WHEN** AI 认为任务已完成
- **THEN** AI 必须在回复中包含 `<TASK_COMPLETE>` 标记
- **AND** Agent 检测到该标记后立即停止多轮循环
- **AND** 不再等待达到 max_turns

### Scenario: 对话历史压缩

- **WHEN** 对话轮数超过 10 轮
- **THEN** Agent 触发历史压缩
- **AND** 保留 system message + 最近 4 条消息
- **AND** 将早期对话摘要为简短总结
- **AND** 压缩后的历史总 token 数减少至少 60%

### Scenario: 压缩配置

- **WHEN** 配置 `enable_history_compression = false`
- **THEN** Agent 禁用历史压缩功能
- **AND** 用于调试或需要完整历史的场景
- **AND** 默认配置：true（启用压缩）

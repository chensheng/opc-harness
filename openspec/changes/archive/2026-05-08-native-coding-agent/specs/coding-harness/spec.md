## MODIFIED Requirements

### Requirement: Worktree 隔离环境管理

系统 SHALL 为每个 Vibe Coding Agent 创建独立的 Git worktree 作为开发环境，确保多个 Agent 可以并行工作而不互相干扰。

**原有内容**:
- Worktree 由 AgentWorker 在启动 CLI 进程前创建
- CLI 工具在 worktree 目录中执行
- Worktree 路径格式为 `.worktrees/agent-{agent_id}`

**修改内容**:
- Worktree 由 NativeCodingAgent 内部管理，通过 Git Tools 调用
- Native Agent 直接在 worktree 目录中执行文件操作
- 增加 worktree 自动清理机制（Story 完成后删除）

#### Scenario: 成功创建 worktree
- **WHEN** NativeCodingAgent 需要为 Story 创建隔离环境
- **THEN** 系统基于当前 HEAD 创建新分支并检出到 worktree 目录
- **AND** worktree 路径格式为 `.worktrees/agent-{agent_id}`

#### Scenario: Worktree 创建失败降级
- **WHEN** Git worktree 创建命令执行失败
- **THEN** 系统记录详细错误日志并通过 WebSocket 通知前端
- **AND** 回退到使用项目根目录作为工作目录
- **AND** Agent 继续执行任务（不中断）

#### Scenario: Worktree 已存在检查
- **WHEN** 尝试为已存在 worktree 的 agent_id 创建新的 worktree
- **THEN** 系统检测到路径已存在并返回错误
- **AND** 不执行 git worktree add 命令

#### Scenario: Worktree 自动清理
- **WHEN** Story 执行完成（成功或失败）
- **THEN** NativeCodingAgent 调用 cleanup_worktree 工具
- **AND** 删除 worktree 目录和对应分支
- **AND** 释放磁盘空间

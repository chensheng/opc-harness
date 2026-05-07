## Why

Vibe Coding 智能体在执行任务时会自动创建 Git worktree 作为隔离的开发环境。当前实现中，`git worktree add` 命令要求分支必须已存在于仓库中，但代码直接尝试从可能不存在的分支（如 `story-US-002`）创建 worktree，导致错误：

```
fatal: invalid reference: story-US-002
```

这使得智能体无法正常工作，回退到使用项目根目录，失去了工作树隔离的优势。需要在创建 worktree 之前确保分支存在，或者使用 `-b` 参数创建新分支。

## What Changes

- **修复 Worktree 创建逻辑**：修改 `WorktreeManager::create_worktree()` 方法，使用 `git worktree add -b <branch_name>` 命令基于当前 HEAD 创建新分支
- **增强错误处理**：如果分支创建失败，提供更清晰的错误信息和降级策略
- **添加分支存在性检查**：在创建 worktree 前验证分支是否存在，避免无效引用错误

## Capabilities

### New Capabilities

<!-- 此变更不引入新的能力，只是修复现有功能的 bug -->

### Modified Capabilities

- `coding-harness`: 修复 worktree 创建流程，确保分支正确创建

## Impact

**受影响的代码：**
- `src-tauri/src/agent/worktree_manager.rs` - Worktree 创建逻辑
- `src-tauri/src/agent/agent_worker.rs` - Agent Worker 调用 worktree 创建

**影响范围：**
- 所有使用 Vibe Coding 智能体的项目
- 现有的 agent sessions 不受影响（仅影响新创建的 worktree）

**API 变化：**
- 无 API 变化，内部实现修复

**依赖影响：**
- 无新增依赖
- Git 版本要求不变（worktree 功能自 Git 2.5+ 支持）

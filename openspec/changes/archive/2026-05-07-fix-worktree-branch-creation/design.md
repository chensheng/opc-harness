## Context

当前 Vibe Coding 智能体系统使用 Git worktree 为每个 Agent 提供隔离的开发环境。WorktreeManager 负责创建和管理这些工作树，但在实现中存在一个关键问题：

**当前实现（有问题）：**
```rust
// worktree_manager.rs:237-242
let output = tokio::process::Command::new("git")
    .current_dir(project_path)
    .args(&["worktree", "add", &worktree_path_str, branch_name])
    .output()
    .await
```

这个命令假设 `branch_name` 已经存在于仓库中。但实际情况是：
- Agent Worker 为每个 Story 生成新的分支名（如 `story-US-002`）
- 这些分支在创建 worktree 时通常不存在
- Git 报错：`fatal: invalid reference: story-US-002`

**影响：**
- Worktree 创建失败
- 系统回退到使用项目根目录
- 失去了工作树隔离的优势（多个 Agent 可能互相干扰）

**约束条件：**
- 必须保持向后兼容（不影响现有的 worktree）
- 不能改变对外 API（AgentWorker 调用方式不变）
- 需要在 Windows 和 Unix 系统上都正常工作

## Goals / Non-Goals

**Goals:**
1. 修复 worktree 创建逻辑，确保新分支能正确创建
2. 提供清晰的错误信息和降级策略
3. 保持代码简洁，不引入复杂的分支管理逻辑
4. 确保所有现有功能不受影响

**Non-Goals:**
1. 不实现完整的 Git 分支管理系统（如自动合并、冲突解决）
2. 不改变 worktree 的命名规范或存储位置
3. 不优化磁盘空间使用（这是另一个独立的问题）
4. 不处理已存在 worktree 的情况（已有检查逻辑）

## Decisions

### Decision 1: 使用 `-b` 参数创建新分支

**选择：** 修改 git 命令为 `git worktree add -b <branch_name> <path>`

**理由：**
- `-b` 参数会基于当前 HEAD 创建新分支并检出到 worktree
- 这是 Git 官方支持的标准用法
- 简单直接，不需要额外的分支创建步骤
- 符合智能体的使用场景（每个 Story 都是独立的开发任务）

**替代方案考虑：**
1. **先创建分支，再创建 worktree**
   - 需要两步操作：`git branch <name>` + `git worktree add <path> <name>`
   - 增加了复杂性和失败点
   - 没有明显优势

2. **检查分支是否存在，决定使用哪种命令**
   - 需要额外的 `git branch --list` 查询
   - 增加了一次 Git 调用
   - 对于我们的场景（总是创建新分支），没有必要

### Decision 2: 保持现有的错误处理和降级策略

**选择：** 如果 worktree 创建失败，继续使用项目根目录作为工作目录

**理由：**
- 当前的降级策略已经在 agent_worker.rs 中实现
- 保证系统可用性优先于完美隔离
- 避免因为 Git 问题导致整个 Agent 无法工作

**改进：**
- 增强错误日志，明确说明失败原因
- 通过 WebSocket 向前端发送更友好的错误消息

### Decision 3: 不添加分支存在性预检查

**选择：** 直接尝试创建 worktree，让 Git 返回错误

**理由：**
- Git 本身会进行验证，重复检查是冗余的
- 减少了一次 Git 命令调用
- 简化了代码逻辑
- 错误信息足够清晰

## Risks / Trade-offs

### Risk 1: 与现有 worktree 冲突

**风险：** 如果同一个 agent_id 的 worktree 已存在，`git worktree add` 会失败

**缓解：**
- 现有代码在第 217-222 行已经检查了 worktree 路径是否存在
- 如果存在，直接返回错误，不会执行 git 命令
- 这个检查在修改前后都有效

### Risk 2: 分支命名冲突

**风险：** 如果分支名已存在但不是 worktree 分支，可能导致意外行为

**缓解：**
- 分支命名规范为 `story-{story_number}`，由系统生成
- 每个 Story 有唯一的 ID，理论上不会冲突
- 如果真的冲突，Git 会报错，系统会降级到项目根目录

### Risk 3: Git 版本兼容性

**风险：** `-b` 参数可能在旧版本 Git 中不支持

**缓解：**
- `git worktree` 功能自 Git 2.5+ (2015年) 开始支持
- `-b` 参数是 worktree 的基础功能，从一开始就存在
- 项目要求 Rust 1.70+，隐含要求较新的开发环境
- 如果遇到极老的 Git 版本，会有明确的错误提示

### Trade-off: 简单性 vs 灵活性

**权衡：** 选择简单的实现，不支持从现有分支创建 worktree

**理由：**
- 当前业务场景只需要创建新分支
- 如果需要从现有分支创建，可以后续扩展
- YAGNI 原则（You Aren't Gonna Need It）

## Migration Plan

**部署步骤：**
1. 代码修改只涉及 Rust 后端，无需数据库迁移
2. 重新编译 Tauri 应用
3. 用户重启应用即可生效

**回滚策略：**
- 如果发现问题，回滚到之前的代码版本
- 已创建的 worktree 不受影响（只是创建方式不同）
- 没有数据迁移，回滚安全

**验证步骤：**
1. 启动 Tauri 开发环境
2. 触发 Agent Worker 创建新的 worktree
3. 验证 worktree 成功创建且分支存在
4. 检查 Git 日志确认分支基于正确的 commit

## Open Questions

无开放问题。这是一个明确的 bug 修复，技术方案清晰。

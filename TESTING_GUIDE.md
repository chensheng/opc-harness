# Agent Loop 测试指南

## 快速开始

### 1. 启动应用

```bash
npm run tauri:dev
```

等待应用编译完成并自动打开窗口。

### 2. 准备测试数据

在数据库中创建测试数据:

```sql
-- 1. 创建项目
INSERT INTO projects (id, name, description) 
VALUES ('test-project-001', 'Test Project', 'Project for testing Agent Loop');

-- 2. 创建活跃的 Sprint
INSERT INTO sprints (id, project_id, name, status, start_date, end_date)
VALUES (
    'sprint-001',
    'test-project-001',
    'Sprint 1',
    'active',
    datetime('now', '-1 day'),  -- 昨天开始
    datetime('now', '+7 days')   -- 7天后结束
);

-- 3. 创建待执行的用户故事
INSERT INTO user_stories (id, sprint_id, title, description, priority, status)
VALUES 
(
    'story-001',
    'sprint-001',
    'Implement user authentication',
    'Create login and registration functionality',
    'P0',
    'approved'
),
(
    'story-002',
    'sprint-001',
    'Add dashboard analytics',
    'Display key metrics on dashboard',
    'P1',
    'approved'
);
```

### 3. 启动 Agent Loop

1. 打开应用窗口
2. 导航到 **Vibe Coding** 标签页
3. 找到 **Agent Loop Control** 面板
4. 点击 **"启动 Agent Loop"** 按钮

### 4. 观察日志输出

在控制台或应用日志中查看以下关键日志:

#### 成功流程日志

```
[AgentLoop] Starting continuous loop with 300s interval
[AgentLoop] Detecting active sprint for project test-project-001
[AgentLoop] Found active sprint: sprint-001
[AgentLoop] Loading pending stories for sprint sprint-001
[AgentLoop] Found 2 pending stories
[AgentLoop] Attempting to lock story story-001: Implement user authentication (Priority: P0)
[DB::lock_user_story] Locked story: story-001 for agent coding-abc
[AgentLoop] Successfully locked story story-001
[AgentLoop] Creating worktree for agent coding-abc and story story-001
[WorktreeManager] Creating worktree at .worktrees/agent-coding-abc from branch main
[WorktreeManager] Successfully created worktree at /path/to/project/.worktrees/agent-coding-abc
[AgentLoop] Worktree created at: /path/to/project/.worktrees/agent-coding-abc
[Daemon] Spawning coding agent in worktree /path/to/project/.worktrees/agent-coding-abc for story story-001 with STDIO monitoring
[Daemon] Retrieved story context for story-001: title='Implement user authentication', acceptance_criteria_length=156
[Daemon] Building CLI command with full context: kimi ["--story-id", "story-001", "--title", "Implement user authentication", "--acceptance-criteria", "...", "--agent-type", "coding", "--worktree", "/path/to/project/.worktrees/agent-coding-abc"]
[Daemon] Agent coding-abc spawned in worktree /path/to/project/.worktrees/agent-coding-abc with PID: Some(12345)
[AICLIInteraction] Starting to listen for agent coding-abc with 1800s timeout
[AICLI:coding-abc] STDOUT: Thinking about authentication implementation...
[AgentLoop:coding-abc] AI Output: Thinking about authentication implementation...
[AICLI:coding-abc] STDOUT: [GENERATED_CODE] src/auth.rs:fn login() { /* ... */ }
[AgentLoop:coding-abc] Generated code for file: src/auth.rs
[CodeWriter] Successfully wrote 156 bytes to "/path/to/project/.worktrees/agent-coding-abc/src/auth.rs"
[AgentLoop:coding-abc] Successfully wrote generated code to: src/auth.rs
[AICLI:coding-abc] Agent process completed with status: ExitStatus(unix_wait_status(0))
[AgentLoop:coding-abc] Task completed: SUCCESS - Process exited with status: ExitStatus(unix_wait_status(0))
[GitOps] Starting commit and push for worktree: /path/to/project/.worktrees/agent-coding-abc
[GitOps] Detected changes:
 M src/auth.rs
[GitOps] Successfully staged all changes
[GitOps] Commit successful: [story-story-001 abc1234] Auto-generated code for story story-001
 1 file changed, 156 insertions(+)
 create mode 100644 src/auth.rs
[GitOps] Successfully pushed to branch: story-story-001
[AgentLoop:coding-abc] Successfully committed and pushed changes: Auto-generated code for story story-001
[StoryStatus] Updating story story-001 status to completed
[DB::complete_user_story] Completed story: story-001
[StoryStatus] Successfully updated 1 story(s) to completed
[AgentLoop:coding-abc] Successfully updated story status to completed
[AgentMonitoring] Agent coding-abc completed, cleaning up worktree
[WorktreeManager] Removing worktree at /path/to/project/.worktrees/agent-coding-abc
[WorktreeManager] Successfully removed worktree
```

#### 失败场景日志

**AI CLI 超时**:
```
[AICLIInteraction] Starting to listen for agent coding-xyz with 1800s timeout
[AICLI:coding-xyz] STDOUT: Starting analysis...
[AgentLoop:coding-xyz] AI Output: Starting analysis...
... (长时间无输出) ...
[AICLI:coding-xyz] Agent process timed out after 1800 seconds
[AgentLoop:coding-xyz] Task completed: FAILED - Agent process timed out after 1800 seconds
[StoryStatus] Updating story story-002 status to failed: Agent process timed out after 1800 seconds
[DB::fail_user_story] Failed story: story-002 - Agent process timed out after 1800 seconds
[StoryStatus] Successfully updated 1 story(s) to failed
```

**Git Push 失败**:
```
[GitOps] Starting commit and push for worktree: /path/to/project/.worktrees/agent-coding-abc
[GitOps] Detected changes: M src/auth.rs
[GitOps] Successfully staged all changes
[GitOps] Commit successful: [story-story-001 abc1234] Auto-generated code for story story-001
[GitOps] Push failed (possibly branch doesn't exist): error: remote origin not found
[GitOps] Branch creation failed (may already exist): fatal: a branch named 'story-story-001' already exists
[GitOps] Push failed after retry: error: remote origin not found
[AgentLoop:coding-abc] Failed to commit and push changes: git push failed after retry: error: remote origin not found
[StoryStatus] Updating story story-001 status to failed: Git operation failed: git push failed after retry: error: remote origin not found
```

### 5. 验证结果

#### 检查数据库

```sql
-- 查看 Story 状态
SELECT id, title, status, assigned_agent, locked_at, completed_at, failed_at, error_message
FROM user_stories
WHERE id IN ('story-001', 'story-002');

-- 预期结果:
-- story-001: status='completed', completed_at=<timestamp>, assigned_agent=NULL, locked_at=NULL
-- story-002: status='failed', failed_at=<timestamp>, error_message='...'
```

#### 检查 Git 仓库

```bash
# 查看所有分支
git branch -a

# 应该看到:
# * main
#   story-story-001
#   remotes/origin/story-story-001

# 查看提交历史
git log --oneline --all

# 应该看到:
# abc1234 (story-story-001) Auto-generated code for story story-001
# def5678 (main) Previous commit

# 查看文件变更
git show story-story-001:src/auth.rs
```

#### 检查 Worktree

```bash
# 列出所有 Worktree
git worktree list

# 应该看不到已完成的 Agent 的 Worktree (已被自动清理)
# 如果有正在运行的 Agent,会看到:
# /path/to/project/.worktrees/agent-coding-xyz  <branch-name>
```

### 6. 停止 Agent Loop

点击 **"停止 Agent Loop"** 按钮,观察日志:

```
[AgentLoop] Stop signal received
[AgentLoop] Continuous loop stopped
```

## 常见问题排查

### Q1: Agent Loop 没有检测到活跃 Sprint?

**检查**:
```sql
SELECT id, name, status, start_date, end_date
FROM sprints
WHERE project_id = 'test-project-001';

-- 确保:
-- 1. status = 'active'
-- 2. start_date <= NOW() <= end_date
```

### Q2: Story 没有被锁定?

**检查**:
```sql
SELECT id, title, status, assigned_agent, locked_at
FROM user_stories
WHERE sprint_id = 'sprint-001';

-- 确保:
-- 1. status IN ('draft', 'refined', 'approved')
-- 2. assigned_agent IS NULL OR locked_at < NOW() - 30 minutes
```

### Q3: Worktree 创建失败?

**检查**:
- Git 是否已安装: `git --version`
- 项目是否是 Git 仓库: `git status`
- 是否有权限创建目录: `ls -la .worktrees/`

### Q4: AI CLI 没有响应?

**检查**:
- Kimi/Claude Code 是否已安装: `kimi --version` 或 `claude --version`
- 环境变量是否正确配置
- 查看 `[AICLI:xxx]` 前缀的日志了解详细错误

### Q5: Git Push 失败?

**检查**:
- 远程仓库是否配置: `git remote -v`
- 是否有推送权限: `git push --dry-run`
- 网络连接是否正常

## 性能调优

### 调整并发数

在 `daemon_core.rs` 中修改:

```rust
pub struct DaemonConfig {
    pub max_concurrent_agents: usize,  // 默认 5,可调整为 3-10
    // ...
}
```

### 调整超时时间

在 `daemon_core.rs` 中修改:

```rust
// 当前 1800 秒 (30 分钟),可根据需要调整
interaction_clone.start_listening_with_timeout(1800).await
```

### 调整轮询间隔

在前端调用时指定:

```typescript
startAgentLoop(projectId, 60); // 每 60 秒检测一次 (默认 300 秒)
```

## 下一步

1. **实际运行**: 按照上述步骤启动应用并测试
2. **观察日志**: 重点关注 `[AgentLoop]`, `[Daemon]`, `[AICLI]`, `[GitOps]`, `[StoryStatus]` 前缀的日志
3. **验证结果**: 检查数据库、Git 仓库、Worktree 的状态
4. **问题反馈**: 如果遇到任何问题,记录完整日志并报告

祝测试顺利! 🚀

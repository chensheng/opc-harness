# Agent Loop 自动化执行引擎

## 概述

Agent Loop 是 Vibe Coding 的核心自动化执行引擎,能够根据当前时间自动检测活跃的 Sprint,并从中选取待执行的用户故事,启动 Coding Agent 进行开发。

## 架构设计

### 核心组件

#### 后端 (Rust)

1. **`agent_loop.rs`** - AgentLoop 主循环
   - `execute_once()`: 单次执行流程
     - 检测活跃 Sprint (基于时间)
     - 加载待执行故事 (按优先级排序)
     - 乐观锁竞争 (30分钟超时)
     - 启动 Coding Agent
   - `start_continuous()`: 持续运行模式 (后台任务)

2. **`worktree_manager.rs`** - Worktree 管理器 (新增 ✅)
   - `create_worktree(agent_id, story_id, branch_name)`: 为 Agent 创建独立工作树
   - `remove_worktree(agent_id)`: 删除指定 Worktree
   - `list_worktrees()`: 列出所有 Worktrees
   - `cleanup_orphaned_worktrees()`: 清理孤立的 Worktrees
   - `get_disk_usage()`: 获取磁盘使用量
   - 路径规范: `.worktrees/agent-{agent_id}`
   - 磁盘限额: 默认 10GB

3. **`agent_manager_core.rs`** - AgentManager 集成
   - 存储 AgentLoop 实例
   - 提供控制方法: `start_agent_loop()`, `stop_agent_loop()`, `execute_agent_loop_once()`
   - 初始化时自动配置 Worktree Manager

4. **`agent_manager_commands.rs`** - Tauri Commands
   - **Agent Loop 命令**:
     - `start_agent_loop(project_id, interval_secs?)`: 启动持续运行
     - `execute_agent_loop_once(project_id)`: 手动触发一次
     - `stop_agent_loop()`: 停止运行
     - `is_agent_loop_running()`: 查询状态
   - **Worktree 命令** (新增 ✅):
     - `create_worktree(agent_id, story_id, branch_name)`: 创建 Worktree
     - `remove_worktree(agent_id)`: 删除 Worktree
     - `list_worktrees()`: 列出所有 Worktrees
     - `cleanup_orphaned_worktrees()`: 清理孤立 Worktrees
     - `get_worktree_disk_usage()`: 获取磁盘使用量

5. **`main.rs`** - 命令注册
   - 在 `invoke_handler` 中注册所有 Agent Loop 和 Worktree 命令

#### 前端 (TypeScript)

6. **`useAgentLoop.ts`** - React Hook
   - `startAgentLoop()`: 启动持续运行
   - `executeOnce()`: 手动触发一次
   - `stopAgentLoop()`: 停止运行
   - `isRunning`: 查询运行状态

7. **`AgentLoopControl.tsx`** - 控制面板组件
   - 启动/停止按钮
   - 手动触发按钮
   - 状态显示
   - 实时日志输出

8. **`Dashboard.tsx`** - 集成到主仪表板
   - 在 Vibe Coding 标签页中显示 Agent Loop 控制面板

## 完整工作流程

```
┌─────────────────────────────────────────────┐
│  应用启动                                    │
│  main.rs::initialize()                      │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  AgentManager::initialize()                  │
│  - 创建 DaemonManager                       │
│  - 创建 WorktreeManager                     │
│  - 创建 AgentLoop                           │
│  - 启动 Agent 监控任务 (每10秒检查)          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  用户点击"启动 Agent Loop"                   │
│  start_agent_loop(project_id, 300)          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  AgentLoop::start_continuous(300s)           │
│  进入循环:                                   │
│  while is_running {                         │
│    execute_once(project_id)                 │
│    sleep(300s)                              │
│  }                                          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  execute_once() 单次执行                     │
│                                              │
│  1. 检测活跃 Sprint (基于时间)                │
│     get_active_sprint()                     │
│                                              │
│  2. 加载待执行故事 (按优先级排序)              │
│     get_pending_stories_by_sprint()         │
│                                              │
│  3. 对每个故事:                              │
│     a. 尝试锁定 (乐观锁,30分钟超时)           │
│        lock_user_story()                    │
│     b. 如果锁定成功:                         │
│        - 生成 Agent ID                      │
│        - 生成分支名: story-{story_number}    │
│        - 调用 start_coding_agent()          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  start_coding_agent()                        │
│                                              │
│  1. 创建 Worktree                           │
│     wt_manager.create_worktree()            │
│     → .worktrees/agent-{agent_id}           │
│                                              │
│  2. 创建消息通道                             │
│     mpsc::channel::<AICLIMessage>(100)      │
│                                              │
│  3. 启动后台消息处理任务                      │
│     tokio::spawn(async move {               │
│       while let Some(msg) = rx.recv().await {│
│         match msg {                         │
│           Stdout => log::debug!             │
│           Stderr => log::warn!              │
│           GeneratedCode => write_file()     │
│           TaskCompleted => git_ops()        │
│         }                                   │
│       }                                     │
│     })                                      │
│                                              │
│  4. 从数据库查询 Story 详细信息               │
│     get_story_context(story_id)             │
│     → title, acceptance_criteria            │
│                                              │
│  5. 构建 AICLIConfig                         │
│     ai_config.story_title = context.title   │
│     ai_config.acceptance_criteria = ...     │
│                                              │
│  6. 启动 AI CLI (带 STDIO 监控)              │
│     daemon.spawn_agent_with_stdio_          │
│       monitoring()                          │
│     → kimi --story-id ... --title ...       │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  AICLIInteraction::start_listening_          │
│    with_timeout(1800s)                      │
│                                              │
│  后台异步任务:                               │
│  - 读取 STDOUT/STDERR                       │
│  - 解析每行输出                              │
│  - 匹配 [GENERATED_CODE] 或 ``` 标记        │
│  - 发送 AICLIMessage 到通道                  │
│  - 30分钟超时后自动终止进程                  │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  消息处理任务接收消息                         │
│                                              │
│  GeneratedCode { file_path, content }       │
│    ↓                                         │
│  write_generated_code()                     │
│    - PathBuf::from(worktree).join(file_path)│
│    - fs::create_dir_all(parent_dir)         │
│    - fs::write(full_path, content)          │
│                                              │
│  TaskCompleted { success, summary }         │
│    ↓                                         │
│  if success:                                │
│    commit_and_push_changes()                │
│      - git status --porcelain               │
│      - git add .                            │
│      - git commit -m "Auto-generated..."    │
│      - git push -u origin story-{id}        │
│      - 失败时自动创建分支并重试              │
│    ↓                                         │
│    update_story_status_to_completed()       │
│      - db::complete_user_story()            │
│      - SET status='completed'               │
│      - completed_at=NOW()                   │
│      - assigned_agent=NULL                  │
│      - locked_at=NULL                       │
│  else:                                      │
│    update_story_status_to_failed(summary)   │
│      - db::fail_user_story()                │
│      - SET status='failed'                  │
│      - failed_at=NOW()                      │
│      - error_message=summary                │
│      - retry_count+1                        │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  Agent 监控任务 (每10秒检查)                  │
│  check_completed_agents()                   │
│                                              │
│  - 遍历所有 running_agents                  │
│  - child.try_wait() 非阻塞检查              │
│  - 如果进程已结束:                           │
│    - 更新 Agent 状态为 Completed/Failed     │
│    - 删除对应 Worktree                      │
│    - 从 running_agents 移除                 │
└─────────────────────────────────────────────┘
```

## 测试和优化

### 已实施的优化

#### 1. **增强的 AI 输出解析** (ai_cli_interaction.rs)

`parse_generated_code()` 现在支持多种格式:

**格式 1: [GENERATED_CODE] marker**
```
[GENERATED_CODE] src/auth.rs:fn login() { /* ... */ }
```

**格式 2: Markdown code block** (常见于 Claude Code)
````
```src/auth.rs
fn login(email: String, password: String) -> Result<User, AuthError> {
    // implementation
}
```
````

**格式 3: JSON 格式** (预留接口)
```json
{"file": "src/auth.rs", "code": "fn login() {...}"}
```

#### 2. **超时机制** (ai_cli_interaction.rs)

新增 `start_listening_with_timeout(timeout_secs)` 方法:
- **默认超时时间**: 30 分钟 (1800 秒)
- **超时行为**: 
  - 自动终止 AI CLI 进程
  - 发送 `TaskCompleted(success=false)` 消息
  - 记录错误日志
  - 防止资源泄漏

**使用示例**:
```rust
// 在 daemon_core.rs 中
tokio::spawn(async move {
    if let Err(e) = interaction_clone.start_listening_with_timeout(1800).await {
        log::error!("[Daemon] Failed to start listening for agent {}: {}", agent_id_for_log, e);
    }
});
```

#### 3. **错误恢复**

- **消息通道发送失败**: 使用 `let _ = message_tx.send(...).await` 忽略错误,避免 panic
- **进程等待失败**: 记录详细错误日志,发送 TaskCompleted 消息
- **超时清理**: 超时后自动调用 `terminate()` 终止进程

### 待实施的优化

#### 1. **健康检查定时任务**

```rust
// 在 AgentManager 中添加
pub fn start_health_check(&self, interval_secs: u64) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            
            // 检查所有运行中的 Agent
            for agent_id in running_agents {
                // 检查进程是否存活
                // 检查资源使用量 (CPU, 内存)
                // 检查 Worktree 磁盘使用量
            }
        }
    });
}
```

#### 2. **Git Push 指数退避重试**

```rust
async fn push_with_retry(git_cmd: &mut Command, max_retries: u32) -> Result<(), String> {
    let mut retries = 0;
    let mut delay = Duration::from_secs(1);
    
    while retries < max_retries {
        match git_cmd.output().await {
            Ok(output) if output.status.success() => return Ok(()),
            _ => {
                retries += 1;
                log::warn!("Push failed, retrying in {:?}... ({}/{})", 
                    delay, retries, max_retries);
                tokio::time::sleep(delay).await;
                delay *= 2; // 指数退避
            }
        }
    }
    
    Err(format!("Push failed after {} retries", max_retries))
}
```

#### 3. **资源监控和告警**

```rust
// 监控 Worktree 磁盘使用量
let disk_usage = wt_manager.get_disk_usage()?;
if disk_usage > 8 * 1024 * 1024 * 1024 { // 8GB
    log::warn!("Worktree disk usage exceeded 8GB: {}", disk_usage);
    // 发送告警通知
}

// 监控连续失败次数
if consecutive_failures > 3 {
    log::error!("Consecutive failures exceeded threshold: {}", consecutive_failures);
    // 暂停 Agent Loop,发送告警
}
```

#### 4. **性能指标收集**

```rust
struct AgentMetrics {
    story_lock_time: Duration,      // Story 锁定耗时
    worktree_creation_time: Duration, // Worktree 创建耗时
    ai_execution_time: Duration,    // AI 执行耗时
    git_commit_time: Duration,      // Git 操作耗时
    total_time: Duration,           // 总耗时
}

// 在关键步骤记录时间戳
let start = Instant::now();
// ... 执行操作 ...
let duration = start.elapsed();
log::info!("Operation completed in {:?}", duration);
```

### 测试建议

#### 1. **单元测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_generated_code_marker_format() {
        let output = "[GENERATED_CODE] src/main.rs:fn main() {}";
        let result = parse_generated_code(output);
        assert!(result.is_some());
        assert_eq!(result.unwrap().file_path, "src/main.rs");
    }
    
    #[test]
    fn test_parse_generated_code_markdown_format() {
        let output = "```src/main.rs\nfn main() {}\n```";
        let result = parse_generated_code(output);
        assert!(result.is_some());
    }
    
    #[test]
    fn test_parse_generated_code_invalid_format() {
        let output = "Just some random output";
        let result = parse_generated_code(output);
        assert!(result.is_none());
    }
}
```

#### 2. **集成测试**

模拟完整的 Agent Loop 流程:
1. 创建测试数据库,插入测试数据 (Sprint, UserStory)
2. 启动 Agent Loop
3. 验证 Worktree 创建
4. 模拟 AI CLI 输出
5. 验证文件写入
6. 验证 Git commit
7. 验证 Story 状态更新

#### 3. **压力测试**

并发启动多个 Agent,验证:
- 资源管理 (Worktree 数量、磁盘使用量)
- 并发控制 (max_concurrent 限制)
- 数据库连接池
- 消息通道背压处理

#### 4. **故障注入**

模拟异常场景:
- Git 远程仓库不可用
- AI CLI 进程崩溃
- 数据库连接失败
- 磁盘空间不足
- 网络中断

### 已知限制

1. **JSON 解析未实现**: `parse_generated_code()` 的 JSON 格式解析需要添加 `serde_json` 依赖
2. **无健康检查定时任务**: 目前没有定期检查 Agent 健康状态的后台任务
3. **简单重试策略**: Git push 失败时只有简单重试,没有指数退避
4. **缺少性能指标**: 没有详细记录每个步骤的耗时

## 部署和运维

### 环境要求

- **Kimi CLI** 或 **Claude Code CLI** 已安装并配置
- **Git** 已安装,远程仓库已配置
- **SQLite** 数据库已初始化
- 足够的磁盘空间 (默认限额 10GB)

### 配置项

```rust
// 在 DaemonConfig 中配置
pub struct DaemonConfig {
    pub max_concurrent_agents: usize,  // 最大并发 Agent 数 (默认 5)
    pub worktree_disk_limit_gb: u64,   // Worktree 磁盘限额 (默认 10GB)
    pub agent_timeout_secs: u64,       // Agent 超时时间 (默认 1800s)
    pub health_check_interval_secs: u64, // 健康检查间隔 (待实现)
}
```

### 监控和日志

**关键日志**:
```
[AgentLoop] Locked story US-001: Implement user authentication (Priority: P0)
[AgentLoop] Creating worktree for agent coding-abc and story uuid-of-us-001
[WorktreeManager] Successfully created worktree at /path/.worktrees/agent-coding-abc
[Daemon] Spawning coding agent in worktree ... with STDIO monitoring
[Daemon] Retrieved story context for uuid-of-us-001: title='...', acceptance_criteria_length=156
[Daemon] Building CLI command with full context: kimi ["--story-id", "...", ...]
[Daemon] Agent coding-xyz spawned with PID: Some(12345)
[AICLIInteraction] Starting to listen for agent coding-xyz with 1800s timeout
[AICLI:coding-xyz] STDOUT: Thinking about the implementation...
[AgentLoop:coding-xyz] AI Output: Thinking about the implementation...
[AICLI:coding-xyz] STDOUT: [GENERATED_CODE] src/auth.rs:fn login() {...}
[AgentLoop:coding-xyz] Generated code for file: src/auth.rs
[CodeWriter] Successfully wrote 156 bytes to "/path/.worktrees/agent-coding-xyz/src/auth.rs"
[AgentLoop:coding-xyz] Successfully wrote generated code to: src/auth.rs
[AICLI:coding-xyz] Agent process completed with status: ExitStatus(unix_wait_status(0))
[AgentLoop:coding-xyz] Task completed: SUCCESS - Process exited with status: ExitStatus(unix_wait_status(0))
[GitOps] Starting commit and push for worktree: /path/.worktrees/agent-coding-xyz
[GitOps] Detected changes: M src/auth.rs
[GitOps] Successfully staged all changes
[GitOps] Commit successful: [story-US-001 abc1234] Auto-generated code for story US-001
[GitOps] Successfully pushed to branch: story-US-001
[AgentLoop:coding-xyz] Successfully committed and pushed changes: Auto-generated code for story US-001
[StoryStatus] Updating story uuid-of-us-001 status to completed
[DB::complete_user_story] Completed story: uuid-of-us-001
[StoryStatus] Successfully updated 1 story(s) to completed
[AgentLoop:coding-xyz] Successfully updated story status to completed
```

**告警规则** (待实现):
- 连续失败次数 > 3: 暂停 Agent Loop,发送告警
- Worktree 磁盘使用量 > 8GB: 发送警告
- Agent 执行时间 > 30 分钟: 发送警告
- Story 锁定超时: 自动释放锁,发送告警

## 常见问题

### Q1: AI CLI 长时间无响应怎么办?

**A**: 系统已实现 30 分钟超时机制,超时后会自动终止进程并标记 Story 为 failed。可以调整 `start_listening_with_timeout()` 的参数来修改超时时间。

### Q2: Git push 失败如何处理?

**A**: 系统会自动尝试创建分支并重试。如果仍然失败,会标记 Story 为 failed 并记录错误信息。可以查看日志中的 `[GitOps]` 前缀了解详细原因。

### Q3: 如何查看 Agent 的执行进度?

**A**: 通过前端 `AgentLoopControl` 组件可以实时查看日志输出。后端日志中 `[AICLI:xxx]` 前缀的消息显示 AI 的输出,`[AgentLoop:xxx]` 前缀的消息显示处理进度。

### Q4: Worktree 占用太多磁盘空间怎么办?

**A**: 系统会在 Agent 完成后自动删除 Worktree。也可以手动调用 `cleanup_orphaned_worktrees()` 清理孤立的 Worktree。磁盘限额默认为 10GB,可以在配置中调整。

### Q5: 如何调试 AI 生成的代码?

**A**: AI 生成的代码会写入 Worktree 目录 (`.worktrees/agent-{agent_id}/`)。可以进入该目录查看生成的文件,使用 Git 查看变更历史。

## 未来规划

1. **分布式合并锁**: 基于 SQLite 实现原子锁,支持多 Agent 并行执行
2. **两步合并策略**: main ↔ agent_branch 双向合并,AI 辅助冲突解决
3. **结构化日志**: 关键事件审计和性能指标收集
4. **告警系统**: 连续失败、资源超限、锁超时通知
5. **AI 模型选择**: 支持配置不同的 AI CLI 工具 (Kimi, Claude Code, CodeFree 等)
6. **代码审查**: 自动生成 PR,邀请人工审查
7. **回滚机制**: 如果生成的代码有问题,支持自动回滚

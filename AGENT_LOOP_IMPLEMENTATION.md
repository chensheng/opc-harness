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

1. **`useAgentLoop.ts`** - React Hooks
   - `useAgentLoop()`: 封装 Agent Loop 控制命令
   - `useWorktreeManager()` (新增 ✅): 封装 Worktree 管理命令
     - 状态: `worktrees`, `diskUsage`, `isLoading`, `error`
     - 方法: `createWorktree()`, `removeWorktree()`, `listWorktrees()`, `cleanupOrphaned()`, `getDiskUsage()`

2. **`App.tsx`** - 自动启动
   - 应用挂载后 2 秒自动调用 `startAgentLoop()`
   - 从 localStorage 获取当前项目 ID

3. **`AgentLoopControl.tsx`** - 控制面板
   - Agent Loop 控制 (启动/停止/执行一次)
   - Worktree 管理面板 (新增 ✅)
     - 显示 Worktree 列表
     - 显示磁盘使用量
     - 刷新和清理按钮
   - 集成到 Dashboard

## 使用方式

### 1. 自动启动 (推荐)

应用启动时会自动启动 Agent Loop,无需手动干预。

```typescript
// App.tsx 中的逻辑
useEffect(() => {
  const initAgentLoop = async () => {
    const running = await checkStatus()
    if (!running && currentProjectId) {
      await startAgentLoop(currentProjectId, 60) // 每 60 秒检测一次
    }
  }
  
  const timer = setTimeout(initAgentLoop, 2000)
  return () => clearTimeout(timer)
}, [startAgentLoop, checkStatus])
```

### 2. 手动控制

通过 Dashboard 中的 "Agent Loop 自动化执行引擎" 面板:

- **启动**: 开始持续运行 (默认 60 秒间隔)
- **执行一次**: 立即触发单次检测和执行
- **停止**: 终止持续运行
- **刷新 Worktrees**: 查看当前所有 Worktrees
- **清理孤立 Worktrees**: 删除不再需要的 Worktrees

### 3. 编程调用

``typescript
import { useAgentLoop, useWorktreeManager } from '@/hooks/useAgentLoop'

const { startAgentLoop, executeOnce, stopAgentLoop } = useAgentLoop()
const { createWorktree, removeWorktree, listWorktrees, cleanupOrphaned } = useWorktreeManager()

// 启动 Agent Loop
await startAgentLoop('project-uuid', 60)

// 执行一次
const count = await executeOnce('project-uuid')
console.log(`Started ${count} agents`)

// 创建 Worktree
const path = await createWorktree('agent-123', 'story-456', 'feature/user-auth')
console.log(`Worktree created at: ${path}`)

// 列出所有 Worktrees
const worktrees = await listWorktrees()
console.log(`Found ${worktrees.length} worktrees`)

// 清理孤立 Worktrees
const cleaned = await cleanupOrphaned()
console.log(`Cleaned up ${cleaned} orphaned worktrees`)

// 停止
await stopAgentLoop()
```

## 工作流程

```
┌─────────────────────────────────────────────┐
│         Agent Loop 主循环 (60s 间隔)          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  1. 检测活跃 Sprint                           │
│     WHERE start_date <= now <= end_date      │
│     AND status = 'active'                    │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  2. 加载待执行故事                             │
│     WHERE sprint_id = ?                      │
│     AND status IN ('draft','refined','approved')│
│     ORDER BY priority ASC                    │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  3. 乐观锁竞争                                │
│     UPDATE user_stories                      │
│     SET status = 'in_development',           │
│         assigned_agent = ?,                  │
│         locked_at = NOW()                    │
│     WHERE id = ? AND                         │
│       (locked_at IS NULL OR                  │
│        locked_at < NOW() - 30min)            │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  4. 创建 Worktree (新增 ✅)                   │
│     git worktree add .worktrees/agent-{id}  │
│                        {branch_name}         │
│     - 隔离性强,每个 Agent 独立操作             │
│     - 节省空间,共享 .git 目录                 │
│     - 支持并发,无需频繁切换分支               │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  5. 启动 Coding Agent                        │
│     daemon.spawn_agent("coding", project_path)│
│     - 在专属 Worktree 中执行 (TODO)          │
│     - 调用 Kimi/Claude CLI (TODO)            │
│     - 生成代码并提交 (TODO)                  │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│  6. 监控与合并 (TODO)                         │
│     - 等待 Agent 完成                         │
│     - 获取分布式合并锁                        │
│     - 两步合并策略                            │
│     - 标记故事为 completed                    │
│     - 删除 Worktree (TODO)                   │
└─────────────────────────────────────────────┘
```

## 数据库字段

### user_stories 表新增字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `assigned_agent` | TEXT | 当前执行 Agent ID |
| `locked_at` | TEXT | 锁定时间戳 (用于超时检测) |
| `started_at` | TEXT | 开始执行时间 |
| `completed_at` | TEXT | 完成时间 |
| `failed_at` | TEXT | 失败时间 |
| `error_message` | TEXT | 失败原因 |
| `retry_count` | INTEGER | 重试次数 (默认 0) |

## Git Worktree 优势

### 为什么使用 Worktree?

1. **空间效率**: 
   - 完整克隆: 每个副本包含完整的 `.git` 目录 (~100MB+)
   - Worktree: 共享主仓库的 `.git` 目录,仅占用工作文件空间 (~10MB)

2. **并发安全**:
   - 传统方式: 多个 Agent 同时操作同一工作目录会导致冲突
   - Worktree: 每个 Agent 拥有独立的工作树,互不干扰

3. **性能优化**:
   - 切换分支: `git checkout` 需要重写整个工作目录 (慢)
   - Worktree: 直接切换到对应工作树 (快)

4. **隔离性强**:
   - 每个 Agent 可以独立修改文件、提交代码
   - 不会影响其他 Agent 或主分支

### Worktree 命令示例

``bash
# 创建 Worktree
git worktree add .worktrees/agent-123 feature/user-auth

# 列出所有 Worktrees
git worktree list

# 删除 Worktree
git worktree remove .worktrees/agent-123 --force

# 查看详细信息
git worktree list --porcelain
```

## 容错机制

### 1. 锁超时回收

- 锁定超时: 30 分钟
- SQL 查询条件: `locked_at < datetime('now', '-30 minutes')`
- 超时后自动允许其他 Agent 重新抢占

### 2. 失败重试

- 最大重试次数: 3 次
- 重试间隔: 指数退避 (1min → 5min → 15min)
- 超过限制后标记为 'blocked'

### 3. Agent 崩溃恢复

- 定期健康检查 (TODO)
- 检测到崩溃后重新分配任务 (TODO)
- 保存中间状态便于断点续传 (TODO)

### 4. Worktree 清理

- 自动检测孤立 Worktrees (对应的 Agent 已不存在)
- 手动触发清理: `cleanup_orphaned_worktrees()`
- 磁盘空间监控: 超过 10GB 时告警并阻止创建新 Worktree

## 当前状态

### ✅ 已完成

- [x] Agent Loop 框架搭建
- [x] 时间驱动的 Sprint 检测
- [x] 用户故事优先级排序
- [x] 乐观锁竞争机制
- [x] Daemon Manager 集成
- [x] Tauri Commands 暴露
- [x] 前端 Hook 封装
- [x] 应用启动自动触发
- [x] 可视化控制面板
- [x] **Worktree 管理器实现** (新增 ✅)
  - [x] 创建/删除 Worktree
  - [x] 列出所有 Worktrees
  - [x] 清理孤立 Worktrees
  - [x] 磁盘使用量监控
  - [x] 前端管理面板
- [x] **Worktree 与 Agent 深度集成** (新增 ✅)
  - [x] Daemon 新增 `spawn_agent_in_worktree()` 方法
  - [x] Agent 在专属 Worktree 中执行而非项目根目录
  - [x] 传递 Story 上下文给 AI CLI (`--story-id`, `--worktree`, `--agent-type`)
  - [x] 失败回退机制: Worktree 创建/启动失败时回退到项目根目录
  - [x] 添加 `cleanup_completed_worktrees()` 方法,自动清理已完成 Agent 的 Worktree
  - [x] 完善日志记录,便于追踪 Agent 在哪个 Worktree 中执行
- [x] **Agent 完成后自动删除 Worktree** (新增 ✅)
  - [x] Daemon 新增 `check_completed_agents()` 方法,非阻塞检测进程状态
  - [x] 使用 `child.try_wait()` 检查 Agent 进程是否已结束
  - [x] 自动更新 Agent 状态为 Completed 或 Failed
  - [x] AgentManager 新增 `start_agent_monitoring()` 后台监控任务
  - [x] 每 10 秒自动检查已完成的 Agent 并清理对应 Worktree
  - [x] 在 `initialize()` 中自动启动监控任务,无需手动干预

### ❌ 待完善

- [ ] **真实 CLI 调用**: Daemon spawn_agent 需调用 Kimi/Claude Code (当前为 Mock)
- [ ] **Coding Agent 代码生成**: 集成真实 AI API,替换 Mock 实现
- [ ] **分布式合并锁**: 基于 SQLite 实现原子锁
- [ ] **两步合并策略**: main ↔ agent_branch 双向合并
- [ ] **AI 辅助冲突解决**: 解析 Git Conflicts 并自动修复
- [ ] **结构化日志**: 关键事件审计和性能指标
- [ ] **告警机制**: 连续失败、资源超限、锁超时通知

## 配置参数

### 轮询间隔

默认: **60 秒**

修改位置:
- 前端: `App.tsx` 中 `startAgentLoop(currentProjectId, 60)`
- 后端: `agent_manager_commands.rs` 中 `interval_secs.unwrap_or(60)`

### 锁超时时间

默认: **30 分钟**

修改位置: `sprint_repository.rs` 中 SQL 查询
```
WHERE ... OR locked_at < datetime('now', '-30 minutes')
```

### 最大并发 Agent 数

默认: **5**

修改位置: `daemon_types.rs` 中 `DaemonConfig.max_concurrent_agents`

### Worktree 磁盘限额

默认: **10 GB**

修改位置: `worktree_manager.rs` 中 `WorktreeManagerConfig.max_disk_usage_bytes`

## 调试技巧

### 1. 查看后端日志

```
# Rust 日志输出
log::info!("[AgentLoop] Starting execution for project: {}", project_id);
log::info!("[AgentLoop] Found active sprint: {} ({} stories)", name, count);
log::info!("[AgentLoop] Locked story {}: {} (Priority: {})", id, title, priority);
log::info!("[WorktreeManager] Creating worktree for agent {} at {}", agent_id, path);
```

### 2. 前端控制台

```
// useAgentLoop Hook 会输出详细日志
console.log('[useAgentLoop] Agent Loop started for project:', projectId)
console.log('[useAgentLoop] Executed once, started X agents')

// useWorktreeManager Hook
console.log('[useWorktreeManager] Created worktree at:', path)
console.log('[useWorktreeManager] Listed', count, 'worktrees')
```

### 3. 数据库查询

```
-- 查看当前锁定的故事
SELECT id, story_number, title, assigned_agent, locked_at, status
FROM user_stories
WHERE status = 'in_development';

-- 查看失败的 Story
SELECT id, story_number, error_message, retry_count
FROM user_stories
WHERE status = 'failed';
```

### 4. Git Worktree 命令

```
# 查看所有 Worktrees
cd /path/to/project
git worktree list

# 查看详细信息
git worktree list --porcelain

# 手动清理 (谨慎使用)
git worktree prune
```

## 下一步开发计划

按照优先级依次实现:

1. **P0**: 真实 CLI 调用 + Coding Agent 代码生成集成
2. **P0**: Worktree 与 Agent 深度集成 (Agent 在 Worktree 中执行)
3. **P1**: 分布式合并锁 + 两步合并策略
4. **P1**: 失败重试机制
5. **P2**: 结构化日志 + 告警系统

详见项目根目录的规划文档。

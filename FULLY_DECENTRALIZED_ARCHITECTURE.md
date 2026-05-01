# 完全去中心化智能体系统架构

## 🎯 设计目标

实现**完全去中心化**的智能体系统，每个 Agent Worker 都是独立的执行单元，拥有完整的 Agent Loop 逻辑，不依赖任何中心调度器。

---

## 📐 核心架构

```
┌─────────────────────────────────────────────────┐
│              数据库 (SQLite)                      │
│                                                   │
│  ┌──────────────┐    ┌──────────────────────┐   │
│  │   Sprints    │    │   User Stories       │   │
│  │              │    │   - id               │   │
│  │              │    │   - status           │   │
│  │              │    │   - locked_by        │   │
│  │              │    │   - locked_at        │   │
│  └──────────────┘    └──────────────────────┘   │
└────┬──────────┬──────────┬──────────┬──────────┘
     │          │          │          │
     ▼          ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
│Worker 1│ │Worker 2│ │Worker 3│ │Worker N│  ← 完全独立的 Agent Loop
│        │ │        │ │        │ │        │
│查询DB  │ │查询DB  │ │查询DB  │ │查询DB  │
│乐观锁  │ │乐观锁  │ │乐观锁  │ │乐观锁  │
│执行任务│ │执行任务│ │执行任务│ │执行任务│
└────────┘ └────────┘ └────────┘ └────────┘
```

---

## 🔧 核心组件

### 1. AgentWorker (完全去中心化的智能体)

**实现**: `src-tauri/src/agent/agent_worker.rs`

每个 Worker 包含完整的 Agent Loop 逻辑：

```rust
pub struct AgentWorker {
    config: AgentWorkerConfig,
    daemon_manager: Arc<RwLock<DaemonManager>>,
    worktree_manager: Option<Arc<WorktreeManager>>,
    is_running: bool,
    current_story_id: Option<String>,
}

impl AgentWorker {
    /// 启动 Worker（开始独立运行 Agent Loop）
    pub async fn start(&mut self) -> Result<(), String> {
        // 在后台启动独立的 Agent Loop
        tokio::spawn(async move {
            loop {
                // Step 1: 查询活跃 Sprint
                let active_sprint = db::get_active_sprint(&conn)?;
                
                // Step 2: 加载待执行的 User Stories
                let pending_stories = db::get_pending_stories_by_sprint(&conn, &active_sprint.id)?;
                
                // Step 3: 尝试锁定第一个可用的 Story（乐观锁）
                for story in pending_stories.iter() {
                    let locked = db::lock_user_story(&conn, &story.id, &agent_id)?;
                    
                    if locked {
                        // Step 4: 启动 Coding Agent
                        Self::start_coding_agent(...).await?;
                        break; // 每个 Worker 每次循环只处理一个 Story
                    }
                }
                
                // 等待下一个周期
                tokio::time::sleep(Duration::from_secs(check_interval)).await;
            }
        });
    }
}
```

**特性**:
- ✅ **完全独立**: 每个 Worker 拥有自己的 Agent Loop
- ✅ **定时查询**: 每 N 秒查询一次数据库
- ✅ **乐观锁**: 通过数据库字段 `locked_by` 和 `locked_at` 实现竞争领取
- ✅ **自动状态更新**: 任务完成后自动更新 Story 状态
- ✅ **零外部依赖**: 无需 Redis、EventBus 等中间件

---

### 2. 乐观锁机制

**数据库表结构** (`user_stories`):

```sql
CREATE TABLE user_stories (
    id TEXT PRIMARY KEY,
    story_number TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL,  -- 'draft', 'refined', 'approved', 'in_progress', 'completed', 'failed'
    locked_by TEXT,         -- 锁定该 Story 的 Agent ID
    locked_at TIMESTAMP,    -- 锁定时间
    completed_at TIMESTAMP,
    failed_at TIMESTAMP,
    error_message TEXT,
    ...
);
```

**锁定逻辑** (`db::lock_user_story`):

```sql
UPDATE user_stories 
SET locked_by = ?, 
    locked_at = NOW() 
WHERE id = ? 
  AND (
    locked_by IS NULL 
    OR locked_at < datetime('now', '-30 minutes')
  );
```

**解锁逻辑** (`db::unlock_user_story`):

```sql
UPDATE user_stories 
SET locked_by = NULL, 
    locked_at = NULL 
WHERE id = ?;
```

**特性**:
- ✅ **原子操作**: SQL UPDATE 保证原子性
- ✅ **超时释放**: 30 分钟未完成任务自动释放锁
- ✅ **幂等性**: 多次调用不会产生副作用

---

### 3. Tauri Commands

**启动 Worker**:

```typescript
const workerId = await invoke('start_agent_worker', {
  workerId: 'worker-1',  // 可选，留空自动生成
  projectId: 'project-uuid',
  checkInterval: 30,     // 查询间隔（秒）
})
```

**停止 Worker**:

```typescript
await invoke('stop_agent_worker', {
  workerId: 'worker-1'
})
```

**列出 Workers**:

```typescript
const workers = await invoke('list_agent_workers')
// [
//   { worker_id: 'worker-1', is_running: true, current_story_id: 'US-001' },
//   { worker_id: 'worker-2', is_running: true, current_story_id: null }
// ]
```

---

## 🔄 完整工作流程

### 场景: 3 个 Worker 竞争处理 Story US-001

```
时间线:
T0: 用户创建 Story US-001 (status='approved')
    ↓
T1: Worker 1, Worker 2, Worker 3 同时查询数据库
    ↓
T2: 各 Worker 尝试乐观锁
    - Worker 1: UPDATE ... WHERE locked_by IS NULL → ✅ 成功 (影响 1 行)
    - Worker 2: UPDATE ... WHERE locked_by IS NULL → ❌ 失败 (影响 0 行)
    - Worker 3: UPDATE ... WHERE locked_by IS NULL → ❌ 失败 (影响 0 行)
    ↓
T3: Worker 1 继续执行
    - 创建 Worktree
    - 启动 AI CLI
    - 生成代码
    - Git commit & push
    - 更新 Story 状态为 completed
    ↓
T4: Worker 2 和 Worker 3 放弃，等待下一个查询周期
```

---

## 💡 技术优势

| 维度 | 中心化 Agent Loop | 部分去中心化 (Node) | 完全去中心化 (Worker) |
|------|------------------|-------------------|---------------------|
| **架构复杂度** | ✅ 低 | ⚠️ 中 | ✅ 低 |
| **外部依赖** | ✅ 无 | ❌ EventBus + LockManager | ✅ 无 |
| **容错性** | ❌ Loop 崩溃则全部停止 | ✅ Node 独立崩溃不影响其他 | ✅ Worker 独立崩溃不影响其他 |
| **扩展性** | ⚠️ 需修改并发配置 | ✅ 动态增减 Node | ✅ 动态增减 Worker |
| **性能** | ✅ 高（内存操作） | ✅ 高（内存操作） | ✅ 高（数据库索引优化） |
| **可观测性** | ✅ 集中式日志 | ⚠️ 分散式日志 | ⚠️ 分散式日志 |
| **适用场景** | 简单单机部署 | 高可用需求 | **完全自治需求** |

---

## 🎯 适用场景

### ✅ 适合使用完全去中心化方案
- 需要**完全自治**的智能体系统
- 不想依赖任何中心调度器
- 追求**极简架构**，降低运维成本
- 每个智能体需要**独立决策**能力
- 数据库作为唯一的状态存储

### ❌ 不适合使用完全去中心化方案
- 需要复杂的任务编排（如 DAG 工作流）
- 需要全局资源协调（如共享 GPU）
- 需要跨机器的分布式锁（当前仅支持单机）
- 已有成熟的中心调度器基础设施

---

## 📊 性能对比

**数据库查询延迟**:
- SQLite (本地): ~0.1-1ms
- 乐观锁 UPDATE: ~0.5-2ms

**Worker 启动延迟**:
- Tokio Task 创建: <1μs

**吞吐量**:
- 单 Worker: ~1 Story / 几分钟（取决于 AI CLI 执行时间）
- 多 Worker 并发: 受限于数据库写锁和 AI CLI 进程数

---

## 🚀 快速开始

### 1. 启动第一个 Worker

```typescript
// 前端调用
const workerId1 = await invoke('start_agent_worker', {
  projectId: 'my-project-id',
  checkInterval: 30,
})
```

### 2. 启动第二个 Worker

```typescript
const workerId2 = await invoke('start_agent_worker', {
  projectId: 'my-project-id',
  checkInterval: 30,
})
```

### 3. 查看运行中的 Workers

```typescript
const workers = await invoke('list_agent_workers')
console.log(workers)
// [
//   { worker_id: 'worker-abc123', is_running: true, current_story_id: null },
//   { worker_id: 'worker-def456', is_running: true, current_story_id: 'US-001' }
// ]
```

### 4. 停止 Worker

```typescript
await invoke('stop_agent_worker', { workerId: 'worker-abc123' })
```

---

## 🔍 调试技巧

### 1. 查看 Worker 日志

在终端中应该看到类似日志:

```log
[AgentWorker:worker-abc123] 🚀 Starting independent agent loop
[AgentWorker:worker-abc123] Independent loop started (interval: 30s)
[AgentWorker:worker-abc123] 🔄 Starting execution cycle
[AgentWorker:worker-abc123] Found active Sprint: Sprint 1 (uuid-xxx)
[AgentWorker:worker-abc123] Found 3 pending story(s)
[AgentWorker:worker-abc123] 🔒 Locked story US-001: Implement authentication
[AgentWorker:worker-abc123] Starting coding agent for story US-001
[AgentWorker:worker-abc123] Worktree created at: .worktrees/agent-xxx
[AgentWorker:worker-abc123] AI CLI process spawned with PID: 12345
[StoryStatus] Updating story uuid-of-us-001 status to completed
[DB::complete_user_story] Completed story: uuid-of-us-001
[AgentWorker:worker-abc123] Successfully updated story status to completed
```

### 2. 监控数据库锁状态

```sql
-- 查看当前被锁定的 Story
SELECT id, story_number, locked_by, locked_at
FROM user_stories
WHERE locked_by IS NOT NULL;

-- 查看锁超时的 Story（可能需要手动清理）
SELECT id, story_number, locked_by, locked_at
FROM user_stories
WHERE locked_at < datetime('now', '-30 minutes');
```

### 3. 压力测试

```typescript
// 在控制台快速启动 10 个 Worker
for (let i = 0; i < 10; i++) {
  await invoke('start_agent_worker', {
    projectId: 'my-project-id',
    checkInterval: 10  // 更频繁的查询
  })
}

// 查看所有 Workers
const workers = await invoke('list_agent_workers')
console.log(`Total workers: ${workers.length}`)
```

---

## ❓ 常见问题

### Q1: 多个 Worker 会重复处理同一个 Story 吗？
**A**: 不会。乐观锁确保只有一个 Worker 能成功锁定 Story，其他 Worker 会自动跳过。

### Q2: Worker 崩溃后，Story 会永远处于 locked 状态吗？
**A**: 不会。锁有 30 分钟超时，超时后其他 Worker 可以重新锁定该 Story。

### Q3: 如何调整查询间隔？
**A**: 在启动 Worker 时指定 `checkInterval` 参数，单位是秒。建议范围：10-300 秒。

### Q4: 如何持久化 Worker 状态？
**A**: 当前设计是重启后 Worker 丢失。如需持久化，可以将 Worker 配置保存到 SQLite，启动时自动恢复。

### Q5: 与旧的 DecentralizedAgentNode 有什么区别？
**A**: 
- **DecentralizedAgentNode** (旧): 通过 EventBus 接收事件，需要 SharedEventBus 和 SharedLockManager
- **AgentWorker** (新): 直接查询数据库，使用乐观锁，零外部依赖

---

## 📝 总结

**完全去中心化方案的核心价值**:
- ✅ **零外部依赖**: 无需安装 Redis 或维护 EventBus
- ✅ **完全自治**: 每个 Worker 独立运行，不依赖中心调度器
- ✅ **简单易用**: 开箱即用，无需复杂配置
- ✅ **高容错性**: 单个 Worker 崩溃不影响其他 Worker
- ✅ **易于扩展**: 动态增减 Worker 数量

**适用场景**:
- 单机部署的自动化代码生成系统
- 需要完全自治的智能体系统
- 追求极简架构的项目

**未来扩展**:
- 支持跨机器的分布式锁（引入 Redis）
- 支持更复杂的任务编排（引入工作流引擎）
- 支持 Worker 状态持久化（保存到数据库）

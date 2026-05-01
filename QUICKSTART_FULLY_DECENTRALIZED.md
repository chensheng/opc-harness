# 完全去中心化 Agent Worker - 快速开始

## 🚀 5 分钟快速测试

### Step 1: 启动开发服务器

```bash
npm run tauri:dev
```

**首次编译**: 约 3-5 分钟 (下载和编译依赖)  
**后续启动**: 约 20-30 秒 (增量编译)

---

### Step 2: 访问测试页面

浏览器打开: `http://localhost:1420/decentralized-worker-test`

或者直接点击应用内的导航链接 (如果已添加菜单项)

---

### Step 3: 启动多个 Worker

在测试页面上:

1. **输入 Project ID** (必填)
2. **点击"启动 Worker"按钮** → 创建 worker-1
3. **再次点击"启动 Worker"按钮** → 创建 worker-2
4. **观察 Worker 列表** → 应该看到 2 个运行中的 Worker

---

### Step 4: 验证功能

#### 预期行为

✅ **每个 Worker 独立运行**
- worker-1 和 worker-2 有唯一的 ID
- 状态显示为"运行中" (绿色标签)

✅ **零外部依赖**
- 不需要 Redis
- 不需要 EventBus
- 仅依赖 SQLite 数据库

✅ **可以独立停止**
- 点击任意 Worker 的停止按钮
- 该 Worker 停止，其他 Worker 不受影响

---

## 📊 监控日志输出

在终端中应该看到类似日志:

```log
[AgentWorkerCommand] Starting fully decentralized agent worker: worker-abc123 for project my-project-id
[AgentWorker:worker-abc123] Created with config: AgentWorkerConfig { ... }
[AgentWorker:worker-abc123] Worktree manager initialized for path: /path/to/workspaces
[AgentWorker:worker-abc123] 🚀 Starting independent agent loop
[AgentWorker:worker-abc123] Independent loop started (interval: 30s)
[AgentWorker:worker-abc123] 🔄 Starting execution cycle
[AgentWorker:worker-abc123] Found active Sprint: Sprint 1 (uuid-xxx)
[AgentWorker:worker-abc123] Found 3 pending story(s)
[AgentWorker:worker-abc123] 🔒 Locked story US-001: Implement authentication
[AgentWorker:worker-abc123] Starting coding agent for story US-001
[AgentWorker:worker-abc123] Worktree created at: .worktrees/agent-xxx
[AgentWorker:worker-abc123] AI CLI process spawned with PID: 12345
[AICLI:agent-xxx] Starting to listen for agent output
[StoryStatus] Updating story uuid-of-us-001 status to completed
[DB::complete_user_story] Completed story: uuid-of-us-001
[AgentWorker:worker-abc123] Successfully updated story status to completed
```

---

## 🧪 手动测试 (浏览器控制台)

打开浏览器开发者工具 (F12),在 Console 中执行:

```javascript
// 1. 导入 Hook
import { useAgentWorkers } from './hooks/useAgentWorkers'

// 注意: 实际使用时需要在 React 组件中调用
// 这里仅用于演示 API

// 2. 通过 Tauri invoke 直接调用
const workerId = await invoke('start_agent_worker', {
  projectId: 'my-project-id',
  checkInterval: 30
})
console.log('Started worker:', workerId)

// 3. 列出所有 Workers
const workers = await invoke('list_agent_workers')
console.log('All workers:', workers)

// 4. 停止 Worker
await invoke('stop_agent_worker', { workerId: workerId })
console.log('Stopped worker')
```

---

## 🎯 完整测试场景

### 场景: 模拟 Story 竞争处理

```rust
// 伪代码演示
1. 用户创建 Story US-001 (status='approved')
   ↓
2. Worker #1, Worker #2, Worker #3 同时查询数据库
   ↓
3. 各 Worker 尝试乐观锁
   - Worker #1: UPDATE ... WHERE locked_by IS NULL → ✅ 成功
   - Worker #2: UPDATE ... WHERE locked_by IS NULL → ❌ 失败 (已被锁定)
   - Worker #3: UPDATE ... WHERE locked_by IS NULL → ❌ 失败 (已被锁定)
   ↓
4. Worker #1 继续执行
   - 创建 Worktree
   - 启动 AI CLI
   - 生成代码
   - Git commit & push
   - 更新 Story 状态为 completed
   ↓
5. Worker #2 和 Worker #3 放弃，等待下一个查询周期
```

---

## 🔍 调试技巧

### 1. 查看数据库锁状态

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

### 2. 监控 Worker 状态

在测试页面上可以看到：
- **总 Worker 数**: 当前创建的 Worker 总数
- **运行中**: 正在运行的 Worker 数量
- **处理 Story 中**: 正在执行任务的 Worker 数量

### 3. 压力测试

```javascript
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

### Q1: Worker 启动后立即停止
**A**: 检查日志是否有错误。可能是 Daemon Manager 未初始化或 Worktree Manager 配置错误。

### Q2: 多个 Worker 同时处理同一个 Story
**A**: 这是正常的竞争行为。乐观锁确保只有一个 Worker 能成功获取锁，其他 Worker 会自动放弃。

### Q3: 如何调整查询间隔？
**A**: 在启动 Worker 时指定 `checkInterval` 参数，单位是秒。建议范围：10-300 秒。

### Q4: Worker 崩溃后，Story 会永远处于 locked 状态吗？
**A**: 不会。锁有 30 分钟超时，超时后其他 Worker 可以重新锁定该 Story。

---

## 📈 性能基准测试

### 数据库查询延迟测试

```rust
#[tokio::test]
async fn test_database_query_latency() {
    let conn = db::get_connection().unwrap();
    
    let start = Instant::now();
    let stories = db::get_pending_stories_by_sprint(&conn, "sprint-id").unwrap();
    let elapsed = start.elapsed();
    
    println!("Database query latency: {:?}", elapsed);
    assert!(elapsed < Duration::from_millis(10)); // 应该 < 10ms
}
```

### 乐观锁获取延迟测试

```rust
#[tokio::test]
async fn test_optimistic_lock_latency() {
    let conn = db::get_connection().unwrap();
    
    let start = Instant::now();
    let locked = db::lock_user_story(&conn, "story-id", "worker-1").unwrap();
    let elapsed = start.elapsed();
    
    println!("Optimistic lock acquisition latency: {:?}", elapsed);
    assert!(locked);
    assert!(elapsed < Duration::from_millis(10)); // 应该 < 10ms
}
```

---

## 🎉 下一步

测试完成后，你可以:

1. **集成到主界面**: 将 DecentralizedWorkerTestPage 添加到 Dashboard
2. **自定义查询间隔**: 根据项目需求调整 Worker 的查询频率
3. **监控性能**: 观察多 Worker 并发时的数据库负载
4. **扩展功能**: 添加更多 Worker 管理功能（如批量启动/停止）

---

## 📝 总结

**完全去中心化 Agent Worker 的核心价值**:
- ✅ **零外部依赖**: 无需安装 Redis 或维护 EventBus
- ✅ **完全自治**: 每个 Worker 独立运行，不依赖中心调度器
- ✅ **简单易用**: 开箱即用，无需复杂配置
- ✅ **高容错性**: 单个 Worker 崩溃不影响其他 Worker
- ✅ **易于扩展**: 动态增减 Worker 数量

**立即开始测试吧！** 🚀

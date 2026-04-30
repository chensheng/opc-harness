# 去中心化智能体系统 - 快速开始

## 🚀 5 分钟快速测试

### Step 1: 启动开发服务器

```bash
npm run tauri:dev
```

**首次编译**: 约 3-5 分钟 (下载和编译依赖)  
**后续启动**: 约 20-30 秒 (增量编译)

---

### Step 2: 访问测试页面

浏览器打开: `http://localhost:1420/decentralized-test`

或者直接点击应用内的导航链接 (如果已添加菜单项)

---

### Step 3: 启动多个 Node

在测试页面上:

1. **点击"启动新 Node"按钮** → 创建 node-1
2. **再次点击"启动新 Node"按钮** → 创建 node-2
3. **观察节点列表** → 应该看到 2 个运行中的 Node

---

### Step 4: 验证功能

#### 预期行为

✅ **每个 Node 独立运行**
- node-1 和 node-2 有唯一的 ID
- 状态显示为"运行中" (绿色圆点)

✅ **共享资源**
- 两个 Node 共享同一个 EventBus
- 两个 Node 共享同一个 LockManager

✅ **可以独立停止**
- 点击任意 Node 的停止按钮
- 该 Node 停止,其他 Node 不受影响

---

## 📊 监控日志输出

在终端中应该看到类似日志:

```log
[DecentralizedNode] Created node node-abc123 with config: NodeConfig { ... }
[DecentralizedNode:node-abc123] Starting decentralized agent loop
[DecentralizedNode:node-abc123] Event listener started
[EventBus] Published event: story.created from node abc123
[Lock] Node node-abc123 acquired lock for story US-001
```

---

## 🧪 手动测试 (浏览器控制台)

打开浏览器开发者工具 (F12),在 Console 中执行:

```javascript
// 1. 导入 Hook
import { useDecentralizedNodes } from './hooks/useDecentralizedNodes'

// 注意: 实际使用时需要在 React 组件中调用
// 这里仅用于演示 API

// 2. 通过 Tauri invoke 直接调用
const nodeId = await invoke('start_decentralized_node', {
  nodeId: 'test-node-1',
  maxConcurrent: 3
})
console.log('Started node:', nodeId)

// 3. 列出所有 Nodes
const nodes = await invoke('list_decentralized_nodes')
console.log('All nodes:', nodes)

// 4. 停止 Node
await invoke('stop_decentralized_node', { nodeId: 'test-node-1' })
console.log('Stopped node')
```

---

## 🎯 完整测试场景

### 场景: 模拟 Story 竞争处理

```rust
// 伪代码演示
1. 用户创建 Story US-001
   ↓
2. EventBus.publish("story.created", { story_id: "US-001" })
   ↓
3. node-1 和 node-2 同时收到事件
   ↓
4. node-1: should_accept_story() → true (负载低)
   node-2: should_accept_story() → false (负载高)
   ↓
5. node-1: try_lock_story("US-001") → ✅ 成功
   ↓
6. node-1: 创建 Worktree,启动 Agent
   ↓
7. node-2: 放弃,等待下一个 Story
```

---

## 🔍 调试技巧

### 1. 查看共享资源状态

在 Rust 代码中添加临时日志:

```rust
// 在 event_bus.rs 中
pub fn publish(&self, ...) {
    log::info!("[EventBus] Total subscribers: {}", self.sender.receiver_count());
    // ...
}

// 在 distributed_lock.rs 中
pub async fn try_lock_story(&self, ...) {
    log::info!("[LockManager] Current locks: {}", self.locks.len());
    // ...
}
```

### 2. 监控内存使用

```bash
# Windows 任务管理器
# 查看 opc-harness.exe 的内存占用

# 或使用 PowerShell
Get-Process opc-harness | Select-Object WorkingSet, PrivateMemorySize
```

### 3. 压力测试

```javascript
// 在控制台快速启动 10 个 Node
for (let i = 0; i < 10; i++) {
  await invoke('start_decentralized_node', {
    nodeId: `stress-test-${i}`,
    maxConcurrent: 1
  })
}

// 查看所有 Nodes
const nodes = await invoke('list_decentralized_nodes')
console.log(`Total nodes: ${nodes.length}`)
```

---

## ❓ 常见问题

### Q1: 启动 Node 时报错 "Failed to connect to Redis"
**A**: 我们使用的是纯内存实现,不需要 Redis。检查是否使用了旧版本的代码。

### Q2: Node 启动后立即停止
**A**: 检查日志是否有错误。可能是 Daemon Manager 未初始化或 Worktree Manager 配置错误。

### Q3: 多个 Node 同时处理同一个 Story
**A**: 这是正常的竞争行为。分布式锁确保只有一个 Node 能成功获取锁,其他 Node 会自动放弃。

### Q4: 如何持久化 Node 状态?
**A**: 当前设计是重启后状态丢失。如需持久化,可以将 Node 配置保存到 SQLite,启动时自动恢复。

---

## 📈 性能基准测试

### 事件发布延迟测试

```rust
#[tokio::test]
async fn test_event_bus_latency() {
    let bus = Arc::new(SharedEventBus::new());
    let mut rx = bus.subscribe();
    
    let start = Instant::now();
    bus.publish(EventType::StoryCreated, json!({}), "test").unwrap();
    rx.recv().await.unwrap();
    let elapsed = start.elapsed();
    
    println!("Event latency: {:?}", elapsed);
    assert!(elapsed < Duration::from_micros(10)); // 应该 < 10μs
}
```

### 锁获取延迟测试

```rust
#[tokio::test]
async fn test_lock_latency() {
    let lock_mgr = SharedLockManager::new(600);
    
    let start = Instant::now();
    let acquired = lock_mgr.try_lock_story("test-story", "node-1").await.unwrap();
    let elapsed = start.elapsed();
    
    println!("Lock acquisition latency: {:?}", elapsed);
    assert!(acquired);
    assert!(elapsed < Duration::from_micros(10)); // 应该 < 10μs
}
```

---

## 🎉 下一步

测试完成后,你可以:

1. **集成到主界面**: 将 DecentralizedNodesPanel 添加到 Dashboard
2. **实现 Story 监听**: 让 Node 真正监听并处理数据库中的 Story
3. **添加负载均衡**: 根据 CPU/内存使用情况动态调整并发数
4. **实现故障转移**: 当某个 Node 崩溃时,其他 Node 自动接管任务

需要我帮你实现这些功能吗?

# 单机去中心化智能体系统架构 (无 Redis)

## 🎯 设计目标

在单台机器上实现多个独立的 Agent Loop 实例并发运行,无需外部依赖(如 Redis),完全基于内存实现。

---

## 📐 核心架构

```
┌─────────────────────────────────────────────────┐
│           AgentManager (全局单例)                │
│                                                   │
│  ┌──────────────────────────────────────────┐   │
│  │  SharedEventBus (共享事件总线)            │   │
│  │  - tokio::sync::broadcast::channel(1000) │   │
│  └──────────────────────────────────────────┘   │
│                                                   │
│  ┌──────────────────────────────────────────┐   │
│  │  SharedLockManager (共享锁管理器)         │   │
│  │  - DashMap<String, Mutex<LockInfo>>      │   │
│  └──────────────────────────────────────────┘   │
└────┬──────────┬──────────┬──────────┬──────────┘
     │          │          │          │
┌────▼───┐ ┌───▼────┐ ┌───▼────┐ ┌──▼────┐
│Node 1  │ │Node 2  │ │Node 3  │ │Node N │  ← 多个独立的 Agent Loop
│        │ │        │ │        │ │       │
│引用    │ │引用    │ │引用    │ │引用   │
│EventBus│ │EventBus│ │EventBus│ │EventBus│
│LockMgr │ │LockMgr │ │LockMgr │ │LockMgr│
└────────┘ └────────┘ └────────┘ └───────┘
```

---

## 🔧 核心组件

### 1. SharedEventBus (共享事件总线)

**实现**: `tokio::sync::broadcast::channel`

```rust
pub struct SharedEventBus {
    sender: broadcast::Sender<EventMessage>,
}

impl SharedEventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000); // 容量 1000 条消息
        Self { sender }
    }
    
    pub fn publish(&self, event_type: EventType, payload: impl Serialize, source_node_id: &str) {
        self.sender.send(message)?;
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<EventMessage> {
        self.sender.subscribe()
    }
}
```

**特性**:
- ✅ **一对多广播**: 一个 Node 发布,所有 Node 接收
- ✅ **异步非阻塞**: 不阻塞发布者
- ✅ **容量限制**: 最多缓存 1000 条消息,超出后丢弃旧消息
- ✅ **零外部依赖**: 纯 Tokio 实现

---

### 2. SharedLockManager (共享锁管理器)

**实现**: `DashMap<String, Arc<Mutex<LockInfo>>>`

```rust
pub struct SharedLockManager {
    locks: DashMap<String, Arc<tokio::sync::Mutex<LockInfo>>>,
    default_ttl_secs: u64,
}

struct LockInfo {
    node_id: String,
    acquired_at: Instant,
    ttl_secs: u64,
}
```

**特性**:
- ✅ **原子操作**: DashMap 保证插入/删除的线程安全
- ✅ **TTL 自动过期**: 锁超过 TTL 后自动失效
- ✅ **续期支持**: 长任务可调用 `renew_lock()` 延长锁时间
- ✅ **零外部依赖**: 纯内存实现

---

### 3. DecentralizedAgentNode (去中心化节点)

每个 Node 实例持有:
- `EventBusClient`: 引用共享的 EventBus
- `SharedLockManager`: 引用共享的 LockManager
- 本地配置: `node_id`, `max_concurrent_agents`

**工作流程**:
```rust
// 1. Node 启动时订阅事件
let mut event_rx = self.event_bus.subscribe();

// 2. 监听 story.created 事件
while let Ok(message) = event_rx.recv().await {
    if message.event_type == "story.created" {
        // 3. 自主决策是否处理
        if !self.should_accept_story().await {
            continue;
        }
        
        // 4. 尝试获取分布式锁
        if self.lock_manager.try_lock_story(story_id, node_id).await? {
            // 5. 创建 Worktree 并启动 Agent
            self.process_story(story_id).await?;
        }
    }
}
```

---

## 🔄 完整协作流程

### 场景: 3 个 Node 竞争处理 Story US-001

```
时间线:
T0: 用户创建 Story US-001
    ↓
    EventBus.publish("story.created", {story_id: "US-001"})
    ↓
T1: Node 1, Node 2, Node 3 同时收到事件
    ↓
T2: 各 Node 自主决策
    - Node 1: 当前负载低 → 决定接受 ✓
    - Node 2: 当前负载高 → 拒绝 ✗
    - Node 3: 随机决策 → 决定接受 ✓
    ↓
T3: Node 1 和 Node 3 竞争锁
    - Node 1: try_lock_story("US-001") → ✅ 成功
    - Node 3: try_lock_story("US-001") → ❌ 失败 (已被 Node 1 锁定)
    ↓
T4: Node 1 继续执行
    - 创建 Worktree
    - 启动 Agent
    - 发布 "agent.started" 事件
    ↓
T5: Node 3 放弃,等待下一个 Story
```

---

## 💡 技术优势

| 维度 | Redis 方案 | 纯内存方案 |
|------|-----------|-----------|
| **外部依赖** | ❌ 需要安装 Redis | ✅ 零依赖 |
| **部署复杂度** | ❌ 需配置 Redis 服务 | ✅ 开箱即用 |
| **性能** | ⚠️ 网络延迟 (~1ms) | ✅ 内存访问 (<1μs) |
| **持久化** | ✅ Redis RDB/AOF | ❌ 重启丢失 (可接受) |
| **跨进程** | ✅ 支持多进程共享 | ❌ 仅单进程 |
| **适用场景** | 分布式集群 | 单机多实例 |

---

## 🎯 适用场景

### ✅ 适合使用纯内存方案
- 单机部署,无需跨机器通信
- Agent Loop 实例在同一进程中
- 重启后状态可重建 (从数据库恢复)
- 追求极简部署,不想维护 Redis

### ❌ 不适合使用纯内存方案
- 需要跨多台机器部署
- 多个独立进程需要共享状态
- 要求锁信息持久化 (重启后保留)
- 已有 Redis 基础设施

---

## 📊 性能对比

**事件发布延迟**:
- Redis Pub/Sub: ~1-5ms (网络往返)
- Tokio Broadcast: <1μs (内存拷贝)

**锁获取延迟**:
- Redis SET NX: ~1-3ms
- DashMap Insert: <1μs

**吞吐量**:
- Redis: ~10K ops/sec (受网络限制)
- Memory: ~1M ops/sec (CPU 限制)

---

## 🔮 未来扩展

如果需要扩展到多机部署,可以:

1. **保持接口不变**: `SharedEventBus` 和 `SharedLockManager` 定义为 Trait
2. **替换实现**: 
   - `RedisEventBus` 实现 `EventBusTrait`
   - `RedisLockManager` 实现 `LockManagerTrait`
3. **配置驱动**: 通过配置文件选择使用内存还是 Redis

```rust
pub trait EventBusTrait {
    fn publish(&self, event_type: EventType, payload: impl Serialize) -> Result<(), String>;
    fn subscribe(&self) -> EventStream;
}

// 内存实现
pub struct InMemoryEventBus { ... }
impl EventBusTrait for InMemoryEventBus { ... }

// Redis 实现 (未来)
pub struct RedisEventBus { ... }
impl EventBusTrait for RedisEventBus { ... }
```

---

## 🚀 快速开始

### 1. 启动第一个 Node

```typescript
// 前端调用
const nodeId1 = await invoke('start_decentralized_node', {
  nodeId: 'node-1',
  maxConcurrent: 3,
})
```

### 2. 启动第二个 Node

```typescript
const nodeId2 = await invoke('start_decentralized_node', {
  nodeId: 'node-2',
  maxConcurrent: 5,
})
```

### 3. 查看运行中的 Nodes

```typescript
const nodes = await invoke('list_decentralized_nodes')
console.log(nodes)
// [
//   { node_id: 'node-1', is_running: true },
//   { node_id: 'node-2', is_running: true }
// ]
```

### 4. 停止 Node

```typescript
await invoke('stop_decentralized_node', { nodeId: 'node-1' })
```

---

## 📝 总结

**纯内存去中心化方案的核心价值**:
- ✅ **零外部依赖**: 无需安装 Redis,降低运维成本
- ✅ **高性能**: 内存操作比网络请求快 1000 倍
- ✅ **简单易用**: 开箱即用,无需配置
- ✅ **足够可靠**: 对于单机场景,内存方案完全够用

**局限性**:
- ❌ 不支持跨进程/跨机器
- ❌ 重启后状态丢失 (但可从数据库恢复)

**推荐场景**: 单机多实例部署,追求极简架构的项目。

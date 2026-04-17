# Vibe Coding 智能体并行执行流程设计文档

> **版本**: v3.0 (极简版)  
> **最后更新**: 2024-04-17  
> **状态**: 设计中  
> **作者**: OPC-HARNESS Team

---

## 📋 目录

- [1. 概述](#1-概述)
- [2. 核心架构](#2-核心架构)
- [3. 完整执行流程](#3-完整执行流程)
- [4. 技术实现细节](#4-技术实现细节)
- [5. 异常处理与容错](#5-异常处理与容错)
- [6. 监控与可观测性](#6-监控与可观测性)
- [7. 性能优化策略](#7-性能优化策略)
- [8. 安全与权限控制](#8-安全与权限控制)
- [9. 附录](#9-附录)

---

## 1. 概述

### 1.1 设计目标

本方案旨在实现**完全自动化的多智能体并行开发流程**，通过 Git Worktree 隔离和即时合并机制，让多个 AI 智能体能够同时、独立地执行用户故事（User Story），每个故事完成后立即合并到主分支，实现"一人公司"的愿景。

**核心价值**：
- ✅ **零人工干预**: 从故事选择到代码合并全流程自动化
- ✅ **高效并行**: 基于系统负载动态调整并发数，最大化资源利用
- ✅ **安全可靠**: Worktree 隔离 + 自动测试 + 即时合并机制
- ✅ **可追溯**: 完整的审计日志和 Git 历史记录
- ✅ **极简流程**: 移除批次和选择器，基于时间自动获取当前 Sprint

### 1.2 适用范围

本文档适用于 OPC-HARNESS 项目的 **Vibe Coding** 模块，具体场景包括：
- 基于时间的 Sprint 自动执行
- 多智能体并行编码任务调度
- 自动化代码生成、测试、审查和即时合并

### 1.3 关键术语

| 术语 | 定义 |
|------|------|
| **Agent** | AI 编码智能体实例，负责执行单个用户故事的完整开发生命周期 |
| **Worktree** | Git 工作树，为每个 Agent 提供独立的文件系统环境 |
| **Agent Pool** | 一组持久化的 Agent 实例集合，支持动态任务分配 |
| **Database Task Coordinator** | 数据库任务协调器，通过乐观锁实现分布式任务竞争 |
| **Instant Merge** | 即时合并，每个故事完成后立即合并到 main 分支 |
| **Time-based Sprint** | 基于时间周期的 Sprint，智能体自动获取当前周期内的故事 |
| **Optimistic Locking** | 乐观锁机制，通过 `UPDATE ... WHERE` + `FOR UPDATE SKIP LOCKED` 确保任务唯一性 |

---

## 2. 核心架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│              Time-based Sprint Detection                     │
│         (根据当前时间自动获取活跃 Sprint)                      │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│          Load Current Sprint Stories (数据层)                │
│  - 查询数据库中当前时间范围内的 Sprint                        │
│  - 获取该 Sprint 下所有 pending 状态的故事                    │
│  - 按优先级排序                                              │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│         Database Task Coordinator (任务协调层)               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  SQLite Database (user_stories table)               │   │
│  │  - status: pending/in_progress/completed/failed     │   │
│  │  - assigned_agent: Agent ID                         │   │
│  │  - locked_at: timestamp for timeout detection       │   │
│  │  Optimistic Locking via UPDATE ... WHERE          │   │
│  └─────────────────────────────────────────────────────┘   │
└────────────┬────────────────────────┬──────────────────────┘
             │                        │
             ▼                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Agent Pool Manager (执行层)                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │ Agent-01 │  │ Agent-02 │  │ Agent-03 │  ...            │
│  │Worktree-A│  │Worktree-B│  │Worktree-C│                 │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                 │
│       │              │              │                       │
│       └──────────────┼──────────────┘                       │
│                      │                                      │
│    Pull-based Claim: UPDATE user_stories SET ...           │
│    WHERE id = (SELECT ... FOR UPDATE SKIP LOCKED)          │
└──────────────────────┼──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│              Instant Merge to Main (合并层)                  │
│  - 每个故事完成后立即合并                                    │
│  - AI 辅助冲突解决                                           │
│  - 合并后重置 Agent 分支                                     │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│              Report & Cleanup (收尾层)                       │
│  - 生成统计报告                                              │
│  - 清理临时资源                                              │
│  - 通知用户（仅异常时）                                      │
└─────────────────────────────────────────────────────────────┘
```

**架构特点**:
- **数据库驱动**: 所有任务状态存储在 SQLite，支持分布式部署
- **乐观锁竞争**: Agent 通过原子 SQL 操作竞争任务，无需中央队列
- **无状态 Agent**: Agent 本身不维护任务列表，从数据库动态拉取
- **故障自愈**: 锁超时机制自动回收卡住的任务

### 2.2 核心组件

#### 2.2.1 Time-based Sprint Loader（基于时间的 Sprint 加载器）

**职责**: 根据当前时间自动获取活跃的 Sprint 及其用户故事

**工作原理**:
```rust
async fn load_current_sprint_stories() -> Result<Vec<UserStory>, String> {
    let now = Utc::now();
    
    // 1. 查询当前时间范围内的活跃 Sprint
    let sprint = db.query(
        "SELECT * FROM sprints 
         WHERE start_date <= ? AND end_date >= ? 
         AND status = 'active'
         ORDER BY created_at DESC 
         LIMIT 1",
        &[&now, &now],
    ).await?;
    
    if sprint.is_none() {
        return Err("No active sprint found for current time".to_string());
    }
    
    // 2. 获取该 Sprint 下所有 pending 状态的故事
    let stories = db.query(
        "SELECT * FROM user_stories 
         WHERE sprint_id = ? AND status = 'pending'
         ORDER BY priority DESC, created_at ASC",
        &[&sprint.unwrap().id],
    ).await?;
    
    Ok(stories)
}
```

**Sprint 时间配置示例**:
```
sprints:
  - id: sprint-2024-Q1-01
    name: "Q1 First Sprint"
    start_date: "2024-01-01T00:00:00Z"
    end_date: "2024-01-14T23:59:59Z"
    status: active
    
  - id: sprint-2024-Q1-02
    name: "Q1 Second Sprint"
    start_date: "2024-01-15T00:00:00Z"
    end_date: "2024-01-28T23:59:59Z"
    status: planned
```

**特性**:
- **自动检测**: 无需手动触发，根据系统时间自动识别当前 Sprint
- **简单直接**: 移除复杂的评分和依赖分析逻辑
- **时间驱动**: Sprint 的生命周期由起止时间决定

#### 2.2.2 Agent Pool Manager（智能体池管理器）

**职责**: 管理 Agent 实例的生命周期和资源分配

**核心功能**:
- **动态并发控制**: 根据 CPU/内存使用率自动调整 Agent 数量
  ```
  并发数 = min(CPU核心数/2, 可用内存GB/2, 配置上限)
  ```
- **Worktree 管理**: 创建/销毁/监控持久化 Worktrees
- **健康检查**: 心跳监控、故障检测、自动重启
- **负载均衡**: 监控各 Agent 负载，动态调整任务分配

#### 2.2.3 Database Task Coordinator（数据库任务协调器）

**职责**: 通过数据库原子操作实现分布式任务竞争和状态管理

**核心机制**:
- **乐观锁竞争**: 使用 `UPDATE ... WHERE` + `FOR UPDATE SKIP LOCKED` 确保只有一个 Agent 能锁定同一故事
- **无状态设计**: 无需维护内存队列，所有状态持久化在数据库中
- **自动故障恢复**: 基于时间戳的锁超时检测，自动回收卡住的任务
- **优先级调度**: 通过 SQL `ORDER BY priority DESC` 实现高优先级优先

**数据结构** (存储在 `user_stories` 表):
```typescript
interface UserStory {
  id: string;
  storyNumber: string;           // "US-001"
  title: string;
  priority: number;              // 0-100
  storyPoints: number;
  sprintId: string;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  assignedAgent?: string;        // 当前执行的 Agent ID
  lockedAt?: string;             // 锁定时间戳（用于超时检测）
  startedAt?: string;
  completedAt?: string;
  failedAt?: string;
  errorMessage?: string;
  retryCount: number;            // 重试次数
  dependencies: string[];        // 依赖的故事 ID 列表
}
```

**关键优势**:
- ✅ **分布式友好**: 支持跨机器部署多个 Agent 实例
- ✅ **容错性强**: Agent 崩溃不影响其他 Agent，锁超时自动恢复
- ✅ **简化架构**: 移除中央队列服务，降低系统复杂度
- ✅ **可追溯**: 所有状态变更都有数据库审计日志

#### 2.2.4 Coding Agent（编码智能体）

**职责**: 在专属 Worktree 中执行用户故事的完整开发生命周期

**执行阶段**:
1. **Analyzing**: 解析 PRD，制定技术方案
2. **Coding**: 调用 AI Provider 生成代码
3. **Testing**: 运行单元测试，自动修复失败
4. **Reviewing**: 执行 Lint/TypeScript 检查
5. **Committing**: 自动生成 Commit Message 并提交
6. **Merging**: 立即合并到 main 分支并重置

**特性**:
- **完全自主**: 无需用户确认任何操作
- **即时合并**: 每个故事完成后立即合并到 main
- **错误恢复**: 自动重试、降级、隔离

---

## 3. 完整执行流程

### 3.1 阶段 1: 基于时间的 Sprint 自动加载

#### 3.1.1 时间驱动的 Sprint 检测

系统根据当前时间自动识别活跃的 Sprint，无需手动触发：

```rust
async fn detect_active_sprint() -> Result<Option<Sprint>, String> {
    let now = Utc::now();
    
    // 查询当前时间在范围内的活跃 Sprint
    let sprint = db.query_one(
        "SELECT * FROM sprints 
         WHERE start_date <= ? AND end_date >= ? 
         AND status = 'active'
         ORDER BY created_at DESC 
         LIMIT 1",
        &[&now, &now],
    ).await?;
    
    Ok(sprint)
}
```

**Sprint 生命周期**:
- **Planned**: 已规划但未开始
- **Active**: 当前时间处于 start_date 和 end_date 之间
- **Completed**: 已结束
- **Cancelled**: 已取消

#### 3.1.2 加载用户故事

获取当前 Sprint 下所有待执行的故事：

```rust
async fn load_sprint_stories(sprint_id: &str) -> Result<Vec<UserStory>, String> {
    let stories = db.query(
        "SELECT * FROM user_stories 
         WHERE sprint_id = ? AND status IN ('pending', 'in_progress')
         ORDER BY priority DESC, story_number ASC",
        &[&sprint_id],
    ).await?;
    
    info!("Loaded {} stories for Sprint {}", stories.len(), sprint_id);
    Ok(stories)
}
```

**故事状态流转**:
```
pending → locked → in_progress → completed/failed
```

#### 3.1.3 资源预检

在启动 Agent Pool 前进行系统性检查：

```rust
async fn preflight_check() -> Result<PreflightReport, String> {
    let checks = vec![
        check_git_environment(),
        check_ai_provider_config(),
        check_disk_space(MIN_DISK_SPACE_GB),
        check_system_resources(),
    ];
    
    for check in checks {
        if let Err(e) = check.await {
            return Err(format!("Preflight check failed: {}", e));
        }
    }
    
    Ok(PreflightReport::all_passed())
}
```

**检查项**:
- ✅ Git 环境是否正常（版本、配置）
- ✅ AI Provider API Key 是否有效
- ✅ 磁盘空间是否充足（至少 10GB）
- ✅ CPU/内存使用率是否在安全范围

### 3.2 阶段 2: Agent Pool 初始化

#### 3.2.1 确定并发规模

基于系统资源动态计算：

```typescript
function calculateConcurrency(): number {
  const cpuCores = os.cpus().length;
  const availableMemoryGB = getAvailableMemory() / (1024 * 1024 * 1024);
  
  const cpuBased = Math.floor(cpuCores / 2);
  const memoryBased = Math.floor(availableMemoryGB / 2);
  const configLimit = CONFIG.maxConcurrency || 8;
  
  return Math.min(cpuBased, memoryBased, configLimit);
}
```

**示例**:
- 8 核 CPU + 16GB 内存 → 并发 4 个 Agent
- 16 核 CPU + 32GB 内存 → 并发 8 个 Agent
- 低配机器（4 核 + 8GB）→ 并发 2 个 Agent

#### 3.2.2 创建持久化 Worktrees

为每个 Agent 创建独立的 Worktree：

```bash
# 创建 N 个持久化 Worktrees
for i in $(seq 1 $CONCURRENCY); do
  agent_id=$(printf "agent-%03d" $i)
  branch_name="agent-pool/${agent_id}"
  worktree_path="worktree/${agent_id}"
  
  git worktree add -b ${branch_name} ${worktree_path} main
  
  # 初始化环境
  cd ${worktree_path}
  npm install
  git config user.name "AI Agent ${agent_id}"
  git config user.email "agent-${agent_id}@opc-harness.local"
done
```

**Worktree 结构**:
```
project-root/
├── .git/
├── worktree/
│   ├── agent-001/  (branch: agent-pool/agent-001)
│   ├── agent-002/  (branch: agent-pool/agent-002)
│   ├── agent-003/  (branch: agent-pool/agent-003)
│   └── agent-004/  (branch: agent-pool/agent-004)
├── src/
├── .git/
├── worktree/
│   ├── agent-001/  (branch: agent-pool/agent-001)
│   ├── agent-002/  (branch: agent-pool/agent-002)
│   ├── agent-003/  (branch: agent-pool/agent-003)
│   └── agent-004/  (branch: agent-pool/agent-004)
├── src/
├── package.json
└── ...
```

#### 3.2.3 启动 Agent 实例

为每个 Worktree 启动一个 Coding Agent 进程：

**初始化流程**:
1. 遍历并发数，为每个 Agent 生成唯一 ID（如 agent-001, agent-002）
2. 验证对应的 Worktree 目录存在
3. 创建 AgentInstance 对象，绑定 worktree 路径和分支名
4. 注册到 Agent Manager
5. 设置初始状态为 idle

```
async fn initialize_agent_pool(concurrency: usize) -> Result<Vec<AgentInstance>, String> {
    let mut agents = Vec::new();
    
    for i in 1..=concurrency {
        let agent_id = format!("agent-{:03}", i);
        let worktree_path = format!("worktree/{}", agent_id);
        let branch_name = format!("agent-pool/{}", agent_id);
        
        // 验证 Worktree 存在
        if !PathBuf::from(&worktree_path).exists() {
            return Err(format!("Worktree {} not found", worktree_path));
        }
        
        // 启动 Agent 进程
        let agent = AgentInstance::new(
            agent_id.clone(),
            worktree_path,
            branch_name,
        ).await?;
        
        // 注册到 Agent Manager
        agent_manager.register(agent.clone()).await?;
        
        agents.push(agent);
    }
    
    Ok(agents)
}
```


**Agent 初始状态**:
- `status`: idle
- `currentTask`: None
- `completedTasks`: 空列表
- `lastHeartbeat`: 当前时间

#### 3.2.4 构建任务队列

将加载的故事加入全局队列：

**任务队列构建流程**:
1. 从数据库查询当前 Sprint 的所有 pending 故事
2. 提取关键字段：storyId, priority, dependencies
3. 按优先级降序排序
4. 初始化 retryCount 为 0
5. 返回排序后的任务列表

```
async function buildTaskQueue(stories: UserStory[]): Promise<TaskQueue> {
  const queue = stories.map(story => ({
    storyId: story.id,
    priority: story.priority || 50,  // 直接使用故事自带的优先级
    dependencies: story.dependencies || [],
    status: 'pending',
    retryCount: 0,
  }));
  
  // 按优先级降序排序
  queue.sort((a, b) => b.priority - a.priority);
  
  return new TaskQueue(queue);
}
```


### 3.3 阶段 3: 基于数据库乐观锁的任务竞争

#### 3.3.1 数据库驱动的任务选择

每个 Agent 通过**数据库原子操作**直接竞争选择用户故事，无需内存队列：

**任务竞争机制**:
- 使用 `UPDATE ... WHERE` + `FOR UPDATE SKIP LOCKED` 原子操作
- 只有第一个执行成功的 Agent 能更新成功（影响行数 = 1）
- 其他 Agent 的 UPDATE 返回 0 行，表示竞争失败
- 自动跳过已被锁定的任务，无需重试逻辑

```
async fn claim_next_story(agent_id: &str) -> Result<Option<UserStory>, String> {
    let now = Utc::now();
    let lock_timeout = chrono::Duration::minutes(LOCK_TIMEOUT_MINUTES);
    
    // 使用 UPDATE ... WHERE 原子操作竞争任务
    // 只有第一个执行成功的 Agent 能更新成功（影响行数 = 1）
    let result = db.execute(
        "UPDATE user_stories 
         SET status = 'in_progress',
             assigned_agent = ?,
             locked_at = ?,
             updated_at = ?,
         WHERE id = (
             SELECT id FROM user_stories 
             WHERE sprint_id = (
                 SELECT id FROM sprints 
                 WHERE start_date <= ? AND end_date >= ? 
                 AND status = 'active'
                 LIMIT 1
             )
             AND status = 'pending'
             AND (
                 assigned_agent IS NULL 
                 OR (locked_at IS NOT NULL AND locked_at < ?)
             )
             ORDER BY priority DESC, story_number ASC
             LIMIT 1
             FOR UPDATE SKIP LOCKED
         )
         RETURNING *",
        &[&agent_id,
          &now,
          &now,
          &now,
          &now,
          &(now - lock_timeout),
        ],
    ).await?;
    
    if result.rows_affected() == 0 {
        // 没有可用任务
        return Ok(None);
    }
    
    // 查询刚锁定的故事详情
    let story = db.query_one(
        "SELECT * FROM user_stories WHERE assigned_agent = ? AND status = 'in_progress' ORDER BY updated_at DESC LIMIT 1",
        &[&agent_id],
    ).await?;
    
    Ok(story)
}
```

**核心 SQL 逻辑**:
```
UPDATE user_stories 
SET status = 'in_progress',
    assigned_agent = ?,
    locked_at = ?,
    updated_at = ?
WHERE id = (
    SELECT id FROM user_stories 
    WHERE sprint_id = (...)
      AND status = 'pending'
      AND (assigned_agent IS NULL OR locked_at < timeout_threshold)
    ORDER BY priority DESC, story_number ASC
    LIMIT 1
    FOR UPDATE SKIP LOCKED
)
RETURNING *;
```

**关键特性**:
- ✅ **原子性**: 数据库事务保证只有一个 Agent 能锁定同一故事
- ✅ **优先级调度**: ORDER BY priority DESC 确保高优先级任务先执行
- ✅ **锁超时检测**: locked_at < timeout_threshold 自动回收卡住的任务
- ✅ **无阻塞**: FOR UPDATE SKIP LOCKED 跳过已锁定行，避免等待
                // 4. 标记完成（更新数据库状态）
                mark_story_completed(&story.id, agent_id).await.ok();
            }
            Ok(None) => {
                // 没有可用任务，短暂休眠
                sleep(Duration::from_millis(500)).await;
            }
            Err(e) => {
                error!("Failed to claim story: {}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
```

**优势**:
- ✅ **无单点故障**: 不依赖中央 Task Queue 服务
- ✅ **天然分布式**: 多个 Agent 实例可跨机器部署
- ✅ **自动故障恢复**: Agent 崩溃后，锁超时自动释放任务
- ✅ **简化架构**: 移除内存队列的同步复杂度

#### 3.3.3 数据库表结构优化

```
-- 用户故事表（增强版）
CREATE TABLE user_stories (
    id TEXT PRIMARY KEY,
    story_number TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    acceptance_criteria TEXT,
    priority INTEGER NOT NULL DEFAULT 50,
    story_points INTEGER,
    sprint_id TEXT REFERENCES sprints(id),
    status TEXT NOT NULL DEFAULT 'pending',  -- pending | in_progress | completed | failed
    assigned_agent TEXT,                      -- 当前执行的 Agent ID
    locked_at TIMESTAMP,                      -- 锁定时间戳（用于超时检测）
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    failed_at TIMESTAMP,
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    dependencies TEXT,                        -- JSON 数组: ["US-001", "US-002"]
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引优化
CREATE INDEX idx_stories_sprint_status ON user_stories(sprint_id, status);
CREATE INDEX idx_stories_priority ON user_stories(priority DESC);
CREATE INDEX idx_stories_agent ON user_stories(assigned_agent);
CREATE INDEX idx_stories_locked_at ON user_stories(locked_at);
```

#### 3.3.4 锁超时与任务回收

自动检测并回收超时的任务：

```rust
async fn reclaim_timed_out_tasks() -> Result<usize, String> {
    let now = Utc::now();
    let lock_timeout = chrono::Duration::minutes(LOCK_TIMEOUT_MINUTES);
    let timeout_threshold = now - lock_timeout;
    
    // 找回超时且仍在进行中的任务
    let result = db.execute(
        "UPDATE user_stories 
         SET status = 'pending',
             assigned_agent = NULL,
             locked_at = NULL,
             retry_count = retry_count + 1,
             updated_at = ?
         WHERE status = 'in_progress'
           AND locked_at IS NOT NULL
           AND locked_at < ?",
        &[&now, &timeout_threshold],
    ).await?;
    
    let reclaimed_count = result.rows_affected();
    
    if reclaimed_count > 0 {
        warn!("Reclaimed {} timed-out tasks", reclaimed_count);
    }
    
    Ok(reclaimed_count as usize)
}
```

**定时清理任务**:
```
// 每 5 分钟执行一次超时任务回收
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        reclaim_timed_out_tasks().await.ok();
    }
});
```

**锁超时配置**:
- 默认超时时间: **30 分钟**
- 超时后自动将任务状态重置为 `pending`
- `retry_count` 递增，用于检测频繁失败的任务
- 防止 Agent 崩溃导致任务永久锁定

### 3.4 阶段 4: Agent 自主执行

#### 3.4.1 执行流程概览

每个 Agent 在其专属 Worktree 中顺序执行以下步骤：

```
┌─────────────────────────────────────────────┐
│  Agent-001 (Worktree: agent-001)           │
│  Branch: agent-pool/agent-001              │
├─────────────────────────────────────────────┤
│                                             │
│  1. 确保分支正确                           │
│     git checkout agent-pool/agent-001      │
│     ↓                                       │
│  2. 拉取任务 US-001                        │
│     ↓                                       │
│  3. Analyzing Phase (10%)                  │
│     - 解析 PRD                             │
│     - 分析现有代码                         │
│     - 制定技术方案                         │
│     ↓                                       │
│  4. Coding Phase (20%-70%)                 │
│     - 调用 AI Provider                     │
│     - 流式生成代码                         │
│     - 写入文件系统                         │
│     ↓                                       │
│  5. Testing Phase (70%-85%)                │
│     - 生成单元测试                         │
│     - 运行测试套件                         │
│     - 自动修复失败测试                     │
│     ↓                                       │
│  6. Reviewing Phase (85%-95%)              │
│     - 运行 ESLint/Prettier                 │
│     - TypeScript 类型检查                  │
│     - 自动修复 lint 错误                   │
│     ↓                                       │
│  7. Committing Phase (95%-100%)            │
│     - 生成 Commit Message                  │
│     - git add .                            │
│     - git commit -m "..."                  │
│     ↓                                       │
│  8. Merging Phase (100%) ⭐               │
│     - 合并到 main                          │
│     - 解决冲突（如有）                     │
│     - 推送到远程                           │
│     - 重置 Agent 分支                      │
│     ↓                                       │
│  9. 标记 US-001 完成                       │
│     ↓                                       │
│  10. 返回 idle，拉取 US-002                │
│     ↓                                       │
│  11. 重复步骤 3-10...                      │
│                                             │
└─────────────────────────────────────────────┘
```

#### 3.4.2 Analyzing Phase（分析阶段）

**目标**: 理解需求，制定实现方案

**步骤**:
1. **加载故事详情**
   ```rust
   let story = load_user_story(&task.story_id).await?;
   let prd = load_prd(&story.prd_id).await?;
   ```

2. **解析验收标准**
   - 提取功能需求
   - 识别技术约束
   - 确定依赖模块

3. **分析现有代码库**
   ```rust
   let codebase_analysis = analyze_codebase(&worktree_path).await?;
   // 扫描相关文件、函数、类
   ```

4. **制定技术方案**
   - 选择设计模式
   - 确定文件结构
   - 规划模块划分

5. **更新进度**: 10%

#### 3.4.3 Coding Phase（编码阶段）

**目标**: 生成符合需求的代码

**步骤**:
1. **构建 Prompt**
   ```rust
   let prompt = build_coding_prompt(
       &story,
       &prd,
       &codebase_analysis,
       &technical_plan,
   );
   ```

2. **调用 AI Provider**
   ```rust
   let ai_response = ai_provider.generate_code(&prompt).await?;
   // 流式接收代码片段
   ```

3. **写入文件系统**
   ```rust
   for file_change in ai_response.file_changes {
       let file_path = PathBuf::from(&worktree_path).join(&file_change.path);
       
       // 确保目录存在
       if let Some(parent) = file_path.parent() {
           tokio::fs::create_dir_all(parent).await?;
       }
       
       // 写入文件
       tokio::fs::write(&file_path, &file_change.content).await?;
   }
   ```

4. **遵循架构约束**
   - 参考 `AGENTS.md` 规范
   - 遵守项目目录结构
   - 使用约定的命名规范

5. **更新进度**: 20% → 70%

#### 3.4.4 Testing Phase（测试阶段）

**目标**: 确保代码质量，自动修复问题

**步骤**:
1. **生成单元测试**
   ```rust
   let test_files = generate_unit_tests(&story.acceptance_criteria).await?;
   write_test_files(&worktree_path, &test_files).await?;
   ```

2. **运行测试套件**
   ```bash
   cd worktree/agent-001
   npm test -- --passWithNoTests
   ```

3. **分析测试结果**
   ```rust
   let test_result = run_tests(&worktree_path).await?;
   
   if !test_result.passed {
       // 4. 自动修复失败的测试
       let fixed_code = ai_fix_test_failures(
           &test_result.failures,
           &current_code,
       ).await?;
       
       apply_fixes(&worktree_path, &fixed_code).await?;
       
       // 5. 重新运行测试（最多重试 3 次）
       for attempt in 1..=MAX_TEST_RETRIES {
           let retry_result = run_tests(&worktree_path).await?;
           if retry_result.passed {
               break;
           }
       }
   }
   ```

4. **更新进度**: 70% → 85%

#### 3.4.5 Reviewing Phase（审查阶段）

**目标**: 确保代码符合质量标准

**步骤**:
1. **运行 Linter**
   ```bash
   cd worktree/agent-001
   npm run lint
   ```

2. **TypeScript 类型检查**
   ```bash
   npx tsc --noEmit
   ```

3. **自动修复 Lint 错误**
   ```bash
   npm run lint:fix
   ```

4. **质量门禁检查**
   ```rust
   let quality_gate = QualityGate {
       eslint_errors: 0,
       typescript_errors: 0,
       test_coverage: 80.0, // 最低 80%
   };
   
   if !quality_gate.passed() {
       // 尝试自动修复
       auto_fix_quality_issues(&worktree_path).await?;
   }
   ```

5. **生成质量报告**
   ```rust
   let report = generate_quality_report(&worktree_path).await?;
   save_report(&task.story_id, &report).await?;
   ```

6. **Update progress**: 85% → 95%

#### 3.4.6 Committing Phase（提交阶段）

**目标**: 自动提交代码变更，无需用户确认

**步骤**:
1. **生成规范的 Commit Message**
   ```rust
   fn generate_commit_message(story: &UserStory, agent_id: &str) -> String {
       format!(
           "feat({}): {}\n\n{}\n\nAI Agent: {} | Story Points: {} | Sprint: {}",
           story.story_number.to_lowercase(),
           story.title.to_lowercase(),
           story.acceptance_criteria.join("\n- "),
           agent_id,
           story.story_points.unwrap_or(0),
           story.sprint_id.as_deref().unwrap_or("unknown"),
       )
   }
   ```

   **示例**:
   ```
   feat(us-001): implement user login feature
   
   - Add JWT authentication service
   - Create login form with validation
   - Implement session management
   - Add unit tests for auth flows
   
   AI Agent: agent-001 | Story Points: 5 | Sprint: sprint-2024-Q1-01
   ```

2. **暂存所有变更**
   ```bash
   cd worktree/agent-001
   git add .
   ```

3. **执行提交**
   ```bash
   git commit -m "feat(us-001): implement user login feature..."
   ```

4. **记录提交信息**
   ```rust
   let commit_hash = get_latest_commit_hash(&worktree_path).await?;
   record_commit(&task.story_id, &commit_hash, agent_id).await?;
   ```

5. **更新进度**: 95% → 100%

#### 3.4.7 Merging Phase（合并阶段）⭐ 新增

**目标**: 将当前故事立即合并到 main 分支，确保互斥执行

**关键机制**: **分布式合并锁** - 使用数据库原子操作确保同一时间只有一个 Agent 能执行合并

##### 3.4.7.1 获取合并锁

在开始合并前，先竞争获取全局合并锁：

```rust
async fn acquire_merge_lock(agent_id: &str) -> Result<bool, String> {
    let now = Utc::now();
    let lock_timeout = chrono::Duration::minutes(MERGE_LOCK_TIMEOUT_MINUTES);
    
    // 尝试获取合并锁（原子操作）
    let result = db.execute(
        "INSERT OR REPLACE INTO merge_lock (id, locked_by, locked_at, expires_at)
         VALUES ('global_merge_lock', ?, ?, ?)
         WHERE NOT EXISTS (
             SELECT 1 FROM merge_lock 
             WHERE id = 'global_merge_lock' 
             AND expires_at > ?
         )",
        &[
            &agent_id,
            &now,
            &(now + lock_timeout),
            &now,
        ],
    ).await?;
    
    // 如果影响行数为 1，说明成功获取锁
    Ok(result.rows_affected() == 1)
}
```

**锁表结构**:
```
CREATE TABLE merge_lock (
    id TEXT PRIMARY KEY DEFAULT 'global_merge_lock',
    locked_by TEXT NOT NULL,           -- 当前持有锁的 Agent ID
    locked_at TIMESTAMP NOT NULL,      -- 锁定时间
    expires_at TIMESTAMP NOT NULL,     -- 过期时间（自动释放）
    UNIQUE(id)
);

-- 初始化锁记录
INSERT OR IGNORE INTO merge_lock (id, locked_by, locked_at, expires_at)
VALUES ('global_merge_lock', NULL, NULL, NULL);
```

##### 3.4.7.2 带锁的合并流程

完整的合并流程，包含锁的竞争、同步和释放：

```rust
async fn merge_story_to_main(
    story_id: &str,
    agent_id: &str,
    worktree_path: &str,
) -> Result<(), String> {
    // 1. 竞争获取合并锁（最多重试 5 次，每次间隔 2 秒）
    let mut acquired = false;
    for attempt in 1..=MAX_MERGE_LOCK_RETRIES {
        match acquire_merge_lock(agent_id).await {
            Ok(true) => {
                acquired = true;
                info!("Agent {} acquired merge lock (attempt {})", agent_id, attempt);
                break;
            }
            Ok(false) => {
                warn!("Merge lock held by another agent, retrying... (attempt {}/{})", 
                      attempt, MAX_MERGE_LOCK_RETRIES);
                sleep(Duration::from_secs(MERGE_LOCK_RETRY_INTERVAL_SECS)).await;
            }
            Err(e) => {
                error!("Failed to acquire merge lock: {}", e);
                return Err(e);
            }
        }
    }
    
    if !acquired {
        return Err(format!(
            "Failed to acquire merge lock after {} attempts",
            MAX_MERGE_LOCK_RETRIES
        ));
    }
    
    // 2. 确保在函数退出时释放锁（即使发生错误）
    let _lock_guard = MergeLockGuard::new(agent_id.to_string());
    
    // 3. 【新增】先同步 main 分支的最新代码到 agent 分支
    sync_agent_branch_with_main(worktree_path).await?;
    
    // 4. 执行实际的合并操作（agent → main）
    execute_merge_operation(story_id, agent_id, worktree_path).await?;
    
    // 5. 锁会在 _lock_guard drop 时自动释放
    info!("Merge completed, lock released by Agent {}", agent_id);
    
    Ok(())
}
```

**锁守卫（RAII 模式）**:
```rust
struct MergeLockGuard {
    agent_id: String,
}

impl MergeLockGuard {
    fn new(agent_id: String) -> Self {
        Self { agent_id }
    }
}

impl Drop for MergeLockGuard {
    fn drop(&mut self) {
        // 自动释放锁
        tokio::spawn(async move {
            release_merge_lock(&self.agent_id).await.ok();
        });
    }
}

async fn release_merge_lock(agent_id: &str) -> Result<(), String> {
    db.execute(
        "UPDATE merge_lock 
         SET locked_by = NULL, 
             locked_at = NULL, 
             expires_at = NULL
         WHERE locked_by = ?",
        &[&agent_id],
    ).await?;
    
    Ok(())
}
```

##### 3.4.7.3 同步主分支变更到 Agent 分支 ⭐ 新增

**目标**: 在合并前先将 main 分支的最新代码同步到 agent 分支，减少主分支冲突

**步骤**:

1. **切换到 agent 分支**
   ```bash
   cd worktree/agent-001
   git checkout agent-pool/agent-001
   ```

2. **拉取最新的 main 分支**
   ```bash
   git fetch origin main
   ```

3. **将 main 合并到 agent 分支**
   ```bash
   git merge origin/main --no-ff -m "chore: sync main into agent branch"
   ```

4. **冲突处理（在此阶段解决）**
   ```rust
   if has_merge_conflicts().await? {
       info!("Conflict detected during sync, resolving with AI assistance...");
       
       // AI 辅助解决冲突
       let ours = get_ours_version().await?;
       let theirs = get_theirs_version().await?;
       let context = get_merge_context().await?;
       
       let resolved = ai_resolve_conflict(&ours, &theirs, &context).await?;
       apply_resolution(&resolved).await?;
       
       git add . && git commit -m "chore: resolve sync conflicts from main";
   }
   ```

5. **验证同步结果**
   ```bash
   # 确保编译通过
   npm run build || cargo build
   
   # 运行测试
   npm test || cargo test
   ```

6. **切换回 main 分支准备最终合并**
   ```bash
   git checkout main
   ```

**关键优势**:
- ✅ **减少主分支冲突**: 在 agent 分支上先解决与 main 的冲突
- ✅ **保证线性历史**: main 分支的合并记录更清晰
- ✅ **降低回滚风险**: 如果同步失败，不影响其他 Agent
- ✅ **符合 Git Flow**: 标准的特性分支工作流

##### 3.4.7.4 执行合并操作

获取锁并完成同步后，执行实际的 Git 合并（agent → main）：

**步骤**:
1. **切换到 main 分支**
   ```bash
   cd project-root
   git checkout main
   git pull origin main  # 确保最新
   ```

2. **合并 Agent 分支**
   ```bash
   git merge agent-pool/agent-001 --no-ff -m "chore: merge US-001 from agent-001"
   ```

3. **冲突处理（理论上不应该有冲突，因为已提前同步）**
   ```rust
   if has_merge_conflicts().await? {
       warn!("Unexpected conflict after sync, resolving...");
       // AI 辅助解决冲突
       let resolved = ai_resolve_conflict(&ours, &theirs, &context).await?;
       apply_resolution(&resolved).await?;
       git add . && git commit -m "chore: resolve unexpected merge conflicts for US-001";
   }
   ```

4. **推送到远程**
   ```bash
   git push origin main
   ```

5. **重置 Agent 分支**
   ```bash
   cd worktree/agent-001
   git fetch origin
   git reset --hard origin/main
   ```

6. **标记任务完成**
   ```rust
   mark_story_as_completed(&task.story_id, agent_id).await?;
   ```

7. **回到 idle 状态**
   ```rust
   self.status = AgentStatus::Idle;
   self.current_task = None;
   ```

##### 3.4.7.5 锁超时与自动释放

防止 Agent 崩溃导致锁永久占用：

```rust
async fn reclaim_expired_merge_locks() -> Result<usize, String> {
    let now = Utc::now();
    
    // 找回过期的锁并释放
    let result = db.execute(
        "UPDATE merge_lock 
         SET locked_by = NULL,
             locked_at = NULL,
             expires_at = NULL
         WHERE expires_at IS NOT NULL 
           AND expires_at < ?",
        &[&now],
    ).await?;
    
    let reclaimed_count = result.rows_affected();
    
    if reclaimed_count > 0 {
        warn!("Reclaimed {} expired merge locks", reclaimed_count);
    }
    
    Ok(reclaimed_count as usize)
}
```

**定时清理任务**:
```rust
// 每 2 分钟检查一次过期的合并锁
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(120));
    loop {
        interval.tick().await;
        reclaim_expired_merge_locks().await.ok();
    }
});
```

**配置参数**:
```yaml
merge_lock:
  timeout_minutes: 10          # 锁超时时间（合并操作通常较快）
  max_retries: 5               # 最大重试次数
  retry_interval_seconds: 2    # 重试间隔
  reclaim_interval_seconds: 120 # 每 2 分钟回收过期锁
```

##### 3.4.7.6 并发控制流程图

```
Agent-001                    Database                  Agent-002
    |                           |                          |
    |-- acquire_merge_lock ---->|                          |
    |                           |-- INSERT OR REPLACE ---->|
    |                           |   WHERE NOT EXISTS       |
    |<-- success (rows=1) ------|                          |
    |                           |                          |
    |-- sync main→agent ------> |                          |
    |   (resolve conflicts)     |                          |
    |                           |                          |
    |                           |<--- acquire_merge_lock --|
    |                           |--- check expires_at ---->|
    |                           |--- lock exists! -------->|
    |                           |                          |
    |                           |<-- failed (rows=0) ------|
    |                           |                          |
    |                           |                          |-- wait 2s ---|
    |                           |                          |              |
    |-- merge agent→main ------>|                          |              |
    |-- release_lock ---------->|                          |              |
    |                           |-- UPDATE SET NULL ------>|              |
    |                           |                          |              |
    |                           |                          |-- retry ---->|
    |                           |<--- acquire_merge_lock --|              |
    |                           |--- lock available! ----->|              |
    |                           |<-- success (rows=1) -----|              |
    |                           |                          |-- sync main→agent ->
```

**关键保证**:
- ✅ **互斥性**: 同一时间只有一个 Agent 能持有合并锁
- ✅ **原子性**: 使用 `INSERT OR REPLACE ... WHERE NOT EXISTS` 确保原子操作
- ✅ **容错性**: 锁超时自动释放，防止死锁
- ✅ **公平性**: 先请求的 Agent 优先获得锁（基于数据库事务顺序）
- ✅ **冲突最小化**: 先在 agent 分支同步 main，再合并到 main

### 3.5 阶段 5: 自主迭代执行

#### 3.5.1 循环逻辑

``rust
async fn sprint_execution_loop(sprint_id: &str) {
    loop {
        // 1. 检查是否还有未执行的任务
        let remaining_tasks = task_queue.get_pending_tasks().await;
        
        if remaining_tasks.is_empty() {
            info!("All tasks completed for Sprint {}", sprint_id);
            break;
        }
        
        // 2. 检查是否有空闲 Agent
        let idle_agents = agent_manager.get_idle_agents().await;
        
        if idle_agents.is_empty() {
            // 所有 Agent 都在忙，等待
            sleep(Duration::from_secs(5)).await;
            continue;
        }
        
        // 3. Agents 会自动拉取任务并执行（见阶段 4）
        // 此处只需监控进度
        
        // 4. 动态调整策略
        adjust_concurrency_based_on_load().await;
    }
    
    // 5. Sprint 完成，生成报告
    generate_sprint_report(sprint_id).await?;
    
    // 6. 清理资源
    cleanup_sprint_resources(sprint_id).await?;
}
```

#### 3.5.2 动态策略调整

根据执行情况实时优化：

```rust
async fn adjust_strategy(execution_metrics: &ExecutionMetrics) {
    // 如果失败率高，降低并发数
    if execution_metrics.failure_rate > 0.3 {
        reduce_concurrency().await;
    }
    
    // 如果执行速度快，提高并发数
    if execution_metrics.average_story_time < EXPECTED_TIME && 
       system_load.low() {
        increase_concurrency().await;
    }
}
```

### 3.6 阶段 6: 完成总结与报告

#### 3.6.1 生成统计报告

```
# Sprint Summary Report

**Sprint ID**: sprint-2024-Q1-01  
**Execution Time**: 3h 25m  
**Completed At**: 2024-04-17 14:30:00 UTC

## Overview
- Total Stories: 18
- Completed: 15 (83%)
- Failed: 3 (17%)
- Total Story Points: 45/50

## Performance Metrics
- Average Time per Story: 13.7min
- Fastest Story: US-003 (8min)
- Slowest Story: US-012 (25min)
- Parallel Efficiency: 87%

## Quality Metrics
- Test Coverage: 85%
- Lint Errors Fixed: 23
- Auto-resolved Conflicts: 5
- Manual Intervention Required: 2

## Agent Performance
| Agent ID | Stories Completed | Avg Time | Success Rate |
|----------|------------------|----------|--------------|
| agent-001 | 5 | 12min | 100% |
| agent-002 | 4 | 15min | 75% |
| agent-003 | 3 | 14min | 100% |
| agent-004 | 3 | 11min | 100% |

## Failed Stories
- US-009: Timeout during AI code generation (retried 3 times)
- US-014: Complex merge conflict requires manual resolution
- US-017: Test coverage below threshold (65% < 80%)

## Recommendations
1. Review failed stories and consider manual implementation
2. Optimize AI prompts for US-009 type tasks
3. Increase test coverage requirements for future sprints
```
#### 3.6.3 通知用户（可选）

仅在以下情况通知用户：
- Sprint 完全完成
- 有失败的故事需要人工介入
- 执行时间超出预期（>2x 预估时间）

**通知渠道**:
- 桌面通知
- 邮件
- Slack/Teams webhook

---

## 5. 异常处理与容错

### 5.1 异常分类与处理策略

  worktreePath: string;          // "/path/to/worktree/agent-001"
  branchName: string;            // "agent-pool/agent-001"
  status: AgentStatus;           // idle | running | paused | failed
  currentTask?: string;          // 当前处理的故事 ID
  completedTasks: string[];      // 已完成的故事 ID 列表
  createdAt: string;
  lastHeartbeat: string;
  resourceUsage: {
    cpuPercent: number;
    memoryMB: number;
  };
  metrics: {
    totalStoriesCompleted: number;
    averageStoryTime: number;
    successRate: number;
  };
}
```

#### 4.1.2 任务队列项

```typescript
interface TaskQueueItem {
  storyId: string;
  storyNumber: string;           // "US-001"
  title: string;
  priority: number;              // 0-100
  storyPoints: number;
  dependencies: string[];
  status: 'pending' | 'locked' | 'completed' | 'failed';
  lockedBy?: string;             // Agent ID
  lockedAt?: string;             // ISO timestamp
  retryCount: number;
  maxRetries: number;            // 默认 3
  assignedAgent?: string;
  startedAt?: string;
  completedAt?: string;
  errorMessage?: string;
}
```

#### 4.1.3 Worktree Pool

```typescript
interface WorktreePool {
  agentWorktrees: Map<string, WorktreeInfo>;
  maxConcurrency: number;
  currentConcurrency: number;
  autoScale: boolean;
  scalingHistory: ScalingEvent[];
}

interface WorktreeInfo {
  agentId: string;
  path: string;
  branchName: string;
  status: 'active' | 'initializing' | 'error';
  createdAt: string;
  lastUsedAt: string;
}
```

### 4.2 Git 操作封装

#### 4.2.1 Git Command Executor

-- 用户故事表（增强版）
CREATE TABLE user_stories (
    id TEXT PRIMARY KEY,
    story_number TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    acceptance_criteria TEXT,
    priority INTEGER NOT NULL DEFAULT 50,
    story_points INTEGER,
    sprint_id TEXT REFERENCES sprints(id),
    status TEXT NOT NULL DEFAULT 'pending',  -- pending | in_progress | completed | failed
    assigned_agent TEXT,                      -- 当前执行的 Agent ID
    locked_at TIMESTAMP,                      -- 锁定时间戳（用于超时检测）
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    failed_at TIMESTAMP,
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    dependencies TEXT,                        -- JSON 数组: ["US-001", "US-002"]
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引优化
CREATE INDEX idx_stories_sprint_status ON user_stories(sprint_id, status);
CREATE INDEX idx_stories_priority ON user_stories(priority DESC);
CREATE INDEX idx_stories_agent ON user_stories(assigned_agent);
CREATE INDEX idx_stories_locked_at ON user_stories(locked_at);

-- Agent 实例表
CREATE TABLE agent_instances (
    id TEXT PRIMARY KEY,
    worktree_path TEXT NOT NULL,
    branch_name TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_heartbeat TIMESTAMP
);

-- 合并锁表（新增）
CREATE TABLE merge_lock (
    id TEXT PRIMARY KEY DEFAULT 'global_merge_lock',
    locked_by TEXT,                    -- 当前持有锁的 Agent ID
    locked_at TIMESTAMP,               -- 锁定时间
    expires_at TIMESTAMP,              -- 过期时间（自动释放）
    UNIQUE(id)
);

-- 初始化合并锁记录
INSERT OR IGNORE INTO merge_lock (id, locked_by, locked_at, expires_at)
VALUES ('global_merge_lock', NULL, NULL, NULL);

-- 提交记录表
CREATE TABLE commit_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    commit_message TEXT,
    committed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (story_id) REFERENCES user_stories(id),
    FOREIGN KEY (agent_id) REFERENCES agent_instances(id)
);

-- Sprint 执行历史表
CREATE TABLE sprint_executions (
    sprint_id TEXT PRIMARY KEY,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    status TEXT NOT NULL,
    total_stories INTEGER,
    completed_stories INTEGER,
    total_story_points INTEGER,
    completed_story_points INTEGER,
    report_url TEXT
);
### 5.1 异常分类与处理策略

| 异常类型 | 检测方式 | 处理策略 | 影响范围 |
|---------|---------|---------|---------|
| **Agent 崩溃** | 心跳超时（>5分钟） | 1. 保存现场（git stash）<br>2. 重启 Agent<br>3. 恢复现场（git stash pop）<br>4. 继续执行 | 单个故事 |
| **Worktree 损坏** | Git 命令失败 | 1. 删除损坏的 worktree<br>2. 重新创建<br>3. 启动新 Agent 实例<br>4. 任务重回队列 | 单个 Agent |
| **任务执行失败** | 超过最大重试次数 | 1. 标记故事为 failed<br>2. 记录错误日志<br>3. 任务不再重试<br>4. 加入"需人工审查"列表 | 单个故事 |
| **AI Provider 故障** | API 调用失败 | 1. 切换到备用 Provider<br>2. 如无备用，重试（指数退避）<br>3. 仍失败则标记任务失败 | 当前任务 |
| **合并锁竞争失败** | acquire_merge_lock 返回 false | 1. 等待重试间隔（2秒）<br>2. 最多重试 5 次<br>3. 仍失败则标记故事需人工介入 | 单个故事 |
| **合并锁超时** | expires_at < now() | 1. 定时任务自动回收（每 2 分钟）<br>2. 其他 Agent 可重新竞争 | 全局 |
| **合并冲突** | Git merge 失败 | 1. AI 辅助解决<br>2. 保留双版本 + TODO<br>3. 继续合并不阻塞 | 单个故事 |
| **编译失败** | 合并后构建失败 | 1. 自动回滚（git merge --abort）<br>2. 释放合并锁<br>3. 标记故事需人工介入<br>4. 通知用户 | 单个故事 |
| **资源耗尽** | 磁盘/CPU/内存不足 | 1. 降低并发数<br>2. 暂停新任务分配<br>3. 等待资源释放<br>4. 通知用户 | 全局 |

### 5.2 重试机制

#### 5.2.1 指数退避策略

```rust
async fn execute_with_retry<F, Fut, T>(
    operation: F,
    max_retries: u32,
    base_delay: Duration,
) -> Result<T, String>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, String>>,
{
    let mut last_error = String::new();
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = e;
                
                if attempt < max_retries {
                    // 指数退避: base_delay * 2^attempt
                    let delay = base_delay * (2_u32.pow(attempt));
                    warn!("Attempt {} failed: {}. Retrying in {:?}...", 
                          attempt + 1, last_error, delay);
                    sleep(delay).await;
                }
            }
        }
    }
    
    Err(format!("Operation failed after {} retries: {}", max_retries, last_error))
}
```

**使用示例**:
```rust
// AI 代码生成，最多重试 3 次，初始延迟 1 秒
let code = execute_with_retry(
    || ai_provider.generate_code(&prompt),
    3,
    Duration::from_secs(1),
).await?;
```

### 5.3 故障转移

#### 5.3.1 Agent 故障转移

```rust
async fn handle_agent_failure(agent_id: &str) -> Result<(), String> {
    info!("Handling failure for Agent {}", agent_id);
    
    // 1. 获取 Agent 当前任务
    let agent = agent_manager.get_agent(agent_id).await?;
    
    if let Some(current_task) = agent.current_task {
        warn!("Agent {} was working on {}, releasing lock", agent_id, current_task);
        
        // 2. 释放任务锁
        task_queue.release_lock(&current_task).await?;
        
        // 3. 保存现场（如果有未提交的变更）
        let worktree_path = &agent.worktree_path;
        if has_uncommitted_changes(worktree_path).await? {
            info!("Stashing uncommitted changes for Agent {}", agent_id);
            execute_git_command(worktree_path, &["stash", "save", "Auto-stash before restart"]).await?;
        }
    }
    
    // 4. 重启 Agent
    info!("Restarting Agent {}", agent_id);
    agent_manager.restart_agent(agent_id).await?;
    
    // 5. 恢复现场
    if let Some(current_task) = agent.current_task {
        execute_git_command(&agent.worktree_path, &["stash", "pop"]).await.ok();
        
        // 6. 重新分配同一任务（可选）
        // task_queue.reassign_task(&current_task, agent_id).await?;
    }
    
    Ok(())
}
```

### 5.4 数据一致性保证

#### 5.4.1 事务性操作

```rust
async fn complete_story_transaction(
    story_id: &str,
    agent_id: &str,
    commit_hash: &str,
) -> Result<(), String> {
    // 使用数据库事务确保原子性
    let tx = db.begin_transaction().await?;
    
    // 1. 更新任务状态
    tx.execute(
        "UPDATE task_queue SET status = 'completed', completed_at = ?, assigned_agent = ? WHERE story_id = ?",
        &[&Utc::now(), &agent_id, &story_id],
    ).await?;
    
    // 2. 记录提交
    tx.execute(
        "INSERT INTO commit_records (story_id, agent_id, commit_hash) VALUES (?, ?, ?)",
        &[&story_id, &agent_id, &commit_hash],
    ).await?;
    
    // 3. 更新 Agent 统计
    tx.execute(
        "UPDATE agent_instances SET completed_tasks = completed_tasks + 1 WHERE id = ?",
        &[&agent_id],
    ).await?;
    
    // 4. 提交事务
    tx.commit().await?;
    
    Ok(())
}
```

---

## 6. 监控与可观测性

### 6.1 实时监控面板

#### 6.1.1 办公室拟人化动画

**映射关系**:
```
Agent-001 → 绿色角色在工作区A
  - 气泡显示: "Working on US-001"
  - 进度条: ████████░░ 80%
  
Agent-002 → 绿色角色在工作区B
  - 气泡显示: "Testing US-002"
  - 进度条: █████████░ 90%
  
Agent-003 → 休息区沙发
  - 状态: idle, waiting for task
  
Agent-004 → 调试区服务器旁
  - 状态: failed, error icon
  - 错误: "AI timeout"
```

**状态动画**:
- `running`: 打字动画 (`running-type`)
- `idle`: 呼吸动画 (`idle-breathe`)
- `testing`: 思考动画 (`paused-think`)
- `failed`: 困惑动画 (`failed-confused`)
- `completed`: 庆祝动画 (`completed-celebrate`)

#### 6.1.2 实时统计数据

```typescript
interface RealtimeMetrics {
  activeAgents: number;
  idleAgents: number;
  failedAgents: number;
  tasksInQueue: number;
  tasksCompleted: number;
  tasksFailed: number;
  averageStoryTime: number;
  currentThroughput: number; // stories/hour
  systemLoad: {
    cpuPercent: number;
    memoryPercent: number;
    diskUsagePercent: number;
  };
}
```

### 6.2 详细日志系统

#### 6.2.1 日志分级

```rust
#[derive(Debug, Clone, PartialEq)]
enum LogLevel {
    TRACE,   // 详细的调试信息
    DEBUG,   // 开发调试信息
    INFO,    // 一般信息（默认级别）
    WARN,    // 警告信息
    ERROR,   // 错误信息
}
```

#### 6.2.2 结构化日志

```
{
  "timestamp": "2024-04-17T10:30:00Z",
  "level": "INFO",
  "agent_id": "agent-001",
  "story_id": "US-001",
  "phase": "coding",
  "message": "Generated 5 files for user login feature",
  "metadata": {
    "files_generated": [
      "src/auth/LoginForm.tsx",
      "src/auth/useAuth.ts",
      "src/auth/AuthContext.tsx",
      "tests/auth/LoginForm.test.tsx",
      "tests/auth/useAuth.test.ts"
    ],
    "tokens_used": 2450,
    "duration_ms": 15230
  }
}
```

#### 6.2.3 日志存储与查询

- **本地存储**: SQLite + 文件日志
- **日志轮转**: 每 100MB 或每天轮转
- **查询接口**: 
  ```rust
  GET /api/logs?agent_id=agent-001&story_id=US-001&level=ERROR
  ```

### 6.3 告警机制

#### 6.3.1 告警规则

| 告警类型 | 触发条件 | 通知渠道 | 严重程度 |
|---------|---------|---------|---------|
| **连续失败** | 同一 Agent 失败 >3 次 | 桌面通知 + 邮件 | High |
| **执行超时** | 单个故事执行 >60min | 桌面通知 | Medium |
| **资源告警** | 磁盘使用率 >90% | 桌面通知 + 邮件 | High |
| **队列积压** | 待执行任务 >50 | 桌面通知 | Low |
| **合并失败** | 故事合并失败 | 桌面通知 + 邮件 | High |

#### 6.3.2 告警去重

```rust
async fn send_alert(alert: Alert) -> Result<(), String> {
    // 检查是否在冷却期内
    if is_in_cooldown_period(&alert.type_, &alert.source).await? {
        return Ok(()); // 跳过重复告警
    }
    
    // 发送告警
    match alert.severity {
        AlertSeverity::High => {
            send_desktop_notification(&alert).await?;
            send_email(&alert).await?;
        }
        AlertSeverity::Medium => {
            send_desktop_notification(&alert).await?;
        }
        AlertSeverity::Low => {
            log_warning(&alert.message).await;
        }
    }
    
    // 记录告警时间
    record_alert_timestamp(&alert.type_, &alert.source).await?;
    
    Ok(())
}
```

### 6.4 历史追溯

#### 6.4.1 Sprint 执行历史

```typescript
interface SprintExecutionHistory {
  sprintId: string;
  startedAt: string;
  completedAt: string;
  duration: number; // minutes
  totalStories: number;
  completedStories: number;
  failedStories: number;
  totalStoryPoints: number;
  completedStoryPoints: number;
  agentsUsed: string[];
  averageStoryTime: number;
  successRate: number;
  reportUrl: string;
}
```

#### 6.4.2 故事执行详情

```typescript
interface StoryExecutionDetail {
  storyId: string;
  storyNumber: string;
  title: string;
  assignedAgent: string;
  startedAt: string;
  completedAt: string;
  duration: number; // minutes
  status: 'completed' | 'failed';
  commitHash: string;
  filesChanged: string[];
  testsPassed: number;
  testsFailed: number;
  lintErrorsFixed: number;
  errorMessage?: string;
  retryCount: number;
}
```

---

## 7. 性能优化策略

### 7.1 并发控制优化

#### 7.1.1 动态并发调整

```rust
async fn adjust_concurrency(current_metrics: &SystemMetrics) -> usize {
    let cpu_usage = current_metrics.cpu_percent;
    let memory_usage = current_metrics.memory_percent;
    let current_concurrency = agent_manager.get_active_count().await;
    
    // 高负载时降低并发
    if cpu_usage > 80.0 || memory_usage > 85.0 {
        return (current_concurrency - 1).max(1);
    }
    
    // 低负载时提高并发
    if cpu_usage < 50.0 && memory_usage < 60.0 {
        let max_allowed = calculate_max_concurrency();
        return (current_concurrency + 1).min(max_allowed);
    }
    
    // 保持当前并发
    current_concurrency
}
```

#### 7.1.2 任务预取

```rust
// Agent 在完成当前任务前 10% 时预取下一个任务
if progress > 90.0 && next_task.is_none() {
    next_task = task_queue.peek_next_available_task().await;
}
```

### 7.2 缓存优化

#### 7.2.1 AI 响应缓存

```rust
// 缓存相似的 AI 请求结果
struct AICache {
    cache: LruCache<String, CodeGenerationResult>,
}

impl AICache {
    fn get_or_generate(&mut self, prompt_hash: &str, generator: impl FnOnce() -> CodeGenerationResult) -> CodeGenerationResult {
        if let Some(result) = self.cache.get(prompt_hash) {
            return result.clone();
        }
        
        let result = generator();
        self.cache.put(prompt_hash.to_string(), result.clone());
        result
    }
}
```

#### 7.2.2 依赖安装缓存

```
# 共享 node_modules 缓存
ln -s /shared-cache/node_modules worktree/agent-001/node_modules
ln -s /shared-cache/node_modules worktree/agent-002/node_modules
```

### 7.3 I/O 优化

#### 7.3.1 异步文件操作

```rust
// 并行写入多个文件
let write_futures: Vec<_> = file_changes
    .iter()
    .map(|change| {
        let path = change.path.clone();
        let content = change.content.clone();
        tokio::spawn(async move {
            tokio::fs::write(&path, &content).await
        })
    })
    .collect();

// 等待所有写入完成
let results = futures::future::join_all(write_futures).await;
```

#### 7.3.2 Git 操作批量化

```rust
// 批量添加文件，减少 git add 调用次数
execute_git_command(&worktree_path, &["add", "-A"]).await?;
// 而非逐个 git add file1 file2 ...
```

### 7.4 网络优化

#### 7.4.1 AI Provider 连接池

```rust
struct AIConnectionPool {
    pool: r2d2::Pool<AIConnectionManager>,
}

impl AIConnectionPool {
    async fn get_connection(&self) -> Result<AIConnection, String> {
        self.pool.get().await.map_err(|e| e.to_string())
    }
}
```

#### 7.4.2 流式响应处理

```rust
// 流式接收 AI 响应，边接收边写入文件
let mut stream = ai_provider.generate_code_stream(&prompt).await?;
let mut file = File::create(&file_path).await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk).await?;
    file.flush().await?;
}
```

---

## 8. 安全与权限控制

### 8.1 沙箱执行

#### 8.1.1 文件系统限制

```rust
// Agent 只能访问其专属 worktree
struct FileSystemGuard {
    allowed_root: PathBuf,
}

impl FileSystemGuard {
    fn validate_path(&self, path: &Path) -> Result<(), String> {
        if !path.starts_with(&self.allowed_root) {
            return Err(format!("Access denied: {} is outside allowed root {}", 
                              path.display(), self.allowed_root.display()));
        }
        Ok(())
    }
}
```

#### 8.1.2 禁止危险操作

```rust
// 黑名单：禁止执行的命令
const DANGEROUS_COMMANDS: &[&str] = &[
    "rm -rf /",
    "sudo",
    "chmod 777",
    "dd if=/dev/zero",
    // ...
];

fn is_command_safe(command: &str) -> bool {
    !DANGEROUS_COMMANDS.iter().any(|dangerous| command.contains(dangerous))
}
```

### 8.2 权限控制

#### 8.2.1 Agent 权限矩阵

| 操作 | 允许 | 说明 |
|------|------|------|
| 读取项目文件 | ✅ | 仅限 worktree 内 |
| 写入项目文件 | ✅ | 仅限 worktree 内 |
| 执行 Git 命令 | ✅ | 仅限当前 worktree |
| 调用 AI API | ✅ | 使用配置的 API Key |
| 运行测试 | ✅ | npm test, cargo test 等 |
| 修改配置文件 | ❌ | 禁止修改 `.env`, `config.json` 等 |
| 访问网络 | ⚠️ | 仅限 AI Provider 域名 |
| 执行 Shell 脚本 | ❌ | 禁止任意命令执行 |

#### 8.2.2 API Key 安全管理

```rust
// 从操作系统密钥链读取 API Key，不硬编码
async fn get_api_key(provider: &str) -> Result<String, String> {
    let keyring = keyring::Entry::new("opc-harness", &format!("{}_api_key", provider));
    keyring.get_password().map_err(|e| e.to_string())
}
```

### 8.3 审计日志

#### 8.3.1 操作审计

```rust
struct AuditLog {
    timestamp: DateTime<Utc>,
    agent_id: String,
    action: String,
    resource: String,
    result: String,
    ip_address: Option<String>,
}

// 记录所有敏感操作
async fn log_audit(agent_id: &str, action: &str, resource: &str, result: &str) {
    let log = AuditLog {
        timestamp: Utc::now(),
        agent_id: agent_id.to_string(),
        action: action.to_string(),
        resource: resource.to_string(),
        result: result.to_string(),
        ip_address: None,
    };
    
    db.insert_audit_log(log).await.ok();
}
```

---

## 9. 附录

### 9.1 Git 分支管理规范

#### 9.1.1 分支命名规范

```
main                          # 主分支
├─ agent-pool/agent-001       # Agent-001 的持久化分支
├─ agent-pool/agent-002       # Agent-002 的持久化分支
├─ agent-pool/agent-003       # Agent-003 的持久化分支
└─ agent-pool/agent-004       # Agent-004 的持久化分支
```

**注意**: Agent 分支始终保持与 main 同步，每个故事完成后立即合并并重置。

#### 9.1.2 Commit Message 规范

```
<type>(us-<ID>): <subject>

<body>

AI Agent: <agent-id> | Story Points: <points> | Sprint: <sprint-id>
```

**Type 枚举**:
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具变动

**示例**:
```
feat(us-001): implement user login feature

- Add JWT authentication service
- Create login form with validation
- Implement session management
- Add unit tests for auth flows

AI Agent: agent-001 | Story Points: 5 | Sprint: sprint-2024-Q1-01
```

#### 9.1.3 合并策略

```
# 每个故事完成后立即合并
cd project-root
git checkout main
git pull origin main
git merge agent-pool/agent-001 --no-ff -m "chore: merge US-001 from agent-001"
git push origin main

# 重置 Agent 分支
cd worktree/agent-001
git fetch origin
git reset --hard origin/main
```

### 9.2 配置文件示例

#### 9.2.1 Agent Pool 配置

```yaml
# config/agent-pool.yaml
agent_pool:
  max_concurrency: 8
  min_concurrency: 2
  auto_scale: true
  
  resource_thresholds:
    cpu_percent: 80
    memory_percent: 85
    disk_gb: 10
  
  worktree:
    base_path: "./worktree"
    branch_prefix: "agent-pool"
    auto_cleanup: true
    retention_days: 7
  
  database_task_coordinator:
    lock_timeout_minutes: 30
    max_retries: 3
    retry_backoff_base_seconds: 1
    reclaim_interval_seconds: 300  # 每 5 分钟回收超时任务
  
  merge_lock:
    timeout_minutes: 10            # 合并锁超时时间
    max_retries: 5                 # 最大重试次数
    retry_interval_seconds: 2      # 重试间隔
    reclaim_interval_seconds: 120  # 每 2 分钟回收过期锁
  
  merge:
    strategy: "instant"  # 即时合并，每个故事完成后立即合并
    conflict_resolution: "ai-assisted"  # AI 辅助解决冲突
```

#### 9.2.2 AI Provider 配置

```
# config/ai-providers.yaml
ai_providers:
  primary:
    type: openai
    model: gpt-4-turbo
    api_key_env: OPENAI_API_KEY
    temperature: 0.7
    max_tokens: 4096
    timeout_seconds: 60
  
  fallback:
    type: claude
    model: claude-3-opus
    api_key_env: ANTHROPIC_API_KEY
    temperature: 0.7
    max_tokens: 4096
    timeout_seconds: 60
```

### 9.3 常见问题（FAQ）

#### Q1: 为什么采用即时合并而非批次合并？

**A**: 
- **降低复杂度**: 避免批次管理的复杂性
- **减少冲突**: 逐个故事合并，冲突更易解决
- **快速反馈**: 每个故事立即可见，便于及时发现问题
- **简化回滚**: 单个故事回滚比批次回滚更简单

#### Q2: 为什么移除 Story Selector？

**A**:
- **时间驱动**: Sprint 由起止时间自动定义，无需复杂的选择逻辑
- **简化流程**: 直接加载当前 Sprint 的所有 pending 故事，按优先级排序
- **减少决策开销**: 避免复杂的评分和依赖分析
- **更透明**: 用户可以清楚看到哪些故事会被执行

#### Q3: 如何保证多个 Agent 不会选择同一个故事？

**A**:
- **数据库乐观锁**: 使用 `UPDATE ... WHERE` + `FOR UPDATE SKIP LOCKED` 原子操作
- **唯一性保证**: 只有第一个执行成功的 Agent 能更新成功（影响行数 = 1）
- **无竞争窗口**: 数据库事务确保不会出现两个 Agent 同时锁定同一故事的情况
- **分布式友好**: 即使 Agent 部署在不同机器上，也能通过数据库保证一致性

**示例 SQL**:
```sql
UPDATE user_stories 
SET status = 'in_progress',
    assigned_agent = 'agent-001',
    locked_at = NOW()
WHERE id = (
    SELECT id FROM user_stories 
    WHERE status = 'pending'
    ORDER BY priority DESC
    LIMIT 1
    FOR UPDATE SKIP LOCKED
);
```

#### Q4: 如何处理 Agent 间的代码冲突？

**A**:
- **预防**: 通过故事的依赖字段，确保有依赖的故事按顺序执行
- **检测**: 合并时自动检测冲突
- **解决**: 
  - 简单冲突：AI 辅助自动解决
  - 复杂冲突：保留双版本 + TODO 标记，不阻塞流程
  - 严重冲突：自动回滚，通知用户人工介入

#### Q5: Agent 崩溃后如何恢复？

**A**:
1. **锁超时检测**: 定时任务检查 `locked_at` 超过阈值的故事
2. **自动回收**: 将超时任务状态重置为 `pending`，`retry_count` 递增
3. **重新分配**: 其他 Agent 会自动竞争并领取该任务
4. **现场恢复**: 如果 Agent 重启，可尝试 `git stash pop` 恢复未提交的变更

#### Q6: 如何保证代码质量？

**A**:
- **自动化测试**: 每个故事必须通过单元测试
- **Lint 检查**: 强制执行 ESLint/Prettier
- **TypeScript 类型检查**: 确保类型安全
- **质量门禁**: 测试覆盖率 ≥80%，无 lint 错误
- **自动修复**: AI 自动修复常见问题

#### Q7: 是否可以手动干预执行过程？

**A**:
- **紧急停止**: 提供"紧急停止"按钮，立即终止所有 Agent
- **暂停/恢复**: 支持暂停整个 Sprint 执行，稍后恢复
- **人工审查**: 失败的故事可查看日志，手动修复后重新入队
- **配置调整**: 运行时可调整并发数、合并策略等参数

#### Q8: 如果没有活跃的 Sprint 会怎样？

**A**:
- 系统会记录警告日志："No active sprint found for current time"
- Agent Pool 不会启动
- 用户可以通过管理界面创建新的 Sprint 或调整现有 Sprint 的时间范围

#### Q9: 数据库驱动相比内存队列有什么优势？

**A**:
- **无单点故障**: 不依赖中央 Task Queue 服务
- **天然分布式**: 支持跨机器部署多个 Agent 实例
- **持久化状态**: 所有任务状态存储在数据库，重启不丢失
- **简化架构**: 移除内存队列的同步和序列化复杂度
- **审计追溯**: 完整的数据库历史记录，便于问题排查

#### Q10: 如何确保多个 Agent 不会同时合并代码？

**A**:
- **分布式合并锁**: 使用 `merge_lock` 表和原子 SQL 操作
- **互斥保证**: `INSERT OR REPLACE ... WHERE NOT EXISTS` 确保只有一个 Agent 能获取锁
- **自动释放**: 使用 RAII 模式的 `MergeLockGuard`，函数退出时自动释放锁
- **超时回收**: 每 2 分钟定时任务检查并释放过期的锁
- **重试机制**: 获取锁失败时等待 2 秒后重试，最多 5 次

**示例流程**:
```rust
// 1. 竞争获取锁
if acquire_merge_lock(agent_id).await? {
    // 2. 同步 main → agent（解决冲突）
    sync_agent_branch_with_main(...).await?;
    // 3. 合并 agent → main
    execute_merge_operation(...).await?;
    // 4. 自动释放锁（Drop trait）
}
```

#### Q11: 为什么采用"先同步 main→agent，再合并 agent→main"的两步策略？

**A**:
- **减少主分支冲突**: 在 agent 分支上先解决与 main 的冲突，避免影响其他 Agent
- **保证线性历史**: main 分支的合并记录更清晰，易于追溯
- **降低回滚风险**: 如果同步失败，只影响当前 Agent，不阻塞全局
- **符合 Git Flow**: 标准的特性分支工作流，业界最佳实践
- **提高成功率**: 提前解决冲突，最终合并到 main 时几乎不会有冲突

**对比单步合并**:
```
❌ 单步合并 (agent → main):
   - 直接在 main 上解决冲突
   - 可能影响其他正在工作的 Agent
   - 回滚复杂度高

✅ 两步合并 (main → agent, then agent → main):
   - 在 agent 分支上隔离解决冲突
   - 不影响其他 Agent
   - 最终合并几乎无冲突
```

#### Q12: 如果 Agent 在合并过程中崩溃会怎样？

**A**:
1. **锁不会永久占用**: `expires_at` 字段设置超时时间（默认 10 分钟）
2. **自动回收**: 定时任务每 2 分钟检查一次，释放过期的锁
3. **其他 Agent 可继续**: 锁释放后，其他 Agent 可以重新竞争并执行合并
4. **数据一致性**: Git 操作的原子性确保不会出现半完成的合并

### 9.4 参考资料

- [Git Worktree 官方文档](https://git-scm.com/docs/git-worktree)
- [Tauri v2 Documentation](https://v2.tauri.app/)
- [Harness Engineering 理念](./AGENTS.md)
- [OPC-HARNESS 项目架构](./README.md)

### 9.5 版本历史

| 版本 | 日期 | 变更说明 |
|------|------|---------|
| v1.0 | 2024-04-17 | 初始版本，定义完整的智能体并行执行流程 |
| v2.0 | 2024-04-17 | 简化版本，移除批次合并，采用即时合并机制 |
| v3.0 | 2024-04-17 | 极简版本，移除 Sprint Trigger 和 Story Selector，改为基于时间的自动加载 |
| v3.1 | 2024-04-17 | 引入数据库驱动的乐观锁任务竞争机制，替代内存队列 |
| v3.2 | 2024-04-17 | 添加分布式合并锁机制，确保同一时间只有一个 Agent 能执行合并 |
| v3.3 | 2024-04-17 | 实现两步合并策略：先同步 main→agent，再合并 agent→main |

---

**文档维护者**: OPC-HARNESS Team  
**反馈渠道**: [GitHub Issues](https://github.com/chensheng/opc-harness/issues)  
**许可证**: MIT
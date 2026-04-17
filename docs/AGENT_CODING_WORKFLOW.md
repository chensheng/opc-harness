# Vibe Coding 智能体并行执行流程设计文档

> **版本**: v1.0  
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

本方案旨在实现**完全自动化的多智能体并行开发流程**，通过 Git Worktree 隔离和 Agent Pool 架构，让多个 AI 智能体能够同时、独立地执行用户故事（User Story），最终自动合并到主分支，实现"一人公司"的愿景。

**核心价值**：
- ✅ **零人工干预**: 从故事选择到代码合并全流程自动化
- ✅ **高效并行**: 基于系统负载动态调整并发数，最大化资源利用
- ✅ **安全可靠**: Worktree 隔离 + 自动测试 + 失败回滚机制
- ✅ **可追溯**: 完整的审计日志和 Git 历史记录

### 1.2 适用范围

本文档适用于 OPC-HARNESS 项目的 **Vibe Coding** 模块，具体场景包括：
- Sprint 级别的批量用户故事执行
- 多智能体并行编码任务调度
- 自动化代码生成、测试、审查和合并

### 1.3 关键术语

| 术语 | 定义 |
|------|------|
| **Agent** | AI 编码智能体实例，负责执行单个用户故事的完整开发生命周期 |
| **Worktree** | Git 工作树，为每个 Agent 提供独立的文件系统环境 |
| **Agent Pool** | 一组持久化的 Agent 实例集合，支持动态任务分配 |
| **Task Queue** | 全局任务队列，存储待执行的用户故事及其状态 |
| **Story Selector** | 智能故事选择器，基于优先级、依赖等因素自主选择故事 |
| **Batch Merge** | 批次合并，将多个 Agent 的变更一次性合并到 main 分支 |

---

## 2. 核心架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    Sprint Trigger                            │
│              (定时/事件/手动触发)                             │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│                 Story Selector (决策层)                      │
│  - 扫描待执行 Sprint                                         │
│  - 应用 smart-grouping 策略                                  │
│  - 计算评分与依赖关系                                        │
│  - 生成执行批次计划                                          │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│              Agent Pool Manager (协调层)                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │ Agent-01 │  │ Agent-02 │  │ Agent-03 │  ...            │
│  │Worktree-A│  │Worktree-B│  │Worktree-C│                 │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                 │
└───────┼──────────────┼──────────────┼──────────────────────┘
        │              │              │
        ▼              ▼              ▼
┌─────────────────────────────────────────────────────────────┐
│              Global Task Queue (任务层)                      │
│  [US-001] → [US-002] → [US-003] → [US-004] → ...          │
│  Pull-based 分发机制                                         │
└─────────────────────────────────────────────────────────────┘
        │              │              │
        ▼              ▼              ▼
┌─────────────────────────────────────────────────────────────┐
│              Autonomous Execution (执行层)                   │
│  每个 Agent 在其专属 Worktree 中:                            │
│  Analyze → Code → Test → Review → Commit                   │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│              Batch Merge & Sync (合并层)                     │
│  - 定期收集所有 Agent 分支                                   │
│  - AI 辅助冲突解决                                           │
│  - 合并到 main 并重置 Agent 分支                             │
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

### 2.2 核心组件

#### 2.2.1 Story Selector（智能故事选择器）

**职责**: 自主分析 Sprint 中的用户故事，生成最优执行计划

**选择策略**（默认 `smart-grouping`）:
- **优先级得分** (30%): P0=100, P1=75, P2=50, P3=25
- **依赖就绪度** (30%): 检查所有依赖是否已完成
- **复杂度适配** (20%): 平衡高低复杂度任务
- **技术相关性** (20%): 匹配 Agent 专长领域

**输出**: 
- 排序后的故事列表
- 执行批次划分（基于依赖拓扑排序）
- 每个故事的推荐理由

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

#### 2.2.3 Global Task Queue（全局任务队列）

**职责**: 维护待执行故事的有序队列，支持并发安全的任务分发

**数据结构**:
```typescript
interface TaskQueueItem {
  storyId: string;
  priority: number;
  dependencies: string[];
  status: 'pending' | 'locked' | 'completed' | 'failed';
  lockedBy?: string;      // 锁定该任务的 Agent ID
  lockedAt?: string;      // 锁定时间戳
  retryCount: number;     // 重试次数
  assignedAgent?: string; // 最终执行的 Agent ID
}
```

**关键机制**:
- **Pull-based 分发**: Agent 主动拉取任务，而非被动推送
- **任务锁定**: 防止多个 Agent 竞争同一故事
- **锁超时**: 默认 30 分钟，超时自动释放
- **优先级队列**: 高优先级故事优先出队

#### 2.2.4 Coding Agent（编码智能体）

**职责**: 在专属 Worktree 中执行用户故事的完整开发生命周期

**执行阶段**:
1. **Analyzing**: 解析 PRD，制定技术方案
2. **Coding**: 调用 AI Provider 生成代码
3. **Testing**: 运行单元测试，自动修复失败
4. **Reviewing**: 执行 Lint/TypeScript 检查
5. **Committing**: 自动生成 Commit Message 并提交

**特性**:
- **完全自主**: 无需用户确认任何操作
- **连续执行**: 在同一 Worktree 中处理多个故事
- **错误恢复**: 自动重试、降级、隔离

---

## 3. 完整执行流程

### 3.1 阶段 1: Sprint 触发与初始化

#### 3.1.1 触发方式

支持三种触发模式：

1. **定时触发**
   - 配置 Cron 表达式（如每天 9:00 AM）
   - 自动扫描状态为 `planning` 的 Sprint
   - 按优先级和创建时间排序

2. **事件触发**
   - PRD 审批通过后自动创建 Sprint
   - 立即启动执行流程

3. **手动触发**
   - 用户点击"开始 Sprint"按钮
   - 后续流程完全自动化

#### 3.1.2 资源预检

在执行前进行系统性检查：

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
├── package.json
└── ...
```

#### 3.2.3 启动 Agent 实例

为每个 Worktree 启动一个 Coding Agent 进程：

```rust
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
- `status`: `idle`
- `currentTask`: `None`
- `completedTasks`: `[]`
- `lastHeartbeat`: `now()`

#### 3.2.4 构建任务队列

从 Story Selector 获取已排序的故事列表，加入全局队列：

```typescript
async function buildTaskQueue(stories: UserStory[]): Promise<TaskQueue> {
  const queue = stories.map(story => ({
    storyId: story.id,
    priority: calculatePriority(story),
    dependencies: story.dependencies || [],
    status: 'pending',
    retryCount: 0,
  }));
  
  // 按优先级降序排序
  queue.sort((a, b) => b.priority - a.priority);
  
  return new TaskQueue(queue);
}
```

### 3.3 阶段 3: 动态任务分发

#### 3.3.1 Pull-based 任务拉取

每个 Agent 独立运行拉取循环：

```rust
async fn agent_task_loop(agent_id: &str) {
    loop {
        // 1. 检查 Agent 状态
        if self.status != AgentStatus::Idle {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        
        // 2. 尝试拉取下一个可用任务
        match task_queue.pull_next_available_task().await {
            Some(task) => {
                // 3. 锁定任务
                task_queue.lock_task(&task.story_id, agent_id).await;
                
                // 4. 执行任务
                self.execute_story(&task).await;
                
                // 5. 释放锁
                task_queue.release_lock(&task.story_id).await;
            }
            None => {
                // 队列为空，短暂休眠
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
```

#### 3.3.2 任务可用性检查

在拉取任务时，检查以下条件：

```rust
fn is_task_available(task: &TaskQueueItem, completed_stories: &HashSet<String>) -> bool {
    // 1. 状态必须是 pending
    if task.status != TaskStatus::Pending {
        return false;
    }
    
    // 2. 检查依赖是否已满足
    let dependencies_met = task.dependencies.iter().all(|dep_id| {
        completed_stories.contains(dep_id)
    });
    
    if !dependencies_met {
        return false;
    }
    
    // 3. 检查是否已被其他 Agent 锁定
    if task.locked_by.is_some() {
        // 检查锁是否超时
        if let Some(locked_at) = task.locked_at {
            let elapsed = Utc::now() - locked_at;
            if elapsed.num_minutes() < LOCK_TIMEOUT_MINUTES {
                return false; // 锁未超时，不可用
            }
            // 锁已超时，可以抢占
        } else {
            return false;
        }
    }
    
    true
}
```

#### 3.3.3 任务锁定机制

防止多个 Agent 竞争同一任务：

```rust
async fn lock_task(&self, story_id: &str, agent_id: &str) -> Result<(), String> {
    let mut queue = self.queue.lock().await;
    
    if let Some(task) = queue.iter_mut().find(|t| t.story_id == story_id) {
        if task.status == TaskStatus::Pending {
            task.status = TaskStatus::Locked;
            task.locked_by = Some(agent_id.to_string());
            task.locked_at = Some(Utc::now());
            Ok(())
        } else {
            Err(format!("Task {} is not available", story_id))
        }
    } else {
        Err(format!("Task {} not found", story_id))
    }
}
```

**锁超时配置**:
- 默认超时时间: **30 分钟**
- 超时后自动释放，任务重回队列
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
│  8. 标记 US-001 完成                       │
│     ↓                                       │
│  9. 返回 idle，拉取 US-004                 │
│     ↓                                       │
│  10. 重复步骤 3-9...                       │
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

6. **更新进度**: 85% → 95%

#### 3.4.6 Committing Phase（提交阶段） ⭐

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

6. **标记任务完成**
   ```rust
   mark_story_as_completed(&task.story_id, agent_id).await?;
   ```

#### 3.4.7 连续执行下一个任务

完成当前故事后，Agent 立即进入下一轮循环：

```rust
// 回到 idle 状态
self.status = AgentStatus::Idle;
self.current_task = None;

// 循环继续，拉取下一个任务
// 可能在同一分支上继续开发 US-004, US-007, ...
```

**Git 历史示例**（Agent-001 分支）:
```
commit-004: feat(us-013): add payment gateway
commit-003: feat(us-010): implement shopping cart
commit-002: feat(us-007): create product catalog
commit-001: feat(us-001): implement user login feature
commit-000: Initial commit from main
```

### 3.5 阶段 5: 批次同步与主分支合并

#### 3.5.1 合并触发条件

支持两种触发模式：

1. **定时触发**（推荐）
   - 每隔 N 分钟执行一次（默认 30 分钟）
   - 或在所有任务完成后立即执行

2. **阈值触发**
   - 当某个 Agent 分支积累了 M 个故事（默认 5 个）
   - 或累计故事点达到阈值（默认 20 点）

#### 3.5.2 合并流程

**步骤 1: 收集所有 Agent 分支**
```bash
cd project-root
git fetch origin agent-pool/agent-001 agent-pool/agent-002 agent-pool/agent-003 agent-pool/agent-004
```

**步骤 2: 冲突检测与预合并**
```rust
async fn detect_merge_conflicts(agent_branches: &[String]) -> Result<ConflictReport, String> {
    let mut conflicts = Vec::new();
    
    // 分析各分支的文件变更交集
    for i in 0..agent_branches.len() {
        for j in (i+1)..agent_branches.len() {
            let changed_files_i = get_changed_files(&agent_branches[i]).await?;
            let changed_files_j = get_changed_files(&agent_branches[j]).await?;
            
            let intersection: HashSet<_> = changed_files_i.intersection(&changed_files_j).collect();
            
            if !intersection.is_empty() {
                conflicts.push(ConflictInfo {
                    branch_a: agent_branches[i].clone(),
                    branch_b: agent_branches[j].clone(),
                    conflicting_files: intersection.into_iter().cloned().collect(),
                });
            }
        }
    }
    
    Ok(ConflictReport { conflicts })
}
```

**步骤 3: 顺序合并到 main**
```bash
git checkout main

# 依次合并每个 Agent 分支
git merge agent-pool/agent-001 --no-ff -m "chore: merge agent-001 batch (5 stories: US-001, US-004, US-007, US-010, US-013)"
git merge agent-pool/agent-002 --no-ff -m "chore: merge agent-002 batch (4 stories: US-002, US-005, US-008, US-011)"
git merge agent-pool/agent-003 --no-ff -m "chore: merge agent-003 batch (6 stories)"
git merge agent-pool/agent-004 --no-ff -m "chore: merge agent-004 batch (3 stories)"
```

**步骤 4: 冲突处理策略**

| 冲突类型 | 处理策略 |
|---------|---------|
| **无冲突** | 自动完成合并 |
| **简单冲突**<br>（空格、换行符） | 自动解决，采用最新变更 |
| **中等冲突**<br>（代码逻辑冲突） | AI 辅助解决：<br>1. 发送冲突片段给 AI<br>2. AI 分析上下文生成合并方案<br>3. 自动应用方案 |
| **复杂冲突**<br>（架构级冲突） | 保留双版本：<br>```javascript<br>// <<< HEAD (agent-001)<br>const x = 1;<br>// ===<br>const x = 2;<br>// >>> agent-002<br>```<br>生成 TODO 任务，不阻塞流程 |
| **合并失败**<br>（编译错误） | 自动回滚：<br>`git merge --abort`<br>标记批次需人工介入 |

**步骤 5: 推送并重置 Agent 分支**
```bash
# 推送到远程
git push origin main

# 重置所有 Agent 分支到最新的 main
git checkout agent-pool/agent-001 && git reset --hard origin/main
git checkout agent-pool/agent-002 && git reset --hard origin/main
git checkout agent-pool/agent-003 && git reset --hard origin/main
git checkout agent-pool/agent-004 && git reset --hard origin/main
```

**步骤 6: 通知 Agents 继续工作**
- Agents 检测到分支已重置
- 继续从任务队列拉取新故事
- 在新基础上继续开发

### 3.6 阶段 6: 自主迭代执行

#### 3.6.1 循环逻辑

```rust
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
        
        // 4. 定期检查是否需要合并
        if should_trigger_merge().await {
            execute_batch_merge().await?;
        }
        
        // 5. 动态调整策略
        adjust_concurrency_based_on_load().await;
    }
    
    // 6. Sprint 完成，执行最终合并
    execute_final_merge().await?;
    
    // 7. 生成报告
    generate_sprint_report(sprint_id).await?;
    
    // 8. 清理资源
    cleanup_sprint_resources(sprint_id).await?;
}
```

#### 3.6.2 动态策略调整

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
    
    // 如果频繁冲突，调整批次大小
    if execution_metrics.conflict_rate > 0.2 {
        reduce_batch_size().await;
    }
}
```

### 3.7 阶段 7: 完成总结与报告

#### 3.7.1 生成统计报告

```markdown
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

#### 3.7.2 更新 Sprint 状态

```rust
async fn complete_sprint(sprint_id: &str) -> Result<(), String> {
    // 1. 更新 Sprint 状态为 completed
    update_sprint_status(sprint_id, SprintStatus::Completed).await?;
    
    // 2. 更新已完成的故事点
    let completed_points = calculate_completed_story_points(sprint_id).await?;
    update_sprint_completed_points(sprint_id, completed_points).await?;
    
    // 3. 记录完成时间
    update_sprint_completed_at(sprint_id, Utc::now()).await?;
    
    // 4. 保存报告链接
    let report_url = save_sprint_report(sprint_id).await?;
    link_report_to_sprint(sprint_id, &report_url).await?;
    
    Ok(())
}
```

#### 3.7.3 通知用户（可选）

仅在以下情况通知用户：
- Sprint 完全完成
- 有失败的故事需要人工介入
- 执行时间超出预期（>2x 预估时间）

**通知渠道**:
- 桌面通知
- 邮件
- Slack/Teams webhook

---

## 4. 技术实现细节

### 4.1 数据结构设计

#### 4.1.1 Agent 实例

```typescript
interface AgentInstance {
  id: string;                    // "agent-001"
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

```rust
pub struct GitExecutor {
    repo_path: PathBuf,
}

impl GitExecutor {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo_path: PathBuf::from(repo_path),
        }
    }
    
    pub async fn execute_command(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(args)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git command: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    
    pub async fn checkout_branch(&self, branch_name: &str) -> Result<(), String> {
        self.execute_command(&["checkout", branch_name]).await?;
        Ok(())
    }
    
    pub async fn commit_changes(&self, message: &str) -> Result<String, String> {
        self.execute_command(&["add", "."]).await?;
        self.execute_command(&["commit", "-m", message]).await?;
        self.get_latest_commit_hash().await
    }
    
    pub async fn merge_branch(&self, branch_name: &str, no_ff: bool) -> Result<(), String> {
        let args = if no_ff {
            vec!["merge", "--no-ff", branch_name]
        } else {
            vec!["merge", branch_name]
        };
        
        self.execute_command(&args).await?;
        Ok(())
    }
    
    pub async fn reset_to_remote(&self, branch_name: &str) -> Result<(), String> {
        self.execute_command(&["reset", "--hard", &format!("origin/{}", branch_name)]).await?;
        Ok(())
    }
}
```

### 4.3 AI Provider 集成

#### 4.3.1 Provider 抽象层

```rust
pub trait AIProvider: Send + Sync {
    async fn generate_code(&self, prompt: &str) -> Result<CodeGenerationResult, String>;
    async fn fix_test_failures(&self, failures: &[TestFailure], code: &str) -> Result<String, String>;
    async fn resolve_merge_conflict(&self, ours: &str, theirs: &str, context: &str) -> Result<String, String>;
}

pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

pub struct ClaudeProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

// 支持多种 Provider，运行时切换
pub enum AIProviderType {
    OpenAI(OpenAIProvider),
    Claude(ClaudeProvider),
    Kimi(KimiProvider),
    // ...
}
```

### 4.4 数据库持久化

#### 4.4.1 SQLite Schema

```sql
-- Agent 实例表
CREATE TABLE agent_instances (
    id TEXT PRIMARY KEY,
    worktree_path TEXT NOT NULL,
    branch_name TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_heartbeat TIMESTAMP
);

-- 任务队列表
CREATE TABLE task_queue (
    story_id TEXT PRIMARY KEY,
    story_number TEXT NOT NULL,
    priority INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    locked_by TEXT REFERENCES agent_instances(id),
    locked_at TIMESTAMP,
    retry_count INTEGER DEFAULT 0,
    assigned_agent TEXT REFERENCES agent_instances(id),
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    error_message TEXT
);

-- 提交记录表
CREATE TABLE commit_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    commit_message TEXT,
    committed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (story_id) REFERENCES task_queue(story_id),
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
```

---

## 5. 异常处理与容错

### 5.1 异常分类与处理策略

| 异常类型 | 检测方式 | 处理策略 | 影响范围 |
|---------|---------|---------|---------|
| **Agent 崩溃** | 心跳超时（>5分钟） | 1. 保存现场（git stash）<br>2. 重启 Agent<br>3. 恢复现场（git stash pop）<br>4. 继续执行 | 单个故事 |
| **Worktree 损坏** | Git 命令失败 | 1. 删除损坏的 worktree<br>2. 重新创建<br>3. 启动新 Agent 实例<br>4. 任务重回队列 | 单个 Agent |
| **任务执行失败** | 超过最大重试次数 | 1. 标记故事为 failed<br>2. 记录错误日志<br>3. 任务不再重试<br>4. 加入"需人工审查"列表 | 单个故事 |
| **AI Provider 故障** | API 调用失败 | 1. 切换到备用 Provider<br>2. 如无备用，重试（指数退避）<br>3. 仍失败则标记任务失败 | 当前任务 |
| **合并冲突** | Git merge 失败 | 1. AI 辅助解决<br>2. 保留双版本 + TODO<br>3. 继续合并不阻塞 | 批次合并 |
| **编译失败** | 合并后构建失败 | 1. 自动回滚（git merge --abort）<br>2. 标记批次需人工介入<br>3. 通知用户 | 整个批次 |
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

```json
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
| **合并失败** | 批次合并失败 | 桌面通知 + 邮件 | High |

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

```bash
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

**注意**: 不再为每个故事创建临时分支，所有故事直接在 Agent 分支上提交。

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

```bash
# 批次合并到 main
git checkout main
git merge agent-pool/agent-001 --no-ff -m "chore: merge agent-001 batch (5 stories)"
git merge agent-pool/agent-002 --no-ff -m "chore: merge agent-002 batch (4 stories)"
git push origin main

# 重置 Agent 分支
git checkout agent-pool/agent-001 && git reset --hard origin/main
git checkout agent-pool/agent-002 && git reset --hard origin/main
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
  
  task_queue:
    lock_timeout_minutes: 30
    max_retries: 3
    retry_backoff_base_seconds: 1
  
  merge:
    strategy: "auto"  # auto | semi-auto | manual
    trigger_interval_minutes: 30
    batch_size_threshold: 5
    story_points_threshold: 20
```

#### 9.2.2 AI Provider 配置

```yaml
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

#### Q1: 为什么选择"一智能体一分支"而非"一故事一支"？

**A**: 
- **简化 Git 操作**: 避免频繁的分支创建/删除
- **清晰的线性历史**: 每个 Agent 的分支形成自然的故事批次
- **高效的合并**: 减少合并次数，降低冲突概率
- **易于追溯**: 直接查看 Agent 分支历史即可了解该 Agent 的所有工作

#### Q2: 如何处理 Agent 间的代码冲突？

**A**:
- **预防**: 通过 Story Selector 的依赖感知，避免分配可能冲突的故事给不同 Agent
- **检测**: 合并前分析文件变更交集
- **解决**: 
  - 简单冲突：AI 辅助自动解决
  - 复杂冲突：保留双版本 + TODO 标记，不阻塞流程
  - 严重冲突：自动回滚，通知用户人工介入

#### Q3: Agent 崩溃后如何恢复？

**A**:
1. 检测到心跳超时
2. 保存现场（`git stash`）
3. 重启 Agent 进程
4. 恢复现场（`git stash pop`）
5. 继续执行当前任务或重新拉取

#### Q4: 如何保证代码质量？

**A**:
- **自动化测试**: 每个故事必须通过单元测试
- **Lint 检查**: 强制执行 ESLint/Prettier
- **TypeScript 类型检查**: 确保类型安全
- **质量门禁**: 测试覆盖率 ≥80%，无 lint 错误
- **自动修复**: AI 自动修复常见问题

#### Q5: 是否可以手动干预执行过程？

**A**:
- **紧急停止**: 提供"紧急停止"按钮，立即终止所有 Agent
- **暂停/恢复**: 支持暂停整个 Sprint 执行，稍后恢复
- **人工审查**: 失败的故事可查看日志，手动修复后重新入队
- **配置调整**: 运行时可调整并发数、合并策略等参数

### 9.4 参考资料

- [Git Worktree 官方文档](https://git-scm.com/docs/git-worktree)
- [Tauri v2 Documentation](https://v2.tauri.app/)
- [Harness Engineering 理念](./AGENTS.md)
- [OPC-HARNESS 项目架构](./README.md)

### 9.5 版本历史

| 版本 | 日期 | 变更说明 |
|------|------|---------|
| v1.0 | 2024-04-17 | 初始版本，定义完整的智能体并行执行流程 |

---

**文档维护者**: OPC-HARNESS Team  
**反馈渠道**: [GitHub Issues](https://github.com/chensheng/opc-harness/issues)  
**许可证**: MIT

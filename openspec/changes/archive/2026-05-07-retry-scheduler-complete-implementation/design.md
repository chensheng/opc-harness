## Context

当前 Vibe Coding 系统已实现完整的重试引擎核心组件（RetryEngine、ErrorClassifier、BackoffCalculator），以及 RetryScheduler 的基础数据结构。但关键的后台调度功能尚未完成：

1. **定时扫描逻辑缺失** - 没有后台线程定期查询数据库中的待重试队列
2. **任务触发机制未完成** - 无法自动启动 Agent Worker 执行重试任务
3. **调度器生命周期管理缺失** - Agent Worker 启动/停止时未集成调度器
4. **并发控制框架存在但未使用** - RetryScheduler 的并发限制逻辑已完成，但未与实际的 Story 查询和任务分发结合

这导致失败的用户故事即使被标记为 `scheduled_retry`，也无法自动恢复执行，需要人工干预。

## Goals / Non-Goals

**Goals:**
- 实现完整的后台调度器，每 30 秒自动扫描待重试队列
- 实现并发控制，最多同时执行 3 个重试任务
- 在 Agent Worker 启动时自动启动调度器，关闭时优雅停止
- 完善重试历史记录，记录每次重试的开始时间、结束时间和结果
- 通过 WebSocket 实时推送重试任务的状态变更

**Non-Goals:**
- 不修改现有的 RetryEngine 决策逻辑（错误分类、指数退避算法保持不变）
- 不改变数据库 schema（复用现有的 `user_stories` 和 `user_story_retry_history` 表）
- 不实现分布式调度（当前为单节点部署，未来可扩展）
- 不实现手动重试 UI（已有基础界面，本次仅完善后端逻辑）

## Decisions

### Decision 1: 调度器运行模式

**选择**：在 Agent Worker 的后台线程中运行独立的调度循环

**理由**：
- ✅ 与现有的 Agent Loop 架构一致（都是 tokio::spawn 异步任务）
- ✅ 共享相同的数据库连接和 WebSocket 管理器
- ✅ 无需额外的进程或线程管理复杂度
- ✅ 便于日志统一收集和监控

**替代方案**：
- 独立进程：增加部署复杂度和通信开销
- 前端驱动：依赖用户操作，无法实现真正的自动化

**实现细节**：
```rust
// 在 AgentWorker::start() 中新增调度器启动逻辑
pub async fn start(&mut self) -> Result<(), String> {
    // ... 现有的 Agent Loop 启动代码 ...
    
    // 启动重试调度器
    if let Some(websocket_manager) = &self.websocket_manager {
        let scheduler_config = SchedulerConfig::default();
        let mut scheduler = RetryScheduler::new(scheduler_config);
        
        tokio::spawn(async move {
            scheduler.run(project_id, websocket_manager.clone()).await;
        });
    }
}
```

### Decision 2: 待重试队列查询策略

**选择**：使用 SQL 查询直接过滤 `status='scheduled_retry' AND next_retry_at <= now()`

**理由**：
- ✅ 简单高效，利用 SQLite 索引加速查询
- ✅ 原子性保证，避免竞态条件
- ✅ 支持 LIMIT 子句控制并发数量

**SQL 查询**：
```sql
SELECT * FROM user_stories 
WHERE status = 'scheduled_retry' 
  AND next_retry_at <= datetime('now')
ORDER BY next_retry_at ASC
LIMIT :max_concurrent_retries
```

**索引优化**：
- 已有索引：`idx_user_stories_status` (status 字段)
- 建议新增：`idx_user_stories_next_retry_at` (next_retry_at 字段)

### Decision 3: 重试任务触发方式

**选择**：调用现有的 `execute_user_story` 方法，复用完整的 Agent 执行流程

**理由**：
- ✅ 代码复用，避免重复实现
- ✅ 保证重试任务与新任务行为一致
- ✅ 自动继承 Worktree 创建、AI CLI 执行、Git Commit 等完整流程

**实现细节**：
```rust
// 在 RetryScheduler::run() 中
for story in pending_stories {
    // 1. 更新状态为 in_progress
    db::update_user_story_status(&conn, &story.id, "in_progress")?;
    
    // 2. 记录重试开始时间
    let retry_record = UserStoryRetryHistory {
        user_story_id: story.id.clone(),
        retry_number: story.retry_count + 1,
        triggered_at: Utc::now().to_rfc3339(),
        result: Some("pending".to_string()),
        ..Default::default()
    };
    db::create_retry_history_record(&conn, &retry_record)?;
    
    // 3. 调用现有的 execute_user_story 方法
    Self::execute_user_story(
        &story,
        &daemon_manager,
        &websocket_manager,
        &worktree_manager,
    ).await?;
}
```

### Decision 4: 优雅停止机制

**选择**：使用 tokio::select! 监听停止信号，等待当前重试任务完成后退出

**理由**：
- ✅ 避免强制中断正在执行的重试任务
- ✅ 保证数据一致性（状态更新、日志记录完整）
- ✅ 符合 Rust 异步编程最佳实践

**实现细节**：
```rust
pub async fn run(&mut self, project_id: String, websocket_manager: Arc<RwLock<WebSocketManager>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(self.config.check_interval_seconds));
    let mut shutdown_signal = tokio::signal::ctrl_c().fuse();
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // 执行单次扫描
                if let Err(e) = self.scan_and_trigger(&project_id, &websocket_manager).await {
                    log::error!("[RetryScheduler] Scan failed: {}", e);
                }
            }
            _ = &mut shutdown_signal => {
                log::info!("[RetryScheduler] Received shutdown signal, waiting for active retries...");
                
                // 等待所有活跃的重试任务完成
                while !self.active_retries.is_empty() {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
                
                log::info!("[RetryScheduler] All active retries completed, shutting down");
                break;
            }
        }
    }
}
```

### Decision 5: 健康检查与监控

**选择**：通过共享的 `active_retries` HashMap 提供调度器状态查询接口

**理由**：
- ✅ 简单轻量，无需额外的监控系统
- ✅ 可集成到现有的 AgentMonitor 面板
- ✅ 支持实时查询活跃重试数量和详情

**实现细节**：
```rust
impl RetryScheduler {
    /// 获取调度器状态（用于监控）
    pub fn get_status(&self) -> SchedulerStatus {
        SchedulerStatus {
            is_running: true,
            active_retry_count: self.active_retries.len(),
            active_retries: self.active_retries.clone(),
            last_scan_at: self.last_scan_at.clone(),
        }
    }
}
```

## Risks / Trade-offs

### Risk 1: 数据库查询性能
**风险**：每 30 秒扫描一次数据库，可能在高负载下影响性能  
**缓解**：
- 使用索引优化查询（status + next_retry_at 复合索引）
- LIMIT 子句限制返回数量
- 监控查询耗时，必要时调整扫描间隔

### Risk 2: 并发冲突
**风险**：多个 Worker 同时扫描可能导致重复触发重试  
**缓解**：
- 使用乐观锁机制（assigned_agent 字段）
- 在更新状态前检查是否已被其他 Worker 锁定
- 记录并发冲突日志，便于排查问题

### Risk 3: 重试风暴
**风险**：大量 Story 同时到达重试时间，导致资源过载  
**缓解**：
- 严格的并发限制（max_concurrent_retries = 3）
- 指数退避算法分散重试时间
- 监控队列长度，必要时告警

### Trade-off 1: 调度精度 vs 资源消耗
**选择**：30 秒扫描间隔  
**权衡**：
- 更短的间隔（如 10 秒）→ 更快的响应，但更高的 CPU/DB 负载
- 更长的间隔（如 60 秒）→ 更低的负载，但重试延迟增加
- 30 秒是合理的平衡点，可根据实际运行情况调整

### Trade-off 2: 优雅停止等待时间
**选择**：无限等待直到所有活跃重试完成  
**权衡**：
- 无限等待 → 保证数据一致性，但可能延长关闭时间
- 超时强制停止 → 快速关闭，但可能导致状态不一致
- 当前选择保守策略，优先保证一致性

## Migration Plan

**部署步骤**：
1. 代码合并到 main 分支
2. 运行 `cargo test` 确保所有测试通过
3. 构建新版本 Tauri 应用
4. 用户更新应用后自动启用新调度器

**回滚策略**：
- 如果调度器导致严重问题，可通过配置禁用（TODO: 添加配置开关）
- 回滚到上一版本应用即可恢复旧行为
- 数据库 schema 无变化，回滚无需数据迁移

**监控指标**：
- 调度器启动成功日志：`[RetryScheduler] Started successfully`
- 每次扫描的 Story 数量：`[RetryScheduler] Found X pending retries`
- 活跃重试数量监控：通过 AgentMonitor 面板查看

## Open Questions

1. **是否需要配置化扫描间隔？** - 当前硬编码为 30 秒，是否需要允许用户自定义？
2. **是否需要重试优先级队列？** - 当前按 `next_retry_at` 排序，是否需要考虑 Story 优先级？
3. **是否需要重试失败告警？** - 当连续多次重试失败时，是否通知用户？
4. **是否需要手动触发立即重试？** - 当前已有 Command，但是否需要在 UI 中暴露？

这些问题可在后续迭代中根据实际需求决定。

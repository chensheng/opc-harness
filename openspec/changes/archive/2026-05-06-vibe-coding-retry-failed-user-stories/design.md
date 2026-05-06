## Context

当前系统已实现基础的用户故事失败状态管理：
- 数据库表 `user_stories` 包含 `status`、`failed_at`、`error_message`、`retry_count` 字段
- 前端提供了手动重试功能，将 failed 状态的 Story 重置为 draft
- Agent Worker 在执行失败时会调用 `fail_user_story` 更新状态

但现有实现存在以下局限：
1. **被动重试**：需要用户手动触发，Agent 无法自主处理
2. **无智能决策**：不考虑失败原因和错误类型，盲目重试
3. **无退避策略**：可能立即重试导致连续失败
4. **缺少历史追踪**：无法分析重试模式和优化策略

## Goals / Non-Goals

**Goals:**
1. 实现智能重试决策引擎，根据错误类型自动决定是否重试
2. 实现指数退避算法，避免频繁重试导致的资源浪费
3. 添加重试历史记录，支持分析和优化
4. 提供可配置的重试策略，允许用户自定义行为
5. 改进前端 UI，清晰展示重试状态和历史

**Non-Goals:**
1. 不改变现有的用户故事状态机（draft/refined/approved/in_development/completed/failed）
2. 不修改 Agent Worker 的核心执行逻辑（仅扩展重试决策层）
3. 不支持跨项目的重试策略共享（每个项目独立配置）
4. 不实现分布式重试协调（单节点即可满足需求）

## Decisions

### Decision 1: 重试决策引擎架构

**选择**：在 Agent Worker 中嵌入轻量级重试决策引擎，而非独立服务

**理由**：
- ✅ 降低系统复杂度，无需新增服务进程
- ✅ 减少网络开销，决策逻辑与执行逻辑在同一进程
- ✅ 便于调试和维护，代码集中在一处
- ❌ 耦合度略高，但通过接口隔离可缓解

**替代方案**：
- 独立重试服务：增加部署复杂度和运维成本，不适合当前规模
- 前端驱动重试：依赖用户操作，无法实现自动化

**实现细节**：
```rust
// src-tauri/src/agent/retry_engine.rs
pub struct RetryEngine {
    max_retries: u32,
    backoff_strategy: BackoffStrategy,
    error_classifier: ErrorClassifier,
}

impl RetryEngine {
    pub fn should_retry(&self, story: &UserStory, error: &str) -> RetryDecision {
        // 1. 检查是否超过最大重试次数
        // 2. 分类错误类型（临时/永久）
        // 3. 计算下次重试时间（指数退避）
        // 4. 返回决策结果
    }
}
```

### Decision 2: 错误分类策略

**选择**：基于正则表达式和关键词匹配的错误分类器

**分类规则**：
- **临时错误**（可重试）：
  - 网络超时：`timeout`、`connection refused`、`network error`
  - API 限流：`rate limit`、`429 Too Many Requests`
  - 服务暂时不可用：`503 Service Unavailable`、`temporary failure`
  
- **永久错误**（不可重试）：
  - 代码逻辑错误：`syntax error`、`compilation failed`、`type error`
  - 依赖缺失：`module not found`、`dependency resolution failed`
  - 权限问题：`permission denied`、`unauthorized`、`forbidden`
  - 数据错误：`invalid input`、`data validation failed`

**理由**：
- ✅ 简单高效，无需训练 ML 模型
- ✅ 易于维护和扩展，通过配置文件调整规则
- ✅ 透明度高，开发者可理解分类逻辑

**替代方案**：
- ML 分类器：需要大量标注数据，维护成本高
- 人工标记：依赖用户输入，不够自动化

### Decision 3: 指数退避算法

**选择**：标准指数退避 + 随机抖动（Jitter）

**公式**：
```
delay = min(base_delay * 2^retry_count + random_jitter, max_delay)
```

**参数**：
- `base_delay`: 60 秒（1 分钟）
- `max_delay`: 3600 秒（1 小时）
- `jitter_range`: ±10% 的 delay 值

**示例**：
- 第 1 次重试：60s ± 6s
- 第 2 次重试：120s ± 12s
- 第 3 次重试：240s ± 24s

**理由**：
- ✅ 避免多个 Story 同时重试导致的资源竞争
- ✅ 给予系统恢复时间，提高重试成功率
- ✅ 随机抖动防止"惊群效应"

**替代方案**：
- 固定延迟：不够灵活，可能导致过度等待或重试过快
- 线性退避：增长速度慢，不适合长时间故障场景

### Decision 4: 重试历史存储方案

**选择**：新增独立表 `user_story_retry_history`

**表结构**：
```sql
CREATE TABLE user_story_retry_history (
    id TEXT PRIMARY KEY,
    user_story_id TEXT NOT NULL REFERENCES user_stories(id),
    retry_number INTEGER NOT NULL,
    triggered_at TEXT NOT NULL,
    error_message TEXT,
    error_type TEXT, -- 'temporary' | 'permanent'
    decision TEXT NOT NULL, -- 'retry' | 'abort'
    next_retry_at TEXT,
    completed_at TEXT,
    result TEXT, -- 'success' | 'failed' | 'pending'
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

**理由**：
- ✅ 规范化设计，避免在 `user_stories` 表中存储大量历史数据
- ✅ 支持复杂查询和分析（如重试成功率统计）
- ✅ 便于清理过期数据（可定期归档或删除）

**替代方案**：
- JSON 字段存储在 `user_stories`：查询效率低，不利于分析
- 日志文件：难以结构化查询，不适合长期存储

### Decision 5: 前端重试配置 UI

**选择**：在项目设置页面添加"重试策略"配置面板

**配置项**：
1. 最大重试次数（1-10，默认 3）
2. 基础延迟时间（30s-300s，默认 60s）
3. 最大延迟时间（300s-7200s，默认 3600s）
4. 启用/禁用自动重试（默认启用）

**UI 位置**：
- 主入口：`Settings` → `Vibe Coding` → `Retry Strategy`
- 快速入口：用户故事表格的操作列中，点击"重试"时弹出配置对话框

**理由**：
- ✅ 符合用户习惯，配置集中在设置页面
- ✅ 提供快速配置入口，方便临时调整
- ✅ 配置持久化到项目级别，不同项目可有不同策略

## Risks / Trade-offs

### Risk 1: 重试风暴

**风险**：多个 Story 同时失败并触发重试，导致系统负载激增

**缓解措施**：
- 实现随机抖动，分散重试时间
- 添加全局重试队列，限制并发重试数量（最多 3 个）
- 监控系统负载，超过阈值时暂停新重试

### Risk 2: 无效重试浪费资源

**风险**：对永久错误进行重试，浪费计算资源和 API 配额

**缓解措施**：
- 严格的错误分类，永久错误直接标记为失败
- 记录重试历史，相同错误重复出现时提前终止
- 提供手动覆盖选项，允许用户强制重试

### Risk 3: 数据库迁移兼容性

**风险**：新增表和字段可能导致旧版本客户端兼容性问题

**缓解措施**：
- 所有新字段设置为可选，带默认值
- 数据库迁移脚本包含版本检查和回滚逻辑
- 前端代码使用防御性编程，处理字段缺失情况

### Trade-off 1: 复杂度 vs 灵活性

**权衡**：引入重试引擎增加了系统复杂度，但提供了更高的自动化程度

**决策**：接受适度复杂度，因为：
- 重试逻辑封装在独立模块，不影响其他代码
- 带来的自动化收益远大于维护成本
- 可通过配置简化使用，高级功能按需启用

### Trade-off 2: 实时性 vs 准确性

**权衡**：立即重试响应快但成功率低，延迟重试成功率高但用户体验稍差

**决策**：采用指数退避，平衡两者：
- 首次重试较快（1 分钟），适合临时故障
- 后续重试逐步延迟，给系统充分恢复时间
- 前端显示预计重试时间，管理用户预期

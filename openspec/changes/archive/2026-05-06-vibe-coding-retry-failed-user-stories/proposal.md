## Why

当前 Vibe Coding 系统已经实现了用户故事的失败状态管理（failed status），包括错误消息、重试次数和失败时间戳等字段。前端也提供了基本的重试功能，可以将 failed 状态的 Story 重置为 draft。

然而，当前的重试机制存在以下问题：
1. **缺少智能重试策略**：简单地将状态重置为 draft，没有考虑失败原因和上下文
2. **Agent Worker 未自动处理重试**：需要手动触发，无法利用 Agent 的自主能力
3. **缺少重试限制和退避策略**：可能导致无限重试或资源浪费
4. **缺少重试历史追踪**：无法分析重试模式和失败趋势

本变更旨在增强 Vibe Coding 的智能体支持，使其能够自动识别、处理和优化失败用户故事的重试流程，提升系统的鲁棒性和自动化程度。

## What Changes

- **新增智能重试决策引擎**：Agent Worker 根据失败原因、重试次数和错误类型自动决定是否重试
- **实现指数退避重试策略**：避免频繁重试导致的资源浪费，首次重试等待 1 分钟，后续按指数增长
- **增加最大重试次数限制**：默认最多重试 3 次，超过后标记为永久失败并通知用户
- **增强错误分类和处理**：区分临时错误（网络超时、API 限流）和永久错误（代码逻辑错误、依赖缺失）
- **添加重试历史记录**：记录每次重试的时间、结果和错误信息，用于分析和优化
- **改进前端重试 UI**：显示重试次数、上次失败原因和预计下次重试时间
- **新增重试配置选项**：允许用户在设置中自定义最大重试次数和退避策略

## Capabilities

### New Capabilities
- `agent-retry-engine`: 智能体重试决策引擎，包括错误分类、重试策略和退避算法
- `retry-history-tracking`: 重试历史记录追踪，记录每次重试的详细信息和分析数据

### Modified Capabilities
- `vibe-coding`: 扩展现有用户故事状态机，增加重试相关的行为和约束
- `agent-initialization`: 更新 Agent Worker 初始化逻辑，集成重试引擎配置

## Impact

**受影响的代码**:
- `src-tauri/src/agent/agent_worker.rs` - Agent Worker 核心逻辑，需要集成重试引擎
- `src-tauri/src/db/sprint_repository.rs` - 数据库操作，需要新增重试历史相关方法
- `src/stores/userStoryStore.ts` - 前端状态管理，需要扩展重试相关字段和方法
- `src/components/vibe-coding/UserStoryManager.tsx` - 前端 UI，需要显示重试信息和配置
- `src/types/index.ts` - 类型定义，需要新增重试历史和相关配置类型

**数据库变更**:
- 新增 `user_story_retry_history` 表，存储每次重试的详细记录
- 在 `user_stories` 表中新增 `next_retry_at` 字段（下次重试时间）
- 在 `user_stories` 表中新增 `max_retries` 字段（最大重试次数配置）

**API 变更**:
- 新增 Command: `get_user_story_retry_history` - 获取用户故事的重试历史
- 新增 Command: `update_user_story_retry_config` - 更新用户故事的重试配置
- 修改 Command: `decompose_user_stories_streaming` - 返回时包含重试配置信息

**向后兼容性**:
- ✅ 所有新字段均为可选，带有默认值
- ✅ 现有用户故事不受影响，可以平滑迁移
- ⚠️ 需要运行数据库迁移脚本添加新表和字段

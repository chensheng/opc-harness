## Context

### 背景
当前 OPC-Harness 的 Vibe Coding 功能通过 AI 智能体（Initializer、Coding、MR_Creation）自动化软件开发流程。然而，智能体的运行过程对用户而言是一个"黑盒"：
- 用户只能看到智能体的基本状态（idle/running/completed）
- 日志通过 WebSocket 实时推送，但缺乏结构化和分类
- 无法追踪智能体的思考过程和决策路径
- 出现问题时难以定位根因

### 当前状态
- **AgentMonitor.tsx**: 展示智能体列表和基本状态，日志存储在内存中
- **LogTerminal.tsx**: 使用 Mock 数据，未与后端真实集成
- **useAgent Hook**: 处理 WebSocket 连接和消息分发
- **后端**: 通过 stdio 通道与智能体通信，日志通过 JSON-RPC 消息传递

### 约束
- 必须保持与现有 Tauri v2 架构的兼容性
- 前端使用 React + TypeScript + Zustand
- 后端使用 Rust + SQLite
- 实时性要求：日志延迟 < 1 秒
- 性能要求：支持同时监控 10+ 个智能体

## Goals / Non-Goals

**Goals:**
1. 实现智能体运行状态的实时可视化（状态、进度、当前任务）
2. 建立结构化日志系统，支持分级、过滤、搜索和导出
3. 记录智能体的思考链和工具调用轨迹
4. 提供性能指标监控（CPU、内存、API 调用）
5. 实现异常检测和告警机制

**Non-Goals:**
1. 不改变智能体本身的执行逻辑
2. 不引入新的 AI 厂商依赖
3. 不支持历史日志的实时回溯（仅支持当前会话）
4. 不提供智能体间的通信监控

## Decisions

### 1. 日志存储策略：内存优先 + 异步持久化

**决策**: 采用双层存储架构
- **内存层**: 使用 Zustand store 缓存最近 100 条日志，保证 UI 实时响应
- **持久层**: 异步写入 SQLite，用于审计和复盘

**理由**:
- ✅ 避免每次日志都触发数据库 I/O，保证实时性
- ✅ 内存层支持快速过滤和搜索
- ✅ 持久层提供数据持久化，支持历史查询

**替代方案考虑**:
- ❌ 纯内存存储：重启后数据丢失，不符合审计要求
- ❌ 纯数据库存储：高频日志写入会导致性能瓶颈

### 2. 实时通信：保持现有 WebSocket 架构

**决策**: 复用现有的 `useAgent` Hook 和 WebSocket 通道

**理由**:
- ✅ 已有基础设施，无需新增依赖
- ✅ WebSocket 天然支持双向实时通信
- ✅ 已实现消息分发和状态同步机制

**消息格式设计**:
```typescript
interface AgentMessage {
  type: 'log' | 'status' | 'progress' | 'error' | 'trace'
  sessionId: string        // agent-{agentId}
  timestamp: string
  content: string
  metadata?: {
    level?: 'info' | 'warn' | 'error' | 'debug' | 'success'
    source?: string        // 日志来源（如 'git', 'eslint', 'coding-agent'）
    progress?: number      // 0-100
    currentTask?: string
    trace?: TraceData      // 思考链数据
  }
}
```

### 3. 追踪数据模型：事件溯源模式

**决策**: 使用事件溯源模式记录智能体执行轨迹

**理由**:
- ✅ 完整记录状态变化历史，支持回放
- ✅ 事件不可变，便于审计和调试
- ✅ 支持按时间线查询和分析

**数据模型**:
```rust
struct AgentTrace {
    id: String,
    agent_id: String,
    session_id: String,
    event_type: String,      // 'thought', 'tool_call', 'tool_result', 'decision'
    timestamp: DateTime,
    data: String,            // JSON 序列化的事件数据
    parent_id: Option<String>, // 支持事件关联
}
```

### 4. 性能监控：采样 + 聚合

**决策**: 采用采样策略收集性能指标

**实现**:
- CPU/内存：每 5 秒采样一次
- API 调用：实时记录，但 UI 聚合显示（如"过去 1 分钟：10 次调用"）
- 响应时间：记录 P50/P90/P99 分位数

**理由**:
- ✅ 避免高频采样导致的性能开销
- ✅ 聚合显示减少 UI 渲染压力
- ✅ 分位数统计更能反映真实体验

### 5. 告警机制：基于规则的本地检测

**决策**: 在前端实现基于规则的告警检测

**规则示例**:
- 智能体超过 5 分钟无状态更新 → 警告
- 错误日志频率 > 10 条/分钟 → 严重告警
- CPU 使用率 > 90% 持续 1 分钟 → 警告

**理由**:
- ✅ 前端检测响应更快，无需后端额外服务
- ✅ 规则配置简单，易于扩展
- ✅ 避免引入复杂的告警系统（如 Prometheus）

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| 日志量过大导致内存占用过高 | 限制内存缓存数量（100 条），旧日志自动淘汰 |
| WebSocket 连接不稳定 | 实现重连机制，断线时本地缓存日志，重连后补传 |
| 数据库写入成为瓶颈 | 使用批量写入（每 100 条或每 10 秒），异步执行 |
| 追踪数据量过大 | 仅记录关键事件，过滤低价值日志 |
| 多智能体同时监控导致 UI 卡顿 | 使用虚拟列表（react-window），按需渲染 |

## Migration Plan

### 阶段 1：基础设施（Week 1）
1. 创建数据库表（`agent_logs`, `agent_traces`, `agent_alerts`）
2. 实现后端日志收集和存储 Service
3. 扩展 WebSocket 消息格式

### 阶段 2：前端集成（Week 2）
1. 升级 `AgentMonitor.tsx`，增强状态展示
2. 重构 `LogTerminal.tsx`，集成真实数据
3. 新增 `AgentTracing.tsx` 组件

### 阶段 3：告警与优化（Week 3）
1. 实现告警规则和通知机制
2. 性能优化（虚拟列表、懒加载）
3. E2E 测试和文档

### 回滚策略
- 数据库变更使用迁移脚本，支持回滚
- 新功能通过 Feature Flag 控制，可随时关闭
- 保持向后兼容，旧版本前端仍可正常使用

## Open Questions

1. **日志保留策略**: 应该保留多久的历史日志？（建议：7 天）
2. **告警通知方式**: 仅 UI 提示，还是需要邮件/弹窗？（建议：先实现 UI 提示）
3. **追踪数据粒度**: 应该记录到函数级别还是任务级别？（建议：任务级别，避免数据量过大）
4. **性能指标采集**: 是否需要采集网络延迟？（建议：暂不采集，后续根据需求扩展）

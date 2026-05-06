## Why

当前 Vibe Coding 中的 AI 智能体运行过程缺乏细粒度的可观测性，用户无法实时了解智能体的执行状态、思考过程和任务进度。这导致：
- 用户面对"黑盒"执行过程，无法判断智能体是否正常工作
- 出现问题时难以定位原因（是网络问题、代码错误还是逻辑问题？）
- 缺乏结构化的执行追踪，难以进行问题复盘和性能优化

## What Changes

- **新增智能体运行状态实时追踪**：展示智能体的当前任务、执行阶段、进度百分比
- **新增结构化日志系统**：按级别（INFO/WARN/ERROR/DEBUG/SUCCESS）分类显示，支持过滤和搜索
- **新增智能体思考过程可视化**：展示 AI 的思考链（Chain of Thought）、决策过程和工具调用
- **新增性能指标监控**：实时显示 CPU、内存使用情况，API 调用次数和耗时
- **新增执行轨迹记录**：记录智能体的完整执行路径，支持回溯和回放
- **新增告警机制**：当智能体出现异常（长时间无响应、错误率过高）时主动通知用户

## Capabilities

### New Capabilities

- **agent-observability**: 智能体可观测性核心能力，包含状态追踪、日志管理、性能监控
- **agent-tracing**: 智能体执行追踪能力，包含思考链记录、工具调用追踪、执行路径回放
- **agent-alerting**: 智能体告警能力，包含异常检测、阈值配置、通知推送

### Modified Capabilities

- (无现有能力需要修改)

## Impact

### 前端代码
- `src/components/vibe-coding/AgentMonitor.tsx` - 增强状态展示和日志可视化
- `src/components/vibe-coding/LogTerminal.tsx` - 升级为结构化日志终端
- `src/components/vibe-coding/` - 新增 `AgentTracing.tsx`、`AgentAlertPanel.tsx` 组件
- `src/hooks/useAgent.ts` - 增强 WebSocket 消息处理和状态同步
- `src/stores/agentStore.ts` - 新增智能体状态和日志的全局管理

### 后端代码
- `src-tauri/src/commands/agent.rs` - 新增可观测性相关命令
- `src-tauri/src/services/agent_service.rs` - 增强智能体状态管理和日志收集
- `src-tauri/src/db/` - 新增日志和追踪数据存储表
- `src-tauri/src/models/` - 新增日志、追踪、告警相关模型

### 数据库
- 新增 `agent_logs` 表 - 存储结构化日志
- 新增 `agent_traces` 表 - 存储执行轨迹
- 新增 `agent_alerts` 表 - 存储告警配置和历史

### 依赖
- 前端：可能需要 `@tanstack/react-query` 用于数据缓存和同步
- 后端：可能需要 `tracing` crate 用于结构化日志

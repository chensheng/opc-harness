## Why

AgentMonitor 组件中存在大量调试日志(console.log/console.warn),这些日志在开发和生产环境中都会输出,导致控制台信息过载,影响开发者调试效率和用户体验。需要精简不必要的日志输出,保留关键错误和警告信息。

## What Changes

- 移除 AgentMonitor.tsx 中的冗余 console.log 调试日志
- 保留关键的错误日志(console.error)和警告日志(console.warn)
- 优化 WebSocket 消息处理的日志输出策略
- 减少智能体匹配过程中的重复日志输出

## Capabilities

### New Capabilities
<!-- No new capabilities introduced -->

### Modified Capabilities
<!-- No existing spec requirements changed - this is an implementation detail optimization -->

## Impact

- **Affected Files**: 
  - `src/components/vibe-coding/AgentMonitor.tsx` (主要修改)
- **Impact Level**: Low - 仅影响前端日志输出,不改变功能逻辑
- **Breaking Changes**: None
- **Benefits**: 
  - 减少控制台噪音,提升开发体验
  - 降低生产环境的日志开销
  - 保留关键错误信息便于问题排查

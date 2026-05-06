## Why

在 Tauri 开发环境中,前端调试控制台(console.log、console.error 等)的日志无法在后端 Rust 日志中看到。这导致开发者需要在浏览器 DevTools 和后端日志之间频繁切换,降低了调试效率和问题排查体验。通过实现前端日志到后端的桥接,可以在单一位置查看所有日志,提升开发体验和可观测性。

## What Changes

- 在前端实现 console 方法拦截器,捕获所有 console 输出
- 通过 Tauri invoke 将前端日志发送到后端 Rust 命令
- 后端使用 log/tracing 框架记录前端日志,支持不同级别(log/info/warn/error)
- 在开发模式下自动启用此功能,生产模式可选配置
- 保留原始 console 行为,确保浏览器 DevTools 仍能看到日志

## Capabilities

### New Capabilities
- `frontend-console-bridge`: 前端控制台日志到后端的桥接能力,包括 console 拦截、Tauri 通信和后端日志记录

### Modified Capabilities
- `agent-tracing`: 扩展 Agent 追踪能力,纳入前端日志流,提供更完整的可观测性视图

## Impact

**受影响代码**:
- 前端: `src/hooks/` 新增 useConsoleBridge hook
- 后端: `src-tauri/src/commands/` 新增 console_log 命令
- 应用初始化: `src/main.tsx` 或 `src/App.tsx` 集成 console bridge

**API 变化**:
- 新增 Tauri command: `console_log(level: string, message: string, metadata?: object)`

**依赖影响**:
- 前端: 无新依赖(使用原生 console 和 Tauri API)
- 后端: 可能需确认 log/tracing crate 已配置(项目已有)

**系统影响**:
- 仅在开发环境启用,对生产性能无影响
- 日志量较大时可能增加 IPC 通信开销(可通过节流优化)

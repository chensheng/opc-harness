## ADDED Requirements

### Requirement: Frontend console interception

系统 SHALL 拦截前端所有 console 方法调用(log, info, warn, error, debug),并将日志消息转发到后端 Rust 日志系统。

#### Scenario: Intercept console.log call
- **WHEN** 前端代码调用 `console.log("User logged in", { userId: 123 })`
- **THEN** 系统捕获该调用并发送到后端,后端记录为 info 级别日志

#### Scenario: Intercept console.error call
- **WHEN** 前端代码调用 `console.error("Failed to fetch data", error)`
- **THEN** 系统捕获该调用并发送到后端,后端记录为 error 级别日志

#### Scenario: Preserve original console behavior
- **WHEN** 前端调用任何 console 方法
- **THEN** 浏览器 DevTools Console 仍然显示原始日志输出,功能不受影响

### Requirement: Tauri command for console logging

系统 SHALL 提供名为 `console_log` 的 Tauri command,接收日志级别和消息参数,并使用 tracing 框架记录日志。

#### Scenario: Command receives log message
- **WHEN** 前端调用 `invoke('console_log', { level: 'error', message: 'Something failed' })`
- **THEN** 后端执行 `tracing::error!("[Frontend] Something failed")`

#### Scenario: Command handles different log levels
- **WHEN** 前端发送不同级别的日志(debug, info, warn, error)
- **THEN** 后端使用对应的 tracing 宏(debug!, info!, warn!, error!)记录

#### Scenario: Command includes frontend source identifier
- **WHEN** 后端记录来自前端的日志
- **THEN** 日志消息包含 `[Frontend]` 前缀以区分来源

### Requirement: Safe serialization of console arguments

系统 SHALL 安全地序列化 console 调用的额外参数(对象、数组等),处理循环引用和不可序列化的值。

#### Scenario: Serialize simple objects
- **WHEN** console.log 包含简单对象 `{ name: "Alice", age: 30 }`
- **THEN** 对象被序列化为 JSON 字符串并附加到日志消息

#### Scenario: Handle circular references
- **WHEN** console.log 包含循环引用的对象
- **THEN** 系统捕获序列化错误并使用降级策略(如 `[Circular Object]`),不抛出异常

#### Scenario: Handle non-serializable values
- **WHEN** console.log 包含函数、Symbol 或 undefined
- **THEN** 这些值被转换为字符串表示(如 `"function() {}"`, `"Symbol()"`, `"undefined"`)

### Requirement: Development mode auto-enable

系统 SHALL 在开发模式(`import.meta.env.DEV === true`)下自动启用 console bridge,生产模式默认禁用。

#### Scenario: Auto-enable in development
- **WHEN** 应用以 `npm run tauri:dev` 启动
- **THEN** console bridge 自动初始化,前端日志转发到后端

#### Scenario: Disable in production by default
- **WHEN** 应用以生产模式构建并运行
- **THEN** console bridge 不初始化,避免性能开销

#### Scenario: Override via environment variable
- **WHEN** 设置环境变量 `VITE_ENABLE_CONSOLE_BRIDGE=true`
- **THEN** 无论开发或生产模式,console bridge 都启用

### Requirement: Performance optimization

系统 SHALL 确保 console bridge 对应用性能的影响最小化,避免大量日志拖慢应用。

#### Scenario: Async invoke without blocking UI
- **WHEN** 前端频繁调用 console.log
- **THEN** invoke 调用异步执行,不阻塞 UI 线程

#### Scenario: Throttle excessive logs (future)
- **WHEN** 每秒日志数量超过阈值(如 100 条/秒)
- **THEN** 系统可选地丢弃超额日志或合并为摘要(此功能标记为未来增强)

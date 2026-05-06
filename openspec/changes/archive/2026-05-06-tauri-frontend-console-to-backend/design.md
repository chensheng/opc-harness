## Context

当前 Tauri 应用的前端日志(React/TypeScript)和后端日志(Rust)是分离的:
- 前端日志显示在浏览器 DevTools Console
- 后端日志通过 `tracing` crate 输出到终端或文件

开发者调试时需要同时打开两个窗口,降低了效率。特别是在处理跨层问题(如前端调用后端命令失败)时,需要在两个日志源之间来回切换。

**约束条件**:
- 不能影响生产环境性能
- 必须保留浏览器 DevTools 的原始 console 功能
- 需要支持不同日志级别(log/info/warn/error/debug)
- 项目已使用 `tracing` crate 作为后端日志框架

## Goals / Non-Goals

**Goals:**
- 实现前端 console 日志自动转发到后端 Rust 日志系统
- 支持所有 console 方法(console.log, console.error, console.warn, console.info, console.debug)
- 在后端日志中清晰标识日志来源(前端/后端)和原始级别
- 仅在开发模式启用,可通过环境变量控制
- 保持低开销,避免大量日志拖慢应用

**Non-Goals:**
- 不替换浏览器 DevTools 的 console 功能
- 不实现日志过滤或搜索功能(依赖现有日志工具)
- 不持久化前端日志到文件(由后端日志系统负责)
- 不支持生产环境的默认启用(需显式配置)

## Decisions

### Decision 1: 使用 Console 拦截器而非自定义日志 API

**选择**: 拦截原生 console 方法,而非要求开发者使用新的日志 API

**理由**:
- ✅ 零侵入性:现有代码无需修改,所有 console.log 自动转发
- ✅ 完整性:捕获第三方库的日志输出
- ✅ 简单性:只需在应用启动时初始化一次

**替代方案**:
- ❌ 自定义 hook(useLogger):需要改造所有日志调用,工作量大且易遗漏
- ❌ ESLint 规则强制使用新 API:增加开发负担,无法捕获第三方库日志

### Decision 2: 通过 Tauri Invoke 而非 Event 通信

**选择**: 使用 `invoke()` 同步调用后端命令,而非 `emit()` 异步事件

**理由**:
- ✅ 简单性:invoke 是请求-响应模式,无需管理事件监听器生命周期
- ✅ 可靠性:invoke 有明确的错误处理机制
- ✅ 性能:console 日志是低频操作,invoke 的开销可接受

**替代方案**:
- ⚠️ Event emit:更适合高频、单向数据流,但需要额外管理事件监听,复杂度更高

### Decision 3: 日志级别映射策略

**映射关系**:
```
console.log    → tracing::info!
console.info   → tracing::info!
console.warn   → tracing::warn!
console.error  → tracing::error!
console.debug  → tracing::debug!
```

**理由**:
- 语义对齐:console 级别与 tracing 级别自然对应
- 可见性:warn/error 在后端日志中高亮显示,便于快速定位问题

### Decision 4: 元数据序列化策略

**选择**: 将 console 的额外参数序列化为 JSON 字符串附加到日志消息

**实现**:
```typescript
// 前端
const metadata = args.slice(1).map(arg => 
  typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
).join(' ');
invoke('console_log', { level, message: `${args[0]} ${metadata}` });
```

**理由**:
- ✅ 完整性:保留对象、数组等复杂类型的可读表示
- ✅ 简单性:后端只需记录字符串,无需解析结构化数据
- ⚠️ 权衡:大对象可能导致日志冗长(可通过截断优化)

### Decision 5: 开发模式自动启用

**选择**: 在 `main.tsx` 中检测 `import.meta.env.DEV`,自动初始化 console bridge

**理由**:
- ✅ 零配置:开发者无需手动启用
- ✅ 安全性:生产环境默认关闭,避免性能影响
- ✅ 灵活性:可通过环境变量 `VITE_ENABLE_CONSOLE_BRIDGE` 覆盖

## Risks / Trade-offs

### Risk 1: 大量日志导致 IPC 开销

**风险**: 如果前端频繁输出日志(如在循环中),可能拖慢应用

**缓解措施**:
- 仅在开发模式启用
- 未来可添加节流机制(如每秒最多 100 条)
- 开发者可通过条件日志减少输出

### Risk 2: 循环引用导致 JSON.stringify 失败

**风险**: 如果 console.log 包含循环引用的对象,JSON.stringify 会抛出异常

**缓解措施**:
- 使用安全的序列化函数(检测循环引用)
- 降级策略:序列化失败时使用 `[Object]` 占位符

### Risk 3: 日志顺序可能不一致

**风险**: 由于异步 invoke,后端日志顺序可能与前端调用顺序不完全一致

**影响**: 低(日志主要用于调试,微小顺序差异可接受)

**缓解措施**:
- 在前端消息中添加时间戳
- 后端日志框架通常保证单线程内的顺序性

## Migration Plan

**部署步骤**:
1. 实现 useConsoleBridge hook 和后端 command
2. 在 `main.tsx` 中集成(仅开发模式)
3. 测试验证:运行 `npm run tauri:dev`,确认后端日志显示前端输出
4. 更新文档:在 AGENTS.md 中添加使用说明

**回滚策略**:
- 移除 `main.tsx` 中的初始化代码即可禁用
- 无数据库变更,无 API 破坏性变更

**Open Questions**:
- 是否需要支持日志采样率配置?(当前不需要,后续可按需添加)
- 是否需要区分不同页面的日志?(当前通过消息内容区分,暂不需额外元数据)

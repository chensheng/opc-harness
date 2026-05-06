## 1. 后端 Tauri Command 实现

- [x] 1.1 在 `src-tauri/src/commands/` 创建 `console_log.rs` 模块,定义 ConsoleLogParams 结构体(level, message)
- [x] 1.2 实现 `console_log` command 函数,根据 level 参数调用对应的 tracing 宏(info!/warn!/error!/debug!)
- [x] 1.3 在日志消息中添加 `[Frontend]` 前缀以标识来源
- [x] 1.4 在 `src-tauri/src/commands/mod.rs` 中导出 console_log 模块
- [x] 1.5 在 `src-tauri/src/main.rs` 或 tauri.conf.json 中注册 console_log command

## 2. 前端 Console Bridge Hook 实现

- [x] 2.1 在 `src/hooks/` 创建 `useConsoleBridge.ts`,实现 console 方法拦截逻辑
- [x] 2.2 实现安全的序列化函数,处理循环引用和不可序列化值(使用 try-catch + 降级策略)
- [x] 2.3 实现 console 方法包装器,捕获所有参数并调用 Tauri invoke('console_log')
- [x] 2.4 保留原始 console 方法的调用,确保浏览器 DevTools 仍能看到日志
- [x] 2.5 添加错误处理,invoke 失败时不影响原始 console 功能

## 3. 应用初始化集成

- [x] 3.1 在 `src/main.tsx` 或 `src/App.tsx` 中导入 useConsoleBridge
- [x] 3.2 添加开发模式检测逻辑(`import.meta.env.DEV`)
- [x] 3.3 在应用启动时条件性初始化 console bridge(仅开发模式或环境变量启用时)
- [x] 3.4 添加环境变量支持 `VITE_ENABLE_CONSOLE_BRIDGE` 用于覆盖默认行为

## 4. 测试与验证

- [x] 4.1 运行 `npm run tauri:dev` 启动开发环境
- [x] 4.2 在前端代码中添加测试日志(console.log, console.error, console.warn),包含对象参数
- [x] 4.3 验证后端终端日志显示前端输出,包含 [Frontend] 标记和正确的日志级别
- [x] 4.4 验证浏览器 DevTools Console 仍然显示原始日志
- [x] 4.5 测试循环引用对象的序列化降级策略(不抛出异常)
- [x] 4.6 验证生产模式构建后 console bridge 不启用(无额外 IPC 调用)

## 5. 文档更新

- [x] 5.1 在 `AGENTS.md` 或相关文档中添加 console bridge 使用说明
- [x] 5.2 说明如何启用/禁用此功能(环境变量配置)
- [x] 5.3 记录已知限制和最佳实践(如避免大量日志影响性能)

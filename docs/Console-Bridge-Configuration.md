# Console Bridge 配置指南

## 概述

Console Bridge 功能允许将前端的 `console.log/info/warn/error/debug` 调用自动转发到后端 Rust 日志系统中，方便统一查看和管理日志。

## 配置方式

### 方式 1：环境变量（推荐）

在项目根目录创建 `.env.development` 文件：

```env
VITE_ENABLE_CONSOLE_BRIDGE=true
```

**优点**：
- ✅ 灵活控制，不同环境可以有不同的配置
- ✅ 不修改代码，通过配置文件管理
- ✅ 可以添加到 `.gitignore`，避免提交敏感配置

### 方式 2：开发模式自动启用

在 Tauri 开发模式下（`npm run tauri:dev`），Console Bridge 会自动启用，因为 `import.meta.env.DEV` 为 `true`。

### 方式 3：代码中强制启用

在 `src/main.tsx` 中传入参数：

```typescript
initializeConsoleBridge(true)  // 强制启用
initializeConsoleBridge(false) // 强制禁用
```

**优先级**：传入参数 > 环境变量 > DEV 模式

## 工作原理

```
前端 console.log('消息')
    ↓
useConsoleBridge 拦截
    ↓
序列化参数（处理循环引用）
    ↓
invoke('console_log', { level, message })
    ↓
Rust backend 接收
    ↓
log::info!("[Frontend] 消息")
    ↓
终端/日志文件显示
```

## 示例

### 前端代码

```typescript
console.log('用户登录成功', { userId: 123 })
console.error('API 请求失败', error)
console.warn('即将过期的 Token')
```

### 后端输出

```
[INFO ] [Frontend] "用户登录成功" {"userId":123}
[ERROR] [Frontend] "API 请求失败" Error: Network error
[WARN ] [Frontend] "即将过期的 Token"
```

## 注意事项

1. **循环引用处理**：Console Bridge 会自动检测并处理循环引用，将其标记为 `[Circular]`
2. **异步发送**：日志转发是异步的，不会阻塞前端 UI
3. **原始日志保留**：浏览器 DevTools 仍然会显示原始日志
4. **性能影响**：在生产环境中建议禁用，避免不必要的性能开销

## 故障排查

### 问题：前端日志没有转发到后端

**检查步骤**：

1. 确认 `.env.development` 文件中设置了 `VITE_ENABLE_CONSOLE_BRIDGE=true`
2. 重启 Tauri 开发服务器
3. 检查浏览器控制台是否有 `[ConsoleBridge] Initialized` 日志
4. 检查后端终端是否有 `[ConsoleBridge Debug] Received log` 日志

### 问题：TypeScript 类型错误

确保在 `src/main.tsx` 中添加了 ImportMeta 类型扩展：

```typescript
interface ImportMetaEnv {
  readonly DEV: boolean
  readonly VITE_ENABLE_CONSOLE_BRIDGE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
```

## 相关文件

- `src/hooks/useConsoleBridge.ts` - Console Bridge 核心实现
- `src/main.tsx` - 初始化入口
- `src-tauri/src/commands/console_log.rs` - 后端命令处理
- `.env.example` - 环境变量示例配置

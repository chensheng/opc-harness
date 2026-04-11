# ADR-005: SSE 流式输出实现方案

**状态**: 已采纳  
**日期**: 2026-03-22  
**作者**: OPC-HARNESS Team  
**优先级**: 高

## 背景与问题

当前 AI 响应为一次性返回，用户体验较差，无法实时看到 AI 生成内容。需要实现 Server-Sent Events (SSE) 流式输出，让 AI 响应能够逐字显示。

### 问题描述

1. **用户等待时间长**：AI 生成完整响应前用户无法看到任何内容
2. **缺乏进度反馈**：用户不知道 AI 是否在工作或卡住了
3. **无法中断**：一旦开始请求，无法中途取消或干预

### 约束条件

- Tauri v2 架构限制
- 需要支持多会话并发
- 保持前后端解耦

## 决策内容

采用 **Tauri v2 的事件系统（Event API）**实现 Rust 到前端的流式推送。

### 技术方案

- **Rust 端**：使用 `tauri::Emitter` trait 发送事件
- **前端**：使用 `@tauri-apps/api/core` 的 `listen` 函数监听事件
- **事件命名**：
  - `ai-stream-chunk` - 数据块（包含增量内容）
  - `ai-stream-complete` - 完成信号
  - `ai-stream-error` - 错误信号
- **数据格式**：

```json
{
  "session_id": "uuid",
  "content": "string",
  "is_complete": false,
  "error": null
}
```

### 技术选型对比

| 方案                     | 实时性     | 复杂度     | Tauri 兼容性 | 综合评分 |
| ------------------------ | ---------- | ---------- | ------------ | -------- |
| **Tauri Events（选中）** | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐⭐⭐⭐   | 9/10     |
| WebSocket                | ⭐⭐⭐⭐⭐ | ⭐⭐       | ⭐⭐⭐       | 7/10     |
| HTTP SSE                 | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐         | 6/10     |
| 轮询                     | ⭐⭐       | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐     | 5/10     |

选择 Tauri Events 的原因：

1. **原生支持**：Tauri v2 内置事件系统，无需额外依赖
2. **类型安全**：Rust 和 TypeScript 都有良好的类型定义
3. **简单易用**：API 简洁，学习成本低
4. **性能优秀**：基于 IPC，比 WebSocket 更轻量

## 技术影响

### ✅ 优势

1. **用户体验提升**
   - 实时显示 AI 生成内容
   - 打字机效果降低感知延迟
   - 支持长时间运行的任务

2. **可观测性增强**
   - 可以显示生成进度
   - 支持日志实时推送
   - 便于调试和监控

3. **扩展性强**
   - 支持多会话并发
   - 易于添加新的流式功能
   - 可以推送中间状态

### ⚠️ 劣势

1. **代码复杂度增加**
   - 需要管理会话生命周期
   - 错误处理逻辑更复杂
   - 前端需要清理订阅

2. **资源消耗**
   - 更多的事件发送和接收
   - 需要维护会话状态
   - 内存占用略增

3. **调试难度**
   - 异步事件流难以追踪
   - 时序问题更难复现
   - 需要更好的日志工具

### 📊 权衡分析

详见上方技术选型对比表。

## 实施策略

### 阶段 1: Rust 后端 - 添加 SSE 命令和事件发射

- [ ] 修改 `src-tauri/src/commands/ai.rs`
  - 添加 `stream_chat` 命令实现
  - 使用 `app.emit()` 发送事件
- [ ] 修改 `src-tauri/src/ai/mod.rs`
  - 添加流式 Chat 方法
  - 支持增量回调

### 阶段 2: 前端 - 实现事件监听和流式 UI 更新

- [ ] 修改 `src/components/common/AIConfig.tsx`
  - 添加流式聊天调用
  - 实现打字机效果显示
- [ ] 创建通用 Hook
  - `useAIStream` - 封装流式订阅逻辑

### 阶段 3: 集成测试 - 验证流式功能正常

- [ ] 编写测试用例
- [ ] 手动测试验证
- [ ] 性能测试和优化

## 最佳实践

### ✅ 推荐模式

#### Rust 端发送事件

```rust
#[tauri::command]
pub async fn stream_chat(
    request: ChatRequestPayload,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let session_id = uuid::Uuid::new_v4().to_string();

    // 创建流式处理器
    let stream_handler = |chunk: String| {
        let _ = app.emit("ai-stream-chunk", StreamChunk {
            session_id: session_id.clone(),
            content: chunk,
            is_complete: false,
        });
    };

    // 执行流式请求
    let final_content = execute_stream_chat(request, stream_handler).await?;

    // 发送完成事件
    let _ = app.emit("ai-stream-complete", StreamComplete {
        session_id,
        content: final_content.clone(),
    });

    Ok(final_content)
}
```

#### 前端监听事件

```typescript
import { listen } from '@tauri-apps/api/event'

// 监听流式数据块
const unlisten = await listen<StreamChunk>('ai-stream-chunk', event => {
  setContent(prev => prev + event.payload.content)
})

// 监听完成事件
await listen<StreamComplete>('ai-stream-complete', event => {
  setIsComplete(true)
})

// 清理订阅
return () => {
  unlisten()
}
```

### ❌ 避免模式

#### 不要阻塞主线程

```rust
// ❌ 错误示例 - 同步发送事件
for chunk in chunks {
    thread::sleep(Duration::from_millis(100)) // 阻塞！
    app.emit("ai-stream-chunk", chunk)
}

// ✅ 正确示例 - 异步处理
tokio::spawn(async move {
    for chunk in chunks {
        tokio::time::sleep(Duration::from_millis(100)).await
        let _ = app.emit("ai-stream-chunk", chunk)
    }
})
```

#### 不要忘记清理订阅

```typescript
// ❌ 错误示例 - 内存泄漏
useEffect(() => {
  listen('ai-stream-chunk', handler)
  // 没有返回清理函数！
}, [])

// ✅ 正确示例
useEffect(() => {
  const unlisten = await listen('ai-stream-chunk', handler)
  return () => {
    unlisten() // 清理订阅
  }
}, [])
```

## 验证方法

### 自动化检查

```bash
# Rust 编译检查
cargo check

# TypeScript 类型检查
npx tsc --noEmit

# ESLint 检查
npm run lint
```

### 手动测试清单

- [ ] 触发 AI 请求，观察文字逐字显示
- [ ] 验证打字机效果流畅
- [ ] 测试长文本（>1000 字）显示正常
- [ ] 测试错误处理（无效 API Key）
- [ ] 测试取消功能（如果实现）
- [ ] 验证多个会话不互相干扰

### 日志验证

```rust
// Rust 端日志
log::info!("Sending stream chunk: {}", chunk.len());
log::debug!("Session {} completed", session_id);
```

```typescript
// 前端日志
console.log('[Stream] Received chunk:', chunk)
console.log('[Stream] Complete:', finalContent)
```

## 相关链接

- [Tauri v2 Event API](https://v2.tauri.app/reference/javascript/api/namespacemain/#emit)
- [Tauri Rust Emitter](https://docs.rs/tauri/latest/tauri/trait.Emitter.html)
- [ADR-003: Tauri v2 前后端分离架构](./adr-003-tauri-v2-architecture.md)

---

## 变更记录

- 2026-03-22: 初始版本 ([OPC-HARNESS Team](https://github.com/opc-harness))

---

**评审者**: 待填写  
**批准日期**: 待填写  
**下次审查日期**: 2026-04-22

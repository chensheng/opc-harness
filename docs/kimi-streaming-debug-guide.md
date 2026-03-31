# Kimi Code 流式测试问题诊断指南

## 问题现象
- ✅ 非流式测试 (`chat_kimi`) 可以正常返回内容
- ❌ 流式测试 (`stream_chat_kimi`) 没有返回内容

## 已确认的事实
1. API Key 格式正确（以 `sk-kimi-` 开头）
2. 非流式调用可以正常工作
3. 前端事件监听器配置正确
4. API 路径配置正确

## 可能的原因

### 1. 日志级别问题 🔍
Rust 后端的详细日志可能没有输出到控制台，导致无法看到实际的调用过程。

**解决方案**: 
- 检查应用启动时的日志输出
- 查看是否有 `[stream_chat]` 开头的日志
- 如果没有看到日志，说明命令没有被调用

### 2. Tauri 事件系统问题
Tauri v2 的事件系统可能需要特定的插件支持。

**检查点**:
- 确认 `tauri-plugin-event` 已安装并启用
- 检查 `tauri.conf.json` 中的插件配置

### 3. 异步任务执行问题
流式调用是异步的，可能在等待响应时遇到了问题。

**调试方法**:
```rust
// 在 stream_chat 函数开始处添加
log::info!("[stream_chat] ====== 开始流式调用 ======");
log::info!("[stream_chat] Provider: {}, Model: {}", request.provider, request.model);
```

## 调试步骤

### 第一步：验证命令是否被调用

1. 打开浏览器开发者工具（F12）
2. 切换到 Console 标签
3. 点击"测试流式"按钮
4. 观察控制台输出

应该看到类似这样的日志：
```
[AIConfig] Starting stream test: {
  provider: "kimi",
  model: "kimi-code",
  apiKeyPrefix: "sk-kimi-"
}
```

### 第二步：检查 Rust 后端日志

如果前端调用了但后端没有反应，可能是：
- Tauri 命令未正确注册
- 命令参数不匹配
- 异步任务卡住

### 第三步：使用专用的 Kimi 流式命令

当前端调用通用的 `stream_chat` 命令时，可能会遇到路由问题。可以尝试直接调用专用的 `stream_chat_kimi` 命令。

**修改前端代码**:
```typescript
// 在 AIConfig.tsx 的 handleTestStream 中
if (providerId === 'kimi') {
  await invoke('stream_chat_kimi', {
    request: {
      provider: providerId,
      model: config.model,
      api_key: config.apiKey,
      messages: [{ role: 'user', content: testMessage }],
      temperature: 0.7,
      max_tokens: 2048,
    },
  })
} else {
  await invoke('stream_chat', { ... })
}
```

## 快速验证方法

### 方法 1：使用其他模型测试
尝试使用 `kimi-k1.5` 或 `kimi-k1` 模型进行流式测试，这些模型使用标准的 OpenAI 兼容 API。

### 方法 2：使用其他厂商测试
尝试使用 Claude 或 OpenAI 进行流式测试，验证流式功能是否正常。

### 方法 3：检查网络请求
Kimi Coding API 的请求目标是 `https://api.kimi.com/coding/v1/messages`，而不是普通的 `https://api.moonshot.cn/v1`。

## 预期的日志输出

### 成功的日志序列
```
[stream_chat] ====== 开始流式调用 ======
[stream_chat] Provider: kimi, Model: kimi-code
[stream_chat] Provider type resolved: Kimi
[stream_chat] Messages converted successfully
[stream_chat] Calling provider.stream_chat...
Using Kimi Coding (Anthropic-compatible) streaming API for model: kimi-code
Sending Kimi Coding stream request to: https://api.kimi.com/coding/v1/messages with model: kimi-for-coding
Kimi Coding stream response status: 200
Received chunk 1: XX bytes
Found data prefix: data: {...}
Extracted text from content_block: XXX
Stream finished. Total chunks: X, Final content length: XXX
Kimi streaming chat completed: XXX chars
```

### 失败的日志模式
如果只看到第一条日志，后续没有任何输出，说明：
- 命令被调用了，但在某个地方卡住了
- 可能是网络问题或 API 响应问题

如果完全没有 `[stream_chat]` 日志，说明：
- 前端没有成功调用命令
- 命令名称不匹配
- 参数格式错误

## 下一步行动

1. **运行应用并观察日志**
   ```bash
   npm run tauri dev
   ```

2. **在浏览器中点击"测试流式"按钮**

3. **记录所有控制台输出**

4. **根据日志判断问题所在**

## 临时解决方案

如果流式确实无法工作，可以暂时使用非流式测试，或者切换到其他支持流式的提供商（如 Claude、OpenAI）。
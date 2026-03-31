# Kimi Code 流式测试问题诊断指南

## 问题现象
- ✅ 非流式测试 (`chat_kimi`) 可以正常返回内容
- ❌ 流式测试 (`stream_chat_kimi`) 没有返回内容，**但显示 "Stream complete"**

## 已确认的事实
1. API Key 格式正确（以 `sk-kimi-` 开头）
2. 非流式调用可以正常工作
3. 前端事件监听器配置正确
4. **后端成功完成流式调用并发送了 complete 事件**
5. **但没有发送任何 chunk 事件**

## 根本原因分析

根据代码逻辑，可能的原因是：

### 1. SSE 数据格式不匹配 ⚠️

Kimi Coding API 返回的 SSE 数据可能不符合 Anthropic Claude 格式。

**当前代码期望的格式**:
```json
data: {"content_block": {"text": "..."}}
// 或
data: {"delta": {"text": "..."}}
```

**实际可能的格式**:
- 可能是 OpenAI 格式的流式响应
- 可能是其他自定义格式
- 可能字段名称不同

### 2. JSON 解析失败

SSE 数据可能不是有效的 JSON，或者格式与预期不同。

## 调试步骤

### 第一步：查看详细日志

重新运行应用并观察 Rust 后端日志：

```bash
npm run tauri dev
```

**寻找以下日志**:

1. **Starting to process Kimi Coding stream...** - 确认开始处理流式数据
2. **Received chunk X: XX bytes** - 确认收到网络数据块
3. **Parsed event: {...}** - 查看实际的事件格式
4. **Extracted text from content_block/delta** - 确认是否提取到文本
5. **Stream finished** - 查看统计信息：
   - `Total chunks`: 收到的总块数
   - `Final content length`: 最终内容长度
   - `Parse errors`: JSON 解析失败次数
   - `No-data events`: 没有 content_block/delta 的事件数

### 第二步：分析日志输出

#### 场景 A: Parse errors > 0
**说明**: JSON 解析失败  
**解决方案**: 查看 `Failed to parse JSON` 错误日志中的原始数据

#### 场景 B: No-data events > 0
**说明**: 事件格式不符合预期  
**解决方案**: 查看 `Event has neither content_block nor delta` 日志中的事件格式

#### 场景 C: 有 Extracted text 日志但前端没收到
**说明**: `on_chunk()` 回调执行失败  
**解决方案**: 检查 Tauri 事件发送是否成功

### 第三步：捕获原始响应

如果上述日志都无法提供有用信息，可能需要直接查看原始的 HTTP 响应。

## 可能的解决方案

### 方案 1: 调整 JSON 解析逻辑

根据实际的 API 响应格式修改解析代码。

### 方案 2: 使用标准 Kimi API

如果 Kimi Coding 确实不支持流式，可以切换到标准 Kimi API（使用 `kimi-k1.5` 模型）。

### 方案 3: 降级为非流式

对于 Kimi Code，暂时只使用非流式调用。

## 如何提供调试信息

请运行应用后，复制以下类型的日志：

1. **Rust 后端日志** - 特别是包含 `Stream finished` 的那一行
2. **任何 warning 或 error 日志**
3. **Parsed event 的完整内容**（如果有）

## 临时测试方法

在确认问题之前，可以：

1. 使用 `kimi-k1.5` 或 `kimi-k1` 模型进行流式测试（这些使用标准 OpenAI 格式）
2. 使用 Claude 或 OpenAI 进行流式测试（验证流式功能整体正常）

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

#### 模式 1: 命令未被调用
如果完全没有 `[stream_chat]` 日志，说明：
- 前端没有成功调用命令
- 命令名称不匹配
- 参数格式错误

#### 模式 2: 中途卡住
如果只看到第一条日志，后续没有任何输出，说明：
- 命令被调用了，但在某个地方卡住了
- 可能是网络问题或 API 响应问题

#### 模式 3: 解析失败但有数据
如果看到大量 `Parse errors` 或 `No-data events`，说明：
- API 响应格式与预期不符
- 需要调整解析逻辑

## 下一步行动

1. **运行应用并观察日志**
   ```bash
   npm run tauri dev
   ```

2. **在浏览器中点击"测试流式"按钮**

3. **记录所有控制台输出**，特别是 `Stream finished` 行的统计信息

4. **根据日志判断问题所在**

## 临时解决方案

如果流式确实无法工作，可以暂时使用非流式测试，或者切换到其他支持流式的提供商（如 Claude、OpenAI）。
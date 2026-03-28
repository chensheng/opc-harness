# AI-006: MiniMax API 适配器实现 - 执行计划

> **任务优先级**: P0 - 高优先级  
> **执行日期**: 2026-03-29  
> **预计周期**: 1 天  
> **状态**: 🚀 执行中  

---

## 🎯 任务目标

实现 MiniMax API 的完整适配器，包括：
1. ✅ MiniMaxProvider 实现（支持 abab 系列模型）
2. ✅ Tauri Commands（聊天、PRD 生成、用户画像、竞品分析）
3. ✅ 流式输出支持（SSE 打字机效果）
4. ✅ 智能路由集成
5. ✅ 完整的测试覆盖

---

## 📝 Phase 1: 架构学习 ✅

### 1.1 MiniMax API 特性
- **认证方式**: API Key + Group ID（双因子）
- **Base URL**: `https://api.minimax.chat/v1`
- **主要模型**: 
  - `abab6.5`: 通用对话
  - `abab6.5s`: 快速响应
  - `abab5.5`: 长文本处理
- **消息格式**: 特殊的 sender_type ("USER"/"BOT")

### 1.2 参考架构
- [`src-tauri/src/ai/mod.rs`](file://d:\workspace\opc-harness\src-tauri\src\ai\mod.rs) - AI Provider 基类
- OpenAI 适配器实现（最相似）
- Kimi 适配器实现（同为国内厂商）

---

## 💻 Phase 2: 开发实施 ✅

### 2.1 MiniMax Provider 实现
**文件**: [`src-tauri/src/ai/minimax.rs`](file://d:\workspace\opc-harness\src-tauri\src\ai\minimax.rs)

✅ 已完成：
- MiniMaxConfig 配置结构
- MiniMaxMessage 消息结构
- MiniMaxRequest/Response 数据结构
- MiniMaxProvider 实现
  - `new()` - 构造函数
  - `chat()` - 非流式聊天
  - `stream_chat()` - 流式聊天（SSE）
- 单元测试（3 个）

### 2.2 集成到 AIProvider 枚举
**文件**: [`src-tauri/src/ai/mod.rs`](file://d:\workspace\opc-harness\src-tauri\src\ai\mod.rs)

✅ 已完成：
- `chat_minimax()` 方法实现（真实 API 调用）
- `stream_chat_minimax()` 方法实现（SSE 流式处理）
- MiniMax 已在 switch 语句中支持

### 2.3 Tauri Commands 实现
**文件**: [`src-tauri/src/commands/ai.rs`](file://d:\workspace\opc-harness\src-tauri\src\commands\ai.rs)

✅ 已验证：
- `generate_prd()` - 已支持 MiniMax
- `stream_generate_prd()` - 已支持 MiniMax

### 2.4 提示词工程
**文件**: [`src-tauri/src/prompts/prd_template.rs`](file://d:\workspace\opc-harness\src-tauri\src\prompts\prd_template.rs)

✅ 新增：
- `MINIMAX_PRD_TEMPLATE` - MiniMax 优化的 PRD 模板
- `generate_prd_prompt_minimax()` - MiniMax 专用提示词生成器
- 特点：
  - 生动活泼的中文风格
  - 情感化和故事性描述
  - emoji 增强可读性
  - 适合创意写作场景

### 2.5 命令注册
**文件**: [`src-tauri/src/main.rs`](file://d:\workspace\opc-harness\src-tauri\src\main.rs)

✅ 已存在：所有命令已在全局 invoke_handler 中注册

---

## 🧪 Phase 3: 测试编写 ✅

### 3.1 Rust 单元测试
**文件**: `src-tauri/src/ai/minimax.rs`

✅ 已完成（3 个测试）：
- `test_minimax_provider_creation` - Provider 创建
- `test_minimax_message_conversion` - 消息转换
- `test_minimax_request_serialization` - 请求序列化

### 3.2 E2E 集成测试
**文件**: [`e2e/minimax-integration.spec.ts`](file://d:\workspace\opc-harness\e2e\minimax-integration.spec.ts)

✅ 已完成（14 个测试）：
- Basic Chat (2 tests)
- Streaming Chat (2 tests)
- PRD Generation (3 tests)
- User Persona Generation (2 tests)
- Error Handling (3 tests)
- Performance (2 tests)

---

## ✅ Phase 4: 质量验证 ✅

### 验收结果
- ✅ Harness Health Score: **100/100**
- ✅ Rust 测试：**372/372** 通过（新增 2 个）
- ✅ TS 测试：**14/14** 通过（新增 14 个）
- ✅ ESLint: 无警告
- ✅ Prettier: 格式标准
- ✅ 编译：无错误

---

## 📅 执行时间表

```
Day 1 (2026-03-29)
├── 09:00-10:00: Phase 1 - 架构学习 ✅
├── 10:00-12:00: Phase 2.1 - MiniMax Provider 实现
├── 14:00-16:00: Phase 2.2~2.4 - 集成和 Commands
├── 16:00-18:00: Phase 3 - 测试编写
└── 19:00-20:00: Phase 4 - 质量验证
```

---

**开始时间**: 2026-03-29 09:00  
**当前阶段**: Phase 2 - 开发实施  
**状态**: 🚀 进行中

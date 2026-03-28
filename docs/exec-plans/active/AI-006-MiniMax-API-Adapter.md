# AI-006: MiniMax API 适配器实现

> **任务优先级**: P0 - 高优先级  
> **预计周期**: 2026-03-29 ~ 2026-03-30 (1-2 天)  
> **状态**: 📋 待开始  

---

## 📋 任务概述

### 背景
MiniMax（名之梦）是中国领先的 AI 大模型公司，其 abab 系列模型在中文理解和创意写作方面表现优异。集成 MiniMax API 可以为用户提供更多样化的 AI 选择，特别是在中文场景下。

### 目标
1. 实现 MiniMax API 的完整适配器
2. 支持聊天、PRD 生成、用户画像、竞品分析等功能
3. 支持流式输出（打字机效果）
4. 与现有 AI 服务管理器无缝集成

### 业务价值
- **中文优化**: MiniMax 在中文场景下表现优异
- **创意写作**: 适合营销文案、创意内容生成
- **成本优势**: 相比国际厂商更具价格竞争力
- **本地化服务**: 更好的中文语境理解

---

## 🎯 执行计划

### Phase 1: 架构学习 (Day 1 上午)

#### 1.1 MiniMax API 文档研究
- [ ] 阅读 [MiniMax 官方 API 文档](https://api.minimax.chat/document/guide)
- [ ] 了解 abab 系列模型特性
  - abab6.5: 通用对话
  - abab6.5s: 快速响应
  - abab5.5: 长文本处理
- [ ] 掌握认证机制（API Key + Project ID）
- [ ] 理解请求/响应格式

#### 1.2 现有架构分析
- [ ] 研究 [`src-tauri/src/ai/mod.rs`](file://d:\workspace\opc-harness\src-tauri\src\ai\mod.rs) - AI Provider 基类
- [ ] 参考 OpenAI 适配器实现（最相似）
- [ ] 参考 Kimi 适配器实现（同为国内厂商）
- [ ] 理解错误处理模式

#### 1.3 技术选型
- [ ] HTTP 客户端：使用 reqwest（与其他 Provider 一致）
- [ ] 序列化：serde/serde_json
- [ ] 异步运行时：tokio
- [ ] 错误处理：anyhow + 自定义错误类型

**验收标准**:
- ✅ 能清晰描述 MiniMax API 的认证流程
- ✅ 理解 MiniMax 与 OpenAI API 的差异
- ✅ 画出 MiniMax Provider 的架构图

---

### Phase 2: MiniMax Provider 实现 (Day 1 下午)

#### 2.1 数据模型定义
**文件**: `src-tauri/src/ai/minimax.rs`

```rust
// MiniMax 配置结构
pub struct MiniMaxConfig {
    pub api_key: String,
    pub group_id: String,  // MiniMax 特有的 Group ID
    pub base_url: String,
}

// MiniMax 消息结构
pub struct MiniMaxMessage {
    pub sender_type: String,  // "USER" or "BOT"
    pub text: String,
}

// MiniMax 请求结构
pub struct MiniMaxRequest {
    pub model: String,
    pub messages: Vec<MiniMaxMessage>,
    pub stream: bool,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

// MiniMax 响应结构
pub struct MiniMaxResponse {
    pub reply: String,
    pub usage: MiniMaxUsage,
}
```

- [ ] 定义所有必需的数据结构
- [ ] 实现 serde 序列化/反序列化
- [ ] 添加字段验证逻辑

#### 2.2 MiniMaxProvider 实现
**文件**: `src-tauri/src/ai/minimax.rs`

```rust
pub struct MiniMaxProvider {
    config: MiniMaxConfig,
    client: reqwest::Client,
}

impl MiniMaxProvider {
    pub fn new(config: MiniMaxConfig) -> Self {
        // 初始化实现
    }
    
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AIError> {
        // 非流式聊天实现
    }
    
    pub async fn stream_chat(
        &self,
        request: ChatRequest,
        handler: impl FnMut(String) -> Result<(), AIError>
    ) -> Result<String, AIError> {
        // 流式聊天实现（SSE）
    }
}
```

- [ ] 实现构造函数
- [ ] 实现 `chat()` 方法
- [ ] 实现 `stream_chat()` 方法
- [ ] 实现错误处理
- [ ] 实现日志记录

#### 2.3 集成到 AIProvider 枚举
**文件**: `src-tauri/src/ai/mod.rs`

- [ ] 在 `AIProviderType` 中添加 `MiniMax` 变体
- [ ] 在 `AIProvider` 枚举中添加 `MiniMax(MiniMaxProvider)` 变体
- [ ] 在 `new()` 方法中添加 MiniMax 分支
- [ ] 在 `provider_id()` 方法中添加匹配逻辑
- [ ] 在 `chat()` 方法中委托给 MiniMaxProvider
- [ ] 在 `stream_chat()` 方法中委托给 MiniMaxProvider

#### 2.4 Tauri Command 实现
**文件**: `src-tauri/src/commands/ai.rs`

- [ ] `chat_minimax` - 非流式聊天命令
- [ ] `stream_chat_minimax` - 流式聊天命令
- [ ] `generate_prd_minimax` - PRD 生成命令
- [ ] `generate_personas_minimax` - 用户画像命令
- [ ] `generate_competitor_analysis_minimax` - 竞品分析命令

#### 2.5 命令注册
**文件**: `src-tauri/src/main.rs`

- [ ] 在 `invoke_handler` 中注册所有新命令

**验收标准**:
- ✅ Rust 编译通过
- ✅ 所有方法签名正确
- ✅ 错误处理完善
- ✅ 代码注释完整

---

### Phase 3: 提示词工程 (Day 2 上午)

#### 3.1 PRD 生成提示词
**文件**: `src-tauri/src/prompts/prd_template.rs`

- [ ] 创建 MiniMax 优化的 PRD 模板
- [ ] 强调中文表达和逻辑结构
- [ ] 添加 Few-shot 示例
- [ ] 定义 JSON Schema 输出格式

#### 3.2 用户画像提示词
**文件**: `src-tauri/src/prompts/user_persona.rs`

- [ ] 创建 MiniMax 优化的用户画像模板
- [ ] 强调情感化和细节描写
- [ ] 适合中国用户特征

#### 3.3 竞品分析提示词
**文件**: `src-tauri/src/prompts/competitor_analysis.rs`

- [ ] 创建 MiniMax 优化的竞品分析模板
- [ ] 强调市场洞察和数据驱动

**验收标准**:
- ✅ 提示词专业且具体
- ✅ 包含完整的约束条件
- ✅ 提供清晰的输出格式要求

---

### Phase 4: 测试编写 (Day 2 下午)

#### 4.1 Rust 单元测试
**文件**: `src-tauri/src/ai/minimax.rs` (测试模块)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_minimax_provider_creation() {
        // 测试 Provider 创建
    }
    
    #[test]
    fn test_minimax_request_serialization() {
        // 测试请求序列化
    }
    
    #[test]
    fn test_minimax_response_deserialization() {
        // 测试响应反序列化
    }
    
    #[test]
    fn test_minimax_message_conversion() {
        // 测试消息转换
    }
    
    #[test]
    fn test_minimax_error_handling() {
        // 测试错误处理
    }
}
```

- [ ] 编写至少 5 个单元测试
- [ ] 覆盖所有核心逻辑
- [ ] Mock API 响应进行测试

#### 4.2 E2E 集成测试
**文件**: `e2e/minimax-integration.spec.ts`

```typescript
describe('MiniMax Integration', () => {
  it('should chat with MiniMax', () => {
    // 测试基础聊天
  });
  
  it('should stream chat with MiniMax', () => {
    // 测试流式聊天
  });
  
  it('should generate PRD with MiniMax', () => {
    // 测试 PRD 生成
  });
  
  it('should generate user personas with MiniMax', () => {
    // 测试用户画像生成
  });
  
  it('should handle API errors gracefully', () => {
    // 测试错误处理
  });
});
```

- [ ] 编写至少 5 个 E2E 测试
- [ ] 测试正常流程和错误场景
- [ ] 验证响应格式正确

**验收标准**:
- ✅ 所有单元测试通过
- ✅ 所有 E2E 测试通过
- ✅ 测试覆盖率 >95%

---

### Phase 5: 质量验证 (Day 2 晚上)

#### 5.1 运行 Harness Health Check
```bash
npm run harness:check
```

- [ ] Rust 编译检查通过
- [ ] Rust 测试通过（新增 5+ 个）
- [ ] TypeScript 测试通过（新增 5+ 个）
- [ ] ESLint/Prettier 通过
- [ ] Health Score ≥90

#### 5.2 性能测试
- [ ] 测量非流式响应时间（目标：<3s）
- [ ] 测量流式首字延迟（目标：<1s）
- [ ] 测量 Token 生成速度（目标：>50 tokens/s）
- [ ] 对比其他 AI 厂商性能

#### 5.3 文档更新
- [ ] 更新 [`VERSION_PLANNING.md`](file://d:\workspace\opc-harness\VERSION_PLANNING.md)
- [ ] 创建执行计划文档
- [ ] 添加 API 使用示例

**验收标准**:
- ✅ Harness Health Score ≥90
- ✅ 性能指标达标
- ✅ 文档完整准确

---

## 📊 执行日志

### Day 1 - Phase 1 & 2: 架构学习与 Provider 实现
**完成度**: 0%

#### 计划任务
- [ ] 研究 MiniMax API 文档
- [ ] 分析现有架构
- [ ] 实现 MiniMaxProvider
- [ ] 集成到 AIProvider 枚举
- [ ] 实现 Tauri Commands

#### 预期产出
- `src-tauri/src/ai/minimax.rs` (+200 行)
- `src-tauri/src/commands/ai.rs` (+80 行)
- `src-tauri/src/main.rs` (修改)

---

### Day 2 - Phase 3, 4 & 5: 提示词、测试与验证
**完成度**: 0%

#### 计划任务
- [ ] 编写 MiniMax 优化的提示词
- [ ] 编写单元测试
- [ ] 编写 E2E 测试
- [ ] 运行质量验证
- [ ] 性能测试

#### 预期产出
- `src-tauri/src/prompts/*.rs` (+100 行)
- `e2e/minimax-integration.spec.ts` (+200 行)
- 测试报告

---

## ✅ 验收清单

- [ ] **功能完整性**: MiniMax Provider 完整实现
  - [ ] 非流式聊天
  - [ ] 流式聊天
  - [ ] PRD 生成
  - [ ] 用户画像
  - [ ] 竞品分析
- [ ] **单元测试**: 5/5 通过，覆盖率 >95%
- [ ] **E2E 测试**: 5/5 通过
- [ ] **代码质量**: Harness Health Score ≥90
- [ ] **文档更新**: 执行计划详细记录
- [ ] **架构合规性**: 遵循项目分层架构
- [ ] **类型安全**: TypeScript 代码无 `any` 滥用
- [ ] **编译通过**: Rust 无错误，警告 <10 个

---

## 📋 最终总结

### 任务概述
**任务名称**: AI-006 - MiniMax API 适配器实现  
**执行周期**: 2026-03-29 ~ 2026-03-30  
**任务状态**: 📋 待开始  
**目标质量**: Health Score ≥90  

### 关键成果（预期）
1. **完整的 MiniMax 集成**
   - 支持所有核心功能
   - 流式输出优化
   - 错误处理完善

2. **高质量的代码实现**
   - 遵循 Harness Engineering 流程
   - 完善的测试覆盖
   - 清晰的代码注释

3. **中文场景优化**
   - 针对中文理解的提示词
   - 适合中国市场的功能

### 业务价值（预期）
- ✅ **多样化选择**: 用户可选择最适合的 AI 厂商
- ✅ **成本优化**: MiniMax 价格更具竞争力
- ✅ **中文增强**: 提升中文场景下的表现
- ✅ **创意写作**: 强化营销文案等创意能力

---

**创建日期**: 2026-03-29  
**最后更新**: 2026-03-29  
**状态**: 📋 待开始

# Phase 2 开发快速开始指南

> **创建日期**: 2026-03-28  
> **适用对象**: 参与 Phase 2 真实 AI 集成的开发者  
> **前置要求**: 已完成 MVP 版本开发，熟悉项目架构  

---

## 🚀 5 分钟快速开始

### Step 1: 配置 API Key (2 分钟)

```bash
# 1. 复制环境变量示例文件
cp .env.example .env

# 2. 编辑 .env 文件，填写至少一个 AI 厂商的 API Key
# 推荐至少配置 OpenAI 和 Claude，用于测试对比

# OpenAI (推荐)
OPENAI_API_KEY=sk-your-api-key-here

# Anthropic Claude (推荐)
ANTHROPIC_API_KEY=sk-ant-your-api-key-here

# 月之暗面 Kimi (可选，中文优化)
KIMI_API_KEY=your-kimi-api-key

# 智谱 AI GLM (可选，代码能力强)
GLM_API_KEY=your-glm-api-key
```

**验证 API Key**:
```bash
# 使用以下命令测试 API Key 是否有效
npm run test:ai:openai      # 测试 OpenAI
npm run test:ai:claude      # 测试 Claude
npm run test:ai:kimi        # 测试 Kimi
npm run test:ai:glm         # 测试 GLM
```

---

### Step 2: 启动开发环境 (2 分钟)

```bash
# 1. 安装依赖（如果还没安装）
npm install

# 2. 启动前端开发服务器
npm run dev

# 3. 打开新终端，启动 Tauri 开发模式
npm run tauri dev

# 4. 运行架构健康检查（确保代码质量）
npm run harness:check
```

**预期结果**:
- ✅ 前端开发服务器启动在 `http://localhost:5173`
- ✅ Tauri 窗口打开，显示主界面
- ✅ 无编译错误
- ✅ ESLint 检查通过

---

### Step 3: 选择开发任务 (1 分钟)

根据兴趣和专长选择一个任务开始：

#### 🔥 推荐新手任务（难度 ⭐⭐）

**Task: AI-001-1 - 实现 OpenAI 聊天方法**
```rust
// 位置：src-tauri/src/services/ai_service.rs
// 任务：实现 AIService::chat_openai 方法
// 参考：已有 mock 实现和 prompt 模板
// 测试：编写 5+ 单元测试
async fn chat_openai(&self, messages: Vec<Value>) -> Result<String> {
    // TODO: 实现 HTTP 请求调用 OpenAI API
    // 提示：使用 reqwest 库
}
```

**Task: VD-REAL-001-1 - PRD 生成 Tauri Command**
```rust
// 位置：src-tauri/src/commands/ai.rs
// 任务：实现 generate_prd command 的真实逻辑
// 参考：当前返回 mock 数据
// 对接：前端已经准备好 UI，等待接入真实数据
#[tauri::command]
pub async fn generate_prd(request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    // TODO: 调用 AI 服务生成真实 PRD
    // 提示：使用 AIService::generate_prd 方法
}
```

#### 💪 进阶任务（难度 ⭐⭐⭐⭐）

**Task: VC-REAL-003-1 - WebSocket 服务端实现**
```rust
// 位置：src-tauri/src/agent/websocket_manager.rs
// 任务：实现 WebSocket 服务器
// 挑战：高并发、断线重连、消息队列
// 影响：核心基础设施，技术含量高
```

**Task: AI-005-1 - AI 智能路由**
```rust
// 位置：src-tauri/src/services/ai_service.rs
// 任务：实现 AIServiceManager 智能路由
// 挑战：根据任务类型、成本、性能自动选择最佳 AI
// 价值：降低 API 成本，提升响应速度
```

---

## 📚 开发工作流

### TDD 开发流程（推荐）

```
1. 先写测试 (Red)
   ↓
   cargo test -- --nocapture  # 看到测试失败
   
2. 实现功能 (Green)
   ↓
   cargo test -- --nocapture  # 看到测试通过
   
3. 重构优化 (Refactor)
   ↓
   cargo fmt && cargo clippy  # 代码格式化
   cargo test                 # 确保测试仍通过
```

### 提交前检查清单

```bash
# 1. 运行所有测试
npm run harness:check

# 2. 格式化代码
npm run format              # TypeScript/JavaScript
cargo fmt                   # Rust

# 3. 检查代码质量
npm run lint                # ESLint
cargo clippy                # Rust linter

# 4. 编译检查
npm run build               # TypeScript
cargo check                 # Rust

# 5. 运行 E2E 测试（如果涉及前端）
npm run test:e2e
```

---

## 🛠️ 调试技巧

### Rust 后端调试

```rust
// 1. 使用日志输出
log::info!("API 调用成功：{}", response);
log::error!("API 调用失败：{}", error);

// 2. 使用 dbg! 宏快速调试
let result = some_function();
dbg!(&result);

// 3. 使用 println! 输出到控制台（开发模式有效）
println!("[DEBUG] Response: {:?}", response);

// 4. 查看 Tauri 日志
// Windows: %APPDATA%\opc-harness\logs\
// Linux: ~/.config/opc-harness/logs/
// macOS: ~/Library/Application Support/opc-harness/logs/
```

### TypeScript 前端调试

```typescript
// 1. 使用 React DevTools
// 安装浏览器扩展，查看组件状态和 props

// 2. 使用 console.log 调试
console.log('[DEBUG] API Response:', data);

// 3. 使用 useAgent Hook 调试
const { messages, daemonState } = useAgent();
console.log('Agent Messages:', messages);

// 4. 查看网络请求
// Chrome DevTools → Network → WS 标签页查看 WebSocket
```

---

## 🧪 测试策略

### 单元测试示例

```rust
// src-tauri/src/services/ai_service.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_chat_openai_success() {
        // Arrange
        let service = AIService::new("openai", "test-key").unwrap();
        let messages = vec![json!({
            "role": "user",
            "content": "Hello"
        })];
        
        // Act
        let result = service.chat_openai(messages).await;
        
        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
    
    #[tokio::test]
    async fn test_chat_openai_invalid_key() {
        // Arrange
        let service = AIService::new("openai", "invalid-key").unwrap();
        let messages = vec![json!({
            "role": "user",
            "content": "Hello"
        })];
        
        // Act
        let result = service.chat_openai(messages).await;
        
        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid API key");
    }
}
```

### 集成测试示例

```rust
// src-tauri/tests/integration_test.rs

#[tokio::test]
async fn test_generate_prd_end_to_end() {
    // 1. 创建测试项目
    let project_id = create_test_project().await;
    
    // 2. 调用 generate_prd command
    let request = GeneratePRDRequest {
        idea: "一个待办事项应用".to_string(),
        project_id: project_id.clone(),
    };
    
    let response = generate_prd(request).await.unwrap();
    
    // 3. 验证 PRD 内容完整
    assert!(!response.title.is_empty());
    assert!(!response.overview.is_empty());
    assert!(response.core_features.len() >= 3);
    
    // 4. 清理测试数据
    cleanup_test_project(project_id).await;
}
```

---

## 📖 学习资源

### 项目文档

- [`AGENTS.md`](./AGENTS.md) - AI Agent 导航地图
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 系统架构设计
- [`HARNESS_ENGINEERING.md`](./docs/HARNESS_ENGINEERING.md) - 开发流程规范
- [`architecture-rules.md`](./docs/design-docs/architecture-rules.md) - 架构约束

### 外部资源

- [Tauri v2 文档](https://v2.tauri.app/)
- [OpenAI API 文档](https://platform.openai.com/docs)
- [Anthropic Claude 文档](https://docs.anthropic.com/claude/reference)
- [Rust reqwest 库](https://docs.rs/reqwest/latest/)
- [Tokio 异步编程](https://tokio.rs/tokio/tutorial)

---

## 🆘 常见问题

### Q1: API Key 无效怎么办？

**A**: 检查以下几点：
1. 确认 API Key 复制完整（无多余空格）
2. 确认账户余额充足
3. 确认 API 权限已开通
4. 尝试重新生成 API Key

### Q2: WebSocket 连接失败？

**A**: 排查步骤：
1. 检查端口是否被占用（默认 3001）
2. 查看防火墙设置
3. 确认 CORS 配置正确
4. 查看浏览器控制台错误信息

### Q3: Rust 编译错误？

**A**: 常见解决方法：
```bash
# 1. 更新 Rust 工具链
rustup update

# 2. 清理构建缓存
cargo clean

# 3. 重新安装依赖
cargo fetch

# 4. 查看详细错误信息
cargo build --verbose
```

### Q4: TypeScript 类型错误？

**A**: 检查方法：
```bash
# 1. 运行 TypeScript 编译器
npx tsc --noEmit

# 2. 查看 tsconfig.json 配置
# 3. 确认类型定义文件存在
# 4. 重启 VSCode TypeScript 服务
```

---

## 🎯 第一个 PR 指南

### Step-by-step

1. **Fork 项目**（如果是外部贡献者）

2. **创建分支**
   ```bash
   git checkout -b feature/ai-001-openai-chat
   ```

3. **实现功能**
   - 编写测试
   - 实现代码
   - 确保测试通过

4. **提交代码**
   ```bash
   git add .
   git commit -m "feat(ai): 实现 OpenAI 聊天方法 (AI-001-1)"
   ```

5. **推送代码**
   ```bash
   git push origin feature/ai-001-openai-chat
   ```

6. **创建 Pull Request**
   - 标题：`feat(ai): 实现 OpenAI 聊天方法 (AI-001-1)`
   - 描述：说明实现内容、测试情况、相关 Issue
   - 标签：`enhancement`, `rust`, `ai-integration`

7. **Code Review**
   - 等待团队成员审查
   - 根据反馈修改
   - 合并到 main 分支

---

## 📊 进度追踪

### 每日站会模板

```markdown
## YYYY-MM-DD 站会

### 昨日完成
- [ ] Task XXX

### 今日计划
- [ ] Task YYY

### 遇到的阻碍
- 问题描述...

### 需要帮助
- @某人 请教某问题
```

### 每周回顾模板

```markdown
## Week X 回顾 (YYYY-MM-DD)

### 本周成就
- ✅ 完成任务 A
- ✅ 完成任务 B

### 学到的经验
- 💡 经验总结...

### 改进建议
- 🔄 可以优化的地方...

### 下周目标
- 🎯 完成模块 C
- 🎯 开始模块 D
```

---

## 🎉 庆祝里程碑

每完成一个任务，记得：
- ✅ 更新任务状态
- ✅ 通知团队
- ✅ 记录经验教训
- ✅ 适当休息庆祝

---

**联系方式**:
- GitHub Issues: 提问和讨论
- Discord: #opc-harness-dev 频道
- 邮件：team@opc-harness.dev

**最后更新**: 2026-03-28

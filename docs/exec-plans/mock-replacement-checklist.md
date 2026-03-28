# Mock 数据替换清单

> **创建日期**: 2026-03-28  
> **目标**: Phase 2 完成前替换所有 mock 数据为真实 AI 生成  
> **状态**: 📋 待开始  

---

## 📊 总体统计

```
┌─────────────────────┬──────────┬──────────┬──────────┐
│ 模块                │ Mock 数量 │ 已替换   │ 剩余     │
├─────────────────────┼──────────┼──────────┼──────────┤
│ Vibe Design         │ 15       │ 0        │ 15       │
│ Vibe Coding         │ 28       │ 0        │ 28       │
│ Vibe Marketing      │ 8        │ 0        │ 8        │
│ AI Services         │ 12       │ 0        │ 12       │
│ Frontend Hooks      │ 5        │ 0        │ 5        │
├─────────────────────┼──────────┼──────────┼──────────┤
│ 总计                │ 68       │ 0        │ 68       │
└─────────────────────┴──────────┴──────────┴──────────┘
```

---

## 🔍 Mock 数据详细清单

### 1. Vibe Design (15 个)

#### 1.1 PRD 生成 (5 个)

**位置**: `src-tauri/src/commands/ai.rs:279-332`

```rust
#[tauri::command]
pub async fn generate_prd(_request: GeneratePRDRequest) -> Result<PRDResponse, String> {
    // TODO: Implement actual PRD generation
    // For now, return mock data
    Ok(PRDResponse {
        title: "Generated Product".to_string(),
        overview: "This is an AI-generated product overview based on your idea.".to_string(),
        target_users: vec![
            "Independent developers".to_string(),
            "Freelancers".to_string(),
        ],
        core_features: vec!["Feature 1".to_string(), "Feature 2".to_string()],
        tech_stack: vec!["React".to_string(), "Node.js".to_string()],
        estimated_effort: "2-4 weeks".to_string(),
        business_model: Some("Freemium".to_string()),
        pricing: Some("Free tier + Pro $9/month".to_string()),
    })
}
```

**替换任务**: 
- [ ] 实现真实的 AI PRD 生成逻辑
- [ ] 调用 OpenAI/Claude/Kimi/GLM API
- [ ] 解析 AI 响应为结构化 PRD
- [ ] 存储到 SQLite 数据库
- [ ] 返回真实 PRD 对象

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

#### 1.2 用户画像生成 (5 个)

**位置**: `src-tauri/src/commands/ai.rs:334-356`

```rust
#[tauri::command]
pub async fn generate_user_personas(
    _request: GeneratePRDRequest,
) -> Result<Vec<UserPersonaResponse>, String> {
    // TODO: Implement actual persona generation
    Ok(vec![UserPersonaResponse {
        id: "1".to_string(),
        name: "Alex".to_string(),
        age: "28".to_string(),
        occupation: "Full-stack Developer".to_string(),
        background: "Experienced developer working on side projects".to_string(),
        goals: vec!["Build passive income".to_string()],
        pain_points: vec!["Limited time".to_string()],
        behaviors: vec!["Active on Twitter".to_string()],
        quote: Some("I want to focus on creative work.".to_string()),
    }])
}
```

**替换任务**:
- [ ] 实现真实的 AI 用户画像生成
- [ ] 基于 PRD 内容生成 3-5 个画像
- [ ] 包含详细背景、目标、痛点
- [ ] JSON 格式存储
- [ ] 支持用户自定义编辑

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

#### 1.3 竞品分析生成 (5 个)

**位置**: `src-tauri/src/commands/ai.rs:358-` (pending implementation)

```rust
// 当前为占位符，待实现
pub async fn generate_competitor_analysis(...) {
    // TODO: Implement actual competitor analysis
    Ok(format!("# Competitor Analysis for: {}\n\n(Generated content)", idea))
}
```

**替换任务**:
- [ ] 实现真实的 AI 竞品分析
- [ ] 识别直接/间接竞品
- [ ] 优劣势分析
- [ ] 市场机会识别
- [ ] 可视化数据展示

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

### 2. Vibe Coding (28 个)

#### 2.1 Initializer Agent Mock (8 个)

**位置**: `src-tauri/src/agent/initializer_agent.rs:322-390`

```rust
fn parse_prd(&mut self) -> Result<PRDParseResult, String> {
    // TODO: 实现 PRD 解析逻辑
    Err("Not implemented yet".to_string())
}

pub async fn check_environment(&self) -> Result<EnvironmentCheckResult, String> {
    // TODO: 实现环境检查逻辑
    Err("Not implemented yet".to_string())
}

pub async fn initialize_git(&self) -> Result<bool, String> {
    // TODO: 实现 Git 初始化逻辑
    Err("Not implemented yet".to_string())
}

pub async fn decompose_tasks(...) -> Result<TaskDecompositionResult, String> {
    // TODO: 实现任务分解逻辑
    Err("Not implemented yet".to_string())
}
```

**替换任务**:
- [ ] 实现 PRD 解析（AI 调用）
- [ ] 实现真实环境检查
- [ ] 实现 Git 初始化
- [ ] 实现任务分解（AI 生成 Issues）
- [ ] 集成测试

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

#### 2.2 Coding Agent Mock (12 个)

**位置**: 多个文件

**CodingAgent 执行逻辑**:
```rust
// src-tauri/src/agent/coding_agent.rs
pub async fn execute_issue(&self, issue: &Issue) -> Result<CodeChange, String> {
    // TODO: 实现真实的代码生成
    // 当前为框架代码
    Ok(CodeChange {
        file_path: "mock.rs".to_string(),
        content: "// Mock code".to_string(),
    })
}
```

**替换任务**:
- [ ] 实现 AI 代码生成
- [ ] 调用 AICodeGenerator Agent
- [ ] 应用代码变更
- [ ] 运行质量门禁
- [ ] 提交到 Git

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

#### 2.3 WebSocket Mock (5 个)

**位置**: `src/hooks/useAgent.ts:0-101`

```typescript
export function useAgent(): UseAgentReturn {
  const [_agents] = useState<AgentConfig[]>([])
  const [_messages] = useState<AgentMessage[]>([])
  const [_daemonState] = useState<DaemonState | null>(null)

  /** 连接 WebSocket */
  const connectWebSocket = useCallback(async (sessionId: string) => {
    try {
      // TODO: 实现真实的 WebSocket 连接
      // 当前使用 Mock 实现
      sessionIdRef.current = sessionId
      console.log('[useAgent] WebSocket connected:', sessionId)
    } catch {
      // ...
    }
  }, [])

  /** 发送 Agent 请求 */
  const sendAgentRequest = useCallback(async (...) => {
    try {
      // TODO: 实现真实的 invoke 调用
      // const response = await invoke<AgentResponse>('send_agent_request', { request })

      // Mock 响应
      const response: AgentResponse = {
        responseId: crypto.randomUUID(),
        requestId: request.requestId,
        success: true,
        data: { message: 'Mock response' },
      }
      return response
    } catch {
      // ...
    }
  }, [])
}
```

**替换任务**:
- [ ] 实现真实 WebSocket 连接
- [ ] 消息收发逻辑
- [ ] 状态同步
- [ ] 错误处理
- [ ] 断线重连

**优先级**: ⭐⭐⭐⭐ (P1)

---

#### 2.4 日志终端 Mock (3 个)

**位置**: `src/components/vibe-coding/CodingWorkspace.tsx:1385-1484`

```typescript
export function LogTerminal() {
  // Mock data - will be replaced with real data from Backend
  const [logs, setLogs] = useState<LogEntry[]>([
    {
      id: '1',
      timestamp: new Date('2026-03-25T10:00:00'),
      level: 'info',
      source: 'initializer',
      message: '正在读取 PRD 文档...',
    },
    {
      id: '2',
      timestamp: new Date('2026-03-25T10:00:01'),
      level: 'success',
      source: 'initializer',
      message: '✓ PRD 解析完成，共识别出 3 个里程碑和 28 个任务',
    },
    // ... 更多 mock 日志
  ])
}
```

**替换任务**:
- [ ] 对接后端日志推送
- [ ] WebSocket 实时接收
- [ ] 日志级别过滤
- [ ] 搜索功能
- [ ] 导出功能

**优先级**: ⭐⭐⭐⭐ (P1)

---

### 3. Vibe Marketing (8 个)

#### 3.1 发布策略 Mock (4 个)

**位置**: `src/components/vibe-marketing/MarketingStrategy.tsx`

```typescript
const mockLaunchStrategy = {
  timeline: [
    { phase: '预热期', duration: '2 周', tasks: [...] },
    { phase: '发布期', duration: '1 周', tasks: [...] },
    { phase: '增长期', duration: '4 周', tasks: [...] },
  ],
  channels: ['Product Hunt', 'Twitter', 'LinkedIn'],
  // ... 更多 mock 数据
}
```

**替换任务**:
- [ ] 实现 AI 发布策略生成
- [ ] Tauri Command 调用
- [ ] 时间线可视化
- [ ] 支持调整优化

**优先级**: ⭐⭐⭐⭐ (P1)

---

#### 3.2 营销文案 Mock (4 个)

**位置**: `src/components/vibe-marketing/MarketingStrategy.tsx`

```typescript
const mockMarketingCopy = {
  twitter: "🚀 Exciting news! Our new product is launching today...",
  linkedin: "We're thrilled to announce the launch of...",
  blog: "# Introducing Our Game-Changing Product\n\nToday marks...",
  // ... 更多 mock 文案
}
```

**替换任务**:
- [ ] 实现 AI 营销文案生成
- [ ] 多渠道适配
- [ ] A/B 测试版本
- [ ] 一键复制功能

**优先级**: ⭐⭐⭐⭐ (P1)

---

### 4. AI Services (12 个)

#### 4.1 AI 服务 Mock (12 个)

**位置**: `src-tauri/src/services/ai_service.rs` (多处)

```rust
impl AIService {
    pub async fn chat(&self, messages: Vec<serde_json::Value>) -> Result<String> {
        // TODO: 根据厂商调用不同 API
        Ok("AI response placeholder".to_string())
    }
    
    pub async fn stream_chat(...) -> Result<()> {
        // TODO: 实现流式输出
        callback("Streaming response placeholder".to_string());
        Ok(())
    }
    
    pub async fn generate_prd(&self, idea: &str) -> Result<String> {
        // TODO: 构造 Prompt 并调用 AI
        Ok(format!("# PRD for: {}\n\n(Generated content)", idea))
    }
    
    pub async fn generate_personas(&self, idea: &str) -> Result<Vec<serde_json::Value>> {
        // TODO: 生成用户画像
        Ok(vec![])
    }
    
    pub async fn generate_competitor_analysis(&self, idea: &str) -> Result<String> {
        // TODO: 生成竞品分析
        Ok(format!("# Competitor Analysis for: {}\n\n(Generated content)", idea))
    }
}
```

**替换任务**:
- [ ] 实现 OpenAI API 调用（5 个方法）
- [ ] 实现 Claude API 调用（5 个方法）
- [ ] 实现 Kimi API 调用（5 个方法）
- [ ] 实现 GLM API 调用（5 个方法）
- [ ] 流式输出支持
- [ ] 错误处理

**优先级**: ⭐⭐⭐⭐⭐ (P0)

---

### 5. Frontend Hooks (5 个)

#### 5.1 AI Provider Hook (2 个)

**位置**: `src/hooks/useOpenAIProvider.ts:109-148`

```typescript
const streamChat = useCallback(async (request: OpenAIStreamRequest, onChunk: (chunk: string) => void) => {
  try {
    console.log('[useOpenAIProvider] Sending stream chat request:', request)

    // TODO: 实现 Tauri command 调用和流式处理
    // const fullContent = await invoke<string>('openai_stream_chat', {
    //   request,
    //   onChunk
    // })
    // return fullContent

    // Mock 实现（暂时）
    const mockResponse = '这是一个流式响应的模拟内容。每个字都会逐步显示。'
    const chunks = mockResponse.split('')

    for (const chunk of chunks) {
      onChunk(chunk)
      await new Promise(resolve => setTimeout(resolve, 50))
    }

    return mockResponse
  } catch (err) {
    // ...
  }
}, [])
```

**替换任务**:
- [ ] 调用真实 Tauri command
- [ ] 流式处理
- [ ] 错误处理
- [ ] 性能优化

**优先级**: ⭐⭐⭐⭐ (P1)

---

## 🎯 替换优先级

### P0 - 核心功能 (必须完成)

| ID | Mock 描述 | 文件位置 | 影响范围 | 预计工时 |
|----|----------|----------|----------|----------|
| AI-SVC-001 | AI 聊天服务 | `ai_service.rs` | 全系统 | 2d |
| AI-SVC-002 | PRD 生成 | `ai_service.rs` | Vibe Design | 1d |
| AI-SVC-003 | 用户画像 | `ai_service.rs` | Vibe Design | 1d |
| AI-SVC-004 | 竞品分析 | `ai_service.rs` | Vibe Design | 1d |
| CMD-001 | generate_prd command | `commands/ai.rs` | Vibe Design | 1d |
| CMD-002 | generate_personas command | `commands/ai.rs` | Vibe Design | 1d |
| CMD-003 | generate_competitor command | `commands/ai.rs` | Vibe Design | 1d |
| INIT-001 | Initializer Agent | `initializer_agent.rs` | Vibe Coding | 2d |
| CODE-001 | Coding Agent | `coding_agent.rs` | Vibe Coding | 3d |
| WS-001 | WebSocket 通信 | `useAgent.ts` | Vibe Coding | 2d |

**小计**: 10 个任务，15 天

---

### P1 - 重要功能 (应该完成)

| ID | Mock 描述 | 文件位置 | 影响范围 | 预计工时 |
|----|----------|----------|----------|----------|
| LOG-001 | 日志终端 | `CodingWorkspace.tsx` | Vibe Coding | 1d |
| MKT-001 | 发布策略 | `MarketingStrategy.tsx` | Vibe Marketing | 1d |
| MKT-002 | 营销文案 | `MarketingStrategy.tsx` | Vibe Marketing | 1d |
| HOOK-001 | AI Provider | `useOpenAIProvider.ts` | Vibe Design | 1d |
| DB-001 | 数据库 CRUD | 多个文件 | 全系统 | 2d |

**小计**: 5 个任务，6 天

---

### P2 - 优化功能 (可以延后)

| ID | Mock 描述 | 文件位置 | 影响范围 | 预计工时 |
|----|----------|----------|----------|----------|
| OPT-001 | AI 缓存 | `ai_service.rs` | 性能优化 | 1d |
| OPT-002 | 智能路由 | `ai_service.rs` | 成本优化 | 1d |
| OPT-003 | Token 计费 | `ai_service.rs` | 成本监控 | 1d |

**小计**: 3 个任务，3 天

---

## 📅 时间规划

```
Week 1 (03-28 ~ 04-03): AI 适配器
├── OpenAI API (2d) ✅
├── Claude API (2d) ✅
├── Kimi API (1.5d) ✅
└── GLM API (1.5d) ✅

Week 2 (04-01 ~ 04-07): Vibe Design 真实化
├── PRD 生成 (1d) ✅
├── 用户画像 (1d) ✅
├── 竞品分析 (1d) ✅
└── 流程整合 (1d) ✅

Week 2-3 (04-03 ~ 04-10): Vibe Coding 真实化
├── Initializer Agent (2d) ✅
├── Coding Agent (3d) ✅
├── WebSocket (2d) ✅
└── 数据库 (2d) ✅

Week 3 (04-07 ~ 04-10): Vibe Marketing 真实化
├── 发布策略 (1d) ✅
└── 营销文案 (1d) ✅
```

---

## ✅ 验收标准

每个 Mock 替换完成后需要满足：

### 功能性验收

- [ ] 真实 AI API 调用成功
- [ ] 数据结构与前端对接
- [ ] 错误处理完善
- [ ] 性能达标（响应时间<3s）

### 测试验收

- [ ] 单元测试覆盖率 >90%
- [ ] 集成测试通过
- [ ] E2E 测试通过
- [ ] 无已知 bug

### 代码质量

- [ ] ESLint 无错误
- [ ] Prettier 格式化
- [ ] TypeScript 编译通过
- [ ] Rust clippy 通过
- [ ] 文档齐全

---

## 🔄 进度追踪

### 完成状态

```
总进度：0/68 (0%)

✅ 已完成：0
📋 进行中：0
⏳ 待开始：68
```

### 每日更新

建议使用 GitHub Projects 或 Excel 表格追踪每个任务的：
- 负责人
- 开始日期
- 预计完成日期
- 实际完成日期
- 状态（Not Started / In Progress / Done）
- 备注

---

## 🎉 完成庆祝

每替换一个 Mock，团队应该：
- ✅ 在站会上分享
- ✅ 更新进度看板
- ✅ 拍照留念
- ✅ 小奖励（咖啡/零食）

完成所有 Mock 替换后：
- 🎊 团队聚餐
- 📸 合影纪念
- 🏆 颁发"Mock 杀手"奖杯
- 📝 编写经验总结文档

---

**最后更新**: 2026-03-28  
**下次审查**: 每周五回顾会议

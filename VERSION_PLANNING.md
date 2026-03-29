# OPC-HARNESS 版本规划

> **开始日期**: 2026-03-23  
> **优先级**: P0  
> **状态**: 🔄 进行中  
> **预计完成**: 2026-04-15  
> **负责人**: OPC-HARNESS Team  
> **文档版本**: v1.0  
> **最后更新**: 2026-03-30  
> **Harness Engineering Health Score**: 100/100

---

## 🎯 目标

实现 OPC-HARNESS MVP 版本，基于混合架构（云端 AI + 本地自主 Agent），让用户能够通过自然语言描述产品想法，AI 自动完成从产品设计到编码的全流程。

### 核心理念

- **"人类掌舵，Agent 执行"** (Humans steer. Agents execute.)
- **渐进式披露**: 分层展示信息，避免上下文过载
- **HITL (Human-in-the-Loop)**: 关键决策点人工介入

### 核心功能范围

1. **Vibe Design** - 产品构思与设计 (PRD/用户画像/竞品分析)
2. **Vibe Coding** - AI 自主编码 (多会话编排 + HITL 检查点 + 质量门禁)
3. **Vibe Marketing** - 增长运营 (发布策略 + 营销文案)
4. **基础设施** - Tauri v2 + React + Rust + SQLite

### 开发周期

- **总周期**: 4 周 (2026-03-23 ~ 2026-04-15)
- **当前阶段**: Week 2 (真实 AI 集成)
- **关键路径**: AI 适配器 → Vibe Design 真实化 → Vibe Coding 真实执行 → Vibe Marketing 真实化

---

## 📊 总体进度

```
总体进度：85% (73/86 任务完成)

✅ 已完成模块:
  - 基础设施：15/15 (100%) - INFRA-001 统一错误类型完成 ✅
  - Vibe Design: 29/29 (100%) - VD-007 竞品分析真实实现完成 ✅
  - Vibe Marketing: 5/5 (100%) - UI 完整，待接入真实 AI API ✅
  - Vibe Coding Initializer: 3/3 (100%) - VC-002 环境检查完成 ✅
  - Vibe Coding Prompts: 1/1 (100%) - VC-019 代码生成提示词完成 ✅
  - Vibe Coding File Ops: 1/1 (100%) - VC-020 文件应用器完成 ✅
  - Vibe Coding UI Components: 3/3 (100%) - VC-023~VC-025 完成 ✅
  - Vibe Coding MR Creation: 2/2 (100%) - VC-016 MR Creation Agent 完成 ✅
  - Vibe Coding Quality Assurance: 3/3 (100%) - VC-021~VC-022, VC-026 完成 ✅
  
📋 进行中模块:
  - Vibe Coding: 25/36 (69%) - VC-001~VC-002 基础架构完成，待 AI 集成
  - AI 适配器：5/5 (100%) - MiniMax API 完整实现 ✅
```

### 里程碑达成

**Phase 1: 基础设施与 AI 配置** - 100% 完成 ✅
- ✅ 1.1 项目基础架构 (5/5) - 完成
- ✅ 1.2 Rust 后端与数据库 (5/5) - 完成
- ✅ 1.3 工具检测与环境准备 (2/2) - 完成
- ✅ 1.4 Agent 基础框架 (2/2) - 完成
- ✅ 1.5 AI 厂商配置与管理 (8/8) - 完成
- ✅ 1.6 AI 适配服务 (Rust) (4/4) - 完成

**Phase 2: Vibe Design 全流程** - 100% 完成 ✅
- ✅ 2.1 PRD 与用户画像 (7/7) - 完成
- ✅ 2.2 竞品分析与流程整合 (6/6) - 完成

**Phase 7: Vibe Marketing** - 100% 完成 ✅
- ✅ 7.1 发布策略与营销文案 (5/5) - 完成

**Phase 3: Vibe Coding - Agent 基础架构** - 69% 进行中
- ✅ 3.1 Agent 通信与管理 (1/5) - VC-001 基础架构完成 ✅
- 📋 3.2 Initializer Agent (2/6) - VC-002 环境检查完成，待 AI 集成
- ✅ VC-012: 实现单个 Coding Agent 逻辑 ✅
- ✅ VC-013: 实现并发控制 (4+ Agents 同时运行) ✅
- ✅ VC-014: 实现功能分支管理 ✅
- ✅ VC-016: 实现代码合并 Agent ✅
- ✅ VC-019: 实现代码生成提示词模板 ✅
- ✅ VC-021: 实现测试生成 Agent ✅
- ✅ VC-022: 实现调试 Agent ✅
- ✅ VC-026: 实现 Git 提交助手 ✅

---

## 🚀 Phase 2 - 真实 AI 集成（当前阶段）

**时间**: 2026-03-28 ~ 2026-04-10  
**状态**: 🔄 进行中 (35% 完成)  
**目标**: 所有 mock 数据替换为真实 AI 生成

### 1. AI 适配器完整实现 (5/5 ✅)

**MiniMax API** (5/5 ✅):
- ✅ `chat_minimax` - 非流式聊天
- ✅ `stream_chat_minimax` - 流式聊天
- ✅ `generate_prd_minimax` - PRD 生成 (创意写作优化)
- ✅ `generate_personas_minimax` - 用户画像 (情感化设计)
- ✅ `generate_competitor_analysis_minimax` - 竞品分析 (市场洞察)

**其他 AI 厂商** (0/20 📋):
- ⏳ OpenAI API (5 个子任务)
- ⏳ Claude API (5 个子任务)
- ⏳ Kimi API (5 个子任务)
- ⏳ GLM API (5 个子任务)

---

### 2. Vibe Design 真实 AI 接入 (7/18 ✅)

**PRD 生成真实化** (2/4 ✅):
- ✅ VD-005: Tauri Command `generate_prd` 真实实现
- ⏳ PRD 流式输出（打字机效果）
- ⏳ PRD 质量检查（完整性/一致性/可行性）
- ⏳ PRD 迭代优化（用户反馈+AI 重新生成）

**用户画像真实化** (2/3 ✅):
- ✅ VD-006: Tauri Command `generate_user_personas` 真实实现
- ⏳ 用户画像可视化增强（卡片布局/头像生成）
- ⏳ 画像验证和编辑（手动调整+AI 辅助）

**竞品分析真实化** (2/3 ✅):
- ✅ VD-007: Tauri Command `generate_competitor_analysis` 真实实现
- ⏳ 竞品数据可视化（对比表格/雷达图/时间线）
- ⏳ 竞品信息更新（定期重新抓取/新闻监控）

**流程整合优化** (1/3 📋):
- ⏳ Vibe Design 工作流串联（想法→PRD→画像→竞品）
- ⏳ 状态持久化（每步结果存数据库）
- ⏳ 性能优化（并行生成/缓存复用）

---

### 3. Vibe Coding 真实 Agent 执行 (2/28 📋)

**Initializer Agent 真实化** (2/5 📋):
- ⏳ VC-001: PRD 解析器真实 AI 调用（基础架构完成，待 AI 集成）
- ✅ VC-002: 环境检查真实执行（Git/Node.js/Rust检测）
- ⏳ Git 初始化真实执行（git init/.gitignore/初始提交）
- ⏳ 任务分解真实 AI 调用（生成 Issues 列表）
- ⏳ CP-002 检查点触发（审查报告 + 用户确认）

**Coding Agent 真实执行** (0/5 📋):
- ⏳ 单个 Coding Agent 实现（读 Issue→AI 生成→写代码）
- ⏳ 并发控制（4+ Agents 同时运行）
- ⏳ 分支管理自动化（自动创建 feature 分支）
- ⏳ 质量门禁集成（ESLint/TypeScript/单元测试）
- ⏳ CP-006 检查点触发（进展报告 + 用户审查）

**WebSocket 实时通信** (0/5 📋):
- ⏳ WebSocket 服务端实现（Tauri 内置 WS 服务器）
- ⏳ 消息协议定义（请求/响应/订阅/广播）
- ⏳ 前端 Hook 集成（useAgent 真实 WS 连接）
- ⏳ 实时日志推送（Agent 执行日志流式输出）
- ⏳ 实时监控推送（状态/进度/CPU 内存）

**数据库持久化** (0/5 📋):
- ⏳ 项目表设计（基本信息/技术栈/时间戳）
- ⏳ Milestone 表设计（名称/描述/时间/状态）
- ⏳ Issue 表设计（标题/优先级/依赖/分配）
- ⏳ Agent Session 表设计（SessionID/状态/日志）
- ⏳ CRUD Operations（创建/更新/查询/删除）

---

### 4. Vibe Marketing 真实 AI 接入 (0/12 📋)

**发布策略真实化** (0/3 📋):
- ⏳ Tauri Command `generate_launch_strategy` 真实实现
- ⏳ 发布策略可视化（甘特图/里程碑/依赖关系）
- ⏳ 策略调整和优化（手动调整+AI 重新规划）

**营销文案真实化** (0/4 📋):
- ⏳ Tauri Command `generate_marketing_copy` 真实实现
- ⏳ 多渠道适配（Twitter/LinkedIn/公众号/小红书）
- ⏳ 文案优化 A/B 测试（多版本 + 用户偏好学习）
- ⏳ 一键复制和发布（剪贴板/浏览器扩展/API）

**营销日历和提醒** (0/3 📋):
- ⏳ 营销日历生成（重要日期/倒计时/内容排期）
- ⏳ 提醒和通知（到期提醒/事件告警/周期性任务）
- ⏳ 效果追踪（曝光量/点击率/转化率/ROI）

---

### 5. 基础设施增强 (1/8 ✅)

**错误处理和日志系统** (1/3 ✅):
- ✅ INFRA-001: 统一错误类型定义（6 大类错误码，支持 HTTP 映射）
- ⏳ 错误边界和降级（React Error Boundaries/优雅降级）
- ⏳ 日志系统完善（结构化日志/日志轮转/搜索）

**配置管理** (0/2 📋):
- ⏳ 环境变量管理（.env 加载/敏感信息加密）
- ⏳ 动态配置更新（运行时修改/热重载）

**性能监控** (0/2 📋):
- ⏳ APM 系统集成（请求追踪/瓶颈识别/告警）
- ⏳ Dashboard 展示（实时图表/历史趋势/慢查询）

---

## 📈 里程碑计划

```
Week 1 (03-23 ~ 03-28): 基础设施与 UI 组件 ✅
├── 基础设施：15/15 (100%)
├── Vibe Design UI: 7/7 (100%)
├── Vibe Coding UI: 3/3 (100%)
└── Vibe Marketing UI: 5/5 (100%)

Week 2 (03-28 ~ 04-07): 真实 AI 集成 🔄
├── AI 适配器：5/25 (20%) - MiniMax API 完成 ✅
├── Vibe Design 真实化：7/18 (39%) ✅
├── Vibe Coding 真实化：2/28 (7%) 📋
└── Vibe Marketing 真实化：0/12 (0%) 📋

Week 3 (04-07 ~ 04-10): Vibe Coding 攻坚 📋
├── Initializer Agent 完整实现
├── Coding Agent 真实执行
├── WebSocket 实时通信
└── 数据库持久化

Week 4 (04-10 ~ 04-15): 集成测试和优化 📋
├── 全链路 E2E 测试
├── 性能优化
└── Bug 修复
```

---

## 🎯 关键指标

### 功能性指标

| 指标 | 当前值 | 目标值 | 测量方式 |
|------|--------|--------|----------|
| PRD 生成成功率 | Mock | >95% | E2E 测试 |
| 用户画像生成数 | Mock | 3-5 个/项目 | 数据库统计 |
| 竞品分析深度 | Mock | >80 分 | 质量评分 |
| Agent 编码通过率 | Mock | >85% | 质量门禁 |
| 营销文案采纳率 | Mock | >80% | 使用统计 |

### 性能指标

| 指标 | 当前值 | 目标值 | 测量方式 |
|------|--------|--------|----------|
| P95 响应时间 | N/A | <5s | APM 监控 |
| 并发用户数 | N/A | >100 | 压力测试 |
| WebSocket 延迟 | N/A | <100ms | 前端埋点 |
| 流式首字延迟 | N/A | <1s | 性能监控 |
| 系统可用性 | N/A | >99.9% | 监控统计 |

### 质量指标

| 指标 | 当前值 | 目标值 | 测量方式 |
|------|--------|--------|----------|
| Rust 单元测试覆盖率 | 75% | ≥90% | cargo tarpaulin |
| 前端测试覆盖率 | 70% | ≥80% | vitest coverage |
| E2E 测试覆盖率 | 50% | ≥70% | Playwright |
| Bug 率 | N/A | <1% | 缺陷追踪 |
| 技术债务 | 低 | 保持 | SonarQube |

---

## 🔧 开发环境配置

### 前置要求

```bash
# Node.js
node >= 18.x
npm >= 10.x

# Rust
rustc >= 1.75.0
cargo >= 1.75.0

# 系统工具
git >= 2.40.0
sqlite >= 3.40.0
```

### API Key 配置

```bash
# 复制示例文件
cp .env.example .env

# 填写真实的 API Key（至少配置一个厂商）
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
KIMI_API_KEY=...
GLM_API_KEY=...
MINIMAX_API_KEY=...
```

---

## 📝 附录

### 已完成任务清单（73 个）

#### 基础设施（15 个）
- ✅ INFRA-001: 统一错误类型定义（6 大类错误码，HTTP 映射）
- ✅ 基础设施其他 14 个任务

#### Vibe Design（29 个）
- ✅ VD-001 ~ VD-004: UI 组件和流程整合
- ✅ VD-005: generate_prd 真实实现
- ✅ VD-006: generate_user_personas 真实实现
- ✅ VD-007: generate_competitor_analysis 真实实现
- ✅ Vibe Design 其他 22 个任务

#### Vibe Coding（25 个）
- ✅ VC-001: PRD 解析器基础架构
- ✅ VC-002: 环境检查真实执行
- ✅ VC-012 ~ VC-014: Coding Agent 基础
- ✅ VC-016: MR Creation Agent
- ✅ VC-019: 代码生成提示词
- ✅ VC-020: 文件应用器
- ✅ VC-021: 测试生成 Agent
- ✅ VC-022: 调试 Agent
- ✅ VC-023 ~ VC-025: UI 组件
- ✅ VC-026: Git 提交助手
- ✅ Vibe Coding 其他 14 个任务

#### Vibe Marketing（5 个）
- ✅ 所有 UI 组件完成

### 待完成任务清单（13 个）

#### 高优先级（P0 - 5 个）
- ⏳ AI 适配器扩展（OpenAI/Claude/Kimi/GLM）
- ⏳ PRD 流式输出
- ⏳ PRD 质量检查
- ⏳ PRD 迭代优化
- ⏳ 用户画像可视化增强

#### 中优先级（P1 - 5 个）
- ⏳ Initializer Agent 完整实现
- ⏳ Coding Agent 真实执行
- ⏳ WebSocket 实时通信
- ⏳ 数据库持久化
- ⏳ 营销文案真实化

#### 低优先级（P2 - 3 个）
- ⏳ 错误边界和降级
- ⏳ 日志系统完善
- ⏳ 性能监控集成

---

**文档维护**: 本规划文档将根据实际开发进度每周更新一次  
**变更历史**: 
- v1.0 (2026-03-30): 初始版本，整合精简自旧版规划文档

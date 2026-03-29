# OPC-HARNESS 版本规划

> **创建日期**: 2026-03-28  
> **最后更新**: 2026-03-28  
> **状态**: 📋 规划中  

---

## 📊 版本总览

```
┌─────────────────────────────────────────────────────────────┐
│  MVP Version (Phase 1)          │  Phase 2 (Real AI)        │
│  ✅ UI 组件完成 (100%)           │  📋 AI 集成待开始 (0%)     │
│  ✅ 架构基础完成 (100%)          │  📋 Mock 数据替换 (0%)     │
│  ✅ Agent 框架完成 (100%)        │  📋 WebSocket 通信 (0%)    │
│  ⚠️  Mock 数据待替换             │  📋 数据库持久化 (0%)      │
└─────────────────────────────────────────────────────────────┘
```

---

## 🎯 MVP 版本 (Phase 1) - 即将完成

**时间**: 2026-03-23 ~ 2026-03-28  
**状态**: ✅ 99% 完成  
**目标**: 完成所有 UI 组件和架构基础

### 已完成模块

#### ✅ 基础设施 (100%)
- Tauri v2 + React + Rust 架构
- SQLite 数据库设计
- AI 配置管理界面
- 工具检测系统

#### ✅ Vibe Design UI (100%)
- 想法输入界面
- PRD 展示界面
- 用户画像界面
- 竞品分析界面
- **注**: 当前使用 mock 数据，待接入真实 AI

#### ✅ Vibe Coding UI (100%)
- 文件浏览器组件
- 代码编辑器组件
- 终端模拟器组件
- 检查点审查界面
- Initializer 工作流 UI
- Agent 监控面板
- **注**: 当前使用 mock 数据，待 Agent 真实执行

#### ✅ Vibe Marketing UI (100%)
- 发布策略界面
- 营销文案界面
- 一键复制功能
- **注**: 当前使用 mock 数据，待接入真实 AI

#### ✅ Agent 框架 (100%)
- Agent 基础架构
- 消息协议定义
- 并发控制机制
- 分支管理
- MR Creation Agent
- 测试生成 Agent
- 调试 Agent
- 代码审查 Agent
- Git 提交助手
- **注**: 大部分为框架代码，待真实 AI 调用

---

## 🚀 Phase 2 - 真实 AI 集成

**时间**: 2026-03-28 ~ 2026-04-10  
**状态**: 📋 待开始  
**目标**: 所有 mock 数据替换为真实 AI 生成

### 核心任务

#### 1. AI 适配器完整实现 (12 个任务)

**OpenAI API** (5 个子任务):
- [ ] `chat_openai` - 非流式聊天
- [ ] `stream_chat_openai` - 流式聊天
- [ ] `generate_prd_openai` - PRD 生成
- [ ] `generate_personas_openai` - 用户画像
- [ ] `generate_competitor_analysis_openai` - 竞品分析

**Claude API** (5 个子任务):
- [ ] `chat_claude` - 非流式聊天
- [ ] `stream_chat_claude` - 流式聊天
- [ ] `generate_prd_claude` - PRD 生成
- [ ] `generate_personas_claude` - 用户画像
- [ ] `generate_competitor_analysis_claude` - 竞品分析

**Kimi API** (5 个子任务):
- [ ] `chat_kimi` - 非流式聊天
- [ ] `stream_chat_kimi` - 流式聊天
- [ ] `generate_prd_kimi` - PRD 生成 (中文优化)
- [ ] `generate_personas_kimi` - 用户画像 (本地化)
- [ ] `generate_competitor_analysis_kimi` - 竞品分析 (中国市场)

**GLM API** (5 个子任务):
- [ ] `chat_glm` - 非流式聊天
- [ ] `stream_chat_glm` - 流式聊天
- [ ] `generate_prd_glm` - PRD 生成 (技术导向)
- [ ] `generate_personas_glm` - 用户画像 (开发者)
- [ ] `generate_competitor_analysis_glm` - 竞品分析 (开源)

**MiniMax API** (5 个子任务):
- [x] `chat_minimax` - 非流式聊天 ✅ **已完成**
- [x] `stream_chat_minimax` - 流式聊天 ✅ **已完成**
- [x] `generate_prd_minimax` - PRD 生成 (创意写作优化) ✅ **已完成**
- [x] `generate_personas_minimax` - 用户画像 (情感化设计) ✅ **已完成**
- [x] `generate_competitor_analysis_minimax` - 竞品分析 (市场洞察) ✅ **已完成**

**AI 服务管理器增强** (4 个子任务):
- [ ] 智能路由（根据任务选择最佳 AI）
- [ ] Token 计数和计费
- [ ] 缓存机制（减少重复请求）
- [ ] 速率限制（避免 API 限流）

**验收标准**:
- ✅ 所有 AI 厂商 API 可正常调用
- ✅ 支持流式输出（打字机效果）
- ✅ 错误处理完善
- ✅ 响应时间达标（<3s 非流式，<1s首字流式）

---

#### 2. Vibe Design 真实 AI 接入 (18 个任务)

**PRD 生成真实化** (4 个子任务):
- [ ] Tauri Command `generate_prd` 真实实现
- [ ] PRD 流式输出（打字机效果）
- [ ] PRD 质量检查（完整性/一致性/可行性）
- [ ] PRD 迭代优化（用户反馈+AI 重新生成）

**用户画像真实化** (3 个子任务):
- [ ] Tauri Command `generate_user_personas` 真实实现
- [ ] 用户画像可视化增强（卡片布局/头像生成）
- [ ] 画像验证和编辑（手动调整+AI 辅助）

**竞品分析真实化** (3 个子任务):
- [ ] Tauri Command `generate_competitor_analysis` 真实实现
- [ ] 竞品数据可视化（对比表格/雷达图/时间线）
- [ ] 竞品信息更新（定期重新抓取/新闻监控）

**流程整合优化** (3 个子任务):
- [ ] Vibe Design 工作流串联（想法→PRD→画像→竞品）
- [ ] 状态持久化（每步结果存数据库）
- [ ] 性能优化（并行生成/缓存复用）

**验收标准**:
- ✅ PRD 生成成功率 >95%
- ✅ 平均生成时间 <10s
- ✅ 用户满意度 >4.5/5
- ✅ Mock 数据完全替换

---

#### 3. Vibe Coding 真实 Agent 执行 (28 个任务)

**Initializer Agent 真实化** (5 个子任务):
- [ ] PRD 解析器真实 AI 调用（提取里程碑/任务依赖）
- [ ] 环境检查真实执行（Git/Node.js/Rust检测）
- [ ] Git 初始化真实执行（git init/.gitignore/初始提交）
- [ ] 任务分解真实 AI 调用（生成 Issues 列表）
- [ ] CP-002 检查点触发（审查报告 + 用户确认）

**Coding Agent 真实执行** (5 个子任务):
- [ ] 单个 Coding Agent 实现（读 Issue→AI 生成→写代码）
- [ ] 并发控制（4+ Agents 同时运行）
- [ ] 分支管理自动化（自动创建 feature 分支）
- [ ] 质量门禁集成（ESLint/TypeScript/单元测试）
- [ ] CP-006 检查点触发（进展报告 + 用户审查）

**WebSocket 实时通信** (5 个子任务):
- [ ] WebSocket 服务端实现（Tauri 内置 WS 服务器）
- [ ] 消息协议定义（请求/响应/订阅/广播）
- [ ] 前端 Hook 集成（useAgent 真实 WS 连接）
- [ ] 实时日志推送（Agent 执行日志流式输出）
- [ ] 实时监控推送（状态/进度/CPU 内存）

**数据库持久化** (5 个子任务):
- [ ] 项目表设计（基本信息/技术栈/时间戳）
- [ ] Milestone 表设计（名称/描述/时间/状态）
- [ ] Issue 表设计（标题/优先级/依赖/分配）
- [ ] Agent Session 表设计（SessionID/状态/日志）
- [ ] CRUD Operations（创建/更新/查询/删除）

**验收标准**:
- ✅ Initializer 流程全自动
- ✅ Agent 自主编码能力强
- ✅ WebSocket 连接稳定（延迟<100ms）
- ✅ 数据库持久化可靠

---

#### 4. Vibe Marketing 真实 AI 接入 (12 个任务)

**发布策略真实化** (3 个子任务):
- [ ] Tauri Command `generate_launch_strategy` 真实实现
- [ ] 发布策略可视化（甘特图/里程碑/依赖关系）
- [ ] 策略调整和优化（手动调整+AI 重新规划）

**营销文案真实化** (4 个子任务):
- [ ] Tauri Command `generate_marketing_copy` 真实实现
- [ ] 多渠道适配（Twitter/LinkedIn/公众号/小红书）
- [ ] 文案优化 A/B 测试（多版本 + 用户偏好学习）
- [ ] 一键复制和发布（剪贴板/浏览器扩展/API）

**营销日历和提醒** (3 个子任务):
- [ ] 营销日历生成（重要日期/倒计时/内容排期）
- [ ] 提醒和通知（到期提醒/事件告警/周期性任务）
- [ ] 效果追踪（曝光量/点击率/转化率/ROI）

**验收标准**:
- ✅ 发布策略可行实用
- ✅ 文案质量高吸引眼球
- ✅ 多渠道覆盖全面
- ✅ 操作流程简单便捷

---

#### 5. 基础设施增强 (8 个任务)

**错误处理和日志系统** (3 个子任务):
- [ ] 统一错误类型定义（AI API/网络/数据库/文件系统）
- [ ] 错误边界和降级（React Error Boundaries/优雅降级）
- [ ] 日志系统完善（结构化日志/日志轮转/搜索）

**配置管理** (2 个子任务):
- [ ] 环境变量管理（.env 加载/敏感信息加密）
- [ ] 动态配置更新（运行时修改/热重载）

**性能监控** (2 个子任务):
- [ ] APM 系统集成（请求追踪/瓶颈识别/告警）
- [ ] Dashboard 展示（实时图表/历史趋势/慢查询）

**验收标准**:
- ✅ 错误处理全覆盖
- ✅ 配置管理灵活
- ✅ 监控系统完善

---

## 📈 里程碑计划

```
Week 1 (03-28 ~ 04-03): AI 适配器完整实现
├── OpenAI API ✅
├── Claude API ✅
├── Kimi API ✅
├── GLM API ✅
└── MiniMax API 📋

Week 2 (04-01 ~ 04-07): Vibe Design 真实化
├── PRD 生成 ✅
├── 用户画像 ✅
└── 竞品分析 ✅

Week 2-3 (04-03 ~ 04-10): Vibe Coding 真实执行
├── Initializer Agent ✅
├── Coding Agent ✅
├── WebSocket 通信 ✅
└── 数据库持久化 ✅

Week 3 (04-07 ~ 04-10): Vibe Marketing 真实化
├── 发布策略 ✅
└── 营销文案 ✅

Week 3-4 (04-10 ~ 04-15): 集成测试和优化
├── 全链路 E2E 测试 ✅
├── 性能优化 ✅
└── Bug 修复 ✅
```

---

## 🎯 关键指标

### 功能性指标

| 指标 | 当前值 | 目标值 | 测量方式 |
|------|--------|--------|----------|
| PRD 生成成功率 | 0% (mock) | >95% | E2E 测试 |
| 用户画像生成数 | 0 (mock) | 3-5 个/项目 | 数据库统计 |
| 竞品分析深度 | 0 (mock) | >80 分 | 质量评分 |
| Agent 编码通过率 | 0% (mock) | >85% | 质量门禁 |
| 营销文案采纳率 | 0% (mock) | >80% | 使用统计 |

### 性能指标

| 指标 | 当前值 | 目标值 | 测量方式 |
|------|--------|--------|----------|
| P95 响应时间 | N/A | <5s | APM 监控 |
| 并发用户数 | N/A | >100 | 压力测试 |
| WebSocket 延迟 | N/A | <100ms | 前端埋点 |
| 流式首字延迟 | N/A | <1s | 性能监控 |
| 系统可用性 | N/A | >99.9% | 监控统计 |

### 质量指标

| 指标 | 当前값 | 目标값 | 测量方式 |
|------|--------|--------|----------|
| 单元测试覆盖率 | 75% | ≥90% (Rust) | cargo tarpaulin |
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

```
# 复制示例文件
cp .env.example .env

# 填写真实的 API Key（至少配置一个厂商）
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
KIMI_API_KEY=...
GLM_API_KEY=...
MINIMAX_API_KEY=...

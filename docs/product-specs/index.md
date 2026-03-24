# 产品规范索引

> 本目录包含所有产品需求文档和功能规格说明  
> **最后更新**: 2026-03-24  
> **文档状态**: 🔄 持续更新中

## 📋 核心产品文档

### 战略与规划

| 文档 | 版本 | 状态 | 最后更新 | 位置 |
|------|------|------|----------|------|
| [MVP版本规划](./mvp-roadmap.md) | v2.10 | 🔄 进行中 | 2026-03-24 | 当前目录 |
| [产品设计文档](./product-design.md) | v1.1 | ✅ 已完成 | 2026-03-22 | 当前目录 |

### MVP 功能模块

| 模块 | 阶段 | 状态 | 完成度 | 负责人 |
|------|------|------|--------|--------|
| **Vibe Design** | Phase 2 | ✅ 已完成 | 100% | 产品团队 |
| **Vibe Coding** | Phase 3-6 | 🔄 开发中 | 28% | 技术团队 |
| **Vibe Marketing** | Phase 7 | ✅ 已完成 | 100% | 产品团队 |

## 🎯 功能规格说明

### Vibe Design (产品构思)

**目标**: AI 驱动的产品设计助手，从想法到完整 PRD

| 功能 | 状态 | 详细说明 |
|------|------|---------|
| 想法输入 | ✅ 完成 | 自然语言描述产品想法 |
| PRD 生成 | ✅ 完成 | AI 生成结构化产品需求文档 |
| 用户画像 | ✅ 完成 | 自动生成目标用户画像 |
| 竞品分析 | ✅ 完成 | AI 搜索并分析竞品 |
| 技术评估 | ✅ 完成 | 技术可行性和工作量评估 |

**相关文档**: 
- [`vibe-design-spec.md`](./vibe-design-spec.md) - 详细功能规格
- [Product Design Document §6.1](./product-design.md#6-product-solution) - Product Architecture

### Vibe Coding (自主编码) ⭐

**目标**: AI 驱动的自主编码系统，从 PRD 到可部署代码

| 功能 | 状态 | 优先级 | 详细说明 |
|------|------|--------|---------|
| 多会话编排 | 🔄 开发中 | P0 | Initializer + Coding Agents + MR Creation |
| HITL 检查点 | 📋 规划中 | P0 | 8 个关键决策点的人工审查 |
| 质量门禁 | 🔄 部分完成 | P0 | ESLint/TSC/Pytest自动检查 |
| 守护进程 | ✅ 完成 | P0 | 代理生命周期管理 |
| 实时日志 | 📋 规划中 | P0 | WebSocket 实时推送 |
| Git 集成 | 📋 规划中 | P1 | 自动分支、提交、MR |

**核心机制**:
1. **多会话编排**: 将复杂任务分解为多个子任务，由不同 AI 代理协同完成
2. **HITL 检查点**: 在关键决策点设置人工审查环节（CP-001 ~ CP-008）
3. **质量门禁**: 多层次质量保障（QG-001 ~ QG-005）
4. **守护进程架构**: 后台管理代理生命周期，支持断点续传

**相关文档**:
- [`vibe-coding-spec.md`](./vibe-coding-spec.md) - 详细功能规格（待创建）
- [产品设计文档 §6.2](./product-design.md#62-vibe-coding-核心机制) - 核心机制
- [Symphony 参考文档](../references/symphony.md) - 技术参考

### Vibe Marketing (增长运营)

**目标**: AI 辅助的增长运营助手

| 功能 | 状态 | 详细说明 |
|------|------|---------|
| 发布策略 | ✅ 完成 | AI 生成产品发布计划 |
| 营销文案 | ✅ 完成 | 社交媒体文案自动生成 |
| 多渠道发布 | 📋 规划中 | Product Hunt/Twitter 等集成 |
| 数据看板 | 📋 规划中 | 基础数据监控 |
| 用户反馈 | 📋 规划中 | 反馈收集和分析 |

**相关文档**:
- [`vibe-marketing-spec.md`](./vibe-marketing-spec.md) - 详细功能规格
- [产品设计文档 §7](./product-design.md#7-交互设计) - 交互设计

## 🛠️ 基础设施

### AI 配置管理

- **需求**: 支持多 AI 厂商配置
- **状态**: ✅ 已完成
- **支持厂商**: OpenAI, Kimi, GLM, Anthropic, MiniMax
- **文档**: [`ai-config-spec.md`](./ai-config-spec.md)

### 项目管理

- **需求**: 项目创建、进度追踪、状态管理
- **状态**: ✅ 已完成
- **特性**: SQLite 持久化、Git 集成
- **文档**: [`project-management-spec.md`](./project-management-spec.md)

### CLI 集成

- **需求**: 支持 Kimi/Claude/Codex CLI工具
- **状态**: ✅ 已完成
- **文档**: [`cli-integration-spec.md`](./cli-integration-spec.md)

### 一键部署

- **需求**: Vercel/Netlify自动化部署
- **状态**: 📋 规划中
- **平台**: Vercel, Netlify, Cloudflare
- **文档**: [`deployment-spec.md`](./deployment-spec.md) (待创建)

## 📊 数据指标

### 产品健康度

| 指标 | 定义 | 目标值 | 当前值 | 状态 |
|------|------|--------|--------|------|
| 项目完成率 | 成功完成从构思到发布的项目数 / 总项目数 | > 75% | - | 📊 待统计 |
| 模块使用率 | 使用全部三个模块的用户数 / 总用户数 | > 60% | - | 📊 待统计 |
| AI 编码成功率 | 成功完成编码任务的次数 / 总编码次数 | > 85% | - | 📊 待统计 |
| 检查点通过率 | 自动批准的检查点数 / 总检查点数 | > 70% | - | 📊 待统计 |
| 平均完成时间 | 从想法到 MVP 的平均时间 | < 6 小时 | - | 📊 待统计 |

### 技术指标

| 指标 | 定义 | 目标值 | 当前值 | 状态 |
|------|------|--------|--------|------|
| 界面响应时间 | 用户操作到界面响应的时间 | < 300ms | - | 📊 待统计 |
| AI 生成时间 (PRD) | 从输入到 AI 输出 PRD 的时间 | < 30s | - | 📊 待统计 |
| 代码质量分数 | 通过质量门禁的会话数 / 总会话数 | > 95% | - | 📊 待统计 |
| 部署成功率 | 一键部署成功的次数 / 总部署次数 | > 95% | - | 📊 待统计 |
| 日志延迟 | 从代理输出到界面显示的时间 | < 100ms | - | 📊 待统计 |

### 业务指标

| 指标 | 定义 | 目标值 | 当前값 | 状态 |
|------|------|--------|--------|------|
| 付费转化率 | 付费用户数 / 总活跃用户数 | > 15% | - | 📊 待统计 |
| 客单价 (ARPU) | 平均每用户收入 | > $20/月 | - | 📊 待统计 |
| 用户生命周期价值 (LTV) | 用户全生命周期价值 | > $100 | - | 📊 待统计 |

**数据埋点**: 详见 [产品设计文档 §8](./product-design.md#8-数据埋点)

## 🔄 版本历史

### v0.1.0 (2026-03-22) - MVP 基础

**交付内容**:
- ✅ Vibe Design 完整流程
  - 想法输入 → PRD 生成 → 用户画像 → 竞品分析
- ✅ Vibe Coding 工作环境
  - Agent 基础架构、Stdio/WebSocket通信
  - AgentManager、会话持久化
- ✅ Vibe Marketing 文案生成
  - 发布策略、营销文案自动生成
- ✅ AI 厂商配置管理
  - 5 家 AI 厂商支持（OpenAI, Kimi, GLM, Anthropic, MiniMax）
- ✅ SQLite 数据持久化
  - 项目数据、AI 配置、Agent 会话状态

**技术亮点**:
- Health Score 持续保持 100/100 ⭐
- 测试覆盖率 > 95%
- 零架构违规，零技术债务

### v0.2.0 (规划中) - Vibe Coding 核心

**计划交付**:
- 📋 Initializer Agent (VC-006 ~ VC-011)
  - PRD 解析、环境检查、任务分解
- 📋 Coding Agent 集群 (VC-012 ~ VC-017)
  - 代码生成、测试编写、并发控制
- 📋 质量门禁系统 (VC-018 ~ VC-022)
  - ESLint、TSC、Pytest自动检查和修复
- 📋 HITL 检查点 (VC-023 ~ VC-028)
  - 8 个关键决策点的人工审查界面

**预期时间**: 2026-04-10

### v0.3.0 (规划中) - 完整闭环

**计划交付**:
- 📋 MR Creation Agent (VC-029 ~ VC-032)
  - 分支合并、回归测试、MR 生成
- 📋 监控与日志 (VC-033 ~ VC-036)
  - 实时日志流、进度可视化、错误检测
- 📋 一键部署功能
  - Vercel、Netlify集成
- 📋 端到端测试和优化

**预期时间**: 2026-04-15

## 📚 相关资源

### 内部文档
- [`product-design.md`](./product-design.md) - 完整产品需求和市场定位
- [`MVP版本规划.md`](./MVP版本规划.md) - MVP版本规划和路线图
- [`../design-docs/system-architecture.md`](../design-docs/system-architecture.md) - 技术架构和系统设计
- [`../references/symphony.md`](../references/symphony.md) - AI编码代理编排参考
- [`../../AGENTS.md`](../../AGENTS.md) - AI Agent 导航地图

### 外部参考
- [Anthropic Claude API](https://docs.anthropic.com/)
- [OpenAI GPT API](https://platform.openai.com/)
- [Vercel Deployment API](https://vercel.com/docs/api)
- [Netlify API](https://docs.netlify.com/api/)

---

**维护者**: OPC-HARNESS Product Team  
**联系方式**: product@opc-harness.dev  
**贡献指南**: 提交 Issue 或 Pull Request

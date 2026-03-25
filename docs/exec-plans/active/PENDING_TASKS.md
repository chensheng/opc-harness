# 未完成任务清单 (Pending Tasks)

> **创建日期**: 2026-03-25  
> **最后更新**: 2026-03-25  
> **来源文档**: [MVP版本规划](../product-specs/mvp-roadmap.md)  
> **当前状态**: 📋 待执行  

---

## 📊 总体统计

- **总任务数**: 81 个
- **已完成**: 56 个 (70%) ✅
- **未完成**: 25 个 (30%) 📋
- **模块分布**: 
  - Vibe Coding: 24 个待完成
  - AI 适配器：5 个待完成

---

## 🔴 Vibe Coding 模块 (24 个任务)

### Phase 3.2: Initializer Agent 完整工作流

**状态**: 部分 UI 已完成，后端逻辑待整合

#### VC-010: 实现 Initializer Agent 主逻辑 ⭐⭐⭐ P0
- **类型**: Backend Logic
- **描述**: 整合 PRD 解析、环境检查、Git 初始化，实现完整的 Initializer Agent 工作流
- **依赖**: VC-006 (PRD 解析器), VC-007 (环境检查), VC-008 (Git 初始化), VC-009 (任务分解) ✅ 已完成
- **前置条件**: PRD 解析器、环境检查、Git 仓库初始化、任务分解算法均已完成
- **关键路径**: 是 ⭐
- **预计工作量**: 2-3 天
- **技术要点**:
  - 整合现有组件到统一工作流
  - 实现 Agent 生命周期管理
  - 处理错误和回滚逻辑
  - 添加日志和进度追踪

#### VC-011: 实现 Initializer Agent UI ✅ COMPLETED
- **状态**: 已完成 (TASK_COMPLETION_VC011.md)
- **实现文件**: `src/components/vibe-coding/CodingWorkspace.tsx` (InitializerWorkflow 组件)

#### VC-015: 实现 Agent Monitor 监控面板 ✅ COMPLETED
- **状态**: 已完成 (TASK_COMPLETION_AGENT_MONITOR.md)
- **实现文件**: `src/components/vibe-coding/CodingWorkspace.tsx` (AgentMonitor 组件)

#### VC-016: 实现 Progress Visualization 进度可视化 ✅ COMPLETED
- **状态**: 已完成 (TASK_COMPLETION_PROGRESS_VIZ.md)
- **实现文件**: `src/components/vibe-coding/CodingWorkspace.tsx` (ProgressVisualization 组件)

#### VC-017: 实现 LogTerminal 实时日志终端 ✅ COMPLETED
- **状态**: 已完成 (TASK_COMPLETION_LOG_TERMINAL.md)
- **实现文件**: `src/components/vibe-coding/CodingWorkspace.tsx` (LogTerminal 组件)

---

### Phase 3.3: Coding Agent 集群核心逻辑

#### VC-019: 实现代码生成提示词模板 ⭐⭐⭐ P0
- **类型**: Prompt Engineering
- **描述**: 创建针对不同场景的代码生成提示词库
- **优先级**: P0 (关键路径)
- **预计工作量**: 1-2 天
- **技术要点**:
  - React/TypeScript 组件生成提示词
  - Rust 后端 API 生成提示词
  - 数据库操作生成提示词
  - 测试代码生成提示词
  - 支持增量修改和全量生成模式

#### VC-020: 实现文件修改应用器 ⭐⭐⭐ P0
- **类型**: Backend + File System
- **描述**: 将 AI 生成的代码应用到实际文件
- **优先级**: P0 (关键路径)
- **预计工作量**: 2-3 天
- **技术要点**:
  - 文件读取和写入
  - 代码差异对比 (diff)
  - 安全备份机制
  - 批量文件操作
  - 错误恢复和回滚

#### VC-021: 实现代码审查提示词模板 P1
- **类型**: Prompt Engineering
- **描述**: 创建代码审查的提示词和评分标准
- **优先级**: P1
- **预计工作量**: 1 天
- **技术要点**:
  - 代码质量评分标准
  - 最佳实践检查清单
  - 性能和安全审查
  - 可读性评估

#### VC-022: 实现测试生成提示词模板 P1
- **类型**: Prompt Engineering
- **描述**: 自动生成单元测试和集成测试
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - Vitest 单元测试生成
  - React Testing Library 组件测试
  - Rust 集成测试生成
  - Mock 数据生成策略

---

### Phase 3.4: 文件与编辑器集成

#### VC-023: 实现文件树组件 File Explorer ✅ COMPLETED
- **状态**: 已完成 (FileExplorer.tsx)
- **实现文件**: `src/components/vibe-coding/FileExplorer.tsx`

#### VC-024: 实现代码编辑器集成 ⭐⭐⭐ P0
- **类型**: Frontend Integration
- **描述**: 集成 Monaco Editor 或 CodeMirror
- **优先级**: P0 (关键路径)
- **预计工作量**: 2-3 天
- **技术要点**:
  - Monaco Editor vs CodeMirror 选型
  - TypeScript/JavaScript 语法高亮
  - Rust 语法支持
  - 自动补全和智能提示
  - 多标签页编辑
  - 实时保存和格式化

#### VC-025: 实现 CP-002 任务分解审查界面 ✅ COMPLETED
- **状态**: 已完成 (CheckpointReview.tsx)
- **实现文件**: `src/components/vibe-coding/CheckpointReview.tsx`

#### VC-026: 实现差异查看器 (Diff Viewer) P1
- **类型**: Frontend Component
- **描述**: 展示代码修改前后对比
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - monaco-diff-editor 集成
  - 并排对比和行内对比模式
  - 差异统计和导航
  - 接受/拒绝单个变更

---

### Phase 3.5: 质量门禁系统 (Quality Gates)

#### VC-018: 实现 QG-001 代码检查 (ESLint) ✅ COMPLETED
- **状态**: 已完成

#### VC-027: 实现 QG-002 类型检查 (TypeScript) ⭐⭐⭐ P0
- **类型**: Backend + Tooling
- **描述**: 集成 tsc 进行类型检查
- **优先级**: P0 (关键路径)
- **预计工作量**: 1-2 天
- **技术要点**:
  - 调用 tsc --noEmit
  - 解析 TypeScript 编译器输出
  - 错误分类和严重性评级
  - 快速修复建议

#### VC-028: 实现 QG-003 格式化检查 (Prettier) P1
- **类型**: Backend + Tooling
- **描述**: 集成 Prettier 进行代码格式化检查
- **优先级**: P1
- **预计工作量**: 1 天
- **技术要点**:
  - Prettier CLI 集成
  - 检查 vs 修复模式
  - 配置文件支持
  - 批量格式化

#### VC-029: 实现 QG-004 测试覆盖率检查 P1
- **类型**: Backend + Tooling
- **描述**: 运行测试并检查覆盖率阈值
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - Vitest 覆盖率收集
  - 覆盖率阈值配置
  - 覆盖率报告生成
  - 历史覆盖率对比

---

### Phase 3.6: HITL 检查点 (Human-in-the-Loop)

#### VC-030: 定义 HITL 检查点协议 ⭐⭐⭐ P0
- **类型**: Protocol Design
- **描述**: 定义何时需要人工介入的标准
- **优先级**: P0 (关键路径)
- **预计工作量**: 1-2 天
- **技术要点**:
  - 检查点触发条件
  - 风险等级分类
  - 审批流程设计
  - 超时和默认行为

#### VC-031: 实现检查点触发机制 ⭐⭐⭐ P0
- **类型**: Backend Logic
- **描述**: 自动检测并触发检查点
- **优先级**: P0 (关键路径)
- **预计工作量**: 2-3 天
- **技术要点**:
  - 基于规则的触发器
  - 基于风险的触发器
  - 检查点状态管理
  - 通知和等待机制

#### VC-032: 实现用户审批界面 P1
- **类型**: Frontend Component
- **描述**: 用户审查和批准 AI 的决策
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - 检查点详情展示
  - 风险可视化
  - 批准/拒绝/修改操作
  - 批注和反馈

#### VC-033: 实现检查点历史记录 P2
- **类型**: Backend + Frontend
- **描述**: 记录和查询历史检查点
- **优先级**: P2
- **预计工作量**: 1 天
- **技术要点**:
  - 数据库表设计
  - 查询和过滤功能
  - 统计和报表

#### VC-034: 实现批量审批功能 P2
- **类型**: Frontend Feature
- **描述**: 支持批量批准类似决策
- **优先级**: P2
- **预计工作量**: 1 天
- **技术要点**:
  - 规则匹配和分组
  - 批量操作 UI
  - 确认和撤销

#### VC-035: 实现审批规则学习 P3
- **类型**: ML/AI Feature
- **描述**: 从用户历史审批中学习偏好
- **优先级**: P3 (可选)
- **预计工作量**: 3-5 天
- **技术要点**:
  - 用户行为分析
  - 规则提取和泛化
  - 自动审批推荐

---

### Phase 3.7: MR Creation (Merge Request)

#### VC-036: 实现 MR 创建功能 P1
- **类型**: Git Integration
- **描述**: 自动创建 Git Merge Request
- **优先级**: P1
- **预计工作量**: 2-3 天
- **技术要点**:
  - GitLab/GitHub API 集成
  - 分支管理和推送
  - MR 模板支持
  - Reviewer 指派

#### VC-037: 实现 MR 描述生成 P1
- **类型**: AI Generation
- **描述**: 基于代码变更生成 MR 描述
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - 变更摘要生成
  - 影响分析
  - 测试覆盖说明
  - 截图和演示建议

#### VC-038: 实现代码变更摘要 P2
- **类型**: Analysis
- **描述**: 自动生成变更影响分析
- **优先级**: P2
- **预计工作量**: 1 天
- **技术要点**:
  - 文件变更统计
  - 依赖影响分析
  - API 变更检测

#### VC-039: 实现 CI/CD 集成 P2
- **类型**: DevOps Integration
- **描述**: 与 CI 流水线集成
- **优先级**: P2
- **预计工作量**: 2-3 天
- **技术要点**:
  - GitHub Actions / GitLab CI 集成
  - 流水线状态监控
  - 失败重试和通知

---

## 🔵 AI 适配器模块 (5 个任务)

**状态**: 0/5 完成，待接入真实 AI API

#### AI-ADAPTER-001: 实现真实 AI API 调用 ⭐⭐⭐ P0
- **类型**: Backend Integration
- **描述**: 替换 Mock 数据，接入真实 OpenAI/Anthropic API
- **优先级**: P0 (关键路径)
- **预计工作量**: 2-3 天
- **技术要点**:
  - OpenAI API 集成 (GPT-4, GPT-3.5)
  - Anthropic API 集成 (Claude)
  - Kimi API 集成 (Moonshot AI)
  - GLM API 集成 (智谱 AI)
  - API 密钥管理和轮换
  - 请求超时和错误处理

#### AI-ADAPTER-002: 实现流式响应处理 ⭐⭐⭐ P0
- **类型**: Backend + Frontend
- **描述**: 处理 SSE 流式输出
- **优先级**: P0 (关键路径)
- **预计工作量**: 1-2 天
- **技术要点**:
  - Server-Sent Events (SSE) 实现
  - 前端流式接收和渲染
  - 打字机效果
  - 中断和取消

#### AI-ADAPTER-003: 实现错误重试机制 P1
- **类型**: Backend Resilience
- **描述**: 网络错误自动重试
- **优先级**: P1
- **预计工作量**: 1 天
- **技术要点**:
  - 指数退避策略
  - 最大重试次数
  - 错误分类和处理
  - 用户友好错误消息

#### AI-ADAPTER-004: 实现 Token 计数和限流 P1
- **类型**: Backend Management
- **描述**: 跟踪 Token 使用并实施限流
- **优先级**: P1
- **预计工作量**: 1-2 天
- **技术要点**:
  - Token 使用统计
  - 速率限制实现
  - 配额管理
  - 成本估算和预警

#### AI-ADAPTER-005: 实现多模型路由 P2
- **类型**: Backend Intelligence
- **描述**: 根据任务类型选择最优模型
- **优先级**: P2
- **预计工作量**: 1-2 天
- **技术要点**:
  - 任务分类器
  - 模型能力矩阵
  - 成本和延迟优化
  - A/B 测试框架

---

## 📈 优先级排序和建议执行顺序

### P0 最高优先级 (关键路径 - 9 个任务) ⭐⭐⭐

这些任务是解锁后续功能的基础，必须优先完成:

1. **VC-010**: Initializer Agent 主逻辑整合
   - 原因：整合所有已完成的子组件，形成完整工作流
   - 依赖：✅ VC-006, VC-007, VC-008, VC-009 已完成

2. **VC-019**: 代码生成提示词模板
   - 原因：Coding Agent 的核心能力

3. **VC-020**: 文件修改应用器
   - 原因：将 AI 输出应用到实际文件

4. **VC-024**: 代码编辑器集成
   - 原因：用户体验的关键

5. **VC-027**: TypeScript 类型检查
   - 原因：质量门禁的基础

6. **VC-030**: HITL 检查点协议
   - 原因：人机协作的核心机制

7. **VC-031**: 检查点触发机制
   - 原因：实现自动化的关键

8. **AI-ADAPTER-001**: 真实 AI API 调用
   - 原因：整个系统的 AI 能力基础

9. **AI-ADAPTER-002**: 流式响应处理
   - 原因：提升用户体验

### P1 高优先级 (10 个任务) ⭐⭐

这些任务重要但不阻塞关键路径:

10. **VC-021**: 代码审查提示词模板
11. **VC-022**: 测试生成提示词模板
12. **VC-026**: 差异查看器
13. **VC-028**: Prettier 格式化检查
14. **VC-029**: 测试覆盖率检查
15. **VC-032**: 用户审批界面
16. **VC-036**: MR 创建功能
17. **VC-037**: MR 描述生成
18. **AI-ADAPTER-003**: 错误重试机制
19. **AI-ADAPTER-004**: Token 计数和限流

### P2 中低优先级 (5 个任务) ⭐

这些任务可以延后或在时间充裕时完成:

20. **VC-033**: 检查点历史记录
21. **VC-034**: 批量审批功能
22. **VC-038**: 代码变更摘要
23. **VC-039**: CI/CD 集成
24. **AI-ADAPTER-005**: 多模型路由

### P3 可选任务 (1 个任务)

25. **VC-035**: 审批规则学习
   - 原因：ML 功能，开发周期长，非 MVP 必需

---

## 🎯 下一步行动计划

### Week 3-4: 核心功能突破 (预计 2 周)

**目标**: 完成所有 P0 任务，实现端到端的 Vibe Coding 流程

**任务分配**:
- Week 3: VC-010, VC-019, VC-020, VC-027 (后端重点)
- Week 4: VC-024, VC-030, VC-031, AI-ADAPTER-001, AI-ADAPTER-002 (前后端集成)

**预期成果**:
- ✅ Initializer Agent 可运行完整工作流
- ✅ Coding Agent 能生成和应用代码
- ✅ 基础质量门禁生效
- ✅ HITL 机制可用
- ✅ 真实 AI API 集成

### Week 5-6: 功能完善和测试 (预计 2 周)

**目标**: 完成 P1 任务，提升产品质量

**任务分配**:
- Week 5: VC-021, VC-022, VC-026, VC-028, VC-029 (质量工具)
- Week 6: VC-032, VC-036, VC-037, AI-ADAPTER-003, AI-ADAPTER-004 (用户体验)

**预期成果**:
- ✅ 完整的代码审查和测试生成
- ✅ 差异查看器和审批界面
- ✅ MR 自动创建
- ✅ 错误处理和限流机制

### Week 7: 收尾和发布准备 (预计 1 周)

**目标**: 选择性完成 P2/P3 任务，准备 MVP 发布

**任务分配**:
- 选择性完成 2-3 个 P2 任务
- 整体测试和 Bug 修复
- 文档完善
- 性能优化

---

## 📝 任务状态追踪

| 任务 ID | 名称 | 优先级 | 状态 | 负责人 | 开始日期 | 完成日期 |
|---------|------|--------|------|--------|----------|----------|
| VC-010 | Initializer Agent 主逻辑 | P0 | 📋 Pending | - | - | - |
| VC-019 | 代码生成提示词模板 | P0 | 📋 Pending | - | - | - |
| VC-020 | 文件修改应用器 | P0 | 📋 Pending | - | - | - |
| VC-024 | 代码编辑器集成 | P0 | 📋 Pending | - | - | - |
| VC-027 | TypeScript 类型检查 | P0 | 📋 Pending | - | - | - |
| VC-030 | HITL 检查点协议 | P0 | 📋 Pending | - | - | - |
| VC-031 | 检查点触发机制 | P0 | 📋 Pending | - | - | - |
| AI-ADAPTER-001 | 真实 AI API 调用 | P0 | 📋 Pending | - | - | - |
| AI-ADAPTER-002 | 流式响应处理 | P0 | 📋 Pending | - | - | - |
| VC-021 | 代码审查提示词模板 | P1 | 📋 Pending | - | - | - |
| ... | ... | ... | ... | ... | ... | ... |

*注：完整表格见上方详细列表*

---

## 🔗 相关链接

- [MVP版本规划](../product-specs/mvp-roadmap.md) - 原始任务来源
- [HARNESS_ENGINEERING.md](./HARNESS_ENGINEERING.md) - 工程规范
- [tech-debt-tracker.md](./tech-debt-tracker.md) - 技术债务追踪

---

**文档维护**: 每次完成任务后，请及时更新此文档并将任务移至 `completed/` 目录。

**最后更新**: 2026-03-25

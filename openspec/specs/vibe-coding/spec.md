## ADDED Requirements

### Requirement: AI 驱动的自主编码系统
系统 SHALL 提供 AI 驱动的自主编码能力,通过多会话编排和 HITL 检查点机制,实现从 PRD 到可部署代码的全流程自动化。

**核心理念**: "人类掌舵,AI 执行" (Humans steer. AI execute.)

#### Scenario: 从 PRD 生成完整项目
- **WHEN** 用户提供完整的 PRD 文档
- **THEN** 系统自动分解任务、创建 Issues、并行编码、生成 MR

#### Scenario: HITL 检查点人工审查
- **WHEN** Coding Agent 到达关键检查点(如架构决策、API 设计)
- **THEN** 系统暂停并等待用户确认或修改建议

### Requirement: 多会话编排架构
系统 SHALL 支持三种 Agent 类型的协同工作:
- **Initializer Agent**: 环境初始化、任务分解 (5-10 分钟)
- **Coding Agent**: 具体 Issue 实现,可并行运行多个 (15-30 分钟/Issue)
- **MR Creation Agent**: 汇总提交,创建合并请求 (10-20 分钟)

#### Scenario: 并行执行多个 Coding Agents
- **WHEN** Initializer 创建了 5 个独立 Issues
- **THEN** 系统启动 5 个 Coding Agents 并行处理

#### Scenario: Initializer 任务分解
- **WHEN** 接收到 PRD 文档
- **THEN** Initializer Agent 解析 PRD,创建 Milestones 和 Issues,初始化 Git 仓库

### Requirement: HITL 检查点机制
系统 SHALL 实现 8 个关键检查点 (Human-in-the-Loop),在关键时刻暂停并等待用户审查:
- **CP-001**: Initializer 开始前 - 项目目录验证、Git 仓库检查 (< 1min)
- **CP-002**: 任务分解后 - Issue 列表质量、优先级合理性 (2-5min)
- **CP-003**: Issue 丰富化后 - 上下文信息完整性 (1-2min)
- **CP-004**: 编码会话前 - 回归测试结果审查 (1-3min)
- **CP-005**: 选择下一个 Issue - Issue 优先级确认 (< 1min)
- **CP-006**: Issue 完成后 - 实现完整性、测试覆盖率 (2-5min)
- **CP-007**: 所有 Issue 完成后 - 是否进入 MR 创建阶段 (1-2min)
- **CP-008**: MR 创建后 - MR 描述、变更内容审查 (3-10min)

#### Scenario: 自动接受检查点
- **WHEN** 用户配置信任度 > 90% 且满足自动接受条件
- **THEN** 系统自动通过检查点,无需人工干预

#### Scenario: 检查点拒绝处理
- **WHEN** 用户在检查点拒绝当前结果并提供反馈
- **THEN** Agent 根据反馈重新生成或调整方案

### Requirement: 质量门禁与监控
系统 MUST 在每个阶段执行多层次质量检查:
- **QG-001**: 代码检查 (ESLint/Prettier/Ruff) - 最多 3 次修复尝试
- **QG-002**: 类型检查 (TypeScript/Pyright) - 最多 3 次修复尝试
- **QG-003**: 单元测试 (npm test/pytest) - 覆盖率 ≥ 70%
- **QG-004**: 回归测试 (E2E 测试验证) - 失败后暂停并通知
- **QG-005**: E2E 测试 (Puppeteer 浏览器自动化) - 失败后暂停并通知

**失败恢复策略**:
1. 第一次失败：自动分析错误并尝试修复
2. 第二次失败：回滚最近提交，重新生成
3. 第三次失败：跳过该 Issue，记录警告
4. 超过限制：暂停并通知用户介入

#### Scenario: 自动修复质量问题
- **WHEN** Coding Agent 生成的代码未通过质量检查
- **THEN** 系统自动尝试修复,最多重试 3 次

#### Scenario: 质量检查失败处理
- **WHEN** 自动修复 3 次后仍未通过
- **THEN** 系统标记 Issue 为失败状态,通知用户人工介入

### Requirement: 实时日志流式输出
系统 SHALL 提供实时的 Agent 执行日志流式输出,包括:
- Token 级别的 AI 响应流
- 文件操作日志(创建、修改、删除)
- 命令执行日志(编译、测试、lint)
- 错误和警告信息

#### Scenario: 前端实时显示日志
- **WHEN** Agent 正在执行任务
- **THEN** 前端界面实时滚动显示日志,支持暂停/恢复

#### Scenario: 日志分类过滤
- **WHEN** 用户选择查看特定类型的日志
- **THEN** 系统过滤显示 ai-response、file-ops、commands 或 errors

### Requirement: 进度追踪与可视化
系统 MUST 提供多维度的进度追踪:
- 总体进度百分比
- 每个 Milestone 的完成状态
- 每个 Issue 的详细状态(pending/in-progress/completed/failed)
- 实时统计(已完成文件数、测试通过率等)

#### Scenario: 可视化进度仪表板
- **WHEN** 用户打开 Vibe Coding 界面
- **THEN** 显示进度条、Milestone 列表、Issues 状态表格

#### Scenario: 失败任务重试
- **WHEN** 某个 Issue 执行失败
- **THEN** 用户可以点击重试按钮重新执行该 Issue

### Requirement: 代码生成与文件管理
系统 SHALL 支持:
- 自动生成项目结构和配置文件
- 创建、修改、删除源代码文件
- 生成单元测试和 E2E 测试
- 维护 import/export 依赖关系

#### Scenario: 生成 React + TypeScript 组件
- **WHEN** Coding Agent 实现前端页面
- **THEN** 生成 .tsx 文件、对应的 .test.tsx、CSS module

#### Scenario: 保持代码一致性
- **WHEN** 修改共享类型定义
- **THEN** 系统自动更新所有引用该类型的文件

### Requirement: 性能指标追踪
系统 MUST 记录和分析关键性能指标:
- AI 编码成功率 (目标: > 85%)
- 检查点通过率 (目标: > 70%)
- 代码质量分数 (目标: > 95%)
- 平均完成时间 (目标: < 4 小时)
- 部署成功率 (目标: > 95%)

#### Scenario: 生成性能报告
- **WHEN** Vibe Coding 会话完成
- **THEN** 系统生成本次会话的性能指标报告

#### Scenario: 历史数据分析
- **WHEN** 用户查看历史会话列表
- **THEN** 显示每次会话的关键指标和趋势图

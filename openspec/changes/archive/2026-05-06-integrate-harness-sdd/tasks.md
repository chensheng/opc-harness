## 1. 创建新的 Capability Spec

- [x] 1.1 创建 `harness-sdd-integration` spec,定义整合开发模式的核心需求
- [x] 1.2 在 spec 中明确 ADR 与 OpenSpec Change 的双向引用机制
- [x] 1.3 在 spec 中定义 SDD 更新决策矩阵
- [x] 1.4 在 spec 中明确 SDD 在 Harness 流程各阶段的集成点

## 2. 更新 Development Workflow Spec

- [x] 2.1 在 `development-workflow/spec.md` 的 "Harness Engineering 开发流程" 中增加 SDD 影响评估场景
- [x] 2.2 新增 "SDD 文档维护" Requirement,明确三种场景的更新策略
- [x] 2.3 新增 "ADR 创建规范" Requirement,定义需要创建 ADR 的场景
- [x] 2.4 确保所有 Requirements 都有对应的 Scenarios

## 3. 更新 Design Documentation Spec

- [x] 3.1 在 `design-documentation/spec.md` 中新增 "SDD 更新决策矩阵" Requirement
- [x] 3.2 修改 "架构决策记录 (ADR)" Requirement,增加 "Related Changes" 字段要求
- [x] 3.3 修改 "设计文档与 OpenSpec 集成" Requirement,增加双向引用机制
- [x] 3.4 确保所有新增和修改的 Requirements 都有对应的 Scenarios

## 4. 更新 AGENTS.md 导航文档

- [x] 4.1 精简 "📐 SDD 软件设计文档" 章节,保留关键要点和链接
- [x] 4.2 在 "🏗️ 三大支柱" 的 "架构约束" 部分强调 SDD 的作用
- [x] 4.3 在 "❓ 常见问题" 中新增 "Harness 与 SDD 如何协同?" 问答
- [x] 4.4 更新快速入口部分,确保链接指向正确的 specs

## 5. 验证文档一致性

- [x] 5.1 运行 `npm run harness:check` 验证整体健康度
- [x] 5.2 检查所有新增的 specs 格式是否符合 OpenSpec 规范
- [x] 5.3 验证 proposal/design/specs/tasks 之间的一致性
- [x] 5.4 确认所有 scenarios 使用正确的 `#### Scenario:` 格式

## 6. 准备归档

- [x] 6.1 创建 quality-check.md,记录文档变更的质量检查结果
- [x] 6.2 创建 runtime-check.md,说明如何验证新规范的执行
- [x] 6.3 确认所有任务已完成并标记为 `[x]`
- [x] 6.4 准备归档命令 `/opsx:archive integrate-harness-sdd`

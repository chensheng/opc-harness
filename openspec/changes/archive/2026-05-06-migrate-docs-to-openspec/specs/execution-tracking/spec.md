## ADDED Requirements

### Requirement: 执行计划生命周期
执行计划 MUST 遵循以下生命周期:
1. **创建** (active/): 新计划在此阶段制定
2. **执行** (active/): 开发过程中持续更新进度
3. **完成** (completed/): 任务全部完成后移动到此
4. **归档** (openspec/changes/archive/): 最终归档到 OpenSpec

#### Scenario: 创建新执行计划
- **WHEN** 开发者开始 US-001 任务
- **THEN** 应在 `docs/exec-plans/active/` 创建 `US-001-agent-communication-protocol.md`

#### Scenario: 完成执行计划
- **WHEN** 所有任务标记为完成且测试通过
- **THEN** 应将计划从 `active/` 移动到 `completed/`

### Requirement: 执行计划模板
所有执行计划 MUST 使用标准模板 (`docs/exec-plans/templates/`),包含:
- **任务信息**: ID、名称、优先级、预估工作量
- **执行步骤**: 详细的实施阶段和子任务
- **验收标准**: 如何验证任务完成
- **质量检查**: 必须通过的测试和检查项
- **实际耗时**: 记录实际花费时间用于估算改进

#### Scenario: 使用模板创建计划
- **WHEN** 开发者运行 `npm run harness:new-plan`
- **THEN** 系统应从模板生成新的执行计划文件

### Requirement: 执行计划与 OpenSpec Tasks 同步
执行计划中的任务 SHOULD 与 OpenSpec `tasks.md` 保持一致:
- OpenSpec tasks 是实施级别的 checklist
- 执行计划是项目管理级别的追踪
- 两者应相互引用,避免重复维护

#### Scenario: 追踪任务进度
- **WHEN** 查看 OpenSpec change 的 `tasks.md`
- **THEN** 应看到与执行计划对应的任务分解和完成状态

### Requirement: 技术债务追踪
系统 SHALL 支持技术债务的记录和追踪:
- 在 `docs/exec-plans/tech-debts/` 记录已知技术债务
- 每个债务项包含: 描述、影响、优先级、修复计划
- 定期回顾并优先处理高优先级债务

#### Scenario: 记录技术债务
- **WHEN** 发现某模块测试覆盖率仅 50%
- **THEN** 应在 `tech-debts/` 创建记录,标记为 P1 优先级

### Requirement: 执行计划索引
系统 MUST 维护执行计划索引 (`docs/exec-plans/index.md`):
- 列出所有 active 和 recent completed 计划
- 按优先级和状态排序
- 提供快速链接到各个计划

#### Scenario: 查找进行中的任务
- **WHEN** 团队成员查看 `docs/exec-plans/index.md`
- **THEN** 应看到当前所有 active 任务列表和负责人

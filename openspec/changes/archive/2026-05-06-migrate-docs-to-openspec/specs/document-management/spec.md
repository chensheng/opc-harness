## ADDED Requirements

### Requirement: 文档分类体系
系统 SHALL 提供清晰的文档分类体系,将文档分为以下类别:
- **设计文档** (design-docs): 架构设计、决策记录、系统规范
- **产品规格** (product-specs): 产品功能定义、用户故事、能力边界
- **执行计划** (exec-plans): 任务分解、实施步骤、验收标准
- **Sprint 计划** (sprint-plans): 迭代规划、任务分配、进度跟踪
- **参考文档** (references): 最佳实践、集成指南、教程

#### Scenario: 开发者查找架构约束
- **WHEN** 开发者需要查看架构约束规则
- **THEN** 系统应在 `docs/design-docs/architecture-rules.md` 提供完整规则列表

#### Scenario: 产品经理查看产品规格
- **WHEN** 产品经理需要了解 vibe-coding 模块的功能定义
- **THEN** 系统应在 `docs/product-specs/vibe-coding-spec.md` 提供详细规格

### Requirement: 文档命名规范
所有文档 MUST 使用 kebab-case 命名,文件扩展名为 `.md`。目录名同样遵循 kebab-case。

#### Scenario: 创建新设计文档
- **WHEN** 团队创建新的架构决策记录
- **THEN** 文件名应为 `YYYY-MM-DD-<decision-title>.md` 格式,如 `2026-05-06-migrate-docs-to-openspec.md`

### Requirement: 文档归档策略
系统 SHALL 支持文档归档机制,将已完成的历史文档移动到归档位置:
- 已完成的执行计划 → `openspec/changes/archive/YYYY-MM-DD-<name>/`
- 历史决策记录 → `openspec/changes/archive/YYYY-MM-DD-<name>/`
- 过时的 Sprint 计划 → `docs/sprint-plans/archive/`

#### Scenario: 归档执行计划
- **WHEN** 执行计划标记为完成
- **THEN** 系统应将其从 `docs/exec-plans/active/` 移动到 `docs/exec-plans/completed/`,最终归档到 `openspec/changes/archive/`

### Requirement: 文档引用完整性
所有文档内部链接 MUST 保持有效。迁移或重命名文档时,系统 SHALL 提供引用检查工具验证链接有效性。

#### Scenario: 检测断裂链接
- **WHEN** 运行文档质量检查
- **THEN** 系统应扫描所有 markdown 文件,报告无效的相对路径链接

#### Scenario: 迁移后更新引用
- **WHEN** 文档从 `docs/` 移动到 `openspec/`
- **THEN** 系统应提供脚本批量更新相关文档中的引用路径

### Requirement: 文档索引导航
系统 SHALL 在每个文档目录下提供 `index.md` 作为导航入口,列出该目录下的所有文档及其简要描述。

#### Scenario: 浏览设计文档
- **WHEN** 开发者访问 `docs/design-docs/index.md`
- **THEN** 应看到架构文档、决策记录的列表和链接

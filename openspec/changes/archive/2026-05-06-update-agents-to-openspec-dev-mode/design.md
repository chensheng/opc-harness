## Context

AGENTS.md 已在 `remove-docs-to-openspec` change 中更新为 OpenSpec 导航,包含:
- ✅ 完整的 18 capabilities 列表
- ✅ OpenSpec specs 和 archive 的链接
- ✅ 基本的开发流程说明

**当前不足**:
- ❌ 缺少实际的 `/opsx:` 命令使用示例
- ❌ 没有详细的 step-by-step 快速入门指南
- ❌ 缺少最佳实践和常见问题解答
- ❌ explore mode vs apply mode 的使用场景不清晰

**约束**:
- 这是纯文档增强,不涉及代码修改
- 需要保持与 README.md 的一致性(README 已有基础 OpenSpec 介绍)
- AGENTS.md 应该作为权威参考,比 README 更详细

## Goals / Non-Goals

**Goals**:
- 添加 OpenSpec 快速入门章节,提供实际操作指南
- 补充完整的命令示例和工作流程说明
- 添加最佳实践部分,指导何时使用不同模式
- 增加 FAQ 章节,解答常见问题
- 优化 capabilities 导航,提供更清晰的分类
- **补充 SDD (软件设计文档) 内容**,说明 SDD 标准结构、ADR 编写指南、与 OpenSpec 的关系

**Non-Goals**:
- 不改变现有的 capabilities 结构
- 不修改 openspec/specs/ 中的能力规范
- 不改变 Harness Engineering 的核心理念

## Decisions

### Decision 1: 快速入门章节结构

**选择**: 添加 "🚀 OpenSpec 快速入门" 章节,位于 "🎯 快速入口" 之后

**内容结构**:
1. 前置要求 (安装 OpenSpec CLI)
2. 第一个变更 (step-by-step 示例)
3. 完整工作流概览
4. 常用命令速查

**理由**:
- 新贡献者最需要的是实际操作指南
- Step-by-step 示例比抽象描述更容易理解
- 放在靠前位置确保可见性

### Decision 2: 最佳实践内容

**选择**: 添加 "💡 OpenSpec 最佳实践" 章节

**内容包括**:
- 何时使用 `/opsx:propose` vs `/opsx:explore`
- 变更粒度建议 (小而频繁的变更)
- Spec 编写技巧 (ADDED vs MODIFIED)
- 归档时机和注意事项

**理由**:
- 帮助开发者避免常见错误
- 提高变更质量和可维护性
- 建立团队共识

### Decision 3: FAQ 章节

**选择**: 添加 "❓ 常见问题" 章节

**典型问题**:
- "如何开始一个新特性?"
- "我的变更需要 spec 吗?"
- "如何处理冲突的变更?"
- "归档后还能修改吗?"

**理由**:
- 减少重复性问题
- 加速问题解决
- 作为自助服务资源

### Decision 4: SDD (软件设计文档) 内容

**选择**: 在 AGENTS.md 中添加 "📐 SDD 软件设计文档" 章节

**内容包括**:
1. **SDD 标准结构**:
   - 系统概述 (System Overview)
   - 分层架构 (Layered Architecture)
   - 数据流规则 (Data Flow Rules)
   - 技术栈说明 (Technology Stack)
   - 组件交互图 (Component Interaction)

2. **ADR (架构决策记录) 编写指南**:
   - ADR 模板 (标题、状态、背景、决策、后果)
   - 何时创建 ADR (重大技术选型、架构变更)
   - ADR 生命周期管理 (Proposed → Accepted → Deprecated)

3. **SDD 与 OpenSpec 的关系**:
   - SDD = 长期稳定的架构文档
   - OpenSpec design.md = 特定变更的设计说明
   - 两者互补: SDD 定义整体架构,OpenSpec design.md 描述具体实现

4. **示例和引用**:
   - 链接到 `design-documentation` capability spec
   - 引用已归档的架构变更作为示例

**理由**:
- SDD 是软件工程的标准实践,帮助团队理解系统整体设计
- ADR 记录重要决策的历史和 rationale
- 明确 SDD 与 OpenSpec 的分工,避免混淆
- 为新贡献者提供清晰的架构理解路径

### Decision 5: development-workflow spec 更新

**选择**: 仅在 proposal 中明确列出时才更新 spec

**当前决策**: 本次主要是 AGENTS.md 文档增强,development-workflow spec 的核心需求未改变,因此不需要更新 spec。

**理由**:
- Spec 定义稳定的能力需求,而非文档细节
- AGENTS.md 是用户指南,spec 是能力规范
- 避免不必要的 spec 变更

## Risks / Trade-offs

### Risk 1: 信息重复
**风险**: AGENTS.md 和 README.md 可能有重复的 OpenSpec 介绍  
**缓解**:
- README 保持简洁,提供概览
- AGENTS.md 作为详细参考,包含完整指南
- 明确分工: README = "是什么", AGENTS.md = "怎么做"

### Risk 2: 文档过时
**风险**: OpenSpec 工作流演进导致文档过时  
**缓解**:
- 建立文档审查机制
- 每次 OpenSpec 重大更新时同步检查
- 鼓励社区贡献和改进

### Trade-off: 详细 vs 简洁
**权衡**: AGENTS.md 应该在详细和简洁之间平衡  
**决策**: 
- 优先详细,因为 AGENTS.md 是权威参考
- 使用清晰的章节结构和导航
- 提供快速扫描的摘要和深入阅读的链接

## Migration Plan

由于这是纯文档增强,迁移计划很简单:

1. **更新 AGENTS.md** - 添加新章节和内容
2. **验证链接** - 确保所有内部链接有效
3. **格式化** - 运行 Prettier 确保格式一致
4. **提交变更** - Git commit 并推送

**Rollback Strategy**: 
- 如果发现问题,可以通过 Git revert 轻松回滚
- 无数据迁移或配置变更,回滚风险低

## Open Questions

None - 这是一个明确的文档增强任务,没有未决的技术决策。

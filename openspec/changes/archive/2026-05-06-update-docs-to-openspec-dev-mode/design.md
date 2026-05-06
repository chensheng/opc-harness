## Context

项目最近完成了从 `docs/` 目录到 OpenSpec 工作流的全面迁移(`remove-docs-migrate-to-openspec` change)。然而,README.md 仍然保留着旧的开发流程描述,没有反映新的 OpenSpec 工作方式。

**当前状态**:
- ✅ AGENTS.md 已更新为完整的 OpenSpec 导航(18 capabilities)
- ✅ docs/ 目录已删除,所有内容迁移到 OpenSpec
- ❌ README.md 仍引用旧的开发模式,缺少 OpenSpec 工作流说明
- ⚠️ development-workflow capability spec 可能需要更新以强调 OpenSpec

**约束**:
- 这是纯文档变更,不涉及代码修改
- 需要保持与 AGENTS.md 的一致性
- README 应该简洁明了,适合作为项目入口文档

## Goals / Non-Goals

**Goals**:
- 更新 README.md 以反映 OpenSpec 开发工作流
- 添加 `/opsx:` 命令的快速入门示例
- 强化 Harness Engineering 与 OpenSpec 的关联
- 确保新贡献者能够快速理解并使用正确的开发流程

**Non-Goals**:
- 不修改 AGENTS.md (已在之前迁移中完成)
- 不改变项目的技术架构或功能
- 不添加新的 capabilities 或 specs

## Decisions

### Decision 1: README 结构优化

**选择**: 在 README 中添加专门的 "开发工作流" 章节,突出 OpenSpec

**理由**:
- README 是项目的第一印象,应该清晰展示核心特色
- OpenSpec 是项目的创新点,值得在显眼位置展示
- 分离"快速开始"(安装运行)和"开发工作流"(贡献代码)

**替代方案**:
- ❌ 仅在 Harness Engineering 章节中提及 - 不够突出
- ❌ 完全重写 README - 工作量过大,可能破坏现有信息架构

### Decision 2: OpenSpec 命令示例

**选择**: 提供实际的命令示例,展示完整的变更生命周期

**示例内容**:
```bash
# 1. 创建变更
/opsx:propose add-new-feature

# 2. 实施任务
/opsx:apply add-new-feature

# 3. 归档变更
/opsx:archive add-new-feature
```

**理由**:
- 实际示例比抽象描述更容易理解
- 展示完整的工作流,帮助新贡献者建立心智模型
- 降低学习曲线

### Decision 3: development-workflow spec 更新

**选择**: 仅当 proposal 中明确列出的 modified capabilities 才创建 delta spec

**当前决策**: 本次变更主要是 README 更新,development-workflow spec 的核心需求(测试先行、架构约束等)未改变,因此不需要更新 spec。

**理由**:
- Spec 应该定义稳定的能力需求,而非实现细节
- README 是用户文档,spec 是能力规范,两者职责不同
- 避免不必要的 spec 变更

## Risks / Trade-offs

### Risk 1: 文档过时风险
**风险**: OpenSpec 工作流本身可能演进,导致 README 再次过时  
**缓解**: 
- 在 README 中添加指向 AGENTS.md 的链接作为权威参考
- 建立文档审查机制,每次 OpenSpec 重大更新时同步检查 README

### Risk 2: 信息重复
**风险**: README 和 AGENTS.md 可能有重复内容  
**缓解**:
- README 保持简洁,提供概览和快速入门
- AGENTS.md 作为详细参考,包含完整的 capabilities 列表和架构约束
- 明确分工:README = "是什么",AGENTS.md = "怎么做"

### Trade-off: 简洁 vs 完整
**权衡**: README 应该在简洁和完整之间平衡  
**决策**: 优先简洁,提供足够的信息让新贡献者开始工作,详细信息引导至 AGENTS.md

## Migration Plan

由于这是纯文档变更,迁移计划很简单:

1. **更新 README.md** - 添加 OpenSpec 工作流章节
2. **验证链接** - 确保所有内部链接有效
3. **提交变更** - Git commit 并推送
4. **通知团队** - 告知团队成员文档已更新

**Rollback Strategy**: 
- 如果发现问题,可以通过 Git revert 轻松回滚
- 无数据迁移或配置变更,回滚风险低

## Open Questions

None - 这是一个明确的文档更新任务,没有未决的技术决策。

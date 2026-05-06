## Context

当前 `openspec/changes/archive/` 目录中有 9 个归档,其中 4 个不符合 harness-quality schema 规范:

**不符合规范的归档**:
1. `2026-05-06-adrs-batch-1` - 只有 proposal.md + ADR 文件
2. `2026-05-06-exec-plans-batch-1` - 只有 proposal.md + 用户故事文件
3. `2026-05-06-improve-ai-agent-observability-in-vibe-coding` - 缺少 quality-check.md, runtime-check.md
4. `2026-05-06-init-git-on-project-creation` - 有非标准文件,缺少质量验证文档

**符合规范的归档**:
- `2026-05-06-complete-user-story-failed-status` - 完整 artifacts
- `2026-05-06-migrate-docs-to-openspec` - 完整 artifacts
- `2026-05-06-remove-docs-migrate-to-openspec` - 完整 artifacts
- `2026-05-06-update-agents-to-openspec-dev-mode` - 完整 artifacts
- `2026-05-06-update-docs-to-openspec-dev-mode` - 完整 artifacts

**约束**:
- 这是清理操作,不涉及代码修改
- 删除的归档可以通过 Git history 恢复(如果需要)
- 需要确保保留的归档都符合标准

## Goals / Non-Goals

**Goals**:
- 删除 4 个不符合 harness-quality schema 的归档
- 确保所有剩余归档都包含完整的 artifacts
- 提高项目文档的一致性和专业性

**Non-Goals**:
- 不修复或删除这些归档中的内容(直接删除整个目录)
- 不影响符合规范的归档
- 不修改 harness-quality schema 本身

## Decisions

### Decision 1: 删除策略

**选择**: 直接删除整个归档目录

**理由**:
- 这些归档本身就是不完整的,没有保留价值
- 如果未来需要参考,可以从 Git history 中恢复
- 简化清理流程,避免部分删除导致的混乱

### Decision 2: 是否备份

**选择**: 不单独备份,依赖 Git history

**理由**:
- Git 已经记录了所有文件的历史
- 单独备份会增加复杂度
- 这些归档本身就不符合规范,不应该作为参考

### Decision 3: 是否需要迁移内容

**选择**: 不迁移任何内容

**理由**:
- `adrs-batch-1` 中的 ADR 文件应该存储在专门的位置,而不是 OpenSpec archive
- `exec-plans-batch-1` 中的用户故事文件也应该有专门的存储位置
- 这些内容的正确归属不在本次变更范围内

### Decision 4: 验证保留的归档

**选择**: 在删除前验证保留的归档都符合规范

**验证标准**:
- 必须包含: proposal.md, design.md, tasks.md, quality-check.md, runtime-check.md
- 可选: specs/ (如果有 modified capabilities)
- 不应包含非标准文件

## Risks / Trade-offs

### Risk 1: 误删有价值的内容

**风险**: 可能删除了未来需要参考的历史记录  
**缓解**:
- 所有内容都在 Git history 中,可以随时恢复
- 只删除明显不符合规范的归档
- 保留所有符合 harness-quality schema 的归档

### Risk 2: Git history 混乱

**风险**: 大量文件删除可能导致 Git history 难以追踪  
**缓解**:
- 使用清晰的 commit message 说明删除原因
- 在 proposal.md 中详细记录被删除的归档列表

### Trade-off: 历史完整性 vs 文档质量

**权衡**: 保留所有历史记录 vs 保持文档质量标准  
**决策**: 
- 优先文档质量和一致性
- Git history 仍然保留完整记录
- 新贡献者看到的是高质量的标准示例

## Migration Plan

由于这是纯删除操作,迁移计划很简单:

1. **验证保留的归档** - 确认它们都符合 harness-quality schema
2. **删除不符合规范的归档** - 使用 `Remove-Item` 删除 4 个目录
3. **提交变更** - Git commit 并推送
4. **验证结果** - 确认只剩符合规范的归档

**Rollback Strategy**: 
- 如果发现问题,可以通过 Git revert 恢复被删除的目录
- 或者从 Git history 中手动恢复特定文件

## Open Questions

None - 这是一个明确的清理任务,没有未决的技术决策。

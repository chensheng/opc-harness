## Why

当前 `openspec/changes/archive/` 目录中存在多个不符合 harness-quality schema 规范的归档变更。这些不完整的归档会导致:

1. **混淆新贡献者** - 不清楚标准的 OpenSpec change 应该包含哪些 artifacts
2. **破坏一致性** - 部分归档缺少必要的质量验证文档 (quality-check.md, runtime-check.md)
3. **降低可信度** - 不完整的归档让人质疑项目的文档质量标准
4. **历史遗留问题** - 这些是早期迁移过程中创建的临时归档,不应作为标准参考

## What Changes

### 移除不符合规范的归档

以下归档将被完全删除:

1. **2026-05-06-adrs-batch-1**
   - 问题: 只有 proposal.md,包含非标准 ADR 文件
   - 原因: 这是 ADR 批量导入的临时归档,不是标准的 OpenSpec change

2. **2026-05-06-exec-plans-batch-1**
   - 问题: 只有 proposal.md,包含大量用户故事文件
   - 原因: 这是执行计划批量导入的临时归档,不是标准的 OpenSpec change

3. **2026-05-06-improve-ai-agent-observability-in-vibe-coding**
   - 问题: 缺少 quality-check.md 和 runtime-check.md
   - 原因: 早期实施时未完整遵循 harness-quality schema

4. **2026-05-06-init-git-on-project-creation**
   - 问题: 包含非标准文件 frontend-test-guide.md,缺少 quality-check.md 和 runtime-check.md
   - 原因: 早期实施时 schema 还未完全标准化

### 保留符合规范的归档

以下归档将保留(已符合 harness-quality schema):

- ✅ 2026-05-06-complete-user-story-failed-status (完整 artifacts)
- ✅ 2026-05-06-migrate-docs-to-openspec (完整 artifacts)
- ✅ 2026-05-06-remove-docs-migrate-to-openspec (完整 artifacts)
- ✅ 2026-05-06-update-agents-to-openspec-dev-mode (完整 artifacts)
- ✅ 2026-05-06-update-docs-to-openspec-dev-mode (完整 artifacts)

### Capabilities

**Modified Capabilities**: None (this is a cleanup operation, no capability changes)

**New Capabilities**: None

## Impact

**Affected Files**:
- `openspec/changes/archive/2026-05-06-adrs-batch-1/` - 删除整个目录
- `openspec/changes/archive/2026-05-06-exec-plans-batch-1/` - 删除整个目录
- `openspec/changes/archive/2026-05-06-improve-ai-agent-observability-in-vibe-coding/` - 删除整个目录
- `openspec/changes/archive/2026-05-06-init-git-on-project-creation/` - 删除整个目录

**Breaking Changes**: 
- ⚠️ 这些归档将被永久删除,无法恢复(除非从 Git history 找回)
- 但这些归档本身就不符合规范,不应该作为参考

**Benefits**:
- ✅ 所有归档都符合 harness-quality schema 标准
- ✅ 新贡献者看到的一致且完整的示例
- ✅ 提高项目文档质量和专业性
- ✅ 减少混淆和维护负担


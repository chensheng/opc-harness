## Why

当前 README.md 和 AGENTS.md 中的开发流程描述仍然引用旧的 docs/ 目录结构和传统开发模式,但项目已完成向 OpenSpec 工作流的迁移。这导致:

1. **文档不一致** - README 中提到的开发流程与实际使用的 OpenSpec 工作流不符
2. **开发者困惑** - 新贡献者可能按照旧文档操作,导致工作流程混乱
3. **错失推广机会** - OpenSpec 是项目的核心创新点,应在 README 中突出展示

通过更新文档,可以确保所有开发者了解并使用正确的 OpenSpec 开发模式。

## What Changes

### README.md 更新

- **移除旧的开发流程描述** - 删除对 docs/ 目录的引用
- **添加 OpenSpec 工作流介绍** - 说明如何使用 `/opsx:` 命令管理变更
- **更新 Harness Engineering 章节** - 强调 OpenSpec 作为核心协作机制
- **添加快速入门示例** - 展示如何创建、实施、归档变更

### AGENTS.md 优化 (已在之前迁移中完成)

AGENTS.md 已在 `remove-docs-migrate-to-openspec` change 中更新为完整的 OpenSpec 导航,本次变更将:

- **验证链接有效性** - 确保所有 OpenSpec specs 链接可访问
- **补充开发流程示例** - 添加实际的 OpenSpec 命令使用示例
- **强化最佳实践** - 明确何时使用 explore mode vs apply mode

### Capabilities

**Modified Capabilities**: None (this is a documentation update only, no capability requirements changed)

**New Capabilities**: None

## Impact

**Affected Files**:
- `README.md` - 主要更新目标
- `AGENTS.md` - 可能需要微调(已在上次迁移中大幅更新)
- `openspec/specs/development-workflow/spec.md` - 可能需要更新以反映新的开发流程

**Breaking Changes**: None (documentation only)

**Benefits**:
- ✅ 统一的开发流程文档
- ✅ 新贡献者更容易上手
- ✅ 突出 OpenSpec 作为项目核心特色
- ✅ 减少因文档不一致导致的混淆

## Why

AGENTS.md 虽然在之前的迁移中已更新为 OpenSpec 导航,但仍可进一步优化以更好地指导开发者使用 OpenSpec 工作流:

1. **缺少实际示例** - 没有展示如何使用 `/opsx:` 命令的实际案例
2. **工作流程不够清晰** - 没有详细说明从 propose 到 archive 的完整流程
3. **最佳实践缺失** - 缺少何时使用 explore mode vs apply mode 的指导
4. **常见问题未解答** - 新贡献者可能遇到的典型问题没有 FAQ

通过完善 AGENTS.md,可以让新贡献者更快上手 OpenSpec 工作流,减少学习曲线。

## What Changes

### AGENTS.md 增强

- **添加 OpenSpec 快速入门章节** - 提供 step-by-step 指南
- **补充实际命令示例** - 展示真实的变更创建、实施、归档流程
- **添加最佳实践部分** - 说明何时使用不同的 OpenSpec 模式
- **增加 FAQ 章节** - 回答常见问题
- **优化 Capabilities 导航** - 提供更清晰的 capability 分类和链接
- **补充 SDD (软件设计文档) 内容** - 说明 SDD 标准结构、ADR 编写指南、与 OpenSpec 的关系

### Capabilities

**Modified Capabilities**:
- `development-workflow` - 可能需要更新以反映更详细的 OpenSpec 工作流程指导

**New Capabilities**: None (this is a documentation enhancement)

## Impact

**Affected Files**:
- `AGENTS.md` - 主要更新目标,添加更多 OpenSpec 使用指导
- `openspec/specs/development-workflow/spec.md` - 可能需要小幅更新(如果需要)

**Breaking Changes**: None (documentation enhancement only)

**Benefits**:
- ✅ 新贡献者更容易理解和使用 OpenSpec
- ✅ 减少因不清楚工作流程导致的错误
- ✅ 提供实际示例,降低学习门槛
- ✅ 建立最佳实践,提高代码质量

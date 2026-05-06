## Why

当前用户故事状态机存在**类型不一致问题**:后端 Agent Worker 在任务失败时会设置 `status = "failed"`,但前端 TypeScript 类型定义中缺少该状态。这导致:

1. **类型安全风险**: 前端读取到 `failed` 状态的 Story 时,TypeScript 类型不匹配,可能导致运行时错误
2. **UI 渲染异常**: 状态颜色和标签映射中缺少 `failed`,显示为 undefined 或默认样式
3. **功能缺失**: 用户无法查看、筛选或重试失败的 Story,影响开发流程的可观测性
4. **数据一致性**: 数据库中存在 `failed` 状态记录,但前端无法正确处理

此问题在去中心化 Agent Worker 架构下尤为关键,因为多个 Worker 并发执行任务时,失败是正常现象,必须被妥善追踪和处理。

## What Changes

- **前端类型定义**: 在 `UserStory` 接口中添加 `'failed'` 状态
- **UI 组件更新**: 
  - 在状态颜色映射和标签中添加 `failed` 的视觉样式
  - 在编辑对话框的状态选择器中添加 `failed` 选项
  - 添加失败原因的显示区域
- **状态转换逻辑**: 
  - 支持从 `failed` 回退到 `draft` 或 `refined` (重试机制)
  - 在 Agent Worker 中保持现有的失败处理逻辑不变
- **用户体验优化**:
  - 在用户故事列表中清晰标识失败的 Story
  - 提供"重试"操作按钮(将状态重置为 `draft`)
  - 显示失败原因和重试次数

## Capabilities

### New Capabilities
- `user-story-failed-status`: 完整的 failed 状态支持,包括类型定义、UI 展示、状态转换和重试机制

### Modified Capabilities
<!-- 无现有规格需要修改,这是新增能力 -->

## Impact

**受影响的代码**:
- 前端类型定义: `src/types/index.ts`
- 前端 Store: `src/stores/userStoryStore.ts`
- UI 组件: 
  - `src/components/vibe-coding/UserStoryTable.tsx`
  - `src/components/vibe-coding/UserStoryEditDialog.tsx`
  - `src/components/vibe-coding/UserStoryManager.tsx`
- 后端 Rust 代码: 无需修改(已正确实现)

**API 影响**: 无 breaking changes,仅扩展类型定义

**依赖影响**: 无新增依赖

**系统影响**: 
- 提升系统的可观测性和健壮性
- 改善用户在任务失败时的体验
- 符合完整的状态机设计规范

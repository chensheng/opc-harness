## Context

**当前状态**:
- 后端 Rust 代码已正确实现 `failed` 状态,定义在 `src-tauri/src/db/sprint_repository.rs` 的 `user_story_status::FAILED` 常量
- Agent Worker 在任务失败时调用 `fail_user_story()` 设置状态为 `"failed"`,并记录错误信息和重试次数
- 前端 TypeScript 类型定义缺少 `'failed'` 状态,导致类型不匹配
- UI 组件中缺少对 `failed` 状态的视觉呈现和交互支持

**约束条件**:
- 不能修改后端 Rust 代码(已经正确实现)
- 必须保持向后兼容,不影响现有功能
- 需要符合项目的架构约束(CODE-001: 单文件不超过 500 行)
- 需要遵循现有的代码风格和命名规范

**利益相关者**:
- 前端开发者: 需要正确的类型定义和 UI 组件
- 用户: 需要清晰看到失败的 Story 并能重试
- Agent Worker: 需要正确报告失败状态

## Goals / Non-Goals

**Goals:**
1. 在前端完整支持 `failed` 状态,消除类型不一致
2. 提供清晰的 UI 展示,让用户能识别失败的 Story
3. 实现重试机制,允许用户将失败的 Story 重置为可执行状态
4. 显示失败原因和重试次数,提升可观测性
5. 确保所有状态转换符合业务逻辑

**Non-Goals:**
1. 不修改后端的状态转换逻辑(已正确实现)
2. 不实现自动重试机制(由用户手动触发)
3. 不添加复杂的失败分类或分析功能
4. 不改变现有的数据库 schema

## Decisions

### Decision 1: 状态定义扩展

**选择**: 在前端类型中添加 `'failed'` 作为合法状态

**理由**:
- 与后端保持一致,确保类型安全
- 最小化改动,只需扩展联合类型
- 符合开闭原则(对扩展开放,对修改封闭)

**替代方案考虑**:
- ❌ 移除后端的 `failed` 状态: 会丢失语义清晰度,无法区分成功完成和失败
- ❌ 使用 `completed` + `error_message`: 混淆了成功和失败的语义

### Decision 2: UI 视觉设计

**选择**: 使用红色系配色方案标识失败状态

```typescript
const statusColors = {
  // ... existing colors
  failed: 'bg-red-50 text-red-700 border border-red-200',
}

const statusLabels = {
  // ... existing labels
  failed: '失败',
}
```

**理由**:
- 红色是通用的错误/失败标识色,符合用户心智模型
- 与其他状态的颜色形成明显对比
- 保持与现有设计风格一致(浅色背景 + 深色文字 + 边框)

### Decision 3: 重试机制设计

**选择**: 提供"重试"按钮,将状态从 `failed` 重置为 `draft`

**实现方式**:
```typescript
// 在 UserStoryManager 中添加重试处理函数
const handleRetryStory = async (storyId: string) => {
  await updateStory(projectId, storyId, {
    status: 'draft',
    updatedAt: new Date().toISOString(),
  })
}
```

**理由**:
- 简单直接,符合用户的预期行为
- `draft` 状态表示待处理,可以被 Agent 重新选取
- 保留 `error_message` 和 `retry_count` 字段用于追踪历史

**替代方案考虑**:
- ❌ 重置为 `refined`: 可能需要重新评审,增加复杂度
- ❌ 新增 `retrying` 状态: 过度设计,`draft` 已足够

### Decision 4: 失败信息展示

**选择**: 在编辑对话框中添加只读的失败信息区域

**展示内容**:
- 失败原因 (`error_message`)
- 重试次数 (`retry_count`)
- 失败时间 (`failed_at`)

**理由**:
- 提供完整的失败上下文,帮助用户诊断问题
- 只读设计防止用户误修改系统生成的信息
- 仅在状态为 `failed` 时显示,避免界面混乱

### Decision 5: 筛选和排序支持

**选择**: 在筛选器中添加 `failed` 选项

**实现**:
```typescript
// 在 UserStoryManager 的状态筛选下拉框中添加
<SelectItem value="failed">失败</SelectItem>
```

**理由**:
- 用户可以快速定位所有失败的 Story
- 便于批量重试或分析失败模式
- 与现有的筛选功能保持一致

## Risks / Trade-offs

### Risk 1: 类型兼容性
**风险**: 旧数据可能包含未知的状态值

**缓解**: 
- TypeScript 运行时不会强制检查,但编译时会捕获新代码的类型错误
- 添加防御性编程: `statusColors[story.status] || 'bg-gray-100'`

### Risk 2: 用户体验
**风险**: 用户可能不理解为什么 Story 会失败

**缓解**:
- 清晰显示失败原因
- 提供友好的重试提示
- 考虑未来添加失败原因的通俗解释

### Trade-off 1: 简单性 vs 完整性
**选择**: 优先简单性,仅提供基本的重试功能

**权衡**:
- ✅ 优点: 快速实现,易于理解和使用
- ❌ 缺点: 缺少高级功能(如自动重试、失败分类)
- **决策**: 可以在未来迭代中增强

### Trade-off 2: 前端验证 vs 后端验证
**选择**: 仅在前端添加类型定义,不在前端强制执行状态转换规则

**权衡**:
- ✅ 优点: 灵活性高,后端是唯一的真相源
- ❌ 缺点: 可能在提交前无法捕获所有无效转换
- **决策**: 后端已有乐观锁和状态检查,足够安全

## Migration Plan

**部署步骤**:
1. 更新前端类型定义
2. 更新 UI 组件(表格、编辑对话框、筛选器)
3. 添加重试功能
4. 测试验证(手动测试 + E2E 测试)
5. 部署到生产环境

**回滚策略**:
- 如果发现严重问题,可以回滚前端代码
- 后端无需回滚(保持不变)
- 数据库中的 `failed` 状态记录不受影响

**数据迁移**:
- 无需数据迁移,数据库 schema 未变更
- 现有的 `failed` 状态记录将自动被前端正确显示

## Open Questions

1. **是否需要失败通知?** 
   - 当前设计不包含主动通知机制
   - 可以考虑未来集成到 Agent Alerting 系统

2. **重试次数是否应该限制?**
   - 当前无限制,用户可以无限重试
   - 可以考虑添加最大重试次数(如 3 次)

3. **是否需要失败统计?**
   - 当前仅显示单个 Story 的重试次数
   - 可以考虑在项目级别添加失败率统计

这些问题可以在后续迭代中根据用户反馈决定。

# Vibe Coding - HITL 检查点审查界面

## CP-002 任务分解审查

### 功能概述

CP-002 检查点在 Initializer Agent 完成任务分解后触发，用于审查生成的 Issues 列表质量。

**核心功能**:
- ✅ 里程碑视图展示（按 Milestone 分组）
- ✅ 任务列表过滤和排序
- ✅ 依赖关系可视化
- ✅ 风险识别和警告
- ✅ 统计数据面板
- ✅ 任务编辑功能
- ✅ 批准/拒绝/修改决策

### 访问路径

```
/checkpoint/:projectId/:checkpointId
```

示例：`/checkpoint/proj-123/CP-002`

### 界面布局

```
┌─────────────────────────────────────────────────────────────┐
│ 🔍 CP-002 任务分解审查                    [取消] [拒绝] [批准] │
├─────────────────────────────────────────────────────────────┤
│ 📊 统计卡片：总任务 | P0 | P1 | 工时 | 依赖 | 里程碑          │
├─────────────────────────────────────────────────────────────┤
│ ⚠️  风险识别 (如有)                                          │
│   - 高：P0 优先级任务过多                                    │
│   - 中：依赖关系复杂                                         │
├─────────────────────────────────────────────────────────────┤
│ [里程碑视图] [任务列表] [依赖关系]                            │
│                                                              │
│ ┌────────────────────────────────────────────────────────┐ │
│ │ 里程碑 1: 用户认证系统                    4 个任务        │ │
│ │ #1 实现用户注册 API                        P0           │ │
│ │ #2 实现用户登录 API                        P0           │ │
│ │ ...                                                    │ │
│ └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 数据结构

#### Checkpoint
```typescript
interface Checkpoint {
  id: CheckpointId          // 'CP-002'
  name: string              // '任务分解审查'
  description: string       // 详细描述
  triggeredAt: string       // ISO 时间戳
  agentId: string           // 触发的 Agent ID
  status: CheckpointStatus  // 'pending' | 'reviewed' | 'approved' | 'rejected'
  reviewItems: ReviewItem[] // 审查项目列表
  autoAcceptEnabled: boolean
  trustThreshold: number    // 0.0 - 1.0
}
```

#### Issue
```typescript
interface Issue {
  id: string
  iid: number                // Issue 编号
  title: string
  description: string
  acceptanceCriteria: string[]
  priority: 'P0' | 'P1' | 'P2' | 'P3'
  status: 'todo' | 'in_progress' | 'done'
  estimatedHours?: number
  dependencies?: number[]    // 依赖的 Issue iid 列表
  labels: string[]
}
```

### 自动接受条件

MVP 阶段简化策略:
- ✅ Issue 数量 < 20
- ✅ 无 P0 优先级的依赖关系
- ✅ 信任度阈值 > 80%

满足条件时可启用自动接受模式。

### 用户操作

1. **批准继续** (`handleApprove`)
   - 调用 Tauri Command: `approve_checkpoint(checkpoint_id)`
   - 跳转到 Coding Workspace
   - Initializer Agent 继续执行

2. **拒绝修改** (`handleReject`)
   - 调用 Tauri Command: `reject_checkpoint(checkpoint_id, feedback)`
   - 提供反馈文本框
   - Initializer Agent 重新分解任务

3. **修改任务** (`handleModify`)
   - 调用 Tauri Command: `modify_issue(issue_iid, field, new_value)`
   - 支持修改：优先级、工时估算、依赖关系
   - 修改后需重新审查

### 风险识别规则

```typescript
// 高风险
- P0 优先级任务 > 5 个
- 关键路径缺失

// 中风险
- 任务总数 > 20 个
- 依赖关系复杂度 > 50%
- 工时估算不合理
```

### 下一步计划

#### Phase 1: 基础功能 (当前已完成) ✅
- [x] UI 组件完整实现
- [x] Mock 数据展示
- [x] 过滤和排序功能
- [x] 风险识别逻辑

#### Phase 2: Backend 集成 (待开发)
- [ ] Tauri Commands 实现
  - `approve_checkpoint()`
  - `reject_checkpoint()`
  - `modify_issue()`
- [ ] Rust Backend 逻辑
  - CheckpointManager 实现
  - Issue 持久化
  - Agent 通信协议

#### Phase 3: 高级功能 (延后)
- [ ] 依赖关系图可视化 (使用 D3.js 或 React Flow)
- [ ] 任务拖拽调整顺序
- [ ] 实时协作审查
- [ ] AI 辅助优化建议

### 相关文件

- 组件：[`CheckpointReview.tsx`](./src/components/vibe-coding/CheckpointReview.tsx)
- 类型定义：[`types/index.ts`](./src/types/index.ts)
- 路由配置：[`App.tsx`](./src/App.tsx)
- 架构设计：[`docs/架构设计.md`](./docs/架构设计.md#hitl-检查点机制)
- MVP 规划：[`docs/product-specs/mvp-roadmap.md`](./docs/product-specs/mvp-roadmap.md)

### 测试用例

```typescript
// TODO: 添加单元测试
describe('CheckpointReview', () => {
  it('should display milestone statistics', () => {})
  it('should filter issues by priority', () => {})
  it('should show risk alerts when conditions met', () => {})
  it('should handle approve action', () => {})
  it('should handle reject action', () => {})
})
```

### 注意事项

1. **Mock 数据**: 当前使用 Mock 数据，Backend 集成后替换为真实API
2. **Tauri Commands**: 需要 Rust 后端实现对应的命令处理
3. **状态管理**: 考虑使用 Zustand store 管理 Checkpoint 状态
4. **性能优化**: 大量 Issues (>50) 时需优化渲染性能（虚拟滚动）

---

**文档版本**: v1.0  
**最后更新**: 2026-03-25  
**状态**: 📝 草稿

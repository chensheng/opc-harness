## Context

当前项目已完成 Native Coding Agent 的完整实现，Health Score 达到 100/100。`VITE_USE_NATIVE_AGENT` 环境变量最初用于灰度发布和降级方案，但现在：

1. Native Agent 已稳定运行并通过所有测试
2. CLI Agent 已被标记为 deprecated（`ai_cli_interaction.rs`）
3. 前端设置界面已有 Native Agent 配置选项
4. 保留该标志增加了代码复杂度和维护成本

**约束条件**：
- 必须保持向后兼容（CLI Agent 仍作为降级方案存在）
- 不能破坏现有的 AgentWorker 架构
- 需要确保健康评分保持 100/100

## Goals / Non-Goals

**Goals:**
- 移除 `VITE_USE_NATIVE_AGENT` 环境变量及相关逻辑
- 简化前端 Settings 组件，移除切换选项
- 清理后端 AgentWorker 中的条件分支
- 更新配置文件和文档

**Non-Goals:**
- 不删除 CLI Agent 代码（保留作为历史参考和紧急降级方案）
- 不改变 Native Agent 的核心实现
- 不涉及新的功能开发

## Decisions

### Decision 1: 完全移除环境变量检查

**选择**：从所有代码中移除 `VITE_USE_NATIVE_AGENT` 的检查逻辑

**理由**：
- Native Agent 已成为唯一的生产方案
- 减少配置复杂度，避免用户困惑
- 简化代码路径，降低维护成本

**替代方案**：
- ❌ 保留但默认设为 true：仍然增加配置负担
- ❌ 仅在开发环境保留：不一致的行为会导致混淆

### Decision 2: 前端 Settings 移除 Native Agent 卡片

**选择**：从 Settings.tsx 中移除整个 Native Agent 配置卡片

**理由**：
- 不再有切换需求，UI 应保持简洁
- 避免用户误以为可以切换回 CLI Agent
- 减少前端状态管理的复杂度

**替代方案**：
- ❌ 保留但禁用：无意义的 UI 元素
- ❌ 改为只读显示：仍然占用空间且无价值

### Decision 3: AppSettings 类型保留 useNativeAgent 字段（可选）

**选择**：暂时保留 `useNativeAgent` 字段但标记为 deprecated，或直接移除

**理由**：
- 如果未来可能需要重新引入切换功能，保留字段更方便
- 但会增加类型定义的复杂度

**最终决定**：**直接移除**该字段，因为：
1. 架构决策已明确：Native Agent 是唯一方案
2. TypeScript 编译会捕获所有引用，便于清理
3. 如需恢复，可以重新添加（Git 历史可追溯）

### Decision 4: AgentWorker 简化执行逻辑

**选择**：移除 `execute_native_agent` 和 `execute_cli_agent` 的条件分支，统一调用 Native Agent

**理由**：
- 代码更清晰，单一职责
- 减少运行时判断开销
- 符合架构演进方向

**实现方式**：
```rust
// Before
if use_native_agent {
    execute_native_agent(...).await
} else {
    execute_cli_agent(...).await
}

// After
execute_native_agent(...).await
```

## Risks / Trade-offs

### Risk 1: 失去快速降级能力

**风险**：如果 Native Agent 出现严重 bug，无法快速切换回 CLI Agent

**缓解措施**：
- CLI Agent 代码仍然存在于代码库中
- 可通过 Git revert 快速回滚此变更
- Native Agent 已有完善的错误处理和重试机制

### Risk 2: 遗留代码引用导致编译错误

**风险**：移除环境变量后，可能有遗漏的引用导致编译失败

**缓解措施**：
- 使用 TypeScript/Rust 编译器捕获所有引用
- 运行 `npm run harness:check` 确保零错误
- 全面搜索代码库确认无遗漏

### Trade-off: 灵活性 vs 简洁性

**权衡**：移除配置选项减少了灵活性，但显著提升了代码简洁性和可维护性

**接受理由**：
- Native Agent 已证明稳定性（100/100 Health Score）
- 架构决策应明确，避免"永远保留选项"的技术债务
- 如需恢复，Git 历史和归档的变更提供了完整回溯能力

## Migration Plan

### 步骤 1: 清理前端代码
1. 从 `src/types/index.ts` 移除 `useNativeAgent` 字段
2. 从 `src/stores/appStore.ts` 移除相关初始化和持久化逻辑
3. 从 `src/components/common/Settings.tsx` 移除 Native Agent 卡片
4. 更新相关测试文件

### 步骤 2: 清理后端代码
1. 从 `src-tauri/src/agent/agent_worker.rs` 移除环境变量读取逻辑
2. 简化 `execute_story` 方法，直接调用 Native Agent
3. 移除 CLI Agent 执行分支

### 步骤 3: 更新配置文件
1. 从 `.env.development` 移除 `VITE_USE_NATIVE_AGENT`
2. 从 `.env.example` 移除 `VITE_USE_NATIVE_AGENT`
3. 更新相关文档

### 步骤 4: 验证与测试
1. 运行 `npm run harness:check` 确保健康评分 100/100
2. 运行所有单元测试
3. 手动测试 Agent 执行流程

### Rollback Strategy
- Git revert 此变更即可恢复所有功能
- 无需数据迁移或数据库变更

## Open Questions

无开放问题。所有技术决策已明确。

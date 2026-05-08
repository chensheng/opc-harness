## Why

Native Coding Agent 已经稳定运行并通过所有测试（Health Score: 100/100），CLI-based Agent 已成为历史遗留方案。保留 `VITE_USE_NATIVE_AGENT` 环境变量增加了配置复杂度，且当前架构已完全迁移到 Native Agent。移除该标志可以简化代码、减少维护负担，并明确项目的技术方向。

## What Changes

- **移除环境变量**：删除 `VITE_USE_NATIVE_AGENT` 及相关配置逻辑
- **简化前端代码**：移除 Settings 中的 Native Agent 切换选项
- **清理后端逻辑**：移除 AgentWorker 中的条件分支判断
- **更新配置文件**：从 `.env.development` 和 `.env.example` 中移除该变量
- **归档 CLI Agent**：将 `ai_cli_interaction.rs` 标记为 deprecated（已完成）

## Capabilities

### New Capabilities
<!-- 无新增能力 -->

### Modified Capabilities
- `coding-harness`: 移除 Native/CLI Agent 切换逻辑，统一使用 Native Agent
- `agent-initialization`: 简化 Agent 初始化流程，移除环境变量检查

## Impact

**Affected Code**:
- 前端：`src/components/common/Settings.tsx`, `src/stores/appStore.ts`, `src/types/index.ts`
- 后端：`src-tauri/src/agent/agent_worker.rs`
- 配置：`.env.development`, `.env.example`

**Breaking Changes**: 
- 无（Native Agent 已是默认行为，移除标志不影响现有功能）

**Dependencies**: 
- 无外部依赖变更

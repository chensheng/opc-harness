## Why

当前 Vibe Coding 智能体已实现基础的自动化编码流程，但在实际使用中存在三个关键问题：

1. **工具集不完整**：AI 无法主动搜索代码库或管理依赖，限制了其自主性
2. **缺乏人工干预机制**：用户无法在执行过程中审核关键决策，错误的方向无法及时纠正
3. **资源泄漏风险**：Worktree 未自动清理，长期运行会占用大量磁盘空间

这些问题影响了智能体的实用性、可控性和稳定性，需要通过系统性优化来解决。

## What Changes

- **扩展工具集**：添加代码搜索工具（grep/find）和依赖管理工具（npm install/cargo add）
- **实现 HITL Checkpoint 机制**：在关键步骤暂停，等待用户批准/拒绝 AI 的更改
- **Worktree 自动清理**：Story 完成后（成功或失败）自动删除 worktree，防止磁盘泄漏
- **优化对话历史管理**：添加任务完成标记，实现历史压缩以减少 token 成本
- **改进质量检查流程**：分阶段检查（lint → type-check → test），提供精确错误位置

## Capabilities

### New Capabilities

- `agent-code-search`: 代码搜索能力，支持 grep/find 模式匹配和函数查找
- `agent-dependency-management`: 依赖管理能力，支持 npm/cargo 包安装和版本管理
- `agent-hitl-checkpoint`: Human-in-the-Loop 检查点机制，支持用户审核和干预
- `agent-worktree-lifecycle`: Worktree 生命周期管理，包括自动创建和清理

### Modified Capabilities

- `coding-harness`: 增强质量检查流程，支持分阶段检查和增量检查
- `agent-initialization`: 优化对话历史管理，添加任务完成标记和历史压缩

## Impact

**Affected Code**:
- 后端：`src-tauri/src/agent/native_coding_agent.rs`, `src-tauri/src/agent/tools/`, `src-tauri/src/agent/agent_worker.rs`
- 前端：可能需要添加 Checkpoint UI 组件
- 数据库：可能需要新增 checkpoint 相关表

**Breaking Changes**: 
- 无（向后兼容，现有功能不受影响）

**Dependencies**: 
- 无外部依赖变更

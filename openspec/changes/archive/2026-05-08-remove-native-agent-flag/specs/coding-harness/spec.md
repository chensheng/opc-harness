## MODIFIED Requirements

### Requirement: 自主编码 Harness 规范
系统 SHALL 提供完整的自主编码 Harness 实现指南，包括 Agent 通信协议、任务分解策略和质量控制机制。系统 MUST 统一使用 Native Coding Agent，不再支持 CLI-based Agent 切换。

#### Scenario: Agent 间通信
- **WHEN** Initializer Agent 完成任务分解
- **THEN** 通过标准化协议将 Issues 分发给 Coding Agents

#### Scenario: 质量控制循环
- **WHEN** Coding Agent 生成代码
- **THEN** Harness 自动执行 lint、test、type-check，失败时触发修复循环

#### Scenario: Native Agent 统一执行
- **WHEN** Agent Worker 接收到 Story 执行请求
- **THEN** 系统直接调用 NativeCodingAgent 执行，不进行环境变量检查
- **AND** 不使用 VITE_USE_NATIVE_AGENT 配置项

## REMOVED Requirements

### Requirement: Native/CLI Agent 切换机制
**Reason**: Native Agent 已成为唯一的生产方案，CLI Agent 已标记为 deprecated
**Migration**: 移除所有 VITE_USE_NATIVE_AGENT 环境变量引用，代码中直接调用 NativeCodingAgent

## ADDED Requirements

### Requirement: Agent Worker 重试引擎集成

Agent Worker SHALL 在初始化时加载并配置重试引擎和重试调度器，使其能够自动扫描待重试队列并触发重试任务。

#### Scenario: Agent Worker 启动时初始化重试调度器

- **WHEN** Agent Worker 的 `start()` 方法被调用
- **THEN** 系统在后台线程中启动 RetryScheduler 的 `run()` 方法，传入 project_id 和 websocket_manager

#### Scenario: Agent Worker 优雅关闭时停止调度器

- **WHEN** Agent Worker 接收到关闭信号或调用 `stop()` 方法
- **THEN** 系统向调度器发送停止信号，等待所有活跃的重试任务完成后退出后台线程

#### Scenario: 调度器与 Agent Loop 并行运行

- **WHEN** Agent Worker 正常运行时
- **THEN** 系统的 Agent Loop（查询 pending Stories）和 RetryScheduler（查询 scheduled_retry Stories）同时在后台运行，互不干扰

### Requirement: AGENTS.md 文档结构

AGENTS.md SHALL 保持简洁，作为 AI Agent 的导航地图，仅包含快速入口、三大支柱概述和文档导航链接，其他详细信息应引导至 OpenSpec specs。

#### Scenario: AI Agent 查看 AGENTS.md

- **WHEN** AI Agent 首次访问项目
- **THEN** 他们能在 30 秒内找到核心文档链接和理解项目架构

#### Scenario: 用户需要详细工作流说明

- **WHEN** 用户需要了解 OpenSpec 工作流的详细步骤
- **THEN** AGENTS.md 提供清晰的链接指向 openspec/specs/development-workflow/spec.md

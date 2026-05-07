## ADDED Requirements

### Requirement: Agent Worker 重试引擎集成

Agent Worker SHALL 在初始化时加载并配置重试引擎，使其能够自动处理失败的用户故事。

#### Scenario: Agent Worker 启动时初始化重试引擎
- **WHEN** Agent Worker 进程启动
- **THEN** 系统从数据库读取项目的重试配置，初始化 RetryEngine 实例

#### Scenario: Agent Worker 注册重试调度器定时器
- **WHEN** Agent Worker 完成初始化
- **THEN** 系统启动一个后台定时器，每 30 秒检查一次待重试队列

#### Scenario: Agent Worker 优雅关闭时清理定时器
- **WHEN** Agent Worker 接收到关闭信号
- **THEN** 系统停止重试调度器定时器，等待当前重试任务完成后退出

### Requirement: AGENTS.md 文档结构

AGENTS.md SHALL 保持简洁，作为 AI Agent 的导航地图，仅包含快速入口、三大支柱概述和文档导航链接，其他详细信息应引导至 OpenSpec specs。

#### Scenario: AI Agent 查看 AGENTS.md

- **WHEN** AI Agent 首次访问项目
- **THEN** 他们能在 30 秒内找到核心文档链接和理解项目架构

#### Scenario: 用户需要详细工作流说明

- **WHEN** 用户需要了解 OpenSpec 工作流的详细步骤
- **THEN** AGENTS.md 提供清晰的链接指向 openspec/specs/development-workflow/spec.md

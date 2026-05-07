## MODIFIED Requirements

### Requirement: AGENTS.md 文档结构

AGENTS.md SHALL 保持简洁，作为 AI Agent 的导航地图，仅包含快速入口、三大支柱概述和文档导航链接，其他详细信息应引导至 OpenSpec specs。

#### Scenario: AI Agent 查看 AGENTS.md

- **WHEN** AI Agent 首次访问项目
- **THEN** 他们能在 30 秒内找到核心文档链接和理解项目架构

#### Scenario: 用户需要详细工作流说明

- **WHEN** 用户需要了解 OpenSpec 工作流的详细步骤
- **THEN** AGENTS.md 提供清晰的链接指向 openspec/specs/development-workflow/spec.md

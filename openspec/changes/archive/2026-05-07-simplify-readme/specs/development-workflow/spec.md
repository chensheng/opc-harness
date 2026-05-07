## MODIFIED Requirements

### Requirement: README 文档结构

README.md SHALL 保持简洁，仅包含概述、核心功能、快速开始三个核心部分，其他详细信息应引导至 AGENTS.md 和 OpenSpec specs。

#### Scenario: 新用户查看 README

- **WHEN** 新用户首次访问项目仓库
- **THEN** 他们能在 30 秒内理解项目价值和快速上手步骤

#### Scenario: 用户需要详细技术信息

- **WHEN** 用户需要了解开发工作流或技术架构
- **THEN** README 提供清晰的链接指向 AGENTS.md 和 openspec/specs/

### Requirement: 文档导航

README.md SHALL 在末尾提供"了解更多"章节，包含到 AGENTS.md 和 OpenSpec 文档的链接。

#### Scenario: 用户探索更多文档

- **WHEN** 用户阅读完 README 的快速开始部分
- **THEN** 他们能看到明确的下一步文档指引

## Why

当前 README.md 包含过多细节信息（开发工作流、技术架构、Harness Engineering 等），导致新用户难以快速理解项目核心价值，且与 AGENTS.md 和 OpenSpec specs 中的内容重复。通过精简 README，保留概述、核心功能、快速开始三个核心部分，可以提升新用户的上手体验，同时将详细文档引导至专门的文档位置。

## What Changes

- **保留概述部分** - 项目简介和核心价值主张
- **保留核心功能** - Vibe Design、Vibe Coding、Vibe Marketing 三大模块介绍
- **保留快速开始** - 环境要求、安装步骤、开发和构建命令
- **删除开发工作流章节** - OpenSpec 工作流详情已整合到 AGENTS.md
- **删除技术架构章节** - 技术栈详情可参考 AGENTS.md 和 src/AGENTS.md、src-tauri/AGENTS.md
- **删除 Harness Engineering 章节** - 核心理念和文档指引已在 AGENTS.md 详细说明
- **添加文档导航** - 在末尾添加简洁的"了解更多"链接，指向 AGENTS.md 和 OpenSpec 文档

## Capabilities

### New Capabilities

<!-- 此变更不涉及新增能力 -->

### Modified Capabilities

- `development-workflow`: 简化 README 中的工作流说明，将详细内容迁移至 AGENTS.md

## Impact

**受影响的文件**:
- `README.md` - 从 178 行精简至约 65 行

**受影响的系统**:
- 无代码变更，纯文档优化
- 用户通过 README 快速了解项目后，可通过链接跳转至详细文档

**依赖影响**:
- 无

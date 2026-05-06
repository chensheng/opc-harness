## Why

harness:check 脚本中包含的文档检测功能(第 424-462 行的 `Invoke-DocumentationCheck` 函数)在当前项目架构下已不再适用。项目已完成从 `docs/` 目录到 OpenSpec 工作流的迁移,`docs/` 目录已被移除。当前的文档检测仍在检查已不存在的 `docs/` 目录结构和相关索引文件,导致每次运行 harness:check 时都会产生警告,降低了检查工具的可信度和实用性。

## What Changes

- 从 `scripts/harness-check.ps1` 中移除 `Invoke-DocumentationCheck` 函数(第 424-462 行)
- 从配置中移除文档相关的权重配置(`Documentation = 10`)
- 从配置中移除文档相关的检查项(`KeyDocuments`, `IndexFiles`)
- 调整检查步骤编号(从 9 步改为 8 步)
- 重新分配权重分数,保持总分 100 分

## Capabilities

### New Capabilities
<!-- No new capabilities being introduced -->

### Modified Capabilities
<!-- No existing capabilities are being modified - this is a script cleanup only -->

## Impact

- **Affected files**: 
  - `scripts/harness-check.ps1` (主要修改)
- **Impact type**: 脚本功能精简,无功能性破坏
- **Breaking changes**: 无(仅移除已过时的检测项)
- **Dependencies**: 无
- **Health Score impact**: 预期提升(移除误报警告)

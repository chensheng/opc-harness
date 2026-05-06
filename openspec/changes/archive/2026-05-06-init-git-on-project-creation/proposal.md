## Why

当前项目在创建新项目时不会自动初始化 Git 仓库，导致以下问题：

1. **前端 GitDetector 组件无法正常工作**：前端实现了完整的 Git 管理 UI（[GitDetector.tsx](file://d:/workspace/opc-harness/src/components/common/GitDetector.tsx)），但后端缺少对应的 Tauri 命令实现，导致"Initialize Git Repository"按钮点击后会失败。

2. **Worktree 创建时的被动初始化不够及时**：虽然 [WorktreeManager](file://d:/workspace/opc-harness/src-tauri/src/agent/worktree_manager.rs#L260-L295) 会在创建 worktree 时执行 `git init`，但这是一种被动的、延迟的初始化方式，不符合"项目创建即具备版本控制能力"的最佳实践。

3. **Initializer Agent 仅配置用户信息**：[InitializerAgent](file://d:/workspace/opc-harness/src-tauri/src/agent/initializer_agent.rs#L640-L677) 只配置了 `user.name` 和 `user.email`，但没有执行 `git init`，导致配置可能应用到不存在的仓库。

4. **用户体验不一致**：用户期望创建项目后立即拥有版本控制能力，而不是在某个后续操作时才被动触发。

## What Changes

- **在项目创建时立即执行 Git 初始化**：修改 `create_project` 命令，在工作区目录创建后自动执行 `git init`
- **实现缺失的 Git 管理 Tauri 命令**：补充前端已调用但未实现的后端命令：
  - `check_git_status`: 检查 Git 仓库状态
  - `init_git_repo`: 初始化 Git 仓库
  - `set_git_config`: 设置 Git 配置项
  - `get_git_config`: 获取单个 Git 配置项
  - `get_all_git_config`: 获取所有 Git 配置项
- **优化 Initializer Agent**：移除冗余的 Git 初始化逻辑，改为依赖项目创建时的初始化
- **更新 WorktreeManager**：简化 `ensure_git_initialized` 方法，因为项目创建时已初始化

## Capabilities

### New Capabilities
- `git-repository-management`: 完整的 Git 仓库管理能力，包括状态检查、初始化、配置管理等核心功能

### Modified Capabilities
- `project-creation`: 项目创建工作流增强，在创建工作区目录后立即初始化 Git 仓库
- `agent-initialization`: Agent 初始化流程优化，移除冗余的 Git 初始化步骤

## Impact

**受影响的代码模块**：
- `src-tauri/src/commands/database.rs`: 修改 `create_project` 函数，添加 Git 初始化逻辑
- `src-tauri/src/commands/system.rs`: 新增 Git 管理相关的 Tauri 命令
- `src-tauri/src/main.rs`: 注册新的 Git 管理命令到 `invoke_handler`
- `src-tauri/src/agent/initializer_agent.rs`: 简化 Git 配置逻辑
- `src-tauri/src/agent/worktree_manager.rs`: 简化 `ensure_git_initialized` 方法
- `src/hooks/useGit.ts`: 前端 Hook 无需修改（已实现）
- `src/components/common/GitDetector.tsx`: 前端组件无需修改（已实现）

**API 变更**：
- 新增 5 个 Tauri 命令供前端调用
- `create_project` 命令的行为变更：现在会自动初始化 Git 仓库

**依赖影响**：
- 无新增外部依赖
- 需要系统已安装 Git（已有依赖，无需变更）

**向后兼容性**：
- ✅ 完全向后兼容
- 现有项目的 Git 仓库不受影响
- 新创建的项目将自动包含 `.git` 目录

## 1. Git 管理命令实现

- [x] 1.1 在 `src-tauri/src/commands/system.rs` 中实现 `check_git_status` 命令，返回 GitStatus 结构
- [x] 1.2 实现 `init_git_repo` 命令，执行 git init、创建 .gitignore 文件并创建初始空 commit
- [x] 1.3 实现 `set_git_config` 命令，设置指定路径的 Git 配置项
- [x] 1.4 实现 `get_git_config` 命令，获取单个 Git 配置项的值
- [x] 1.5 实现 `get_all_git_config` 命令，获取 userName 和 userEmail 配置

## 2. 项目创建流程增强

- [x] 2.1 在 `src-tauri/src/commands/database.rs` 中创建 `initialize_git_repository` 辅助函数
- [x] 2.2 在 `initialize_git_repository` 中实现 .gitignore 文件创建逻辑，忽略 `.opc-harness/` 目录
- [x] 2.3 修改 `create_project` 函数，在工作区目录创建后调用 Git 初始化
- [x] 2.4 添加 Git 用户配置默认值设置逻辑（检查全局配置，缺失时设置项目级默认值）
- [x] 2.5 实现优雅降级：Git 初始化失败时记录警告但不阻止项目创建

## 3. 项目打开时的 Git 检测与初始化

- [x] 3.1 在 `database.rs` 中创建 `ensure_workspace_and_git` 辅助函数
- [x] 3.2 修改 `get_project_by_id` 命令，在返回项目前调用 Git 检测和初始化
- [x] 3.3 处理工作区目录不存在的情况：重新创建并初始化 Git
- [x] 3.4 处理 Git 未初始化的情况：自动执行初始化流程
- [x] 3.5 确保幂等性：已初始化的项目不会重复初始化

## 4. Tauri 命令注册

- [x] 4.1 在 `src-tauri/src/main.rs` 的 `invoke_handler` 中注册 `check_git_status` 命令
- [x] 4.2 注册 `init_git_repo` 命令
- [x] 4.3 注册 `set_git_config` 命令
- [x] 4.4 注册 `get_git_config` 命令
- [x] 4.5 注册 `get_all_git_config` 命令

## 5. Initializer Agent 优化

- [x] 5.1 简化 `src-tauri/src/agent/initializer_agent.rs` 中的 Git 配置逻辑
- [x] 5.2 移除冗余的 Git 初始化检查（假设项目创建时已初始化）
- [x] 5.3 更新注释说明 Git 初始化由项目创建流程负责

## 6. WorktreeManager 简化

- [x] 6.1 简化 `src-tauri/src/agent/worktree_manager.rs` 中的 `ensure_git_initialized` 方法
- [x] 6.2 将方法重命名为 `validate_git_repository` 以反映其新职责
- [x] 6.3 更新日志消息，说明 Git 应在项目创建时已初始化
- [x] 6.4 如果检测到 `.git` 不存在，返回错误而非尝试初始化

## 7. 单元测试

- [x] 7.1 为 `check_git_status` 编写单元测试，测试有效仓库、非仓库、不存在目录三种情况
- [x] 7.2 为 `init_git_repo` 编写单元测试，验证 .git 目录创建、.gitignore 文件内容和初始 commit
- [x] 7.3 为 `set_git_config` 和 `get_git_config` 编写单元测试
- [x] 7.4 为 `create_project` 编写集成测试，验证 Git 自动初始化和 .gitignore 文件创建
- [x] 7.5 为 `get_project_by_id` 编写集成测试，验证打开项目时的 Git 检测和初始化

## 8. 前端验证测试

- [x] 8.1 手动测试 GitDetector 组件的 "Check Status" 功能
- [x] 8.2 手动测试 "Initialize Git Repository" 按钮（应检测到已初始化）
- [x] 8.3 验证 Git 配置显示正确（userName 和 userEmail）
- [x] 8.4 测试创建新项目后 GitDetector 的状态显示
- [x] 8.5 测试打开旧项目（无 Git）时自动初始化的行为

## 9. 文档和清理

- [x] 9.1 更新 AGENTS.md 或相关文档，说明项目创建和打开时的 Git 自动初始化行为
- [x] 9.2 检查并清理代码中的 TODO 注释（移除未使用的导入）
- [x] 9.3 运行 `cargo clippy` 确保无警告（修复了未使用的导入）
- [x] 9.4 运行 `npm run harness:check` 验证整体架构健康度

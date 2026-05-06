## Context

**当前状态**：
- 前端已实现完整的 Git 管理 UI（`GitDetector.tsx` + `useGit.ts`）
- 前端调用了 5 个 Tauri 命令，但这些命令未在后端实现
- 项目创建时仅创建工作区目录，不初始化 Git
- Git 初始化仅在创建 Worktree 时被动触发（`WorktreeManager::ensure_git_initialized`）
- Initializer Agent 仅配置 Git 用户信息，不执行 `git init`

**约束条件**：
- 必须保持向后兼容，不影响现有项目
- Git 是系统级依赖，无需新增外部库
- 需要支持 Windows、macOS、Linux 跨平台
- 遵循 Rust 异步编程最佳实践（使用 `tokio::process::Command`）

**利益相关者**：
- 前端开发者：需要可靠的 Git 管理 API
- 后端开发者：需要清晰的命令实现规范
- 最终用户：期望项目创建后立即具备版本控制能力

## Goals / Non-Goals

**Goals:**
1. 在 `create_project` 命令中自动执行 Git 初始化
2. 实现 5 个缺失的 Git 管理 Tauri 命令
3. 简化 Initializer Agent 和 WorktreeManager 中的冗余逻辑
4. 确保前端 GitDetector 组件完全可用
5. 提供完善的错误处理和日志记录

**Non-Goals:**
1. 不实现复杂的 Git 操作（如分支管理、合并、rebase 等）— 这些由 BranchManager 处理
2. 不修改现有的 Git 工作流或策略
3. 不添加 Git GUI 功能
4. 不实现 Git hook 管理
5. 不处理远程仓库配置（push/pull/fetch）

## Decisions

### Decision 1: Git 初始化时机

**选择**：在 `create_project` 命令中，创建工作区目录后立即执行 `git init`

**理由**：
- ✅ 符合"项目创建即具备版本控制"的最佳实践
- ✅ 避免后续操作的被动初始化带来的不确定性
- ✅ 简化 WorktreeManager 和 InitializerAgent 的逻辑
- ✅ 用户体验一致且可预测

**备选方案**：
- ❌ 懒加载（首次需要时初始化）：导致行为不一致，增加复杂度
- ❌ 让用户手动初始化：增加用户负担，容易遗漏

**实现位置**：
```rust
// src-tauri/src/commands/database.rs
pub fn create_project(...) -> Result<String, String> {
    let project_id = Uuid::new_v4().to_string();
    let workspace_path = create_workspace_directory(&project_id)?;
    
    // ✨ 新增：立即初始化 Git
    initialize_git_repository(&workspace_path)?;
    
    db::create_project(&conn, &project)?;
    Ok(project.id)
}
```

### Decision 2: Git 命令的实现方式

**选择**：使用 `tokio::process::Command` 调用系统 Git 命令

**理由**：
- ✅ 与现有代码库保持一致（WorktreeManager、BranchManager 均使用此方式）
- ✅ 跨平台兼容性好
- ✅ 异步非阻塞，适合 Tauri 环境
- ✅ 无需引入额外的 Git Rust 库（如 `git2`），减少依赖

**备选方案**：
- ❌ 使用 `git2` crate：增加依赖复杂度，学习曲线陡峭
- ❌ 使用同步 `std::process::Command`：可能阻塞主线程

**示例实现**：
```rust
async fn check_git_status(path: &str) -> Result<GitStatus, String> {
    let output = tokio::process::Command::new("git")
        .current_dir(path)
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .await
        .map_err(|e| format!("Failed to execute git: {}", e))?;
    
    let is_git_repo = output.status.success();
    // ... 其他检查
}
```

### Decision 3: Git 配置的用户名和邮箱策略

**选择**：优先使用全局配置，如果不存在则设置项目级默认值

**理由**：
- ✅ 尊重用户的全局 Git 配置
- ✅ 提供合理的默认值，避免配置缺失导致的错误
- ✅ 项目级配置不影响其他项目

**实现逻辑**：
```rust
async fn ensure_git_user_config(project_path: &str) -> Result<(), String> {
    // 检查全局配置
    let user_name = get_global_git_config("user.name").await?;
    let user_email = get_global_git_config("user.email").await?;
    
    if user_name.is_none() || user_email.is_none() {
        // 设置项目级默认值
        set_git_config(project_path, "user.name", "OPC-HARNESS User").await?;
        set_git_config(project_path, "user.email", "harness@opc.local").await?;
    }
}
```

### Decision 4: 错误处理策略

**选择**：Graceful Degradation（优雅降级）

**理由**：
- ✅ Git 初始化失败不应阻止项目创建
- ✅ 记录详细日志便于排查问题
- ✅ 返回警告而非错误，允许用户后续手动修复

**实现方式**：
```rust
// 在 create_project 中
match initialize_git_repository(&workspace_path) {
    Ok(_) => log::info!("Git repository initialized successfully"),
    Err(e) => {
        log::warn!("Git initialization failed (non-critical): {}", e);
        log::warn!("User can initialize Git manually later via GitDetector");
    }
}
```

### Decision 5: 命令注册的组织方式

**选择**：在 `system.rs` 中集中实现所有 Git 管理命令

**理由**：
- ✅ `system.rs` 已包含系统级工具命令（如文件读写）
- ✅ Git 管理属于系统基础设施，与 PRD 文件读写同类
- ✅ 避免创建新的模块文件，保持简洁

**备选方案**：
- ❌ 创建独立的 `git.rs` 模块：过度设计，命令数量不多
- ❌ 分散到 `database.rs`：职责不清，数据库命令应专注于数据持久化

### Decision 6: .gitignore 文件内容和策略

**选择**：在 Git 初始化时自动创建 `.gitignore` 文件，至少包含 `.opc-harness/` 目录

**理由**：
- ✅ `.opc-harness/` 目录包含 CodeFree CLI 上下文文件、临时数据等，不应纳入版本控制
- ✅ 自动创建确保所有新项目都有一致的忽略规则
- ✅ 用户可以后续添加其他忽略规则，不会冲突
- ✅ 符合最佳实践：项目特定的忽略规则应该在项目级别而非全局

**实现方式**：
```rust
async fn create_gitignore(project_path: &str) -> Result<(), String> {
    let gitignore_path = Path::new(project_path).join(".gitignore");
    
    // 如果 .gitignore 已存在，跳过创建
    if gitignore_path.exists() {
        return Ok(());
    }
    
    // OPC-HARNESS 项目的标准 .gitignore 内容
    let content = "# OPC-HARNESS context files\n.opc-harness/\n";
    
    tokio::fs::write(&gitignore_path, content)
        .await
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
    
    Ok(())
}
```

**备选方案**：
- ❌ 使用全局 .gitignore：不够灵活，不同项目可能有不同需求
- ❌ 不创建 .gitignore：用户需要手动配置，容易遗漏
- ❌ 创建完整的 .gitignore（包含 node_modules、target 等）：过度复杂，这些应该在模板项目中处理

### Decision 7: 项目打开时的 Git 初始化时机

**选择**：在 `get_project_by_id` 命令中检测并初始化 Git，作为项目加载流程的一部分

**理由**：
- ✅ 确保用户每次打开项目时都有可用的 Git 仓库
- ✅ 修复历史遗留问题：之前创建的项目可能没有 Git 初始化
- ✅ 透明的用户体验：自动修复，无需用户干预
- ✅ 幂等性：如果已初始化则跳过，不会重复操作

**实现位置**：
```rust
// src-tauri/src/commands/database.rs
#[tauri::command]
pub fn get_project_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Option<Project>, String> {
    let conn = db::get_connection().map_err(|e| e.to_string())?;
    
    match db::get_project_by_id(&conn, &id)? {
        Some(project) => {
            // ✨ 新增：检查工作区目录并初始化 Git（如果需要）
            ensure_workspace_and_git(&id, &app_handle)?;
            
            Ok(Some(project))
        }
        None => Ok(None),
    }
}

fn ensure_workspace_and_git(project_id: &str, app_handle: &tauri::AppHandle) -> Result<(), String> {
    use crate::utils::paths::get_workspaces_dir;
    use std::path::Path;
    
    let workspaces_root = get_workspaces_dir();
    let workspace_path = workspaces_root.join(project_id);
    
    // 如果工作区目录不存在，创建它
    if !workspace_path.exists() {
        log::info!("[get_project_by_id] Workspace directory missing, recreating: {:?}", workspace_path);
        std::fs::create_dir_all(&workspace_path)
            .map_err(|e| format!("Failed to create workspace directory: {}", e))?;
    }
    
    // 检查 Git 是否已初始化
    let git_dir = workspace_path.join(".git");
    if !git_dir.exists() {
        log::info!("[get_project_by_id] Git not initialized, initializing now...");
        // 调用 Git 初始化逻辑（复用 create_project 中的函数）
        initialize_git_repository(&workspace_path.to_string_lossy())?;
    }
    
    Ok(())
}
```

**备选方案**：
- ❌ 在前端组件中检测并初始化：职责不清，后端应该负责数据完整性
- ❌ 仅在 Initializer Agent 中处理：太晚，用户可能在运行 Agent 之前就期望 Git 可用
- ❌ 不处理历史项目：导致不一致的用户体验

## Risks / Trade-offs

### Risk 1: Git 未安装在系统中

**风险**：如果用户系统未安装 Git，初始化会失败

**缓解措施**：
- 捕获错误并记录警告日志
- 不阻止项目创建流程
- 前端 GitDetector 可以检测并提示用户安装 Git
- 文档中明确说明 Git 是必需依赖

### Risk 2: 工作区目录权限问题

**风险**：在某些情况下可能无法在工作区目录执行 Git 命令

**缓解措施**：
- 使用 `current_dir` 确保在正确的目录执行命令
- 详细的错误日志包含路径信息
- 允许用户手动初始化作为后备方案

### Risk 3: 与现有 WorktreeManager 逻辑冲突

**风险**：WorktreeManager 的 `ensure_git_initialized` 可能会重复执行 `git init`

**缓解措施**：
- `ensure_git_initialized` 已包含 `.git` 目录存在性检查
- 如果已初始化，直接返回 `Ok(())`
- 简化该方法，移除注释说明项目创建时已初始化

### Risk 4: Initializer Agent 的配置覆盖问题

**风险**：Initializer Agent 可能会覆盖项目创建时设置的 Git 配置

**缓解措施**：
- Initializer Agent 仅在全局配置缺失时设置项目级配置
- 使用 `git config`（项目级）而非 `git config --global`
- 记录配置变更日志便于追踪

### Risk 5: .gitignore 文件已存在

**风险**：如果工作区目录中已经存在 `.gitignore` 文件（例如从模板复制），自动创建可能会覆盖用户自定义规则

**缓解措施**：
- 在创建前检查 `.gitignore` 是否存在
- 如果已存在，跳过创建并记录日志
- 允许用户手动添加 `.opc-harness/` 到现有的 `.gitignore`
- 提供文档说明推荐的忽略规则

### Risk 6: 项目打开时的性能影响

**风险**：在 `get_project_by_id` 中添加 Git 检测和初始化逻辑可能影响项目加载速度

**缓解措施**：
- Git 检测只是检查 `.git` 目录是否存在，非常快（毫秒级）
- 仅在未初始化时才执行完整的 git init 流程
- 使用异步操作避免阻塞主线程
- 记录详细的性能日志以便监控

### Trade-off: 简单性 vs 灵活性

**权衡**：选择在项目创建时自动初始化，牺牲了一定的灵活性（用户可能不需要 Git）

**理由**：
- OPC-HARNESS 是一个开发工具平台，版本控制是核心需求
- 99% 的使用场景都需要 Git
- 用户可以通过删除 `.git` 目录来禁用（虽然不推荐）
- 简化的架构带来的收益远大于灵活性的损失

## Migration Plan

### 部署步骤

1. **实现阶段**：
   - 在 `system.rs` 中实现 5 个 Git 管理命令
   - 修改 `database.rs` 的 `create_project` 函数
   - 在 `main.rs` 中注册新命令
   - 简化 `initializer_agent.rs` 和 `worktree_manager.rs`

2. **测试阶段**：
   - 单元测试：验证每个 Git 命令的正确性
   - 集成测试：创建新项目并验证 Git 初始化
   - E2E 测试：通过前端 GitDetector 组件测试完整流程

3. **部署阶段**：
   - 编译新版本 Tauri 应用
   - 发布更新
   - 更新文档说明 Git 自动初始化行为

### 回滚策略

如果发现问题：
1.  revert `create_project` 的 Git 初始化逻辑
2. 从 `invoke_handler` 中移除新注册的命令
3. 保留命令实现代码（标记为 deprecated），以便后续修复后重新启用

### 数据迁移

**无需数据迁移**：
- 现有项目的 `.git` 目录不受影响
- 新创建的项目会自动初始化
- 无数据库 schema 变更

## Open Questions

### Question 1: 是否应该创建初始 commit？

**当前设计**：在 `initialize_git_repository` 中创建一个空的初始 commit

**待决策**：
- 选项 A：创建空 commit（`git commit --allow-empty -m "Initial commit"`）
  - 优点：避免后续操作的警告，worktree 创建更顺畅
  - 缺点：用户可能需要自定义首次提交消息
  
- 选项 B：不创建初始 commit
  - 优点：用户完全控制首次提交
  - 缺点：某些 Git 操作可能需要至少一个 commit

**建议**：采用选项 A，因为：
- WorktreeManager 已经这样做
- 空 commit 不影响用户的实际代码历史
- 提供更好的开箱即用体验

### Question 2: 默认用户名和邮箱是否应该可配置？

**当前设计**：硬编码默认值为 `"OPC-HARNESS User"` 和 `"harness@opc.local"`

**待决策**：
- 选项 A：保持硬编码（简单）
- 选项 B：从应用配置中读取（灵活）
- 选项 C：在首次运行时询问用户（友好但复杂）

**建议**：采用选项 A，原因：
- 这是项目级配置，不影响用户的全局 Git 配置
- 用户可以在需要时手动修改
- 保持实现简单，后续可以根据反馈优化

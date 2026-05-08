## Architecture

### 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Vibe Coding Agent System                  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Frontend (React)          Backend (Rust/Tauri)            │
│  ┌──────────────┐         ┌──────────────────────┐        │
│  │ Checkpoint UI│◀───────▶│ AgentWorker          │        │
│  │              │ WebSocket│ • Scan pending       │        │
│  │ Progress     │          │ • Launch agents      │        │
│  │ Approval     │          │ • Cleanup worktrees  │        │
│  └──────────────┘         └──────────┬───────────┘        │
│                                      │                     │
│                           ┌──────────▼───────────┐        │
│                           │ NativeCodingAgent    │        │
│                           │ • Multi-turn loop    │        │
│                           │ • Tool execution     │        │
│                           │ • Quality checks     │        │
│                           └──────────┬───────────┘        │
│                                      │                     │
│                    ┌─────────────────┼─────────────────┐  │
│                    ▼                 ▼                 ▼  │
│           ┌──────────────┐ ┌──────────────┐ ┌──────────┐│
│           │CodeSearch    │ │Dependency    │ │Quality   ││
│           │Tools         │ │Management    │ │Tools     ││
│           │• grep/find   │ │• npm install │ │• lint    ││
│           │• symbol scan │ │• cargo add   │ │• test    ││
│           └──────────────┘ └──────────────┘ └──────────┘│
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 核心组件

1. **NativeCodingAgent** - 智能体核心执行引擎
2. **Tool Extensions** - 扩展的工具集（代码搜索、依赖管理）
3. **Checkpoint Manager** - HITL 检查点管理器
4. **Worktree Lifecycle Manager** - Worktree 生命周期管理
5. **Conversation History Optimizer** - 对话历史优化器

## Implementation Strategy

### Phase 1: 工具集扩展（基础能力）

#### 1.1 Code Search Tools

**文件**: `src-tauri/src/agent/tools/code_search.rs`

**功能**:
- `grep(pattern, path)` - 正则表达式搜索
- `find_files(pattern, extensions)` - 文件查找
- `find_symbol(symbol_name)` - 符号查找（函数/类/变量）

**实现要点**:
```rust
pub struct CodeSearchTools {
    workspace_root: PathBuf,
}

impl CodeSearchTools {
    pub async fn grep(&self, pattern: &str, path: Option<&str>) -> Result<Vec<SearchMatch>, String> {
        // 使用 ripgrep 或自定义实现
        // 返回匹配的行号、内容和上下文
    }
    
    pub async fn find_symbol(&self, symbol: &str) -> Result<Vec<SymbolLocation>, String> {
        // 解析 AST 或使用语言服务器协议
        // 返回符号定义位置
    }
}
```

**集成到 NativeCodingAgent**:
- 在 `execute_tool_call` 中添加 `code_search_grep` 和 `code_search_find_symbol` 分支
- 更新系统提示词，告知 AI 可用新工具

#### 1.2 Dependency Management Tools

**文件**: `src-tauri/src/agent/tools/dependency_manager.rs`

**功能**:
- `npm_install(package, version?)` - 安装 npm 包
- `cargo_add(crate, features?)` - 添加 Rust crate
- `list_dependencies()` - 列出当前依赖

**安全考虑**:
- 限制可安装的包来源（仅官方 registry）
- 记录所有依赖变更到日志
- 提供 dry-run 模式预览变更

**实现要点**:
```rust
pub struct DependencyManager {
    workspace_root: PathBuf,
    package_manager: PackageManager, // npm | cargo
}

impl DependencyManager {
    pub async fn npm_install(&self, package: &str, version: Option<&str>) -> Result<String, String> {
        // 验证包名合法性
        // 执行 npm install <package>@<version>
        // 返回安装结果和版本信息
    }
}
```

### Phase 2: HITL Checkpoint 机制（核心交互）

#### 2.1 Checkpoint 数据结构

**数据库表**: `agent_checkpoints`

```sql
CREATE TABLE agent_checkpoints (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    story_id TEXT NOT NULL,
    checkpoint_type TEXT NOT NULL, -- 'code_review' | 'dependency_approval' | 'architecture_decision'
    status TEXT NOT NULL, -- 'pending' | 'approved' | 'rejected'
    data JSONB NOT NULL, -- 检查点数据（代码diff、依赖列表等）
    user_feedback TEXT, -- 用户反馈
    created_at TIMESTAMP DEFAULT NOW(),
    resolved_at TIMESTAMP
);
```

#### 2.2 Checkpoint Manager

**文件**: `src-tauri/src/agent/checkpoint_manager.rs`

**核心逻辑**:
```rust
pub struct CheckpointManager {
    db: Arc<Database>,
    websocket: Arc<WebSocketManager>,
}

impl CheckpointManager {
    pub async fn create_checkpoint(
        &self,
        agent_id: &str,
        story_id: &str,
        checkpoint_type: CheckpointType,
        data: serde_json::Value,
    ) -> Result<String, String> {
        // 1. 创建 checkpoint 记录
        // 2. 通过 WebSocket 通知前端
        // 3. 暂停 Agent 执行，等待用户响应
    }
    
    pub async fn resolve_checkpoint(
        &self,
        checkpoint_id: &str,
        decision: CheckpointDecision, // Approve | Reject
        feedback: Option<String>,
    ) -> Result<(), String> {
        // 1. 更新 checkpoint 状态
        // 2. 恢复 Agent 执行或回滚
    }
}
```

#### 2.3 Agent 集成

**修改**: `native_coding_agent.rs`

在关键步骤插入 checkpoint：

```rust
// 在代码生成后
if self.config.enable_hitl {
    let diff = self.generate_code_diff();
    let checkpoint_id = self.checkpoint_manager
        .create_checkpoint("code_review", diff)
        .await?;
    
    // 等待用户决策（阻塞）
    let decision = self.wait_for_checkpoint_decision(&checkpoint_id).await?;
    
    match decision {
        CheckpointDecision::Approve => continue,
        CheckpointDecision::Reject => {
            self.rollback_changes();
            return Err(AgentError::UserRejected);
        }
    }
}
```

#### 2.4 前端 UI

**新增组件**: `src/components/vibe-coding/CheckpointApprovalDialog.tsx`

**功能**:
- 显示代码 diff（使用 react-diff-viewer）
- 批准/拒绝按钮
- 可选的反馈输入框
- 实时状态更新（通过 WebSocket）

### Phase 3: Worktree 自动清理（资源管理）

#### 3.1 Worktree Lifecycle Manager

**文件**: `src-tauri/src/agent/worktree_lifecycle.rs`

**核心逻辑**:
```rust
pub struct WorktreeLifecycleManager {
    worktree_manager: Arc<WorktreeManager>,
}

impl WorktreeLifecycleManager {
    pub async fn cleanup_after_story(
        &self,
        agent_id: &str,
        story_id: &str,
        outcome: StoryOutcome, // Success | Failure
    ) -> Result<(), String> {
        // 1. 获取 worktree 路径
        let worktree_path = self.get_worktree_path(agent_id)?;
        
        // 2. 删除 worktree
        tokio::fs::remove_dir_all(&worktree_path).await?;
        
        // 3. 删除 Git worktree 引用
        self.worktree_manager.remove_worktree(agent_id).await?;
        
        // 4. 记录清理日志
        log::info!("Cleaned up worktree for agent {} (story {})", agent_id, story_id);
        
        Ok(())
    }
}
```

#### 3.2 AgentWorker 集成

**修改**: `agent_worker.rs`

在 `execute_native_agent` 结束时调用清理：

```rust
match native_agent.execute_story(...).await {
    Ok(result) => {
        // ... 现有成功处理逻辑
        
        // 清理 worktree
        if let Err(e) = worktree_lifecycle.cleanup_after_story(agent_id, &story.id, StoryOutcome::Success).await {
            log::warn!("Failed to cleanup worktree: {}", e);
        }
        
        Ok(())
    }
    Err(e) => {
        // ... 现有失败处理逻辑
        
        // 即使失败也要清理
        if let Err(cleanup_err) = worktree_lifecycle.cleanup_after_story(agent_id, &story.id, StoryOutcome::Failure).await {
            log::warn!("Failed to cleanup worktree after failure: {}", cleanup_err);
        }
        
        Err(e)
    }
}
```

### Phase 4: 对话历史优化（性能提升）

#### 4.1 任务完成标记

**修改**: `native_coding_agent.rs`

在系统提示词中添加明确的完成信号：

```rust
fn build_system_prompt(&self, ...) -> String {
    format!(
        r#"...
        
**完成任务信号**:
当你认为任务已完成时，必须在回复中包含特殊标记：
<TASK_COMPLETE>
总结：[简要说明完成的工作]
</TASK_COMPLETE>

如果没有此标记，我将认为任务仍在进行中。
"#
    )
}
```

在解析响应时检测标记：

```rust
fn parse_completion_signal(&self, response: &str) -> bool {
    response.contains("<TASK_COMPLETE>")
}
```

#### 4.2 历史压缩

**实现策略**:
- 每 5 轮对话后触发压缩
- 将早期对话摘要为："Turn 1-5: AI 尝试读取文件 X，写入文件 Y，遇到错误 Z"
- 保留最近 2 轮的完整对话

```rust
async fn compress_history(&mut self) {
    if self.conversation_history.len() > 10 {
        // 保留 system message + 最近 4 条消息
        let recent = self.conversation_history.split_off(self.conversation_history.len() - 4);
        let old = std::mem::replace(&mut self.conversation_history, vec![system_msg]);
        
        // 生成摘要
        let summary = self.generate_summary(&old).await;
        self.conversation_history.push(Message {
            role: "system".to_string(),
            content: format!("Previous conversation summary: {}", summary),
        });
        
        // 追加最近消息
        self.conversation_history.extend(recent);
    }
}
```

### Phase 5: 质量检查改进（可靠性）

#### 5.1 分阶段检查

**修改**: `src-tauri/src/agent/tools/quality.rs`

```rust
pub async fn run_quality_checks_staged(&self) -> Result<StagedQualityResult, String> {
    // Stage 1: Lint
    let lint_result = self.run_lint().await?;
    if !lint_result.passed {
        return Ok(StagedQualityResult {
            stage: "lint",
            passed: false,
            errors: lint_result.errors,
        });
    }
    
    // Stage 2: Type check
    let typecheck_result = self.run_type_check().await?;
    if !typecheck_result.passed {
        return Ok(StagedQualityResult {
            stage: "type-check",
            passed: false,
            errors: typecheck_result.errors,
        });
    }
    
    // Stage 3: Tests
    let test_result = self.run_tests().await?;
    
    Ok(StagedQualityResult {
        stage: "complete",
        passed: test_result.passed,
        errors: test_result.errors,
    })
}
```

#### 5.2 增量检查

**优化**: 只检查修改过的文件

```rust
pub async fn run_incremental_lint(&self, modified_files: &[PathBuf]) -> Result<QualityResult, String> {
    // 使用 ESLint 的 --cache 选项
    // 或手动过滤只检查修改的文件
    let args: Vec<String> = modified_files
        .iter()
        .map(|f| f.to_string_lossy().to_string())
        .collect();
    
    self.execute_lint_with_args(&args).await
}
```

## Data Flow

### Checkpoint 流程

```
Agent 生成代码
    │
    ▼
检测到需要 checkpoint
    │
    ▼
创建 checkpoint 记录 (DB)
    │
    ▼
发送 WebSocket 事件 → 前端显示审批对话框
    │
    ▼
用户点击 批准/拒绝
    │
    ▼
前端发送决策 → 后端更新 checkpoint
    │
    ├─ 批准 → Agent 继续执行
    └─ 拒绝 → Agent 回滚并终止
```

### Worktree 清理流程

```
Story 执行完成（成功/失败）
    │
    ▼
AgentWorker 调用 cleanup_after_story
    │
    ▼
删除 worktree 目录
    │
    ▼
移除 Git worktree 引用
    │
    ▼
记录清理日志
```

## Testing Strategy

### 单元测试

1. **CodeSearchTools**
   - 测试 grep 在不同文件类型上的表现
   - 测试路径安全检查（防止越界访问）

2. **DependencyManager**
   - 测试包名验证逻辑
   - Mock npm/cargo 命令执行

3. **CheckpointManager**
   - 测试 checkpoint 创建和解析
   - 测试并发 checkpoint 处理

4. **WorktreeLifecycle**
   - 测试 worktree 创建和清理
   - 测试异常情况下的清理（panic 恢复）

### 集成测试

1. **端到端 HITL 流程**
   - 启动 Agent → 触发 checkpoint → 模拟用户批准 → 验证继续执行
   - 启动 Agent → 触发 checkpoint → 模拟用户拒绝 → 验证回滚

2. **Worktree 清理验证**
   - 执行 Story → 验证 worktree 被删除
   - 执行失败的 Story → 验证 worktree 仍被删除

### 性能测试

1. **对话历史压缩**
   - 测量压缩前后的 token 数量
   - 测量压缩对响应时间的影响

2. **增量质量检查**
   - 对比全量检查和增量检查的时间差异

## Risks and Mitigations

### Risk 1: HITL 导致执行延迟

**问题**: 等待用户审批会阻塞 Agent 执行

**缓解**:
- 设置超时机制（例如 30 分钟无响应自动拒绝）
- 允许配置哪些 checkpoint 是必需的，哪些可以跳过
- 提供批量审批功能

### Risk 2: Worktree 清理过早

**问题**: 用户在 Story 完成后想查看生成的代码，但 worktree 已被删除

**缓解**:
- 在清理前将代码合并到主分支或创建永久分支
- 提供配置选项控制是否立即清理
- 记录 worktree 路径到日志，方便手动恢复

### Risk 3: 依赖安装安全风险

**问题**: AI 可能安装恶意包

**缓解**:
- 仅允许从官方 registry 安装包
- 记录所有依赖变更到审计日志
- 提供白名单机制，限制可安装的包

### Risk 4: 对话历史压缩丢失关键信息

**问题**: 摘要可能遗漏重要上下文

**缓解**:
- 保留最近的完整对话（至少 2 轮）
- 在摘要中标记关键决策和错误
- 提供配置选项禁用压缩（用于调试）

## Migration Plan

### 向后兼容性

所有新功能都是**可选的**，通过配置开关控制：

```rust
pub struct NativeAgentConfig {
    // ... 现有字段
    
    // 新功能开关
    pub enable_hitl: bool,
    pub enable_code_search: bool,
    pub enable_dependency_management: bool,
    pub enable_history_compression: bool,
    pub auto_cleanup_worktree: bool,
}
```

默认值全部为 `false`，确保现有行为不变。

### 数据库迁移

新增 `agent_checkpoints` 表：

```sql
-- migrations/005_create_agent_checkpoints.sql
CREATE TABLE IF NOT EXISTS agent_checkpoints (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    story_id TEXT NOT NULL,
    checkpoint_type TEXT NOT NULL,
    status TEXT NOT NULL,
    data JSONB NOT NULL,
    user_feedback TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMP
);

CREATE INDEX idx_checkpoints_agent ON agent_checkpoints(agent_id);
CREATE INDEX idx_checkpoints_story ON agent_checkpoints(story_id);
CREATE INDEX idx_checkpoints_status ON agent_checkpoints(status);
```

### 逐步 rollout

1. **Phase 1** (Week 1): 部署工具集扩展，内部测试
2. **Phase 2** (Week 2-3): 部署 HITL checkpoint，小范围用户测试
3. **Phase 3** (Week 4): 部署 worktree 清理，全面启用
4. **Phase 4-5** (Week 5-6): 优化性能和用户体验

## Success Metrics

1. **自主性提升**: AI 能够独立完成的任务比例从 60% → 85%
2. **用户满意度**: HITL 干预后的成功率从 70% → 95%
3. **资源效率**: 磁盘空间占用减少 50%（通过 worktree 清理）
4. **成本优化**: Token 消耗减少 30%（通过历史压缩）
5. **质量提升**: 首次通过率从 65% → 80%（通过分阶段质量检查）

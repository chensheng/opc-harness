## Context

**当前状态**:
- Vibe Coding 智能体通过 `AgentWorker` 启动外部 CLI 进程（Kimi/Claude Code）
- 使用 `AICLIInteraction` 进行 STDIO 双向通信
- CLI 参数格式不统一，Kimi CLI 不支持 `--story-id` 参数导致任务失败
- 调试困难，错误信息需要通过解析 STDOUT/STDERR 获取

**约束条件**:
- 必须保持前端 API 不变（Tauri Command 接口稳定）
- 必须支持多 AI Provider（OpenAI/Anthropic/Kimi/GLM）
- 必须保证文件系统操作安全（沙箱隔离）
- 必须与现有 Git Worktree 机制兼容
- Rust 代码必须符合项目健康评分要求（零警告）

**利益相关者**:
- 前端团队：需要无缝迁移，无感知切换
- 后端团队：需要清晰的架构和可维护的代码
- 用户：需要更稳定的编码体验和更快的执行速度

## Goals / Non-Goals

**Goals:**
1. 实现纯 Rust 的 Native Coding Agent，完全摆脱 CLI 依赖
2. 提供统一的 AI Provider 抽象层，支持多模型自由切换
3. 实现安全的文件系统工具集，支持工作空间沙箱隔离
4. 集成 Git 操作工具，支持版本控制和分支管理
5. 实现代码质量检查自动化（Lint/Test/Type Check）
6. 保持向后兼容，支持配置开关降级到 CLI 模式

**Non-Goals:**
1. 不实现完整的 LangChain/Rig 框架（避免过度抽象）
2. 不支持离线模式（必须联网调用 AI API）
3. 不实现复杂的 Agent 编排（如 ReAct、Plan-and-Execute）
4. 不实现 RAG 系统（文档检索非核心需求）
5. 不实现多 Agent 协作（单 Agent 执行单 Story）

## Decisions

### Decision 1: 不使用 LangChain Rust，采用自研轻量级实现

**选择**: 自研 Native Coding Agent，直接调用 AI Provider API

**理由**:
- LangChain Rust 项目成熟度不足（版本迭代过快，docs.rs 构建失败）
- 内置工具有限，仍需大量自研（FileSystem/Git/Quality Tools）
- 引入额外抽象层增加性能开销和学习成本
- 自研实现更可控，长期维护成本更低

**备选方案**:
- ❌ LangChain Rust：项目不稳定，社区支持弱
- ❌ Rig Framework：学习曲线陡峭，团队经验不足
- ✅ 自研：完全可控，性能最优，符合项目技术栈

---

### Decision 2: 使用 Function Calling 而非文本解析

**选择**: 基于 AI Provider 原生的 Function Calling 能力

**理由**:
- 类型安全：工具调用通过结构化数据，非正则解析
- Token 效率高：不需要在 Prompt 中描述工具格式
- 可靠性高：模型原生支持，错误率低
- Kimi K2.5、GPT-4、Claude 3.5 均支持 Function Calling

**备选方案**:
- ❌ ReAct 模式：依赖正则解析，可靠性低，Token 消耗高
- ✅ Function Calling：类型安全，高效可靠

---

### Decision 3: 文件系统工具采用沙箱隔离

**选择**: 所有文件操作限制在工作空间根目录内

**实现**:
```rust
fn validate_path(&self, path: &str) -> Result<PathBuf, String> {
    let full_path = self.workspace_root.join(path);
    if !full_path.starts_with(&self.workspace_root) {
        return Err("Access denied: path outside workspace".to_string());
    }
    Ok(full_path)
}
```

**理由**:
- 防止 AI 误操作系统文件
- 确保多 Agent 并行执行时互不干扰
- 符合最小权限原则

---

### Decision 4: 增量代码编辑使用 Diff-based 策略

**选择**: AI 生成代码片段 + 应用 Patch，而非全量覆盖

**实现**:
- 使用 `diffy` crate 计算和应用 diff
- AI 输出格式：`<<<<<<< ORIGINAL ... ======= ... >>>>>>> NEW`
- 支持部分文件修改，保留用户手动更改

**理由**:
- 避免覆盖用户的临时修改
- 减少 Token 消耗（只传输变更部分）
- 支持并发编辑同一文件的不同区域

**备选方案**:
- ❌ 全量覆盖：简单但会丢失用户修改
- ✅ Diff-based：复杂但更安全

---

### Decision 5: 质量检查采用异步并行执行

**选择**: ESLint、TypeScript Check、Test Runner 并行执行

**实现**:
```rust
let (lint_result, ts_result, test_result) = tokio::join!(
    run_eslint(&workspace),
    run_typescript_check(&workspace),
    run_tests(&workspace),
);
```

**理由**:
- 减少总执行时间（串行可能需要 30s+，并行只需 10s）
- 快速反馈，提升用户体验
- 充分利用多核 CPU

---

### Decision 6: 错误自动修复采用有限重试循环

**选择**: 最多 3 次自动修复尝试，失败后标记为 permanently_failed

**实现**:
```rust
for attempt in 1..=3 {
    let result = agent.execute(prompt).await?;
    
    match run_quality_checks() {
        Ok(_) => break, // 成功
        Err(e) if attempt < 3 => {
            prompt = format!("Previous attempt failed: {}\nFix the issue.", e);
            continue;
        }
        Err(e) => return Err(e), // 达到最大重试次数
    }
}
```

**理由**:
- 避免无限循环消耗 API 配额
- 平衡自动化程度和成本控制
- 失败后交由 RetryScheduler 处理

---

### Decision 7: 配置开关支持渐进式迁移

**选择**: 通过环境变量 `VITE_USE_NATIVE_AGENT=true` 控制

**实现**:
```rust
// AgentWorker
if std::env::var("VITE_USE_NATIVE_AGENT") == Ok("true".to_string()) {
    self.execute_native_agent(story).await?;
} else {
    self.execute_cli_agent(story).await?; // 降级方案
}
```

**理由**:
- 允许灰度测试，降低风险
- 发现问题时可快速回退
- 给用户选择权

## Risks / Trade-offs

### Risk 1: AI Provider API 限流

**风险**: 高频调用可能触发 API 速率限制

**缓解**:
- 实现指数退避重试策略
- 添加请求队列，控制并发数
- 缓存常见问题的响应（可选）

---

### Risk 2: 大文件处理性能问题

**风险**: 读取/写入超大文件（>1MB）可能导致内存溢出

**缓解**:
- 限制单次读取文件大小（最大 500KB）
- 大文件采用分块读取策略
- 监控内存使用，超过阈值时拒绝操作

---

### Risk 3: Git 操作冲突

**风险**: 多个 Agent 同时修改同一文件可能导致合并冲突

**缓解**:
- 每个 Agent 使用独立的 Git Worktree
- 提交前检查文件锁定状态
- 冲突时自动标记为 failed，由人工介入

---

### Risk 4: Prompt Injection 攻击

**风险**: 恶意用户故事内容可能注入恶意指令

**缓解**:
- 对用户输入进行 sanitization
- 限制工具调用的权限范围
- 记录所有工具调用日志用于审计

---

### Trade-off 1: 开发复杂度 vs 灵活性

**权衡**: 自研实现增加初期开发成本，但获得完全控制权

**决策**: 接受更高的初期投入，换取长期灵活性和可维护性

---

### Trade-off 2: 功能完整性 vs 简洁性

**权衡**: 不实现完整的 Agent 框架（如 LangChain），牺牲部分高级功能

**决策**: 聚焦核心需求（代码生成+文件操作+质量检查），保持代码简洁

## Migration Plan

### Phase 1: 基础实现（1-2 周）
1. 实现 NativeCodingAgent 核心结构
2. 实现 FileSystemTools（read/write/list）
3. 实现 GitTools（status/diff/commit）
4. 单元测试覆盖率达到 70%

### Phase 2: 集成测试（1 周）
1. 集成到 AgentWorker
2. 端到端测试单个 Story 执行
3. 性能基准测试（对比 CLI 模式）

### Phase 3: 灰度发布（1 周）
1. 添加配置开关 `VITE_USE_NATIVE_AGENT`
2. 内部团队试用，收集反馈
3. 修复发现的问题

### Phase 4: 全面推广（1 周）
1. 默认启用 Native Agent
2. 保留 CLI 模式作为降级方案
3. 监控生产环境指标

### Rollback Strategy
- 发现严重问题时，设置 `VITE_USE_NATIVE_AGENT=false` 立即回退
- 保留 `ai_cli_interaction.rs` 模块至少 3 个月
- 前端无需修改，回滚透明

## Open Questions

1. **是否需要实现 Tool 调用日志持久化？**
   - 当前方案：仅内存记录
   - 备选：写入数据库用于审计
   
2. **是否支持自定义 Tool 注册？**
   - 当前方案：硬编码内置工具
   - 备选：插件化架构，允许用户扩展

3. **是否需要实现 Agent 间的通信机制？**
   - 当前方案：单 Agent 独立执行
   - 备选：支持 Initializer → Coding → MR 链路传递上下文

4. **如何处理 AI 生成的代码包含敏感信息？**
   - 当前方案：信任 AI Provider 的安全性
   - 备选：实现内容过滤器（增加复杂度）

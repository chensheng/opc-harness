## ADDED Requirements

### Requirement: Native Coding Agent 核心执行引擎

系统 SHALL 提供纯 Rust 实现的自主编码智能体，直接调用 AI Provider API 执行用户故事，无需依赖外部 CLI 工具。

#### Scenario: 成功执行用户故事
- **WHEN** NativeCodingAgent 接收到有效的用户故事（包含标题和验收标准）
- **THEN** 系统构建上下文信息（项目结构、相关文件）
- **AND** 调用 AI Provider API 生成代码实现
- **AND** 应用代码更改到工作空间
- **AND** 返回执行结果（成功/失败）

#### Scenario: Function Calling 工具调用
- **WHEN** AI 模型决定需要读取文件或执行 Git 操作
- **THEN** 系统解析 Function Call 请求
- **AND** 执行对应的工具（read_file/git_status 等）
- **AND** 将工具执行结果返回给 AI 模型
- **AND** AI 模型基于结果继续生成代码

#### Scenario: 多轮对话上下文管理
- **WHEN** 单次 Story 执行需要多轮 AI 交互
- **THEN** 系统维护对话历史（最多保留最近 10 轮）
- **AND** 每轮对话包含：用户指令、AI 响应、工具调用结果
- **AND** 超过阈值时自动摘要历史以节省 Token

#### Scenario: AI Provider 切换
- **WHEN** 用户配置不同的 AI Provider（Kimi/GPT-4/Claude）
- **THEN** 系统使用对应的 API Key 和 Base URL
- **AND** 保持相同的工具调用接口
- **AND** 前端无感知切换

---

### Requirement: 错误自动修复循环

系统 SHALL 在代码质量检查失败时自动触发修复循环，最多尝试 3 次。

#### Scenario: 首次执行失败自动修复
- **WHEN** AI 生成的代码通过 Lint/Test 检查失败
- **THEN** 系统将错误信息反馈给 AI 模型
- **AND** AI 模型生成修复后的代码
- **AND** 重新执行质量检查

#### Scenario: 达到最大重试次数
- **WHEN** 连续 3 次修复尝试均失败
- **THEN** 系统将 Story 状态标记为 permanently_failed
- **AND** 记录详细错误日志
- **AND** 通知用户手动介入

#### Scenario: 修复成功后继续执行
- **WHEN** 第 2 次或第 3 次修复尝试通过质量检查
- **THEN** 系统提交代码更改
- **AND** 将 Story 状态标记为 completed
- **AND** 记录修复次数用于统计分析

---

### Requirement: 增量代码编辑支持

系统 SHALL 支持 Diff-based 增量代码编辑，避免覆盖用户手动修改。

#### Scenario: 应用代码 Patch
- **WHEN** AI 输出包含 diff 格式的代码变更
- **THEN** 系统解析 `<<<<<<< ORIGINAL` 和 `>>>>>>> NEW` 标记
- **AND** 计算差异并应用到目标文件
- **AND** 保留文件中未被修改的部分

#### Scenario: 并发编辑冲突检测
- **WHEN** 多个 Agent 同时修改同一文件的不同区域
- **THEN** 系统检测行级冲突
- **AND** 若无重叠则合并成功
- **AND** 若有重叠则标记为冲突，要求人工解决

---

### Requirement: 执行超时控制

系统 SHALL 对单个 Story 执行设置超时限制，防止无限运行。

#### Scenario: 正常执行完成
- **WHEN** Story 在超时时间内（默认 30 分钟）完成
- **THEN** 系统返回执行结果
- **AND** 释放资源

#### Scenario: 执行超时
- **WHEN** Story 执行超过 30 分钟
- **THEN** 系统终止 AI 会话
- **AND** 清理临时文件
- **AND** 将 Story 标记为 failed (timeout)
- **AND** 记录超时日志用于优化

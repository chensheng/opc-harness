## ADDED Requirements

### Requirement: Git 状态查询

系统 SHALL 提供 Git 仓库状态查询工具，返回未提交的更改列表。

#### Scenario: 查询工作区状态
- **WHEN** AI 调用 git_status 工具
- **THEN** 系统执行 `git status --porcelain`
- **AND** 解析输出为结构化数据
- **AND** 返回 modified/added/deleted/untracked 文件列表

#### Scenario: 空仓库处理
- **WHEN** 工作区不是有效的 Git 仓库
- **THEN** 系统返回错误 "Not a git repository"
- **AND** 建议初始化仓库

---

### Requirement: Git 差异对比

系统 SHALL 提供文件差异对比工具，支持 staged 和 unstaged 变更。

#### Scenario: 查看文件差异
- **WHEN** AI 调用 git_diff 工具并提供文件路径
- **THEN** 系统执行 `git diff <file>`
- **AND** 返回 unified diff 格式的差异内容
- **AND** 包含行号和变更标记（+/-）

#### Scenario: 查看所有变更
- **WHEN** AI 不提供文件路径参数
- **THEN** 系统执行 `git diff HEAD`
- **AND** 返回所有未提交文件的差异汇总

---

### Requirement: Git 提交操作

系统 SHALL 提供安全的 Git 提交工具，自动生成符合规范的提交消息。

#### Scenario: 提交代码变更
- **WHEN** AI 调用 git_commit 工具并提供变更摘要
- **THEN** 系统执行 `git add .`
- **AND** 生成符合 Conventional Commits 规范的提交消息
- **AND** 执行 `git commit -m "<message>"`
- **AND** 返回 commit hash

#### Scenario: 无变更时拒绝提交
- **WHEN** 工作区没有未提交的变更
- **THEN** 系统拒绝提交
- **AND** 返回错误 "No changes to commit"

---

### Requirement: Git Worktree 管理

系统 SHALL 提供 Git Worktree 创建和管理工具，支持多 Agent 并行执行。

#### Scenario: 创建新 Worktree
- **WHEN** AgentWorker 为新 Story 调用 create_worktree 工具
- **THEN** 系统基于当前 HEAD 创建新分支 `agent-{agent_id}`
- **AND** 检出到 `.worktrees/agent-{agent_id}` 目录
- **AND** 返回 worktree 路径

#### Scenario: Worktree 已存在
- **WHEN** 尝试为已存在的 agent_id 创建 worktree
- **THEN** 系统检测到路径已存在
- **AND** 返回错误 "Worktree already exists"
- **AND** 不执行 git worktree add 命令

#### Scenario: Worktree 清理
- **WHEN** Story 执行完成后调用 cleanup_worktree 工具
- **THEN** 系统删除 worktree 目录
- **AND** 执行 `git worktree remove`
- **AND** 删除对应分支

---

### Requirement: Git 分支操作

系统 SHALL 提供分支创建、切换、删除等操作工具。

#### Scenario: 创建新分支
- **WHEN** AI 调用 create_branch 工具
- **THEN** 系统执行 `git checkout -b <branch_name>`
- **AND** 返回新分支名称

#### Scenario: 列出所有分支
- **WHEN** AI 调用 list_branches 工具
- **THEN** 系统执行 `git branch -a`
- **AND** 返回本地和远程分支列表
- **AND** 标记当前分支

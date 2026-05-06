# Git 自动初始化功能 - 前端验证测试指南

## 测试环境准备

1. 确保应用已编译并运行：`npm run tauri:dev`
2. 准备一个干净的环境（可选）：删除 `~/.opc-harness/workspaces/` 目录以测试新项目创建

---

## 测试用例 1：GitDetector 组件状态检查

### 步骤
1. 打开任意项目页面
2. 找到 GitDetector 组件（通常在项目设置或侧边栏）
3. 点击 "Check Status" 按钮

### 预期结果
- ✅ 如果项目是新创建的，应显示：
  - Is Git Repo: ✓ Yes
  - Branch: main
  - Commit Count: 1 (初始空提交)
  - Is Dirty: No
  - User Name: OPC-HARNESS User (或自定义名称)
  - User Email: harness@opc.local (或自定义邮箱)

- ✅ 如果项目是旧的且未初始化，应显示：
  - Is Git Repo: ✗ No
  - 其他字段为空

---

## 测试用例 2：Initialize Git Repository 按钮

### 场景 A：项目已初始化
1. 创建一个新项目（会自动初始化 Git）
2. 打开 GitDetector 组件
3. 查看 "Initialize Git Repository" 按钮

**预期结果**：
- ✅ 按钮应显示为禁用状态或提示 "Already initialized"
- ✅ 点击后不应有任何变化（幂等性）

### 场景 B：手动初始化旧项目
1. 找到一个未初始化 Git 的旧项目
2. 点击 "Initialize Git Repository" 按钮

**预期结果**：
- ✅ 按钮变为加载状态
- ✅ 初始化成功后，Git 状态更新为已初始化
- ✅ .gitignore 文件自动创建
- ✅ 初始 commit 创建成功

---

## 测试用例 3：Git 配置显示

### 步骤
1. 打开已初始化 Git 的项目
2. 在 GitDetector 中查看配置信息

### 预期结果
- ✅ User Name 和 User Email 正确显示
- ✅ 如果全局配置存在，显示全局配置值
- ✅ 如果全局配置不存在，显示默认值（OPC-HARNESS User / harness@opc.local）

### 测试修改配置
1. 点击 "Set Config" 或类似按钮（如果有）
2. 输入自定义的用户名和邮箱
3. 重新检查状态

**预期结果**：
- ✅ 配置更新成功
- ✅ 状态显示新的配置值

---

## 测试用例 4：创建新项目后的 Git 状态

### 步骤
1. 在主界面点击 "Create New Project"
2. 输入项目名称和描述
3. 等待项目创建完成
4. 导航到项目页面
5. 打开 GitDetector 组件

### 预期结果
- ✅ Git 状态显示为已初始化
- ✅ 分支为 "main"
- ✅ 有 1 个初始提交
- ✅ .gitignore 文件存在且包含 `.opc-harness/`
- ✅ 工作区目录结构正确：`~/.opc-harness/workspaces/{project_id}/`

### 验证文件系统
```bash
# 在终端中执行
ls -la ~/.opc-harness/workspaces/{project_id}/
```

**预期结果**：
- ✅ `.git/` 目录存在
- ✅ `.gitignore` 文件存在
- ✅ 可以执行 `git log` 看到初始提交

---

## 测试用例 5：打开旧项目时的自动初始化

### 准备工作
1. 找到一个在 Git 自动初始化功能添加之前创建的项目
2. 或者手动删除项目的 `.git` 目录来模拟旧项目

```bash
# 模拟旧项目
rm -rf ~/.opc-harness/workspaces/{old_project_id}/.git
```

### 步骤
1. 在项目列表中点击该旧项目
2. 观察控制台日志（开发者工具）
3. 打开 GitDetector 组件

### 预期结果
- ✅ 控制台显示警告日志：`[get_project_by_id] Git not initialized, initializing now...`
- ✅ Git 自动初始化成功
- ✅ GitDetector 显示已初始化状态
- ✅ .gitignore 文件被创建
- ✅ 项目正常打开，无错误

### 验证日志
在浏览器开发者工具的 Console 中查找：
```
[get_project_by_id] Git not initialized, initializing now...
✓ Git repository initialized at ...
```

---

## 测试用例 6：.gitignore 内容验证

### 步骤
1. 打开任意新创建的项目
2. 在工作区目录中找到 `.gitignore` 文件
3. 查看文件内容

### 预期结果
```
# OPC-HARNESS context files
.opc-harness/
```

- ✅ 文件存在
- ✅ 包含注释行
- ✅ 包含 `.opc-harness/` 条目
- ✅ 没有多余的空行或格式问题

### 测试不被覆盖
1. 手动编辑 `.gitignore`，添加自定义规则：
   ```
   # My custom rules
   *.log
   node_modules/
   ```
2. 重新初始化 Git（通过 GitDetector 或删除 .git 后重新打开项目）

**预期结果**：
- ✅ 原有自定义规则保持不变
- ✅ 不会被覆盖或重置

---

## 测试用例 7：WorktreeManager 验证

### 步骤
1. 创建一个 Vibe Coding 任务（触发 Agent）
2. 观察 Worktree 创建过程
3. 检查控制台日志

### 预期结果
- ✅ Worktree 创建成功
- ✅ 日志显示：`[WorktreeManager] ✓ Git repository verified at ...`
- ✅ 不会出现 Git 初始化相关的错误
- ✅ Worktree 目录正确创建在 `worktrees/` 子目录下

---

## 测试用例 8：Initializer Agent 验证

### 步骤
1. 启动一个新项目
2. 观察 Initializer Agent 的执行日志
3. 检查是否有 Git 相关的警告

### 预期结果
- ✅ Initializer Agent 不再尝试初始化 Git
- ✅ 日志显示：`✅ Git 仓库已初始化：...`
- ✅ 如果出现警告，应该是关于旧项目的提示

---

## 常见问题排查

### 问题 1：Git 未初始化
**症状**：GitDetector 显示 "Is Git Repo: No"

**解决方案**：
1. 检查项目是否是新创建的（应该自动初始化）
2. 如果是旧项目，点击 "Initialize Git Repository" 按钮
3. 检查控制台错误日志

### 问题 2：.gitignore 缺失
**症状**：工作区目录中没有 `.gitignore` 文件

**解决方案**：
1. 重新初始化 Git（删除 .git 后重新打开项目）
2. 检查是否有权限问题
3. 手动创建 `.gitignore` 文件

### 问题 3：Git 配置未设置
**症状**：User Name/Email 显示为空

**解决方案**：
1. 设置全局 Git 配置：
   ```bash
   git config --global user.name "Your Name"
   git config --global user.email "your@email.com"
   ```
2. 或者通过 GitDetector 设置项目级配置

### 问题 4：Worktree 创建失败
**症状**：Agent 执行时出现 Git 相关错误

**解决方案**：
1. 验证主项目 Git 已正确初始化
2. 检查磁盘空间
3. 查看 WorktreeManager 日志

---

## 测试完成检查清单

- [ ] 测试用例 1：GitDetector 状态检查 ✓
- [ ] 测试用例 2：Initialize 按钮功能 ✓
- [ ] 测试用例 3：Git 配置显示 ✓
- [ ] 测试用例 4：新项目创建验证 ✓
- [ ] 测试用例 5：旧项目自动初始化 ✓
- [ ] 测试用例 6：.gitignore 内容验证 ✓
- [ ] 测试用例 7：WorktreeManager 验证 ✓
- [ ] 测试用例 8：Initializer Agent 验证 ✓

**所有测试通过后，标记任务 8.1-8.5 为完成。**

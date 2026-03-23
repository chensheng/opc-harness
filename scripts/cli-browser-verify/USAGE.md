# CLI Browser 验证使用指南

## 最简单的方式：直接对话

既然你正在使用 Kimi Code CLI，最简单的方法是**直接在对话中请求验证**：

### 示例 1：快速验证

```
请帮我验证 http://localhost:1420 的页面是否正常加载，显示标题是否包含 "OPC-HARNESS"
```

### 示例 2：完整冒烟测试

```
请帮我执行以下验证任务：

目标：http://localhost:1420

1. 访问首页，检查标题是否为 "OPC-HARNESS"
2. 查看导航菜单是否包含 Dashboard、Idea、Coding、Marketing
3. 点击 Idea 导航，检查是否进入 Idea 输入页面
4. 返回首页，检查 Coding 工作区是否可访问
5. 检查页面是否有 JavaScript 错误

请报告每个步骤的结果。
```

### 示例 3：特定功能验证

```
请验证 http://localhost:1420/idea 页面：
1. 页面是否正常加载
2. 是否有文本输入区域
3. 是否有提交或生成按钮
4. 输入框是否可以输入文字

不要真正提交表单，只验证界面功能。
```

## 使用自动化脚本

如果你希望更结构化的验证：

```bash
# 运行 CLI 浏览器验证
npm run harness:verify:cli

# 指定测试套件
npm run harness:verify:cli -- -Suite critical

# 强制指定 CLI 类型
npm run harness:verify:cli -- -ForceCLI kimi
```

## 在 Kimi CLI 中的完整工作流程

```bash
# 1. 启动开发环境
npm run tauri:dev

# 2. 等待服务器启动（在另一个终端）

# 3. 在 Kimi CLI 中直接请求验证
```

然后在 Kimi CLI 中输入：

```
@browser http://localhost:1420

然后请帮我验证：
1. 页面标题是否包含 "OPC-HARNESS"
2. 导航菜单是否可见
3. 点击 "Idea" 菜单是否能进入 Idea 输入页面
```

## 验证任务模板

### 模板文件位置

验证任务定义在：
- `scripts/cli-browser-verify/tasks/smoke.yaml` - 冒烟测试
- `scripts/cli-browser-verify/tasks/critical.yaml` - 关键路径测试

### 自定义验证任务

创建新的 YAML 文件：

```yaml
# scripts/cli-browser-verify/tasks/my-test.yaml
name: "我的测试"
description: "自定义验证任务"
url: "http://localhost:1420"

steps:
  - id: 1
    action: "navigate"
    description: "访问页面"
    expect: "页面加载成功"

  - id: 2
    action: "check_element"
    description: "检查特定元素"
    selector: ".my-element"
    expect: "元素可见"
```

然后运行：
```bash
npm run harness:verify:cli -- -Suite my-test
```

## 对比方案

| 方案 | 优点 | 缺点 | 适用场景 |
|-----|------|------|---------|
| **直接对话** | 最简单，无需配置 | 需要手动输入 | 快速验证 |
| **自动化脚本** | 结构化，可重复 | 需要理解 YAML | 定期验证 |
| **Browser Use** | 完全自动化 | 需要 API Key | CI/CD |
| **Playwright** | 精确控制 | 需要安装浏览器 | 回归测试 |

## 推荐工作流

### 开发阶段（推荐）

```
1. 启动开发环境: npm run tauri:dev
2. 在 Kimi CLI 中直接请求验证
3. 根据结果修复问题
```

### 提交前检查

```bash
# 1. 快速代码检查
npm run harness:quick

# 2. Tauri 应用验证
npm run harness:verify:tauri

# 3. CLI 浏览器验证（人工确认）
npm run harness:verify:cli
```

## 故障排除

### CLI 未检测到

```bash
# 强制指定 CLI
$env:HARNESS_CLI_BROWSER="kimi"
npm run harness:verify:cli
```

### 开发服务器未运行

```bash
# 检查端口
Get-NetTCPConnection -LocalPort 1420

# 启动服务器
npm run tauri:dev
```

### 验证失败

在 Kimi CLI 中直接执行：
```
@browser http://localhost:1420

然后告诉我你看到了什么？
```

## 总结

CLI Browser 方案的核心优势：**无需额外配置，直接使用你正在使用的 AI CLI 的浏览器能力**。

最简单的方式就是**在对话中直接描述验证任务**，Kimi 会自动使用浏览器工具完成验证。

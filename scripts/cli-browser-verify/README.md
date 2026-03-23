# CLI Browser 验证方案

利用现有 CLI 工具（Kimi、Claude Code 等）内置的浏览器能力进行自动化验证，**无需配置 API Key**。

## 原理

现代 AI CLI 工具都内置了浏览器功能：
- **Kimi Code CLI**: 支持 `@browser` 和网页访问
- **Claude Code**: 内置 `Browser` 工具
- **OpenCode**: 支持浏览器操作

## 使用方式

### 方式一：直接对话验证（推荐）

在 Kimi Code CLI 对话中直接请求：

```
请帮我验证 http://localhost:1420 的以下内容：
1. 页面是否正常加载，显示 OPC-HARNESS 标题
2. 导航菜单是否可见（Dashboard、Idea、Coding、Marketing）
3. 点击 Idea 菜单是否能进入 Idea 输入页面
4. 检查页面是否有明显错误
```

### 方式二：自动化脚本

```bash
# 运行 CLI 浏览器验证
npm run harness:verify:cli
```

## 支持的 CLI 工具

| CLI 工具 | 浏览器能力 | 检测方式 | 状态 |
|---------|-----------|---------|------|
| Kimi Code CLI | `@browser` | 环境变量 `KIMI_CLI_VERSION` | ✅ 支持 |
| Claude Code | `Browser` 工具 | 环境变量 `CLAUDE_CODE_VERSION` | ✅ 支持 |
| OpenCode | 内置浏览器 | 命令 `opencode --version` | ✅ 支持 |

## 配置

```bash
# 强制指定 CLI 工具
$env:HARNESS_CLI_BROWSER="kimi"  # 或 "claude", "opencode"
```

## 对比

| 特性 | Browser Use | CLI Browser |
|------|-------------|-------------|
| 需要 API Key | ✅ 需要 | ❌ 不需要 |
| 本地浏览器 | ✅ 需下载 | ❌ 使用 CLI 自带 |
| 速度 | 慢 | 快 |
| 适用场景 | CI/CD | 开发验证 |

## 验证任务示例

在 `scripts/cli-browser-verify/tasks/` 中定义任务：

```yaml
name: "冒烟测试"
url: "http://localhost:1420"
steps:
  - action: "navigate"
    description: "访问首页"
    expect: "显示 OPC-HARNESS 标题"
  
  - action: "check_element"
    selector: "nav"
    description: "检查导航菜单"
    expect: "包含 Dashboard、Idea、Coding"
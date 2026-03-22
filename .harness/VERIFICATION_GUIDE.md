# Harness 验证方案指南

本文档介绍 OPC-HARNESS 项目的验证方案。

## 快速选择

| 场景 | 推荐方案 | 命令 |
|-----|---------|------|
| 开发时快速验证 | CLI Browser | 直接对话 / `npm run harness:verify:cli` |
| 代码提交前 | 快速验证 | `npm run harness:quick` |
| Tauri 应用检查 | Tauri 验证 | `npm run harness:verify:tauri` |

## 验证方案

### 1. CLI Browser 验证 ⭐ 推荐

**适用场景**: 开发验证、快速检查

**优点**:
- ✅ 无需配置 API Key
- ✅ 无需下载浏览器
- ✅ 速度最快
- ✅ 直接使用 Kimi/Claude 的浏览器能力

**使用方式**:

1. **直接对话（推荐）**:
```
请帮我验证 http://localhost:1420：
1. 页面是否正常加载 OPC-HARNESS 标题
2. 导航菜单是否包含 Dashboard、Idea、Coding
3. 点击 Idea 是否能进入输入页面
```

2. **使用浏览器命令**:
```
@browser http://localhost:1420

然后告诉我你看到了什么？
```

3. **自动化脚本**:
```bash
npm run harness:verify:cli
```

**验证任务**:
- `smoke` - 冒烟测试（基础功能）
- `critical` - 关键路径测试（核心业务）

---

### 2. Tauri 开发验证

**适用场景**: Tauri 应用开发、进程检查

**优点**:
- ✅ 检查 Tauri 进程和 Vite 服务器
- ✅ 验证 Rust 编译
- ✅ 速度快

**使用方式**:
```bash
npm run harness:verify:tauri
```

**检查内容**:
- Tauri 进程运行状态
- Vite 服务器端口监听
- 前端文件完整性
- Rust 代码编译
- Tauri CLI 版本

---

### 3. 快速验证

**适用场景**: 代码提交前检查

**优点**:
- ✅ 检查 TypeScript 类型
- ✅ 检查 ESLint
- ✅ 检查 Rust 编译
- ✅ 速度最快

**使用方式**:
```bash
npm run harness:quick
```

**检查内容**:
- TypeScript 类型检查
- ESLint 代码规范
- Rust 编译检查
- 开发服务器状态

---

## 推荐工作流

### 开发阶段

```bash
# 1. 启动开发环境
npm run tauri:dev

# 2. 在 Kimi CLI 中直接验证
请帮我验证 http://localhost/1420 ...

# 3. 修复问题

# 4. 快速检查代码
npm run harness:quick
```

### 提交前

```bash
# 1. 代码质量检查
npm run harness:quick

# 2. Tauri 应用验证
npm run harness:verify:tauri

# 3. 在 Kimi CLI 中执行 CLI Browser 验证
```

## 文件结构

```
.harness/
├── scripts/
│   ├── harness-check.ps1              # 架构健康检查
│   ├── harness-gc.ps1                 # 垃圾回收
│   ├── harness-quick-verify.ps1       # 快速验证
│   ├── harness-verify-tauri.ps1       # Tauri 验证
│   └── example-task.ps1               # 示例任务
└── cli-browser-verify/                # CLI Browser 方案
    ├── verify_runner.ps1              # 验证运行器
    ├── cli_detector.ps1               # CLI 检测
    ├── tasks/
    │   ├── smoke.yaml                 # 冒烟测试
    │   └── critical.yaml              # 关键路径测试
    ├── quick-verify.txt               # 快速验证指令
    ├── README.md                      # 使用说明
    └── USAGE.md                       # 详细指南
```

## 验证任务模板

### 基础验证

复制到 Kimi CLI 使用：

```
验证 http://localhost:1420：
- 页面标题是否为 "OPC-HARNESS"
- 导航菜单是否包含 Dashboard、Idea、Coding、Marketing
- 是否有错误信息
```

### 功能验证

```
验证 http://localhost:1420/idea：
- 页面是否正常加载
- 是否有文本输入框
- 是否有提交按钮
```

### 完整验证

```
执行完整冒烟测试：
1. 访问 http://localhost:1420，检查标题
2. 检查导航菜单（Dashboard、Idea、Coding、Marketing）
3. 点击 Idea，检查进入输入页面
4. 点击 Coding，检查工作区
5. 检查控制台是否有错误
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

| 方案 | API Key | 浏览器 | 速度 | 推荐度 |
|-----|---------|--------|------|--------|
| **CLI Browser** ⭐ | ❌ 不需要 | CLI 自带 | ⚡ 最快 | ⭐⭐⭐⭐⭐ |
| Tauri 验证 | ❌ 不需要 | ❌ 不启动 | ⚡ 快 | ⭐⭐⭐⭐ |
| 快速验证 | ❌ 不需要 | ❌ 不启动 | ⚡ 最快 | ⭐⭐⭐⭐ |

**推荐**: 开发时用 CLI Browser，提交前用 Tauri + 快速验证。

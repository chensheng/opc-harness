# Scripts

本目录包含 OPC-HARNESS 项目的开发和工程化脚本。

## 📁 目录结构

```
scripts/
├── README.md                       # 本文件（使用说明）
├── harness-check.ps1               # 架构健康检查（主入口）
├── harness-doc-check.ps1           # 文档一致性检查
├── harness-dead-code.ps1           # 死代码检测
├── harness-e2e.ps1                 # E2E 测试运行器
├── harness-gc.ps1                  # 垃圾回收清理
├── harness-quick-verify.ps1        # 快速验证
├── harness-verify-tauri.ps1        # Tauri 环境验证
├── fix-code-quality.ps1            # 代码质量修复
├── example-task.ps1                # 任务执行示例
└── cli-browser-verify/             # CLI 浏览器验证模块
    ├── README.md                   # CLI Browser 使用文档
    ├── USAGE.md                    # 详细用法指南
    ├── cli_detector.ps1            # CLI 检测脚本
    ├── verify_runner.ps1           # 验证运行器
    ├── tasks/                      # 验证任务定义
    │   ├── smoke.yaml              # 冒烟测试
    │   └── critical.yaml           # 关键路径测试
    ├── reports/                    # 生成的报告
    └── screenshots/                # 截图输出
```

**注意**: 所有 Harness Engineering 相关的自动化脚本都集中在 `scripts/` 目录中，便于管理和维护。

## 📁 脚本列表

### 核心脚本

| 脚本 | 用途 | npm 命令 |
|------|------|----------|
| `harness-check.ps1` | 架构健康检查（TypeScript、ESLint、Prettier、Rust） | `npm run harness:check` |
| `harness-gc.ps1` | 垃圾清理（临时文件、构建产物、过期文档） | `npm run harness:gc` |
| `harness-quick-verify.ps1` | 快速验证（开发环境健康检查） | `npm run harness:quick` |
| `harness-verify-tauri.ps1` | Tauri 开发环境完整验证 | `npm run harness:verify:tauri` |
| `fix-code-quality.ps1` | 自动修复代码质量问题 | `npm run harness:fix` |
| `example-task.ps1` | Harness 任务执行器示例 | - |

### CLI Browser 验证

CLI Browser 验证脚本位于 [`cli-browser-verify/`](./cli-browser-verify/) 目录，用于利用 AI CLI工具的浏览器能力进行自动化验证。

## 🚀 使用方式

### 架构健康检查（主入口）⭐

```
# 基础检查
npm run harness:check

# 扩展检查（按需使用）
npm run harness:check -- -DocCheck    # + 文档一致性检查
npm run harness:check -- -DeadCode    # + 死代码检测
npm run harness:check -- -All         # 完整检查（推荐提交前使用）
```

**注意**: 以下命令已被整合到 `harness:check` 中，不再作为独立命令存在：
- ~~`harness:doc:check`~~ → `npm run harness:check -- -DocCheck`
- ~~`harness:dead:code`~~ → `npm run harness:check -- -DeadCode`
- ~~`harness:gc`~~、~~`harness:fix`~~、~~`harness:quick`~~、~~`harness:verify:*`~~ 已移除

### 代码质量修复

```
# 自动修复代码格式和 lint 问题
npm run harness:fix

# 仅预览将要执行的操作
npm run harness:fix -- -DryRun

# 跳过类型检查
npm run harness:fix -- -SkipTypeCheck
```

### 垃圾清理

```
# 执行清理（会确认每个文件）
npm run harness:gc

# 强制清理（无需确认）
npm run harness:gc -- -Force

# 预览模式（不实际删除）
npm run harness:gc -- -DryRun
```

### 快速验证

```
# 快速验证开发环境
npm run harness:quick
```

### Tauri 环境验证

```
# 完整验证 Tauri 开发环境
npm run harness:verify:tauri
```

## 📋 脚本功能详解

### harness-check.ps1

**检查项目：**
1. TypeScript 类型检查
2. ESLint 代码质量检查
3. Prettier 格式化检查
4. Rust 编译检查
5. 依赖完整性检查
6. 目录结构检查

**评分标准：**
- 90-100: 优秀
- 70-89: 良好
- <70: 需要修复

### harness-gc.ps1

**清理内容：**
1. 临时文件（*.tmp, *.bak, *.log 等）
2. Node.js 构建产物（dist/, build/ 等）
3. Rust 构建产物（src-tauri/target/）
4. 过期文档（>30 天）
5. 扫描代码注释标记（TODO/FIXME/HACK）

### fix-code-quality.ps1

**修复步骤：**
1. TypeScript 类型检查
2. Prettier 格式化
3. ESLint 自动修复

## 🔧 开发工作流建议

### 日常开发
```
# 1. 运行单元测试
npm run test:unit

# 2. 代码修改完成后运行架构检查
npm run harness:check

# 3. 提交前运行完整验证
npm run test:unit && npm run test:e2e && npm run harness:check -- -All
```

### 定期维护
```bash
# 每周运行一次完整检查（包含文档和死代码）
npm run harness:check -- -All

# 按需使用 npx 命令
npx vitest run --coverage   # 生成覆盖率报告
npx vitest --ui             # 打开 UI 界面
```

### CI/CD 集成
```
# 在 CI 中使用 JSON 输出
npm run harness:check -- -Json > health-report.json

# 快速验证用于预检
npm run test:unit
```

## 📝 注意事项

1. **PowerShell 执行策略**：所有脚本都需要 PowerShell 执行权限
   ```powershell
   # 如果遇到执行策略错误，运行：
   Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
   ```

2. **跨平台兼容**：当前脚本仅支持 Windows PowerShell，Linux/macOS 版本待开发

3. **依赖要求**：
   - Node.js >= 18.0.0
   - Rust >= 1.70.0
   - PowerShell >= 7.0

## 🤝 贡献指南

添加新脚本时，请遵循以下规范：

1. 使用 `.ps1` 扩展名
2. 在脚本开头添加 shebang: `#!/usr/bin/env pwsh`
3. 提供清晰的参数说明和用法注释
4. 使用统一的色彩方案（Cyan/Yellow/Green/Red）
5. 在 package.json 中添加对应的 npm 命令

## 📚 相关文档

### 核心文档

- [README.md](../README.md) - 项目主文档
- [AGENTS.md](../AGENTS.md) - AI Agent 导航地图
- [ARCHITECTURE.md](../ARCHITECTURE.md) - 架构设计文档
- [scripts/README.md](./README.md) - Harness 脚本使用说明

### 测试和 Harness 文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/testing/HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [docs/testing/E2E-STRATEGY.md](./testing/E2E-STRATEGY.md) - E2E 测试方案
- [docs/references/harness-user-guide.md](../references/harness-user-guide.md) - Harness 用户指南
- [docs/references/best-practices.md](../references/best-practices.md) - 最佳实践
- [docs/references/architecture-rules.json](../references/architecture-rules.json) - 架构规则配置

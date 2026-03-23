# Scripts

本目录包含 OPC-HARNESS 项目的开发和工程化脚本。

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

### 架构健康检查

```powershell
# 运行完整检查
npm run harness:check

# 自动修复问题
npm run harness:check -- -Fix

# 输出详细日志
npm run harness:check -- -Verbose

# JSON 格式输出（用于 CI/CD）
npm run harness:check -- -Json
```

### 代码质量修复

```powershell
# 自动修复代码格式和 lint 问题
npm run harness:fix

# 仅预览将要执行的操作
npm run harness:fix -- -DryRun

# 跳过类型检查
npm run harness:fix -- -SkipTypeCheck
```

### 垃圾清理

```powershell
# 执行清理（会确认每个文件）
npm run harness:gc

# 强制清理（无需确认）
npm run harness:gc -- -Force

# 预览模式（不实际删除）
npm run harness:gc -- -DryRun
```

### 快速验证

```powershell
# 快速验证开发环境
npm run harness:quick
```

### Tauri 环境验证

```powershell
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
```powershell
# 1. 开始开发前快速验证
npm run harness:quick

# 2. 开发完成后修复代码质量
npm run harness:fix

# 3. 提交前运行完整检查
npm run harness:check
```

### 定期维护
```powershell
# 每周清理一次构建产物
npm run harness:gc -- -Force

# 每月运行一次完整验证
npm run harness:verify:tauri
```

### CI/CD 集成
```powershell
# 在 CI 中使用 JSON 输出
npm run harness:check -- -Json > health-report.json

# 快速验证用于预检
npm run harness:quick
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

- [CLI Browser 验证](./cli-browser-verify/README.md)
- [项目架构](../ARCHITECTURE.md)
- [开发指南](../README.md)

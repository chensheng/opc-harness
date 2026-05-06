## Why

当前项目 scripts 目录下的所有脚本都使用 PowerShell (.ps1) 编写,这导致在 Windows 以外的平台(macOS、Linux)上无法直接运行。虽然可以通过 `powershell -ExecutionPolicy Bypass` 调用,但这增加了跨平台兼容性问题,并且与项目主要使用 TypeScript/JavaScript 的技术栈不一致。改用 Node.js 脚本可以提升跨平台兼容性,简化开发环境配置,并与项目技术栈保持一致。

## What Changes

- 将 `scripts/` 目录下所有 `.ps1` 文件重写为 `.js` 或 `.ts` 文件
- 更新 `package.json` 中的 npm scripts 配置,从调用 PowerShell 改为直接执行 Node.js 脚本
- 保持所有脚本的功能和输出格式不变
- 移除 PowerShell 特定的命令,使用 Node.js 标准库或跨平台 npm 包替代
- 添加必要的依赖包(如 `chalk` 用于彩色输出, `execa` 用于进程执行等)

**受影响的脚本**:
- `harness-check.ps1` → `harness-check.js` (主健康检查脚本)
- `fix-code-quality.ps1` → `fix-code-quality.js` (代码质量修复)
- `harness-gc.ps1` → `harness-gc.js` (垃圾清理)
- `harness-e2e.ps1` → `harness-e2e.js` (E2E 测试)
- `harness-rust-tests.ps1` → `harness-rust-tests.js` (Rust 测试)
- `harness-ts-tests.ps1` → `harness-ts-tests.js` (TypeScript 测试)
- `fast-check.ps1` → `fast-check.js` (快速检查)
- `test-decentralized.ps1` → `test-decentralized.js` (去中心化测试)
- `test-rust-simple.ps1` → `test-rust-simple.js` (Rust 简单测试)

## Capabilities

### New Capabilities
<!-- No new capabilities being introduced -->

### Modified Capabilities
<!-- No existing specs are being modified - this is an implementation detail change -->

## Impact

- **Affected files**: 
  - `scripts/*.ps1` → `scripts/*.js` (9 个脚本文件重写)
  - `package.json` (npm scripts 配置更新)
  - `package-lock.json` (新增依赖包)
- **Impact type**: 基础设施改进,无功能性破坏
- **Breaking changes**: 无(对外接口保持不变)
- **Dependencies**: 需要添加 cross-platform npm 包
- **Benefits**: 
  - ✅ 跨平台兼容性(Win/macOS/Linux)
  - ✅ 与技术栈一致(TypeScript/JavaScript)
  - ✅ 简化开发环境配置
  - ✅ 更好的错误处理和调试体验

## Context

当前项目有 9 个 PowerShell 脚本位于 `scripts/` 目录,用于执行各种开发任务:
- harness-check.ps1 (17.4KB) - 主健康检查脚本,最复杂
- harness-gc.ps1 (9.3KB) - 垃圾清理
- fix-code-quality.ps1 (3.4KB) - 代码质量修复
- harness-ts-tests.ps1 (3.3KB) - TypeScript 测试
- test-decentralized.ps1 (3.2KB) - 去中心化测试
- harness-e2e.ps1 (2.6KB) - E2E 测试
- harness-rust-tests.ps1 (1.0KB) - Rust 测试
- test-rust-simple.ps1 (0.8KB) - Rust 简单测试
- fast-check.ps1 (0.6KB) - 快速检查

这些脚本使用 PowerShell 特有的语法和命令(如 `Write-Host`, `Test-Path`, `Get-Content` 等),在 macOS/Linux 上无法直接运行。

**技术约束**:
- 必须保持所有脚本的输出格式不变(颜色、布局、消息文本)
- 必须保持 Health Score 计算逻辑一致
- 必须支持相同的命令行参数
- Node.js 版本: ≥18 (项目要求)

## Goals / Non-Goals

**Goals:**
- 将所有 .ps1 脚本重写为 .js 脚本
- 实现跨平台兼容(Windows/macOS/Linux)
- 保持功能完全一致(输出、退出码、行为)
- 使用现代 Node.js 特性(async/await, ES modules)
- 添加适当的错误处理和日志记录

**Non-Goals:**
- 不改变脚本的功能或业务逻辑
- 不重构脚本的架构(保持原有结构)
- 不添加新功能
- 不修改 npm scripts 的命令名称(保持向后兼容)

## Decisions

### 决策 1: 使用 JavaScript (.js) 而非 TypeScript (.ts)

**选择**: 使用 `.js` 文件而非 `.ts`

**理由**:
- Scripts 是工具脚本,不需要类型检查的严格性
- 减少编译步骤,直接运行更快
- 避免 ts-node 依赖和配置复杂性
- 与现有 Vite 构建流程解耦
- 更简单的调试体验

**替代方案**:
- 使用 .ts + ts-node → 拒绝:增加复杂性和启动时间
- 使用 .ts 预编译 → 拒绝:需要额外的构建步骤

### 决策 2: 使用 ES Modules (import/export)

**选择**: 使用 ES Modules 而非 CommonJS

**理由**:
- 与现代 JavaScript 标准一致
- 更好的 tree-shaking 支持
- package.json 中已设置 `"type": "module"`
- 更清晰的依赖管理

**实施**:
```javascript
// 使用 import
import { execa } from 'execa';
import chalk from 'chalk';

// 而非 require
// const execa = require('execa');
```

### 决策 3: 核心依赖包选择

**选择的包**:
1. **chalk** - 彩色终端输出(替代 PowerShell 的 `-ForegroundColor`)
2. **execa** - 进程执行(替代 PowerShell 的 `& command`)
3. **fs/promises** - Node.js 内置,异步文件系统操作
4. **path** - Node.js 内置,路径处理
5. **process** - Node.js 内置,环境变量和退出码

**理由**:
- chalk: 业界标准,API 简洁,性能优秀
- execa: 比 child_process 更友好,自动处理 stdout/stderr
- 内置模块: 无额外依赖,稳定性高

**替代方案**:
- colors.js → 拒绝:有安全风险历史
- shelljs → 拒绝:抽象过度,不如 execa 直接

### 决策 4: 迁移策略 - 逐个替换并测试

**选择**: 一次迁移一个脚本,立即测试

**顺序**(从简单到复杂):
1. fast-check.ps1 (最简单,验证基础框架)
2. test-rust-simple.ps1 (简单 Rust 测试)
3. harness-rust-tests.ps1 (Rust 测试包装器)
4. harness-ts-tests.ps1 (TypeScript 测试)
5. test-decentralized.ps1 (去中心化测试)
6. harness-e2e.ps1 (E2E 测试)
7. fix-code-quality.ps1 (代码修复)
8. harness-gc.ps1 (垃圾清理)
9. harness-check.ps1 (最后,最复杂,依赖其他脚本)

**理由**:
- 降低风险,每个脚本独立验证
- 早期发现共性问题
- 可以并行测试多个脚本
- harness-check.ps1 最后,因为它调用其他脚本

### 决策 5: 保持输出格式 100% 一致

**选择**: 精确复制 PowerShell 脚本的输出格式

**实施要点**:
- 使用 chalk 匹配颜色(Green=success, Red=error, Yellow=warn, Cyan=header)
- 保持相同的缩进和分隔线
- 保持相同的消息文本
- 保持相同的退出码(0=success, 1=failure)

**示例映射**:
```javascript
// PowerShell: Write-Host "  [PASS] Message" -ForegroundColor Green
// Node.js: console.log(chalk.green('  [PASS] Message'));

// PowerShell: Write-Host "========================================" -ForegroundColor Cyan
// Node.js: console.log(chalk.cyan('='.repeat(40)));
```

## Risks / Trade-offs

**[Risk] 输出格式细微差异** → Mitigation:
- 逐行对比测试输出
- 使用 git diff 验证关键输出
- 保留 PowerShell 脚本作为参考直到完全验证

**[Risk] 跨平台路径问题** → Mitigation:
- 使用 path.join() 而非字符串拼接
- 使用 path.sep 处理路径分隔符
- 测试 Windows 和 Unix 路径格式

**[Risk] 进程执行行为差异** → Mitigation:
- execa 自动处理跨平台兼容性
- 捕获 stdout/stderr 分别处理
- 正确传递退出码

**[Risk] 性能差异** → Mitigation:
- Node.js 启动速度通常快于 PowerShell
- 异步执行可能更快
- 如有性能问题,可优化并行执行

**[Trade-off] 学习曲线** → 接受:
- 团队需熟悉 Node.js 脚本编写
- 但长期收益大于短期成本
- 提供示例代码和文档

**[Trade-off] 依赖管理** → 接受:
- 新增 2-3 个 npm 依赖
- 但都是成熟稳定的包
- 定期更新维护

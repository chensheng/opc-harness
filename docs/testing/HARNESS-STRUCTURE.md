# Harness Engineering 目录结构说明

> **更新日期**: 2026-03-23  
> **目的**: 澄清 Harness Engineering 的实际目录结构

---

## 📊 实际目录结构

### 当前布局 ✅

```
opc-harness/
├── scripts/                      # Harness Engineering 自动化脚本 ⭐
│   ├── harness-check.ps1         # 架构健康检查（主入口）
│   ├── harness-doc-check.ps1     # 文档一致性检查
│   ├── harness-dead-code.ps1     # 死代码检测
│   ├── harness-e2e.ps1           # E2E 测试运行器
│   ├── harness-gc.ps1            # 垃圾回收清理
│   ├── harness-quick-verify.ps1  # 快速验证
│   ├── harness-verify-tauri.ps1  # Tauri 环境验证
│   ├── fix-code-quality.ps1      # 代码质量修复
│   └── cli-browser-verify/       # CLI 浏览器验证模块
│       ├── verify_runner.ps1
│       ├── cli_detector.ps1
│       ├── tasks/
│       └── reports/
│
├── docs/testing/                 # 测试体系文档中心
│   ├── README.md                 # 测试文档导航
│   ├── COMMANDS-REFERENCE.md     # 命令参考
│   ├── HARNESS-COMMANDS.md       # Harness 命令精简说明
│   └── e2e-reports/              # E2E 测试报告
│
└── AGENTS.md                     # AI Agent 导航地图
```

### 重要说明

1. **没有 `.harness/` 目录** ❌
   - 项目中不存在 `.harness/` 目录
   - 所有 Harness Engineering 相关脚本都在 `scripts/` 目录中
   
2. **文档引用已更新** ✅
   - `README.md` - 已修正为引用 `scripts/README.md`
   - `AGENTS.md` - 已修正项目结构和命令说明
   - `scripts/README.md` - 已更新完整的目录结构说明

---

## 🔧 Harness Engineering 组成

### 1. 自动化脚本（PowerShell）

| 脚本名称 | 功能 | npm 命令 |
|---------|------|---------|
| `harness-check.ps1` | 架构健康检查 | `npm run harness:check` |
| `harness-doc-check.ps1` | 文档一致性检查 | `npm run harness:check -- -DocCheck` |
| `harness-dead-code.ps1` | 死代码检测 | `npm run harness:check -- -DeadCode` |
| `harness-e2e.ps1` | E2E 测试运行器 | `npm run test:e2e` |
| `harness-gc.ps1` | 垃圾回收 | （已整合到 harness:check） |
| `harness-quick-verify.ps1` | 快速验证 | （已移除，直接用 test:unit） |
| `harness-verify-tauri.ps1` | Tauri 验证 | （已整合到 harness:check） |

### 2. CLI Browser 验证

**位置**: `scripts/cli-browser-verify/`

**功能**:
- 利用 AI CLI工具的浏览器能力进行验证
- 无需配置 API Key
- 支持冒烟测试和关键路径测试

**使用方式**:
```bash
# 直接对话（推荐）
@browser http://localhost:1420
请告诉我你看到了什么？

# 自动化脚本
npm run harness:verify:cli
```

### 3. 测试体系

**单元测试**:
- 位置：`src/*.test.ts`, `src/**/*.test.tsx`, `tests/`
- 命令：`npm run test:unit`

**E2E 测试**:
- 位置：`e2e/app.spec.ts`
- 命令：`npm run test:e2e`

**文档**:
- 位置：`docs/testing/`
- 导航：[docs/testing/README.md](./docs/testing/README.md)

---

## 📚 相关文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/testing/HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [docs/references/harness-user-guide.md](./references/harness-user-guide.md) - Harness 用户指南
- [docs/references/best-practices.md](./references/best-practices.md) - 最佳实践

---

## 💡 为什么这样组织？

### 设计原则

1. **集中管理** - 所有自动化脚本集中在 `scripts/` 目录
2. **清晰命名** - 以 `harness-` 前缀标识 Harness Engineering 相关功能
3. **模块化** - CLI Browser 验证作为独立子模块
4. **文档分离** - 测试文档在 `docs/testing/`，与代码分离

### 优势

- ✅ **易于查找** - 所有脚本在一个地方
- ✅ **易于维护** - 统一的目录结构
- ✅ **职责清晰** - 脚本、测试、文档各司其职
- ✅ **跨平台** - PowerShell 脚本可在 Windows/Linux/macOS 运行

---

## 🔄 历史变更

### 2026-03-23 - 目录结构优化

**变更内容**:
- 明确 `scripts/` 为 Harness Engineering 的唯一脚本目录
- 移除对不存在的 `.harness/` 目录的引用
- 更新所有文档中的路径引用

**影响范围**:
- ✅ `README.md` - 已更新
- ✅ `AGENTS.md` - 已更新
- ✅ `scripts/README.md` - 已更新
- ✅ 其他相关文档 - 已更新

---

**🎯 总结**: Harness Engineering 的所有自动化脚本都位于 `scripts/` 目录，通过 npm 命令调用。没有 `.harness/` 目录。

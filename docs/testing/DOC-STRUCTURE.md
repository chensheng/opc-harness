# Harness Engineering 文档结构说明

> **更新日期**: 2026-03-23  
> **目的**: 明确 Harness Engineering 相关文档的组织结构和引用关系

---

## 📊 文档组织结构

### 1. 自动化脚本（`scripts/`）

**位置**: `scripts/`

**内容**:
- `harness-check.ps1` - 架构健康检查（主入口）
- `harness-doc-check.ps1` - 文档一致性检查
- `harness-dead-code.ps1` - 死代码检测
- `harness-e2e.ps1` - E2E 测试运行器
- `harness-gc.ps1` - 垃圾回收清理
- `cli-browser-verify/` - CLI 浏览器验证模块

**使用说明**: [scripts/README.md](./scripts/README.md)

---

### 2. 测试体系文档（`docs/testing/`）

**位置**: `docs/testing/`

**核心文档**:
- [README.md](./testing/README.md) - 测试体系导航 ⭐
- [COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [HARNESS-COMMANDS-UPDATE.md](./testing/HARNESS-COMMANDS-UPDATE.md) - 命令更新说明
- [HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [E2E-STRATEGY.md](./testing/E2E-STRATEGY.md) - E2E 测试方案
- [testing-full.md](./testing/testing-full.md) - 完整测试指南
- [testing-validation.md](./testing/testing-validation.md) - 安装验证清单
- [RUN-E2E-AUTO.md](./testing/RUN-E2E-AUTO.md) - E2E 自动运行指南

**测试报告**: `docs/testing/e2e-reports/`（动态生成，已加入 .gitignore）

---

### 3. 参考资料库（`docs/references/`）

**位置**: `docs/references/`

**核心资料**:
- [architecture-rules.json](./references/architecture-rules.json) - 架构规则配置
- [best-practices.md](./references/best-practices.md) - 最佳实践指南
- [harness-user-guide.md](./references/harness-user-guide.md) - Harness 用户指南
- [harness-quickstart.md](./references/harness-quickstart.md) - 快速入门
- [index.md](./references/index.md) - 文档索引

---

### 4. 项目级文档（根目录）

**位置**: 项目根目录

**核心文档**:
- [README.md](./README.md) - 项目主文档
- [AGENTS.md](./AGENTS.md) - AI Agent 导航地图
- [ARCHITECTURE.md](./ARCHITECTURE.md) - 架构设计文档

---

## 🔗 文档引用关系

```
README.md (项目主文档)
├── 引用 scripts/README.md (脚本说明)
├── 引用 docs/testing/README.md (测试导航)
└── 引用 docs/references/* (参考资料)

AGENTS.md (AI Agent 导航)
├── 引用 scripts/README.md (脚本说明)
├── 引用 docs/references/architecture-rules.json (架构规则)
└── 引用 docs/references/best-practices.md (最佳实践)

scripts/README.md (脚本说明)
├── 引用 docs/testing/README.md (测试导航)
└── 引用 docs/references/* (参考资料)

docs/testing/README.md (测试导航)
└── 引用 docs/references/* (参考资料)
```

---

## 📋 重要说明

### ❌ 不存在的目录

**没有 `.harness/` 目录** - 这是一个常见的误解。所有 Harness Engineering 相关功能都通过以下两个目录实现：
- `scripts/` - 自动化脚本
- `docs/` - 文档和知识库

### ✅ 正确的引用方式

**引用脚本**:
```markdown
[scripts/README.md](./scripts/README.md)
```

**引用测试文档**:
```markdown
[docs/testing/README.md](./docs/testing/README.md)
[docs/testing/COMMANDS-REFERENCE.md](./docs/testing/COMMANDS-REFERENCE.md)
```

**引用参考资料**:
```markdown
[docs/references/architecture-rules.json](./docs/references/architecture-rules.json)
[docs/references/best-practices.md](./docs/references/best-practices.md)
```

---

## 🎯 常用文档导航

### 对于 AI Agent
1. 首先阅读：[AGENTS.md](./AGENTS.md)
2. 查看约束：[docs/references/architecture-rules.json](./docs/references/architecture-rules.json)
3. 学习最佳实践：[docs/references/best-practices.md](./docs/references/best-practices.md)

### 对于开发者
1. 项目概览：[README.md](./README.md)
2. 测试指南：[docs/testing/README.md](./docs/testing/README.md)
3. 脚本说明：[scripts/README.md](./scripts/README.md)

### 对于测试工作
1. 测试导航：[docs/testing/README.md](./docs/testing/README.md)
2. 命令参考：[docs/testing/COMMANDS-REFERENCE.md](./docs/testing/COMMANDS-REFERENCE.md)
3. E2E 策略：[docs/testing/E2E-STRATEGY.md](./docs/testing/E2E-STRATEGY.md)

---

**🎯 总结**: Harness Engineering 的所有文档都组织在 `docs/` 目录中，脚本在 `scripts/` 目录中，便于查找和维护。

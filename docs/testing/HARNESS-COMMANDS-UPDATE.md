# Harness Engineering 命令更新说明

> **更新时间**: 2026-03-23  
> **目的**: 整合精简测试和 Harness 相关命令，提高开发效率

---

## 📊 命令变更总览

### 测试命令

#### 变更前
```bash
test:run          # 一次性运行所有测试
test:e2e:auto     # E2E 智能运行（已移除）
```

#### 变更后 ⭐
```bash
npm run test:unit           # 单元测试（与 test:e2e 对应）⭐
npm run test:e2e            # E2E 测试（智能运行，自动管理服务器）⭐
```

**按需使用**（不常用）:
```bash
npx vitest run --coverage   # 生成覆盖率报告
npx vitest --ui             # UI 界面
npx vitest                  # 监视模式（开发时用）
```

---

### Harness 命令

#### 变更前（8 个独立命令）
```bash
harness:check           # 架构健康检查
harness:gc              # 垃圾回收
harness:fix             # 自动修复
harness:quick           # 快速验证
harness:doc:check       # 文档一致性检查
harness:dead:code       # 死代码检测
harness:verify:tauri    # Tauri 验证
harness:verify:cli      # CLI 验证
```

#### 变更后（1 个主入口 + 参数扩展）⭐
```bash
npm run harness:check                    # 基础检查（6 项）
npm run harness:check -- -DocCheck       # + 文档一致性检查
npm run harness:check -- -DeadCode       # + 死代码检测
npm run harness:check -- -All            # 完整检查（推荐）⭐
```

**已移除的命令**:
- ❌ `harness:gc` - 功能整合到 `harness:check`
- ❌ `harness:fix` - 使用 `npm run lint:fix && npm run format` 替代
- ❌ `harness:quick` - 直接运行 `npm run test:unit` 更快
- ❌ `harness:doc:check` - 整合为 `-DocCheck` 参数
- ❌ `harness:dead:code` - 整合为 `-DeadCode` 参数
- ❌ `harness:verify:*` - 整合到 `harness:check`

---

## 🎯 新的工作流程

### 日常开发
```bash
# 1. 运行单元测试
npm run test:unit

# 2. 代码修改后运行架构检查
npm run harness:check

# 3. （可选）开发时需要监视模式
npx vitest
```

### 提交前验证 ⭐
```bash
# 方式一：分步执行
npm run test:unit
npm run test:e2e
npm run harness:check

# 方式二：一站式完整检查
npm run harness:check -- -All
npm run test:unit
npm run test:e2e
```

### 定期维护
```bash
# 每周运行一次完整检查
npm run harness:check -- -All

# 按需生成覆盖率报告
npx vitest run --coverage
```

---

## 📋 更新的文档

以下文档已同步更新：

1. ✅ [`AGENTS.md`](./AGENTS.md) - AI Agent 导航地图
2. ✅ [`README.md`](./README.md) - 项目主文档
3. ✅ [`scripts/README.md`](./scripts/README.md) - 脚本使用说明
4. ✅ [`src/AGENTS.md`](./src/AGENTS.md) - 前端开发指南
5. ✅ [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 开发指南
6. ✅ [`docs/testing/README.md`](./docs/testing/README.md) - 测试文档中心
7. ✅ [`docs/testing/COMMANDS-REFERENCE.md`](./docs/testing/COMMANDS-REFERENCE.md) - 命令参考

---

## 📚 相关文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/testing/HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [docs/references/harness-user-guide.md](./references/harness-user-guide.md) - Harness 用户指南
- [docs/references/best-practices.md](./references/best-practices.md) - 最佳实践

---

## 💡 设计原则

1. **单一入口** - Harness 所有功能通过 `harness:check` 统一访问
2. **参数控制** - 通过命令行参数选择检查类型
3. **对称命名** - 测试命令采用 `test:<type>` 格式（`test:unit`、`test:e2e`）
4. **灵活组合** - 可以单独运行某项检查，也可以组合运行
5. **渐进式检查** - 基础检查快速反馈，扩展检查按需运行

---

## 🔍 迁移指南

### 如果你之前使用过时的命令...

| 旧命令 | 新命令 | 说明 |
|--------|--------|------|
| `npm run test:run` | `npm run test:unit` | 更清晰的语义 |
| `npm run harness:doc:check` | `npm run harness:check -- -DocCheck` | 参数化 |
| `npm run harness:dead:code` | `npm run harness:check -- -DeadCode` | 参数化 |
| `npm run harness:quick` | `npm run test:unit` | 直接运行测试 |
| `npm run harness:fix` | `npm run lint:fix && npm run format` | 手动组合 |
| `npm run harness:gc` | `npm run harness:check` | 整合到健康检查 |

---

## ✅ 验证结果

所有命令已测试通过：

```bash
# 单元测试
npm run test:unit
# ✓ 4 test files passed (24 tests)

# E2E 测试
npm run test:e2e
# ✓ 6 e2e tests passed

# 架构健康检查
npm run harness:check
# ✓ Health Score: 85/100

# 完整检查
npm run harness:check -- -All
# ✓ 包含文档检查和死代码检测
```

---

## 🚀 下一步行动

1. **更新 CI/CD 配置** - 将旧的命令替换为新的命令格式
2. **更新团队文档** - 确保团队成员了解新的命令格式
3. **监控反馈** - 收集团队对新命令的反馈，持续优化

---

**🎉 完成！** Harness Engineering 命令更加简洁高效了！✨

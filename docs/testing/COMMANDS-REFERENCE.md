# 📋 测试命令参考

> **最后更新**: 2026-03-23  
> **原则**: 保留核心常用命令，移除冗余命令

---

## ✅ 精简后的命令列表

### 单元测试 (Unit Tests)

| 命令 | 说明 | 使用场景 |
|------|------|----------|
| `npm run test:unit` | 运行单元测试 | 🚀 CI/CD 或最终验证 ⭐ |

**按需使用（不常用）**:
```bash
npx vitest run --coverage   # 生成覆盖率报告 📊
npx vitest --ui             # UI 界面（浏览器）🔍
npx vitest                  # 监视模式（开发时用，可选）💻
```

### E2E 测试 (End-to-End Tests)

| 命令 | 说明 | 使用场景 |
|------|------|----------|
| `npm run test:e2e` | 运行 E2E 测试 | ⚡ 服务器已启动时 |
| `npm run test:e2e:auto` | 智能运行 E2E（自动检测并启动服务器）⭐ | 🎯 推荐！一键运行 |

### Harness Engineering

| 命令 | 说明 | 使用场景 |
|------|------|----------|
| `npm run harness:check` | 架构健康检查（主入口）⭐ | ✅ 提交前验证 |
| `npm run harness:check -- -DocCheck` | + 文档一致性检查 | 📖 文档同步验证 |
| `npm run harness:check -- -DeadCode` | + 死代码检测 | 🗑️ 清理未使用代码 |
| `npm run harness:check -- -All` | 运行所有检查（推荐）⭐ | 🎯 完整验证 |

---

## ❌ 已移除的冗余命令

以下命令已被移除，可用其他命令替代：

### 移除的命令及替代方案

| 已移除命令 | 替代方案 | 说明 |
|-----------|----------|------|
| `test:components` | `npm run test src/components` | 直接指定路径更灵活 |
| `test:stores` | `npm run test src/stores` | 直接指定路径更灵活 |
| `test:e2e:watch` | `npm run test e2e` | Vitest 原生支持 |
| `test:e2e:report` | `ls docs/testing/e2e-reports` | 简单的目录查看 |
| `test:coverage` | `npx vitest run --coverage` | 按需使用 npx 命令 |
| `test:ui` | `npx vitest --ui` | 按需使用 npx 命令 |
| `harness:gc` | `npm run harness:check` | 整合到健康检查 |
| `harness:fix` | `npm run lint:fix && npm run format` | 手动组合更清晰 |
| `harness:quick` | `npm run test:run` | 直接运行测试更快 |
| `harness:gc:dry` | `npm run harness:check` + 查看输出 | 简化操作 |
| `harness:fix:dry` | Git diff after fix | Git 更好追踪变更 |
| `harness:verify:tauri` | `npm run harness:check` | 整合到健康检查 |
| `harness:verify:cli` | `npm run harness:check` | 整合到健康检查 |
| `harness:dead:code:dry` | `npm run harness:check -- -DeadCode` | 整合到主入口 |
| `harness:test` | `npm run test:run && npm run test:e2e` | 手动组合更清晰 |
| `harness:test:full` | `npm run harness:check -- -All && npm run test:coverage && npm run test:e2e` | 手动组合更灵活 |

---

## 🎯 常用工作流

### 日常开发
```bash
# 1. 启动开发服务器
npm run dev

# 2. 在另一个终端运行监视模式测试
npm run test

# 3. 编写代码，实时查看测试反馈
# ... coding ...
```

### 提交前验证
```bash
# 1. 运行所有单元测试
npm run test:run

# 2. 运行 E2E 测试
npm run test:e2e:auto

# 3. 架构健康检查
npm run harness:check

# 4. 格式化代码
npm run format
```

### 完整测试套件
```bash
# 一站式完整验证
npm run harness:check && \
npm run test:coverage && \
npm run test:e2e:auto
```

### 清理和维护
```bash
# 1. 检测死代码
npm run harness:dead:code

# 2. 文档一致性检查
npm run harness:doc:check

# 3. 垃圾回收
npm run harness:gc

# 4. 自动修复
npm run harness:fix
```

---

## 📊 命令分类

### 按频率分类

#### 每天使用 (Daily)
- `npm run dev` - 开发服务器
- `npm run test` - 测试监视
- `npm run test:e2e:auto` - E2E 测试

#### 每周使用 (Weekly)
- `npm run test:run` - 完整测试
- `npm run lint` - 代码检查
- `npm run format` - 格式化
- `npm run harness:check` - 健康检查

#### 按需使用 (As Needed)
- `npm run test:coverage` - 覆盖率检查
- `npm run harness:doc:check` - 文档检查
- `npm run harness:dead:code` - 死代码检测
- `npm run harness:gc` - 垃圾回收
- `npm run harness:fix` - 自动修复

---

## 💡 最佳实践

### 1. 开发时使用监视模式
```bash
npm run test  # 文件变更自动重跑测试
```

### 2. 提交前必须运行
```bash
npm run test:run && npm run harness:check
```

### 3. E2E 测试使用智能脚本
```bash
npm run test:e2e:auto  # 自动管理服务器
```

### 4. 定期清理技术债务
```bash
npm run harness:dead:code
npm run harness:doc:check
npm run harness:gc
```

---

## 🔍 故障排查

### 问题：测试运行缓慢

**解决方案**:
```bash
# 1. 只运行特定测试
npm run test -- src/components/Button.test.tsx

# 2. 使用 run 模式而非 watch
npm run test:run

# 3. 检查死代码
npm run harness:dead:code
```

### 问题：E2E 测试失败

**解决方案**:
```bash
# 1. 使用智能脚本（自动管理服务器）
npm run test:e2e:auto

# 2. 手动启动服务器后运行
npm run dev
# 另一个终端
npm run test:e2e
```

---

## 📚 相关文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/references/harness-user-guide.md](./references/harness-user-guide.md) - Harness 用户指南

---

**🎯 目标**: 保持命令简洁明了，减少认知负担，提高开发效率！

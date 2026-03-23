# Harness Engineering 命令精简说明

> **最后更新**: 2026-03-23  
> **原则**: 整合冗余命令，保留核心入口，通过参数控制功能

---

## ✅ 精简后的命令

### 主入口（唯一推荐）⭐

```bash
npm run harness:check       # 基础架构健康检查
```

**包含的检查项**：
1. ✅ TypeScript Type Checking
2. ✅ ESLint Code Quality Check
3. ✅ Prettier Formatting Check
4. ✅ Rust Compilation Check
5. ✅ Dependency Integrity Check
6. ✅ Directory Structure Check

---

### 扩展检查（可选）

#### 1. 文档一致性检查
```bash
npm run harness:check -- -DocCheck
```
- 验证 AGENTS.md 中的链接是否有效
- 检查 README 文件完整性
- 验证 TODO/FIXME 状态

#### 2. 死代码检测
```bash
npm run harness:check -- -DeadCode
```
- 扫描未使用的 imports
- 检测未使用的变量和函数
- 识别陈旧组件

#### 3. 完整检查（推荐提交前使用）
```bash
npm run harness:check -- -All
```
- 基础检查 + 文档检查 + 死代码检测
- 一站式完整验证

---

## ❌ 已移除的命令

以下命令已被整合或移除：

| 已移除命令 | 替代方案 | 理由 |
|-----------|----------|------|
| `harness:gc` | `npm run harness:check` | 功能整合到主入口 |
| `harness:fix` | `npm run lint:fix && npm run format` | 手动组合更清晰 |
| `harness:quick` | `npm run test:run` | 直接运行测试更快 |
| `harness:doc:check` | `npm run harness:check -- -DocCheck` | 整合到主入口 |
| `harness:dead:code` | `npm run harness:check -- -DeadCode` | 整合到主入口 |
| `harness:verify:tauri` | `npm run harness:check` | 整合到健康检查 |
| `harness:verify:cli` | `npm run harness:check` | 整合到健康检查 |
| `harness:gc:dry` | `npm run harness:check` + 查看输出 | 简化操作 |
| `harness:fix:dry` | Git diff after fix | Git 更好追踪变更 |
| `harness:dead:code:dry` | `npm run harness:check -- -DeadCode` | 整合到主入口 |

---

## 🎯 常用工作流

### 日常开发
```bash
# 快速验证（仅运行测试）
npm run test

# 提交前基础检查
npm run harness:check
```

### 提交前完整验证 ⭐
```bash
# 推荐：完整检查
npm run harness:check -- -All

# 或者分步执行
npm run test:run
npm run harness:check -- -DocCheck
npm run lint:fix
npm run format
```

### 定期维护
```bash
# 每周一次完整检查
npm run harness:check -- -All

# 查看死代码
npm run harness:check -- -DeadCode

# 文档一致性验证
npm run harness:check -- -DocCheck
```

---

## 📊 精简效果

### 精简前（8 个命令）
```bash
harness:check           # 主入口
harness:gc              # 垃圾回收
harness:fix             # 自动修复
harness:quick           # 快速验证
harness:doc:check       # 文档检查
harness:dead:code       # 死代码检测
harness:verify:tauri    # Tauri 验证
harness:verify:cli      # CLI 验证
```

### 精简后（1 个主入口 + 3 个参数）
```bash
harness:check                    # 主入口 ⭐
harness:check -- -DocCheck       # + 文档检查
harness:check -- -DeadCode       # + 死代码检测
harness:check -- -All            # 完整检查
```

**精简率**: 从 8 个命令减少到 1 个主入口（减少 87.5%）✨

---

## 💡 设计原则

1. **单一入口**: 所有 Harness 功能通过 `harness:check` 统一访问
2. **参数控制**: 通过命令行参数选择检查类型
3. **灵活组合**: 可以单独运行某项检查，也可以组合运行
4. **渐进式**: 基础检查快速反馈，扩展检查按需运行

---

## 🔍 故障排查

### 问题：某些检查显示 "unavailable"

**解决方案**:
```bash
# 1. 确保依赖已安装
npm install
cd src-tauri && cargo fetch

# 2. 检查工具链
node --version
npx tsc --version
cargo --version

# 3. 查看详细错误信息
npm run harness:check -- -Verbose
```

### 问题：ESLint 或 Prettier 失败

**解决方案**:
```bash
# 自动修复
npm run lint:fix
npm run format

# 重新检查
npm run harness:check
```

---

## 📚 相关文档

- [docs/testing/README.md](./testing/README.md) - 测试体系导航 ⭐
- [docs/testing/COMMANDS-REFERENCE.md](./testing/COMMANDS-REFERENCE.md) - 完整命令参考
- [docs/testing/HARNESS-COMMANDS.md](./testing/HARNESS-COMMANDS.md) - Harness 命令精简说明
- [docs/testing/HARNESS-STRUCTURE.md](./testing/HARNESS-STRUCTURE.md) - 目录结构说明
- [docs/references/harness-user-guide.md](./references/harness-user-guide.md) - Harness 用户指南
- [docs/references/best-practices.md](./references/best-practices.md) - 最佳实践

---

**🎯 目标**: 简化命令接口，提高开发效率，保持灵活性！✨

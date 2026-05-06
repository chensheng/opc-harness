# Harness Quality Schema - 快速参考

## 🚀 快速开始

### 1. 创建变更（自动使用 harness-quality schema）
```bash
openspec new change my-feature
```

### 2. 生成 artifacts
```bash
# AI 会自动按顺序创建
/opsx:propose my-feature
# 或手动创建每个 artifact
```

### 3. 实施任务
```bash
/opsx:apply my-feature
```

### 4. 质量检查 ⭐ 新增
```bash
npm run harness:check
# 将结果记录到 quality-check.md
```

### 5. 运行时验证 ⭐ 新增
```bash
npm run tauri:dev
# 验证应用启动、功能正常、无错误日志
# 将结果记录到 runtime-check.md
```

### 5. 归档
```bash
/opsx:archive my-feature
```

## 📊 Artifact 依赖关系

```
proposal (无依赖)
    ↓
specs ──┐
        ├──→ tasks ──→ quality-check ──→ runtime-check ──→ apply
design ─┘
```

## ✅ Apply 前置条件

必须完成：
- [x] tasks.md（所有任务勾选）
- [x] quality-check.md（Health Score ≥ 80）
- [x] runtime-check.md（应用启动正常，无错误）

## 🎯 Health Score 要求

**目标**: ≥ 80 / 100

**检查项**:
1. TypeScript Type Checking (20分)
2. ESLint Code Quality (15分)
3. Prettier Formatting (10分)
4. Rust Compilation (25分)
5. Rust Unit Tests (20分)
6. TypeScript Unit Tests (20分)
7. Dependency Integrity (5分)
8. Directory Structure (5分)
9. Documentation (10分)

## 🔧 常用命令

```bash
# 查看当前 schema
openspec schemas

# 验证 schema
openspec schema validate harness-quality

# 查看变更状态
openspec status --change <name>

# 运行质量检查
npm run harness:check

# 自动修复问题
npm run harness:fix

# 查看详细检查结果
npm run harness:check -Verbose
```

## 📁 文件位置

- **Schema 定义**: `openspec/schemas/harness-quality/schema.yaml`
- **模板文件**: `openspec/schemas/harness-quality/templates/`
- **项目配置**: `openspec/config.yaml`
- **质量脚本**: `scripts/harness-check.ps1`

## 💡 提示

- ✅ 在实施过程中定期运行 `harness:check`
- ✅ 使用 `harness:fix` 自动解决格式问题
- ✅ 在 quality-check.md 中详细说明任何例外
- ✅ Health Score < 80 时需要团队批准才能归档

## 🔄 切换 Schema

临时使用默认 schema：
```bash
openspec new change my-feature --schema spec-driven
```

永久切换：
修改 `openspec/config.yaml` 中的 `schema` 字段

---

**更多信息**: 参见 [完整文档](./README.md)

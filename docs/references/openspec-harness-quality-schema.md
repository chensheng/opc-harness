# Harness Quality Schema

## 概述

`harness-quality` 是 OpenSpec 的自定义 schema，在标准工作流中集成了 **Harness Engineering 质量门禁**。

## 工作流程

```
proposal → specs → design → tasks → quality-check → runtime-check → (apply) → archive
```

### 关键区别

与默认的 `spec-driven` schema 相比：

1. **新增 `quality-check` artifact**：在所有任务完成后，必须运行 `npm run harness:check`
2. **新增 `runtime-check` artifact**：质量检查后，必须验证 Tauri 应用能正常启动和运行
3. **Apply 前置条件**：需要 `tasks` + `quality-check` + `runtime-check` 三个 artifacts 才能开始实施
4. **双重质量门禁**：Health Score ≥ 80 + 应用运行时零错误

## Artifacts

### 1. proposal.md
- **依赖**: 无
- **内容**: 变更提案（为什么、改什么、影响范围）

### 2. specs/**/*.md
- **依赖**: proposal
- **内容**: 详细规格（Given/When/Then 场景）

### 3. design.md
- **依赖**: proposal
- **内容**: 技术设计（如何实现、架构决策）

### 4. tasks.md
- **依赖**: specs + design
- **内容**: 实施任务清单（checkbox 格式）

### 5. quality-check.md ⭐ 新增
- **依赖**: tasks
- **内容**: Harness Engineering 健康检查报告
- **要求**: Health Score ≥ 80 / 100

### 6. runtime-check.md ⭐ 新增
- **依赖**: quality-check
- **内容**: Tauri 应用运行时验证报告
- **要求**: 应用能正常启动，前端无控制台错误，后端无 panic

## 使用方法

### 创建变更

```bash
# 使用新 schema 创建变更
openspec new change my-feature --schema harness-quality

# 或者在 openspec/config.yaml 中设置默认 schema 后直接创建
openspec new change my-feature
```

### 生成 Artifacts

按照依赖顺序依次创建：

```bash
# 1. 创建 proposal
openspec instructions proposal --change my-feature

# 2. 创建 specs 和 design（可并行）
openspec instructions specs --change my-feature
openspec instructions design --change my-feature

# 3. 创建 tasks
openspec instructions tasks --change my-feature

# 4. 实施任务
/opsx:apply my-feature

# 5. 运行静态质量检查
npm run harness:check
# 将结果记录到 quality-check.md
openspec instructions quality-check --change my-feature

# 6. 启动 Tauri 开发环境进行运行时验证
npm run tauri:dev
# 验证应用启动、功能正常、无错误日志
# 将结果记录到 runtime-check.md
openspec instructions runtime-check --change my-feature
```

### Apply 阶段

只有当 `tasks`、`quality-check` 和 `runtime-check` 都完成后，才能开始 apply：

```bash
/opsx:apply my-feature
```

系统会检查：
- ✅ 所有任务已完成（tasks.md 中全部勾选）
- ✅ Health Score ≥ 80（quality-check.md 中记录）
- ✅ 应用运行时验证通过（runtime-check.md 中记录）

### 归档阶段

归档前会再次验证质量门禁：

```bash
/opsx:archive my-feature
```

如果 Health Score < 80，会警告用户并要求确认。

## Quality Check 模板

`quality-check.md` 包含以下部分：

```markdown
# Quality Check Report

## Harness Engineering Health Score
**Target**: ≥ 80 / 100  
**Status**: PASS/FAIL  
**Score**: XX / 100

## Check Results
- TypeScript Type Checking: PASS/FAIL/WARN
- ESLint Code Quality: PASS/FAIL/WARN
- Prettier Formatting: PASS/FAIL/WARN
- Rust Compilation: PASS/FAIL/WARN/SKIP
- Rust Unit Tests: PASS/FAIL/WARN/SKIP
- TypeScript Unit Tests: PASS/FAIL/WARN/SKIP
- Dependency Integrity: PASS/FAIL/WARN
- Directory Structure: PASS/FAIL/WARN
- Documentation: PASS/FAIL/WARN

## Issues Found
### Errors
- ...

### Warnings
- ...

## Actions Taken
- [ ] Fix issue 1
- [ ] Fix issue 2

## Final Assessment
<!-- 评估是否满足质量标准 -->
```

## 配置

项目已在 `openspec/config.yaml` 中设置默认 schema：

```yaml
schema: harness-quality
```

如需切换回默认 schema：

```bash
# 临时使用 spec-driven
openspec new change my-feature --schema spec-driven

# 或修改 config.yaml
schema: spec-driven
```

## 优势

### ✅ 质量保证
- 每次变更都必须通过质量门禁
- 防止技术债务积累
- 自动化检查覆盖：TypeScript、ESLint、Prettier、Rust、测试

### ✅ 可追溯性
- quality-check.md 记录每次检查的结果
- 归档时保留完整的质量历史
- 便于审计和问题排查

### ✅ 灵活性
- 基于 OpenSpec 标准 schema 扩展
- 可随时切换回默认 schema
- 支持自定义阈值和检查项

## 最佳实践

1. **早期检查**：在 implementation 过程中定期运行 `harness:check`
2. **自动修复**：使用 `npm run harness:fix` 解决格式问题
3. **文档化**：在 quality-check.md 中详细说明任何例外情况
4. **团队协作**：Review 时重点关注 quality-check.md 中的问题和决策

## 故障排查

### Q: 如何查看当前使用的 schema？
```bash
openspec schemas
```

### Q: 如何验证 schema 是否正确？
```bash
openspec schema validate harness-quality
```

### Q: Health Score 低于 80 怎么办？
1. 运行 `npm run harness:check` 查看详细问题
2. 使用 `npm run harness:fix` 自动修复格式问题
3. 手动修复 TypeScript 错误和测试失败
4. 如果某些问题是可接受的，在 quality-check.md 中说明原因

### Q: 可以跳过 quality-check 吗？
不建议。但如果确实需要（如紧急修复），可以：
1. 在 quality-check.md 中说明跳过原因
2. 获得团队批准后继续
3. 后续补上质量检查

## 相关文件

- Schema 定义: `openspec/schemas/harness-quality/schema.yaml`
- 模板文件: `openspec/schemas/harness-quality/templates/`
- 项目配置: `openspec/config.yaml`
- 质量检查脚本: `scripts/harness-check.ps1`

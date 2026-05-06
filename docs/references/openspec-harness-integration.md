# OpenSpec + Harness:check 集成方案

## 📋 概述

已成功将 `harness:check` 质量门禁集成到 OpenSpec 工作流中，通过自定义 schema `harness-quality` 实现。

## 🎯 目标

确保每次变更在归档前都通过：
1. Harness Engineering 的质量检查（Health Score ≥ 80）
2. Tauri 应用运行时验证（正常启动，无错误日志）

## ✅ 已完成的工作

### 1. 创建自定义 Schema

**位置**: `openspec/schemas/harness-quality/`

**命令**:
```bash
openspec schema fork spec-driven harness-quality
```

**特点**:
- 基于默认的 `spec-driven` schema 派生
- 新增 `quality-check` artifact（静态代码质量）
- 新增 `runtime-check` artifact（运行时验证）
- 修改 apply 前置条件，要求 tasks + quality-check + runtime-check

### 2. Schema 结构

```yaml
name: harness-quality
artifacts:
  - proposal (无依赖)
  - specs (依赖: proposal)
  - design (依赖: proposal)
  - tasks (依赖: specs, design)
  - quality-check (依赖: tasks) ⭐ 静态质量
  - runtime-check (依赖: quality-check) ⭐ 运行时验证

apply:
  requires: [tasks, quality-check, runtime-check]  # 需要三个 artifacts
  tracks: tasks.md
```

### 3. Quality Check Artifact

**模板文件**: `openspec/schemas/harness-quality/templates/quality-check.md`

**内容结构**:
- Health Score (目标 ≥ 80)
- 9 项检查结果详情
- 发现的问题列表
- 已采取的修复行动
- 最终评估

### 4. Runtime Check Artifact ⭐ 新增

**模板文件**: `openspec/schemas/harness-quality/templates/runtime-check.md`

**内容结构**:
- Tauri 应用启动时间
- 前端控制台错误检查
- 后端 Rust 日志检查（panics、errors）
- 核心功能测试
- 性能观察
- 发现的问题
- 最终状态（PASS/FAIL）

### 5. 项目配置更新

**文件**: `openspec/config.yaml`

```yaml
schema: harness-quality  # 从 spec-driven 改为 harness-quality
```

### 6. 文档

- **Schema README**: `openspec/schemas/harness-quality/README.md`
- **集成说明**: 本文档

## 🔄 工作流程对比

### 原始流程 (spec-driven)

```
proposal → specs → design → tasks → apply → archive
```

### 新流程 (harness-quality)

```
proposal → specs → design → tasks → quality-check → runtime-check → apply → archive
                                                      ↓                ↓
                                              静态质量检查      运行时验证
                                              Health ≥ 80     无错误日志
```

## 📝 使用示例

### 创建变更

```bash
# 自动使用 harness-quality schema（已在 config.yaml 中设置）
openspec new change add-user-authentication
```

### 生成 Artifacts

```bash
# 按顺序创建 artifacts
openspec instructions proposal --change add-user-authentication
openspec instructions specs --change add-user-authentication
openspec instructions design --change add-user-authentication
openspec instructions tasks --change add-user-authentication
```

### 实施任务

```bash
# AI Agent 实施任务
/opsx:apply add-user-authentication
```

### 质量检查和运行时验证（新增步骤）

```bash
# 1. 运行静态质量检查
npm run harness:check

# 2. 根据结果创建 quality-check.md
openspec instructions quality-check --change add-user-authentication

# 3. 如果分数 < 80，修复问题
npm run harness:fix
# 重新检查直到 ≥ 80

# 4. 启动 Tauri 开发环境
npm run tauri:dev
# 等待应用启动（30-60秒）
# 检查前端控制台无错误
# 检查后端日志无 panic
# 测试核心功能
# 停止 dev server (Ctrl+C)

# 5. 创建 runtime-check.md
openspec instructions runtime-check --change add-user-authentication
```

### 归档

```bash
# 归档时会验证 quality-check.md 存在且分数达标
/opsx:archive add-user-authentication
```

## 🔍 验证

### Schema 验证

```bash
openspec schema validate harness-quality
# ✓ Schema 'harness-quality' is valid
```

### 查看可用 Schemas

```bash
openspec schemas
# - harness-quality (project) ⭐ 当前默认
# - spec-driven (package)
```

### 测试变更状态

```bash
openspec status --change <name> --json
# 会显示 quality-check artifact 的状态
```

## 🎨 关键设计决策

### 1. 为什么作为 artifact 而不是 hook？

**选择**: 将 quality-check 作为独立的 artifact

**理由**:
- ✅ 符合 OpenSpec 的设计理念（一切都是 artifact）
- ✅ 可追溯性：每次检查的结果都被记录
- ✅ 灵活性：可以在实施过程中多次检查
- ✅ 可见性：在变更目录中明确看到质量状态

**替代方案**（未采用）:
- ❌ Git hook：无法记录历史，不可追溯
- ❌ CI/CD pipeline：不在 OpenSpec 工作流内
- ❌ Apply skill 内部调用：不够透明

### 2. 为什么放在 tasks 之后？

**依赖关系**: `quality-check` requires `tasks`, `runtime-check` requires `quality-check`

**理由**:
- ✅ 所有代码完成后才能进行最终质量检查
- ✅ 确保测试覆盖率包含新功能
- ✅ 避免重复检查（实施过程中可以手动检查）
- ✅ 运行时验证在静态检查之后，形成完整的质量保障链

### 3. 为什么 apply 需要 quality-check 和 runtime-check？

**配置**:
```yaml
apply:
  requires: [tasks, quality-check, runtime-check]
```

**理由**:
- ✅ 强制双重质量门禁：静态分析 + 运行时验证
- ✅ 提前发现问题：在实施前就知道质量和运行状态
- ✅ 防止遗漏：AI Agent 会看到缺少 artifacts
- ✅ 全面保障：代码质量 + 应用功能都得到验证

### 4. Health Score 阈值设为 80

**理由**:
- ✅ 平衡严格性和实用性
- ✅ 允许一些警告（WARN 不扣分太多）
- ✅ 阻止严重错误（FAIL 扣很多分）

**调整方法**: 修改 `scripts/harness-check.ps1` 中的权重配置

## 🚀 优势

### 对开发者

1. **自动化质量保证**：每次变更都有明确的质量标准
2. **即时反馈**：在归档前就知道是否达标
3. **减少返工**：早期发现质量问题
4. **运行时保障**：确保应用能正常启动和运行，不仅仅是代码质量

### 对团队

1. **一致性**：所有变更遵循相同的质量和运行标准
2. **可审计**：每个变更都有质量和运行检查记录
3. **技术债务控制**：防止低质量代码和运行时问题进入主分支
4. **集成问题早发现**：在开发阶段就捕获集成错误

### 对项目

1. **持续改进**：Health Score 趋势可追踪
2. **风险管理**：低分变更或运行时错误会被标记和审查
3. **文档化**：质量和运行决策有明确记录
4. **用户信心**：保证每次变更都不会破坏应用

## 📊 效果预期

### Before

```
变更完成 → 手动检查（可能忘记）→ 归档
              ↓
         质量参差不齐，可能有运行时错误
```

### After

```
变更完成 → quality-check → runtime-check → 归档
           ↓                ↓
    Health Score ≥ 80   应用正常运行
                        无错误日志
            ↓
      双重质量保证
```

## 🔧 维护和扩展

### 添加新的检查项

1. 修改 `scripts/harness-check.ps1` 添加新检查
2. 更新 `templates/quality-check.md` 添加对应章节
3. 调整 ScoreWeights 配置

### 调整阈值

在 `schema.yaml` 的 instruction 中修改：
```yaml
description: Harness Engineering quality gate validation (Health Score ≥ 85 required)
```

### 创建更严格的 schema

```bash
openspec schema fork harness-quality harness-strict
# 然后修改 threshold 为 90
```

## 📚 相关文档

- [Schema 完整文档](./openspec-harness-quality-schema.md)
- [快速参考指南](./openspec-harness-quality-quickstart.md)
- [版本更新日志](./openspec-harness-quality-changelog.md)
- [Schema 定义](../openspec/schemas/harness-quality/schema.yaml)
- [Quality Check 模板](../openspec/schemas/harness-quality/templates/quality-check.md)
- [Runtime Check 模板](../openspec/schemas/harness-quality/templates/runtime-check.md)
- [项目配置](../openspec/config.yaml)
- [质量检查脚本](../scripts/harness-check.ps1)

## 🎓 学习要点

这个集成展示了如何：

1. **遵循 OpenSpec 规范**：使用 `fork` 命令从默认 schema 派生
2. **扩展工作流**：添加自定义 artifact 而不破坏原有流程
3. **集成外部工具**：将 `harness:check` 无缝嵌入 OpenSpec
4. **保持灵活性**：可随时切换回默认 schema

## ✨ 下一步建议

1. **试点使用**：在下一个变更中使用 `harness-quality` schema
2. **收集团队反馈**：了解实际使用中的问题
3. **优化模板**：根据使用情况改进 quality-check.md 和 runtime-check.md 模板
4. **考虑自动化**：未来可以让 AI Agent 自动运行 harness:check 和 tauri:dev 并填充报告
5. **监控指标**：追踪 Health Score 和运行时错误的变化趋势
6. **集成 CI/CD**：在持续集成中自动执行这些检查

---

**创建日期**: 2026-05-06  
**Schema 版本**: 2.0 (包含 runtime-check)  
**基于**: spec-driven schema from OpenSpec package

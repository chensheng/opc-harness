# 文档迁移指南

> **迁移变更**: migrate-docs-to-openspec  
> **执行日期**: 2026-05-06  
> **状态**: 已完成提案和设计,待实施

---

## 📋 迁移概述

本次迁移将 `docs/` 目录下的传统文档体系整合到 OpenSpec 工作流中,实现文档即代码(Docs as Code)。

### 迁移原则

1. **保留核心参考文档** - `docs/` 目录保留作为快速导航入口
2. **归档历史文档** - 已完成的执行计划和决策记录归档到 OpenSpec
3. **建立 Capabilities 体系** - 在 OpenSpec 中定义 5 个新的能力规范
4. **保持链接有效** - 所有内部引用路径保持可访问

---

## 📊 当前文档统计

```
docs/ 目录总计: 51 个 markdown 文件

├── 根目录 (3 个)
│   ├── data-storage.md
│   ├── dev_workflow.md
│   └── migration-guide.md
│
├── design-docs/ (3 个)
│   ├── architecture-rules.md ⭐ 保留为核心参考
│   ├── system-architecture.md ⭐ 保留为核心参考
│   └── index.md
│
├── design-docs/decision-records/ (3 个) 📦 待归档
│   └── [3 个 ADR 文件]
│
├── exec-plans/ (1 个)
│   └── index.md
│
├── exec-plans/completed/ (20 个) 📦 待归档
│   └── [20 个已完成执行计划]
│
├── product-specs/ (5 个) ⭐ 保留为产品规格
│   ├── index.md
│   ├── product-design.md
│   ├── vibe-coding-spec.md
│   ├── vibe-design-spec.md
│   └── vibe-marketing-spec.md
│
├── references/ (8 个)
│   ├── autonomous-coding-harness.md
│   ├── best-practices.md
│   ├── index.md
│   ├── openspec-harness-integration.md
│   ├── openspec-harness-quality-changelog.md
│   ├── openspec-harness-quality-quickstart.md
│   ├── openspec-harness-quality-schema.md
│   └── symphony.md
│
└── sprint-plans/ (4 个)
    ├── index.md
    ├── sprint-1.md
    ├── sprint-2.md
    └── sprint-guide.md
```

---

## 🎯 迁移策略

### 1. 归档历史执行计划 (20 个文件)

**位置**: `docs/exec-plans/completed/` → `openspec/changes/archive/YYYY-MM-DD-<plan-name>/`

**操作**:
- 为每个执行计划创建最小化 OpenSpec change
- 仅包含 proposal.md (标题 + 摘要)
- 原始文件作为附件保留在 archived change 中

**示例**:
```
openspec/changes/archive/2026-05-06-us-001-agent-communication/
├── .openspec.yaml
├── proposal.md
└── attachments/
    └── US-001-agent-communication-protocol.md (原始文件)
```

### 2. 归档决策记录 (3 个 ADR)

**位置**: `docs/design-docs/decision-records/` → `openspec/changes/archive/YYYY-MM-DD-adr-<title>/`

**操作**:
- 为每个 ADR 创建独立的 OpenSpec change
- 保留完整的 ADR 内容在 proposal.md 中
- 在 `docs/design-docs/` 保留索引指向归档位置

### 3. 保留核心参考文档

**保留在 `docs/` 的文档**:
- ✅ `docs/design-docs/system-architecture.md` - 系统架构
- ✅ `docs/design-docs/architecture-rules.md` - 架构约束
- ✅ `docs/product-specs/*` - 产品规格 (5 个文件)
- ✅ `docs/dev_workflow.md` - 开发流程
- ✅ `docs/references/*` - 参考文档 (8 个文件)
- ✅ `docs/sprint-plans/*` - Sprint 计划 (4 个文件)

**理由**: 这些是跨变更的全局规范,不适合作为单个 OpenSpec change

### 4. 新建 OpenSpec Capabilities

已在 `openspec/changes/migrate-docs-to-openspec/specs/` 创建 5 个 capabilities:

1. **document-management** - 文档分类、命名、归档、引用完整性
2. **design-documentation** - 架构文档结构、ADR、约束强制执行
3. **product-specification** - 产品规格层级、版本管理、可测试性
4. **execution-tracking** - 执行计划生命周期、模板、技术债务
5. **sprint-planning** - Sprint 结构、归档、进度跟踪、回顾

---

## 🔗 引用更新清单

### 需要更新的文档

1. **AGENTS.md**
   - 更新文档导航章节
   - 添加 OpenSpec capabilities 引用

2. **docs/product-specs/index.md**
   - 添加指向 OpenSpec specs 的链接
   - 标注哪些是产品级规格,哪些是 capability 级

3. **docs/sprint-plans/index.md**
   - 添加与 OpenSpec changes 的双向链接

4. **docs/exec-plans/index.md**
   - 移除已归档计划的直接链接
   - 添加指向 archive 目录的索引

5. **docs/design-docs/index.md**
   - 添加 ADR 归档位置说明

---

## ✅ 验证清单

### 链接验证
- [ ] 所有 markdown 内部链接可访问
- [ ] 无断裂的相对路径引用
- [ ] AGENTS.md 导航正确

### Schema 验证
- [ ] 所有 spec 文件通过 `openspec validate`
- [ ] tasks.md 格式正确 (checkbox 格式)
- [ ] proposal/design 符合 schema 要求

### 质量门禁
- [ ] `npm run harness:check` 通过 (Health Score ≥ 80)
- [ ] Prettier 格式化一致
- [ ] TypeScript 编译通过

### 运行时验证
- [ ] Tauri 应用正常启动
- [ ] 无控制台错误
- [ ] 后端日志无异常

---

## 📝 实施步骤

### Phase 1: 归档准备 (已完成)
- [x] 统计分析 docs/ 目录
- [x] 识别归档目标
- [x] 创建 OpenSpec change 框架
- [x] 定义 5 个 capabilities

### Phase 2: 执行归档 (待实施)
- [ ] 归档 20 个执行计划
- [ ] 归档 3 个 ADR
- [ ] 创建归档索引

### Phase 3: 更新引用 (待实施)
- [ ] 更新 AGENTS.md
- [ ] 更新各 index.md 文件
- [ ] 修复断裂链接

### Phase 4: 质量验证 (待实施)
- [ ] 运行 harness:check
- [ ] 创建 quality-check.md
- [ ] 创建 runtime-check.md

### Phase 5: 完成归档 (待实施)
- [ ] 编写迁移总结
- [ ] 归档 migrate-docs-to-openspec change

---

## 🚀 后续维护

### 新文档创建流程

1. **技术变更文档** → 使用 OpenSpec workflow
   ```bash
   /opsx:propose <change-name>
   ```

2. **产品规格更新** → 更新 `docs/product-specs/`
   - 同时考虑是否需要在 OpenSpec 中新增 capability

3. **Sprint 计划** → 继续在 `docs/sprint-plans/` 维护
   - Sprint 结束后移动到 `archive/`

4. **架构决策** → 创建 ADR 并归档到 OpenSpec
   ```bash
   openspec new change "adr-<decision-title>"
   ```

### 定期归档

建议每季度执行一次文档归档:
1. 检查 `docs/exec-plans/completed/` 是否有新完成的计划
2. 检查 `docs/design-docs/decision-records/` 是否有新 ADR
3. 批量归档到 `openspec/changes/archive/`
4. 更新索引文件

---

## 📚 相关资源

- [OpenSpec Harness Integration](./references/openspec-harness-integration.md)
- [Harness Quality Schema](../openspec/schemas/harness-quality/schema.yaml)
- [Dev Workflow](./dev_workflow.md)
- [Architecture Rules](./design-docs/architecture-rules.md)

---

**最后更新**: 2026-05-06  
**维护者**: OPC-HARNESS Team

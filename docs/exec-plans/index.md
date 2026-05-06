# 执行计划使用指南

> 本目录存储所有活跃和已完成的执行计划  
> **核心理念**: 计划驱动执行，复盘促进改进  
> **文档定位**: Sprint 战略 → 执行计划战术 → 交付物成果  
> **最后更新**: 2026-05-06  
> **OpenSpec 归档**: [历史执行计划](../../openspec/changes/archive/2026-05-06-exec-plans-batch-1/)

---

## 📂 目录结构

```
exec-plans/
├── index.md                          # 当前文件（使用指南）
├── active/                           # 活跃的执行计划 (当前为空)
├── completed/                        # 已完成的执行计划 (已归档到 OpenSpec)
├── tech-debts/                       # 技术债务
├── templates/                        # 模板
│   ├── how-to-create-exec-plan.md    # 创建执行计划指引
│   └── how-to-complete-exec-plan.md  # 归档执行计划指引
|   └── how-to-track-tech-debt.md     # 技术债务追踪指引
```

**注意**: `completed/` 目录下的 20 个历史执行计划已归档到 OpenSpec:
- 归档位置: `openspec/changes/archive/2026-05-06-exec-plans-batch-1/attachments/`
- 包含: US-031 ~ US-061 共 20 个执行计划
- 详见: [执行计划归档提案](../../openspec/changes/archive/2026-05-06-exec-plans-batch-1/proposal.md)

---

## 🚀 快速开始

### 创建执行计划

**何时**: 开始执行新任务前，或从 Sprint 规划中选择任务后立即创建

**步骤**: [执行计划创建指引](./templates/how-to-create-exec-plan.md)

---

### 归档执行计划

**何时**: 任务完成后（harness:check 通过）

**步骤**: [执行计划归档指引](./templates/how-to-archive-exec-plan.md)

---

### 追踪技术债务

**何时**: 创建技术债务时

**步骤**: [技术债务追踪指引](./templates/how-to-track-tech-debt.md)

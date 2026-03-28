# 执行计划：TD-003 文档链接更新

> **状态**: 🔄 进行中  
> **优先级**: P3  
> **创建日期**: 2026-03-28  
> **预计完成**: 2026-03-28 周  
> **负责人**: OPC-HARNESS Team  
> **文档版本**: v1.0  
> **最后更新**: 2026-03-28  

---

## 📋 任务概述

**技术债务**: TD-003 - 文档链接未完全更新

**问题描述**: 
部分旧文档仍引用已移动的文件路径，导致 Agent 可能找到过时的导航信息。

**目标**: 
- 识别所有过期的文档链接
- 更新引用到正确的路径
- 验证所有链接有效性
- 确保 Agent 导航文档最新

---

## 🎯 执行步骤

### Phase 1: 现状分析（20 分钟）
- [x] 阅读 TD-003 技术债务文档
- [ ] 搜索过期链接模式
- [ ] 识别受影响的文件
- [ ] 记录问题清单

### Phase 2: 链接修复（40 分钟）
- [ ] 更新技术债务文档中的引用
- [ ] 修复 ARCHITECTURE.md 中的链接
- [ ] 清理历史文档中的死链

### Phase 3: 验证测试（20 分钟）
- [ ] 运行 markdown-link-check
- [ ] 手动验证关键链接
- [ ] 确认 Agent 导航正确

### Phase 4: 文档与归档（20 分钟）
- [ ] 更新 TD-003 文档状态
- [ ] 更新技术债务追踪器
- [ ] 创建完成报告
- [ ] Git 提交归档

---

## 📊 验收标准

- [ ] 所有 Markdown 文件无死链
- [ ] Agent 导航文档更新为最新结构
- [ ] ARCHITECTURE.md 链接正确
- [ ] Harness Health Score ≥ 90/100

---

## 🕐 时间估算

| 阶段 | 预计时间 | 实际时间 |
|------|---------|---------|
| Phase 1: 现状分析 | 20 分钟 | - |
| Phase 2: 链接修复 | 40 分钟 | - |
| Phase 3: 验证测试 | 20 分钟 | - |
| Phase 4: 文档归档 | 20 分钟 | - |
| **总计** | **100 分钟** | - |

---

## 📝 实施日志

### 2026-03-28: Phase 1 - 现状分析 ✅

**开始时间**: 21:30
**结束时间**: 21:45

**活动**:
- ✅ 阅读 TD-003 文档
- ✅ 搜索过期链接模式
  - 搜索 `docs/exec-plans/completed/` - 无结果 ✅
  - 搜索 `docs/execution/` - 无结果 ✅
  - 搜索 `tech-debt-tracker.md` 引用 - 无结果 ✅
- ✅ 识别受影响的文件
  - ❌ `src/components/vibe-coding/README_CP002.md` - 2 个错误链接
  - ❌ `eslint-rules/README.md` - 3 个错误链接
- ✅ 记录问题清单
  - README_CP002.md: `../../product-specs/mvp-roadmap.md` → `../../../docs/product-specs/mvp-roadmap.md`
  - README_CP002.md: `../../架构设计.md` → `../../../../ARCHITECTURE.md`
  - eslint-rules/README.md: `../../docs/HARNESS_ENGINEERING.md` → `../docs/HARNESS_ENGINEERING.md`
  - eslint-rules/README.md: `../../tests/architecture/constraints.test.ts` → `../tests/architecture/constraints.test.ts`
  - eslint-rules/README.md: `../docs/exec-plans/tech-debts/TD-004...md` → `../docs/exec-plans/tech-debts/TD-004...md` (正确)

**状态**: 完成

### 2026-03-28: Phase 2 - 链接修复 ✅

**开始时间**: 21:45
**结束时间**: 22:00

**活动**:
- ✅ 修复 `src/components/vibe-coding/README_CP002.md`
  - MVP 版本规划链接路径修正
  - 架构设计链接路径修正
- ✅ 修复 `eslint-rules/README.md`
  - Harness Engineering 流程链接路径修正
  - 架构约束规则链接路径修正
- ✅ 验证所有修复的链接
  - 使用 Test-Path 验证文件存在性

**状态**: 完成

### 2026-03-28: Phase 3 - 验证测试 🔄

**开始时间**: 22:00

**活动**:
- [ ] 运行 markdown-link-check
- [ ] 手动验证关键链接
- [ ] 确认 Agent 导航正确

**状态**: 进行中

---

下一步：Phase 4 - 文档与归档

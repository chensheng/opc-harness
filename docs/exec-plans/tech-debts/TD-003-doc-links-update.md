# TD-003: 文档链接未完全更新

## 📋 基本信息

- **创建日期**: 2026-03-22
- **优先级**: P3 (轻微)
- **状态**: 🔄 进行中
- **影响范围**: 文档系统
- **负责人**: 未分配
- **偿还计划**: 2026-03-25 前完成

---

## 📝 问题描述

部分旧文档仍引用已移动的文件路径，导致 Agent 可能找到过时的导航信息。

### 影响范围

1. **旧执行计划**: 引用已移动的文档路径
2. **历史任务报告**: 包含过时的文件引用
3. **ARCHITECTURE.md**: 部分链接未更新

---

## 🎯 解决方案

### 步骤 1: 识别过期链接

```bash
# 搜索旧路径引用
grep -r "docs/exec-plans/completed/" docs/
grep -r "docs/execution/" docs/
```

### 步骤 2: 更新链接

将旧路径映射到新路径：

| 旧路径 | 新路径 |
|--------|--------|
| `docs/exec-plans/completed/*.md` | `docs/exec-plans/completed/` (保持不变) |
| `docs/tech-debt-tracker.md` | `docs/exec-plans/tech-debts/*.md` |

### 步骤 3: 验证链接有效性

```bash
# 使用 markdown-link-check
npx markdown-link-check docs/**/*.md
```

---

## ✅ 验收标准

- [ ] 所有 Markdown 文件无死链
- [ ] Agent 导航文档更新为最新结构
- [ ] ARCHITECTURE.md 链接正确

---

## 📊 进度追踪

- [x] 识别过期链接
- [ ] 更新技术债务相关引用
- [ ] 验证所有链接

---

**最后更新**: 2026-03-24

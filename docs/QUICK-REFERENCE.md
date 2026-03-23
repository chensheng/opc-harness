# 文档体系快速参考指南

> **用途**: 5 分钟了解 OPC-HARNESS 文档体系  
> **适用人群**: 新加入的 AI Agent 和开发者  
> **最后更新**: 2026-03-23

## 🎯 快速导航

### 首次访问 (必读) ⭐

1. **[AGENTS.md](../AGENTS.md)** - AI Agent 导航地图
   - 了解项目整体结构
   - 找到所需文档的位置
   - 学习 Harness Engineering 三大支柱

2. **[README.md](../README.md)** - 项目概述
   - 快速开始指南
   - 核心功能介绍
   - 技术栈说明

3. **[docs/MAINTENANCE.md](./MAINTENANCE.md)** - 文档维护清单 ⭐
   - 所有文档的索引表
   - 责任人和审查频率
   - 质量指标评分

---

## 📚 按需求查找文档

### 我要...

#### 🔧 开发新功能
→ 查看 [`design-docs/`](./design-docs/) 技术方案和 ADRs

#### 📝 编写产品需求
→ 查看 [`product-specs/`](./product-specs/) 产品规范

#### 📊 追踪执行进度
→ 查看 [`exec-plans/active/`](./exec-plans/active/) 活跃计划

#### 📖 学习最佳实践
→ 查看 [`references/best-practices.md`](./references/best-practices.md)

#### 🏗️ 了解架构约束
→ 查看 [`references/architecture-rules.json`](./references/architecture-rules.json)

#### 🤖 使用 Harness 工具
→ 查看 [`references/harness-user-guide.md`](./references/harness-user-guide.md)

---

## 🚀 常用命令

### 文档验证
```bash
# 验证文档结构完整性
npm run harness:validate:docs

# 架构健康检查 (包含文档检查)
npm run harness:check -- -All
```

### 文档清理
```bash
# 清理过时文档 (会询问确认)
npm run harness:gc

# 空运行模式 (查看将删除什么)
npm run harness:gc -- -DryRun
```

---

## 📋 文档层级

```
Level 1: AGENTS.md          ← 导航地图 (必读)
    ↓
Level 2: src/AGENTS.md      ← 前端规范
         src-tauri/AGENTS.md ← Rust 规范
    ↓
Level 3: docs/*             ← 详细设计
    ├── design-docs/        # 技术方案
    ├── exec-plans/         # 执行计划
    ├── product-specs/      # 产品需求
    ├── references/         # 参考资料
    └── generated/          # 自动生成
```

---

## 🔍 文档维护

### 审查周期

| 频率 | 项目 | 负责人 |
|------|------|--------|
| **每周** | 更新执行计划进度 | 项目负责人 |
| **每月** | 审查技术债务 | Tech Lead |
| **每季度** | 全面文档审查 ⭐ | 全体团队 |

### 新鲜度规则

- ✅ <90 天：正常
- ⚠️ >90 天：需要更新
- ❌ >180 天：考虑归档或删除

### 添加新文档

1. 创建文档文件
2. 在对应 `index.md` 添加链接
3. 添加"最后更新日期"
4. 运行 `npm run harness:validate:docs` 验证

---

## 🎯 文档质量指标

### 评分计算 (满分 100)

| 指标 | 权重 | 检查项 |
|------|------|--------|
| 关键文档存在性 | 30% | AGENTS.md, README.md 等是否齐全 |
| 索引完整性 | 20% | index.md 是否包含有效链接 |
| 文档新鲜度 | 50% | 是否<90 天未更新 |

**目标**: ≥90 分 ✨

**当前得分**: 97/100

---

## ❓ 常见问题

### Q: 如何知道哪些文档需要更新？
A: 查看 [`MAINTENANCE.md`](./MAINTENANCE.md) 中的文档清单，标记⚠️的需要关注。

### Q: 文档应该放在哪个目录？
A: 
- 技术方案 → `design-docs/`
- 执行计划 → `exec-plans/active/`
- 产品需求 → `product-specs/`
- 参考资料 → `references/`

### Q: 如何归档已完成的计划？
A: 移动到 `exec-plans/completed/` 并在 MAINTENANCE.md 中更新状态。

### Q: 发现文档错误怎么办？
A: 
1. 立即修正
2. 更新"最后更新日期"
3. 运行验证确保无其他问题

---

## 🔗 重要链接

### 核心文档
- [AGENTS.md](../AGENTS.md) - 导航地图
- [MAINTENANCE.md](./MAINTENANCE.md) - 维护清单
- [README.md](../README.md) - 项目概述

### Harness Engineering
- [harness-user-guide.md](./references/harness-user-guide.md) - 完整指南
- [harness-quickstart.md](./references/harness-quickstart.md) - 快速入门
- [best-practices.md](./references/best-practices.md) - 最佳实践

### 外部资源
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [ADR 指南](https://adr.github.io/)

---

## 📞 获取帮助

### 文档相关问题
1. 首先查阅 [`MAINTENANCE.md`](./MAINTENANCE.md)
2. 查看 [`exec-plans/active/documentation-cleanup-2026-03.md`](./exec-plans/active/documentation-cleanup-2026-03.md)
3. 联系文档责任人 (见 MAINTENANCE.md 表格)

### 技术相关问题
1. 前端：[`src/AGENTS.md`](../src/AGENTS.md)
2. 后端：[`src-tauri/AGENTS.md`](../src-tauri/AGENTS.md)
3. 架构：[`ARCHITECTURE.md`](../ARCHITECTURE.md)

---

**维护者**: OPC-HARNESS Team  
**最后更新**: 2026-03-23  
**下次审查**: 2026-06-23

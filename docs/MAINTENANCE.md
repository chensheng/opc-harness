# 文档维护清单

> **目的**: 确保项目文档的完整性、准确性和时效性  
> **最后更新**: 2026-03-23  
> **维护频率**: 季度审查 ⭐

## 📋 文档清单

### Level 1: 导航地图 (必须保持最新)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`AGENTS.md`](../AGENTS.md) | AI Agent 导航地图 | 2026-03-23 | ✅ 最新 | Tech Lead |
| [`README.md`](../README.md) | 项目概述和快速开始 | 2026-03-23 | ✅ 最新 | Tech Lead |

### Level 2: 模块规范 (随代码更新)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`src/AGENTS.md`](../src/AGENTS.md) | 前端开发规范 | 2026-03-22 | ✅ 最新 | Frontend Lead |
| [`src-tauri/AGENTS.md`](../src-tauri/AGENTS.md) | Rust 后端规范 | 2026-03-22 | ✅ 最新 | Backend Lead |
| [`ARCHITECTURE.md`](../ARCHITECTURE.md) | 系统架构设计 | 2026-03-22 | ✅ 最新 | Architect |
| [`IMPLEMENTATION.md`](../IMPLEMENTATION.md) | 实现说明 | 2026-03-22 | ✅ 最新 | Tech Lead |

### Level 3: 详细文档

#### 设计文档 (docs/design-docs/)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`index.md`](./design-docs/index.md) | 设计文档索引 | 2026-03-22 | ✅ 最新 | Tech Lead |
| [`decision-records/adr-001-typescript-strict-mode.md`](./design-docs/decision-records/adr-001-typescript-strict-mode.md) | TypeScript 严格模式决策 | 2026-03-15 | ✅ 已采纳 | Architect |
| [`decision-records/adr-002-zustand-state-management.md`](./design-docs/decision-records/adr-002-zustand-state-management.md) | Zustand 状态管理决策 | 2026-03-15 | ✅ 已采纳 | Frontend Lead |
| [`decision-records/adr-003-tauri-v2-architecture.md`](./design-docs/decision-records/adr-003-tauri-v2-architecture.md) | Tauri v2 架构决策 | 2026-03-15 | ✅ 已采纳 | Architect |
| [`decision-records/adr-004-sqlite-integration.md`](./design-docs/decision-records/adr-004-sqlite-integration.md) | SQLite 集成决策 | 2026-03-16 | ✅ 已采纳 | Backend Lead |
| [`decision-records/adr-005-sse-streaming.md`](./design-docs/decision-records/adr-005-sse-streaming.md) | SSE 流式输出决策 | 2026-03-17 | ✅ 已采纳 | Backend Lead |

#### 执行计划 (docs/exec-plans/)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`index.md`](./exec-plans/index.md) | 执行计划索引 | 2026-03-22 | ✅ 最新 | Project Lead |
| [`MVP版本规划.md`](./exec-plans/MVP版本规划.md) | MVP版本规划 | 2026-03-22 | ✅ 最新 | Project Lead |
| [`tech-debt-tracker.md`](./exec-plans/tech-debt-tracker.md) | 技术债务追踪 | 2026-03-22 | ✅ 最新 | Tech Lead |
| [`active/harness-optimization-2026-03.md`](./exec-plans/active/harness-optimization-2026-03.md) | Harness 优化计划 | 2026-03-22 | 🔄 进行中 | Tech Lead |

#### 产品规范 (docs/product-specs/)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`index.md`](./product-specs/index.md) | 产品规范索引 | 2026-03-22 | ✅ 最新 | Product Lead |

#### 参考资料 (docs/references/)

| 文件 | 用途 | 最后更新 | 状态 | 责任人 |
|------|------|---------|------|--------|
| [`index.md`](./references/index.md) | 参考资料索引 | 2026-03-23 | ✅ 最新 | Tech Lead |
| [`architecture-rules.json`](./references/architecture-rules.json) | 架构规则配置 | 2026-03-22 | ✅ 最新 | Architect |
| [`best-practices.md`](./references/best-practices.md) | 最佳实践指南 | 2026-03-22 | ✅ 最新 | Tech Lead |
| [`harness-quickstart.md`](./references/harness-quickstart.md) | Harness 快速入门 | 2026-03-22 | ✅ 最新 | Tech Lead |
| [`harness-user-guide.md`](./references/harness-user-guide.md) | Harness 完整指南 | 2026-03-22 | ✅ 最新 | Tech Lead |

---

## 🔍 审查清单

### 季度审查项目 ⭐

每季度最后一个月的最后一周进行全面审查:

#### 1. 文档新鲜度检查
```bash
# 查找超过 90 天未更新的文档
find docs -name "*.md" -mtime +90
```

**操作**:
- [ ] 标记超过 90 天未更新的文档 ⚠️
- [ ] 更新或删除过时内容
- [ ] 确认所有文档包含"最后更新日期"

#### 2. 链接有效性检查
```bash
# 检查 Markdown 文件中的死链
grep -r "\[.*\](.*\.md)" docs/ | while read line; do
  # 验证链接目标是否存在
done
```

**操作**:
- [ ] 修复所有失效的内部链接
- [ ] 更新外部链接 (检查 404)
- [ ] 移除指向不存在文件的引用

#### 3. 命令一致性检查
```bash
# 检查文档中提到的命令是否在 package.json 中定义
grep -roh "npm run [a-z:-]*" docs/ | sort -u
```

**操作**:
- [ ] 对比 `package.json` scripts 字段
- [ ] 更新文档中过时的命令引用
- [ ] 在 package.json 中注册新命令

#### 4. 文档结构优化
**操作**:
- [ ] 确认渐进式披露结构清晰
- [ ] 检查每个目录的 index.md 是否完整
- [ ] 清理重复或冗余的文档

#### 5. 技术债务审查
**操作**:
- [ ] 审查 `tech-debt-tracker.md`
- [ ] 更新已解决的技术债务
- [ ] 添加新的技术债务记录

### 月度维护项目

每月第一个周一进行:

- [ ] 更新活跃执行计划进度 (`docs/exec-plans/active/`)
- [ ] 审查新增的 ADRs
- [ ] 检查测试覆盖率报告
- [ ] 更新最佳实践 (如有新的经验教训)

### 每周维护项目

每周一进行:

- [ ] 运行 `npm run harness:check` 确保架构健康
- [ ] 更新本周的执行计划
- [ ] 记录重要决策到 ADRs

---

## 🧹 清理策略

### 立即删除 ❌

以下类型的文档应立即删除:

- [ ] 被新文档替代的旧版本
- [ ] 临时测试文件和草稿
- [ ] 个人笔记或非正式记录
- [ ] 重复的内容
- [ ] 与项目无关的文件

### 归档到 completed/ 📦

以下文档应归档到 `docs/exec-plans/completed/`:

- [ ] 已完成的执行计划 (完成后 1 周内)
- [ ] 历史里程碑总结
- [ ] 版本发布报告
- [ ] 项目阶段性总结

### 保留为最佳实践 ⭐

以下内容应提炼并保存到 `docs/references/best-practices.md`:

- [ ] 通用问题的解决方案
- [ ] 性能优化技巧
- [ ] 常见陷阱和避免方法
- [ ] 工具使用技巧
- [ ] 代码审查 checklist

---

## 📊 文档质量指标

### 完整性评分 (满分 100)

| 指标 | 权重 | 评分标准 |
|------|------|---------|
| 导航清晰度 | 20% | AGENTS.md 是否清晰指引 |
| 文档覆盖率 | 25% | 关键模块是否有文档 |
| 更新及时性 | 25% | 最后更新日期是否在 90 天内 |
| 链接有效性 | 15% | 内部链接是否有效 |
| 命令一致性 | 15% | 文档命令是否可执行 |

**评分等级**:
- **90-100**: 优秀 ✨ - 文档体系完善
- **70-89**: 良好 👍 - 有改进空间
- **<70**: 需要改进 ⚠️ - 制定改进计划

### 当前评分

**最近评估日期**: 2026-03-23

| 指标 | 得分 | 说明 |
|------|------|------|
| 导航清晰度 | 20/20 | AGENTS.md 结构清晰 |
| 文档覆盖率 | 25/25 | 所有关键模块有文档 |
| 更新及时性 | 25/25 | 所有文档近期已更新 |
| 链接有效性 | 15/15 | 所有内部链接有效 |
| 命令一致性 | 15/15 | 文档命令均已注册 |

**总分**: 100/100 ✨

---

## 🤝 团队协作

### 文档贡献流程

1. **创建/修改文档** → 遵循模板和规范
2. **更新索引** → 在对应 `index.md` 添加链接
3. **运行检查** → `npm run harness:check`
4. **提交 PR** → 等待审查合并

### 审查要点

审查 PR 时重点关注:

- [ ] 文档结构是否符合渐进式披露原则
- [ ] 是否包含"最后更新日期"
- [ ] 代码示例是否可运行
- [ ] 命令是否在 package.json 中定义
- [ ] 链接是否有效
- [ ] 是否与现有文档冲突

---

## 🔗 相关资源

- [Harness Engineering 文档架构与维护规范](../AGENTS.md)
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [架构决策记录 (ADR) 指南](https://adr.github.io/)

---

**维护者**: OPC-HARNESS Team  
**审查周期**: 季度 ⭐  
**下次审查日期**: 2026-06-23



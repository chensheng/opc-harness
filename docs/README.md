# OPC-HARNESS 文档中心

> **基于 OpenAI Harness Engineering 最佳实践**  
> 最后更新：2026-03-23  
> **维护负责人**: OPC-HARNESS Team

## 🚀 快速开始

### 首次访问？ ⭐
- **[5 分钟快速参考](./QUICK-REFERENCE.md)** - 快速了解文档体系
- **[AGENTS.md](../AGENTS.md)** - AI Agent 导航地图 (必读)
- **[MAINTENANCE.md](./MAINTENANCE.md)** - 文档维护清单

### 查找文档？
- 技术方案 → [`design-docs/`](./design-docs/)
- 执行计划 → [`exec-plans/active/`](./exec-plans/active/)
- 产品需求 → [`product-specs/`](./product-specs/)
- 参考资料 → [`references/`](./references/)

### 验证文档？
```bash
npm run harness:validate:docs    # 验证文档结构完整性
```

---

## 📚 文档导航

### 核心索引

| 目录 | 用途 | 入口文件 | 维护状态 |
|------|------|---------|---------|
| [设计文档](./design-docs/) | 技术方案、架构决策 (ADRs) | [`index.md`](./design-docs/index.md) | ✅ 活跃 |
| [执行计划](./exec-plans/) | 活跃/已完成计划、技术债务 | [`index.md`](./exec-plans/index.md) | ✅ 活跃 |
| [产品规范](./product-specs/) | 产品需求文档、功能规格 | [`index.md`](./product-specs/index.md) | ✅ 活跃 |
| [参考资料](./references/) | 外部参考、最佳实践、工具库 | [`index.md`](./references/index.md) | ✅ 活跃 |
| [生成文档](./generated/) | 自动生成的文档 (如数据库 Schema) | - | 🔄 自动生成 |

### 根目录关键文档

- [`AGENTS.md`](../AGENTS.md) - AI Agent 导航地图（必读）⭐
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) - 系统架构设计
- [`src/AGENTS.md`](../src/AGENTS.md) - 前端开发规范
- [`src-tauri/AGENTS.md`](../src-tauri/AGENTS.md) - Rust 后端规范
- [`README.md`](../README.md) - 项目概述和快速开始

### Harness Engineering ⭐

所有 Harness Engineering 的完整文档已迁移到 [`references/`](./references/):

- **快速入门**: [`harness-quickstart.md`](./references/harness-quickstart.md) - 30 秒了解 Harness
- **完整指南**: [`harness-user-guide.md`](./references/harness-user-guide.md) - 详细使用手册
- **最佳实践**: [`best-practices.md`](./references/best-practices.md) - 编码最佳实践
- **架构规则**: [`architecture-rules.json`](./references/architecture-rules.json) - 前后端约束规则

---

## 🗺️ 文档地图

```
docs/
│
├── README.md ⭐                     # 文档中心导航 (本文件)
├── MAINTENANCE.md ⭐                # 文档维护清单 (新增)
│
├── design-docs/                     # 技术方案和架构决策
│   ├── index.md                    # 设计文档索引
│   └── decision-records/           # 架构决策记录 (ADRs)
│       ├── adr-001-typescript-strict-mode.md
│       ├── adr-002-zustand-state-management.md
│       ├── adr-003-tauri-v2-architecture.md
│       ├── adr-004-sqlite-integration.md
│       └── adr-005-sse-streaming.md
│
├── exec-plans/                      # 执行计划和决策日志
│   ├── index.md                    # 执行计划索引
│   ├── MVP版本规划.md
│   ├── tech-debt-tracker.md        # 技术债务追踪
│   ├── active/                     # 活跃的执行计划 ⭐
│   │   ├── harness-optimization-2026-03.md
│   │   └── documentation-cleanup-2026-03.md (新增)
│   └── completed/                  # 已完成的执行计划
│       └── log-template.md
│
├── product-specs/                   # 产品需求文档
│   └── index.md                    # 产品规范索引
│
├── references/                      # 参考资料库
│   ├── index.md                    # 参考资料索引
│   ├── architecture-rules.json     # 架构规则配置
│   ├── best-practices.md           # 最佳实践指南
│   ├── harness-quickstart.md       # Harness 快速入门
│   ├── harness-user-guide.md       # Harness 完整指南
│   ├── autonomous-coding-harness.md # Autonomous Coding Harness
│   ├── symphony.md                 # Symphony 架构设计
│   ├── 产品设计.md                  # 产品设计文档
│   └── 架构设计.md                  # 架构设计文档
│
└── generated/                       # 自动生成的文档
    └── database-schema.md          # 数据库 Schema (示例)
```

### 使用指南

**快速查找**:
- 找技术方案 → `design-docs/`
- 找执行计划 → `exec-plans/active/`
- 找产品需求 → `product-specs/`
- 找最佳实践 → `references/best-practices.md`
- 找架构规则 → `references/architecture-rules.json`

**文档维护**:
- 查看维护清单 → [`MAINTENANCE.md`](./MAINTENANCE.md) ⭐
- 添加新文档 → 更新对应 `index.md`
- 清理过时文档 → 运行 `npm run harness:gc`
- 季度审查 → 参考 MAINTENANCE.md 中的审查清单

---

## 🎯 文档组织原则

### 1. 渐进式披露 (Progressive Disclosure)

```
Level 1: AGENTS.md (根目录)     ← 导航地图，< 100 行
    ↓
Level 2: src/AGENTS.md          ← 模块规范，具体规则
    ↓
Level 3: docs/*                 ← 详细设计，技术细节
```

**优势**:
- 避免一次性灌入大量上下文
- Agent 按需深入获取信息
- 减少 Token 消耗

### 2. 关注点分离 (Separation of Concerns)

- **技术方案** → `docs/design-docs/`
- **执行计划** → `docs/exec-plans/`
- **产品需求** → `docs/product-specs/`
- **参考资料** → `docs/references/`
- **自动生成** → `docs/generated/`

**优势**:
- 便于 AI 检索特定类型信息
- 分离活跃文档和历史文档
- 支持独立维护和更新

### 3. 可访问性优先 (Accessibility First)

- 每个目录必有 `index.md` 索引
- 关键文档提供快速定位链接
- 使用标准化命名便于搜索

**优势**:
- AI Agent 可以快速找到所需文档
- 减少迷路和重复查询
- 提高上下文检索效率

---

## 🔄 文档生命周期

### 创建流程

```
graph LR
    A[新需求] --> B{文档类型？}
    B -->|技术方案 | C[design-docs/]
    B -->|执行计划 | D[exec-plans/active/]
    B -->|产品需求 | E[product-specs/]
    C --> F[添加到 index.md]
    D --> G[每周更新进度]
    E --> H[关联相关设计文档]
```

### 维护机制

#### 定期审查
- **每周**: 更新执行计划进度
- **每月**: 审查技术债务
- **每季度**: 清理过时文档 (harness:gc) ⭐

#### 新鲜度保证
- 所有文档包含"最后更新日期"
- >90 天未更新的文档自动标记⚠️
- harness:gc 脚本定期扫描过时内容

#### 文档责任人
每个目录应有明确的维护责任人:

| 目录 | 责任人 | 审查频率 |
|------|--------|---------|
| design-docs/ | 技术负责人 | 季度 |
| exec-plans/ | 项目负责人 | 月度 |
| product-specs/ | 产品负责人 | 季度 |
| references/ | 全体团队 | 季度 |

### 清理策略

**立即删除**:
- 被替代的过时内容
- 临时测试文件
- 重复的草稿

**归档到 completed/**:
- 已完成的执行计划
- 历史决策记录
- 版本发布总结

**保留为最佳实践**:
- 通用解决方案
- 架构决策记录
- 技术难点突破

---

## 🛠️ 使用指南

### 对于 AI Agent

1. **从根目录开始**: 先阅读 [`AGENTS.md`](../AGENTS.md)
2. **按需深入**: 根据任务类型选择对应目录
3. **查看索引**: 每个目录的 `index.md` 提供完整导航
4. **更新进度**: 完成任务后更新 `docs/exec-plans/active/`

### 对于人类开发者

1. **理解架构**: 阅读 [`ARCHITECTURE.md`](../ARCHITECTURE.md)
2. **遵循规范**: 查看 `src/AGENTS.md` 或 `src-tauri/AGENTS.md`
3. **运行测试**: 使用 [`docs/testing/README.md`](./testing/README.md)
4. **追踪进度**: 查看 `docs/exec-plans/active/`

---

## 📝 文档模板

### 设计文档模板

```
# {标题}

> **状态**: 草案 / 审查中 / 已采纳  
> **日期**: YYYY-MM-DD

## 背景
[问题描述和动机]

## 方案
[技术方案详细说明]

## 权衡
[优缺点分析、备选方案]

## 影响范围
[受影响的模块和文件]

## 实施计划
- [ ] 任务 1
- [ ] 任务 2

## 参考资源
[相关链接和文档]
```

### 执行计划模板

```
# 执行计划：{计划名称}

> **优先级**: P0-P3  
> **状态**: 🔄 进行中 / ✅ 完成 / ❌ 取消

## 目标
- 目标 1
- 目标 2

## 任务列表
- [ ] 任务 1
- [ ] 任务 2

## 决策日志
### YYYY-MM-DD
- **决策**: ...
- **原因**: ...

## 进展追踪
[统计数据、里程碑]
```

---

## 🚀 持续改进

基于 OpenAI Harness Engineering 最佳实践，我们持续优化文档结构:

- ✅ **Phase 1**: 文档结构重组 (已完成)
- ✅ **Phase 2**: 精简文档内容 (已完成)
- 🔄 **Phase 3**: 架构护栏增强 (进行中)
- 📋 **Phase 4**: 反馈回路优化 (规划中)

---

## 🔗 相关资源

### Harness 工具脚本

所有自动化脚本位于 [`scripts/`](../scripts/):

```bash
# 架构健康检查
npm run harness:check

# 垃圾回收 (清理过时文档)
npm run harness:gc

# 代码质量修复
npm run harness:fix
```

### 架构约束规则

架构规则配置位于 [`references/architecture-rules.json`](./references/architecture-rules.json),定义前后端架构约束规则。

---

**维护者**: OPC-HARNESS Team  
**贡献指南**: 提交 PR 前请运行 `npm run harness:check` 验证

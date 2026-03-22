# Harness Engineering 完整文档索引

欢迎使用 Harness Engineering！这是所有文档的总索引。

## 📚 文档导航

### 🎯 新手入门（按顺序阅读）

1. **[QUICKSTART.md](./QUICKSTART.md)** ⭐️ **从这里开始！**
   - 30 秒了解 Harness Engineering
   - 3 分钟快速开始
   - 常用命令速查
   - 典型使用场景

2. **[README.md](./README.md)** 📘 **详细使用指南**
   - 什么是 Harness Engineering
   - 四大支柱详解
   - 实际应用场景
   - 高级用法和定制

3. **[AGENTS.md](../AGENTS.md)** 🤖 **AI Agent 必读**
   - 项目结构导航
   - 可用工具清单
   - 架构约束规则
   - 最佳实践指引

### 🏗️ 核心规范

4. **[constraints/architecture-rules.md](./constraints/architecture-rules.md)** 📐
   - 分层架构规则
   - 依赖管理约束
   - 性能限制标准
   - 代码规范要求
   - 测试覆盖率要求

5. **[context-engineering/knowledge-base/best-practices.md](./context-engineering/knowledge-base/best-practices.md)** 💡
   - AI 协作最佳实践
   - TypeScript 空值安全
   - Rust 错误处理
   - 调试技巧
   - 性能优化方法

### 📝 决策与日志

6. **[context-engineering/decision-records/](./context-engineering/decision-records/)** 📋
   - [ADR-001](./context-engineering/decision-records/adr-001-typescript-strict-mode.md) - TypeScript 严格模式
   - [ADR Template](./context-engineering/decision-records/adr-template.md) - ADR 模板

7. **[context-engineering/execution-logs/log-template.md](./context-engineering/execution-logs/log-template.md)** 📊
   - 前端日志规范
   - 后端日志规范
   - 错误处理日志
   - 性能分析日志

### 🔧 工具脚本

8. **[scripts/harness-check.ps1](./scripts/harness-check.ps1)** 🔍
   - 架构健康检查
   - 评分系统说明
   - 自动修复功能
   - CI/CD集成

9. **[scripts/harness-gc.ps1](./scripts/harness-gc.ps1)** 🗑️
   - 垃圾回收清理
   - 临时文件清理
   - 构建产物清理
   - 技术债务扫描

### 📖 总结文档

10. **[IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md)** 📊
    - 实施目标与成果
    - 定量效果分析
    - 技术实现细节
    - 后续优化方向

---

## 🗺️ 快速查找表

### 我想知道...

| 问题 | 查看文档 | 章节 |
|------|----------|------|
| Harness Engineering 是什么？ | [QUICKSTART.md](./QUICKSTART.md) | 第 1 节 |
| 如何快速开始？ | [QUICKSTART.md](./QUICKSTART.md) | 第 2 节 |
| 有哪些可用命令？ | [QUICKSTART.md](./QUICKSTART.md) | 第 3 节 |
| AI 如何理解项目？ | [AGENTS.md](../AGENTS.md) | 全文 |
| 架构约束有哪些？ | [constraints/architecture-rules.md](./constraints/architecture-rules.md) | 全文 |
| 如何编写 ADR？ | [decision-records/adr-template.md](./context-engineering/decision-records/adr-template.md) | 全文 |
| 最佳实践是什么？ | [knowledge-base/best-practices.md](./context-engineering/knowledge-base/best-practices.md) | 全文 |
| 如何运行健康检查？ | [README.md](./README.md) | 快速开始 |
| 如何清理技术债务？ | [README.md](./README.md) | 垃圾回收 |
| 实施效果如何？ | [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md) | 实施效果 |

### 我要做...

| 任务 | 查看文档 | 相关命令 |
|------|----------|----------|
| 开发新功能 | [best-practices.md](./context-engineering/knowledge-base/best-practices.md) | `npm run harness:check` |
| AI 辅助编程 | [AGENTS.md](../AGENTS.md) + [best-practices.md](./context-engineering/knowledge-base/best-practices.md) | - |
| 提交前检查 | [QUICKSTART.md](./QUICKSTART.md) | `npm run harness:check` |
| 清理项目 | [README.md](./README.md) | `npm run harness:gc` |
| 记录架构决策 | [adr-template.md](./context-engineering/decision-records/adr-template.md) | - |
| 调试问题 | [log-template.md](./context-engineering/execution-logs/log-template.md) | - |
| 自定义规则 | [README.md](./README.md) | 编辑 scripts/*.ps1 |

---

## 📂 完整目录结构

```
.harness/
│
├── 📄 README.md                           # 详细使用指南
├── 📄 QUICKSTART.md                       # 快速入门（3 分钟）
├── 📄 IMPLEMENTATION_SUMMARY.md           # 实施总结
│
├── 📁 constraints/                        # 架构约束
│   └── architecture-rules.md              # 架构规则详解
│
├── 📁 context-engineering/                # 上下文工程
│   ├── 📁 decision-records/               # 架构决策记录
│   │   ├── adr-template.md                # ADR 模板
│   │   └── adr-001-typescript-strict-mode.md
│   │
│   ├── 📁 execution-logs/                 # 执行日志
│   │   └── log-template.md                # 日志模板
│   │
│   └── 📁 knowledge-base/                 # 知识库
│       └── best-practices.md              # 最佳实践
│
├── 📁 scripts/                            # 自动化脚本
│   ├── harness-check.ps1                  # 健康检查脚本
│   └── harness-gc.ps1                     # 垃圾回收脚本
│
└── 📁 architecture-guardrails/            # (预留扩展)
└── 📁 feedback-loops/                     # (预留扩展)
```

---

## 🎯 推荐阅读路径

### 路径 1: 人类开发者快速上手

```
QUICKSTART.md (10 分钟)
    ↓
README.md 第 1-3 节 (20 分钟)
    ↓
constraints/architecture-rules.md (30 分钟)
    ↓
knowledge-base/best-practices.md (浏览，20 分钟)
    ↓
运行 harness:check 实践 (5 分钟)
```

**总耗时**: ~85 分钟

### 路径 2: AI Agent 协作流程

```
AGENTS.md (AI 阅读，5 分钟)
    ↓
knowledge-base/best-practices.md (AI 学习，10 分钟)
    ↓
接受具体任务
    ↓
参考 architecture-rules.md (按需查阅)
    ↓
生成代码 → harness:check 验证
```

**总耗时**: ~15 分钟（AI 处理时间）

### 路径 3: 项目维护者深度掌握

```
IMPLEMENTATION_SUMMARY.md (30 分钟)
    ↓
README.md 全文 (60 分钟)
    ↓
所有 ADRs (30 分钟)
    ↓
scripts/*.ps1 源码分析 (60 分钟)
    ↓
自定义扩展实践
```

**总耗时**: ~3 小时

---

## 🔗 外部资源

### 核心理念
- [OpenAI Harness Engineering](https://openai.com/zh-Hans-CN/index/harness-engineering/)
- [Context Engineering](https://www.contextengineering.io/)
- [Architecture Decision Records](https://adr.github.io/)

### 技术栈文档
- [Tauri v2](https://v2.tauri.app/)
- [React + TypeScript](https://react.dev/)
- [Rust Guidelines](https://rust-lang.github.io/api-guidelines/)

### 工具链
- [ESLint](https://eslint.org/)
- [Prettier](https://prettier.io/)
- [TypeScript](https://www.typescriptlang.org/)

---

## 📞 获取帮助

### 文档相关问题
1. 首先查看对应文档
2. 在文档中搜索关键词
3. 查看常见问题部分

### 技术问题
1. 运行 `npm run harness:check` 诊断
2. 查看错误日志
3. 联系项目维护者

### 贡献文档
欢迎提交 PR 改进文档！
- 修正错别字或错误信息
- 补充缺失的最佳实践
- 添加新的使用场景
- 优化文档结构

---

## 📊 文档状态

| 文档 | 状态 | 最后更新 | 版本 |
|------|------|----------|------|
| QUICKSTART.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| README.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| IMPLEMENTATION_SUMMARY.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| AGENTS.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| architecture-rules.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| best-practices.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| adr-template.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| adr-001-typescript-strict-mode.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| log-template.md | ✅ 完成 | 2026-03-22 | 1.0.0 |
| harness-check.ps1 | ✅ 完成 | 2026-03-22 | 1.0.0 |
| harness-gc.ps1 | ✅ 完成 | 2026-03-22 | 1.0.0 |

---

## 🎉 开始你的 Harness 之旅

选择适合你的起点：

- 🚀 **想快速开始？** → [QUICKSTART.md](./QUICKSTART.md)
- 📖 **想深入了解？** → [README.md](./README.md)
- 🤖 **AI 要协助开发？** → [AGENTS.md](../AGENTS.md)
- 🛠️ **想自定义规则？** → 编辑 [scripts/](./scripts/) 中的脚本

---

**Happy Coding with Harness! 🚀**

---

**维护者**: OPC-HARNESS Team  
**版本**: 1.0.0  
**最后更新**: 2026-03-22  
**许可**: MIT License

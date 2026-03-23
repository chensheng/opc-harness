# Harness Engineering 快速入门

> **30 秒了解 Harness Engineering**  
> 最后更新：2026-03-23

## 🎯 什么是 Harness Engineering？

Harness Engineering 是一套让 AI Agent 更好地协助你开发项目的工程实践体系。

### 核心价值
- 🤖 **AI 友好**: 让 AI 快速理解你的项目
- ✅ **质量保障**: 自动检查代码质量
- 📚 **知识沉淀**: 积累最佳实践和决策记录
- 🗑️ **自动清理**: 定期清理技术债务

---

## 🚀 3 分钟快速开始

### 第一步：阅读导航地图

```bash
# AI Agent 必读
cat AGENTS.md
```

### 第二步：运行健康检查

```bash
# 一键检查项目健康度（提交前必跑）
npm run harness:check
```

**输出示例**:
```
========================================
  OPC-HARNESS Architecture Health Check
========================================

[1/6] TypeScript Type Checking...
  [PASS] TypeScript type checking passed
[2/6] ESLint Code Quality Check...
  [PASS] ESLint check passed
...

🎉 Health Score: 95/100
Status: Excellent ✨
```

### 第三步：学习最佳实践

```bash
# 学习如何写出符合项目规范的代码
cat docs/references/best-practices.md
```

---

## 🛠️ 常用命令

### 日常开发

```bash
# 提交前检查（强烈推荐）
npm run harness:check

# 自动修复代码规范问题
npm run harness:fix

# 格式化代码
npm run format

# 运行单元测试
npm run test:unit
```

### 定期维护

```bash
# 完整检查（包含文档和死代码检测）
npm run harness:check -- -All

# 清理临时文件和构建产物
npm run harness:gc
```

---

## 💡 典型使用场景

### 场景 1: 开发新功能

**步骤**:
1. 阅读 [`AGENTS.md`](./AGENTS.md) 了解项目结构
2. 查看 [`best-practices.md`](./best-practices.md) 了解编码规范
3. 参考 [`architecture-rules.json`](./architecture-rules.json) 确保不违规
4. 开发完成后运行 `npm run harness:check` 验证质量

### 场景 2: AI 辅助编程

**步骤**:
1. AI 先阅读 [`AGENTS.md`](./AGENTS.md) 了解项目结构
2. 向 AI 提供清晰的上下文（参考 best-practices.md）
3. AI生成代码后，运行 `npm run harness:check` 验证

### 场景 3: 项目维护

**每周五下午**:
```bash
# 1. 运行健康检查
npm run harness:check

# 2. 清理技术债务
npm run harness:gc

# 3. 查看评分趋势
# （未来功能：生成健康度报告）
```

---

## 📊 健康度评分说明

### 评分标准（总分 100）

| 检查项 | 分值 | 说明 |
|--------|------|------|
| TypeScript 类型检查 | 20 | 编译是否通过 |
| ESLint 代码质量 | 15 | 代码风格是否一致 |
| Prettier 格式化 | 10 | 格式是否统一 |
| Rust 编译检查 | 25 | 后端代码是否正确 |
| 单元测试覆盖率 | 20 | >= 70% |
| 架构约束 | 10 | 无违规依赖 |

### 评分等级

- **90-100**: 优秀 ✨ - 可以安全合并
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议合并

---

## ❓ 常见问题

### Q: Harness Engineering 是什么？

A: 一套为 AI 协作优化的工程实践体系，通过构建受控环境让 AI 能够可靠地完成编码任务。

### Q: 为什么需要这个？

A: 
- 🤖 AI生成的代码质量参差不齐
- 📋 团队成员 coding style 不一致
- 🗂️ 项目结构容易混乱
- 📉 技术债务难以发现和管理

### Q: 如何使用？

A: 
1. AI 先读 [`AGENTS.md`](./AGENTS.md)
2. 开发时参考 [`best-practices.md`](./best-practices.md)
3. 提交前运行 `npm run harness:check`
4. 定期运行 `npm run harness:gc` 清理

### Q: 可以自定义吗？

A: 当然可以！
- 编辑 `docs/references/architecture-rules.json` 添加自定义规则
- 在 `docs/design-docs/` 添加新的架构决策记录
- 创建新的最佳实践文档

---

## 🎓 学习路径

### 新手入门（1 小时）

1. ✅ 阅读 [`AGENTS.md`](./AGENTS.md) - 10 分钟
2. ✅ 浏览 [`best-practices.md`](./best-practices.md) - 20 分钟
3. ✅ 运行 `npm run harness:check` 并理解输出 - 10 分钟
4. ✅ 阅读 [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 20 分钟

### 进阶提升（1 天）

1. 📖 精读 [`ARCHITECTURE.md`](./ARCHITECTURE.md)
2. 📝 学习所有架构决策记录
3. 🔧 尝试自定义检查规则
4. 📚 贡献新的最佳实践

---

## 🔗 相关资源

### 官方文档
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [本项目导航地图](../AGENTS.md)
- [最佳实践](./best-practices.md)
- [架构设计](../ARCHITECTURE.md)

### 工具链
- [ESLint - 代码规范检查](https://eslint.org/)
- [Prettier - 代码格式化](https://prettier.io/)
- [cargo - Rust 包管理](https://doc.rust-lang.org/cargo/)
- [Vitest - 单元测试框架](https://vitest.dev/)

---

**Happy Coding! 🚀**

---

**维护者**: OPC-HARNESS Team  
**版本**: 2.0.0 (基于 OpenAI Harness Engineering 最佳实践重构)  
**最后更新**: 2026-03-23  
**许可**: MIT License

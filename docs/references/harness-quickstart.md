# Harness Engineering 快速入门指南

## 🎯 30 秒快速了解

**Harness Engineering** 是一套让 AI Agent 更好地协助你开发项目的工程实践体系。

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

# 人类开发者阅读
cat .harness/README.md
```

### 第二步：运行健康检查

```bash
# 一键检查项目健康度
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

### 第三步：查看最佳实践

```bash
# 学习如何写出符合项目规范的代码
cat .harness/context-engineering/knowledge-base/best-practices.md
```

---

## 📁 目录结构速览

```
.harness/
├── README.md                    # 📘 使用指南（从这里开始）
├── constraints/
│   └── architecture-rules.md    # 📐 架构规则和约束
├── context-engineering/
│   ├── decision-records/        # 📝 重要决策记录 (ADRs)
│   ├── execution-logs/          # 📊 执行日志模板
│   └── knowledge-base/          # 📚 知识库和最佳实践
└── scripts/
    ├── harness-check.ps1        # 🔍 健康检查脚本
    └── harness-gc.ps1           # 🗑️ 垃圾回收脚本
```

---

## 🛠️ 常用命令

### 日常开发

```bash
# 提交前检查（强烈推荐）
npm run harness:check

# 自动修复代码规范问题
npm run lint:fix

# 格式化代码
npm run format
```

### 定期维护

```bash
# 清理临时文件和构建产物
npm run harness:gc

# 预览将删除什么（安全模式）
npm run harness:gc:dry-run

# 强制清理（不询问）
npm run harness:gc -- -Force
```

---

## 💡 典型使用场景

### 场景 1: 开发新功能

**步骤**:
1. 查看 [最佳实践](./.harness/context-engineering/knowledge-base/best-practices.md) 了解编码规范
2. 参考 [架构约束](./.harness/constraints/architecture-rules.md) 确保不违规
3. 开发完成后运行 `npm run harness:check` 验证质量

### 场景 2: AI 辅助编程

**步骤**:
1. AI 先阅读 [AGENTS.md](./AGENTS.md) 了解项目结构
2. 向 AI 提供清晰的上下文（参考 ADRs 和最佳实践）
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
| ESLint 代码规范 | 15 | 代码风格是否一致 |
| Prettier 格式化 | 10 | 格式是否统一 |
| Rust 编译检查 | 25 | 后端代码是否正确 |
| 依赖完整性 | 5 | 依赖文件是否完整 |
| 目录结构 | 5 | 必要目录是否存在 |
| 基础分 | 20 | - |

### 评分等级

- **90-100**: 优秀 ✨ - 可以安全部署
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议部署

---

## 🎓 学习路径

### 新手入门（1 小时）

1. ✅ 阅读 [AGENTS.md](./AGENTS.md) - 10 分钟
2. ✅ 浏览 [.harness/README.md](./.harness/README.md) - 20 分钟
3. ✅ 运行 `npm run harness:check` 并理解输出 - 10 分钟
4. ✅ 阅读 [最佳实践](./.harness/context-engineering/knowledge-base/best-practices.md) - 20 分钟

### 进阶提升（1 天）

1. 📖 精读 [架构约束](./.harness/constraints/architecture-rules.md)
2. 📝 学习所有 [ADRs](./.harness/context-engineering/decision-records/)
3. 🔧 尝试自定义检查规则
4. 📚 贡献新的最佳实践

### 专家级别（1 周）

1. 🏗️ 深入理解 Harness Engineering 理念
2. 🤖 优化 AI 协作流程
3. 📊 建立团队的质量文化
4. 🌟 向社区分享经验

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
1. AI 先读 [AGENTS.md](./AGENTS.md)
2. 开发时参考 [最佳实践](./.harness/context-engineering/knowledge-base/best-practices.md)
3. 提交前运行 `npm run harness:check`
4. 定期运行 `npm run harness:gc` 清理

### Q: 可以自定义吗？

A: 当然可以！
- 编辑 `.harness/scripts/harness-check.ps1` 添加自定义检查
- 在 `.harness/constraints/` 添加自己的规则
- 创建新的 ADR 记录重要决策

### Q: 如何在 CI/CD 中使用？

A: 
```yaml
# GitHub Actions 示例
- name: Harness Check
  run: npm run harness:check
  
# 失败时阻止合并
if: ${{ success() && steps.harness-check.outputs.score >= 80 }}
```

---

## 🔗 相关资源

### 官方文档
- [OpenAI Harness Engineering](https://openai.com/zh-Hans-CN/index/harness-engineering/)
- [本项目详细文档](./.harness/README.md)
- [AGENTS.md 导航地图](./AGENTS.md)

### 学习材料
- [架构决策记录 (ADR) 指南](https://adr.github.io/)
- [TypeScript 严格模式](https://www.typescriptlang.org/tsconfig#strict)
- [Rust 编码规范](https://rust-lang.github.io/api-guidelines/)

### 工具链
- [ESLint - 代码规范检查](https://eslint.org/)
- [Prettier - 代码格式化](https://prettier.io/)
- [cargo - Rust 包管理](https://doc.rust-lang.org/cargo/)

---

## 🎉 下一步

现在你已经掌握了基础知识，接下来：

1. **立即实践**: 运行 `npm run harness:check` 看看项目健康状况
2. **深入学习**: 阅读 [详细使用指南](./.harness/README.md)
3. **分享给团队**: 拉上小伙伴一起提高代码质量
4. **持续改进**: 根据项目需求定制 Harness 规则

---

**Happy Coding! 🚀**

---

**维护者**: OPC-HARNESS Team  
**版本**: 1.0.0  
**最后更新**: 2026-03-22  
**许可**: MIT License

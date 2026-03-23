# AI Agent 导航地图

> **Harness Engineering 核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)
> 
> **适用范围**: 本项目所有 AI Agent 和开发者  
> **最后更新**: 2026-03-23

## 🎯 快速定位

### 项目概述
**OPC-HARNESS** - AI 驱动的桌面应用 MVP
- **技术栈**: React 18 + TypeScript 5 + Tauri v2 + Rust + SQLite
- **核心功能**: Vibe Design → Vibe Coding → Vibe Marketing

### 📍 关键文档（按优先级）

#### 1️⃣ 必读入口
- [`src/AGENTS.md`](./src/AGENTS.md) - 前端开发规范
- [`src-tauri/AGENTS.md`](./src-tauri/AGENTS.md) - Rust 后端规范

#### 2️⃣ 架构与约束
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) - 系统架构设计
- [`docs/references/architecture-rules.json`](./docs/references/architecture-rules.json) - 架构约束规则

#### 3️⃣ 测试与验证
- 单元测试：`npm run test:unit` - 运行所有单元测试 ⭐
- E2E 测试：`npm run test:e2e` - E2E 测试（智能运行，自动管理服务器）⭐
- 架构检查：`npm run harness:check` - 架构健康检查

#### 4️⃣ 最佳实践
- [`docs/references/best-practices.md`](./docs/references/best-practices.md) - 编码最佳实践

---

## 🏗️ Harness Engineering 三大支柱

### 1. 上下文工程 (Context Engineering)

**目的**: 帮助 AI Agent 快速理解项目背景和任务

```
渐进式披露层级:
AGENTS.md (导航) → 模块 AGENTS.md (规范) → docs/ (详细设计)
```

**关键文件**:
- 本文件 - 导航地图
- `docs/design-docs/` - 技术决策记录
- `docs/exec-plans/` - 执行计划和进度
- `docs/MAINTENANCE.md` - 文档维护清单 ⭐

### 2. 架构约束 (Architectural Constraints)

**目的**: 确保 AI 生成的代码符合项目规范

**强制执行方式**:
- ESLint + TypeScript - 前端代码规范
- cargo clippy - Rust 代码规范
- 自定义架构规则 - 防止循环依赖和架构漂移

**核心约束**:
```typescript
// ✅ 允许的数据流
Component → Store → Commands → Services → DB

// ❌ 禁止的依赖
Store → Component  // 状态层不可依赖 UI 层
Services → Commands // 服务层不可依赖命令层
```

### 3. 反馈回路 (Feedback Loops)

**目的**: 快速发现问题并持续改进

**自动化检查**:
```bash
# 提交前必跑
npm run harness:check          # 架构健康检查

# 完整验证（包含文档和死代码）
npm run harness:check -- -All
```

**质量门禁**:
- TypeScript 编译通过
- ESLint 无错误
- Rust cargo check 通过
- 单元测试覆盖率 >= 70%

---

## 🚀 快速开始

### 对于 AI Agent

1. **阅读本文件** - 了解项目结构和文档位置
2. **阅读对应模块的 AGENTS.md** - 前端或后端规范
3. **遵循架构约束** - 参考 `architecture-rules.json`
4. **提交前验证** - 运行 `npm run harness:check`

### 对于人类开发者

1. **理解架构** - 阅读 [`ARCHITECTURE.md`](./ARCHITECTURE.md)
2. **遵循规范** - 查看 `src/AGENTS.md` 或 `src-tauri/AGENTS.md`
3. **运行测试** - 使用 `npm run test:unit` 和 `npm run test:e2e`
4. **追踪进度** - 查看 `docs/exec-plans/active/`

---

## 📁 文档组织原则

### 渐进式披露

```
Level 1: AGENTS.md (本文件)     ← 导航地图，< 100 行
    ↓
Level 2: src/AGENTS.md          ← 模块规范，具体规则  
    ↓
Level 3: docs/*                 ← 详细设计，技术细节
```

### 关注点分离

- **技术方案** → `docs/design-docs/`
- **执行计划** → `docs/exec-plans/`
- **产品需求** → `docs/product-specs/`
- **参考资料** → `docs/references/`
- **自动生成** → `docs/generated/`

---

## 🔧 常用命令

### 日常开发
```bash
# 架构健康检查（提交前必跑）
npm run harness:check

# 自动修复代码规范问题
npm run harness:fix

# 运行单元测试
npm run test:unit

# E2E 测试（智能运行，自动管理服务器）
npm run test:e2e
```

### 定期维护
```bash
# 完整检查（包含文档和死代码）
npm run harness:check -- -All

# 清理临时文件和构建产物
npm run harness:gc
```

---

## 📊 质量门禁标准

| 检查项 | 满分 | 通过标准 |
|--------|------|---------|
| TypeScript 类型检查 | 20 | 编译通过 |
| ESLint 代码规范 | 15 | 无错误 |
| Prettier 格式化 | 10 | 格式统一 |
| Rust 编译检查 | 25 | cargo check 通过 |
| 单元测试覆盖率 | 20 | >= 70% |
| 架构约束 | 10 | 无违规 |

**评分等级**:
- **90-100**: 优秀 ✨ - 可以安全合并
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议合并

---

## 🎓 学习路径

### 新手入门（1 小时）
1. ✅ 阅读本文件 - 10 分钟
2. ✅ 浏览对应模块的 AGENTS.md - 20 分钟
3. ✅ 运行 `npm run harness:check` 并理解输出 - 10 分钟
4. ✅ 阅读最佳实践 - 20 分钟

### 进阶提升（1 天）
1. 📖 精读 [`ARCHITECTURE.md`](./ARCHITECTURE.md)
2. 📝 学习所有架构决策记录
3. 🔧 尝试自定义检查规则
4. 📚 贡献新的最佳实践

---

## ❓ 常见问题

### Q: Harness Engineering 是什么？
A: 一套为 AI 协作优化的工程实践体系，通过构建受控环境让 AI 能够可靠地完成编码任务。

### Q: 为什么需要这个？
A: 
- 🤖 AI 生成的代码质量参差不齐
- 📋 团队成员 coding style 不一致
- 🗂️ 项目结构容易混乱
- 📉 技术债务难以发现和管理

### Q: 如何向 AI 提问？
A: 参考 [`docs/references/best-practices.md`](./docs/references/best-practices.md) 中的"如何向 AI 提问"章节。

### Q: 可以自定义规则吗？
A: 可以！编辑 `docs/references/architecture-rules.json` 添加自定义规则。

---

## 🔗 相关资源

### 官方文档
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [Tauri v2 官方文档](https://v2.tauri.app/)
- [React 官方文档](https://react.dev/)
- [TypeScript 手册](https://www.typescriptlang.org/docs/)

### 工具链
- [ESLint - 代码规范检查](https://eslint.org/)
- [Prettier - 代码格式化](https://prettier.io/)
- [cargo - Rust 包管理](https://doc.rust-lang.org/cargo/)
- [Vitest - 单元测试框架](https://vitest.dev/)

---

**维护者**: OPC-HARNESS Team  
**版本**: 2.0.0 (基于 OpenAI Harness Engineering 最佳实践重构)  
**最后更新**: 2026-03-23
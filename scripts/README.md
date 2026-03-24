# Harness Engineering 使用指南

> **详细说明**  
> 最后更新：2026-03-23

## 🎯 什么是 Harness Engineering？

Harness Engineering 是一套为 AI Agent 协作优化的工程实践体系，通过构建受控环境让 AI 能够可靠地完成编码任务。

**核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)

---

## 📦 Harness 的三大支柱

### 1. 上下文工程 (Context Engineering)

**目的**: 帮助 AI Agent 快速理解项目背景和任务

**组成**:
- **渐进式披露**: AGENTS.md → 模块规范 → docs/详细设计
- **决策记录**: 重要架构决策的背景和原因
- **知识库**: 最佳实践和经验教训

**关键文件**:
```
AGENTS.md                          # 导航地图（必读）
src/AGENTS.md                      # 前端规范
src-tauri/AGENTS.md                # Rust 规范
docs/design-docs/                  # 技术决策
docs/exec-plans/                   # 执行计划
docs/references/best-practices.md  # 最佳实践
```

### 2. 架构约束 (Architectural Constraints)

**目的**: 确保 AI生成的代码符合项目规范

**强制执行方式**:
- ESLint + TypeScript - 前端代码规范
- cargo clippy - Rust 代码规范
- 自定义架构规则 - 防止循环依赖

**核心约束**:
```typescript
// ✅ 允许的数据流
Component → Store → Commands → Services → DB

// ❌ 禁止的依赖
Store → Component    // 状态层不可依赖 UI 层
Services → Commands  // 服务层不可依赖命令层
```

### 3. 反馈回路 (Feedback Loops)

**目的**: 快速发现问题并持续改进

**自动化检查**:
```
# 提交前必跑（包含所有检查项）
npm run harness:check
```

**闭环系统**:
```
AI生成代码 → 运行检查 → 发现问题 → 修复 → 再次检查
     ↓                                           ↑
     └─────────────── 持续集成 ──────────────────┘
```

---

## 🚀 快速开始

### 1. AI Agent 导航

**重要**: AI Agent 在开始工作前必须阅读 [`AGENTS.md`](../AGENTS.md)

### 2. 运行架构健康检查

```
# 基础检查
npm run harness:check

# 输出示例:
# ========================================
#   OPC-HARNESS 架构健康检查
# ========================================
# [1/6] TypeScript 类型检查...
#   ✅ TypeScript 类型检查通过
# [2/6] ESLint 代码规范检查...
#   ✅ ESLint 检查通过
# ...
# 🎉 健康度评分：95/100
```

### 3. 执行垃圾回收

```
# 实际清理（会询问确认）
npm run harness:gc

# 空运行模式（查看将删除什么）
npm run harness:gc -- -DryRun

# 强制清理（不询问）
npm run harness:gc -- -Force
```

---

## 💡 实际应用场景

### 场景 1: AI 辅助开发新功能

**步骤**:

1. **准备上下文**
```bash
# 查看最佳实践
cat docs/references/best-practices.md

# 查看架构规则
cat docs/references/architecture-rules.md
```

2. **向 AI 提问**
```
## 任务：实现用户登录功能

## 上下文
- 已阅读 AGENTS.md 导航地图
- 遵循 best-practices.md 中的错误处理规范
- 参考 architecture-rules.md 中的分层约束

## 约束
- 前端：src/components/auth/Login.tsx
- 后端：src-tauri/src/commands/auth.rs
- 使用 bcrypt 加密
- JWT token 有效期 7 天
- 错误信息用中文

请生成代码并说明如何验证
```

3. **验证代码**
```bash
# 运行健康检查
npm run harness:check

# 运行单元测试
npm run test:unit
```

### 场景 2: 重构现有代码

**步骤**:

1. **记录决策**
```bash
# 创建新的架构决策记录
cp docs/design-docs/adr-template.md \
   docs/design-docs/adr-xxx-refactoring.md
```

2. **执行重构**
```bash
# 重构前先备份
git checkout -b feature/refactoring-backup
```

3. **验证重构结果**
```bash
# 确保健康度不下降
npm run harness:check
```

### 评分计算

| 检查项 | 满分 | 扣分标准 |
|--------|------|---------|
| TypeScript 类型检查 | 20 | 失败扣 20 分 |
| ESLint 代码规范 | 15 | 警告扣 5 分，错误扣 15 分 |
| Prettier 格式化 | 10 | 不通过扣 10 分 |
| Rust 编译检查 | 25 | 失败扣 25 分 |
| Rust 单元测试 | 20 | 失败扣 20 分 |
| TS 单元测试 | 20 | 失败扣 20 分 |
| 依赖完整性 | 5 | 缺失扣 5 分 |
| 目录结构 | 5 | 缺失扣 5 分 |
| 文档结构验证 | 10 | 缺失/过期扣 10 分 |

**总分**: **110 分**（9 项检查）

### 评分等级

- **90-100**: 优秀 ✨ - 可以安全合并（允许少量非关键问题）
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议合并

**注意**: 默认执行所有 9 项检查，总分 110 分。如果存在 Error 级别的问题，脚本将返回非零退出码。

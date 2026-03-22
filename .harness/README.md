# Harness Engineering 使用指南

## 🎯 什么是 Harness Engineering？

Harness Engineering 是一套为 AI Agent 协作优化的工程实践体系，通过构建受控环境让 AI 能够可靠地完成编码任务。

**核心理念**: "人类掌舵，Agent 执行" (Humans steer. Agents execute.)

## 📦 OPC-HARNESS 的 Harness 组成

```
AGENTS.md                          # AI Agent 导航地图（必读）
.harness/
├── constraints/                       # 架构约束规则
│   └── architecture-rules.md          # 详细的架构规范
├── context-engineering/               # 上下文工程数据
│   ├── decision-records/              # 架构决策记录 (ADRs)
│   ├── execution-logs/                # 执行日志和模板
│   └── knowledge-base/                # 知识库和最佳实践
└── scripts/                           # 自动化脚本
    ├── harness-check.ps1              # 架构健康检查
    └── harness-gc.ps1                 # 垃圾回收
```

## 🚀 快速开始

### 1. AI Agent 导航

**重要**: AI Agent 在开始工作前必须阅读 [`AGENTS.md`](../AGENTS.md)

该文档包含:
- 📍 项目结构快速定位
- 🛠️ 可用工具和命令
- 🏗️ 架构约束和依赖规则
- 🔄 反馈回路机制
- 🗑️ 垃圾回收策略

### 2. 运行架构健康检查

```bash
# 完整检查
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

```bash
# 实际清理（会询问确认）
npm run harness:gc

# 空运行模式（查看将删除什么）
npm run harness:gc:dry-run

# 强制清理（不询问）
npm run harness:gc -- -Force
```

## 📖 核心概念详解

### 四大支柱

#### 1. 上下文工程 (Context Engineering)

**目的**: 帮助 AI Agent 快速理解项目和任务背景

**组成**:
- **决策记录 (ADRs)**: 记录重要架构决策的背景和原因
- **执行日志**: 记录关键操作的详细过程
- **知识库**: 积累最佳实践和经验教训

**使用方法**:
```markdown
// 在向 AI 提问时提供上下文
## 上下文
参考 ADR-001: 我们启用了 TypeScript 严格模式
参考最佳实践：TypeScript 空值安全处理规范

## 任务
实现一个函数，要求符合项目的类型安全标准
```

#### 2. 架构约束 (Architectural Constraints)

**目的**: 确保 AI 生成的代码符合项目规范

**主要约束**:
- 分层架构规则（前后端分离）
- 依赖管理规则（禁止循环依赖）
- 性能限制（响应时间、内存使用）
- 代码规范（ESLint、Prettier、Rust 风格）

**验证方法**:
```bash
# 自动验证所有约束
npm run harness:check

# 手动查看约束文档
cat .harness/constraints/architecture-rules.md
```

#### 3. 反馈回路 (Feedback Loops)

**目的**: 快速发现问题并持续改进

**闭环系统**:
```
AI 生成代码 → 运行检查 → 发现问题 → 修复 → 再次检查
     ↓                                           ↑
     └─────────────── 持续集成 ──────────────────┘
```

**反馈渠道**:
1. **即时反馈**: `harness:check` (开发阶段)
2. **提交前反馈**: lint-staged + Husky (Git提交)
3. **持续反馈**: CI/CD流水线 (自动化测试)

#### 4. 垃圾回收 (Garbage Collection)

**目的**: 对抗熵增，保持项目整洁

**自动清理**:
- >30 天未更新的临时文档
- 未使用的导入和死代码
- 过时的模拟数据
- 构建产物

**手动触发**:
```bash
npm run harness:gc
```

## 💡 实际应用场景

### 场景 1: AI 辅助开发新功能

**步骤**:

1. **准备上下文**
```bash
# 1. 查看相关 ADR
cat .harness/context-engineering/decision-records/adr-001-typescript-strict-mode.md

# 2. 查看最佳实践
cat .harness/context-engineering/knowledge-base/best-practices.md
```

2. **向 AI 提问**
```markdown
## 任务：实现用户登录功能

## 上下文
- 已阅读 AGENTS.md 导航地图
- 遵循 ADR-001 TypeScript 严格模式
- 参考最佳实践中的错误处理规范

## 约束
- 前端：src/components/auth/Login.tsx
- 后端：src-tauri/src/commands/auth.rs
- 使用 bcrypt 加密
- 错误信息用中文

请生成代码并说明如何验证
```

3. **验证代码**
```bash
# 运行健康检查
npm run harness:check

# 如果有问题，根据提示修复
npm run lint:fix
npm run format
```

### 场景 2: 重构现有代码

**步骤**:

1. **记录决策**
```bash
# 创建新的 ADR
cp .harness/context-engineering/decision-records/adr-template.md \
   .harness/context-engineering/decision-records/adr-xxx-refactoring.md
   
# 编辑 ADR 内容，说明重构原因和方案
```

2. **执行重构**
```bash
# 重构前先备份
git checkout -b feature/refactoring-backup

# 运行重构工具（如果有）
# 或者手动修改代码
```

3. **验证重构结果**
```bash
# 确保健康度不下降
npm run harness:check

# 对比重构前后的指标
```

### 场景 3: 调试复杂问题

**步骤**:

1. **启用详细日志**
```typescript
// 在前端添加调试日志
console.group('[DEBUG] 问题排查');
console.log('[STATE]', currentState);
console.log('[PARAMS], params);
// ...
console.groupEnd();
```

2. **记录执行日志**
```bash
# 将日志保存到执行日志目录
echo "2026-03-22 调试记录" >> .harness/context-engineering/execution-logs/debug-2026-03-22.md
```

3. **更新知识库**
```markdown
# 解决问题后，添加到最佳实践
## 常见问题：XXX 错误
**原因**: ...
**解决方案**: ...
```

## 🔧 高级用法

### 自定义检查规则

编辑 `.harness/scripts/harness-check.ps1`:

```powershell
# 添加新的检查项
Write-Host "[7/7] 自定义检查..." -ForegroundColor Yellow
# 你的检查逻辑
if (Test-Path "custom-rule") {
    Write-Host "  ✅ 自定义检查通过" -ForegroundColor Green
} else {
    Write-Host "  ❌ 自定义检查失败" -ForegroundColor Red
    $Score -= 10
}
```

### 扩展知识库

在 `.harness/context-engineering/knowledge-base/` 添加新文档:

```markdown
# 新增主题：性能优化技巧

## 背景
...

## 实践方法
...

## 效果对比
...
```

### 集成到 CI/CD

GitHub Actions 示例:

```yaml
name: Harness Check

on: [push, pull_request]

jobs:
  harness:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Harness Check
        run: npm run harness:check
      
      - name: Upload results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: harness-results
          path: harness-report.json
```

## 📊 健康度评分说明

### 评分计算

| 检查项 | 满分 | 扣分标准 |
|--------|------|---------|
| TypeScript 类型检查 | 20 | 失败扣 20 分 |
| ESLint 代码规范 | 15 | 警告扣 5 分，错误扣 15 分 |
| Prettier 格式化 | 10 | 不通过扣 10 分 |
| Rust 编译检查 | 25 | 失败扣 25 分 |
| 依赖完整性 | 5 | 缺失扣 5 分 |
| 目录结构 | 5 | 不完整扣 5 分 |
| 基础分 | 20 | - |

### 评分等级

- **90-100**: 优秀 ✨ - 可以安全部署
- **70-89**: 良好 👍 - 有一些改进空间
- **<70**: 需要修复 ⚠️ - 不建议部署

## 🤝 团队协作建议

### 1. 建立 Harness 文化

- **每次提交前**: 运行 `npm run harness:check`
- **每周**: 运行 `npm run harness:gc` 清理技术债务
- **每月**: 回顾 ADRs，更新最佳实践

### 2. 知识传承

- **新人入职**: 先读 AGENTS.md 和最佳实践
- **重要决策**: 必须写 ADR
- **解决问题**: 更新到知识库

### 3. 持续改进

- **定期审查**: 检查约束规则是否合理
- **收集反馈**: 团队成员提出改进建议
- **迭代更新**: 每季度更新 Harness 体系

## 📚 相关资源

- [OpenAI Harness Engineering 原文](https://openai.com/zh-Hans-CN/index/harness-engineering/)
- [AGENTS.md 导航地图](../AGENTS.md)
- [架构约束规则](./constraints/architecture-rules.md)
- [最佳实践](./context-engineering/knowledge-base/best-practices.md)

---

**维护者**: OPC-HARNESS Team  
**版本**: 1.0.0  
**最后更新**: 2026-03-22

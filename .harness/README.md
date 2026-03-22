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
├── cli-browser-verify/                # CLI 浏览器验证
│   ├── verify_runner.ps1              # 验证运行器
│   ├── cli_detector.ps1               # CLI 检测
│   ├── tasks/                         # 验证任务
│   │   ├── smoke.yaml                 # 冒烟测试
│   │   └── critical.yaml              # 关键路径测试
│   ├── quick-verify.txt               # 快速验证指令
│   ├── README.md                      # 使用说明
│   └── USAGE.md                       # 详细指南
└── scripts/                           # 自动化脚本
    ├── harness-check.ps1              # 架构健康检查
    ├── harness-gc.ps1                 # 垃圾回收
    ├── harness-quick-verify.ps1       # 快速验证
    └── harness-verify-tauri.ps1       # Tauri 应用验证
```

## 🚀 快速开始

### 1. AI Agent 导航

**重要**: AI Agent 在开始工作前必须阅读 [`AGENTS.md`](../AGENTS.md)

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

### 3. CLI Browser 验证

利用你当前使用的 AI CLI（Kimi / Claude / OpenCode）内置的浏览器能力进行验证，**无需配置 API Key**。

**最简单的方式 - 直接对话**:
```
请帮我验证 http://localhost:1420：
1. 页面是否正常加载 OPC-HARNESS 标题
2. 导航菜单是否包含 Dashboard、Idea、Coding
3. 点击 Idea 是否能进入输入页面
```

**使用浏览器命令**:
```
@browser http://localhost:1420
请告诉我你看到了什么？
```

**自动化脚本**:
```bash
# 运行 CLI 浏览器验证
npm run harness:verify:cli

# 指定测试套件
npm run harness:verify:cli -- -Suite critical
```

### 4. 执行垃圾回收

```bash
# 实际清理（会询问确认）
npm run harness:gc

# 空运行模式（查看将删除什么）
npm run harness:gc:dry

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

#### 2. 架构约束 (Architectural Constraints)

**目的**: 确保 AI 生成的代码符合项目规范

**主要约束**:
- 分层架构规则（前后端分离）
- 依赖管理规则（禁止循环依赖）
- 性能限制（响应时间、内存使用）
- 代码规范（ESLint、Prettier、Rust 风格）

#### 3. 反馈回路 (Feedback Loops)

**目的**: 快速发现问题并持续改进

**闭环系统**:
```
AI 生成代码 → 运行检查 → 发现问题 → 修复 → 再次检查
     ↓                                           ↑
     └─────────────── 持续集成 ──────────────────┘
```

#### 4. 垃圾回收 (Garbage Collection)

**目的**: 对抗熵增，保持项目整洁

## 💡 实际应用场景

### 场景 1: AI 辅助开发新功能

**步骤**:

1. **准备上下文**
```bash
# 查看相关 ADR
cat .harness/context-engineering/decision-records/adr-001-typescript-strict-mode.md

# 查看最佳实践
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

# 启动开发环境
npm run tauri:dev

# 在 Kimi CLI 中验证界面
@browser http://localhost:1420
```

### 场景 2: 重构现有代码

**步骤**:

1. **记录决策**
```bash
# 创建新的 ADR
cp .harness/context-engineering/decision-records/adr-template.md \
   .harness/context-engineering/decision-records/adr-xxx-refactoring.md
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

## 🔧 高级用法

### 自定义检查规则

编辑 `.harness/scripts/harness-check.ps1`:

```powershell
# 添加新的检查项
Write-Host "[7/7] 自定义检查..." -ForegroundColor Yellow
# 你的检查逻辑
```

### 扩展知识库

在 `.harness/context-engineering/knowledge-base/` 添加新文档。

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
- [CLI Browser 使用指南](./cli-browser-verify/USAGE.md)

---

**维护者**: OPC-HARNESS Team  
**版本**: 2.0.0  
**最后更新**: 2026-03-22

# Harness Engineering 快速启动指南

> 5 分钟体验 Harness Engineering 核心功能

## 🚀 快速开始（5 分钟）

### 步骤 1: 安装依赖 (2 分钟)

```bash
npm install
```

这将安装：
- `tsx`: TypeScript 执行环境
- 其他 Harness Engineering 相关依赖

### 步骤 2: 查看帮助 (30 秒)

```bash
npm run harness:help
```

你将看到所有可用的 Harness 命令。

### 步骤 3: 运行架构检查 (1 分钟)

```bash
npm run harness:check
```

**预期输出**:
```
🔍 开始运行架构守护检查...

📋 检查：frontend-backend-separation...
✅ 通过：frontend-backend-separation (0ms)

📋 检查：centralized-state-management...
✅ 通过：centralized-state-management (0ms)

...

============================================================
📊 检查报告
============================================================

总计：8/13 通过
违规总数：0
健康度得分：61.5%

⚠️  检测到性能规则错误（预期行为，这些规则在浏览器端运行）
```

### 步骤 4: 启动应用查看 UI (1 分钟)

```bash
npm run tauri:dev
```

在应用中你会看到：
1. **顶部状态栏**: 显示会话连接状态和知识库版本
2. **架构守护者面板**: 实时显示架构健康度得分
3. **违规列表**: 如果有违规，会显示详细信息和修复建议

## 📖 核心概念速览

### 1. Context Engineering (上下文工程)

**是什么**: 为 AI Agent 提供动态的、可追溯的知识库

**怎么用**:
```typescript
import { useHarness } from '@/hooks/useHarness';

const { logExecution, recordDecision } = useHarness();

// 记录操作
await logExecution({
  action: 'prd.generate',
  status: 'success',
});

// 记录决策
await recordDecision({
  title: '选择 Zustand 作为状态管理',
  summary: 'Zustand 更轻量且 API 简洁',
});
```

### 2. Architectural Constraints (架构约束)

**是什么**: 实时监控代码是否符合架构规则

**规则示例**:
- ✅ 前后端必须物理隔离
- ✅ 禁止循环依赖
- ✅ 启动时间 < 800ms

**违规时会看到**:
```
❌ 违规：前端直接访问数据库
📍 位置：src/components/UserList.tsx:45
💡 修复建议:
   1. 创建 Tauri command: src-tauri/src/commands/user.rs
   2. 在前端通过 invoke() 调用
   3. 参考文档：docs/architecture.md
```

### 3. Feedback Loops (反馈回路)

**是什么**: 自动化测试和修复系统

**配置示例** (`.harness/feedback-loops/closed-loop-tests/config.yaml`):
```yaml
- name: PRD 生成验证
  trigger: prd.generated
  expected:
    sections: ["产品概述", "功能需求", "验收标准"]
  autoFix: true
```

### 4. Garbage Collection (垃圾回收)

**是什么**: 定期清理系统熵增

**运行 GC**:
```bash
npm run harness:gc
```

**检测内容**:
- 📄 过时文档（>30 天未更新）
- 💀 死代码（未使用的函数/类）
- ⚙️ 配置文件健康度

## 🎯 日常工作流

### 开发前
```bash
# 1. 恢复会话（自动）
# 应用启动时自动恢复最近的会话

# 2. 查看知识库
# 阅读 .harness/context-engineering/knowledge-base/
```

### 开发中
```bash
# 随时运行架构检查
npm run harness:check

# 或在 UI 中查看实时健康度
```

### 提交前
```bash
# 1. 运行完整检查
npm run harness:check && npm run harness:gc

# 2. 确保健康度 > 90%
```

### 定期维护
```bash
# 每天凌晨 2 点自动运行 GC（cron）
# 或手动运行
npm run harness:gc
```

## 📁 重要文件位置

```
.harness/
├── AGENTS.md                           # AI Agent 必读导航
├── architecture-guardrails/
│   ├── layer-rules.ts                  # 分层规则
│   ├── dependency-rules.ts             # 依赖规则
│   └── performance-rules.ts            # 性能规则
├── context-engineering/
│   ├── execution-logs/                 # 执行日志目录
│   └── decision-records/               # 决策记录目录
└── feedback-loops/
    ├── closed-loop-tests/config.yaml   # 测试配置
    └── garbage-collection/config.yaml  # GC 配置

src/
├── components/harness/
│   ├── ContextProvider.tsx             # 上下文提供者
│   └── ArchitectureGuardian.tsx        # 架构守护者
├── hooks/useHarness.ts                 # Harness Hook
└── stores/harnessStore.ts              # Harness Store
```

## 🛠️ 常用命令速查

```bash
# 帮助
npm run harness:help

# 架构检查
npm run harness:check

# 依赖检查
npm run harness:check-deps

# 垃圾回收
npm run harness:gc
npm run harness:gc-docs
npm run harness:gc-code
npm run harness:gc-consistency

# 组合使用
npm run harness:check && npm run harness:gc
```

## ⚠️ 常见问题

### Q1: 为什么 `harness:check` 有性能规则错误？

**A**: 这是预期的。性能规则使用浏览器 API，在 Node.js 环境中无法运行。这些规则会在浏览器端正常执行。

### Q2: 如何添加新的架构规则？

**A**: 
1. 在 `.harness/architecture-guardrails/` 创建规则文件
2. 实现 `test()` 方法
3. 在 `scripts/harness/check.ts` 中注册

### Q3: AGENTS.md 应该写什么？

**A**: AGENTS.md 只是导航地图（<100 行），不是百科全书。真正的知识应该分散到 `docs/` 目录。

### Q4: 如何查看 GC 报告？

**A**: GC 执行后会在 `.harness/feedback-loops/garbage-collection/` 生成详细报告。

## 📚 深入学习

完成快速启动后，继续阅读：
1. [HARNESS_ENGINEERING_GUIDE.md](./HARNESS_ENGINEERING_GUIDE.md) - 完整实现指南
2. [README.md](../README.md) - 项目主文档
3. [.harness/AGENTS.md](./.harness/AGENTS.md) - AI Agent 工作指南

## 🎉 下一步

现在你已经掌握了 Harness Engineering 的基础，可以：

1. ✅ 运行架构检查确保代码质量
2. ✅ 使用 ContextProvider 管理会话
3. ✅ 查看 ArchitectureGuardian 了解健康度
4. ✅ 运行 GC 保持系统清洁

继续探索高级功能：
- 自定义架构规则
- 配置闭环测试
- 实现自动修复 Agent

---

*最后更新：2026-03-22*  
*预计阅读时间：5 分钟*

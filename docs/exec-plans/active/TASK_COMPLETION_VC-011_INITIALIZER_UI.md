# 任务执行计划：Initializer Agent UI (VC-011)

## 📋 任务概述

**任务 ID**: VC-011  
**任务名称**: 实现 Initializer Agent UI  
**优先级**: P1  
**状态**: 📋 执行中  
**创建日期**: 2026-03-25  
**预计完成时间**: 2026-03-25  

## 🎯 任务目标

实现 Initializer Agent 的用户界面，展示四步工作流和实时日志输出。

### 核心功能需求

- [ ] InitializerWorkflow 组件实现
- [ ] 四步工作流进度展示
- [ ] 实时日志输出面板
- [ ] 错误处理和状态显示
- [ ] 与 Backend 状态同步
- [ ] Mock 数据演示

## 📁 交付文件清单

### 新增文件

1. **InitializerWorkflow.tsx** (`src/components/vibe-coding/InitializerWorkflow.tsx`)
   - Initializer Agent 主界面组件
   - 四步工作流进度条
   - 实时日志输出面板
   - 状态管理和错误处理

2. **InitializerWorkflow.test.tsx** (`src/components/vibe-coding/InitializerWorkflow.test.tsx`)
   - 单元测试（目标：≥7 个测试用例）
   - 覆盖率 ≥70%

### 修改文件

1. **CodingWorkspace.tsx**
   - 集成 InitializerWorkflow 组件
   - 添加 Initializer Agent 启动入口

2. **hooks/useInitializerAgent.ts** (可选)
   - 封装 Initializer Agent 的 Hook
   - 管理状态和命令调用

## 💻 技术设计

### 组件结构

```typescript
InitializerWorkflow (主组件)
├── WorkflowSteps (四步工作流进度条)
│   ├── Step 1: PRD Parsing
│   ├── Step 2: Environment Check
│   ├── Step 3: Git Initialization
│   └── Step 4: Task Decomposition
├── LogPanel (日志输出面板)
│   └── LogEntry (单条日志)
└── StatusCard (状态卡片)
```

### 状态管理

```typescript
interface InitializerWorkflowState {
  status: 'idle' | 'running' | 'completed' | 'failed'
  currentStep: number
  logs: LogEntry[]
  error?: string
  progress: number // 0-100
}

// InitializerStatus 类型映射
const statusMap = {
  'Idle': { step: 0, progress: 0 },
  'ParsingPRD': { step: 1, progress: 25 },
  'CheckingEnvironment': { step: 2, progress: 50 },
  'InitializingGit': { step: 3, progress: 75 },
  'DecomposingTasks': { step: 4, progress: 90 },
  'WaitingForHITL': { step: 4, progress: 95 },
  'Completed': { step: 4, progress: 100 },
  'Failed': { step: -1, progress: 0 }
}
```

### 数据结构

```typescript
interface LogEntry {
  timestamp: Date
  level: 'info' | 'warn' | 'error' | 'debug'
  message: string
  data?: unknown
}

interface InitializerResult {
  success: boolean
  productName?: string
  issues?: Issue[]
  environmentCheck?: EnvCheckResult
  error?: string
}
```

## 🎨 UI 设计

### 布局结构

```
┌─────────────────────────────────────┐
│  Initializer Agent                  │
├─────────────────────────────────────┤
│  ● → ● → ○ → ○                     │
│  PRD  环境  Git  分解               │
├─────────────────────────────────────┤
│  [INFO] 开始解析 PRD 文档...        │
│  [INFO] PRD 解析完成：电商平台      │
│  [INFO] 检查开发环境...             │
│  [INFO] ✓ Git 已安装 (v2.40.0)     │
│  [INFO] ✓ Node.js 已安装 (v20.0.0) │
│  [INFO] Git 仓库初始化完成          │
│  [INFO] 任务分解完成，共 15 个 Issues │
├─────────────────────────────────────┤
│  状态：运行中 | 进度：75%           │
└─────────────────────────────────────┘
```

### 配色方案

- **成功步骤**: 绿色 (#22c55e)
- **当前步骤**: 蓝色 (#3b82f6)
- **待处理步骤**: 灰色 (#e5e7eb)
- **失败状态**: 红色 (#ef4444)
- **日志级别**: 
  - INFO: 默认文本色
  - WARN: 橙色 (#f97316)
  - ERROR: 红色 (#ef4444)

## 📊 质量指标

| 指标 | 目标 | 实际值 | 状态 |
|------|------|--------|------|
| TypeScript 编译 | 通过 | - | ⏳ 待验证 |
| ESLint 检查 | 通过 | - | ⏳ 待验证 |
| Prettier 格式化 | 一致 | - | ⏳ 待验证 |
| 单元测试数量 | ≥7 | - | ⏳ 待验证 |
| 测试通过率 | 100% | - | ⏳ 待验证 |
| 架构约束违规 | 0 | - | ⏳ 待验证 |
| Health Score | ≥90 | - | ⏳ 待验证 |

## 🚀 实施步骤

### 1. 创建 InitializerWorkflow 组件
- [ ] 定义 Props 接口
- [ ] 实现四步工作流进度条
- [ ] 实现日志输出面板
- [ ] 添加状态管理逻辑
- [ ] 错误处理和边界情况

### 2. 编写单元测试
- [ ] 测试基本渲染
- [ ] 测试不同状态显示
- [ ] 测试日志输出
- [ ] 测试用户交互
- [ ] 测试边界情况

### 3. 集成到 CodingWorkspace
- [ ] 导入 InitializerWorkflow 组件
- [ ] 添加启动按钮/入口
- [ ] 连接 Backend 状态
- [ ] 测试完整流程

### 4. 质量验证
- [ ] 运行 harness:check
- [ ] 修复所有问题
- [ ] 确保 Health Score ≥90

## 🔗 依赖关系

### 已完成依赖
- ✅ VC-006: PRD 解析器
- ✅ VC-007: 环境检查逻辑
- ✅ VC-008: Git 仓库初始化
- ✅ VC-009: 任务分解算法
- ✅ VC-010: Initializer Agent 主逻辑

### 外部依赖
- `lucide-react` - 图标库
- Tailwind CSS - 样式系统

## 📈 预期结果

1. **完整的 Initializer Agent UI** - 用户可以直观看到初始化流程
2. **实时日志输出** - 了解每一步的执行情况
3. **清晰的进度展示** - 四步工作流的可视化
4. **错误处理机制** - 失败时提供友好的错误信息

---

**最后更新**: 2026-03-25  
**状态**: 📋 执行中
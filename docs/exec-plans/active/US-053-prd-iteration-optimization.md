# US-053: PRD 迭代优化 - 执行计划

> **任务 ID**: US-053  
> **任务名称**: PRD 迭代优化  
> **优先级**: P1  
> **Epic**: EPIC-01 (Vibe Design - 功能增强)  
> **Feature**: Feature-01.7 (迭代优化)  
> **预计工时**: 4 小时  
> **实际工时**: 待填写  
> **状态**: 🔄 进行中  
> **创建时间**: 2026-03-30  
> **最后更新**: 2026-03-30  

---

## 📋 任务描述

### 用户故事
作为用户，我希望提供反馈并重新生成，以便改进 PRD 质量。

### 背景说明
在 PRD 生成后，用户可能需要基于质量检查结果（完整性、一致性、可行性）进行多轮迭代优化。系统需要：
- 收集用户的反馈意见
- 结合质量检查报告
- 调用 AI 重新生成优化的 PRD
- 保存迭代历史版本
- 支持版本对比和回滚

---

## 🎯 验收标准

### 功能要求
- [x] **反馈输入**: 提供用户反馈输入界面
- [x] **智能优化**: 结合质量报告和用户反馈进行优化
- [x] **多轮迭代**: 支持至少 3 轮迭代
- [x] **版本历史**: 保存所有迭代版本
- [x] **版本对比**: 支持不同版本间的差异对比
- [x] **版本回滚**: 支持回滚到任意历史版本

### 质量要求
- **迭代次数**: ≥ 3 轮
- **版本保存**: 完整记录每次迭代
- **性能**: 单次迭代耗时 < 10 秒
- **测试覆盖**: Rust ≥ 90%, TypeScript ≥ 80%

---

## 🏗️ 技术方案

### 架构设计
```
┌─────────────────────────────────────┐
│   React Component                   │
│   (PRDIterationPanel)               │
│   - Feedback input                  │
│   - Version history                 │
│   - Version diff viewer             │
└──────────────┬──────────────────────┘
               │ usePRDIteration Hook
┌──────────────▼──────────────────────┐
│   Tauri Command                     │
│   (iterate_prd_optimization)        │
└──────────────┬──────────────────────┘
               │ Rust Backend
┌──────────────▼──────────────────────┐
│   PRD Iteration Manager             │
│   - Version management              │
│   - Feedback integration            │
│   - AI re-generation                │
│   - Diff calculation                │
└─────────────────────────────────────┘
```

### 文件结构
```
src-tauri/
├── src/
│   ├── prd/
│   │   ├── mod.rs                    # 导出 iteration_manager
│   │   └── iteration_manager.rs      # 新增：迭代管理器
│   ├── commands/
│   │   ├── mod.rs                    # 导出 prd 模块
│   │   └── prd.rs                    # 新增：iteration 命令
│   └── main.rs                       # 注册 Tauri command

src/
├── hooks/
│   ├── usePRDIteration.ts            # 新增：迭代 Hook
│   └── usePRDIteration.test.ts       # 新增：Hook 单元测试
├── components/
│   └── PRDIterationPanel.tsx         # 新增：迭代面板组件
└── types/
    └── prd-iteration.ts              # 新增：迭代相关类型
```

### 核心算法

#### 1. 版本管理
```rust
pub struct PRDVersion {
    pub version_id: String,
    pub timestamp: DateTime<Utc>,
    pub prd_content: PRD,
    pub iteration_number: u8,
    pub feedback: Option<String>,
    pub quality_report: Option<QualityReport>,
}

pub struct IterationHistory {
    pub current_version: String,
    pub versions: Vec<PRDVersion>,
}
```

#### 2. 反馈整合
```rust
// 将用户反馈与质量报告结合
let optimization_prompt = format!(
    "基于以下质量问题和用户反馈优化 PRD：\n\n\
     质量问题：{:?}\n\
     用户反馈：{}\n\n\
     请生成优化后的 PRD 版本。",
    quality_issues, user_feedback
);
```

#### 3. 差异计算
```rust
// 使用 diff-match-patch 算法或简单的字段对比
pub fn calculate_diff(old_prd: &PRD, new_prd: &PRD) -> PRDDiff {
    PRDDiff {
        added_features: find_added(&old_prd.features, &new_prd.features),
        removed_features: find_removed(&old_prd.features, &new_prd.features),
        modified_fields: find_modified(&old_prd, &new_prd),
    }
}
```

---

## 📝 实施步骤

### Phase 1: Rust 后端实现（2 小时）

#### Step 1.1: 定义数据结构
- [ ] `PRDVersion` 结构体（版本信息）
- [ ] `IterationHistory` 结构体（迭代历史）
- [ ] `PRDDiff` 结构体（版本差异）
- [ ] `IterationRequest` 结构体（迭代请求）

#### Step 1.2: 实现迭代管理器
- [ ] `PRDIterationManager` 结构体
- [ ] `create_version()` 方法
- [ ] `iterate_with_feedback()` 方法
- [ ] `get_history()` 方法
- [ ] `calculate_diff()` 方法
- [ ] `rollback_to_version()` 方法

#### Step 1.3: 实现 Tauri Command
- [ ] `iterate_prd` 命令
- [ ] `get_iteration_history` 命令
- [ ] `compare_versions` 命令
- [ ] `rollback_to_version` 命令

#### Step 1.4: 单元测试
- [ ] 版本创建测试
- [ ] 迭代流程测试
- [ ] 差异计算测试
- [ ] 覆盖率 > 90%

### Phase 2: TypeScript 前端实现（1.5 小时）

#### Step 2.1: 类型定义
- [ ] 创建 `src/types/prd-iteration.ts`
- [ ] 添加迭代相关类型

#### Step 2.2: Hook 实现
- [ ] `usePRDIteration` Hook
- [ ] 调用 Rust API
- [ ] 状态管理

#### Step 2.3: React 组件
- [ ] `PRDIterationPanel` 组件
- [ ] 反馈输入框
- [ ] 版本历史列表
- [ ] 版本对比视图
- [ ] Tailwind CSS 样式

#### Step 2.4: 单元测试
- [ ] Hook 测试（6 个用例）
- [ ] 组件测试（可选）
- [ ] 覆盖率 > 80%

### Phase 3: 质量验证（0.5 小时）

#### Step 3.1: 代码质量
- [ ] 运行 `npm run harness:check`
- [ ] 修复所有 TypeScript 错误
- [ ] 修复所有 ESLint 问题
- [ ] 修复所有 Prettier 格式问题

#### Step 3.2: 测试验证
- [ ] Rust 测试全部通过
- [ ] TypeScript 测试全部通过
- [ ] Health Score = 100/100

#### Step 3.3: Git 提交
- [ ] 编写符合规范的提交信息
- [ ] 提交到 Git
- [ ] 推送到远程仓库

---

## 📊 完成进度

- [ ] Phase 1: Rust 后端实现 (0%)
- [ ] Phase 2: TypeScript 前端实现 (0%)
- [ ] Phase 3: 质量验证 (0%)

---

## 🔍 技术细节

### 版本数据结构

```typescript
interface PRDVersion {
  /** 版本 ID */
  versionId: string
  /** 时间戳 */
  timestamp: string
  /** PRD 内容 */
  prd: PRD
  /** 迭代轮数 */
  iterationNumber: number
  /** 用户反馈 */
  feedback?: string
  /** 质量报告 */
  qualityReport?: QualityReport
}

interface IterationHistory {
  /** 当前版本 ID */
  currentVersionId: string
  /** 所有版本 */
  versions: PRDVersion[]
}

interface PRDDiff {
  /** 新增的功能 */
  addedFeatures: string[]
  /** 移除的功能 */
  removedFeatures: string[]
  /** 修改的字段 */
  modifiedFields: Array<{
    field: string
    oldValue: string
    newValue: string
  }>
}
```

### 迭代流程

```
1. 用户查看当前 PRD 和质量报告
2. 用户提供反馈意见
3. 系统调用 AI 重新生成 PRD
4. 保存新版本到历史记录
5. 显示版本差异
6. 用户可以选择回滚
```

---

## 📚 参考资料

- [US-050 PRD 完整性检查](./US-050-prd-quality-check.md) - 前序任务
- [US-051 PRD 一致性检查](./US-051-prd-consistency-check.md) - 前序任务
- [US-052 PRD 可行性评估](./US-052-prd-feasibility-assessment.md) - 前序任务
- [Harness Engineering 开发流程](../../dev_workflow.md)

---

## ✅ 检查清单

### 开发前
- [x] 阅读并理解任务需求
- [x] 创建执行计划文档
- [x] 学习相关架构和代码

### 开发中
- [ ] 遵循 Rust + TypeScript 架构规范
- [ ] 编写单元测试（TDD）
- [ ] 保持代码格式规范
- [ ] 及时提交 Git

### 开发后
- [ ] 运行完整质量检查
- [ ] 确认 Health Score = 100/100
- [ ] 更新执行计划状态
- [ ] Git 提交并推送

---

**备注**: 本任务依赖于 US-050/051/052 的质量检查基础设施，需要整合这些质量报告进行智能优化。

# US-051: PRD 一致性检查 - 执行计划

> **任务 ID**: US-051  
> **任务名称**: PRD 一致性检查  
> **优先级**: P1  
> **Epic**: EPIC-01 (Vibe Design - 功能增强)  
> **Feature**: Feature-01.6 (质量检查)  
> **预计工时**: 4 小时  
> **实际工时**: 待填写  
> **状态**: 🔄 进行中  
> **创建时间**: 2026-03-30  
> **最后更新**: 2026-03-30  

---

## 📋 任务描述

### 用户故事
作为系统，我希望验证 PRD 的一致性，以便发现矛盾的需求。

### 背景说明
在 PRD 生成过程中，AI 可能会产生前后矛盾的描述。例如：
- 目标用户与核心功能不匹配
- 技术栈与功能需求不一致
- 预估工作量与功能复杂度不符
- 章节内部存在逻辑矛盾

需要实现自动化的 PRD 一致性检查功能。

---

## 🎯 验收标准

### 功能要求
- [x] **一致性评分**: 提供 0-100 分的一致性评分
- [x] **矛盾检测**: 检测至少 5 种类型的矛盾
- [x] **问题定位**: 准确定位矛盾所在章节
- [x] **改进建议**: 为每个矛盾提供修复建议
- [x] **实时反馈**: 支持实时质量检查

### 质量要求
- [x] **准确性**: 矛盾检测准确率 > 90%
- [x] **完整性**: 覆盖所有主要章节
- [x] **性能**: 检查耗时 < 2 秒
- [x] **测试覆盖**: Rust ≥ 90%, TypeScript ≥ 80%

---

## 🏗️ 技术方案

### 架构设计
```
┌─────────────────────────────────────┐
│   React Component                   │
│   (PRDConsistencyCheckPanel)        │
└──────────────┬──────────────────────┘
               │ usePRDConsistencyCheck Hook
┌──────────────▼──────────────────────┐
│   Tauri Command                     │
│   (check_prd_consistency)           │
└──────────────┬──────────────────────┘
               │ Rust Backend
┌──────────────▼──────────────────────┐
│   PRD Consistency Checker           │
│   - Cross-section validation        │
│   - Logic contradiction detection   │
│   - Requirement alignment check     │
└─────────────────────────────────────┘
```

### 文件结构
```
src-tauri/
├── src/
│   ├── quality/
│   │   ├── mod.rs                    # 导出 prd_consistency_checker
│   │   └── prd_consistency_checker.rs  # 新增：一致性检查器
│   ├── commands/
│   │   ├── mod.rs                    # 导出 quality 模块
│   │   └── quality.rs                # 新增：check_prd_consistency 命令
│   └── main.rs                       # 注册 Tauri command

src/
├── hooks/
│   ├── usePRDConsistencyCheck.ts     # 新增：TypeScript Hook
│   └── usePRDConsistencyCheck.test.ts # 新增：Hook 单元测试
├── components/
│   └── PRDConsistencyCheckPanel.tsx  # 新增：React 组件
└── types/
    └── prd-quality.ts                # 更新：新增一致性相关类型
```

### 核心算法

#### 1. 跨章节验证
```rust
// 目标用户 vs 核心功能
for user in target_users {
    if !features.iter().any(|f| f.serves_user(user)) {
        issues.push(Inconsistency::UserNotServed(user));
    }
}

// 技术栈 vs 功能需求
for feature in core_features {
    let required_techs = feature.get_required_technologies();
    if !required_techs.iter().all(|t| tech_stack.contains(t)) {
        issues.push(Inconsistency::TechStackMismatch(feature, required_techs));
    }
}
```

#### 2. 逻辑矛盾检测
```rust
// 检测工作量估算合理性
let complexity_score = calculate_complexity(features, tech_stack);
let estimated_hours = parse_effort(estimated_effort);
if complexity_score * 10 > estimated_hours * 2 {
    issues.push(Inconsistency::EffortUnderestimate(complexity_score, estimated_hours));
}
```

#### 3. 术语一致性
```rust
// 提取全文术语并检查使用一致性
let terms = extract_terms(prd_content);
for (term, occurrences) in terms {
    if occurrences.variants().len() > 1 {
        issues.push(Inconsistency::TerminologyInconsistent(term, occurrences.variants()));
    }
}
```

---

## 📝 实施步骤

### Phase 1: Rust 后端实现（2 小时）

#### Step 1.1: 定义数据结构
- [ ] `PRDDocument` 结构体（包含所有 PRD 字段）
- [ ] `ConsistencyReport` 结构体（报告格式）
- [ ] `InconsistencyType` 枚举（矛盾类型）
- [ ] `Severity` 枚举（严重程度）

#### Step 1.2: 实现检查器核心逻辑
- [ ] `PRDConsistencyChecker` 结构体
- [ ] `check_consistency()` 主方法
- [ ] 跨章节验证逻辑
- [ ] 逻辑矛盾检测逻辑
- [ ] 术语一致性检查逻辑

#### Step 1.3: 实现 Tauri Command
- [ ] `check_prd_consistency` 命令
- [ ] Markdown 解析
- [ ] 错误处理

#### Step 1.4: 单元测试
- [ ] 完整 PRD 测试（应无矛盾）
- [ ] 矛盾 PRD 测试（应检测到矛盾）
- [ ] 边界条件测试
- [ ] 覆盖率 > 90%

### Phase 2: TypeScript 前端实现（1.5 小时）

#### Step 2.1: 类型定义
- [ ] 更新 `src/types/prd-quality.ts`
- [ ] 添加一致性相关类型

#### Step 2.2: Hook 实现
- [ ] `usePRDConsistencyCheck` Hook
- [ ] 调用 Rust API
- [ ] 状态管理

#### Step 2.3: React 组件
- [ ] `PRDConsistencyCheckPanel` 组件
- [ ] 一致性报告展示
- [ ] 矛盾列表可视化
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

### 矛盾类型定义

```rust
pub enum InconsistencyType {
    // 目标用户与功能不匹配
    UserNotServed(String),
    
    // 技术栈与功能需求不匹配
    TechStackMismatch {
        feature: String,
        required_techs: Vec<String>,
        missing_techs: Vec<String>,
    },
    
    // 工作量低估
    EffortUnderestimate {
        complexity_score: f64,
        estimated_hours: f64,
    },
    
    // 术语不一致
    TerminologyInconsistent {
        term: String,
        variants: Vec<String>,
    },
    
    // 章节间逻辑矛盾
    LogicalContradiction {
        section1: String,
        content1: String,
        section2: String,
        content2: String,
        contradiction: String,
    },
}
```

### 一致性报告格式

```typescript
interface ConsistencyReport {
  /** 总体一致性得分 (0-100) */
  overallScore: number
  /** 各维度评分 */
  dimensions: {
    /** 用户 - 功能对齐度 */
    userFeatureAlignment: number
    /** 技术 - 功能对齐度 */
    techFeatureAlignment: number
    /** 工作量合理性 */
    effortReasonableness: number
    /** 术语一致性 */
    terminologyConsistency: number
    /** 逻辑一致性 */
    logicalConsistency: number
  }
  /** 检测到的矛盾列表 */
  inconsistencies: QualityIssue[]
  /** 改进建议 */
  suggestions: string[]
}
```

---

## 📚 参考资料

- [US-050 PRD 完整性检查](./US-050-prd-quality-check.md) - 前序任务
- [PRD 质量检查器类型定义](../../src/types/prd-quality.ts)
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

**备注**: 本任务依赖于 US-050 的基础设施，可以复用其质量检查框架。

# harness:check 命令优化报告

**日期**: 2026-03-24  
**目的**: 将 `harness:check` 命令默认行为改为完整验证（包含文档一致性检查和死代码检测）  

## 📊 优化前后对比

### 参数变更

| 项目 | 优化前 | 优化后 | 变化 |
|------|--------|--------|------|
| **文档检查** | `-DocCheck` (可选) | **默认执行** (`-NoDocCheck` 跳过) | ⬆️ 更严格 |
| **死代码检测** | `-DeadCode` (可选) | **默认执行** (`-NoDeadCode` 跳过) | ⬆️ 更全面 |
| **快速模式** | 无 | **新增 `-Quick`** (仅核心 8 项) | ⭐ 新选项 |
| **All 参数** | `-All` (包含所有) | **已移除** (不再需要) | ❌ 废弃 |

### 默认行为变化

#### 优化前
```powershell
# 默认：仅运行 8 项核心检查
npm run harness:check
# [1/8] TypeScript 类型
# [2/8] ESLint
# [3/8] Prettier
# [4/8] Rust 编译
# [5/8] Rust 测试
# [6/8] TS 测试
# [7/8] 依赖完整性
# [8/8] 目录结构

# 完整验证：需要额外参数
npm run harness:check -- -All
# 或
npm run harness:check -- -DocCheck -DeadCode
```

#### 优化后
```powershell
# 默认：运行 10 项完整检查
npm run harness:check
# [1/10] TypeScript 类型
# [2/10] ESLint
# [3/10] Prettier
# [4/10] Rust 编译
# [5/10] Rust 测试
# [6/10] TS 测试
# [7/10] 依赖完整性
# [8/10] 目录结构
# [9/10] 文档一致性检查 ⭐ 新增（默认）
# [10/10] 死代码检测 ⭐ 新增（默认）

# 快速模式：仅核心 8 项
npm run harness:check -- -Quick
# [1/8] ... [8/8] (跳过文档和死代码)
```

## ✂️ 删除的参数

### `-All` 参数
**删除原因**: 既然默认就是完整验证，`-All` 参数已不再需要。

**影响**: 
- ✅ 向后兼容：使用 `-All` 的脚本会收到"未知参数"警告，但不影响功能
- ⚠️ 建议更新：将 `harness:check -- -All` 改为 `harness:check`

## ✅ 新增的参数

### 1. `-NoDocCheck` (负向开关)
**用途**: 跳过文档一致性检查

**场景**:
- 文档系统临时不可用
- 紧急修复只需验证代码质量
- 本地快速迭代开发

**示例**:
```powershell
npm run harness:check -- -NoDocCheck
```

### 2. `-NoDeadCode` (负向开关)
**用途**: 跳过死代码检测

**场景**:
- 死代码检测工具临时故障
- 开发过程中允许暂时存在未使用代码
- 性能分析阶段

**示例**:
```powershell
npm run harness:check -- -NoDeadCode
```

### 3. `-Quick` (快速模式)
**用途**: 仅运行核心 8 项检查（跳过文档和死代码）

**场景**:
- 本地开发快速验证
- CI/CD 中的初步检查
- 时间敏感的提交前检查

**示例**:
```powershell
npm run harness:check -- -Quick
```

## 🎯 优化理由

### 1. 质量优先原则
**问题**: 开发者容易忘记运行完整检查，导致文档漂移和死代码积累。

**解决**: 默认包含所有检查，确保每次提交都经过完整验证。

### 2. 减少认知负担
**问题**: 需要记住 `-All`、`-DocCheck`、`-DeadCode` 等多个参数的组合。

**解决**: 默认就是最严格的模式，无需记忆复杂参数。

### 3. 灵活退出机制
**优势**: 通过负向开关提供灵活性，特殊情况下可跳过特定检查。

### 4. 快速模式平衡性能
**优势**: `-Quick` 模式满足快速迭代需求，同时保持核心质量门禁。

## 📊 检查项权重分布

### 完整模式（10 项）

| # | 检查项 | 失败扣分 | 说明 |
|---|--------|---------|------|
| 1 | TypeScript 类型检查 | -20 | 编译时验证 |
| 2 | ESLint 代码质量 | -15 | 代码风格 |
| 3 | Prettier 格式 | -10 | 格式化规范 |
| 4 | Rust 编译检查 | -25 | 编译验证 |
| 5 | Rust 单元测试 | -20 | 后端测试 |
| 6 | TypeScript 单元测试 | -20 | 前端测试 |
| 7 | 依赖完整性 | -5 | 文件检查 |
| 8 | 目录结构 | -5 | 结构验证 |
| 9 | **文档一致性** ⭐ | **-5** | **默认执行** |
| 10 | **死代码检测** ⭐ | **-5** | **默认执行** |

**总分**: 100 分 + 额外扣分项（最多扣至 0 分）

### 快速模式（8 项）

跳过第 9、10 项，保留核心 8 项质量门禁。

## 🔧 使用指南

### 日常开发（推荐）
```powershell
# 完整验证（默认）
npm run harness:check

# 查看详细输出
npm run harness:check -- -Verbose
```

### 快速迭代
```powershell
# 快速模式（跳过文档和死代码）
npm run harness:check -- -Quick

# 自动修复代码问题
npm run harness:check -- -Fix
```

### 特殊情况
```powershell
# 跳过文档检查
npm run harness:check -- -NoDocCheck

# 跳过死代码检测
npm run harness:check -- -NoDeadCode

# 组合使用
npm run harness:check -- -Quick -Fix
```

### CI/CD 集成
```yaml
# GitHub Actions 示例
- name: Harness Engineering Health Check
  run: npm run harness:check
  
- name: Quick Check (PR validation)
  run: npm run harness:check -- -Quick
```

## 📈 改进效果

### 质量保障提升
- ⭐⭐⭐⭐⭐ 默认包含文档检查，防止文档漂移
- ⭐⭐⭐⭐⭐ 默认包含死代码检测，保持代码库清洁
- ⭐⭐⭐⭐⭐ 减少人为疏忽，自动化质量门禁

### 开发者体验提升
- ⭐⭐⭐⭐⭐ 无需记忆复杂参数，默认即最佳实践
- ⭐⭐⭐⭐⭐ 快速模式满足敏捷开发需求
- ⭐⭐⭐⭐⭐ 负向开关提供灵活退出机制

### 维护成本降低
- ⭐⭐⭐⭐⭐ 统一默认行为，减少团队困惑
- ⭐⭐⭐⭐⭐ 减少"忘记运行完整检查"的情况
- ⭐⭐⭐⭐⭐ CI/CD 配置更简洁

## ✅ 验证清单

- ✅ 默认运行 10 项完整检查
- ✅ 文档一致性检查默认启用
- ✅ 死代码检测默认启用
- ✅ `-Quick` 模式仅运行 8 项核心检查
- ✅ `-NoDocCheck` 可跳过文档检查
- ✅ `-NoDeadCode` 可跳过死代码检测
- ✅ `-All` 参数已移除（向后兼容）
- ✅ 评分权重正确更新

## 📚 相关文档更新

以下文档需要同步更新引用：

1. **[HARNESS_ENGINEERING.md](file://d:/workspace/opc-harness/docs/HARNESS_ENGINEERING.md)** - 标准开发流程
   - 更新"质量验证"章节的命令示例
   - 更新检查项表格（8 项 → 10 项）

2. **[AGENTS.md](file://d:/workspace/opc-harness/AGENTS.md)** - 导航地图
   - 更新常用命令部分
   - 添加 `-Quick` 模式说明

3. **package.json** - NPM 脚本定义
   - 无需修改（向后兼容）

## 🎯 下一步计划

1. **更新文档**: 在 HARNESS_ENGINEERING.md 和 AGENTS.md 中更新命令说明
2. **团队通知**: 告知团队成员新的默认行为和参数
3. **监控反馈**: 收集团队使用情况，必要时调整默认策略
4. **CI/CD 优化**: 考虑在 PR 验证中使用 `-Quick` 模式加速流程

---

**优化状态**: ✅ 已完成  
**优化日期**: 2026-03-24  
**影响范围**: 所有 Harness Engineering 质量检查流程  
**向后兼容**: ✅ 是（旧命令仍可运行，仅 `-All` 参数无效）  
**脚本位置**: [`scripts/harness-check.ps1`](file://d:/workspace/opc-harness/scripts/harness-check.ps1)

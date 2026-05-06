## Context

当前 `scripts/harness-check.ps1` 脚本包含 9 个检查步骤,其中第 9 步是文档结构检测(`Invoke-DocumentationCheck`)。该函数检查:
1. `docs/` 目录是否存在(已在迁移中移除)
2. `AGENTS.md` 等关键文档
3. `docs/design-docs/index.md` 等索引文件及其链接

由于项目已完成从 `docs/` 到 OpenSpec 的迁移,这些检查已失效并产生误报警告。

**当前权重分配**(总分 150):
- TypeScript: 20
- ESLint: 15
- Prettier: 10
- Rust: 25
- RustTests: 20
- TSTests: 20
- Dependencies: 5
- Directory: 5
- Documentation: 10 ← 需要移除

**问题**: 当前权重总和为 150,但实际显示为 100 分制,说明有权重归一化逻辑或配置错误。

## Goals / Non-Goals

**Goals:**
- 完全移除文档检测功能及相关配置
- 重新平衡权重分配,确保总分合理
- 更新检查步骤编号和输出信息
- 保持其他所有检测功能不变

**Non-Goals:**
- 不添加新的检测功能
- 不修改其他检测项的逻辑
- 不改变脚本的整体架构
- 不影响 Health Score 的计算方式

## Decisions

### 决策 1: 移除文档检测函数及调用

**选择**: 直接删除 `Invoke-DocumentationCheck` 函数及其在主流程中的调用

**理由**:
- 该功能已完全过时,无保留价值
- 简单删除比注释掉更清晰,避免混淆
- 减少代码维护负担

**替代方案**:
- 注释保留 → 拒绝:会造成代码混乱,未来可能误用
- 改为检查 OpenSpec 文档 → 拒绝:OpenSpec 有自己的验证机制,不应混入 harness:check

### 决策 2: 权重重新分配策略

**选择**: 将 Documentation 的 10 分按比例分配给其他核心检测项

**分配方案**:
```
TypeScript:  20 → 22 (+2)
ESLint:      15 → 17 (+2)
Prettier:    10 → 11 (+1)
Rust:        25 → 28 (+3)
RustTests:   20 → 22 (+2)
TSTests:     20 → 22 (+2)
Dependencies: 5 → 6 (+1)
Directory:    5 → 6 (+1)
Documentation: 10 → 0 (removed)
Total: 150 → 134 (然后归一化到 100)
```

**理由**:
- 按原有权重比例分配,保持相对重要性
- 核心编译和测试项获得更多权重
- 简化配置,更易理解

**替代方案**:
- 平均分配 → 拒绝:不符合各项重要性的差异
- 只增加测试权重 → 拒绝:应平衡提升所有核心项

### 决策 3: 检查步骤重新编号

**选择**: 将步骤从 "X/9" 改为 "X/8",并更新所有相关输出

**实施**:
- 步骤 1-7 保持不变
- 步骤 8 (原 Directory Check): "8/9" → "8/8"
- 移除步骤 9 (Documentation Check)

## Risks / Trade-offs

**[Risk] 失去文档完整性检测** → Mitigation: 
- OpenSpec 工作流本身有完整的文档管理
- Git history 可追溯所有文档变更
- Code review 流程可确保重要文档不被遗漏

**[Risk] 权重调整影响 Health Score 基准** → Mitigation:
- 新权重更反映实际代码质量
- 历史分数对比时需注意权重变化
- 可在 changelog 中记录此次调整

**[Trade-off] 简化 vs 全面性** → 接受简化:
- 文档检测已过时,保留无意义
- harness:check 应聚焦代码质量,而非文档结构
- OpenSpec 提供专门的文档管理能力

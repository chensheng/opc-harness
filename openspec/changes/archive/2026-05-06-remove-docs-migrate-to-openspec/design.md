## Context

当前项目文档体系存在双轨制问题:
- **docs/**: 传统文档目录,包含 29 个文件(设计文档、产品规格、Sprint 计划、参考文档等)
- **openspec/**: OpenSpec 工作流目录,已有 11 个 capabilities

之前的迁移(migrate-docs-to-openspec)只完成了部分工作:
- ✅ 归档了 20 个执行计划和 3 个 ADRs
- ✅ 创建了 5 个新 capabilities (document-management, design-documentation, execution-tracking, product-specification, sprint-planning)
- ❌ 仍保留了大部分 docs/ 内容

现在需要完成剩余的迁移工作,彻底统一文档体系。

## Goals / Non-Goals

**Goals:**
- 将所有 docs/ 文档迁移到 OpenSpec 规范
- 删除整个 docs/ 目录
- 建立统一的文档导航系统(通过 AGENTS.md)
- 确保所有文档引用有效且可访问
- 保持文档内容的完整性和可追溯性

**Non-Goals:**
- 不修改文档的核心内容(仅调整组织结构和格式)
- 不改变现有的代码结构或功能
- 不重构已有的 OpenSpec capabilities
- 不处理 docs/generated/ 和 docs/optimizations/ 空目录

## Decisions

### Decision 1: 文档分类策略

**选择**: 按文档类型和目标用户分类到不同的 capabilities

**方案**:
- **产品规格类** → 独立的 vibe-* capabilities (vibe-coding, vibe-design, vibe-marketing)
- **技术规范类** → 整合到现有 capabilities (design-documentation, data-storage)
- **流程规范类** → 整合到 workflow 相关 capabilities (development-workflow, execution-tracking, sprint-planning)
- **参考文档类** → 归档或整合到 best-practices/coding-harness

**理由**:
- 符合 OpenSpec 的 capability 设计理念
- 便于按模块查找和维护
- 与现有 capabilities 保持一致

**替代方案**:
- ❌ 创建一个巨大的 "documentation" capability - 违反单一职责原则
- ❌ 保留部分 docs/ 子目录 - 无法实现统一目标

### Decision 2: 大型参考文档处理

**选择**: 将超大型参考文档(symphony.md 159KB)单独归档,不作为 capability spec

**理由**:
- OpenSpec specs 应该是精简的能力定义,而非详细参考手册
- 大型文档适合作为归档资料或外部链接
- 避免 specs 过于臃肿

**处理方式**:
- 移动到 `openspec/changes/archive/` 作为历史参考
- 在 AGENTS.md 中提供链接

### Decision 3: 重复文档清理

**选择**: 删除 docs/references/ 中与 OpenSpec 重复的文档

**识别的重复**:
- `openspec-harness-integration.md`
- `openspec-harness-quality-changelog.md`
- `openspec-harness-quality-quickstart.md`
- `openspec-harness-quality-schema.md`

**理由**:
- 这些内容已在 openspec/ 中存在
- 保留重复会造成混淆
- OpenSpec 目录是权威来源

### Decision 4: 迁移文档本身的处理

**选择**: 将之前的迁移指南归档

**处理的文档**:
- `docs/MIGRATION_GUIDE.md` (本次迁移前的指南)
- `docs/migration-guide.md` (更早的迁移指南)

**理由**:
- 迁移完成后,这些指南已过时
- 保留在 archive 中供未来参考
- 新的迁移信息将记录在本次 change 中

### Decision 5: AGENTS.md 作为唯一导航入口

**选择**: 将所有文档索引导航集中到根目录的 AGENTS.md

**取代的索引**:
- `docs/design-docs/index.md`
- `docs/product-specs/index.md`
- `docs/exec-plans/index.md`
- `docs/sprint-plans/index.md`
- `docs/references/index.md`

**理由**:
- 单一入口点,降低查找成本
- 符合 AI Agent 导航地图的定位
- 简化文档结构

## Risks / Trade-offs

**[Risk] 链接断裂影响外部引用** → **Mitigation**: 
- 在 commit message 中明确说明 breaking change
- 创建迁移对照表,帮助找到新位置
- 在 README.md 中添加醒目提示

**[Risk] 团队成员不适应新结构** → **Mitigation**:
- 在 AGENTS.md 中提供清晰的导航和搜索指引
- 保留一段时间的过渡期说明
- 提供快速查找示例

**[Risk] 大量文件移动导致 Git 历史复杂** → **Mitigation**:
- 使用单个 commit 完成所有变更
- 清晰的 commit message 说明变更内容
- Git 会自动识别文件移动(rename detection)

**[Trade-off] 失去 docs/ 的直观目录结构** → 接受短期不便,换取长期统一
- docs/ 的树形结构确实直观,但维护成本高
- OpenSpec 的 capability 结构更利于扩展和维护
- 通过 AGENTS.md 提供良好的导航体验

**[Trade-off] 大型文档归档后可见性降低** → 通过索引弥补
- symphony.md 等大型文档归档后不易发现
- 在 AGENTS.md 中专门列出重要参考文档
- 必要时可以提取关键内容到 specs

## Migration Plan

### Phase 1: 准备阶段
1. 读取所有 docs/ 下的文档内容
2. 分析文档类型和归属
3. 规划每个文档的目标位置

### Phase 2: 创建新 Capabilities (7 个)
1. vibe-coding/spec.md
2. vibe-design/spec.md
3. vibe-marketing/spec.md
4. development-workflow/spec.md
5. data-storage/spec.md
6. coding-harness/spec.md
7. best-practices/spec.md

### Phase 3: 更新现有 Capabilities (4 个)
1. design-documentation - 添加架构规则
2. product-specification - 添加产品设计
3. execution-tracking - 添加模板和规范
4. sprint-planning - 添加 Sprint 指南

### Phase 4: 归档大型/历史文档
1. 创建 archive change: `2026-05-06-docs-archive-batch-2`
2. 移动 symphony.md, migration guides 等
3. 创建 proposal 说明归档内容

### Phase 5: 更新导航
1. 重写 AGENTS.md 的文档导航部分
2. 添加完整的 OpenSpec capabilities 列表
3. 添加归档文档索引
4. 更新 README.md 中的文档链接

### Phase 6: 清理
1. 删除重复的 OpenSpec 参考文档
2. 删除所有 docs/ 内容
3. 删除空的 docs/ 目录

### Phase 7: 验证
1. 运行 harness:check 确保质量
2. 手动验证关键链接
3. 创建 quality-check.md 和 runtime-check.md

### Rollback Strategy
如果发现问题:
1. 从 Git 历史恢复 docs/ 目录
2. 回退 AGENTS.md 修改
3. 重新评估迁移策略

## Open Questions

1. **是否需要保留 docs/ 作为 symlink 指向 openspec/?**
   - 优点: 兼容旧链接
   - 缺点: 增加复杂性,可能引起混淆
   - 倾向: 不保留,彻底清理

2. **vibe-* modules 是否应该合并为一个 capability?**
   - 分开: 更清晰,符合模块化
   - 合并: 减少 specs 数量
   - 倾向: 保持分开,因为它们是独立的产品模块

3. **是否需要在迁移前通知所有团队成员?**
   - 如果是团队协作项目,应该提前通知
   - 当前似乎是个人/小团队项目
   - 倾向: 在 commit message 和 README 中充分说明

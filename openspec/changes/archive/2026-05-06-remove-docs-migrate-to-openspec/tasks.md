## 1. 准备阶段 - 读取和分析 docs/ 文档

- [x] 1.1 读取所有 docs/ 下的 markdown 文件内容
- [x] 1.2 分析每个文档的类型和目标归属
- [x] 1.3 规划每个文档的迁移目标位置
- [x] 1.4 识别需要归档的大型文档(symphony.md, migration guides)

## 2. 迁移产品规格文档到 Capabilities

- [x] 2.1 读取 `docs/product-specs/vibe-coding-spec.md` 并提取关键需求
- [x] 2.2 完善 `openspec/specs/vibe-coding/spec.md` (已创建基础框架)
- [x] 2.3 读取 `docs/product-specs/vibe-design-spec.md` 并整合到 vibe-design capability
- [x] 2.4 读取 `docs/product-specs/vibe-marketing-spec.md` 并整合到 vibe-marketing capability
- [x] 2.5 读取 `docs/product-specs/product-design.md` 并整合到 product-specification capability
- [x] 2.6 更新 `openspec/specs/product-specification/spec.md` 添加产品设计内容

## 3. 迁移设计文档到 Capabilities

- [x] 3.1 读取 `docs/design-docs/system-architecture.md`
- [x] 3.2 将系统架构内容整合到 design-documentation capability
- [x] 3.3 读取 `docs/design-docs/architecture-rules.md`
- [x] 3.4 将架构规则整合到 design-documentation capability
- [x] 3.5 更新 `openspec/specs/design-documentation/spec.md`

## 4. 迁移执行计划模板

- [x] 4.1 读取 `docs/exec-plans/templates/how-to-create-exec-plan.md`
- [x] 4.2 读取 `docs/exec-plans/templates/how-to-archive-exec-plan.md`
- [x] 4.3 读取 `docs/exec-plans/templates/how-to-track-tech-debt.md`
- [x] 4.4 将模板内容整合到 execution-tracking capability
- [x] 4.5 更新 `openspec/specs/execution-tracking/spec.md`

## 5. 迁移 Sprint 计划文档

- [x] 5.1 读取 `docs/sprint-plans/sprint-guide.md`
- [x] 5.2 将 Sprint 指南整合到 sprint-planning capability
- [x] 5.3 读取 `docs/sprint-plans/sprint-1.md` 和 `sprint-2.md`
- [x] 5.4 归档历史 Sprint 计划到 `openspec/changes/archive/`
- [x] 5.5 更新 `openspec/specs/sprint-planning/spec.md`

## 6. 迁移参考文档

- [x] 6.1 读取 `docs/references/autonomous-coding-harness.md`
- [x] 6.2 创建或完善 coding-harness capability spec
- [x] 6.3 读取 `docs/references/best-practices.md`
- [x] 6.4 整合到 best-practices capability
- [x] 6.5 读取 `docs/references/symphony.md` (159KB 大型文档)
- [x] 6.6 归档 symphony.md 到 `openspec/changes/archive/`
- [x] 6.7 删除重复的 OpenSpec 相关文档(已在 openspec/ 中存在)

## 7. 迁移其他文档

- [x] 7.1 读取 `docs/dev_workflow.md`
- [x] 7.2 整合到 development-workflow capability
- [x] 7.3 读取 `docs/data-storage.md`
- [x] 7.4 整合到 data-storage capability
- [x] 7.5 归档 `docs/migration-guide.md` 和 `docs/MIGRATION_GUIDE.md`

## 8. 创建归档 Change

- [x] 8.1 创建新的 OpenSpec change: `2026-05-06-docs-archive-batch-2`
- [x] 8.2 移动 symphony.md, migration guides 等大型文档到 archive
- [x] 8.3 创建 proposal.md 说明归档内容
- [x] 8.4 归档该 change

## 9. 更新 AGENTS.md 导航

- [x] 9.1 读取当前 AGENTS.md 内容
- [x] 9.2 重写文档导航章节,移除所有 docs/ 引用
- [x] 9.3 添加完整的 OpenSpec capabilities 列表(18 个)
- [x] 9.4 添加归档文档索引和链接
- [x] 9.5 更新快速入口部分
- [x] 9.6 添加迁移说明和过渡期指引

## 10. 更新 README.md 和其他入口文档

- [x] 10.1 检查 README.md 中是否有指向 docs/ 的链接
- [x] 10.2 更新所有外部文档引用指向 OpenSpec
- [x] 10.3 添加 breaking change 说明

## 11. 清理 docs/ 目录

- [x] 11.1 确认所有重要文档已迁移或归档
- [x] 11.2 删除 `docs/references/` 中重复的 OpenSpec 文档
- [x] 11.3 删除所有剩余的 docs/ 内容
- [x] 11.4 删除空的 docs/ 目录

## 12. 验证链接完整性

- [x] 12.1 扫描所有 markdown 文件中的内部链接
- [x] 12.2 修复断裂的链接
- [x] 12.3 验证 AGENTS.md 中所有链接可访问
- [x] 12.4 验证 OpenSpec specs 之间的交叉引用

## 13. 质量验证

- [x] 13.1 运行 `npm run harness:check` 确保代码质量
- [x] 13.2 验证所有新创建的 spec 文件格式正确
- [x] 13.3 检查 markdown 格式一致性(Prettier)
- [x] 13.4 确认 Health Score ≥ 80 (实际: 95/100)

## 14. 运行时验证

- [x] 14.1 启动 Tauri 开发服务器验证应用正常
- [x] 14.2 确认无控制台错误
- [x] 14.3 验证文档导航功能(如有 UI)
- [x] 14.4 确认后端日志无异常

## 15. 文档与收尾

- [x] 15.1 编写迁移总结,记录变更内容和影响
- [x] 15.2 创建 quality-check.md 记录质量检查结果
- [x] 15.3 创建 runtime-check.md 记录运行时验证结果
- [x] 15.4 准备归档此 change

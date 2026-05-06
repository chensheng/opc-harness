## 1. 准备阶段 - 分析现有文档结构

- [x] 1.1 统计 `docs/` 目录下所有文档数量和类型
- [x] 1.2 识别需要归档的历史文档(exec-plans/completed、decision-records)
- [x] 1.3 识别需要保留的核心参考文档(system-architecture、architecture-rules、product-specs)
- [x] 1.4 列出所有文档间的交叉引用关系

## 2. 归档历史执行计划

- [x] 2.1 读取 `docs/exec-plans/completed/` 下所有执行计划文件
- [x] 2.2 为每个执行计划创建最小化 OpenSpec proposal (仅包含标题和摘要)
- [x] 2.3 移动执行计划文件到 `openspec/changes/archive/YYYY-MM-DD-<plan-name>/`
- [x] 2.4 更新 `docs/exec-plans/index.md` 移除已归档计划的链接
- [x] 2.5 验证归档后的目录结构符合 OpenSpec 规范

## 3. 归档决策记录 (ADRs)

- [x] 3.1 读取 `docs/design-docs/decision-records/` 下所有 ADR 文件
- [x] 3.2 为每个 ADR 创建 OpenSpec change 并归档到 `openspec/changes/archive/`
- [x] 3.3 在 `docs/design-docs/` 保留 ADR 索引文件,指向归档位置
- [x] 3.4 验证 ADR 引用链接仍然有效

## 4. 建立 Capabilities 体系

- [x] 4.1 创建 `document-management` capability spec (已完成)
- [x] 4.2 创建 `design-documentation` capability spec (已完成)
- [x] 4.3 创建 `product-specification` capability spec (已完成)
- [x] 4.4 创建 `execution-tracking` capability spec (已完成)
- [x] 4.5 创建 `sprint-planning` capability spec (已完成)
- [x] 4.6 验证所有 spec 文件格式符合 OpenSpec schema 要求

## 5. 更新产品规格引用

- [x] 5.1 在 `docs/product-specs/index.md` 添加指向 OpenSpec capabilities 的链接
- [x] 5.2 在每个 product-spec 文档头部添加版本号和最后更新日期
- [x] 5.3 在产品规格中添加对相关执行计划的引用链接
- [x] 5.4 验证所有产品规格文档的内部链接有效性

## 6. 整合 Sprint 计划

- [x] 6.1 检查 `docs/sprint-plans/archive/` 目录结构
- [x] 6.2 将旧的 sprint 计划移动到 archive 目录(如尚未归档)
- [x] 6.3 在 `docs/sprint-plans/index.md` 添加指向 OpenSpec changes 的双向链接
- [x] 6.4 在 sprint-guide.md 中补充 OpenSpec workflow 说明

## 7. 更新文档导航系统

- [x] 7.1 更新根目录 `AGENTS.md` 中的文档导航章节
- [x] 7.2 在 `docs/index.md` (如存在) 添加迁移说明
- [x] 7.3 在 `docs/references/` 添加 OpenSpec 集成指南链接
- [x] 7.4 创建 `docs/MIGRATION_GUIDE.md` 记录本次迁移的详细信息

## 8. 修复断裂链接

- [x] 8.1 运行链接检查工具扫描所有 markdown 文件
- [x] 8.2 修复指向已移动文档的相对路径链接
- [x] 8.3 更新外部引用(如 README.md 中的文档链接)
- [x] 8.4 验证关键文档路径可访问性

## 9. 质量验证

- [x] 9.1 运行 `npm run harness:check` 确保代码质量门禁通过
- [x] 9.2 验证所有新创建的 spec 文件通过 schema 校验
- [x] 9.3 检查所有 markdown 文件格式一致性(Prettier)
- [x] 9.4 确认 Health Score ≥ 80

## 10. 运行时验证

- [x] 10.1 启动 Tauri 开发服务器 `npm run tauri:dev`
- [x] 10.2 验证应用正常启动,无控制台错误
- [x] 10.3 检查文档导航功能正常工作(如有 UI 入口)
- [x] 10.4 确认后端日志无异常

## 11. 文档与收尾

- [x] 11.1 编写迁移总结报告,记录变更内容和影响范围
- [x] 11.2 更新 `openspec/changes/migrate-docs-to-openspec/proposal.md` 补充实际执行情况
- [x] 11.3 创建 quality-check.md 记录质量检查结果
- [x] 11.4 创建 runtime-check.md 记录运行时验证结果
- [x] 11.5 准备归档此 change,运行 `openspec archive migrate-docs-to-openspec`

# Harness Engineering 规范更新总结

> **更新日期**: 2026-03-23  
> **版本**: v1.1.0  
> **变更类型**: 测试架构约束 + 开发流程规范化 🔥

---

## 📋 更新概述

本次更新将**E2E 测试要求**和**完整开发流程**正式写入 Harness Engineering 规范，确保所有开发任务都经过充分的端到端验证。

---

## 🆕 新增内容

### 1. 测试架构约束规则 (5 条) 🔥

在 [`architecture-rules.md`](./references/architecture-rules.md) 中新增 **TEST** 章节：

#### TEST-001: 所有功能必须有单元测试覆盖
- **级别**: ❌ 错误
- **要求**: 每个新功能必须有对应的 `.test.ts` 或 `#[cfg(test)]` 测试
- **覆盖率目标**: ≥70%

#### TEST-002: 核心流程必须有 E2E 测试覆盖 🔥
- **级别**: ❌ 错误
- **要求**: 所有核心用户流程必须包含 E2E 测试用例
- **覆盖场景**: 
  - 应用启动
  - 核心页面导航
  - 关键配置流程
  - API 可达性
- **技术要求**:
  - 自动管理开发服务器生命周期
  - 生成 HTML 测试报告
  - 优雅清理机制

#### TEST-003: 测试必须先于功能完成
- **级别**: ❌ 错误
- **要求**: 遵循 TDD 流程 (Red → Green → Refactor)
- **禁止**: 先写功能后补测试

#### TEST-004: E2E 测试必须独立运行
- **级别**: ⚠️ 警告
- **要求**: 使用 Mock 数据，不依赖真实 API
- **目标**: 确保测试可重复且快速

#### TEST-005: 测试覆盖率不达标禁止合并
- **级别**: ❌ 错误
- **要求**: 覆盖率 <70% 的代码禁止合并
- **必需**: PR 附带测试覆盖率报告

### 2. Harness Engineering 开发流程规范 🔥

创建全新文档 [`harness-engineering-process.md`](./harness-engineering-process.md)：

#### 7 个标准阶段
1. **任务选择** - 基于 MVP 规划选择 P0/P1 任务
2. **架构学习** - 学习 FE-ARCH/BE-ARCH/TEST 约束
3. **测试设计** - 设计单元测试 + E2E 测试场景
4. **开发实施** - Rust 后端 + TypeScript 前端实现
5. **质量验证** - harness:check + 单元测试 + E2E 测试
6. **文档更新** - MVP 规划 + 任务完成报告
7. **完成交付** - Git 提交 + 合规性声明

#### 关键亮点
- ✅ 明确的测试设计要求（含示例代码）
- ✅ E2E 测试服务器自动管理
- ✅ 测试报告自动生成
- ✅ 完整的检查清单
- ✅ 时间分配建议
- ✅ 最佳实践总结

### 3. 最佳实践更新

在 `architecture-rules.md` 的"常见陷阱"部分新增：

#### ❌ 缺少测试覆盖 🔴
- **影响**: 回归 bug 风险高，质量无法保障
- **解决**: 遵循 TDD 流程，先写测试再实现功能

#### ❌ E2E 测试依赖外部服务 ⚠️
- **影响**: CI/CD 失败率高，开发效率低
- **解决**: 使用 Mock 数据，确保测试独立性

#### ❌ 后补测试 ❌
- **影响**: 测试覆盖率不足，代码质量差
- **解决**: 强制执行 TDD，测试不通过不提交

---

## 📊 影响范围

### 受影响的开发流程

| 活动 | 变更前 | 变更后 |
|------|--------|--------|
| **任务开始** | 直接编码 | 先学习架构约束 |
| **测试编写** | 可选 | **强制 (单元+E2E)** |
| **质量验证** | 基本检查 | **6 项全面检查** |
| **文档更新** | 简单标记 | **完整报告+合规声明** |
| **交付标准** | 功能可用 | **100/100 Health Score** |

### 受影响的文件

**新增文件**:
- ✅ `docs/references/harness-engineering-process.md` (新增)
- ✅ `docs/references/HarnessEngineering 规范更新总结.md` (本文档)

**修改文件**:
- ✅ `docs/references/architecture-rules.md` - 添加 TEST 章节
- ✅ `docs/references/index.md` - 添加新文档引用

**已有文件** (无需修改):
- ✅ `scripts/harness-e2e.ps1` - E2E 测试脚本 (已存在)
- ✅ `vite.config.ts` - Vitest 配置 (已存在)
- ✅ `eslint.config.mjs` - ESLint 配置 (已存在)

---

## 🎯 执行要求

### 立即生效

所有新开发任务必须：
1. ✅ 遵循 7 阶段开发流程
2. ✅ 遵守 TEST-001 ~ TEST-005 约束
3. ✅ 编写单元测试 (≥70% 覆盖率)
4. ✅ **编写 E2E 测试 (核心流程覆盖)** 🔥
5. ✅ 通过 harness:check (100/100)
6. ✅ 创建任务完成报告

### 过渡期安排

对于已有功能：
- **P0 级功能**: 1 周内补充 E2E 测试
- **P1 级功能**: 2 周内补充 E2E 测试
- **P2 级功能**: 1 个月内补充 E2E 测试
- **P3 级功能**: 视情况而定

---

## 📈 预期收益

### 质量提升

| 指标 | 当前 | 目标 | 改进 |
|------|------|------|------|
| 测试覆盖率 | ~75% | ≥70% | ✅ 已达标 |
| E2E 覆盖 | 6 个场景 | 核心流程 100% | ✅ 已覆盖 |
| Health Score | 100/100 | ≥90 | ✅ 优秀 |
| 回归 Bug 率 | 低 | 更低 | 📉 预期降低 50% |

### 效率提升

- ✅ **自动化程度提高**: E2E 测试自动管理服务器
- ✅ **问题发现提前**: TDD 确保问题早期暴露
- ✅ **回归测试加速**: E2E 测试自动执行
- ✅ **文档完整性**: 任务报告模板化

### 团队协作

- ✅ **统一流程**: 所有人遵循相同标准
- ✅ **质量对齐**: 明确的交付标准
- ✅ **知识沉淀**: 最佳实践文档化
- ✅ **新人友好**: 清晰的学习路径

---

## 🔗 快速参考

### 核心文档
- 📘 [Harness Engineering 开发流程](./references/harness-engineering-process.md)
- 📕 [架构约束规则](./references/architecture-rules.md)
- 📗 [E2E 测试脚本](../scripts/harness-e2e.ps1)

### 常用命令
```bash
# 开发流程
npm run harness:check      # 架构健康检查
npm run test:unit          # 单元测试
npm run test:e2e          # E2E 测试 🔥
npm run format            # 格式化

# 覆盖率报告
npm run test:unit -- --coverage
```

### 示例代码
- ✅ [单元测试示例](../src/hooks/useOpenAIProvider.test.ts)
- ✅ [E2E 测试示例](../e2e/app.spec.ts) 🔥
- ✅ [Rust 测试示例](../src-tauri/src/ai/mod.rs)

---

## 🎓 培训资源

### 新人入门路径
1. 阅读 [`harness-engineering-process.md`](./references/harness-engineering-process.md)
2. 学习 [`architecture-rules.md`](./references/architecture-rules.md) 中的 TEST 约束
3. 查看 [VD-010 任务完成报告](./exec-plans/active/task-completion-vd-010.md)
4. 实践：完成一个小功能 + 测试 + E2E

### 进阶学习
- [OpenAI Harness Engineering](https://openai.com/index/harness-engineering/)
- [TDD 最佳实践](https://martinfowler.com/bliki/TestDrivenDevelopment.html)
- [E2E 测试策略](https://playwright.dev/docs/test-best-practices)

---

## 📞 下一步行动

### 短期 (1 周)
- [ ] 团队内部宣贯新规范
- [ ] 为现有 P0 功能补充 E2E 测试
- [ ] 收集反馈并优化流程

### 中期 (1 个月)
- [ ] 建立测试覆盖率仪表板
- [ ] 集成到 CI/CD 流程
- [ ] 定期回顾和优化

### 长期 (3 个月)
- [ ] 视觉回归测试
- [ ] 性能测试集成
- [ ] 可访问性测试 (A11y)

---

## 💡 常见问题 (FAQ)

### Q1: 为什么需要 E2E 测试？
**A**: E2E 测试验证完整用户流程，确保各组件协同工作正常。单元测试只能验证单个函数/组件，而 E2E 测试能发现集成问题和回归问题。

### Q2: E2E 测试会不会很慢？
**A**: 合理的 E2E 测试应该：
- ✅ 使用 Mock 数据（不依赖真实 API）
- ✅ 自动管理服务器（减少手动等待）
- ✅ 只覆盖核心流程（避免过度测试）
- ✅ 并行执行测试用例

当前项目 E2E 测试仅需 ~2 秒，非常高效。

### Q3: 如何平衡单元测试和 E2E 测试？
**A**: 遵循测试金字塔：
- **70% 单元测试** - 快速、详细的功能验证
- **20% 集成测试** - 组件间协作验证
- **10%E2E 测试** - 核心用户流程验证

### Q4: 新规范会不会降低开发效率？
**A**: 短期可能增加 15-20% 的时间开销，但长期来看：
- ✅ 减少回归 bug 修复时间
- ✅ 降低手动测试成本
- ✅ 提高代码质量和可维护性
- ✅ 加快新成员上手速度

**总体 ROI**: 正向收益明显

---

**维护者**: OPC-HARNESS Team  
**审查周期**: 季度 ⭐  
**下次审查日期**: 2026-06-23

# 执行计划：Harness Engineering 优化

> **开始日期**: 2026-03-22  
> **优先级**: P0  
> **状态**: 🔄 进行中  
> **预计完成**: 2026-03-25

## 🎯 目标

基于 OpenAI Harness Engineering 最佳实践，全面优化 OPC-HARNESS 的文档结构、架构护栏和反馈回路。

### 核心改进点

1. **Context Engineering** - 改进文档结构和可访问性
2. **Architectural Constraints** - 强化架构护栏和自动化检查
3. **Progress Tracking** - 建立执行进度追踪机制
4. **Knowledge Base** - 重组知识库便于 AI 检索

## 📋 任务列表

### Phase 1: 文档结构重组 (✅ 已完成)

- [x] 优化根目录 AGENTS.md 为导航地图 (< 100 行)
- [x] 创建 src/AGENTS.md (前端规范)
- [x] 创建 src-tauri/AGENTS.md (Rust 规范)
- [x] 重组 docs/ 目录结构
  - [x] design-docs/ (设计文档)
  - [x] exec-plans/ (执行计划)
  - [x] product-specs/ (产品规范)
  - [x] references/ (参考资料)
  - [x] generated/ (自动生成文档)
- [x] 创建各目录索引文件 (index.md)
- [x] 创建 ARCHITECTURE.md 精简版

### Phase 2: 架构护栏增强 (🔄 进行中)

- [ ] 创建自定义 ESLint 规则检测架构约束
  - [ ] 禁止 stores → components 依赖
  - [ ] 禁止 services → commands 依赖
  - [ ] 强制使用路径别名
- [ ] 实现 Rust clippy lint 规则
  - [ ] 检查命令层不包含业务逻辑
  - [ ] 强制错误类型统一
- [ ] 添加架构守卫脚本
  - [ ] harness-guardrails.ps1
  - [ ] 自动修复违规问题

### Phase 3: 进度追踪系统 (📋 待开始)

- [ ] 创建 harness-progress.json 文件格式
- [ ] 实现进度追踪脚本
- [ ] 集成到 Git hooks
- [ ] 每次提交自动更新进度

### Phase 4: 反馈回路优化 (📋 待开始)

- [ ] 改进 CLI Browser 验证
  - [ ] 添加更多测试场景
  - [ ] 自动生成验证报告
- [ ] 实现 doc-gardening Agent
  - [ ] 定期扫描过时文档
  - [ ] 自动发 PR 修复
- [ ] 添加架构健康度指标
  - [ ] 文档新鲜度评分
  - [ ] 架构违规次数
  - [ ] 测试覆盖率趋势

## 📝 决策日志

### 2026-03-22

#### 决策 1: AGENTS.md 分层结构
- **决策**: 采用根目录 + 子目录的分层结构
- **原因**: 
  - 根目录作为导航地图，不超过 100 行
  - 子目录包含具体模块的详细规范
  - 符合 OpenAI 最佳实践（88 个 AGENTS.md 文件）
- **权衡**: 需要维护多个文件，但提高了可维护性

#### 决策 2: docs/ 目录结构化
- **决策**: 按照功能分类重组 docs/ 目录
- **原因**:
  - 便于 AI Agent 按上下文检索
  - 分离活跃文档和历史文档
  - 支持渐进式披露
- **权衡**: 初期组织成本较高，长期收益明显

#### 决策 3: 渐进式披露策略
- **决策**: 文档信息分层展示
- **原因**:
  - 避免一次性灌入大量上下文
  - Agent 按需深入获取信息
  - 减少 Token 消耗
- **实施**: 
  - Level 1: 根目录 AGENTS.md (导航)
  - Level 2: 子目录 AGENTS.md (模块规范)
  - Level 3: 详细设计文档 (技术细节)

### 2026-03-23

#### 决策 4: 架构护栏实现方式
- **决策**: 使用自定义 ESLint + Clippy 规则
- **原因**:
  - 错误信息直接包含修复指引
  - Agent 可以自动读取并修复
  - 比 Code Review 更高效
- **实施计划**: 
  - 编写 eslint-plugin-opc-harness
  - 定义架构约束规则
  - 集成到 CI/CD

## 📊 进展追踪

### 完成度统计

| 阶段 | 任务数 | 已完成 | 进行中 | 待开始 | 完成率 |
|------|--------|--------|--------|--------|--------|
| Phase 1 | 9 | 9 | 0 | 0 | 100% |
| Phase 2 | 6 | 0 | 1 | 5 | 0% |
| Phase 3 | 4 | 0 | 0 | 4 | 0% |
| Phase 4 | 6 | 0 | 0 | 6 | 0% |
| **总计** | **25** | **9** | **1** | **15** | **36%** |

### 关键里程碑

- ✅ **2026-03-22**: 文档结构重组完成
- 🔄 **2026-03-23**: 架构护栏设计
- 📋 **2026-03-24**: 自定义 Linter 实现
- 📋 **2026-03-25**: 完整测试和验收

## 🚨 风险和问题

### 已识别风险

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|---------|
| 自定义 Linter 开发复杂度高 | 中 | 中 | 先实现核心规则，逐步完善 |
| 团队学习曲线 | 低 | 低 | 提供详细文档和示例 |
| 过度约束影响开发效率 | 低 | 中 | 保持规则灵活性，支持豁免 |

### 当前问题

无

## 🎯 成功标准

- [ ] ✅ AGENTS.md 文件数量 >= 3 (根目录 + src + src-tauri)
- [ ] ✅ docs/ 目录结构清晰，每个子目录有索引
- [ ] ⏳ 自定义 ESLint 规则运行正常
- [ ] ⏳ 架构违规自动检测率 > 90%
- [ ] ⏳ 文档新鲜度评分 > 80%

## 📚 产出物

### 文档
- [x] `AGENTS.md` (根目录)
- [x] `src/AGENTS.md`
- [x] `src-tauri/AGENTS.md`
- [x] `ARCHITECTURE.md`
- [x] `docs/design-docs/index.md`
- [x] `docs/exec-plans/index.md`
- [x] `docs/product-specs/index.md`
- [x] `docs/references/index.md`

### 代码 (规划中)
- [ ] `eslint-plugin-opc-harness/`
- [ ] `harness-guardrails.ps1`
- [ ] `harness-progress.json`

### 工具 (规划中)
- [ ] doc-gardening Agent
- [ ] 架构健康度仪表板

---

**最后更新**: 2026-03-23  
**负责人**: OPC-HARNESS Team  
**下次审查**: 2026-03-24

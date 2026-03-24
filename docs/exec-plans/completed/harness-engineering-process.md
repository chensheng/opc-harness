# Harness Engineering 标准开发流程与规范（2026-03-24 更新）

## 核心阶段

### 1. 任务选择
- 基于 MVP 规划，优先选择 P0/P1 关键路径且独立性强的任务。

### 2. 开发实施
- **架构约束**: 开发前查阅并遵守 `architecture-rules.md`，确保符合前后端分层及依赖规范，严禁循环依赖。
- **后端 (Rust)**: 
  - 完整类型定义、错误处理、日志记录
  - 遇到模块错误检查 `mod` 声明及 `Cargo.toml` 依赖
  - **必须编写单元测试**，覆盖率目标 ≥70%
- **前端 (TS/React)**: 
  - 类型安全、Hooks 封装 invoke、使用路径别名 (@/)
  - 组件测试和 Hooks 测试

### 3. 单元测试 ⭐ **已优化**

#### Rust 后端测试
```bash
cd src-tauri
cargo test --bin opc-harness
```

**要求**:
- ✅ 所有功能代码必须包含单元测试
- ✅ 新实现的功能测试覆盖率 ≥70%
- ✅ 在开发过程中可以单独运行 Rust 测试进行调试
- ✅ **最终验证由 `harness:check` 自动执行**（第 5 阶段）

**说明**: 
- Rust 测试运行速度快（通常 <5 秒），推荐在开发时频繁运行
- 但**无需在提交前单独运行**，因为 `harness:check` 会自动执行

#### TypeScript 前端测试
```bash
# 仅在开发调试时使用
npm run test:unit

# 提交前验证直接运行
npm run harness:check
```

**要求**:
- ✅ Hooks 测试覆盖所有自定义 Hooks（useAgent, useDaemon等）
- ✅ Store 测试覆盖状态管理逻辑
- ✅ 工具函数测试
- ✅ 关键组件渲染测试
- ✅ 新实现的功能测试覆盖率 ≥70%
- ✅ **最终验证由 `harness:check` 自动执行**（第 5 阶段 - 第 6 步）

**重要**:
- ⚠️ **无需手动运行 `npm run test:unit`**，因为 `harness:check` 已包含
- 💡 仅在开发调试时单独运行测试以查看详细错误
- 🎯 测试失败会阻塞发布流程（扣 20 分）

**环境处理**:
- 若出现 `ECONNREFUSED` 等数据库连接错误，通常因本地 VectorDB 服务未启动（端口 1420）
- Harness 检查会自动识别此类问题，仅扣 5 分（警告级别）
- 解决方案：
  - 启动 VectorDB 服务后重新测试
  - 使用 Mock 数据代替真实数据库调用
  - 在 CI/CD 中配置数据库服务或跳过集成测试

### 4. 端到端测试 (E2E)
- 单元测试通过后必须执行
- 覆盖核心用户流程
- 自动管理服务器生命周期

### 5. 质量验证 ⭐ **已增强**

运行 `npm run harness:check`，目标 Health Score 100/100。

**检查项（10 项）**:
1. ✅ TypeScript 类型检查
2. ✅ ESLint 代码质量
3. ✅ Prettier 格式规范
4. ✅ Rust 编译检查
5. ✅ **Rust 单元测试**
6. ✅ **TypeScript 单元测试** ⭐
7. ✅ 依赖完整性
8. ✅ 目录结构
9. ✅ 文档一致性（可选）
10. ✅ 死代码检测（可选）

**评分权重**:
- TypeScript 类型错误: -20 分
- ESLint 错误: -15 分
- Prettier 错误: -10 分
- Rust 编译错误: -25 分
- **Rust 测试失败**: -20 分
- **TypeScript 测试失败**: -20 分
  - 真实测试失败：-20 分
  - 数据库连接问题：-5 分（警告）
- 其他警告: -5 到 -10 分

**智能错误识别**:
- TypeScript 测试会自动识别 `ECONNREFUSED` 等数据库连接问题
- 环境问题仅扣 5 分（警告级别），非代码缺陷
- 超时保护：测试超过 30 秒会标记为警告

### 6. 文档更新
- 标记任务状态为已完成
- 创建详细完成报告（含合规性声明）
- **归档规范**: 已完成报告必须移至 `docs/exec-plans/completed`

### 7. 完成交付
- 确认所有质量门禁达标（Health Score ≥ 70）
- 提交 Git 并打标签

## 关键原则

### 质量内建
- ✅ 每次代码修改后即时运行健康检查
- ✅ Rust 测试与 TypeScript 测试同等重要
- ✅ 使用 `harness:fix` 和 Prettier 自动修复

### 自动化优先
- ✅ 优先使用自动化脚本代替手动检查
- ✅ CI/CD 集成完整的 Harness Engineering 流程

### 架构确认先行
- ✅ 修改前彻底确认现有结构
- ✅ 避免基于假设编码
- ✅ 保持文档与代码同步

### 测试驱动
- ✅ 功能实现必须伴随单元测试
- ✅ Rust 和 TypeScript 测试并重
- ✅ 测试失败 = 功能未完成

## 常见问题处理

### Rust 测试问题
1. **编译成功但测试失败**: 检查断言逻辑，确保匹配实际的序列化格式（snake_case vs camelCase）
2. **测试超时**: 异步测试需要使用 `#[tokio::test]`
3. **Mock 数据**: 使用 `mockall` crate 进行依赖注入

### TypeScript 测试问题
1. **ECONNREFUSED**: 数据库服务未启动，可跳过集成测试或使用 Mock
2. **React Testing Library**: 使用 `render` 和 `fireEvent` 进行组件测试
3. **Zustand Store**: 直接调用 store 方法进行验证

### 环境配置
- **Rust**: 确保安装 Rust 1.70+ (`rustup install stable`)
- **Node.js**: 确保安装 Node.js 18+ (`nvm install 18`)
- **VectorDB**: 本地测试可选择启动或 Mock

---

**文档版本**: v2.0  
**最后更新**: 2026-03-24  
**维护者**: OPC-HARNESS Team

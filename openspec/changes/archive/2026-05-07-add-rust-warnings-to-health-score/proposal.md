## Why

当前 `harness:check` 的健康评分系统仅检查 Rust 编译是否通过，但忽略了编译警告（warnings）。这导致即使存在大量 Rust 警告（如未使用的变量、导入等），健康评分仍可达到 100/100。

Rust 警告虽然不影响编译成功，但可能暗示代码质量问题：
- 未使用的导入或变量可能是重构遗留的代码
- 某些警告可能掩盖潜在的逻辑错误
- 累积的警告会降低代码库的可维护性

为了提升代码质量，需要将 Rust 编译警告数量纳入健康评分计算，确保项目保持零警告状态。

## What Changes

- **修改健康评分算法**：在 `harness-check.js` 中添加 Rust 警告检测逻辑
- **调整评分权重**：将 Rust 编译检查从简单的 PASS/FAIL 改为基于警告数量的分级评分
- **更新输出格式**：在检查结果中显示警告数量和类型
- **设置阈值**：定义警告数量与扣分的映射关系（例如：每个警告扣 2 分）

## Capabilities

### New Capabilities

<!-- 此变更不引入新的能力 -->

### Modified Capabilities

- `development-workflow`: 修改 harness:check 的健康评分规则，将 Rust 编译警告纳入评分计算

## Impact

**受影响的代码：**
- `scripts/harness-check.js` - 健康评分检查脚本
- `openspec/specs/development-workflow/spec.md` - 开发工作流规范

**影响范围：**
- 所有运行 `npm run harness:check` 的开发者和 CI/CD 流程
- 现有项目的健康评分可能会下降（如果存在 Rust 警告）
- 需要在实施前清理现有的 Rust 警告

**API 变化：**
- 无 API 变化
- 命令行输出格式会有小幅调整（增加警告计数显示）

**依赖影响：**
- 无新增依赖
- 需要确保 `cargo check` 能够输出警告信息（默认行为）

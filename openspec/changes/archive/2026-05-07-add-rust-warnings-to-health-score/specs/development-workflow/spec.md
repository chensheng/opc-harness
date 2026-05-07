## MODIFIED Requirements

### Requirement: 质量门禁验证
系统 SHALL 在开发者完成编码后自动运行 harness:check，确保 Health Score ≥ 80，并将 Rust 编译警告数量纳入评分计算。

#### Scenario: Rust 编译无警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码无编译警告
- **THEN** Rust Compilation Check 项获得满分（12.5 分）
- **AND** 显示 "[PASS] Rust compilation check passed"

#### Scenario: Rust 编译有少量警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码有 1-6 个编译警告
- **THEN** Rust Compilation Check 项根据警告数量扣分（每个警告扣 2 分）
- **AND** 显示 "[WARN] Found N warnings (-M points)"
- **AND** 列出前 5 个警告的摘要信息

#### Scenario: Rust 编译有大量警告
- **WHEN** 运行 `npm run harness:check` 且 Rust 代码有 7 个或更多编译警告
- **THEN** Rust Compilation Check 项得 0 分
- **AND** 显示 "[FAIL] Found N warnings (maximum penalty applied)"
- **AND** 提示开发者清理警告以提高评分

#### Scenario: 健康评分计算
- **WHEN** 所有 8 项检查完成
- **THEN** 系统计算总分时包含 Rust 警告扣分
- **AND** 如果总分 < 80，标记为 FAIL 并提示需要改进

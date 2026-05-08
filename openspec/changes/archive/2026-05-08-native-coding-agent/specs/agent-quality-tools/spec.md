## ADDED Requirements

### Requirement: ESLint 代码检查

系统 SHALL 提供 ESLint 集成工具，自动检测代码规范问题。

#### Scenario: 执行 ESLint 检查
- **WHEN** AI 生成 TypeScript/JavaScript 代码后调用 run_linter 工具
- **THEN** 系统在工作区根目录执行 `npx eslint . --format json`
- **AND** 解析 JSON 输出为结构化错误列表
- **AND** 返回错误数、警告数和详细报告

#### Scenario: 无 Lint 错误
- **WHEN** 代码通过所有 ESLint 规则
- **THEN** 系统返回成功状态
- **AND** 错误数为 0

#### Scenario: 发现 Lint 错误
- **WHEN** 代码违反 ESLint 规则
- **THEN** 系统返回失败状态
- **AND** 包含错误文件路径、行号、规则名称
- **AND** 提供修复建议（如果可用）

---

### Requirement: TypeScript 类型检查

系统 SHALL 提供 TypeScript 编译器类型检查工具。

#### Scenario: 执行类型检查
- **WHEN** AI 调用 run_typescript_check 工具
- **THEN** 系统执行 `npx tsc --noEmit`
- **AND** 捕获编译错误和警告
- **AND** 返回错误列表（文件、行号、错误消息）

#### Scenario: 类型检查通过
- **WHEN** 所有 TypeScript 文件类型正确
- **THEN** 系统返回成功状态
- **AND** 错误数为 0

#### Scenario: 发现类型错误
- **WHEN** 存在类型不匹配或缺失定义
- **THEN** 系统返回失败状态
- **AND** 包含详细错误信息
- **AND** 标记为需要修复

---

### Requirement: 测试运行器

系统 SHALL 提供自动化测试执行工具，支持单元测试和集成测试。

#### Scenario: 执行全部测试
- **WHEN** AI 调用 run_tests 工具
- **THEN** 系统执行 `npm test` 或 `npx vitest run`
- **AND** 捕获测试输出
- **AND** 解析测试结果（通过/失败/跳过）
- **AND** 返回测试覆盖率统计

#### Scenario: 所有测试通过
- **WHEN** 所有测试用例通过
- **THEN** 系统返回成功状态
- **AND** 通过率 100%

#### Scenario: 测试失败
- **WHEN** 存在失败的测试用例
- **THEN** 系统返回失败状态
- **AND** 包含失败测试的名称、错误消息、堆栈跟踪
- **AND** 提供给 AI 用于自动修复

---

### Requirement: 质量检查聚合

系统 SHALL 提供统一的质量检查接口，并行执行所有检查工具。

#### Scenario: 并行执行质量检查
- **WHEN** AI 完成代码生成后调用 run_quality_checks 工具
- **THEN** 系统并行执行 ESLint、TypeScript Check、Test Runner
- **AND** 等待所有检查完成（最多 60 秒超时）
- **AND** 聚合结果为 QualityCheckResult

#### Scenario: 所有检查通过
- **WHEN** Lint/Test/Type Check 全部通过
- **THEN** 系统返回 passed=true
- **AND** 提交代码更改
- **AND** 标记 Story 为 completed

#### Scenario: 部分检查失败
- **WHEN** 任一检查工具返回错误
- **THEN** 系统返回 passed=false
- **AND** 包含失败的检查类型和错误详情
- **AND** 触发自动修复循环

---

### Requirement: 质量检查超时控制

系统 SHALL 对质量检查设置超时限制，防止长时间阻塞。

#### Scenario: 检查超时
- **WHEN** 质量检查执行超过 60 秒
- **THEN** 系统终止检查进程
- **AND** 返回超时错误
- **AND** 标记 Story 为 failed (quality_check_timeout)

#### Scenario: 正常完成
- **WHEN** 所有检查在 60 秒内完成
- **THEN** 系统返回完整结果
- **AND** 记录执行时间用于性能监控

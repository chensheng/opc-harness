## ADDED Requirements

### Requirement: 前端类型定义包含 failed 状态
UserStory 接口的 status 字段 SHALL 包含 'failed' 作为合法的状态值,确保与后端状态定义保持一致。

#### Scenario: TypeScript 编译时类型检查
- **WHEN** 开发者编写代码访问 UserStory.status 字段
- **THEN** TypeScript 编译器应当接受 'failed' 作为合法的状态值,不会产生类型错误

#### Scenario: 运行时状态值匹配
- **WHEN** 前端从后端 API 接收到 status 为 "failed" 的用户故事数据
- **THEN** 该数据应当能够顺利通过 TypeScript 类型检查,不会导致运行时类型错误

### Requirement: UI 组件正确显示 failed 状态
所有展示用户故事状态的 UI 组件 SHALL 为 'failed' 状态提供清晰的视觉标识和文本标签。

#### Scenario: 表格中显示失败状态徽章
- **WHEN** 用户在用户故事列表中查看状态为 failed 的故事
- **THEN** 系统应当显示红色背景的徽章,标签文字为"失败"

#### Scenario: 状态筛选器包含失败选项
- **WHEN** 用户打开用户故事的状态筛选下拉框
- **THEN** 筛选器中应当包含"失败"选项,允许用户筛选出所有失败的 Story

#### Scenario: 编辑对话框中的状态选择
- **WHEN** 用户在编辑对话框中查看状态选择器
- **THEN** 选择器中应当包含"失败"选项,允许手动设置状态为 failed

### Requirement: 失败信息可视化展示
系统 SHALL 在适当的 UI 位置展示失败相关的详细信息,包括失败原因、重试次数和失败时间。

#### Scenario: 编辑对话框显示失败详情
- **WHEN** 用户打开一个状态为 failed 的用户故事的编辑对话框
- **THEN** 系统应当在对话框中显示只读的失败信息区域,包含:
  - 失败原因 (error_message)
  - 重试次数 (retry_count)
  - 失败时间 (failed_at)

#### Scenario: 非失败状态不显示失败信息
- **WHEN** 用户打开一个状态不是 failed 的用户故事的编辑对话框
- **THEN** 系统不应当显示失败信息区域,保持界面简洁

### Requirement: 支持从 failed 状态重试
系统 SHALL 提供机制允许用户将失败的 Story 重置为可执行状态,以便重新处理。

#### Scenario: 重试按钮可用
- **WHEN** 用户查看一个状态为 failed 的用户故事
- **THEN** 系统应当提供"重试"操作按钮或菜单项

#### Scenario: 执行重试操作
- **WHEN** 用户点击"重试"按钮
- **THEN** 系统应当将该 Story 的状态更新为 'draft',并更新 updatedAt 时间戳

#### Scenario: 重试后 Story 可被 Agent 选取
- **WHEN** Story 的状态从 failed 变更为 draft
- **THEN** 该 Story 应当重新出现在待处理列表中,可以被 Agent Worker 选取执行

### Requirement: 防御性编程处理未知状态
UI 组件 SHALL 使用防御性编程策略处理可能出现的未知状态值,避免渲染错误。

#### Scenario: 状态颜色映射的默认值
- **WHEN** 代码访问 statusColors[story.status] 且 status 不在预定义映射中
- **THEN** 系统应当回退到默认的灰色样式 (bg-gray-100 text-gray-700),而不是返回 undefined

#### Scenario: 状态标签映射的默认值
- **WHEN** 代码访问 statusLabels[story.status] 且 status 不在预定义映射中
- **THEN** 系统应当显示原始状态值或"未知",而不是返回 undefined

### Requirement: 失败状态的排序和筛选
系统 SHALL 允许用户按照状态筛选和排序用户故事,包括 failed 状态。

#### Scenario: 按状态筛选失败的 Story
- **WHEN** 用户在筛选器中选择"失败"状态
- **THEN** 列表应当只显示状态为 failed 的用户故事

#### Scenario: 多条件筛选包含 failed
- **WHEN** 用户同时应用状态筛选(包含 failed)和其他筛选条件(如优先级、Sprint)
- **THEN** 系统应当正确应用所有筛选条件,显示符合所有条件的 Story

### Requirement: 后端状态转换保持不变
后端 Rust 代码中的状态转换逻辑 SHALL 保持不变,继续支持从 pending 状态到 in_development,再到 completed 或 failed 的转换。

#### Scenario: Agent Worker 锁定 Story
- **WHEN** Agent Worker 尝试锁定一个状态为 draft/refined/approved 的 Story
- **THEN** 系统应当将状态更新为 in_development,并记录 assigned_agent 和 locked_at

#### Scenario: Agent Worker 完成任务
- **WHEN** Agent Worker 成功完成 Story 的处理
- **THEN** 系统应当将状态更新为 completed,并记录 completed_at

#### Scenario: Agent Worker 任务失败
- **WHEN** Agent Worker 在处理 Story 时遇到错误
- **THEN** 系统应当将状态更新为 failed,记录 error_message、failed_at,并将 retry_count 加 1

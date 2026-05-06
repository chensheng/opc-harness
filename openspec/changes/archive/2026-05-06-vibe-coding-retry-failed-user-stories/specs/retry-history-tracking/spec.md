## ADDED Requirements

### Requirement: 重试历史记录存储

系统 SHALL 提供重试历史记录存储功能，记录每次重试的详细信息。

#### Scenario: 创建重试历史记录
- **WHEN** 重试引擎决定重试某个用户故事
- **THEN** 系统在 `user_story_retry_history` 表中创建新记录，包含 retry_number、triggered_at、error_message 等字段

#### Scenario: 更新重试结果
- **WHEN** 重试任务执行完成
- **THEN** 系统更新对应的历史记录，设置 completed_at 和 result 字段（success/failed）

#### Scenario: 记录错误类型
- **WHEN** 创建重试历史记录时
- **THEN** 系统记录错误分类器判定的错误类型（temporary/permanent）

#### Scenario: 记录决策结果
- **WHEN** 重试引擎做出决策
- **THEN** 系统记录决策结果（retry/abort）到历史记录

### Requirement: 重试历史查询接口

系统 SHALL 提供查询用户故事重试历史的 API 接口。

#### Scenario: 获取单个 Story 的重试历史
- **WHEN** 前端调用 `get_user_story_retry_history` Command，传入 user_story_id
- **THEN** 系统返回该 Story 的所有重试历史记录，按 triggered_at 降序排列

#### Scenario: 获取项目的重试统计
- **WHEN** 前端请求项目的重试统计数据
- **THEN** 系统返回总重试次数、成功率、平均重试次数等聚合指标

#### Scenario: 过滤特定时间段的重试记录
- **WHEN** 查询时指定时间范围（start_date, end_date）
- **THEN** 系统仅返回该时间段内的重试记录

#### Scenario: 按结果过滤重试记录
- **WHEN** 查询时指定 result 过滤器（success/failed/pending）
- **THEN** 系统仅返回匹配结果类型的重试记录

### Requirement: 前端重试历史展示

系统 SHALL 在前端 UI 中清晰展示用户故事的重试历史信息。

#### Scenario: 在用户故事卡片中显示重试次数
- **WHEN** 用户查看用户故事列表或详情
- **THEN** 对于有重试记录的 Story，显示 "已重试 X 次" 徽章

#### Scenario: 查看重试历史时间线
- **WHEN** 用户点击 Story 的"查看重试历史"按钮
- **THEN** 系统弹出对话框，显示时间线形式的重试历史，包括每次的时间、错误信息和结果

#### Scenario: 显示预计下次重试时间
- **WHEN** Story 处于 scheduled_retry 状态
- **THEN** 前端显示 "预计下次重试时间: YYYY-MM-DD HH:mm:ss"

#### Scenario: 区分成功和失败的重试
- **WHEN** 展示重试历史时间线
- **THEN** 成功的重试用绿色标识，失败的重试用红色标识，待处理的重试用黄色标识

### Requirement: 重试数据分析

系统 SHALL 提供重试数据的分析功能，帮助优化重试策略。

#### Scenario: 计算重试成功率
- **WHEN** 用户查看项目的重试统计面板
- **THEN** 系统显示整体重试成功率（成功重试次数 / 总重试次数 * 100%）

#### Scenario: 识别高频失败的故事
- **WHEN** 系统分析重试数据
- **THEN** 标记重试次数超过阈值（如 5 次）的 Story，提示需要人工介入

#### Scenario: 统计常见错误类型
- **WHEN** 用户查看错误类型分布图表
- **THEN** 系统显示各类错误（网络超时、API 限流、代码错误等）的出现频率

#### Scenario: 生成重试趋势报告
- **WHEN** 用户选择时间范围生成报告
- **THEN** 系统生成折线图，显示每日/每周的重试次数和成功率趋势

### Requirement: 重试历史清理策略

系统 SHALL 实现重试历史的自动清理策略，避免数据库无限增长。

#### Scenario: 定期归档旧的重试记录
- **WHEN** 系统执行定期维护任务（每周一次）
- **THEN** 将超过 90 天的重试记录移动到归档表或导出为 JSON 文件

#### Scenario: 删除超期归档记录
- **WHEN** 归档记录超过 365 天
- **THEN** 系统自动删除这些记录以释放存储空间

#### Scenario: 保留关键重试记录
- **WHEN** 清理重试历史时
- **THEN** 始终保留每个 Story 的最后一次重试记录和所有成功的重试记录

#### Scenario: 手动清理重试历史
- **WHEN** 用户在项目设置中点击"清理重试历史"按钮
- **THEN** 系统提示确认操作，确认后删除该项目的所有重试历史记录（保留最近 7 天的记录）

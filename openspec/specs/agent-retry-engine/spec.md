## ADDED Requirements

### Requirement: 智能重试决策引擎

系统 SHALL 提供智能重试决策引擎，根据失败原因、重试次数和错误类型自动决定是否重试用户故事。

#### Scenario: 临时错误自动重试
- **WHEN** Agent Worker 执行用户故事失败，错误类型为临时错误（如网络超时、API 限流）
- **THEN** 重试引擎检查重试次数未超过最大值，计算下次重试时间，将 Story 状态更新为 scheduled_retry

#### Scenario: 永久错误终止重试
- **WHEN** Agent Worker 执行用户故事失败，错误类型为永久错误（如代码逻辑错误、依赖缺失）
- **THEN** 重试引擎直接将 Story 标记为 permanently_failed，不再安排重试

#### Scenario: 超过最大重试次数
- **WHEN** 用户故事的重试次数已达到配置的最大值
- **THEN** 重试引擎终止重试流程，将 Story 标记为 max_retries_exceeded，并通知用户

#### Scenario: 读取重试配置
- **WHEN** 重试引擎初始化或执行决策时
- **THEN** 从项目配置中读取最大重试次数、基础延迟时间和最大延迟时间参数

### Requirement: 错误分类器

系统 SHALL 实现错误分类器，能够区分临时错误和永久错误。

#### Scenario: 识别网络超时错误
- **WHEN** 错误消息包含 "timeout"、"connection refused" 或 "network error"
- **THEN** 分类器将其标记为 temporary 类型

#### Scenario: 识别 API 限流错误
- **WHEN** 错误消息包含 "rate limit"、"429 Too Many Requests"
- **THEN** 分类器将其标记为 temporary 类型

#### Scenario: 识别代码编译错误
- **WHEN** 错误消息包含 "syntax error"、"compilation failed" 或 "type error"
- **THEN** 分类器将其标记为 permanent 类型

#### Scenario: 识别依赖缺失错误
- **WHEN** 错误消息包含 "module not found" 或 "dependency resolution failed"
- **THEN** 分类器将其标记为 permanent 类型

#### Scenario: 未知错误默认处理
- **WHEN** 错误消息无法匹配任何已知模式
- **THEN** 分类器默认将其标记为 permanent 类型以确保安全

### Requirement: 指数退避算法

系统 SHALL 实现指数退避算法，计算下次重试的延迟时间。

#### Scenario: 首次重试延迟计算
- **WHEN** 重试次数为 0（首次重试）
- **THEN** 延迟时间为 base_delay（默认 60 秒）± 10% 随机抖动

#### Scenario: 第二次重试延迟计算
- **WHEN** 重试次数为 1（第二次重试）
- **THEN** 延迟时间为 base_delay * 2^1 = 120 秒 ± 10% 随机抖动

#### Scenario: 第三次重试延迟计算
- **WHEN** 重试次数为 2（第三次重试）
- **THEN** 延迟时间为 base_delay * 2^2 = 240 秒 ± 10% 随机抖动

#### Scenario: 延迟时间不超过最大值
- **WHEN** 计算的延迟时间超过 max_delay（默认 3600 秒）
- **THEN** 使用 max_delay 作为实际延迟时间

#### Scenario: 随机抖动应用
- **WHEN** 计算基础延迟时间后
- **THEN** 添加 ±10% 的随机抖动，避免多个 Story 同时重试

### Requirement: 重试调度器

系统 SHALL 实现重试调度器，管理待重试的用户故事队列。

#### Scenario: 添加重试任务到队列
- **WHEN** 重试引擎决定重试某个用户故事
- **THEN** 调度器将该 Story 添加到待重试队列，设置 next_retry_at 时间戳

#### Scenario: 检查并重试到期任务
- **WHEN** 调度器定期检查（每 30 秒）待重试队列
- **THEN** 对于 next_retry_at <= 当前时间的 Story，触发 Agent Worker 重新执行

#### Scenario: 限制并发重试数量
- **WHEN** 待重试队列中有多个 Story 到期
- **THEN** 调度器最多同时启动 3 个重试任务，其余等待下一轮检查

#### Scenario: 移除已完成的重试任务
- **WHEN** 重试任务执行完成（成功或失败）
- **THEN** 调度器从待重试队列中移除该任务

### Requirement: 重试配置管理

系统 SHALL 允许用户自定义重试策略配置。

#### Scenario: 设置最大重试次数
- **WHEN** 用户在项目设置中修改最大重试次数
- **THEN** 系统将配置保存到数据库，新创建的 Story 使用该配置

#### Scenario: 设置基础延迟时间
- **WHEN** 用户在项目设置中修改基础延迟时间（30s-300s 范围）
- **THEN** 系统验证输入合法性并保存配置

#### Scenario: 设置最大延迟时间
- **WHEN** 用户在项目设置中修改最大延迟时间（300s-7200s 范围）
- **THEN** 系统验证输入合法性并确保大于基础延迟时间

#### Scenario: 启用/禁用自动重试
- **WHEN** 用户切换自动重试开关
- **THEN** 系统更新配置，禁用后所有失败的 Story 不再自动重试

#### Scenario: 恢复默认配置
- **WHEN** 用户点击"恢复默认"按钮
- **THEN** 系统将重试配置重置为默认值（max_retries=3, base_delay=60s, max_delay=3600s）

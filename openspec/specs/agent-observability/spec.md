## ADDED Requirements

### Requirement: 智能体状态实时展示
系统 SHALL 在 AgentMonitor 组件中实时展示每个智能体的运行状态，包括：
- 当前状态（idle/running/paused/completed/failed/stopped）
- 当前执行任务描述
- 进度百分比（0-100）
- 运行时长

#### Scenario: 展示运行中的智能体
- **WHEN** 智能体状态为 running
- **THEN** 系统 SHALL 显示绿色状态标识和实时进度条
- **AND** 系统 SHALL 每 5 秒自动刷新状态数据

#### Scenario: 展示失败状态的智能体
- **WHEN** 智能体状态为 failed
- **THEN** 系统 SHALL 显示红色状态标识
- **AND** 系统 SHALL 显示错误原因摘要

### Requirement: 结构化日志分级显示
系统 SHALL 按照日志级别（INFO/WARN/ERROR/DEBUG/SUCCESS）对日志进行分类和着色显示：
- INFO: 蓝色，普通信息
- WARN: 黄色，警告信息
- ERROR: 红色，错误信息
- DEBUG: 灰色，调试信息
- SUCCESS: 绿色，成功信息

#### Scenario: 按级别过滤日志
- **WHEN** 用户选择"错误"过滤选项
- **THEN** 系统 SHALL 仅显示 level 为 error 的日志
- **AND** 系统 SHALL 显示过滤后的日志数量统计

#### Scenario: 日志级别着色
- **WHEN** 系统渲染日志条目
- **THEN** 系统 SHALL 根据日志级别应用对应的颜色样式
- **AND** 系统 SHALL 显示对应的级别图标

### Requirement: 日志搜索和过滤
系统 SHALL 提供日志搜索和过滤功能：
- 支持按日志级别过滤（全部/信息/警告/错误/调试/成功）
- 支持按关键词搜索（消息内容、来源）
- 支持自动滚动开关

#### Scenario: 关键词搜索日志
- **WHEN** 用户在搜索框输入关键词"error"
- **THEN** 系统 SHALL 显示消息内容或来源包含"error"的日志
- **AND** 系统 SHALL 高亮显示匹配的关键字

#### Scenario: 关闭自动滚动
- **WHEN** 用户点击"自动滚动：开"按钮
- **THEN** 系统 SHALL 切换为"自动滚动：关"
- **AND** 系统 SHALL 停止自动滚动到最新日志

### Requirement: 日志持久化存储
系统 SHALL 将日志异步持久化到 SQLite 数据库：
- 每条日志包含：时间戳、级别、来源、消息内容、智能体 ID
- 批量写入策略：每 100 条或每 10 秒触发一次写入
- 支持按智能体 ID 和时间范围查询

#### Scenario: 批量写入日志
- **WHEN** 内存中累积 100 条日志
- **THEN** 系统 SHALL 触发批量写入数据库
- **AND** 系统 SHALL 在写入成功后清空已写入的日志缓存

#### Scenario: 查询智能体历史日志
- **WHEN** 用户请求查看智能体 agent-xxx 的日志
- **THEN** 系统 SHALL 从数据库加载该智能体的历史日志
- **AND** 系统 SHALL 按时间升序排列显示

### Requirement: 性能指标监控
系统 SHALL 收集和展示智能体的性能指标：
- CPU 使用率（百分比）
- 内存使用量（MB）
- API 调用次数和平均响应时间

#### Scenario: 实时显示 CPU 使用率
- **WHEN** 智能体处于 running 状态
- **THEN** 系统 SHALL 每 5 秒采集一次 CPU 使用率
- **AND** 系统 SHALL 在智能体卡片上显示当前 CPU 使用率

#### Scenario: 显示 API 调用统计
- **WHEN** 智能体完成一次 API 调用
- **THEN** 系统 SHALL 记录调用时间和响应时长
- **AND** 系统 SHALL 在性能面板显示 P50/P90/P99 响应时间

### Requirement: 内存日志缓存管理
系统 SHALL 管理内存中的日志缓存：
- 每个智能体最多保留最近 100 条日志
- 超出限制时自动淘汰最旧的日志
- 静默刷新时保留现有日志数据

#### Scenario: 日志缓存溢出
- **WHEN** 智能体的日志数量达到 100 条
- **THEN** 系统 SHALL 添加新日志时自动移除最旧的日志
- **AND** 系统 SHALL 保持日志总数不超过 100 条

#### Scenario: 静默刷新保留日志
- **WHEN** 系统执行静默刷新（loadAgents(true)）
- **THEN** 系统 SHALL 保留现有智能体的日志数据
- **AND** 系统 SHALL 仅更新状态信息而不清空日志

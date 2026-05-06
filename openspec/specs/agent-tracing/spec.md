## ADDED Requirements

### Requirement: 思考链记录
系统 SHALL 记录 AI 智能体的思考链（Chain of Thought）：
- 记录每个思考步骤的内容和时间戳
- 支持思考步骤的父子关联（形成思考树）
- 思考数据以 JSON 格式存储

#### Scenario: 记录思考步骤
- **WHEN** 智能体产生一个新的思考步骤
- **THEN** 系统 SHALL 创建一条 trace 记录，event_type 为 'thought'
- **AND** 系统 SHALL 包含思考内容、时间戳和可选的父节点 ID

#### Scenario: 展示思考树
- **WHEN** 用户打开智能体的追踪视图
- **THEN** 系统 SHALL 以树形结构展示思考步骤
- **AND** 系统 SHALL 支持展开/折叠子思考步骤

### Requirement: 工具调用追踪
系统 SHALL 记录智能体的工具调用轨迹：
- 记录工具名称、调用参数、调用时间
- 记录工具执行结果（成功/失败）
- 记录工具执行的耗时

#### Scenario: 记录工具调用
- **WHEN** 智能体调用一个工具（如 git_commit、file_write）
- **THEN** 系统 SHALL 创建一条 trace 记录，event_type 为 'tool_call'
- **AND** 系统 SHALL 包含工具名称、参数和调用时间

#### Scenario: 记录工具执行结果
- **WHEN** 工具执行完成
- **THEN** 系统 SHALL 创建一条 trace 记录，event_type 为 'tool_result'
- **AND** 系统 SHALL 关联到对应的 tool_call 记录（通过 parent_id）
- **AND** 系统 SHALL 包含执行结果和耗时

### Requirement: 决策点记录
系统 SHALL 记录智能体的关键决策点：
- 记录决策上下文（当前状态、可用选项）
- 记录最终选择的决策和理由
- 支持决策回溯

#### Scenario: 记录任务分解决策
- **WHEN** 智能体将一个大任务分解为多个子任务
- **THEN** 系统 SHALL 创建一条 trace 记录，event_type 为 'decision'
- **AND** 系统 SHALL 包含原始任务、分解方案和决策理由

#### Scenario: 决策回溯
- **WHEN** 用户查看决策历史记录
- **THEN** 系统 SHALL 按时间顺序展示所有决策点
- **AND** 系统 SHALL 支持点击查看决策详情

### Requirement: 执行轨迹回放
系统 SHALL 支持智能体执行轨迹的回放：
- 按时间顺序展示所有事件（思考、工具调用、决策）
- 支持快进/慢放/单步执行
- 支持跳转到特定时间点

#### Scenario: 时间线回放
- **WHEN** 用户点击"回放"按钮
- **THEN** 系统 SHALL 按时间顺序逐步展示事件
- **AND** 系统 SHALL 显示当前播放进度

#### Scenario: 跳转到特定时间点
- **WHEN** 用户在时间线上拖动到某个位置
- **THEN** 系统 SHALL 显示该时间点之前所有事件的汇总
- **AND** 系统 SHALL 允许从该时间点继续回放

### Requirement: 追踪数据存储
系统 SHALL 将追踪数据持久化到数据库：
- 表结构：agent_traces (id, agent_id, session_id, event_type, timestamp, data, parent_id)
- 支持按 agent_id 和时间范围查询
- 支持按 event_type 过滤

#### Scenario: 查询智能体追踪记录
- **WHEN** 用户请求查看智能体 agent-xxx 的追踪记录
- **THEN** 系统 SHALL 从 agent_traces 表查询该智能体的所有记录
- **AND** 系统 SHALL 按 timestamp 升序返回

#### Scenario: 按事件类型过滤
- **WHEN** 用户选择仅查看"工具调用"事件
- **THEN** 系统 SHALL 仅返回 event_type 为 'tool_call' 和 'tool_result' 的记录
- **AND** 系统 SHALL 保持事件的时序关系

### Requirement: 追踪数据压缩
系统 SHALL 对追踪数据进行压缩存储：
- 相同类型的连续事件可以合并
- 低价值事件（如心跳）可以过滤
- 支持配置压缩策略

#### Scenario: 合并连续思考
- **WHEN** 智能体产生多个连续的 thought 事件
- **THEN** 系统 SHALL 可选地将它们合并为一条记录
- **AND** 系统 SHALL 保留所有思考内容的完整信息

#### Scenario: 过滤低价值事件
- **WHEN** 配置中启用了事件过滤
- **THEN** 系统 SHALL 不记录类型为 'heartbeat' 的事件
- **AND** 系统 SHALL 记录其他关键事件

## MODIFIED Requirements

### Requirement: 执行轨迹回放

系统 SHALL 支持智能体执行轨迹的回放,包括前端控制台日志事件:
- 按时间顺序展示所有事件(思考、工具调用、决策、前端日志)
- 支持快进/慢放/单步执行
- 支持跳转到特定时间点
- 前端日志以独立事件类型展示,区分日志级别

#### Scenario: 时间线回放包含前端日志
- **WHEN** 用户点击"回放"按钮且存在前端 console 日志记录
- **THEN** 系统 SHALL 按时间顺序逐步展示所有事件,包括前端日志
- **AND** 系统 SHALL 在时间线中以不同颜色或图标标识前端日志事件

#### Scenario: 查看前端日志详情
- **WHEN** 用户在回放中遇到前端日志事件
- **THEN** 系统 SHALL 显示日志级别(log/info/warn/error)、消息内容和时间戳
- **AND** 系统 SHALL 标识该日志来源于前端([Frontend] 标记)

### Requirement: 追踪数据存储

系统 SHALL 将追踪数据持久化到数据库,包括前端控制台日志:
- 表结构:agent_traces (id, agent_id, session_id, event_type, timestamp, data, parent_id)
- 新增 event_type: 'frontend_console' 用于标识前端日志事件
- 支持按 agent_id、时间范围和 event_type 查询
- 前端日志数据包含 level、message、source 字段

#### Scenario: 记录前端 console 日志
- **WHEN** 后端接收到来自前端的 console_log 命令调用
- **THEN** 系统 SHALL 创建一条 trace 记录,event_type 为 'frontend_console'
- **AND** 系统 SHALL 在 data 字段存储 { level: "error", message: "...", source: "frontend" }
- **AND** 系统 SHALL 关联到当前活跃的 agent session

#### Scenario: 查询包含前端日志的追踪记录
- **WHEN** 用户请求查看智能体 agent-xxx 的完整追踪记录
- **THEN** 系统 SHALL 从 agent_traces 表查询该智能体的所有记录,包括 frontend_console 事件
- **AND** 系统 SHALL 按 timestamp 升序返回,保持事件时序

#### Scenario: 仅查询前端日志事件
- **WHEN** 用户选择仅查看"前端日志"事件
- **THEN** 系统 SHALL 仅返回 event_type 为 'frontend_console' 的记录
- **AND** 系统 SHALL 支持按日志级别进一步过滤(如仅看 error 级别)

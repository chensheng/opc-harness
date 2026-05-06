## ADDED Requirements

### Requirement: Agent Worker 重试引擎集成

Agent Worker SHALL 在初始化时加载并配置重试引擎，使其能够自动处理失败的用户故事。

#### Scenario: Agent Worker 启动时初始化重试引擎
- **WHEN** Agent Worker 进程启动
- **THEN** 系统从数据库读取项目的重试配置，初始化 RetryEngine 实例

#### Scenario: Agent Worker 注册重试调度器定时器
- **WHEN** Agent Worker 完成初始化
- **THEN** 系统启动一个后台定时器，每 30 秒检查一次待重试队列

#### Scenario: Agent Worker 优雅关闭时清理定时器
- **WHEN** Agent Worker 接收到关闭信号
- **THEN** 系统停止重试调度器定时器，等待当前重试任务完成后退出

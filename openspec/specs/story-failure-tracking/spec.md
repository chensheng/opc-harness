## ADDED Requirements

### Requirement: 捕获并存储失败原因
当智能体执行用户故事最终判定为失败时，系统 SHALL 将详细的失败原因持久化存储到数据库的 `user_stories` 表中。

#### Scenario: CLI 进程参数错误导致失败
- **WHEN** AI CLI 进程因不支持的参数（如 `--story-id`）退出且状态码非零
- **THEN** 系统提取标准错误输出中的关键信息（如 "No such option"），并将其存入 `failure_reason` 字段

#### Scenario: 任务达到最大重试次数
- **WHEN** 任务经历多次重试后仍无法成功
- **THEN** 系统将最后一次尝试的错误摘要和重试次数记录在案

### Requirement: 前端展示失败详情
前端界面 SHALL 在用户故事或智能体监控面板中清晰地展示失败原因。

#### Scenario: 查看失败的用户故事
- **WHEN** 开发者在 Agent Monitor 中点击状态为 `failed` 的智能体卡片
- **THEN** 界面展开显示红色的错误详情块，包含具体的失败描述

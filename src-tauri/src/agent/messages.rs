//! Agent 消息定义
//!
//! 包含请求、响应、事件等消息结构

use crate::agent::types::AgentStatus;
use serde::{Deserialize, Serialize};

/// Agent 请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequest {
    /// 请求唯一标识
    pub request_id: String,
    /// 发送请求的 Agent ID
    pub agent_id: String,
    /// 动作类型
    pub action: String,
    /// 请求载荷
    pub payload: serde_json::Value,
}

impl AgentRequest {
    /// 创建新的请求
    pub fn new(agent_id: String, action: String, payload: serde_json::Value) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            agent_id,
            action,
            payload,
        }
    }
}

/// Agent 响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    /// 响应唯一标识
    pub response_id: String,
    /// 对应的请求 ID
    pub request_id: String,
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

impl AgentResponse {
    /// 创建成功响应
    pub fn success(request_id: String, data: Option<serde_json::Value>) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: true,
            data,
            error: None,
        }
    }

    /// 创建失败响应
    pub fn error(request_id: String, error_msg: String) -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            request_id,
            success: false,
            data: None,
            error: Some(error_msg),
        }
    }
}

/// 消息类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 日志消息
    Log,
    /// 状态更新
    Status,
    /// 进度更新
    Progress,
    /// 错误消息
    Error,
    /// 心跳消息
    Heartbeat,
}

/// Agent 消息 (用于实时推送)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// 消息唯一标识
    pub message_id: String,
    /// 时间戳 (Unix timestamp)
    pub timestamp: i64,
    /// 消息来源："agent" | "daemon" | "frontend"
    pub source: String,
    /// 消息类型
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// 消息内容
    pub content: String,
    /// 附加元数据
    pub metadata: Option<serde_json::Value>,
}

impl AgentMessage {
    /// 创建日志消息
    pub fn log(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Log,
            content,
            metadata: None,
        }
    }

    /// 创建进度消息
    pub fn progress(source: String, content: String, progress: f32) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Progress,
            content,
            metadata: Some(serde_json::json!({ "progress": progress })),
        }
    }

    /// 创建状态消息
    pub fn status(source: String, content: String, status: AgentStatus) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Status,
            content,
            metadata: Some(serde_json::to_value(&status).unwrap_or_default()),
        }
    }

    /// 创建错误消息
    pub fn error(source: String, content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            source,
            message_type: MessageType::Error,
            content,
            metadata: None,
        }
    }
}

/// Stdio 管道命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioCommand {
    /// 命令 ID
    pub command_id: String,
    /// 命令类型
    #[serde(rename = "command")]
    pub cmd_type: String,
    /// 命令参数
    pub args: Vec<String>,
    /// 工作目录
    pub cwd: Option<String>,
    /// 环境变量
    pub env: Option<std::collections::HashMap<String, String>>,
}

impl StdioCommand {
    /// 创建新命令
    pub fn new(cmd_type: String, args: Vec<String>) -> Self {
        Self {
            command_id: uuid::Uuid::new_v4().to_string(),
            cmd_type,
            args,
            cwd: None,
            env: None,
        }
    }
}

/// Stdio 输出行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioOutput {
    /// 标准输出
    pub stdout: Option<String>,
    /// 标准错误
    pub stderr: Option<String>,
    /// 退出码
    pub exit_code: Option<i32>,
    /// 时间戳
    pub timestamp: i64,
}

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum WebSocketMessage {
    /// 客户端连接
    Connect { session_id: String },
    /// 客户端断开
    Disconnect { session_id: String },
    /// 普通消息
    Message { data: serde_json::Value },
    /// 心跳消息
    Heartbeat { timestamp: i64 },
    /// 订阅 Agent 消息
    Subscribe { agent_id: String },
    /// 取消订阅
    Unsubscribe { agent_id: String },
}

// ========== VC-001: Agent 通信协议扩展 ==========

/// Issue/任务优先级
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Priority {
    /// 紧急
    Critical,
    /// 高优先级
    High,
    /// 中等优先级
    #[default]
    Medium,
    /// 低优先级
    Low,
}

/// Issue/任务状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueStatus {
    /// 待处理
    Todo,
    /// 进行中
    InProgress,
    /// 代码审查中
    InReview,
    /// 已完成
    Done,
    /// 已阻塞
    Blocked,
}

/// GitLab/GitHub Issue 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Issue 唯一标识
    pub issue_id: String,
    /// Issue 编号 (如 #123)
    pub issue_number: Option<String>,
    /// Issue 标题
    pub title: String,
    /// Issue 描述
    pub description: String,
    /// 优先级
    pub priority: Priority,
    /// 当前状态
    pub status: IssueStatus,
    /// 关联的 PRD 章节或需求 ID
    pub requirement_id: Option<String>,
    /// 预估工作量 (小时)
    pub estimated_hours: Option<f32>,
    /// 实际工作量 (小时)
    pub actual_hours: Option<f32>,
    /// 负责人 (Agent ID 或用户 ID)
    pub assignee: Option<String>,
    /// 标签列表
    pub labels: Vec<String>,
    /// 创建时间戳
    pub created_at: i64,
    /// 更新时间戳
    pub updated_at: i64,
    /// 截止时间戳
    pub due_date: Option<i64>,
}

impl Issue {
    /// 创建新的 Issue
    pub fn new(title: String, description: String, priority: Priority) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            issue_id: uuid::Uuid::new_v4().to_string(),
            issue_number: None,
            title,
            description,
            priority,
            status: IssueStatus::Todo,
            requirement_id: None,
            estimated_hours: None,
            actual_hours: None,
            assignee: None,
            labels: Vec::new(),
            created_at: now,
            updated_at: now,
            due_date: None,
        }
    }

    /// 设置 Issue 编号
    pub fn with_number(mut self, number: String) -> Self {
        self.issue_number = Some(number);
        self
    }

    /// 设置需求 ID
    pub fn with_requirement(mut self, req_id: String) -> Self {
        self.requirement_id = Some(req_id);
        self
    }

    /// 设置预估工时
    pub fn with_estimated_hours(mut self, hours: f32) -> Self {
        self.estimated_hours = Some(hours);
        self
    }

    /// 设置负责人
    pub fn assign_to(mut self, assignee: String) -> Self {
        self.assignee = Some(assignee);
        self
    }

    /// 添加标签
    pub fn add_label(mut self, label: String) -> Self {
        if !self.labels.contains(&label) {
            self.labels.push(label);
        }
        self
    }

    /// 更新状态
    pub fn update_status(&mut self, status: IssueStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// HITL (Human-in-the-Loop) 检查点类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointType {
    /// CP-001: 项目验证
    ProjectValidation,
    /// CP-002: 任务分解审查
    TaskDecompositionReview,
    /// CP-003 ~ CP-006: Issue 选择和完成审查
    IssueSelection,
    IssueCompletion,
    /// CP-007: MR 创建审查
    MRCreationReview,
    /// CP-008: 最终审查
    FinalReview,
}

impl std::fmt::Display for CheckpointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckpointType::ProjectValidation => write!(f, "CP-001: 项目验证"),
            CheckpointType::TaskDecompositionReview => write!(f, "CP-002: 任务分解审查"),
            CheckpointType::IssueSelection => write!(f, "CP-003~006: Issue 审查"),
            CheckpointType::IssueCompletion => write!(f, "CP-003~006: Issue 完成审查"),
            CheckpointType::MRCreationReview => write!(f, "CP-007: MR 创建审查"),
            CheckpointType::FinalReview => write!(f, "CP-008: 最终审查"),
        }
    }
}

/// HITL 检查点决策
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointDecision {
    /// 批准，继续执行
    Approve,
    /// 拒绝，需要修改
    Reject { reason: String },
    /// 暂停，等待更多信息
    Pause,
    /// 终止流程
    Abort,
}

/// HITL 检查点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointRequest {
    /// 检查点唯一标识
    pub checkpoint_id: String,
    /// 检查点类型
    pub checkpoint_type: CheckpointType,
    /// 关联的 Agent ID
    pub agent_id: String,
    /// 关联的 Issue ID (可选)
    pub issue_id: Option<String>,
    /// 检查点标题
    pub title: String,
    /// 检查点详细描述
    pub description: String,
    /// 需要审查的数据
    pub review_data: serde_json::Value,
    /// 建议的决策
    pub suggested_decision: Option<CheckpointDecision>,
}

impl CheckpointRequest {
    /// 创建新的检查点请求
    pub fn new(
        checkpoint_type: CheckpointType,
        agent_id: String,
        title: String,
        description: String,
        review_data: serde_json::Value,
    ) -> Self {
        Self {
            checkpoint_id: uuid::Uuid::new_v4().to_string(),
            checkpoint_type,
            agent_id,
            issue_id: None,
            title,
            description,
            review_data,
            suggested_decision: None,
        }
    }

    /// 设置关联的 Issue ID
    pub fn with_issue(mut self, issue_id: String) -> Self {
        self.issue_id = Some(issue_id);
        self
    }

    /// 设置建议决策
    pub fn with_suggestion(mut self, decision: CheckpointDecision) -> Self {
        self.suggested_decision = Some(decision);
        self
    }
}

/// HITL 检查点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointResponse {
    /// 对应的检查点 ID
    pub checkpoint_id: String,
    /// 用户决策
    pub decision: CheckpointDecision,
    /// 用户备注 (可选)
    pub comment: Option<String>,
    /// 响应时间戳
    pub responded_at: i64,
}

impl CheckpointResponse {
    /// 创建批准响应
    pub fn approve(checkpoint_id: String) -> Self {
        Self {
            checkpoint_id,
            decision: CheckpointDecision::Approve,
            comment: None,
            responded_at: chrono::Utc::now().timestamp(),
        }
    }

    /// 创建拒绝响应
    pub fn reject(checkpoint_id: String, reason: String) -> Self {
        let reason_clone = reason.clone();
        Self {
            checkpoint_id,
            decision: CheckpointDecision::Reject { reason },
            comment: Some(reason_clone),
            responded_at: chrono::Utc::now().timestamp(),
        }
    }
}

/// 质量门禁检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGateResult {
    /// 是否通过所有检查
    pub passed: bool,
    /// ESLint 检查结果
    pub eslint_passed: bool,
    /// TypeScript 编译检查
    pub typescript_passed: bool,
    /// 单元测试结果
    pub unit_tests_passed: bool,
    /// 错误详情
    pub errors: Vec<String>,
    /// 警告详情
    pub warnings: Vec<String>,
    /// 自动修复尝试次数
    pub auto_fix_attempts: u8,
}

impl QualityGateResult {
    /// 创建成功的检查结果
    pub fn success() -> Self {
        Self {
            passed: true,
            eslint_passed: true,
            typescript_passed: true,
            unit_tests_passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            auto_fix_attempts: 0,
        }
    }

    /// 创建失败的检查结果
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            passed: false,
            eslint_passed: true,
            typescript_passed: true,
            unit_tests_passed: true,
            errors,
            warnings: Vec::new(),
            auto_fix_attempts: 0,
        }
    }

    /// 添加 ESLint 错误
    pub fn with_eslint_errors(mut self, count: usize) -> Self {
        self.eslint_passed = count == 0;
        self.passed &= self.eslint_passed;
        if count > 0 {
            self.errors.push(format!("ESLint 发现 {} 个错误", count));
        }
        self
    }

    /// 添加 TypeScript 错误
    pub fn with_typescript_errors(mut self, count: usize) -> Self {
        self.typescript_passed = count == 0;
        self.passed &= self.typescript_passed;
        if count > 0 {
            self.errors
                .push(format!("TypeScript 发现 {} 个错误", count));
        }
        self
    }

    /// 添加单元测试失败
    pub fn with_test_failures(mut self, count: usize) -> Self {
        self.unit_tests_passed = count == 0;
        self.passed &= self.unit_tests_passed;
        if count > 0 {
            self.errors.push(format!("{} 个单元测试失败", count));
        }
        self
    }
}

/// Agent 会话状态 (用于持久化)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSessionState {
    /// 会话唯一标识
    pub session_id: String,
    /// 项目路径
    pub project_path: String,
    /// 会话开始时间戳
    pub started_at: i64,
    /// 最后更新时间戳
    pub last_updated: i64,
    /// 会话状态：active/paused/completed/failed
    pub status: String,
    /// 当前活跃的 Agent 列表
    pub active_agents: Vec<String>,
    /// 已完成的 Issue 列表
    pub completed_issues: Vec<String>,
    /// 待处理的 Issue 列表
    pub pending_issues: Vec<String>,
    /// 已通过的检查点列表
    pub passed_checkpoints: Vec<String>,
    /// 会话元数据
    pub metadata: Option<serde_json::Value>,
}

impl AgentSessionState {
    /// 创建新的会话状态
    pub fn new(session_id: String, project_path: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            session_id,
            project_path,
            started_at: now,
            last_updated: now,
            status: "active".to_string(),
            active_agents: Vec::new(),
            completed_issues: Vec::new(),
            pending_issues: Vec::new(),
            passed_checkpoints: Vec::new(),
            metadata: None,
        }
    }

    /// 更新最后活动时间戳
    pub fn touch(&mut self) {
        self.last_updated = chrono::Utc::now().timestamp();
    }

    /// 添加活跃的 Agent
    pub fn add_agent(&mut self, agent_id: String) {
        if !self.active_agents.contains(&agent_id) {
            self.active_agents.push(agent_id.clone());
            self.touch();
        }
    }

    /// 标记 Issue 为已完成
    pub fn complete_issue(&mut self, issue_id: String) {
        if let Some(pos) = self.pending_issues.iter().position(|x| x == &issue_id) {
            self.pending_issues.remove(pos);
            self.completed_issues.push(issue_id);
            self.touch();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_request_creation() {
        let request = AgentRequest::new(
            "agent-001".to_string(),
            "initialize".to_string(),
            serde_json::json!({"project": "test"}),
        );

        assert!(!request.request_id.is_empty());
        assert_eq!(request.agent_id, "agent-001");
        assert_eq!(request.action, "initialize");
    }

    #[test]
    fn test_agent_response_success() {
        let response = AgentResponse::success(
            "req-001".to_string(),
            Some(serde_json::json!({"result": "ok"})),
        );

        assert!(response.success);
        assert!(response.error.is_none());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_agent_response_error() {
        let response =
            AgentResponse::error("req-001".to_string(), "Something went wrong".to_string());

        assert!(!response.success);
        assert!(response.error.is_some());
        assert!(response.data.is_none());
    }

    #[test]
    fn test_agent_message_log() {
        let msg = AgentMessage::log(
            "agent-001".to_string(),
            "Starting initialization...".to_string(),
        );

        assert_eq!(msg.message_type, MessageType::Log);
        assert_eq!(msg.source, "agent-001");
        assert_eq!(msg.content, "Starting initialization...");
    }

    #[test]
    fn test_agent_message_progress() {
        let msg = AgentMessage::progress("agent-001".to_string(), "Processing...".to_string(), 0.5);

        assert_eq!(msg.message_type, MessageType::Progress);
        assert!(msg.metadata.is_some());
    }

    #[test]
    fn test_stdio_command_creation() {
        let cmd = StdioCommand::new("git".to_string(), vec!["init".to_string()]);

        assert!(!cmd.command_id.is_empty());
        assert_eq!(cmd.cmd_type, "git");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_websocket_message_serialize() {
        let msg = WebSocketMessage::Connect {
            session_id: "session-001".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"connect\""));
        assert!(json.contains("session-001"));
    }

    // ========== VC-001: 新增单元测试 ==========

    #[test]
    fn test_issue_creation() {
        let mut issue = Issue::new(
            "实现用户登录功能".to_string(),
            "需要实现基于 JWT 的用户登录".to_string(),
            Priority::High,
        );

        assert!(!issue.issue_id.is_empty());
        assert_eq!(issue.title, "实现用户登录功能");
        assert_eq!(issue.priority, Priority::High);
        assert_eq!(issue.status, IssueStatus::Todo);

        issue = issue
            .with_number("#123".to_string())
            .with_requirement("REQ-001".to_string())
            .with_estimated_hours(4.0)
            .assign_to("agent-coding-001".to_string())
            .add_label("feature".to_string());

        assert_eq!(issue.issue_number, Some("#123".to_string()));
        assert_eq!(issue.requirement_id, Some("REQ-001".to_string()));
        assert_eq!(issue.estimated_hours, Some(4.0));
        assert_eq!(issue.assignee, Some("agent-coding-001".to_string()));
        assert!(issue.labels.contains(&"feature".to_string()));
    }

    #[test]
    fn test_issue_status_update() {
        let mut issue = Issue::new(
            "Test Issue".to_string(),
            "Description".to_string(),
            Priority::Medium,
        );

        assert_eq!(issue.status, IssueStatus::Todo);

        // 等待一小段时间，确保时间戳不同
        std::thread::sleep(std::time::Duration::from_millis(10));

        issue.update_status(IssueStatus::InProgress);
        assert_eq!(issue.status, IssueStatus::InProgress);
        assert!(issue.updated_at >= issue.created_at);
    }

    #[test]
    fn test_checkpoint_request() {
        let checkpoint = CheckpointRequest::new(
            CheckpointType::TaskDecompositionReview,
            "agent-init-001".to_string(),
            "任务分解审查".to_string(),
            "请审查以下任务分解是否合理".to_string(),
            serde_json::json!({"tasks": ["task1", "task2"]}),
        );

        assert!(!checkpoint.checkpoint_id.is_empty());
        assert_eq!(
            checkpoint.checkpoint_type,
            CheckpointType::TaskDecompositionReview
        );
        assert_eq!(checkpoint.agent_id, "agent-init-001");
    }

    #[test]
    fn test_checkpoint_response() {
        let checkpoint_id = "cp-001".to_string();

        let approve_resp = CheckpointResponse::approve(checkpoint_id.clone());
        assert_eq!(approve_resp.decision, CheckpointDecision::Approve);
        assert!(approve_resp.comment.is_none());

        let reject_resp =
            CheckpointResponse::reject(checkpoint_id.clone(), "任务分解过于粗糙".to_string());
        assert_eq!(
            reject_resp.decision,
            CheckpointDecision::Reject {
                reason: "任务分解过于粗糙".to_string()
            }
        );
        assert!(reject_resp.comment.is_some());
    }

    #[test]
    fn test_quality_gate_result() {
        let result = QualityGateResult::success();
        assert!(result.passed);
        assert!(result.errors.is_empty());

        let failed_result = QualityGateResult::failure(vec!["ESLint error in file.ts".to_string()])
            .with_eslint_errors(2)
            .with_typescript_errors(1)
            .with_test_failures(3);

        assert!(!failed_result.passed);
        assert!(!failed_result.eslint_passed);
        assert!(!failed_result.typescript_passed);
        assert!(!failed_result.unit_tests_passed);
        assert_eq!(failed_result.errors.len(), 4);
    }

    #[test]
    fn test_agent_session_state() {
        let mut session =
            AgentSessionState::new("session-001".to_string(), "/path/to/project".to_string());

        assert_eq!(session.status, "active");
        assert!(session.active_agents.is_empty());

        session.add_agent("agent-001".to_string());
        assert_eq!(session.active_agents.len(), 1);

        session.pending_issues.push("issue-001".to_string());
        session.complete_issue("issue-001".to_string());
        assert!(session.pending_issues.is_empty());
        assert_eq!(session.completed_issues.len(), 1);
    }
}

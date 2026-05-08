//! HITL Checkpoint Manager for Native Coding Agent
//!
//! 管理 Human-in-the-Loop 检查点，支持用户审核和干预。

use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Checkpoint 类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckpointType {
    /// 代码生成后审核
    CodeGeneration,
    /// 依赖安装前审核
    DependencyInstallation,
    /// 测试执行前审核
    TestExecution,
    /// 提交前审核
    CommitReview,
}

impl std::fmt::Display for CheckpointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckpointType::CodeGeneration => write!(f, "code_generation"),
            CheckpointType::DependencyInstallation => write!(f, "dependency_installation"),
            CheckpointType::TestExecution => write!(f, "test_execution"),
            CheckpointType::CommitReview => write!(f, "commit_review"),
        }
    }
}

impl std::str::FromStr for CheckpointType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "code_generation" => Ok(CheckpointType::CodeGeneration),
            "dependency_installation" => Ok(CheckpointType::DependencyInstallation),
            "test_execution" => Ok(CheckpointType::TestExecution),
            "commit_review" => Ok(CheckpointType::CommitReview),
            _ => Err(format!("Unknown checkpoint type: {}", s)),
        }
    }
}

/// Checkpoint 状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckpointStatus {
    /// 等待用户决策
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
    /// 超时
    TimedOut,
}

impl std::fmt::Display for CheckpointStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckpointStatus::Pending => write!(f, "pending"),
            CheckpointStatus::Approved => write!(f, "approved"),
            CheckpointStatus::Rejected => write!(f, "rejected"),
            CheckpointStatus::TimedOut => write!(f, "timed_out"),
        }
    }
}

impl std::str::FromStr for CheckpointStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(CheckpointStatus::Pending),
            "approved" => Ok(CheckpointStatus::Approved),
            "rejected" => Ok(CheckpointStatus::Rejected),
            "timed_out" => Ok(CheckpointStatus::TimedOut),
            _ => Err(format!("Unknown checkpoint status: {}", s)),
        }
    }
}

/// 用户决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserDecision {
    Approve,
    Reject,
}

impl std::fmt::Display for UserDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDecision::Approve => write!(f, "approve"),
            UserDecision::Reject => write!(f, "reject"),
        }
    }
}

/// Checkpoint 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointData {
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 相关数据（JSON）
    pub payload: serde_json::Value,
    /// 超时时间（秒），默认 1800 秒（30 分钟）
    pub timeout_secs: u64,
}

/// Checkpoint 记录
#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: String,
    pub agent_id: String,
    pub story_id: String,
    pub checkpoint_type: CheckpointType,
    pub status: CheckpointStatus,
    pub data: CheckpointData,
    pub user_decision: Option<UserDecision>,
    pub user_feedback: Option<String>,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub expires_at: Option<String>,
}

/// Checkpoint Manager
pub struct CheckpointManager {
    db_path: PathBuf,
}

impl CheckpointManager {
    /// 创建新的 CheckpointManager
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    /// 获取数据库连接
    fn get_connection(&self) -> Result<Connection, String> {
        Connection::open(&self.db_path).map_err(|e| format!("Failed to open database: {}", e))
    }

    /// 创建 checkpoint
    pub fn create_checkpoint(
        &self,
        agent_id: &str,
        story_id: &str,
        checkpoint_type: CheckpointType,
        data: CheckpointData,
    ) -> Result<String, String> {
        let conn = self.get_connection()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let expires_at = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(data.timeout_secs as i64))
            .map(|t| t.to_rfc3339());

        let data_json = serde_json::to_string(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

        conn.execute(
            "INSERT INTO agent_checkpoints (id, agent_id, story_id, checkpoint_type, status, data, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                agent_id,
                story_id,
                checkpoint_type.to_string(),
                CheckpointStatus::Pending.to_string(),
                data_json,
                now,
                expires_at
            ],
        )
        .map_err(|e| format!("Failed to create checkpoint: {}", e))?;

        log::info!("Checkpoint created: {} (type: {})", id, checkpoint_type);
        Ok(id)
    }

    /// 解析 checkpoint（处理用户决策）
    pub fn resolve_checkpoint(
        &self,
        checkpoint_id: &str,
        decision: UserDecision,
        feedback: Option<String>,
    ) -> Result<(), String> {
        let conn = self.get_connection()?;
        let now = Utc::now().to_rfc3339();
        let status = match decision {
            UserDecision::Approve => CheckpointStatus::Approved,
            UserDecision::Reject => CheckpointStatus::Rejected,
        };

        let updated = conn
            .execute(
                "UPDATE agent_checkpoints 
                 SET status = ?1, user_decision = ?2, user_feedback = ?3, resolved_at = ?4
                 WHERE id = ?5 AND status = 'pending'",
                params![
                    status.to_string(),
                    decision.to_string(),
                    feedback,
                    now,
                    checkpoint_id
                ],
            )
            .map_err(|e| format!("Failed to resolve checkpoint: {}", e))?;

        if updated == 0 {
            return Err("Checkpoint not found or already resolved".to_string());
        }

        log::info!(
            "Checkpoint resolved: {} (decision: {:?})",
            checkpoint_id,
            decision
        );
        Ok(())
    }

    /// 获取 checkpoint
    pub fn get_checkpoint(&self, checkpoint_id: &str) -> Result<Option<Checkpoint>, String> {
        let conn = self.get_connection()?;

        let mut stmt = conn
            .prepare(
                "SELECT id, agent_id, story_id, checkpoint_type, status, data, 
                        user_decision, user_feedback, created_at, resolved_at, expires_at
                 FROM agent_checkpoints WHERE id = ?1",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let checkpoint = stmt
            .query_row(params![checkpoint_id], |row| {
                let id: String = row.get(0)?;
                let agent_id: String = row.get(1)?;
                let story_id: String = row.get(2)?;
                let checkpoint_type_str: String = row.get(3)?;
                let status_str: String = row.get(4)?;
                let data_json: String = row.get(5)?;
                let user_decision_str: Option<String> = row.get(6)?;
                let user_feedback: Option<String> = row.get(7)?;
                let created_at: String = row.get(8)?;
                let resolved_at: Option<String> = row.get(9)?;
                let expires_at: Option<String> = row.get(10)?;

                let checkpoint_type = checkpoint_type_str.parse::<CheckpointType>().map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        3,
                        "checkpoint_type".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?;

                let status = status_str.parse::<CheckpointStatus>().map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        4,
                        "status".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?;

                let data: CheckpointData = serde_json::from_str(&data_json).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                    )
                })?;

                let user_decision = user_decision_str
                    .map(|s| match s.as_str() {
                        "approve" => UserDecision::Approve,
                        "reject" => UserDecision::Reject,
                        _ => UserDecision::Approve, // 默认
                    })
                    .unwrap_or(UserDecision::Approve);

                Ok(Checkpoint {
                    id,
                    agent_id,
                    story_id,
                    checkpoint_type,
                    status,
                    data,
                    user_decision: Some(user_decision),
                    user_feedback,
                    created_at,
                    resolved_at,
                    expires_at,
                })
            })
            .map_err(|e| format!("Failed to query checkpoint: {}", e));

        // 处理 QueryReturnedNoRows 错误
        match checkpoint {
            Ok(cp) => Ok(Some(cp)),
            Err(e) if e.contains("Query returned no rows") => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// 获取待处理的 checkpoints
    pub fn get_pending_checkpoints(&self, agent_id: &str) -> Result<Vec<Checkpoint>, String> {
        let conn = self.get_connection()?;

        let mut stmt = conn
            .prepare(
                "SELECT id, agent_id, story_id, checkpoint_type, status, data, 
                        user_decision, user_feedback, created_at, resolved_at, expires_at
                 FROM agent_checkpoints 
                 WHERE agent_id = ?1 AND status = 'pending'
                 ORDER BY created_at ASC",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let checkpoints = stmt
            .query_map(params![agent_id], |row| {
                let id: String = row.get(0)?;
                let agent_id: String = row.get(1)?;
                let story_id: String = row.get(2)?;
                let checkpoint_type_str: String = row.get(3)?;
                let status_str: String = row.get(4)?;
                let data_json: String = row.get(5)?;
                let user_decision_str: Option<String> = row.get(6)?;
                let user_feedback: Option<String> = row.get(7)?;
                let created_at: String = row.get(8)?;
                let resolved_at: Option<String> = row.get(9)?;
                let expires_at: Option<String> = row.get(10)?;

                let checkpoint_type = checkpoint_type_str.parse::<CheckpointType>().map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        3,
                        "checkpoint_type".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?;

                let status = status_str.parse::<CheckpointStatus>().map_err(|_| {
                    rusqlite::Error::InvalidColumnType(
                        4,
                        "status".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?;

                let data: CheckpointData = serde_json::from_str(&data_json).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                    )
                })?;

                let user_decision = user_decision_str
                    .map(|s| match s.as_str() {
                        "approve" => UserDecision::Approve,
                        "reject" => UserDecision::Reject,
                        _ => UserDecision::Approve,
                    })
                    .unwrap_or(UserDecision::Approve);

                Ok(Checkpoint {
                    id,
                    agent_id,
                    story_id,
                    checkpoint_type,
                    status,
                    data,
                    user_decision: Some(user_decision),
                    user_feedback,
                    created_at,
                    resolved_at,
                    expires_at,
                })
            })
            .map_err(|e| format!("Failed to query checkpoints: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect checkpoints: {}", e))?;

        Ok(checkpoints)
    }

    /// 检查并标记超时的 checkpoints
    pub fn check_timeouts(&self) -> Result<usize, String> {
        let conn = self.get_connection()?;
        let now = Utc::now().to_rfc3339();

        let updated = conn
            .execute(
                "UPDATE agent_checkpoints 
                 SET status = 'timed_out', resolved_at = ?1
                 WHERE status = 'pending' AND expires_at IS NOT NULL AND expires_at <= ?1",
                params![now],
            )
            .map_err(|e| format!("Failed to check timeouts: {}", e))?;

        if updated > 0 {
            log::warn!("{} checkpoints timed out", updated);
        }

        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_checkpoint_type_conversion() {
        assert_eq!(
            CheckpointType::CodeGeneration.to_string(),
            "code_generation"
        );
        assert_eq!(
            "code_generation".parse::<CheckpointType>().unwrap(),
            CheckpointType::CodeGeneration
        );
    }

    #[test]
    fn test_checkpoint_status_conversion() {
        assert_eq!(CheckpointStatus::Pending.to_string(), "pending");
        assert_eq!("pending".parse::<CheckpointStatus>().unwrap(), CheckpointStatus::Pending);
    }

    #[test]
    fn test_user_decision_display() {
        assert_eq!(UserDecision::Approve.to_string(), "approve");
        assert_eq!(UserDecision::Reject.to_string(), "reject");
    }
}

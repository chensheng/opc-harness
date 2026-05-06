use serde::{Deserialize, Serialize};
use crate::db::Entity;
use rusqlite::Row;

/// 智能体日志模型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLog {
    pub id: String,
    pub agent_id: String,
    pub session_id: String,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub created_at: String,
}

impl Entity for AgentLog {
    fn table_name() -> &'static str { "agent_logs" }

    fn primary_key() -> &'static str { "id" }

    fn get_primary_key(&self) -> &str { &self.id }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(AgentLog {
            id: row.get("id")?,
            agent_id: row.get("agent_id")?,
            session_id: row.get("session_id")?,
            timestamp: row.get("timestamp")?,
            level: row.get("level")?,
            source: row.get("source")?,
            message: row.get("message")?,
            created_at: row.get("created_at")?,
        })
    }
}

impl AgentLog {
    /// 创建新的日志条目
    pub fn new(
        id: String,
        agent_id: String,
        session_id: String,
        timestamp: String,
        level: String,
        source: String,
        message: String,
    ) -> Self {
        Self {
            id,
            agent_id,
            session_id,
            timestamp,
            level,
            source,
            message,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

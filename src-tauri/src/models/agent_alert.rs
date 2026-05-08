use crate::db::Entity;
use rusqlite::Row;
use serde::{Deserialize, Serialize};

/// 智能体告警模型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentAlert {
    pub id: String,
    pub agent_id: String,
    pub level: String,
    pub alert_type: String,
    pub message: String,
    pub status: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
}

impl Entity for AgentAlert {
    fn table_name() -> &'static str {
        "agent_alerts"
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn get_primary_key(&self) -> &str {
        &self.id
    }

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(AgentAlert {
            id: row.get("id")?,
            agent_id: row.get("agent_id")?,
            level: row.get("level")?,
            alert_type: row.get("alert_type")?,
            message: row.get("message")?,
            status: row.get("status")?,
            created_at: row.get("created_at")?,
            resolved_at: row.get("resolved_at").ok(),
        })
    }
}

impl AgentAlert {
    /// 创建新的告警记录
    pub fn new(
        id: String,
        agent_id: String,
        level: String,
        alert_type: String,
        message: String,
    ) -> Self {
        Self {
            id,
            agent_id,
            level,
            alert_type,
            message,
            status: "active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved_at: None,
        }
    }

    /// 标记告警为已解决
    pub fn resolve(&mut self) {
        self.status = "resolved".to_string();
        self.resolved_at = Some(chrono::Utc::now().to_rfc3339());
    }
}

/// 告警级别
pub mod alert_level {
    pub const WARNING: &str = "warning";
    pub const CRITICAL: &str = "critical";
}

/// 告警类型
pub mod alert_type {
    pub const NO_RESPONSE: &str = "no_response";
    pub const ERROR_RATE: &str = "error_rate";
    pub const CPU_HIGH: &str = "cpu_high";
    pub const MEMORY_HIGH: &str = "memory_high";
}

//! Agent 告警数据访问层
//!
//! 提供告警的 CRUD 操作

use crate::db::Entity;
use crate::models::AgentAlert;
use rusqlite::{params, Connection};
use uuid::Uuid;

/// 获取告警存储库
pub fn get_alerts_repository(conn: &Connection) -> AgentAlertsRepository<'_> {
    AgentAlertsRepository { conn }
}

/// 告警存储库
pub struct AgentAlertsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentAlertsRepository<'a> {
    /// 插入单条告警记录
    pub fn insert(&self, alert: &AgentAlert) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO agent_alerts (id, agent_id, level, alert_type, message, status, created_at, resolved_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                alert.id,
                alert.agent_id,
                alert.level,
                alert.alert_type,
                alert.message,
                alert.status,
                alert.created_at,
                alert.resolved_at,
            ],
        )?;
        Ok(())
    }

    /// 按智能体 ID 查询告警
    pub fn get_by_agent_id(
        &self,
        agent_id: &str,
        status: Option<&str>,
        limit: i64,
    ) -> Result<Vec<AgentAlert>, rusqlite::Error> {
        let sql = match status {
            Some(_) => {
                "SELECT id, agent_id, level, alert_type, message, status, created_at, resolved_at
                 FROM agent_alerts
                 WHERE agent_id = ?1 AND status = ?2
                 ORDER BY created_at DESC
                 LIMIT ?3"
                    .to_string()
            }
            None => {
                "SELECT id, agent_id, level, alert_type, message, status, created_at, resolved_at
                 FROM agent_alerts
                 WHERE agent_id = ?1
                 ORDER BY created_at DESC
                 LIMIT ?2"
                    .to_string()
            }
        };

        let mut stmt = self.conn.prepare(&sql)?;

        let alerts: Result<Vec<AgentAlert>, _> = match status {
            Some(s) => stmt
                .query_map(params![agent_id, s, limit], |row| AgentAlert::from_row(row))?
                .collect(),
            None => stmt
                .query_map(params![agent_id, limit], |row| AgentAlert::from_row(row))?
                .collect(),
        };

        alerts
    }

    /// 查询所有活跃告警
    pub fn get_active_alerts(&self, limit: i64) -> Result<Vec<AgentAlert>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, level, alert_type, message, status, created_at, resolved_at
             FROM agent_alerts
             WHERE status = 'active'
             ORDER BY created_at DESC
             LIMIT ?1",
        )?;

        let alerts = stmt.query_map(params![limit], |row| AgentAlert::from_row(row))?;

        alerts.collect()
    }

    /// 标记告警为已解决
    pub fn resolve(&self, alert_id: &str) -> Result<usize, rusqlite::Error> {
        let resolved_at = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE agent_alerts SET status = 'resolved', resolved_at = ?2 WHERE id = ?1",
            params![alert_id, resolved_at],
        )
    }

    /// 清空智能体的所有告警
    pub fn clear_by_agent_id(&self, agent_id: &str) -> Result<usize, rusqlite::Error> {
        self.conn.execute(
            "DELETE FROM agent_alerts WHERE agent_id = ?1",
            params![agent_id],
        )
    }
}

/// 创建新的告警 ID
pub fn generate_alert_id() -> String {
    format!("alert-{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE agent_alerts (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                level TEXT NOT NULL,
                alert_type TEXT NOT NULL,
                message TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                resolved_at TEXT
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_insert_and_query() {
        let conn = setup_test_db();
        let repo = get_alerts_repository(&conn);

        let alert = AgentAlert::new(
            "alert-1".to_string(),
            "agent-1".to_string(),
            "warning".to_string(),
            "no_response".to_string(),
            "Agent has not responded for 5 minutes".to_string(),
        );

        repo.insert(&alert).unwrap();

        let alerts = repo.get_by_agent_id("agent-1", None, 10).unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, "no_response");
    }
}

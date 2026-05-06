//! Agent 追踪数据访问层
//! 
//! 提供追踪记录的 CRUD 操作

use crate::db::Entity;
use crate::models::AgentTrace;
use rusqlite::{Connection, params};
use uuid::Uuid;

/// 获取追踪存储库
pub fn get_traces_repository(conn: &Connection) -> AgentTracesRepository {
    AgentTracesRepository { conn }
}

/// 追踪存储库
pub struct AgentTracesRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentTracesRepository<'a> {
    /// 插入单条追踪记录
    pub fn insert(&self, trace: &AgentTrace) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO agent_traces (id, agent_id, session_id, event_type, timestamp, data, parent_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                trace.id,
                trace.agent_id,
                trace.session_id,
                trace.event_type,
                trace.timestamp,
                trace.data,
                trace.parent_id,
                trace.created_at,
            ],
        )?;
        Ok(())
    }

    /// 批量插入追踪记录
    pub fn insert_batch(&self, traces: &[AgentTrace]) -> Result<usize, rusqlite::Error> {
        if traces.is_empty() {
            return Ok(0);
        }

        let mut count = 0;
        let mut stmt = self.conn.prepare(
            "INSERT INTO agent_traces (id, agent_id, session_id, event_type, timestamp, data, parent_id, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )?;

        for trace in traces {
            let affected = stmt.execute(params![
                trace.id,
                trace.agent_id,
                trace.session_id,
                trace.event_type,
                trace.timestamp,
                trace.data,
                trace.parent_id,
                trace.created_at,
            ])?;
            count += affected;
        }

        Ok(count)
    }

    /// 按智能体 ID 查询追踪记录
    pub fn get_by_agent_id(
        &self,
        agent_id: &str,
        limit: i64,
    ) -> Result<Vec<AgentTrace>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, event_type, timestamp, data, parent_id, created_at
             FROM agent_traces
             WHERE agent_id = ?1
             ORDER BY timestamp ASC
             LIMIT ?2"
        )?;

        let traces = stmt.query_map(params![agent_id, limit], |row| {
            AgentTrace::from_row(row)
        })?;

        traces.collect()
    }

    /// 按事件类型过滤
    pub fn get_by_event_type(
        &self,
        agent_id: &str,
        event_type: &str,
        limit: i64,
    ) -> Result<Vec<AgentTrace>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, event_type, timestamp, data, parent_id, created_at
             FROM agent_traces
             WHERE agent_id = ?1 AND event_type = ?2
             ORDER BY timestamp ASC
             LIMIT ?3"
        )?;

        let traces = stmt.query_map(params![agent_id, event_type, limit], |row| {
            AgentTrace::from_row(row)
        })?;

        traces.collect()
    }

    /// 按父 ID 查询子追踪（用于思考树）
    pub fn get_children(&self, parent_id: &str) -> Result<Vec<AgentTrace>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, event_type, timestamp, data, parent_id, created_at
             FROM agent_traces
             WHERE parent_id = ?1
             ORDER BY timestamp ASC"
        )?;

        let traces = stmt.query_map(params![parent_id], |row| {
            AgentTrace::from_row(row)
        })?;

        traces.collect()
    }

    /// 清空智能体的所有追踪记录
    pub fn clear_by_agent_id(&self, agent_id: &str) -> Result<usize, rusqlite::Error> {
        self.conn.execute("DELETE FROM agent_traces WHERE agent_id = ?1", params![agent_id])
    }
}

/// 创建新的追踪 ID
pub fn generate_trace_id() -> String {
    format!("trace-{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        
        conn.execute(
            "CREATE TABLE agent_traces (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                data TEXT NOT NULL,
                parent_id TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).unwrap();

        conn
    }

    #[test]
    fn test_insert_and_query() {
        let conn = setup_test_db();
        let repo = get_traces_repository(&conn);

        let trace = AgentTrace::new_thought(
            "trace-1".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "Thinking about the task".to_string(),
            None,
        );

        repo.insert(&trace).unwrap();

        let traces = repo.get_by_agent_id("agent-1", 10).unwrap();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].event_type, "thought");
    }
}

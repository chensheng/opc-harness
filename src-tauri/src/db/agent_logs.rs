//! Agent 日志数据访问层
//! 
//! 提供日志的 CRUD 操作和批量写入优化

use crate::db::Entity;
use crate::models::AgentLog;
use rusqlite::{Connection, params};
use uuid::Uuid;

/// 获取日志存储库
pub fn get_logs_repository(conn: &Connection) -> AgentLogsRepository<'_> {
    AgentLogsRepository { conn }
}

/// 日志存储库
pub struct AgentLogsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> AgentLogsRepository<'a> {
    /// 插入单条日志
    pub fn insert(&self, log: &AgentLog) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO agent_logs (id, agent_id, session_id, timestamp, level, source, message, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                log.id,
                log.agent_id,
                log.session_id,
                log.timestamp,
                log.level,
                log.source,
                log.message,
                log.created_at,
            ],
        )?;
        Ok(())
    }

    /// 批量插入日志
    pub fn insert_batch(&self, logs: &[AgentLog]) -> Result<usize, rusqlite::Error> {
        if logs.is_empty() {
            return Ok(0);
        }

        let mut count = 0;
        let mut stmt = self.conn.prepare(
            "INSERT INTO agent_logs (id, agent_id, session_id, timestamp, level, source, message, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )?;

        for log in logs {
            let affected = stmt.execute(params![
                log.id,
                log.agent_id,
                log.session_id,
                log.timestamp,
                log.level,
                log.source,
                log.message,
                log.created_at,
            ])?;
            count += affected;
        }

        Ok(count)
    }

    /// 按智能体 ID 查询日志（分页）
    pub fn get_by_agent_id(
        &self,
        agent_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AgentLog>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, timestamp, level, source, message, created_at
             FROM agent_logs
             WHERE agent_id = ?1
             ORDER BY timestamp DESC
             LIMIT ?2 OFFSET ?3"
        )?;

        let logs = stmt.query_map(params![agent_id, limit, offset], |row| {
            AgentLog::from_row(row)
        })?;

        logs.collect()
    }

    /// 按智能体 ID 和时间范围查询日志
    pub fn get_by_agent_id_and_time_range(
        &self,
        agent_id: &str,
        start_time: &str,
        end_time: &str,
        limit: i64,
    ) -> Result<Vec<AgentLog>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, timestamp, level, source, message, created_at
             FROM agent_logs
             WHERE agent_id = ?1 AND timestamp BETWEEN ?2 AND ?3
             ORDER BY timestamp DESC
             LIMIT ?4"
        )?;

        let logs = stmt.query_map(params![agent_id, start_time, end_time, limit], |row| {
            AgentLog::from_row(row)
        })?;

        logs.collect()
    }

    /// 按级别过滤日志
    pub fn get_by_level(
        &self,
        agent_id: &str,
        level: &str,
        limit: i64,
    ) -> Result<Vec<AgentLog>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, timestamp, level, source, message, created_at
             FROM agent_logs
             WHERE agent_id = ?1 AND level = ?2
             ORDER BY timestamp DESC
             LIMIT ?3"
        )?;

        let logs = stmt.query_map(params![agent_id, level, limit], |row| {
            AgentLog::from_row(row)
        })?;

        logs.collect()
    }

    /// 搜索日志（按消息内容或来源）
    pub fn search(
        &self,
        agent_id: &str,
        keyword: &str,
        limit: i64,
    ) -> Result<Vec<AgentLog>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, session_id, timestamp, level, source, message, created_at
             FROM agent_logs
             WHERE agent_id = ?1 AND (message LIKE ?2 OR source LIKE ?2)
             ORDER BY timestamp DESC
             LIMIT ?3"
        )?;

        let pattern = format!("%{}%", keyword);
        let logs = stmt.query_map(params![agent_id, pattern, limit], |row| {
            AgentLog::from_row(row)
        })?;

        logs.collect()
    }

    /// 清空智能体的所有日志
    pub fn clear_by_agent_id(&self, agent_id: &str) -> Result<usize, rusqlite::Error> {
        self.conn.execute("DELETE FROM agent_logs WHERE agent_id = ?1", params![agent_id])
    }

    /// 获取日志统计
    pub fn get_stats(&self, agent_id: &str) -> Result<LogStats, rusqlite::Error> {
        let total: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1",
            params![agent_id],
            |row| row.get(0)
        )?;

        let info_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1 AND level = 'info'",
            params![agent_id],
            |row| row.get(0)
        )?;

        let warn_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1 AND level = 'warn'",
            params![agent_id],
            |row| row.get(0)
        )?;

        let error_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1 AND level = 'error'",
            params![agent_id],
            |row| row.get(0)
        )?;

        let debug_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1 AND level = 'debug'",
            params![agent_id],
            |row| row.get(0)
        )?;

        let success_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agent_logs WHERE agent_id = ?1 AND level = 'success'",
            params![agent_id],
            |row| row.get(0)
        )?;

        Ok(LogStats {
            total,
            info: info_count,
            warn: warn_count,
            error: error_count,
            debug: debug_count,
            success: success_count,
        })
    }
}

/// 日志统计
#[derive(Debug, Clone)]
pub struct LogStats {
    pub total: i64,
    pub info: i64,
    pub warn: i64,
    pub error: i64,
    pub debug: i64,
    pub success: i64,
}

/// 创建新的日志 ID
pub fn generate_log_id() -> String {
    format!("log-{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        
        // 创建测试表
        conn.execute(
            "CREATE TABLE agent_logs (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL,
                source TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).unwrap();

        conn
    }

    #[test]
    fn test_insert_and_query() {
        let conn = setup_test_db();
        let repo = get_logs_repository(&conn);

        let log = AgentLog::new(
            "log-1".to_string(),
            "agent-1".to_string(),
            "session-1".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
            "info".to_string(),
            "test".to_string(),
            "Test message".to_string(),
        );

        repo.insert(&log).unwrap();

        let logs = repo.get_by_agent_id("agent-1", 10, 0).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Test message");
    }

    #[test]
    fn test_batch_insert() {
        let conn = setup_test_db();
        let repo = get_logs_repository(&conn);

        let logs = vec![
            AgentLog::new(
                "log-1".to_string(),
                "agent-1".to_string(),
                "session-1".to_string(),
                "2024-01-01T00:00:00Z".to_string(),
                "info".to_string(),
                "test".to_string(),
                "Message 1".to_string(),
            ),
            AgentLog::new(
                "log-2".to_string(),
                "agent-1".to_string(),
                "session-1".to_string(),
                "2024-01-01T00:00:01Z".to_string(),
                "error".to_string(),
                "test".to_string(),
                "Message 2".to_string(),
            ),
        ];

        let count = repo.insert_batch(&logs).unwrap();
        assert_eq!(count, 2);

        let stats = repo.get_stats("agent-1").unwrap();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.info, 1);
        assert_eq!(stats.error, 1);
    }
}

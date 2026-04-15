use crate::models::AgentSession;
use chrono::Utc;
use rusqlite::{Connection, Result};

/// 创建 Agent Session
pub fn create_agent_session(conn: &Connection, session: &AgentSession) -> Result<()> {
    conn.execute(
        "INSERT INTO agent_sessions (session_id, agent_id, agent_type, project_id, status, phase, created_at, updated_at, stdio_channel_id, registered_to_daemon, metadata)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &session.session_id,
            &session.agent_id,
            &session.agent_type,
            &session.project_id,
            &session.status,
            &session.phase,
            &session.created_at,
            &session.updated_at,
            &session.stdio_channel_id,
            &(if session.registered_to_daemon { "1".to_string() } else { "0".to_string() }),
            &session.metadata,
        ),
    )?;
    Ok(())
}

/// 获取项目的所有 Sessions
pub fn get_sessions_by_project(conn: &Connection, project_id: &str) -> Result<Vec<AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions WHERE project_id = ?1 ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([project_id], |row| {
        Ok(AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_id: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, String>(9)? == "1",
            metadata: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 获取所有 Sessions
pub fn get_all_agent_sessions(conn: &Connection) -> Result<Vec<AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([], |row| {
        Ok(AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_id: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, String>(9)? == "1",
            metadata: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 获取单个 Agent Session
pub fn get_agent_session_by_id(conn: &Connection, agent_id: &str) -> Result<Option<AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions WHERE agent_id = ?1")?;
    let mut rows = stmt.query_map([agent_id], |row| {
        Ok(AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_id: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, String>(9)? == "1",
            metadata: row.get(10)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 按 Session ID 获取 Agent Session
pub fn get_agent_session_by_session_id(conn: &Connection, session_id: &str) -> Result<Option<AgentSession>> {
    let mut stmt = conn.prepare("SELECT * FROM agent_sessions WHERE session_id = ?1")?;
    let mut rows = stmt.query_map([session_id], |row| {
        Ok(AgentSession {
            session_id: row.get(0)?,
            agent_id: row.get(1)?,
            agent_type: row.get(2)?,
            project_id: row.get(3)?,
            status: row.get(4)?,
            phase: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            stdio_channel_id: row.get(8)?,
            registered_to_daemon: row.get::<_, String>(9)? == "1",
            metadata: row.get(10)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新 Agent Session 状态
pub fn update_agent_session_status(conn: &Connection, agent_id: &str, status: &str, phase: &str) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE agent_sessions 
         SET status = ?1, phase = ?2, updated_at = ?3
         WHERE agent_id = ?4",
        [status, phase, &updated_at, agent_id],
    )?;
    Ok(())
}

/// 更新 Agent Session 完整信息
pub fn update_agent_session(conn: &Connection, session: &AgentSession) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE agent_sessions 
         SET session_id = ?2, agent_type = ?3, project_id = ?4, status = ?5, phase = ?6, 
             updated_at = ?7, stdio_channel_id = ?8, registered_to_daemon = ?9, metadata = ?10
         WHERE agent_id = ?1",
        [
            &session.agent_id,
            &session.session_id,
            &session.agent_type,
            &session.project_id,
            &session.status,
            &session.phase,
            &updated_at,
            &session.stdio_channel_id.clone().unwrap_or_default(),
            &(if session.registered_to_daemon { "1".to_string() } else { "0".to_string() }),
            &session.metadata.clone().unwrap_or_default(),
        ],
    )?;
    Ok(())
}

/// 删除 Agent Session
pub fn delete_agent_session(conn: &Connection, agent_id: &str) -> Result<()> {
    conn.execute("DELETE FROM agent_sessions WHERE agent_id = ?1", [agent_id])?;
    Ok(())
}

use crate::models::CLISession;
use rusqlite::{Connection, Result};

/// 创建 CLI 会话
pub fn create_cli_session(conn: &Connection, session: &CLISession) -> Result<()> {
    conn.execute(
        "INSERT INTO cli_sessions (id, tool_type, project_path, created_at)
         VALUES (?1, ?2, ?3, ?4)",
        [
            &session.id,
            &session.tool_type,
            &session.project_path,
            &session.created_at,
        ],
    )?;
    Ok(())
}

/// 获取所有 CLI 会话
pub fn get_all_cli_sessions(conn: &Connection) -> Result<Vec<CLISession>> {
    let mut stmt = conn.prepare("SELECT * FROM cli_sessions ORDER BY created_at DESC")?;
    let sessions = stmt.query_map([], |row| {
        Ok(CLISession {
            id: row.get(0)?,
            tool_type: row.get(1)?,
            project_path: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut result = Vec::new();
    for session in sessions {
        result.push(session?);
    }
    Ok(result)
}

/// 获取单个 CLI 会话
pub fn get_cli_session_by_id(conn: &Connection, id: &str) -> Result<Option<CLISession>> {
    let mut stmt = conn.prepare("SELECT * FROM cli_sessions WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(CLISession {
            id: row.get(0)?,
            tool_type: row.get(1)?,
            project_path: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 删除 CLI 会话
pub fn delete_cli_session(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM cli_sessions WHERE id = ?1", [id])?;
    Ok(())
}

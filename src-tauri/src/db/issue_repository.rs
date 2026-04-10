use crate::models::Issue;
use chrono::Utc;
use rusqlite::{Connection, Result};

/// 创建 Issue
pub fn create_issue(conn: &Connection, issue: &Issue) -> Result<()> {
    conn.execute(
        "INSERT INTO issues (id, project_id, milestone_id, title, description, issue_type, priority, status, assignee, parent_issue_id, order_index, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        (
            &issue.id,
            &issue.project_id,
            &issue.milestone_id,
            &issue.title,
            &issue.description,
            &issue.issue_type,
            &issue.priority,
            &issue.status,
            &issue.assignee,
            &issue.parent_issue_id,
            &issue.order,
            &issue.created_at,
            &issue.updated_at,
        ),
    )?;
    Ok(())
}

/// 获取项目的所有 Issues
pub fn get_issues_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE project_id = ?1 ORDER BY order_index")?;
    let issues = stmt.query_map([project_id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for issue in issues {
        result.push(issue?);
    }
    Ok(result)
}

/// 按里程碑获取 Issues
pub fn get_issues_by_milestone(conn: &Connection, milestone_id: &str) -> Result<Vec<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE milestone_id = ?1 ORDER BY order_index")?;
    let issues = stmt.query_map([milestone_id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for issue in issues {
        result.push(issue?);
    }
    Ok(result)
}

/// 获取单个 Issue
pub fn get_issue_by_id(conn: &Connection, id: &str) -> Result<Option<Issue>> {
    let mut stmt = conn.prepare("SELECT * FROM issues WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Issue {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            issue_type: row.get(5)?,
            priority: row.get(6)?,
            status: row.get(7)?,
            assignee: row.get(8)?,
            parent_issue_id: row.get(9)?,
            order: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新 Issue 信息
pub fn update_issue(conn: &Connection, issue: &Issue) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE issues 
         SET title = ?2, description = ?3, issue_type = ?4, priority = ?5, 
             status = ?6, assignee = ?7, parent_issue_id = ?8, order_index = ?9, updated_at = ?10
         WHERE id = ?1",
        (
            &issue.id,
            &issue.title,
            &issue.description,
            &issue.issue_type,
            &issue.priority,
            &issue.status,
            &issue.assignee,
            &issue.parent_issue_id,
            &issue.order,
            &updated_at,
        ),
    )?;
    Ok(())
}

/// 删除 Issue
pub fn delete_issue(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM issues WHERE id = ?1", [id])?;
    Ok(())
}

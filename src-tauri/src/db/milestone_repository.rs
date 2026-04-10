use crate::models::Milestone;
use chrono::Utc;
use rusqlite::{Connection, Result};

/// 创建里程碑
pub fn create_milestone(conn: &Connection, milestone: &Milestone) -> Result<()> {
    conn.execute(
        "INSERT INTO milestones (id, project_id, title, description, order_index, status, due_date, completed_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &milestone.id,
            &milestone.project_id,
            &milestone.title,
            &milestone.description,
            &milestone.order,
            &milestone.status,
            &milestone.due_date,
            &milestone.completed_at,
            &milestone.created_at,
            &milestone.updated_at,
        ),
    )?;
    Ok(())
}

/// 获取项目的所有里程碑
pub fn get_milestones_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Milestone>> {
    let mut stmt = conn.prepare("SELECT * FROM milestones WHERE project_id = ?1 ORDER BY order_index")?;
    let milestones = stmt.query_map([project_id], |row| {
        Ok(Milestone {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            order: row.get(4)?,
            status: row.get(5)?,
            due_date: row.get(6)?,
            completed_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut result = Vec::new();
    for milestone in milestones {
        result.push(milestone?);
    }
    Ok(result)
}

/// 获取单个里程碑
pub fn get_milestone_by_id(conn: &Connection, id: &str) -> Result<Option<Milestone>> {
    let mut stmt = conn.prepare("SELECT * FROM milestones WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Milestone {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            order: row.get(4)?,
            status: row.get(5)?,
            due_date: row.get(6)?,
            completed_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新里程碑信息
pub fn update_milestone(conn: &Connection, milestone: &Milestone) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE milestones 
         SET title = ?2, description = ?3, order_index = ?4, status = ?5, 
             due_date = ?6, completed_at = ?7, updated_at = ?8
         WHERE id = ?1",
        (
            &milestone.id,
            &milestone.title,
            &milestone.description,
            &milestone.order,
            &milestone.status,
            &milestone.due_date,
            &milestone.completed_at,
            &updated_at,
        ),
    )?;
    Ok(())
}

/// 删除里程碑
pub fn delete_milestone(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM milestones WHERE id = ?1", [id])?;
    Ok(())
}

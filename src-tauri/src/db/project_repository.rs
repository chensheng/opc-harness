use crate::models::Project;
use chrono::Utc;
use rusqlite::{Connection, Result};

/// 创建新项目
pub fn create_project(conn: &Connection, project: &Project) -> Result<()> {
    conn.execute(
        "INSERT INTO projects (id, name, description, status, progress, created_at, updated_at, idea, prd, user_personas, competitor_analysis)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &project.id,
            &project.name,
            &project.description,
            &project.status,
            project.progress,
            &project.created_at,
            &project.updated_at,
            // 使用 as_deref() 将 Option<String> 转换为 Option<&str>
            // rusqlite 可以正确处理 Option<&str> (NULL 或字符串)
            project.idea.as_deref(),
            project.prd.as_deref(),
            project.user_personas.as_deref(),
            project.competitor_analysis.as_deref(),
        ),
    )?;
    Ok(())
}

/// 获取所有项目
pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT * FROM projects ORDER BY updated_at DESC")?;
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            idea: row.get(7)?,
            prd: row.get(8)?,
            user_personas: row.get(9)?,
            competitor_analysis: row.get(10)?,
        })
    })?;

    let mut result = Vec::new();
    for project in projects {
        result.push(project?);
    }
    Ok(result)
}

/// 获取单个项目
pub fn get_project_by_id(conn: &Connection, id: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare("SELECT * FROM projects WHERE id = ?1")?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            idea: row.get(7)?,
            prd: row.get(8)?,
            user_personas: row.get(9)?,
            competitor_analysis: row.get(10)?,
        })
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 更新项目信息
pub fn update_project(conn: &Connection, project: &Project) -> Result<()> {
    let updated_at = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects 
         SET name = ?2, description = ?3, status = ?4, progress = ?5, updated_at = ?6,
             idea = ?7, prd = ?8, user_personas = ?9, competitor_analysis = ?10
         WHERE id = ?1",
        (
            &project.id,
            &project.name,
            &project.description,
            &project.status,
            project.progress,
            &updated_at,
            // 使用 as_deref() 将 Option<String> 转换为 Option<&str>
            // rusqlite 可以正确处理 Option<&str> (NULL 或字符串)
            project.idea.as_deref(),
            project.prd.as_deref(),
            project.user_personas.as_deref(),
            project.competitor_analysis.as_deref(),
        ),
    )?;
    Ok(())
}

/// 删除项目
pub fn delete_project(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    Ok(())
}

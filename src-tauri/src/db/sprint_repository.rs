use crate::db::Entity;
use crate::models::Sprint;
use rusqlite::{Connection, Result};

/// 批量创建或更新Sprint（Upsert）
pub fn upsert_sprints(conn: &Connection, project_id: &str, sprints: &[Sprint]) -> Result<()> {
    println!("[DB::upsert_sprints] Starting upsert for project_id: {}, count: {}", 
             project_id, sprints.len());
    
    // 使用事务确保原子性
    let tx = conn.unchecked_transaction()?;
    
    // 先删除该项目的旧Sprint
    let deleted = tx.execute(
        "DELETE FROM sprints WHERE project_id = ?1",
        [project_id],
    )?;
    println!("[DB::upsert_sprints] Deleted {} old sprints", deleted);

    // 批量插入新Sprint
    let mut inserted_count = 0;
    for sprint in sprints {
        tx.execute(
            "INSERT INTO sprints (
                id, project_id, name, goal, start_date, end_date, status,
                total_story_points, completed_story_points, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                sprint.id,
                sprint.project_id,
                sprint.name,
                sprint.goal,
                sprint.start_date,
                sprint.end_date,
                sprint.status,
                sprint.total_story_points.unwrap_or(0),
                sprint.completed_story_points.unwrap_or(0),
                sprint.created_at,
                sprint.updated_at
            ],
        )?;
        inserted_count += 1;
    }
    println!("[DB::upsert_sprints] Inserted {} new sprints", inserted_count);

    tx.commit()?;
    println!("[DB::upsert_sprints] Transaction committed successfully");
    Ok(())
}

/// 获取项目的所有Sprint
pub fn get_sprints_by_project(conn: &Connection, project_id: &str) -> Result<Vec<Sprint>> {
    println!("[DB::get_sprints_by_project] Querying for project_id: {}", project_id);
    
    let mut stmt = conn.prepare(
        "SELECT * FROM sprints WHERE project_id = ?1 ORDER BY start_date DESC"
    )?;
    
    let sprints = stmt.query_map([project_id], |row| {
        Sprint::from_row(row)
    })?;

    let mut result = Vec::new();
    for sprint_result in sprints {
        result.push(sprint_result?);
    }
    
    println!("[DB::get_sprints_by_project] Retrieved {} sprints", result.len());

    Ok(result)
}

/// 删除单个Sprint
pub fn delete_sprint(conn: &Connection, sprint_id: &str) -> Result<usize> {
    println!("[DB::delete_sprint] Deleting sprint_id: {}", sprint_id);
    
    let deleted = conn.execute(
        "DELETE FROM sprints WHERE id = ?1",
        [sprint_id],
    )?;
    
    println!("[DB::delete_sprint] Deleted {} sprint(s)", deleted);
    Ok(deleted)
}

use crate::db::Entity;
use crate::models::{Sprint, UserStory};
use chrono::Utc;
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

/// 获取当前活跃 Sprint（基于状态）
pub fn get_active_sprint(conn: &Connection) -> Result<Option<Sprint>> {
    let now = Utc::now().to_rfc3339();
    
    log::info!("[DB::get_active_sprint] Current time (UTC): {}", now);
    log::info!("[DB::get_active_sprint] Querying for active sprint with condition: status = 'active'");
    
    // 先查询所有 sprints 以便调试
    let mut debug_stmt = conn.prepare("SELECT id, name, start_date, end_date, status FROM sprints ORDER BY created_at DESC")?;
    let mut debug_rows = debug_stmt.query([])?;
    log::info!("[DB::get_active_sprint] All sprints in database:");
    while let Some(row) = debug_rows.next()? {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let start_date: String = row.get(2)?;
        let end_date: String = row.get(3)?;
        let status: String = row.get(4)?;
        log::info!(
            "[DB::get_active_sprint]   - ID: {}, Name: {}, Start: {}, End: {}, Status: {}",
            id, name, start_date, end_date, status
        );
    }
    
    // 简化查询：只检查状态为 'active' 的 Sprint，不检查日期范围
    // 这样可以避免因日期设置错误导致智能体无法工作
    let mut stmt = conn.prepare(
        "SELECT * FROM sprints 
         WHERE status = 'active'
         ORDER BY created_at DESC 
         LIMIT 1"
    )?;
    
    let mut rows = stmt.query([])?;
    
    if let Some(row) = rows.next()? {
        let sprint = Sprint::from_row(row)?;
        log::info!("[DB::get_active_sprint] Found active sprint: {} (ID: {}, Start: {}, End: {})", 
                   sprint.name, sprint.id, sprint.start_date, sprint.end_date);
        Ok(Some(sprint))
    } else {
        log::warn!("[DB::get_active_sprint] No active sprint found (status != 'active')");
        Ok(None)
    }
}

/// 获取指定 Sprint 下待执行的用户故事（按优先级排序）
pub fn get_pending_stories_by_sprint(conn: &Connection, sprint_id: &str) -> Result<Vec<UserStory>> {
    println!("[DB::get_pending_stories_by_sprint] Querying pending stories for sprint_id: {}", sprint_id);
    
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories 
         WHERE sprint_id = ?1 AND status IN ('draft', 'refined', 'approved')
         ORDER BY 
            CASE priority 
                WHEN 'P0' THEN 1 
                WHEN 'P1' THEN 2 
                WHEN 'P2' THEN 3 
                WHEN 'P3' THEN 4 
                ELSE 5 
            END ASC,
            story_number ASC"
    )?;
    
    let stories = stmt.query_map([sprint_id], |row| {
        UserStory::from_row(row)
    })?;

    let mut result = Vec::new();
    for story_result in stories {
        result.push(story_result?);
    }
    
    println!("[DB::get_pending_stories_by_sprint] Retrieved {} pending stories", result.len());
    Ok(result)
}

/// 锁定用户故事（乐观锁，用于 Agent 竞争）
pub fn lock_user_story(conn: &Connection, story_id: &str, agent_id: &str) -> Result<bool> {
    let now = Utc::now().to_rfc3339();
    
    // 使用原子 UPDATE 操作，只有当故事处于可执行状态且未被锁定时才能成功
    // 注意：这里假设如果 locked_at 超过30分钟则视为锁失效，允许重新抢占
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = 'in_development',
             assigned_agent = ?1,
             locked_at = ?2,
             started_at = ?2,
             updated_at = ?2
         WHERE id = ?3 
           AND status IN ('draft', 'refined', 'approved')
           AND (assigned_agent IS NULL OR locked_at IS NULL 
                OR (locked_at < datetime('now', '-30 minutes')))",
        rusqlite::params![agent_id, now, story_id],
    )?;
    
    let success = updated > 0;
    println!("[DB::lock_user_story] Story {} lock attempt by agent {}: {}", 
             story_id, agent_id, if success { "SUCCESS" } else { "FAILED" });
    
    Ok(success)
}

/// 解锁用户故事（释放锁，允许其他 Agent 重新获取）
pub fn unlock_user_story(conn: &Connection, story_id: &str) -> Result<()> {
    conn.execute(
        "UPDATE user_stories 
         SET assigned_agent = NULL,
             locked_at = NULL,
             updated_at = ?1
         WHERE id = ?2",
        rusqlite::params![Utc::now().to_rfc3339(), story_id],
    )?;
    
    println!("[DB::unlock_user_story] Story {} unlocked", story_id);
    Ok(())
}

/// 更新用户故事状态
pub fn update_user_story_status(conn: &Connection, story_id: &str, status: &str) -> Result<usize> {
    let now = Utc::now().to_rfc3339();
    
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = ?1, updated_at = ?2
         WHERE id = ?3",
        rusqlite::params![status, now, story_id],
    )?;
    
    println!("[DB::update_user_story_status] Updated story {} to status: {}", story_id, status);
    Ok(updated)
}

/// 标记用户故事完成
pub fn complete_user_story(conn: &Connection, story_id: &str) -> Result<usize> {
    let now = Utc::now().to_rfc3339();
    
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = 'completed',
             completed_at = ?1,
             updated_at = ?1
         WHERE id = ?2",
        rusqlite::params![now, story_id],
    )?;
    
    println!("[DB::complete_user_story] Completed story: {}", story_id);
    Ok(updated)
}

/// 标记用户故事失败
pub fn fail_user_story(conn: &Connection, story_id: &str, error_message: &str) -> Result<usize> {
    let now = Utc::now().to_rfc3339();
    
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = 'failed',
             failed_at = ?1,
             error_message = ?2,
             retry_count = retry_count + 1,
             updated_at = ?1
         WHERE id = ?3",
        rusqlite::params![now, error_message, story_id],
    )?;
    
    println!("[DB::fail_user_story] Failed story: {} - {}", story_id, error_message);
    Ok(updated)
}

/// 根据 ID 查询单个用户故事的详细信息
pub fn get_user_story_by_id(conn: &Connection, story_id: &str) -> Result<Option<UserStory>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query([story_id])?;
    
    if let Some(row) = rows.next()? {
        let story = UserStory::from_row(row)?;
        Ok(Some(story))
    } else {
        Ok(None)
    }
}

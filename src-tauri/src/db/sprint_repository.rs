use crate::db::Entity;
use crate::models::{Sprint, UserStory, UserStoryRetryHistory};
use chrono::Utc;
use rusqlite::{Connection, Result};

/// UserStory 状态常量定义（统一状态管理）
mod user_story_status {
    /// 待处理状态（可被 Agent 选取）
    pub const PENDING: &[&str] = &["draft", "refined", "approved"];
    
    /// 开发中状态
    pub const IN_DEVELOPMENT: &str = "in_development";
    
    /// 完成状态
    pub const COMPLETED: &str = "completed";
    
    /// 失败状态
    pub const FAILED: &str = "failed";
}

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

/// 获取指定项目的活跃 Sprint（修复：添加项目隔离）
pub fn get_active_sprint(conn: &Connection, project_id: &str) -> Result<Option<Sprint>> {
    let now = Utc::now().to_rfc3339();
    
    log::info!("[DB::get_active_sprint] Current time (UTC): {}", now);
    log::info!("[DB::get_active_sprint] Querying for active sprint in project: {}", project_id);
    
    // 先查询该项目的所有 sprints 以便调试
    let mut debug_stmt = conn.prepare(
        "SELECT id, name, start_date, end_date, status FROM sprints WHERE project_id = ?1 ORDER BY created_at DESC"
    )?;
    let mut debug_rows = debug_stmt.query([project_id])?;
    log::info!("[DB::get_active_sprint] All sprints in project {}:", project_id);
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
    // ✅ 关键修复：添加 project_id 过滤，实现项目隔离
    let mut stmt = conn.prepare(
        "SELECT * FROM sprints 
         WHERE project_id = ?1 AND status = 'active'
         ORDER BY created_at DESC 
         LIMIT 1"
    )?;
    
    let mut rows = stmt.query([project_id])?;
    
    if let Some(row) = rows.next()? {
        let sprint = Sprint::from_row(row)?;
        log::info!("[DB::get_active_sprint] Found active sprint in project {}: {} (ID: {}, Start: {}, End: {})", 
                   project_id, sprint.name, sprint.id, sprint.start_date, sprint.end_date);
        Ok(Some(sprint))
    } else {
        log::warn!("[DB::get_active_sprint] No active sprint found in project {} (status != 'active')", project_id);
        Ok(None)
    }
}

/// 获取指定 Sprint 下待执行的用户故事（按优先级排序）
pub fn get_pending_stories_by_sprint(conn: &Connection, sprint_id: &str, project_id: &str) -> Result<Vec<UserStory>> {
    println!("[DB::get_pending_stories_by_sprint] Querying pending stories for sprint_id: {}, project_id: {}", sprint_id, project_id);
    
    // ✅ 使用统一的状态常量，并添加项目隔离验证
    let status_list = user_story_status::PENDING.join("','");
    let query = format!(
        "SELECT * FROM user_stories 
         WHERE sprint_id = ?1 
           AND project_id = ?2
           AND status IN ('{}')
         ORDER BY 
            CASE priority 
                WHEN 'P0' THEN 1 
                WHEN 'P1' THEN 2 
                WHEN 'P2' THEN 3 
                WHEN 'P3' THEN 4 
                ELSE 5 
            END ASC,
            story_number ASC",
        status_list
    );
    
    let mut stmt = conn.prepare(&query)?;
    
    let stories = stmt.query_map(rusqlite::params![sprint_id, project_id], |row| {
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
pub fn lock_user_story(conn: &Connection, story_id: &str, agent_id: &str, lock_timeout_minutes: u64) -> Result<bool> {
    let now = Utc::now().to_rfc3339();
    
    // ✅ 使用统一的状态常量
    let status_list = user_story_status::PENDING.join("','");
    let query = format!(
        "UPDATE user_stories 
         SET status = '{}',
             assigned_agent = ?1,
             locked_at = ?2,
             started_at = ?2,
             updated_at = ?2
         WHERE id = ?3 
           AND status IN ('{}')
           AND (assigned_agent IS NULL OR locked_at IS NULL 
                OR (locked_at < datetime('now', '-{} minutes')))",
        user_story_status::IN_DEVELOPMENT,
        status_list,
        lock_timeout_minutes
    );
    
    // 使用原子 UPDATE 操作，只有当故事处于可执行状态且未被锁定时才能成功
    // 注意：这里假设如果 locked_at 超过配置的时间则视为锁失效，允许重新抢占
    let updated = conn.execute(&query, rusqlite::params![agent_id, now, story_id])?;
    
    let success = updated > 0;
    
    if !success {
        // 锁定失败时，查询当前锁定的 Agent 信息
        if let Ok(mut stmt) = conn.prepare(
            "SELECT assigned_agent, locked_at FROM user_stories WHERE id = ?1"
        ) {
            match stmt.query_row([story_id], |row| {
                Ok((
                    row.get::<_, Option<String>>(0)?,
                    row.get::<_, Option<String>>(1)?
                ))
            }) {
                Ok((current_agent, locked_at)) => {
                    println!(
                        "[DB::lock_user_story] Story {} lock FAILED - Already locked by agent {:?} at {:?}",
                        story_id, current_agent, locked_at
                    );
                }
                Err(_) => {
                    println!(
                        "[DB::lock_user_story] Story {} lock FAILED - Story not found or in invalid state",
                        story_id
                    );
                }
            }
        }
    } else {
        println!(
            "[DB::lock_user_story] Story {} lock SUCCESS by agent {} (timeout: {}min)",
            story_id, agent_id, lock_timeout_minutes
        );
    }
    
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
    
    // ✅ 使用统一的状态常量
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = ?1,
             completed_at = ?2,
             updated_at = ?2
         WHERE id = ?3",
        rusqlite::params![user_story_status::COMPLETED, now, story_id],
    )?;
    
    println!("[DB::complete_user_story] Completed story: {}", story_id);
    Ok(updated)
}

/// 标记用户故事失败
pub fn fail_user_story(conn: &Connection, story_id: &str, error_message: &str) -> Result<usize> {
    let now = Utc::now().to_rfc3339();
    
    // ✅ 使用统一的状态常量
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = ?1,
             failed_at = ?2,
             error_message = ?3,
             retry_count = retry_count + 1,
             updated_at = ?2
         WHERE id = ?4",
        rusqlite::params![user_story_status::FAILED, now, error_message, story_id],
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

// ==================== 重试引擎相关函数 ====================

/// 创建重试历史记录
pub fn create_retry_history_record(
    conn: &Connection,
    history: &UserStoryRetryHistory,
) -> Result<usize> {
    let updated = conn.execute(
        "INSERT INTO user_story_retry_history (
            id, user_story_id, retry_number, triggered_at, error_message,
            error_type, decision, next_retry_at, completed_at, result, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            history.id,
            history.user_story_id,
            history.retry_number,
            history.triggered_at,
            history.error_message,
            history.error_type,
            history.decision,
            history.next_retry_at,
            history.completed_at,
            history.result,
            history.created_at,
        ],
    )?;
    
    println!("[DB::create_retry_history_record] Created retry history for story: {}", history.user_story_id);
    Ok(updated)
}

/// 更新重试历史结果
pub fn update_retry_history_result(
    conn: &Connection,
    history_id: &str,
    result: &str,
    completed_at: &str,
) -> Result<usize> {
    let updated = conn.execute(
        "UPDATE user_story_retry_history 
         SET result = ?1,
             completed_at = ?2
         WHERE id = ?3",
        rusqlite::params![result, completed_at, history_id],
    )?;
    
    println!("[DB::update_retry_history_result] Updated retry history: {} with result: {}", history_id, result);
    Ok(updated)
}

/// 获取用户故事的重试历史
pub fn get_user_story_retry_history(
    conn: &Connection,
    story_id: &str,
) -> Result<Vec<UserStoryRetryHistory>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM user_story_retry_history 
         WHERE user_story_id = ?1 
         ORDER BY triggered_at DESC"
    )?;
    
    let rows = stmt.query_map([story_id], |row| {
        UserStoryRetryHistory::from_row(row)
    })?;
    
    let mut histories = Vec::new();
    for row in rows {
        histories.push(row?);
    }
    
    println!("[DB::get_user_story_retry_history] Found {} retry records for story: {}", histories.len(), story_id);
    Ok(histories)
}

/// 获取项目的重试统计数据
pub fn get_project_retry_statistics(
    conn: &Connection,
    project_id: &str,
) -> Result<ProjectRetryStats> {
    // 总重试次数
    let total_retries: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_story_retry_history usrh
         INNER JOIN user_stories us ON usrh.user_story_id = us.id
         WHERE us.project_id = ?1",
        [project_id],
        |row| row.get(0),
    )?;
    
    // 成功重试次数
    let successful_retries: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_story_retry_history usrh
         INNER JOIN user_stories us ON usrh.user_story_id = us.id
         WHERE us.project_id = ?1 AND usrh.result = 'success'",
        [project_id],
        |row| row.get(0),
    )?;
    
    // 失败重试次数
    let failed_retries: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_story_retry_history usrh
         INNER JOIN user_stories us ON usrh.user_story_id = us.id
         WHERE us.project_id = ?1 AND usrh.result = 'failed'",
        [project_id],
        |row| row.get(0),
    )?;
    
    // 待处理重试次数
    let pending_retries: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_story_retry_history usrh
         INNER JOIN user_stories us ON usrh.user_story_id = us.id
         WHERE us.project_id = ?1 AND (usrh.result IS NULL OR usrh.result = 'pending')",
        [project_id],
        |row| row.get(0),
    )?;
    
    // 平均重试次数（每个 Story）
    let avg_retries: f64 = conn.query_row(
        "SELECT AVG(retry_count) FROM user_stories WHERE project_id = ?1",
        [project_id],
        |row| row.get(0),
    ).unwrap_or(0.0);
    
    let success_rate = if total_retries > 0 {
        (successful_retries as f64 / total_retries as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(ProjectRetryStats {
        total_retries: total_retries as i32,
        successful_retries: successful_retries as i32,
        failed_retries: failed_retries as i32,
        pending_retries: pending_retries as i32,
        success_rate,
        avg_retries,
    })
}

/// 项目重试统计数据
#[derive(Debug, Clone)]
pub struct ProjectRetryStats {
    pub total_retries: i32,
    pub successful_retries: i32,
    pub failed_retries: i32,
    pub pending_retries: i32,
    pub success_rate: f64,
    pub avg_retries: f64,
}

/// 更新用户故事的下次重试时间
pub fn update_user_story_next_retry_at(
    conn: &Connection,
    story_id: &str,
    next_retry_at: Option<&str>,
) -> Result<usize> {
    let updated = conn.execute(
        "UPDATE user_stories 
         SET next_retry_at = ?1,
             updated_at = CURRENT_TIMESTAMP
         WHERE id = ?2",
        rusqlite::params![next_retry_at, story_id],
    )?;
    
    println!("[DB::update_user_story_next_retry_at] Updated next_retry_at for story: {}", story_id);
    Ok(updated)
}

/// 获取待重试的用户故事列表
pub fn get_scheduled_retry_stories(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<UserStory>> {
    let now = Utc::now().to_rfc3339();
    
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories 
         WHERE status = 'scheduled_retry'
           AND next_retry_at IS NOT NULL
           AND next_retry_at <= ?1
         ORDER BY next_retry_at ASC
         LIMIT ?2"
    )?;
    
    let rows = stmt.query_map(rusqlite::params![now, limit], |row| {
        UserStory::from_row(row)
    })?;
    
    let mut stories = Vec::new();
    for row in rows {
        stories.push(row?);
    }
    
    println!("[DB::get_scheduled_retry_stories] Found {} stories ready for retry", stories.len());
    Ok(stories)
}

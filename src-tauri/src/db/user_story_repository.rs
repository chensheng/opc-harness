use crate::db::Entity;
use crate::models::UserStory;
use rusqlite::{Connection, Result};

/// 批量创建或更新用户故事（Upsert）
pub fn upsert_user_stories(conn: &Connection, project_id: &str, stories: &[UserStory]) -> Result<()> {
    println!("[DB::upsert_user_stories] Starting upsert for project_id: {}, count: {}", 
             project_id, stories.len());
    
    // 使用事务确保原子性
    let tx = conn.unchecked_transaction()?;
    
    // 先删除该项目的旧故事
    let deleted = tx.execute(
        "DELETE FROM user_stories WHERE project_id = ?1",
        [project_id],
    )?;
    println!("[DB::upsert_user_stories] Deleted {} old stories", deleted);

    // 批量插入新故事
    let mut inserted_count = 0;
    for story in stories {
        tx.execute(
            "INSERT INTO user_stories (
                id, project_id, story_number, title, role, feature, benefit, 
                description, acceptance_criteria, priority, story_points, status,
                epic, labels, dependencies, sprint_id, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            rusqlite::params![
                story.id,
                story.project_id,
                story.story_number,
                story.title,
                story.role,
                story.feature,
                story.benefit,
                story.description,
                story.acceptance_criteria,
                story.priority,
                story.story_points,
                story.status,
                story.epic,
                story.labels,
                story.dependencies,
                story.sprint_id,
                story.created_at,
                story.updated_at
            ],
        )?;
        inserted_count += 1;
    }
    println!("[DB::upsert_user_stories] Inserted {} new stories", inserted_count);

    tx.commit()?;
    println!("[DB::upsert_user_stories] Transaction committed successfully");
    Ok(())
}

/// 获取项目的所有用户故事
pub fn get_user_stories_by_project(conn: &Connection, project_id: &str) -> Result<Vec<UserStory>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories WHERE project_id = ?1 ORDER BY story_number ASC"
    )?;
    
    let stories = stmt.query_map([project_id], |row| {
        UserStory::from_row(row)
    })?;

    let mut result = Vec::new();
    for story_result in stories {
        result.push(story_result?);
    }
    
    Ok(result)
}

/// 获取指定 Sprint 下的所有用户故事
#[allow(dead_code)]
pub fn get_user_stories_by_sprint(conn: &Connection, sprint_id: &str) -> Result<Vec<UserStory>> {
    println!("[DB::get_user_stories_by_sprint] Querying for sprint_id: {}", sprint_id);
    
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories WHERE sprint_id = ?1 ORDER BY story_number ASC"
    )?;
    
    let stories = stmt.query_map([sprint_id], |row| {
        UserStory::from_row(row)
    })?;

    let mut result = Vec::new();
    for story_result in stories {
        result.push(story_result?);
    }
    
    println!("[DB::get_user_stories_by_sprint] Retrieved {} stories", result.len());

    Ok(result)
}

/// 更新用户故事的 Sprint 关联
#[allow(dead_code)]
pub fn update_story_sprint(conn: &Connection, story_id: &str, sprint_id: Option<&str>) -> Result<usize> {
    println!("[DB::update_story_sprint] Updating story_id: {} to sprint_id: {:?}", 
             story_id, sprint_id);
    
    let updated = match sprint_id {
        Some(sid) => conn.execute(
            "UPDATE user_stories SET sprint_id = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            rusqlite::params![sid, story_id],
        )?,
        None => conn.execute(
            "UPDATE user_stories SET sprint_id = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            [story_id],
        )?,
    };
    
    println!("[DB::update_story_sprint] Updated {} story(s)", updated);
    Ok(updated)
}

/// 获取待重试的用户故事队列
/// 查询状态为 'scheduled_retry' 且 next_retry_at <= now() 的故事
pub fn get_pending_retries(conn: &Connection, limit: usize) -> Result<Vec<UserStory>> {
    log::info!("[DB::get_pending_retries] Querying pending retries with limit: {}", limit);
    
    let mut stmt = conn.prepare(
        "SELECT * FROM user_stories 
         WHERE status = 'scheduled_retry' 
           AND next_retry_at <= datetime('now')
         ORDER BY next_retry_at ASC
         LIMIT ?1"
    )?;
    
    let stories = stmt.query_map([limit], |row| {
        UserStory::from_row(row)
    })?;

    let mut result = Vec::new();
    for story_result in stories {
        result.push(story_result?);
    }
    
    log::info!("[DB::get_pending_retries] Found {} pending retries", result.len());
    Ok(result)
}

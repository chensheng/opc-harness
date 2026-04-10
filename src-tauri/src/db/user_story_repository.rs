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
                epic, labels, dependencies, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
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
    println!("[DB::get_user_stories_by_project] Querying for project_id: {}", project_id);
    
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
    
    println!("[DB::get_user_stories_by_project] Retrieved {} stories", result.len());

    Ok(result)
}

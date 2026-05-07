use crate::commands::quality::types::{
    UserStory,
    SaveUserStoriesRequest,
    SaveUserStoriesResponse,
    GetUserStoriesRequest,
    GetUserStoriesResponse,
};
use crate::db;
use crate::models::UserStory as DbUserStory;
use chrono::Utc;

/// 保存用户故事到数据库
#[tauri::command]
pub async fn save_user_stories(
    _app_handle: tauri::AppHandle,
    request: SaveUserStoriesRequest,
) -> Result<SaveUserStoriesResponse, String> {
    println!("[save_user_stories] Received request for project_id: {}, stories count: {}", 
             request.project_id, request.user_stories.len());
    
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[save_user_stories] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    // 转换前端UserStory为数据库UserStory
    let db_stories: Vec<DbUserStory> = request.user_stories.iter().map(|story| {
        let now = Utc::now().to_rfc3339();
        DbUserStory {
            id: story.id.clone(),
            project_id: request.project_id.clone(),
            story_number: story.story_number.clone(),
            title: story.title.clone(),
            role: story.role.clone(),
            feature: story.feature.clone(),
            benefit: story.benefit.clone(),
            description: story.description.clone(),
            acceptance_criteria: serde_json::to_string(&story.acceptance_criteria).unwrap_or_else(|_| "[]".to_string()),
            priority: story.priority.clone(),
            story_points: story.story_points.unwrap_or(0) as i32,
            status: story.status.clone(),
            epic: story.feature_module.clone(),
            labels: Some(serde_json::to_string(&story.labels).unwrap_or_else(|_| "[]".to_string())),
            dependencies: story.dependencies.as_ref().map(|d| serde_json::to_string(d).unwrap_or_else(|_| "[]".to_string())),
            sprint_id: story.sprint_id.clone(),
            assigned_agent: None,
            locked_at: None,
            started_at: None,
            completed_at: None,
            failed_at: None,
            error_message: None,
            retry_count: 0,
            max_retries: 3, // 默认值
            next_retry_at: None,
            failure_reason: None,
            last_error_timestamp: None,
            created_at: story.created_at.clone(),
            updated_at: now,
        }
    }).collect();
    
    println!("[save_user_stories] Converted {} stories to DB format", db_stories.len());
    
    // 批量保存到数据库
    match db::upsert_user_stories(&conn, &request.project_id, &db_stories) {
        Ok(_) => {
            println!("[save_user_stories] Successfully saved {} stories to database for project {}", 
                     db_stories.len(), request.project_id);
        },
        Err(e) => {
            eprintln!("[save_user_stories] Failed to save user stories: {}", e);
            return Err(format!("Failed to save user stories: {}", e));
        }
    }
    
    Ok(SaveUserStoriesResponse {
        success: true,
        count: db_stories.len(),
        error: None,
    })
}

/// 从数据库获取项目的用户故事
#[tauri::command]
pub async fn get_user_stories(
    _app_handle: tauri::AppHandle,
    request: GetUserStoriesRequest,
) -> Result<GetUserStoriesResponse, String> {
    let conn = db::get_connection().map_err(|e| {
        eprintln!("[get_user_stories] Failed to get DB connection: {}", e);
        format!("Failed to get DB connection: {}", e)
    })?;
    
    // 从数据库查询
    let db_stories = match db::get_user_stories_by_project(&conn, &request.project_id) {
        Ok(stories) => {
            stories
        },
        Err(e) => {
            eprintln!("[get_user_stories] Failed to query user stories: {}", e);
            return Err(format!("Failed to get user stories: {}", e));
        }
    };
    
    // 转换数据库UserStory为前端UserStory
    let user_stories: Vec<UserStory> = db_stories.iter().map(|story| {
        UserStory {
            id: story.id.clone(),
            story_number: story.story_number.clone(),
            title: story.title.clone(),
            role: story.role.clone(),
            feature: story.feature.clone(),
            benefit: story.benefit.clone(),
            description: story.description.clone(),
            acceptance_criteria: serde_json::from_str(&story.acceptance_criteria).unwrap_or_default(),
            priority: story.priority.clone(),
            story_points: Some(story.story_points as u32),
            status: story.status.clone(),
            dependencies: story.dependencies.as_ref().and_then(|d| serde_json::from_str(d).ok()),
            feature_module: story.epic.clone(),
            sprint_id: story.sprint_id.clone(),
            labels: story.labels.as_ref().and_then(|l| serde_json::from_str(l).ok()).unwrap_or_default(),
            created_at: story.created_at.clone(),
            updated_at: story.updated_at.clone(),
        }
    }).collect();
    
    Ok(GetUserStoriesResponse {
        success: true,
        user_stories,
        error: None,
    })
}

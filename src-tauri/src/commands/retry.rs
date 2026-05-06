//! 重试引擎相关 Tauri Commands

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db;
use crate::models::UserStoryRetryHistory;
use crate::agent::retry_engine::BackoffConfig;

/// 获取用户故事的重试历史
#[tauri::command]
pub fn get_user_story_retry_history(
    story_id: String,
) -> Result<Vec<UserStoryRetryHistory>, String> {
    let conn = db::get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    db::get_user_story_retry_history(&conn, &story_id)
        .map_err(|e| format!("Failed to query retry history: {}", e))
}

/// 更新用户故事的重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRetryConfigRequest {
    pub project_id: String,
    pub max_retries: Option<u32>,
    pub base_delay_seconds: Option<u64>,
    pub max_delay_seconds: Option<u64>,
}

#[tauri::command]
pub fn update_user_story_retry_config(
    request: UpdateRetryConfigRequest,
) -> Result<(), String> {
    // TODO: 实现项目级别重试配置的持久化存储
    // 目前仅在内存中验证参数范围

    // 验证参数范围
    if let Some(max_retries) = request.max_retries {
        if !(1..=10).contains(&max_retries) {
            return Err("max_retries must be between 1 and 10".to_string());
        }
    }

    if let Some(base_delay) = request.base_delay_seconds {
        if !(30..=300).contains(&base_delay) {
            return Err("base_delay_seconds must be between 30 and 300".to_string());
        }
    }

    if let Some(max_delay) = request.max_delay_seconds {
        if !(300..=7200).contains(&max_delay) {
            return Err("max_delay_seconds must be between 300 and 7200".to_string());
        }
        
        // 确保 max_delay >= base_delay
        if let Some(base_delay) = request.base_delay_seconds {
            if max_delay < base_delay {
                return Err("max_delay_seconds must be >= base_delay_seconds".to_string());
            }
        }
    }

    // TODO: 将配置保存到项目级别配置表
    // 目前先返回成功，实际实现需要创建项目配置表
    log::info!(
        "[RetryConfig] Updated config for project {}: max_retries={:?}, base_delay={:?}, max_delay={:?}",
        request.project_id,
        request.max_retries,
        request.base_delay_seconds,
        request.max_delay_seconds
    );

    Ok(())
}

/// 项目重试统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRetryStats {
    pub total_retries: i32,
    pub successful_retries: i32,
    pub failed_retries: i32,
    pub pending_retries: i32,
    pub success_rate: f64,
    pub avg_retries: f64,
}

/// 获取项目的重试统计数据
#[tauri::command]
pub fn get_project_retry_statistics(
    project_id: String,
) -> Result<ProjectRetryStats, String> {
    let conn = db::get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let stats = db::get_project_retry_statistics(&conn, &project_id)
        .map_err(|e| format!("Failed to get retry statistics: {}", e))?;

    Ok(ProjectRetryStats {
        total_retries: stats.total_retries,
        successful_retries: stats.successful_retries,
        failed_retries: stats.failed_retries,
        pending_retries: stats.pending_retries,
        success_rate: stats.success_rate,
        avg_retries: stats.avg_retries,
    })
}

/// 手动触发用户故事重试
#[tauri::command]
pub fn trigger_manual_retry(
    story_id: String,
) -> Result<(), String> {
    let conn = db::get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    // 获取 Story 信息
    let story = db::get_user_story_by_id(&conn, &story_id)
        .map_err(|e| format!("Failed to query story: {}", e))?
        .ok_or_else(|| format!("Story {} not found", story_id))?;

    // 检查是否处于可重试状态
    if !["failed", "scheduled_retry"].contains(&story.status.as_str()) {
        return Err(format!(
            "Story {} is in '{}' status and cannot be retried",
            story_id, story.status
        ));
    }

    // 计算下次重试时间（立即重试）
    let backoff_config = BackoffConfig::default();
    let calculator = crate::agent::retry_engine::BackoffCalculator::new(backoff_config);
    let next_retry_at = calculator.calculate_next_retry_at(story.retry_count as u32);

    // 更新 Story 状态
    let now = chrono::Utc::now().to_rfc3339();
    let updated = conn.execute(
        "UPDATE user_stories 
         SET status = 'scheduled_retry',
             next_retry_at = ?1,
             updated_at = ?2
         WHERE id = ?3",
        rusqlite::params![next_retry_at, now, story_id],
    ).map_err(|e| format!("Failed to update story status: {}", e))?;

    if updated == 0 {
        return Err(format!("Story {} not found", story_id));
    }

    log::info!(
        "[ManualRetry] Triggered manual retry for story {}. Next retry at: {}",
        story_id,
        next_retry_at
    );

    Ok(())
}

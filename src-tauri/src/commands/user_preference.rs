/// 用户偏好相关 Tauri Commands

use crate::user_preference::manager::{UserPreferenceManager, Feedback, PreferenceModel};
use serde::{Deserialize, Serialize};

/// 获取用户偏好
#[tauri::command]
pub async fn get_user_preferences() -> Result<PreferenceModel, String> {
    let mut manager = UserPreferenceManager::new();
    manager.load_preferences()
}

/// 更新用户偏好
#[tauri::command]
pub async fn update_user_preferences(
    model: PreferenceModel,
) -> Result<(), String> {
    let mut manager = UserPreferenceManager::new();
    manager.save_preferences(&model)
}

/// 从反馈中分析偏好
#[tauri::command]
pub async fn analyze_preference_from_feedback(
    feedback_history: Vec<Feedback>,
) -> Result<PreferenceModel, String> {
    let mut manager = UserPreferenceManager::new();
    Ok(manager.analyze_from_feedback(&feedback_history))
}

/// 应用偏好到 PRD
#[tauri::command]
pub async fn apply_preference_to_prd(
    prd_json: String,
) -> Result<String, String> {
    let manager = UserPreferenceManager::new();
    manager.apply_preferences(&prd_json)
}

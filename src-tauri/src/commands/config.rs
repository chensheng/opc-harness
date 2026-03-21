//! Configuration commands

use crate::models::AppSettings;
use crate::services::config_service::{ConfigItem, ConfigUpdateRequest};
use crate::services::Services;
use tauri::State;

/// Get application settings
#[tauri::command]
pub fn get_settings(services: State<'_, Services>) -> Result<AppSettings, String> {
    services
        .config
        .get_settings()
        .map_err(|e| format!("Failed to get settings: {}", e))
}

/// Save application settings
#[tauri::command]
pub fn save_settings(services: State<'_, Services>, settings: AppSettings) -> Result<(), String> {
    services
        .config
        .save_settings(&settings)
        .map_err(|e| format!("Failed to save settings: {}", e))
}

/// Get a configuration value
#[tauri::command]
pub fn get_config(
    services: State<'_, Services>,
    key: String,
    default: Option<String>,
) -> Result<String, String> {
    let default = default.unwrap_or_default();
    services
        .config
        .get_string(&key, &default)
        .map_err(|e| format!("Failed to get config: {}", e))
}

/// Set a configuration value
#[tauri::command]
pub fn set_config(services: State<'_, Services>, key: String, value: String) -> Result<(), String> {
    services
        .config
        .set(&key, &value)
        .map_err(|e| format!("Failed to set config: {}", e))
}

/// Get boolean configuration
#[tauri::command]
pub fn get_config_bool(
    services: State<'_, Services>,
    key: String,
    default: bool,
) -> Result<bool, String> {
    services
        .config
        .get_bool(&key, default)
        .map_err(|e| format!("Failed to get config: {}", e))
}

/// Set boolean configuration
#[tauri::command]
pub fn set_config_bool(services: State<'_, Services>, key: String, value: bool) -> Result<(), String> {
    services
        .config
        .set_bool(&key, value)
        .map_err(|e| format!("Failed to set config: {}", e))
}

/// Get integer configuration
#[tauri::command]
pub fn get_config_i64(
    services: State<'_, Services>,
    key: String,
    default: i64,
) -> Result<i64, String> {
    services
        .config
        .get_i64(&key, default)
        .map_err(|e| format!("Failed to get config: {}", e))
}

/// Set integer configuration
#[tauri::command]
pub fn set_config_i64(services: State<'_, Services>, key: String, value: i64) -> Result<(), String> {
    services
        .config
        .set_i64(&key, value)
        .map_err(|e| format!("Failed to set config: {}", e))
}

/// Check if configuration exists
#[tauri::command]
pub fn config_exists(services: State<'_, Services>, key: String) -> Result<bool, String> {
    services
        .config
        .exists(&key)
        .map_err(|e| format!("Failed to check config: {}", e))
}

/// Remove a configuration
#[tauri::command]
pub fn remove_config(services: State<'_, Services>, key: String) -> Result<(), String> {
    services
        .config
        .remove(&key)
        .map_err(|e| format!("Failed to remove config: {}", e))
}

/// Get all configurations
#[tauri::command]
pub fn get_all_configs(services: State<'_, Services>) -> Result<Vec<ConfigItem>, String> {
    let configs = services
        .config
        .get_all()
        .map_err(|e| format!("Failed to get all configs: {}", e))?;
    
    let items: Vec<ConfigItem> = configs
        .into_iter()
        .map(|(key, value)| ConfigItem {
            key,
            value,
            default_value: None,
        })
        .collect();
    
    Ok(items)
}

/// Update multiple configurations
#[tauri::command]
pub fn update_configs(
    services: State<'_, Services>,
    updates: Vec<ConfigUpdateRequest>,
) -> Result<(), String> {
    for update in updates {
        services
            .config
            .set(&update.key, &update.value)
            .map_err(|e| format!("Failed to update config {}: {}", update.key, e))?;
    }
    Ok(())
}

/// Reset all configurations to defaults
#[tauri::command]
pub fn reset_configs_to_defaults(services: State<'_, Services>) -> Result<(), String> {
    services
        .config
        .reset_to_defaults()
        .map_err(|e| format!("Failed to reset configs: {}", e))
}

/// Get theme setting
#[tauri::command]
pub fn get_theme(services: State<'_, Services>) -> Result<String, String> {
    services
        .config
        .get_string("theme", "system")
        .map_err(|e| format!("Failed to get theme: {}", e))
}

/// Set theme
#[tauri::command]
pub fn set_theme(services: State<'_, Services>, theme: String) -> Result<(), String> {
    services
        .config
        .set("theme", &theme)
        .map_err(|e| format!("Failed to set theme: {}", e))
}

/// Get language setting
#[tauri::command]
pub fn get_language(services: State<'_, Services>) -> Result<String, String> {
    services
        .config
        .get_string("language", "zh-CN")
        .map_err(|e| format!("Failed to get language: {}", e))
}

/// Set language
#[tauri::command]
pub fn set_language(services: State<'_, Services>, language: String) -> Result<(), String> {
    services
        .config
        .set("language", &language)
        .map_err(|e| format!("Failed to set language: {}", e))
}

/// Get auto save setting
#[tauri::command]
pub fn get_auto_save(services: State<'_, Services>) -> Result<bool, String> {
    services
        .config
        .get_bool("auto_save", true)
        .map_err(|e| format!("Failed to get auto_save: {}", e))
}

/// Set auto save
#[tauri::command]
pub fn set_auto_save(services: State<'_, Services>, auto_save: bool) -> Result<(), String> {
    services
        .config
        .set_bool("auto_save", auto_save)
        .map_err(|e| format!("Failed to set auto_save: {}", e))
}

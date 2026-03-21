//! 配置文件管理服务
//!
//! 管理应用配置，支持默认值、持久化到数据库

use crate::models::AppSettings;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// 配置服务错误
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Lock error: {0}")]
    Lock(String),
}

/// 配置服务
pub struct ConfigService {
    db: Arc<Mutex<rusqlite::Connection>>,
}

impl ConfigService {
    /// 创建新的配置服务
    pub fn new(db: Arc<Mutex<rusqlite::Connection>>) -> Self {
        Self { db }
    }

    /// 获取字符串配置
    pub fn get_string(&self, key: &str, default: &str) -> Result<String, ConfigError> {
        let db = self.db.lock().map_err(|e| ConfigError::Lock(e.to_string()))?;
        
        let result: Option<String> = db
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| row.get(0),
            )
            .optional()?;
        
        Ok(result.unwrap_or_else(|| default.to_string()))
    }

    /// 获取布尔配置
    pub fn get_bool(&self, key: &str, default: bool) -> Result<bool, ConfigError> {
        let value = self.get_string(key, "")?;
        if value.is_empty() {
            return Ok(default);
        }
        Ok(value.parse().unwrap_or(default))
    }

    /// 获取整数配置
    pub fn get_i64(&self, key: &str, default: i64) -> Result<i64, ConfigError> {
        let value = self.get_string(key, "")?;
        if value.is_empty() {
            return Ok(default);
        }
        Ok(value.parse().unwrap_or(default))
    }

    /// 设置配置值
    pub fn set(&self, key: &str, value: &str) -> Result<(), ConfigError> {
        let db = self.db.lock().map_err(|e| ConfigError::Lock(e.to_string()))?;
        
        let now = chrono::Utc::now().timestamp();
        db.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![key, value, now],
        )?;
        
        log::info!("Config set: {} = {}", key, value);
        Ok(())
    }

    /// 设置布尔值
    pub fn set_bool(&self, key: &str, value: bool) -> Result<(), ConfigError> {
        self.set(key, &value.to_string())
    }

    /// 设置整数值
    pub fn set_i64(&self, key: &str, value: i64) -> Result<(), ConfigError> {
        self.set(key, &value.to_string())
    }

    /// 删除配置
    pub fn remove(&self, key: &str) -> Result<(), ConfigError> {
        let db = self.db.lock().map_err(|e| ConfigError::Lock(e.to_string()))?;
        
        db.execute("DELETE FROM settings WHERE key = ?1", [key])?;
        log::info!("Config removed: {}", key);
        Ok(())
    }

    /// 检查配置是否存在
    pub fn exists(&self, key: &str) -> Result<bool, ConfigError> {
        let db = self.db.lock().map_err(|e| ConfigError::Lock(e.to_string()))?;
        
        let count: i64 = db.query_row(
            "SELECT COUNT(*) FROM settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        )?;
        
        Ok(count > 0)
    }

    /// 获取所有配置
    pub fn get_all(&self) -> Result<Vec<(String, String)>, ConfigError> {
        let db = self.db.lock().map_err(|e| ConfigError::Lock(e.to_string()))?;
        
        let mut stmt = db.prepare("SELECT key, value FROM settings ORDER BY key")?;
        
        let configs = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(configs)
    }

    /// 重置为默认值
    pub fn reset_to_defaults(&self) -> Result<(), ConfigError> {
        let defaults = Self::default_configs();
        
        for (key, value) in defaults {
            self.set(&key, &value)?;
        }
        
        log::info!("Config reset to defaults");
        Ok(())
    }

    /// 获取默认配置
    fn default_configs() -> Vec<(String, String)> {
        vec![
            ("theme".to_string(), "system".to_string()),
            ("language".to_string(), "zh-CN".to_string()),
            ("auto_save".to_string(), "true".to_string()),
            ("default_ai_provider".to_string(), "openai".to_string()),
            ("window_width".to_string(), "1200".to_string()),
            ("window_height".to_string(), "800".to_string()),
            ("sidebar_collapsed".to_string(), "false".to_string()),
            ("recent_projects_limit".to_string(), "10".to_string()),
        ]
    }
}

/// AppSettings 专用接口
impl ConfigService {
    /// 获取应用设置
    pub fn get_settings(&self) -> Result<AppSettings, ConfigError> {
        Ok(AppSettings {
            theme: self.get_string("theme", "system")?,
            language: self.get_string("language", "zh-CN")?,
            auto_save: self.get_bool("auto_save", true)?,
        })
    }

    /// 保存应用设置
    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), ConfigError> {
        self.set("theme", &settings.theme)?;
        self.set("language", &settings.language)?;
        self.set_bool("auto_save", settings.auto_save)?;
        Ok(())
    }

    /// 更新单个设置项
    pub fn update_setting(&self, key: &str, value: &str) -> Result<(), ConfigError> {
        match key {
            "theme" | "language" | "auto_save" => {
                self.set(key, value)?;
                Ok(())
            }
            _ => Err(ConfigError::Lock(format!("Unknown setting key: {}", key))),
        }
    }
}

/// 前端交互结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    pub key: String,
    pub value: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdateRequest {
    pub key: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::initialize_database;

    fn setup_test_db() -> (ConfigService, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        initialize_database(&conn).unwrap();
        
        let db = Arc::new(Mutex::new(conn));
        let service = ConfigService::new(db);
        (service, temp_dir)
    }

    #[test]
    fn test_string_config() {
        let (service, _temp) = setup_test_db();
        
        // 设置配置
        service.set("test_key", "test_value").unwrap();
        
        // 读取配置
        let value = service.get_string("test_key", "default").unwrap();
        assert_eq!(value, "test_value");
        
        // 读取不存在的配置（使用默认值）
        let value = service.get_string("non_existent", "default").unwrap();
        assert_eq!(value, "default");
    }

    #[test]
    fn test_bool_config() {
        let (service, _temp) = setup_test_db();
        
        service.set_bool("bool_key", true).unwrap();
        
        let value = service.get_bool("bool_key", false).unwrap();
        assert!(value);
        
        // 默认值
        let value = service.get_bool("non_existent", true).unwrap();
        assert!(value);
    }

    #[test]
    fn test_i64_config() {
        let (service, _temp) = setup_test_db();
        
        service.set_i64("int_key", 42).unwrap();
        
        let value = service.get_i64("int_key", 0).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_exists_and_remove() {
        let (service, _temp) = setup_test_db();
        
        assert!(!service.exists("test_key").unwrap());
        
        service.set("test_key", "value").unwrap();
        assert!(service.exists("test_key").unwrap());
        
        service.remove("test_key").unwrap();
        assert!(!service.exists("test_key").unwrap());
    }

    #[test]
    fn test_app_settings() {
        let (service, _temp) = setup_test_db();
        
        let settings = AppSettings {
            theme: "dark".to_string(),
            language: "en".to_string(),
            auto_save: false,
        };
        
        service.save_settings(&settings).unwrap();
        
        let retrieved = service.get_settings().unwrap();
        assert_eq!(retrieved.theme, "dark");
        assert_eq!(retrieved.language, "en");
        assert!(!retrieved.auto_save);
    }

    #[test]
    fn test_get_all() {
        let (service, _temp) = setup_test_db();
        
        service.set("key1", "value1").unwrap();
        service.set("key2", "value2").unwrap();
        
        let all = service.get_all().unwrap();
        assert!(all.iter().any(|(k, v)| k == "key1" && v == "value1"));
        assert!(all.iter().any(|(k, v)| k == "key2" && v == "value2"));
    }

    #[test]
    fn test_reset_to_defaults() {
        let (service, _temp) = setup_test_db();
        
        // 修改一个默认值
        service.set("theme", "dark").unwrap();
        assert_eq!(service.get_string("theme", "").unwrap(), "dark");
        
        // 重置为默认值
        service.reset_to_defaults().unwrap();
        
        // 验证已重置
        let theme = service.get_string("theme", "").unwrap();
        assert_eq!(theme, "system");
    }
}

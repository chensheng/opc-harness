use crate::models::AIConfig;
use rusqlite::{Connection, Result};

/// 保存 AI 配置 (仅存储 provider 和 model，不存储 API key)
pub fn save_ai_config(conn: &Connection, config: &AIConfig) -> Result<()> {
    // Only store provider and model in database
    // API key is stored securely in OS keychain
    conn.execute(
        "INSERT OR REPLACE INTO ai_configs (provider, model)
         VALUES (?1, ?2)",
        [&config.provider, &config.model],
    )?;
    Ok(())
}

/// 获取所有 AI 配置 (不包含 API key)
pub fn get_all_ai_configs(conn: &Connection) -> Result<Vec<AIConfig>> {
    let mut stmt = conn.prepare("SELECT provider, model FROM ai_configs")?;
    let configs = stmt.query_map([], |row| Ok(AIConfig::new(row.get(0)?, row.get(1)?)))?;

    let mut result = Vec::new();
    for config in configs {
        result.push(config?);
    }
    Ok(result)
}

/// 获取单个 AI 配置 (不包含 API key)
pub fn get_ai_config(conn: &Connection, provider: &str) -> Result<Option<AIConfig>> {
    let mut stmt = conn.prepare("SELECT provider, model FROM ai_configs WHERE provider = ?1")?;
    let mut rows = stmt.query_map([provider], |row| {
        Ok(AIConfig::new(row.get(0)?, row.get(1)?))
    })?;

    if let Some(row) = rows.next() {
        return Ok(Some(row?));
    }
    Ok(None)
}

/// 删除 AI 配置
pub fn delete_ai_config(conn: &Connection, provider: &str) -> Result<()> {
    conn.execute("DELETE FROM ai_configs WHERE provider = ?1", [provider])?;
    Ok(())
}

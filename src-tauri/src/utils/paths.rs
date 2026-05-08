/// 应用路径工具模块
///
/// 参考 OpenClaw、Claude Code 等业界标准做法，在用户 home 目录下创建 .opc-harness 目录来存储所有应用数据
use std::path::PathBuf;
use tauri::Manager;

/// 获取 OPC-HARNESS 应用的根目录
///
/// 返回: ~/.opc-harness (跨平台)
/// - Windows: C:\Users\<username>\.opc-harness
/// - macOS/Linux: /home/<username>/.opc-harness 或 ~/.opc-harness
pub fn get_app_root() -> PathBuf {
    // 优先使用环境变量 OPC_HARNESS_HOME（便于自定义）
    if let Ok(custom_path) = std::env::var("OPC_HARNESS_HOME") {
        return PathBuf::from(custom_path);
    }

    // 默认使用用户 home 目录下的 .opc-harness
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".opc-harness")
}

/// 获取数据库文件路径
///
/// 返回: ~/.opc-harness/opc-harness.db
pub fn get_database_path() -> PathBuf {
    get_app_root().join("opc-harness.db")
}

/// 获取配置目录路径
///
/// 返回: ~/.opc-harness/config/
pub fn get_config_dir() -> PathBuf {
    get_app_root().join("config")
}

/// 获取日志目录路径
///
/// 返回: ~/.opc-harness/logs/
pub fn get_log_dir() -> PathBuf {
    get_app_root().join("logs")
}

/// 获取缓存目录路径
///
/// 返回: ~/.opc-harness/cache/
pub fn get_cache_dir() -> PathBuf {
    get_app_root().join("cache")
}

/// 获取会话数据存储目录
///
/// 返回: ~/.opc-harness/sessions/
pub fn get_sessions_dir() -> PathBuf {
    get_app_root().join("sessions")
}

/// 获取工作区代码存储目录
///
/// 返回: ~/.opc-harness/workspaces/
pub fn get_workspaces_dir() -> PathBuf {
    get_app_root().join("workspaces")
}

/// 确保所有必要的目录存在
///
/// 创建以下目录结构：
/// - ~/.opc-harness/
/// - ~/.opc-harness/config/
/// - ~/.opc-harness/logs/
/// - ~/.opc-harness/cache/
/// - ~/.opc-harness/sessions/
/// - ~/.opc-harness/workspaces/
pub fn ensure_app_directories() -> Result<(), String> {
    let dirs = vec![
        get_app_root(),
        get_config_dir(),
        get_log_dir(),
        get_cache_dir(),
        get_sessions_dir(),
        get_workspaces_dir(),
    ];

    for dir in dirs {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create directory {:?}: {}", dir, e))?;
    }

    Ok(())
}

/// 获取旧的 Tauri AppData 目录路径（用于迁移）
///
/// 返回旧版本的数据库存储位置
pub fn get_legacy_app_data_dir(app_handle: &tauri::AppHandle) -> Option<PathBuf> {
    app_handle.path().app_data_dir().ok()
}

/// 检查是否存在旧版本的数据需要迁移
pub fn has_legacy_data(app_handle: &tauri::AppHandle) -> bool {
    if let Some(legacy_dir) = get_legacy_app_data_dir(app_handle) {
        let legacy_db = legacy_dir.join("opc-harness.db");
        legacy_db.exists()
    } else {
        false
    }
}

/// 从旧位置迁移数据到新位置
///
/// # 参数
/// * `app_handle` - Tauri 应用句柄
///
/// # 返回
/// * `Ok(true)` - 迁移成功
/// * `Ok(false)` - 无需迁移（旧数据不存在）
/// * `Err(String)` - 迁移失败
pub fn migrate_legacy_data(app_handle: &tauri::AppHandle) -> Result<bool, String> {
    // 检查是否有旧数据
    if !has_legacy_data(app_handle) {
        log::info!("No legacy data found, skipping migration");
        return Ok(false);
    }

    // 获取新旧路径
    let legacy_dir =
        get_legacy_app_data_dir(app_handle).ok_or("Cannot determine legacy data directory")?;
    let legacy_db = legacy_dir.join("opc-harness.db");

    let new_db = get_database_path();

    // 如果新位置已有数据，跳过迁移
    if new_db.exists() {
        log::info!("New database already exists, skipping migration");
        return Ok(false);
    }

    log::info!("Migrating database from {:?} to {:?}", legacy_db, new_db);

    // 确保新目录存在
    ensure_app_directories()?;

    // 复制数据库文件
    std::fs::copy(&legacy_db, &new_db)
        .map_err(|e| format!("Failed to copy database file: {}", e))?;

    // 同时复制 WAL 和 SHM 文件（如果存在）
    let wal_file = legacy_db.with_extension("db-wal");
    let shm_file = legacy_db.with_extension("db-shm");

    if wal_file.exists() {
        let new_wal = new_db.with_extension("db-wal");
        std::fs::copy(&wal_file, &new_wal)
            .map_err(|e| format!("Failed to copy WAL file: {}", e))?;
        log::info!("WAL file migrated");
    }

    if shm_file.exists() {
        let new_shm = new_db.with_extension("db-shm");
        std::fs::copy(&shm_file, &new_shm)
            .map_err(|e| format!("Failed to copy SHM file: {}", e))?;
        log::info!("SHM file migrated");
    }

    log::info!("✅ Database migration completed successfully");
    log::info!("   Old location (kept as backup): {:?}", legacy_db);
    log::info!("   New location: {:?}", new_db);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TestCleanup, TEST_MUTEX};

    #[test]
    fn test_get_app_root_default() {
        let _lock = TEST_MUTEX.lock().unwrap();
        std::env::remove_var("OPC_HARNESS_HOME");

        let app_root = get_app_root();

        // 验证路径包含 .opc-harness
        assert!(app_root.to_string_lossy().contains(".opc-harness"));

        // 验证是绝对路径
        assert!(app_root.is_absolute());
    }

    #[test]
    fn test_get_database_path() {
        let _lock = TEST_MUTEX.lock().unwrap();
        std::env::remove_var("OPC_HARNESS_HOME");

        let db_path = get_database_path();

        // 验证文件名正确
        assert_eq!(db_path.file_name().unwrap(), "opc-harness.db");

        // 验证父目录是 app root
        assert!(db_path
            .parent()
            .unwrap()
            .to_string_lossy()
            .contains(".opc-harness"));
    }

    #[test]
    fn test_custom_home_via_env() {
        let _lock = TEST_MUTEX.lock().unwrap();

        // 清除可能存在的环境变量
        std::env::remove_var("OPC_HARNESS_HOME");

        // 设置自定义路径
        std::env::set_var("OPC_HARNESS_HOME", "/tmp/test-opc-harness");

        let app_root = get_app_root();
        assert_eq!(app_root, PathBuf::from("/tmp/test-opc-harness"));

        // 清理环境变量
        std::env::remove_var("OPC_HARNESS_HOME");
    }

    #[test]
    fn test_get_workspaces_dir() {
        let _lock = TEST_MUTEX.lock().unwrap();

        // 确保没有设置自定义环境变量
        std::env::remove_var("OPC_HARNESS_HOME");

        let workspaces_dir = get_workspaces_dir();

        // 验证目录名称正确
        assert_eq!(workspaces_dir.file_name().unwrap(), "workspaces");

        // 验证父目录是 app root
        assert!(workspaces_dir
            .parent()
            .unwrap()
            .to_string_lossy()
            .contains(".opc-harness"));
    }

    #[test]
    fn test_ensure_app_directories_creates_workspaces() {
        let _lock = TEST_MUTEX.lock().unwrap();

        // 清除可能存在的环境变量
        std::env::remove_var("OPC_HARNESS_HOME");

        // 使用临时目录进行测试
        let temp_dir = std::env::temp_dir().join(format!(
            "test-opc-harness-ensure-dirs-{}",
            uuid::Uuid::new_v4()
        ));

        // 清理可能存在的旧测试目录
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }

        // 创建 RAII 守卫，确保无论如何都会清理
        let _cleanup = TestCleanup::new(temp_dir.clone());

        std::env::set_var("OPC_HARNESS_HOME", temp_dir.to_str().unwrap());

        // 确保目录被创建
        let result = ensure_app_directories();
        assert!(
            result.is_ok(),
            "Failed to ensure app directories: {:?}",
            result.err()
        );

        // 验证 workspaces 目录已创建
        let workspaces_dir = get_workspaces_dir();
        assert!(workspaces_dir.exists(), "Workspaces directory should exist");
        assert!(
            workspaces_dir.is_dir(),
            "Workspaces path should be a directory"
        );

        // 验证其他目录也被创建
        assert!(get_config_dir().exists());
        assert!(get_log_dir().exists());
        assert!(get_cache_dir().exists());
        assert!(get_sessions_dir().exists());

        // 不需要手动清理，_cleanup 会在函数退出时自动调用 Drop
    }
}

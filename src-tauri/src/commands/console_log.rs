use tauri::command;

#[command]
pub fn console_log(level: String, message: String) -> Result<(), String> {
    let prefix = "[Frontend]";
    
    // 调试日志：确认 command 被调用
    eprintln!("[ConsoleBridge Debug] Received log - level: {}, message: {}", level, message);
    
    match level.as_str() {
        "debug" => {
            log::debug!("{} {}", prefix, message);
        }
        "log" | "info" => {
            log::info!("{} {}", prefix, message);
        }
        "warn" => {
            log::warn!("{} {}", prefix, message);
        }
        "error" => {
            log::error!("{} {}", prefix, message);
        }
        _ => {
            log::info!("{} [{}] {}", prefix, level, message);
        }
    }
    
    Ok(())
}

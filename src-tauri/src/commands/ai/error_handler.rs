use crate::ai::StreamError;
use tauri::Emitter;

/// 通用的流式错误发射函数
/// 
/// 统一的错误处理逻辑，包含详细的日志输出
/// 
/// # 参数
/// * `app` - Tauri 应用句柄
/// * `event_name` - 事件名称（如 "ai-stream-error" 或 "prd-stream-error"）
/// * `session_id` - 会话 ID
/// * `error` - AI 错误对象
/// 
/// # 示例
/// ```rust
/// emit_stream_error_detailed(&app, "ai-stream-error", &session_id, &error);
/// ```
pub fn emit_stream_error_detailed(
    app: &tauri::AppHandle,
    event_name: &str,
    session_id: &str,
    error: &crate::ai::AIError,
) {
    let error_message = error.to_string();
    
    // 详细的日志输出（与前端 usePRDAIChat 保持一致）
    log::error!("========================================");
    log::error!("[{}] Stream error event:", event_name);
    log::error!("[{}] Session ID: {}", event_name, session_id);
    log::error!("[{}] Error type: string", event_name);
    log::error!("[{}] Error length: {}", event_name, error_message.len());
    log::error!("[{}] Full error content:", event_name);
    log::error!("{}", error_message);
    log::error!("========================================");
    
    // 额外诊断信息：检查错误内容是否包含关键信息
    if error_message.contains("退出码") && !error_message.contains("CodeFree CLI 错误详情") {
        log::warn!("⚠️ [{}] WARNING: Error message only contains exit code without detailed diagnostics!", event_name);
        log::warn!("   [{}] This may indicate that CodeFree CLI failed before producing any output.", event_name);
        log::warn!("   [{}] Possible causes:", event_name);
        log::warn!("     1. CodeFree CLI executable not found or not accessible");
        log::warn!("     2. Configuration file (.codefree-cli/settings.json) missing or invalid");
        log::warn!("     3. Working directory not set correctly (project_id issue)");
        log::warn!("     4. Environment variables not set (CodeFree-oauth)");
    }
    
    // 发送错误事件到前端
    let error_data = StreamError {
        session_id: session_id.to_string(),
        error: error_message,
    };
    
    if let Err(e) = app.emit(event_name, error_data) {
        log::error!("[{}] Failed to emit error event: {}", event_name, e);
    }
}

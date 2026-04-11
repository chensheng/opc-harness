// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod ai;
mod cli;
mod commands;
mod db;
mod error;
mod file;
mod models;
mod prompts;
mod quality;
mod services;
mod test_utils;  // 测试工具模块
mod user_preference;
mod utils;
pub mod agent;
pub mod agent_protocol;
pub mod websocket;

// 导出统一错误类型
pub use error::{AppError, AppResult, ErrorCode};

/// 展开环境变量字符串（Windows）
/// 将 %VAR% 形式的引用替换为实际值
#[cfg(windows)]
fn expand_environment_strings(input: &str) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::System::Environment::ExpandEnvironmentStringsW;
    
    // 首先获取需要的缓冲区大小
    let input_wide: Vec<u16> = input.encode_utf16().chain(std::iter::once(0)).collect();
    let size = unsafe {
        ExpandEnvironmentStringsW(
            input_wide.as_ptr(),
            std::ptr::null_mut(),
            0,
        )
    };
    
    if size == 0 {
        // 如果失败，返回原始字符串
        return input.to_string();
    }
    
    // 分配缓冲区并再次调用
    let mut buffer = vec![0u16; size as usize];
    let result = unsafe {
        ExpandEnvironmentStringsW(
            input_wide.as_ptr(),
            buffer.as_mut_ptr(),
            size,
        )
    };
    
    if result == 0 {
        return input.to_string();
    }
    
    // 转换为 Rust 字符串
    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[..len])
        .to_string_lossy()
        .into_owned()
}

fn main() {
    // 初始化日志记录
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 确保继承系统的环境变量（特别是 PATH）
            #[cfg(windows)]
            {
                use std::env;
                use winreg::RegKey;
                use winreg::enums::*;
                
                log::info!("Ensuring system PATH is inherited on Windows...");
                
                // 获取系统 PATH（机器级别）
                let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                if let Ok(env_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment") {
                    if let Ok(system_path) = env_key.get_value::<String, _>("Path") {
                        log::info!("System PATH found, length: {}", system_path.len());
                        
                        // 展开环境变量引用（如 %SystemRoot% 等）
                        let expanded_system_path = expand_environment_strings(&system_path);
                        log::debug!("Expanded system PATH: {}", expanded_system_path);
                        
                        // 获取当前进程的 PATH
                        let current_path = env::var("PATH").unwrap_or_default();
                        
                        // 如果当前 PATH 不包含系统 PATH 的内容，则合并
                        if !current_path.contains(&expanded_system_path) {
                            let merged_path = if current_path.is_empty() {
                                expanded_system_path.clone()
                            } else {
                                format!("{};{}", current_path, expanded_system_path)
                            };
                            
                            env::set_var("PATH", &merged_path);
                            log::info!("Merged system PATH into process environment");
                        } else {
                            log::info!("System PATH already included in process environment");
                        }
                    }
                }
                
                // 也检查用户级别的 PATH
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok(env_key) = hkcu.open_subkey("Environment") {
                    if let Ok(user_path) = env_key.get_value::<String, _>("Path") {
                        // 展开用户 PATH 中的环境变量
                        let expanded_user_path = expand_environment_strings(&user_path);
                        log::debug!("Expanded user PATH: {}", expanded_user_path);
                        
                        let current_path = env::var("PATH").unwrap_or_default();
                        
                        if !current_path.contains(&expanded_user_path) {
                            let merged_path = format!("{};{}", current_path, expanded_user_path);
                            env::set_var("PATH", &merged_path);
                            log::info!("Merged user PATH into process environment");
                        }
                    }
                }
            }
            
            // Initialize database
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async move {
                if let Err(e) = db::init_database(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });

            // Check and create workspace directories for all projects
            if let Err(e) = db::ensure_all_project_workspaces() {
                log::warn!("Failed to ensure project workspaces: {}", e);
            } else {
                log::info!("All project workspace directories verified");
            }

            // Enable devtools in debug mode
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // AI commands
            commands::ai::validate_ai_key,
            commands::ai::chat,
            commands::ai::stream_chat,
            commands::ai::generate_prd,
            commands::ai::start_prd_stream,
            commands::ai::generate_user_personas,
            commands::ai::generate_competitor_analysis,
            commands::ai::generate_marketing_strategy,
            commands::ai::generate_marketing_copy,
            commands::ai::get_available_providers,
            // Claude API commands (AI-002)
            commands::ai::chat_claude,
            commands::ai::stream_chat_claude,
            commands::ai::generate_personas_claude,
            commands::ai::generate_competitor_analysis_claude,
            // Kimi API commands (AI-003)
            commands::ai::chat_kimi,
            commands::ai::stream_chat_kimi,
            commands::ai::generate_personas_kimi,
            commands::ai::generate_competitor_analysis_kimi,
            // GLM API commands (AI-004)
            commands::ai::chat_glm,
            commands::ai::stream_chat_glm,
            // CLI commands
            commands::cli::detect_tools,
            // commands::cli::create_cli_session,
            // commands::cli::send_cli_prompt,
            // commands::cli::read_cli_output,
            // commands::cli::stop_cli_session,
            // commands::cli::check_environment,
            // Database commands
            commands::database::create_project,
            commands::database::get_all_projects,
            commands::database::get_project_by_id,
            commands::database::update_project,
            commands::database::delete_project,
            commands::database::save_ai_config,
            commands::database::get_all_ai_configs,
            commands::database::get_ai_config,
            commands::database::delete_ai_config,
            commands::database::create_cli_session_db,
            commands::database::get_all_cli_sessions,
            commands::database::get_cli_session_by_id,
            commands::database::delete_cli_session,
            // Milestone commands (DB-002)
            commands::database::create_milestone,
            commands::database::get_milestones_by_project,
            commands::database::get_milestone_by_id,
            commands::database::update_milestone,
            commands::database::delete_milestone,
            // Issue commands (DB-003)
            commands::database::create_issue,
            commands::database::get_issues_by_project,
            commands::database::get_issues_by_milestone,
            commands::database::get_issue_by_id,
            commands::database::update_issue,
            commands::database::delete_issue,
            // Agent Session commands (DB-004)
            commands::database::create_agent_session,
            commands::database::get_sessions_by_project,
            commands::database::get_agent_session_by_id,
            commands::database::get_agent_session_by_session_id,
            commands::database::update_agent_session_status,
            commands::database::update_agent_session,
            commands::database::delete_agent_session,
            // Agent Manager commands (VC-010, VC-015, VC-016, VC-022, VC-026, VC-027)
            agent::agent_manager::run_initializer_agent,
            agent::agent_manager::create_feature_branch,
            agent::agent_manager::checkout_branch,
            agent::agent_manager::delete_branch,
            agent::agent_manager::list_branches,
            agent::agent_manager::get_current_branch,
            agent::agent_manager::create_merge_request,
            agent::agent_manager::run_debug_agent,
            agent::agent_manager::generate_commit_message,
            agent::agent_manager::run_code_review,
            agent::agent_manager::start_realtime_review,
            agent::agent_manager::stop_realtime_review,
            agent::agent_manager::run_tests,
            agent::agent_manager::run_benchmark,
            agent::agent_manager::start_monitoring,
            agent::agent_manager::stop_monitoring,
            agent::agent_manager::get_current_stats,
            agent::agent_manager::generate_code,
            agent::agent_manager::complete_code,
            agent::agent_manager::generate_function,
            agent::agent_manager::start_suggestions,
            agent::agent_manager::stop_suggestions,
            agent::agent_manager::get_suggestions,
            // VC-034: Code Change Tracker
            agent::agent_manager::get_workspace_changes,
            agent::agent_manager::get_file_diff,
            agent::agent_manager::get_change_statistics,
            // MR Description Generator commands
            agent::agent_manager::generate_mr_description,
            agent::agent_manager::get_mr_template,
            // Code Diff Visualizer commands
            agent::agent_manager::get_file_diff_visual,
            agent::agent_manager::get_diff_summary,

            // PRD Quality Check commands (US-050)
            commands::quality::check_prd_quality,
            // PRD Consistency Check commands (US-051)
            commands::quality::check_prd_consistency,
            // PRD Feasibility Assessment commands (US-052)
            commands::quality::assess_prd_feasibility,
            // PRD Iteration Optimization commands (US-053)
            commands::quality::create_initial_version,
            commands::quality::iterate_prd,
            // commands::quality::get_iteration_history,  // 暂时注释（尚未实现）
            commands::quality::rollback_to_version,
            // User Story Decomposition commands (US-XXX)
            commands::quality::decompose_user_stories,
            commands::quality::decompose_user_stories_streaming,
            // User Story Persistence commands
            commands::quality::save_user_stories,
            commands::quality::get_user_stories,
            // Streaming Output commands (US-048)
            commands::ai::start_prd_stream,
            // User Preference commands (US-055)
            commands::user_preference::get_user_preferences,
            commands::user_preference::update_user_preferences,
            commands::user_preference::analyze_preference_from_feedback,
            commands::user_preference::apply_preference_to_prd,

            // System commands
            commands::system::get_app_version,
            commands::system::open_external_link,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

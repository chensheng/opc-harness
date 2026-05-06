// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::RwLock;
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
#[cfg(test)]
mod test_utils;  // 娴嬭瘯宸ュ叿妯″潡锛堜粎鍦ㄦ祴璇曟椂缂栬瘧锛?
mod user_preference;
mod utils;
pub mod agent;
pub mod agent_protocol;
pub mod websocket;

// 瀵煎嚭缁熶竴閿欒绫诲瀷
pub use error::{AppError, AppResult, ErrorCode};

/// 灞曞紑鐜鍙橀噺瀛楃涓诧紙Windows锛?
/// 灏?%VAR% 褰㈠紡鐨勫紩鐢ㄦ浛鎹负瀹為檯鍊?
#[cfg(windows)]
fn expand_environment_strings(input: &str) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::System::Environment::ExpandEnvironmentStringsW;
    
    // 棣栧厛鑾峰彇闇€瑕佺殑缂撳啿鍖哄ぇ灏?
    let input_wide: Vec<u16> = input.encode_utf16().chain(std::iter::once(0)).collect();
    let size = unsafe {
        ExpandEnvironmentStringsW(
            input_wide.as_ptr(),
            std::ptr::null_mut(),
            0,
        )
    };
    
    if size == 0 {
        // 濡傛灉澶辫触锛岃繑鍥炲師濮嬪瓧绗︿覆
        return input.to_string();
    }
    
    // 鍒嗛厤缂撳啿鍖哄苟鍐嶆璋冪敤
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
    
    // 杞崲涓?Rust 瀧楃涓?
    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[..len])
        .to_string_lossy()
        .into_owned()
}

fn main() {
    // 鍒濆鍖栨棩蹇楄褰?
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 纭繚缁ф壙绯荤粺鐨勭幆澧冨彉閲忥紙鐗瑰埆鏄?PATH锛?
            #[cfg(windows)]
            {
                use std::env;
                use winreg::RegKey;
                use winreg::enums::*;
                
                log::info!("Ensuring system PATH is inherited on Windows...");
                
                // 鑾峰彇绯荤粺 PATH锛堟満鍣ㄧ骇鍒級
                let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
                if let Ok(env_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment") {
                    if let Ok(system_path) = env_key.get_value::<String, _>("Path") {
                        log::info!("System PATH found, length: {}", system_path.len());
                        
                        // 灞曞紑鐜鍙橀噺寮曠敤锛堝 %SystemRoot% 绛夛級
                        let expanded_system_path = expand_environment_strings(&system_path);
                        log::debug!("Expanded system PATH: {}", expanded_system_path);
                        
                        // 鑾峰彇褰撳墠杩涚▼鐨?PATH
                        let current_path = env::var("PATH").unwrap_or_default();
                        
                        // 濡傛灉褰撳墠 PATH 涓嶅寘鍚郴缁?PATH 鐨勫唴瀹癸紝鍒欏悎骞?
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
                
                // 涔熸鏌ョ敤鎴风骇鍒殑 PATH
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok(env_key) = hkcu.open_subkey("Environment") {
                    if let Ok(user_path) = env_key.get_value::<String, _>("Path") {
                        // 灞曞紑鐢ㄦ埛 PATH 涓殑鐜鍙橀噺
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

            // Initialize AgentManager state
            let agent_manager = Arc::new(RwLock::new(agent::agent_manager::AgentManager::new(app.app_handle().clone())));
            
            // 自动初始化 Agent Manager (启动 Daemon 和 Agent Loop)
            {
                let manager_clone = agent_manager.clone();
                tauri::async_runtime::block_on(async move {
                    let default_config = agent::daemon_types::DaemonConfig {
                        session_id: format!("main-session-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
                        project_path: crate::utils::paths::get_workspaces_dir().to_string_lossy().to_string(),
                        log_level: "info".to_string(),
                        max_concurrent_agents: 3,
                        workspace_dir: crate::utils::paths::get_workspaces_dir().to_string_lossy().to_string(),
                        lock_timeout_minutes: 30, // 默认 30 分钟超时
                    };
                    
                    match manager_clone.read().await.initialize(default_config).await {
                        Ok(_) => {
                            log::info!("[Main] ✓ Agent Manager automatically initialized with Daemon and Agent Loop");
                        }
                        Err(e) => {
                            log::warn!("[Main] ⚠️ Failed to auto-initialize Agent Manager: {}", e);
                            log::warn!("[Main] 💡 Agent Loop features will be unavailable until manually initialized");
                        }
                    }
                });
            }
            
            app.manage(agent_manager);
            log::info!("AgentManager state registered");

            // Initialize ObservabilityService
            let obs_service = Arc::new(services::ObservabilityService::new());
            app.manage(obs_service);
            log::info!("ObservabilityService registered");

            // Check and create workspace directories for all projects
            if let Err(e) = db::ensure_all_project_workspaces(&app.app_handle()) {
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
            agent::agent_manager::create_agent,
            agent::agent_manager::create_agent_with_cli,
            agent::agent_manager::start_agent,
            agent::agent_manager::stop_agent,
            agent::agent_manager::get_agent_status,
            agent::agent_manager::get_all_agents,
            agent::agent_manager::get_agents_by_session,
            agent::agent_manager::get_agent_manager_stats,
            agent::agent_manager::get_daemon_statuses,
            agent::agent_manager::get_all_agent_sessions,
            agent::agent_manager::initialize_agent_manager,
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
            // Worktree 管理命令 (P0: 隔离环境管理)
            agent::agent_manager::create_worktree,
            agent::agent_manager::remove_worktree,
            agent::agent_manager::list_worktrees,
            agent::agent_manager::cleanup_orphaned_worktrees,
            agent::agent_manager::get_worktree_disk_usage,

            // WebSocket Real-time Communication Commands (VC-003)
            agent::agent_manager_commands::ws_register_connection,
            agent::agent_manager_commands::ws_unregister_connection,
            agent::agent_manager_commands::ws_send_log,
            agent::agent_manager_commands::ws_send_progress,
            agent::agent_manager_commands::ws_send_status,
            agent::agent_manager_commands::ws_send_agent_response,
            agent::agent_manager_commands::ws_send_error,
            agent::agent_manager_commands::ws_send_heartbeat,
            agent::agent_manager_commands::ws_get_stats,
            agent::agent_manager_commands::ws_get_connection_count,
            agent::agent_manager_commands::ws_cleanup_stale_connections,

            // 完全去中心化 Agent Worker 命令 (Fully Decentralized System)
            agent::agent_worker_commands::start_agent_worker,
            agent::agent_worker_commands::stop_agent_worker,
            agent::agent_worker_commands::list_agent_workers,

            // PRD Quality Check commands (US-050)

            // PRD Consistency Check commands (US-051)
            commands::quality::check_prd_consistency,
            // PRD Feasibility Assessment commands (US-052)
            commands::quality::assess_prd_feasibility,
            // PRD Iteration Optimization commands (US-053)
            commands::quality::create_initial_version,
            commands::quality::iterate_prd,
            // commands::quality::get_iteration_history,  // 鏆傛椂娉ㄩ噴锛堝皻鏈疄鐜帮級
            commands::quality::rollback_to_version,
            // User Story Decomposition commands (US-XXX)
            commands::quality::decompose_user_stories,
            commands::quality::decompose_user_stories_streaming,
            // Sprint Story Assignment commands
            commands::quality::assign_stories_to_sprint_streaming,
            // User Story Persistence commands
            commands::quality::save_user_stories,
            commands::quality::get_user_stories,
            // Sprint Persistence commands
            commands::database::save_sprints,
            commands::database::get_sprints_by_project,
            commands::database::delete_sprint,
            commands::database::get_sprint_stories,
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
            commands::system::write_prd_to_file,
            commands::system::read_prd_from_file,
            commands::system::get_project_workspace_path,
            commands::system::write_file_to_project,
            commands::system::read_file_from_project,

            // Observability commands
            commands::observability::get_agent_logs,
            commands::observability::get_agent_log_stats,
            commands::observability::get_agent_traces,
            commands::observability::get_agent_alerts,
            commands::observability::clear_agent_logs,
            commands::observability::export_agent_logs,
            commands::observability::log_agent_message,
            commands::observability::record_agent_thought,
            commands::observability::record_agent_tool_call,
            commands::observability::record_agent_tool_result,
            commands::observability::record_agent_decision,
            commands::observability::record_api_call,
            commands::observability::update_performance_metrics,
            commands::observability::get_performance_metrics,
            commands::observability::resolve_agent_alert,
            commands::observability::get_alert_config,
            commands::observability::update_alert_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

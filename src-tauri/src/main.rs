// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod cli;
mod commands;
mod db;
mod file;
mod models;
mod prompts;
mod quality;
mod services;
mod utils;
pub mod agent;
pub mod agent_protocol;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async move {
                if let Err(e) = db::init_database(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // AI commands
            commands::ai::validate_ai_key,
            commands::ai::chat,
            commands::ai::stream_chat,
            commands::ai::generate_prd,
            commands::ai::generate_user_personas,
            commands::ai::generate_competitor_analysis,
            commands::ai::generate_marketing_strategy,
            commands::ai::generate_marketing_copy,
            // CLI commands
            commands::cli::detect_tools,
            commands::cli::create_cli_session,
            commands::cli::send_cli_prompt,
            commands::cli::read_cli_output,
            commands::cli::stop_cli_session,
            commands::cli::check_environment,
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
            // Agent Manager commands (VC-010, VC-016, VC-022)
            agent::agent_manager::run_initializer_agent,
            agent::agent_manager::create_merge_request,
            agent::agent_manager::run_debug_agent,
            // System commands
            commands::system::get_app_version,
            commands::system::open_external_link,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

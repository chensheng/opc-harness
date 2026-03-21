// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod cli;
mod commands;
mod db;
mod models;
mod services;
mod utils;

use commands::*;
use services::Services;
use tauri::Manager;

fn main() {
    // Initialize database
    let app_dir = utils::app_data_dir().expect("Failed to get app data directory");
    let db = db::init_db(app_dir).expect("Failed to initialize database");

    // Create services
    let services = Services::new(db);

    tauri::Builder::default()
        .setup(|app| {
            // Initialize application
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(services)
        .invoke_handler(tauri::generate_handler![
            // System commands
            greet,
            get_app_version,
            detect_tools,
            get_tool_status,
            detect_tools_detailed,
            detect_required_tools,
            get_missing_required_tools,
            all_required_tools_installed,
            get_tool_info,
            open_in_vscode,
            select_directory,
            check_db_health,
            verify_database,
            get_db_stats,
            reset_database,

            // Config commands
            get_settings,
            save_settings,
            get_config,
            set_config,
            get_config_bool,
            set_config_bool,
            get_config_i64,
            set_config_i64,
            config_exists,
            remove_config,
            get_all_configs,
            update_configs,
            reset_configs_to_defaults,
            get_theme,
            set_theme,
            get_language,
            set_language,
            get_auto_save,
            set_auto_save,

            // AI commands
            get_ai_configs,
            save_ai_config,
            remove_ai_config,
            validate_ai_key,
            generate_prd,
            has_ai_api_key,
            get_ai_key_status,

            // Project commands
            create_project,
            get_projects,
            get_project,
            update_project,
            delete_project,

            // CLI commands
            start_cli_session,
            send_cli_command,
            kill_cli_session,
            get_cli_output,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

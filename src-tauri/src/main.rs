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
            open_in_vscode,
            select_directory,
            check_db_health,

            // AI commands
            get_ai_configs,
            save_ai_config,
            remove_ai_config,
            validate_ai_key,
            generate_prd,

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

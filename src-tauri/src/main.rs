// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod db;
mod ai;
mod cli;
mod utils;

use tauri::Manager;

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
            
            // System commands
            commands::system::get_app_version,
            commands::system::open_external_link,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

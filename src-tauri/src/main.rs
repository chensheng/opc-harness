// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod models;
mod services;

use commands::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化应用
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // 系统命令
            greet,
            get_app_version,
            
            // AI配置命令
            save_ai_config,
            get_ai_config,
            validate_ai_key,
            
            // 工具检测命令
            detect_tool,
            get_tool_status,
            
            // 项目命令
            create_project,
            get_projects,
            get_project,
            update_project,
            delete_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

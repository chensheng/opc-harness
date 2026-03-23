use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersionResponse {
    pub version: String,
    pub name: String,
}

#[tauri::command]
pub fn get_app_version(app_handle: tauri::AppHandle) -> AppVersionResponse {
    let package_info = app_handle.package_info();
    AppVersionResponse {
        version: package_info.version.to_string(),
        name: package_info.name.to_string(),
    }
}

#[tauri::command]
pub async fn open_external_link(_url: String) -> Result<(), String> {
    // TODO: Implement opening external links
    Ok(())
}

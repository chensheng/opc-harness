//! Project commands

use crate::models::Project;
use crate::services::Services;
use tauri::State;

/// Create a new project
#[tauri::command]
pub fn create_project(
    services: State<'_, Services>,
    name: String,
    description: Option<String>,
    path: Option<String>,
) -> Result<Project, String> {
    services
        .project
        .create_project(name, description, path)
        .map_err(|e| e.to_string())
}

/// Get all projects
#[tauri::command]
pub fn get_projects(services: State<'_, Services>) -> Result<Vec<Project>, String> {
    services.project.get_projects().map_err(|e| e.to_string())
}

/// Get project by ID
#[tauri::command]
pub fn get_project(services: State<'_, Services>, id: String) -> Result<Option<Project>, String> {
    services.project.get_project(&id).map_err(|e| e.to_string())
}

/// Update project
#[tauri::command]
pub fn update_project(
    services: State<'_, Services>,
    id: String,
    name: Option<String>,
    description: Option<Option<String>>,
    status: Option<String>,
    path: Option<Option<String>>,
) -> Result<Project, String> {
    use crate::models::ProjectStatus;
    use crate::services::project_service::ProjectUpdate;

    let status = status.map(|s| match s.as_str() {
        "designing" => ProjectStatus::Designing,
        "coding" => ProjectStatus::Coding,
        "marketing" => ProjectStatus::Marketing,
        "completed" => ProjectStatus::Completed,
        _ => ProjectStatus::Draft,
    });

    let updates = ProjectUpdate {
        name,
        description,
        status,
        path,
    };

    services
        .project
        .update_project(&id, updates)
        .map_err(|e| e.to_string())
}

/// Delete project
#[tauri::command]
pub fn delete_project(services: State<'_, Services>, id: String) -> Result<(), String> {
    services.project.delete_project(&id).map_err(|e| e.to_string())
}

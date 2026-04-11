// Database module - organized by entity repositories

mod database;
mod repository;

// Entity repositories
mod project_repository;
mod ai_config_repository;
mod cli_session_repository;
mod agent_session_repository;
mod milestone_repository;
mod issue_repository;
mod user_story_repository;

// Re-export database functions
pub use database::{get_connection, init_database, ensure_all_project_workspaces};

// Re-export repository trait and generic implementation
pub use repository::Entity;
#[allow(unused_imports)]
pub use repository::Repository;

// Re-export Project CRUD operations
pub use project_repository::{
    create_project, delete_project, get_all_projects, get_project_by_id, update_project,
};

// Re-export AI Config CRUD operations
pub use ai_config_repository::{
    delete_ai_config, get_ai_config, get_all_ai_configs, save_ai_config,
};

// Re-export CLI Session CRUD operations
pub use cli_session_repository::{
    create_cli_session, delete_cli_session, get_all_cli_sessions, get_cli_session_by_id,
};

// Re-export Agent Session CRUD operations
pub use agent_session_repository::{
    create_agent_session, delete_agent_session, get_agent_session_by_id,
    get_agent_session_by_session_id, get_all_agent_sessions, get_sessions_by_project,
    update_agent_session, update_agent_session_status,
};

// Re-export Milestone CRUD operations
pub use milestone_repository::{
    create_milestone, delete_milestone, get_milestone_by_id, get_milestones_by_project,
    update_milestone,
};

// Re-export Issue CRUD operations
pub use issue_repository::{
    create_issue, delete_issue, get_issue_by_id, get_issues_by_milestone, get_issues_by_project,
    update_issue,
};

// Re-export User Story CRUD operations
pub use user_story_repository::{get_user_stories_by_project, upsert_user_stories};

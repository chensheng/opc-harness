//! Service layer

pub mod project_service;
pub mod ai_service;
pub mod cli_service;
pub mod tool_detection;

use std::sync::Arc;

/// Service container
pub struct Services {
    pub project: Arc<project_service::ProjectService>,
    pub ai: Arc<ai_service::AIService>,
    pub cli: Arc<cli_service::CLIService>,
    pub tool_detection: Arc<tool_detection::ToolDetectionService>,
}

impl Services {
    pub fn new(db: rusqlite::Connection) -> Self {
        let db = Arc::new(std::sync::Mutex::new(db));
        
        Self {
            project: Arc::new(project_service::ProjectService::new(db.clone())),
            ai: Arc::new(ai_service::AIService::new()),
            cli: Arc::new(cli_service::CLIService::new()),
            tool_detection: Arc::new(tool_detection::ToolDetectionService::new()),
        }
    }
}

//! Project service

use crate::models::{Project, ProjectStatus};
use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};

pub struct ProjectService {
    db: Arc<Mutex<rusqlite::Connection>>,
}

impl ProjectService {
    pub fn new(db: Arc<Mutex<rusqlite::Connection>>) -> Self {
        Self { db }
    }

    /// Get database connection (for health checks)
    pub fn get_db(&self) -> Arc<Mutex<rusqlite::Connection>> {
        self.db.clone()
    }

    /// Create a new project
    pub fn create_project(
        &self,
        name: String,
        description: Option<String>,
        path: Option<String>,
    ) -> Result<Project> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let project = Project {
            id: id.clone(),
            name,
            description,
            status: ProjectStatus::Draft,
            created_at: now,
            updated_at: now,
            path,
        };

        let db = self.db.lock().unwrap();
        db.execute(
            "INSERT INTO projects (id, name, description, status, created_at, updated_at, path) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                project.id,
                project.name,
                project.description,
                format!("{:?}", project.status).to_lowercase(),
                project.created_at,
                project.updated_at,
                project.path
            ],
        )
        .context("Failed to create project")?;

        Ok(project)
    }

    /// Get all projects
    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let db = self.db.lock().unwrap();
        let mut stmt = db
            .prepare("SELECT id, name, description, status, created_at, updated_at, path FROM projects ORDER BY updated_at DESC")
            .context("Failed to prepare statement")?;

        let projects = stmt
            .query_map([], |row| {
                let status_str: String = row.get(3)?;
                let status = match status_str.as_str() {
                    "designing" => ProjectStatus::Designing,
                    "coding" => ProjectStatus::Coding,
                    "marketing" => ProjectStatus::Marketing,
                    "completed" => ProjectStatus::Completed,
                    _ => ProjectStatus::Draft,
                };

                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                    path: row.get(6)?,
                })
            })
            .context("Failed to query projects")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect projects")?;

        Ok(projects)
    }

    /// Get project by ID
    pub fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let db = self.db.lock().unwrap();
        let mut stmt = db
            .prepare("SELECT id, name, description, status, created_at, updated_at, path FROM projects WHERE id = ?1")
            .context("Failed to prepare statement")?;

        let project = stmt
            .query_map([id], |row| {
                let status_str: String = row.get(3)?;
                let status = match status_str.as_str() {
                    "designing" => ProjectStatus::Designing,
                    "coding" => ProjectStatus::Coding,
                    "marketing" => ProjectStatus::Marketing,
                    "completed" => ProjectStatus::Completed,
                    _ => ProjectStatus::Draft,
                };

                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                    path: row.get(6)?,
                })
            })
            .context("Failed to query project")?
            .next()
            .transpose()
            .context("Failed to parse project")?;

        Ok(project)
    }

    /// Update project
    pub fn update_project(&self, id: &str, updates: ProjectUpdate) -> Result<Project> {
        let now = chrono::Utc::now().timestamp();
        let db = self.db.lock().unwrap();

        if let Some(name) = updates.name {
            db.execute(
                "UPDATE projects SET name = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![name, now, id],
            )?;
        }

        if let Some(description) = updates.description {
            db.execute(
                "UPDATE projects SET description = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![description, now, id],
            )?;
        }

        if let Some(status) = updates.status {
            db.execute(
                "UPDATE projects SET status = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![format!("{:?}", status).to_lowercase(), now, id],
            )?;
        }

        if let Some(path) = updates.path {
            db.execute(
                "UPDATE projects SET path = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![path, now, id],
            )?;
        }

        drop(db);
        self.get_project(id)?.context("Project not found after update")
    }

    /// Delete project
    pub fn delete_project(&self, id: &str) -> Result<()> {
        let db = self.db.lock().unwrap();
        db.execute("DELETE FROM projects WHERE id = ?1", [id])
            .context("Failed to delete project")?;
        Ok(())
    }
}

/// Project update data
#[derive(Debug, Default)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<ProjectStatus>,
    pub path: Option<Option<String>>,
}

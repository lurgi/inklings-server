use crate::errors::ServiceError;
use crate::models::project_dto::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use crate::repositories::ProjectRepository;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct ProjectService {
    project_repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            project_repo: ProjectRepository::new(db),
        }
    }

    pub async fn create_project(
        &self,
        _user_id: i32,
        _req: CreateProjectRequest,
    ) -> Result<ProjectResponse, ServiceError> {
        todo!("Phase 1 구현 예정")
    }

    pub async fn list_projects(&self, _user_id: i32) -> Result<Vec<ProjectResponse>, ServiceError> {
        todo!("Phase 1 구현 예정")
    }

    pub async fn get_project(
        &self,
        _user_id: i32,
        _project_id: i32,
    ) -> Result<ProjectResponse, ServiceError> {
        todo!("Phase 1 구현 예정")
    }

    pub async fn update_project(
        &self,
        _user_id: i32,
        _project_id: i32,
        _req: UpdateProjectRequest,
    ) -> Result<ProjectResponse, ServiceError> {
        todo!("Phase 1 구현 예정")
    }

    pub async fn delete_project(
        &self,
        _user_id: i32,
        _project_id: i32,
    ) -> Result<(), ServiceError> {
        todo!("Phase 1 구현 예정")
    }
}

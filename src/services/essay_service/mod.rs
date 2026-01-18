use crate::errors::ServiceError;
use crate::models::essay_dto::{CreateEssayRequest, EssayResponse, UpdateEssayRequest};
use crate::repositories::{EssayRepository, ProjectRepository};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct EssayService {
    essay_repo: EssayRepository,
    project_repo: ProjectRepository,
}

impl EssayService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            essay_repo: EssayRepository::new(db.clone()),
            project_repo: ProjectRepository::new(db),
        }
    }

    pub async fn create_essay(
        &self,
        user_id: i32,
        req: CreateEssayRequest,
    ) -> Result<EssayResponse, ServiceError> {
        todo!("Implement create_essay")
    }

    pub async fn get_essay(
        &self,
        user_id: i32,
        essay_id: i32,
    ) -> Result<EssayResponse, ServiceError> {
        todo!("Implement get_essay")
    }

    pub async fn list_essays_by_project(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<Vec<EssayResponse>, ServiceError> {
        todo!("Implement list_essays_by_project")
    }

    pub async fn update_essay(
        &self,
        user_id: i32,
        essay_id: i32,
        req: UpdateEssayRequest,
    ) -> Result<EssayResponse, ServiceError> {
        todo!("Implement update_essay")
    }

    pub async fn delete_essay(&self, user_id: i32, essay_id: i32) -> Result<(), ServiceError> {
        todo!("Implement delete_essay")
    }
}

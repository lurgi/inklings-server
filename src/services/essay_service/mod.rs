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
        // Project 권한 검증
        let project = self
            .project_repo
            .find_by_id(req.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let essay = self
            .essay_repo
            .create(req.project_id, req.title.clone(), req.content.clone())
            .await?;

        Ok(EssayResponse::from(essay))
    }

    pub async fn get_essay(
        &self,
        user_id: i32,
        essay_id: i32,
    ) -> Result<EssayResponse, ServiceError> {
        let essay = self
            .essay_repo
            .find_by_id(essay_id)
            .await?
            .ok_or(ServiceError::EssayNotFound)?;

        // 권한 검증: essay → project → user
        let project = self
            .project_repo
            .find_by_id(essay.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        Ok(EssayResponse::from(essay))
    }

    pub async fn list_essays_by_project(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<Vec<EssayResponse>, ServiceError> {
        // Project 권한 검증
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let essays = self.essay_repo.find_by_project_id(project_id).await?;
        Ok(essays.into_iter().map(EssayResponse::from).collect())
    }

    pub async fn update_essay(
        &self,
        user_id: i32,
        essay_id: i32,
        req: UpdateEssayRequest,
    ) -> Result<EssayResponse, ServiceError> {
        let essay = self
            .essay_repo
            .find_by_id(essay_id)
            .await?
            .ok_or(ServiceError::EssayNotFound)?;

        // 권한 검증: essay → project → user
        let project = self
            .project_repo
            .find_by_id(essay.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let updated_essay = self
            .essay_repo
            .update(essay_id, req.title.clone(), req.content.clone())
            .await?;

        Ok(EssayResponse::from(updated_essay))
    }

    pub async fn delete_essay(&self, user_id: i32, essay_id: i32) -> Result<(), ServiceError> {
        let essay = self
            .essay_repo
            .find_by_id(essay_id)
            .await?
            .ok_or(ServiceError::EssayNotFound)?;

        // 권한 검증: essay → project → user
        let project = self
            .project_repo
            .find_by_id(essay.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        self.essay_repo.delete(essay_id).await?;

        Ok(())
    }
}

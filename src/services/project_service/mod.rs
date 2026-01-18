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
        user_id: i32,
        req: CreateProjectRequest,
    ) -> Result<ProjectResponse, ServiceError> {
        // 중복 검사
        if let Some(_) = self
            .project_repo
            .find_by_user_id_and_name(user_id, &req.name)
            .await?
        {
            return Err(ServiceError::ProjectNameAlreadyExists);
        }

        let project = self
            .project_repo
            .create(user_id, req.name, req.description)
            .await?;

        Ok(ProjectResponse::from(project))
    }

    pub async fn list_projects(&self, user_id: i32) -> Result<Vec<ProjectResponse>, ServiceError> {
        let projects = self.project_repo.find_by_user_id(user_id).await?;
        Ok(projects.into_iter().map(ProjectResponse::from).collect())
    }

    pub async fn get_project(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<ProjectResponse, ServiceError> {
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        // 권한 검증
        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        Ok(ProjectResponse::from(project))
    }

    pub async fn update_project(
        &self,
        user_id: i32,
        project_id: i32,
        req: UpdateProjectRequest,
    ) -> Result<ProjectResponse, ServiceError> {
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        // 권한 검증
        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        // 이름 변경 시 중복 검사
        if let Some(ref new_name) = req.name {
            if new_name != &project.name {
                if let Some(_) = self
                    .project_repo
                    .find_by_user_id_and_name(user_id, new_name)
                    .await?
                {
                    return Err(ServiceError::ProjectNameAlreadyExists);
                }
            }
        }

        let updated = self
            .project_repo
            .update(project_id, req.name, req.description)
            .await?;

        Ok(ProjectResponse::from(updated))
    }

    pub async fn delete_project(&self, user_id: i32, project_id: i32) -> Result<(), ServiceError> {
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        // 권한 검증
        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        self.project_repo.delete(project_id).await?;
        Ok(())
    }
}

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
    clients::Embedder,
    errors::ServiceError,
    models::{CreateMemoRequest, MemoResponse, UpdateMemoRequest},
    repositories::{MemoRepository, ProjectRepository, QdrantRepo},
};

#[derive(Clone)]
pub struct MemoService {
    memo_repo: MemoRepository,
    project_repo: ProjectRepository,
    qdrant_repo: Arc<dyn QdrantRepo>,
    embedder: Arc<dyn Embedder>,
}

impl MemoService {
    pub fn new(
        db: Arc<DatabaseConnection>,
        qdrant_repo: Arc<dyn QdrantRepo>,
        embedder: Arc<dyn Embedder>,
    ) -> Self {
        Self {
            memo_repo: MemoRepository::new(db.clone()),
            project_repo: ProjectRepository::new(db),
            qdrant_repo,
            embedder,
        }
    }

    pub async fn create_memo(
        &self,
        user_id: i32,
        req: CreateMemoRequest,
    ) -> Result<MemoResponse, ServiceError> {
        // Project 권한 검증
        let project = self
            .project_repo
            .find_by_id(req.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let memo = self
            .memo_repo
            .create(req.project_id, req.content.clone())
            .await?;

        let vector = self.embedder.embed(&req.content).await?;
        self.qdrant_repo
            .upsert_memo(memo.id, req.project_id, vector)
            .await?;

        Ok(MemoResponse::from(memo))
    }

    pub async fn get_memo(&self, user_id: i32, memo_id: i32) -> Result<MemoResponse, ServiceError> {
        let memo = self
            .memo_repo
            .find_by_id(memo_id)
            .await?
            .ok_or(ServiceError::MemoNotFound)?;

        // 권한 검증: memo → project → user
        let project = self
            .project_repo
            .find_by_id(memo.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        Ok(MemoResponse::from(memo))
    }

    pub async fn list_memos_by_project(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<Vec<MemoResponse>, ServiceError> {
        // Project 권한 검증
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let memos = self.memo_repo.find_by_project_id(project_id).await?;
        Ok(memos.into_iter().map(MemoResponse::from).collect())
    }

    pub async fn update_memo(
        &self,
        user_id: i32,
        memo_id: i32,
        req: UpdateMemoRequest,
    ) -> Result<MemoResponse, ServiceError> {
        let memo = self
            .memo_repo
            .find_by_id(memo_id)
            .await?
            .ok_or(ServiceError::MemoNotFound)?;

        // 권한 검증: memo → project → user
        let project = self
            .project_repo
            .find_by_id(memo.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let updated_memo = self.memo_repo.update(memo_id, req.content.clone()).await?;

        let vector = self.embedder.embed(&req.content).await?;
        self.qdrant_repo
            .upsert_memo(memo_id, memo.project_id, vector)
            .await?;

        Ok(MemoResponse::from(updated_memo))
    }

    pub async fn delete_memo(&self, user_id: i32, memo_id: i32) -> Result<(), ServiceError> {
        let memo = self
            .memo_repo
            .find_by_id(memo_id)
            .await?
            .ok_or(ServiceError::MemoNotFound)?;

        // 권한 검증: memo → project → user
        let project = self
            .project_repo
            .find_by_id(memo.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        self.memo_repo.delete(memo_id).await?;
        self.qdrant_repo.delete_memo(memo_id).await?;

        Ok(())
    }

    pub async fn toggle_pin(
        &self,
        user_id: i32,
        memo_id: i32,
    ) -> Result<MemoResponse, ServiceError> {
        let memo = self
            .memo_repo
            .find_by_id(memo_id)
            .await?
            .ok_or(ServiceError::MemoNotFound)?;

        // 권한 검증: memo → project → user
        let project = self
            .project_repo
            .find_by_id(memo.project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let updated_memo = self.memo_repo.toggle_pin(memo_id).await?;
        Ok(MemoResponse::from(updated_memo))
    }
}

#[cfg(test)]
mod tests;

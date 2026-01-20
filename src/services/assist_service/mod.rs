use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
    clients::{Embedder, TextGenerator},
    errors::ServiceError,
    models::assist_dto::{AssistRequest, AssistResponse, SimilarMemo},
    repositories::{MemoRepository, ProjectRepository, QdrantRepo},
};

#[derive(Clone)]
pub struct AssistService {
    memo_repo: MemoRepository,
    project_repo: ProjectRepository,
    qdrant_repo: Arc<dyn QdrantRepo>,
    embedder: Arc<dyn Embedder>,
    text_generator: Arc<dyn TextGenerator>,
}

impl AssistService {
    pub fn new(
        db: Arc<DatabaseConnection>,
        qdrant_repo: Arc<dyn QdrantRepo>,
        embedder: Arc<dyn Embedder>,
        text_generator: Arc<dyn TextGenerator>,
    ) -> Self {
        Self {
            memo_repo: MemoRepository::new(db.clone()),
            project_repo: ProjectRepository::new(db),
            qdrant_repo,
            embedder,
            text_generator,
        }
    }

    pub async fn get_assistance(
        &self,
        user_id: i32,
        project_id: i32,
        req: AssistRequest,
    ) -> Result<AssistResponse, ServiceError> {
        // Project 권한 검증
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await?
            .ok_or(ServiceError::ProjectNotFound)?;

        if project.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        let query_vector = self.embedder.embed(&req.prompt).await?;

        let similar_memo_ids = self
            .qdrant_repo
            .search_similar(project_id, query_vector, req.limit)
            .await?;

        let mut similar_memos = Vec::new();
        let mut context = Vec::new();

        for memo_id in similar_memo_ids {
            if let Some(memo) = self.memo_repo.find_by_id(memo_id).await? {
                if memo.project_id == project_id {
                    context.push(memo.content.clone());
                    similar_memos.push(SimilarMemo {
                        id: memo.id,
                        content: memo.content,
                        created_at: memo.created_at,
                    });
                }
            }
        }

        let suggestion = self.text_generator.generate(&req.prompt, context).await?;

        Ok(AssistResponse {
            suggestion,
            similar_memos,
        })
    }
}

#[cfg(test)]
mod tests;

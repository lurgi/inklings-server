use chrono::Utc;
use sea_orm::*;
use std::sync::Arc;

use crate::entities::essay::{self, Entity as Essay};

#[derive(Clone)]
pub struct EssayRepository {
    db: Arc<DatabaseConnection>,
}

impl EssayRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<essay::Model>, DbErr> {
        Essay::find_by_id(id).one(self.db.as_ref()).await
    }

    pub async fn find_by_project_id(&self, project_id: i32) -> Result<Vec<essay::Model>, DbErr> {
        Essay::find()
            .filter(essay::Column::ProjectId.eq(project_id))
            .order_by_desc(essay::Column::IsPinned)
            .order_by_desc(essay::Column::UpdatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn create(
        &self,
        project_id: i32,
        title: String,
        content: String,
    ) -> Result<essay::Model, DbErr> {
        let now = Utc::now().naive_utc();

        let active_model = essay::ActiveModel {
            project_id: Set(project_id),
            title: Set(title),
            content: Set(content),
            is_pinned: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        active_model.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        id: i32,
        title: String,
        content: String,
    ) -> Result<essay::Model, DbErr> {
        let essay = self
            .find_by_id(id)
            .await?
            .ok_or(DbErr::RecordNotFound("Essay not found".into()))?;

        let mut active_model: essay::ActiveModel = essay.into();
        active_model.title = Set(title);
        active_model.content = Set(content);
        active_model.updated_at = Set(Utc::now().naive_utc());

        active_model.update(self.db.as_ref()).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Essay::delete_by_id(id).exec(self.db.as_ref()).await
    }
}

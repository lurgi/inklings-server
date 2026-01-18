use chrono::Utc;
use sea_orm::*;
use std::sync::Arc;

use crate::entities::memo::{self, Entity as Memo};

#[derive(Clone)]
pub struct MemoRepository {
    db: Arc<DatabaseConnection>,
}

impl MemoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<memo::Model>, DbErr> {
        Memo::find_by_id(id).one(self.db.as_ref()).await
    }

    pub async fn find_by_project_id(&self, project_id: i32) -> Result<Vec<memo::Model>, DbErr> {
        Memo::find()
            .filter(memo::Column::ProjectId.eq(project_id))
            .order_by_desc(memo::Column::IsPinned)
            .order_by_desc(memo::Column::UpdatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn create(&self, project_id: i32, content: String) -> Result<memo::Model, DbErr> {
        let now = Utc::now().naive_utc();

        let active_model = memo::ActiveModel {
            project_id: Set(project_id),
            content: Set(content),
            is_pinned: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        active_model.insert(self.db.as_ref()).await
    }

    pub async fn update(&self, id: i32, content: String) -> Result<memo::Model, DbErr> {
        let memo = self
            .find_by_id(id)
            .await?
            .ok_or(DbErr::RecordNotFound("Memo not found".into()))?;

        let mut active_model: memo::ActiveModel = memo.into();
        active_model.content = Set(content);
        active_model.updated_at = Set(Utc::now().naive_utc());

        active_model.update(self.db.as_ref()).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Memo::delete_by_id(id).exec(self.db.as_ref()).await
    }

    pub async fn toggle_pin(&self, id: i32) -> Result<memo::Model, DbErr> {
        let memo = self
            .find_by_id(id)
            .await?
            .ok_or(DbErr::RecordNotFound("Memo not found".into()))?;

        let mut active_model: memo::ActiveModel = memo.clone().into();
        active_model.is_pinned = Set(!memo.is_pinned);
        active_model.updated_at = Set(Utc::now().naive_utc());

        active_model.update(self.db.as_ref()).await
    }
}

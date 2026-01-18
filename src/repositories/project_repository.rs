use chrono::Utc;
use sea_orm::*;
use std::sync::Arc;

use crate::entities::project::{self, Entity as Project};

#[derive(Clone)]
pub struct ProjectRepository {
    db: Arc<DatabaseConnection>,
}

impl ProjectRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<project::Model>, DbErr> {
        Project::find_by_id(id).one(self.db.as_ref()).await
    }

    pub async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<project::Model>, DbErr> {
        Project::find()
            .filter(project::Column::UserId.eq(user_id))
            .order_by_desc(project::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn find_by_user_id_and_name(
        &self,
        user_id: i32,
        name: &str,
    ) -> Result<Option<project::Model>, DbErr> {
        Project::find()
            .filter(project::Column::UserId.eq(user_id))
            .filter(project::Column::Name.eq(name))
            .one(self.db.as_ref())
            .await
    }

    pub async fn create(
        &self,
        user_id: i32,
        name: String,
        description: Option<String>,
    ) -> Result<project::Model, DbErr> {
        let now = Utc::now().naive_utc();

        let active_model = project::ActiveModel {
            user_id: Set(user_id),
            name: Set(name),
            description: Set(description),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        active_model.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<project::Model, DbErr> {
        let project = self
            .find_by_id(id)
            .await?
            .ok_or(DbErr::RecordNotFound("Project not found".into()))?;

        let mut active_model: project::ActiveModel = project.into();

        if let Some(name) = name {
            active_model.name = Set(name);
        }
        if let Some(description) = description {
            active_model.description = Set(description);
        }
        active_model.updated_at = Set(Utc::now().naive_utc());

        active_model.update(self.db.as_ref()).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Project::delete_by_id(id).exec(self.db.as_ref()).await
    }
}

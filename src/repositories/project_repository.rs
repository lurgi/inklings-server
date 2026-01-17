use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectRepository {
    _db: Arc<DatabaseConnection>,
}

impl ProjectRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { _db: db }
    }
}

mod health_handler;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn create_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/", get(health_handler::health_check))
        .with_state(db)
}

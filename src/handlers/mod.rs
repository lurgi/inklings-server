pub mod auth;
pub mod health_handler;
pub mod memo_handler;

use crate::{
    clients::{Embedder, GeminiClient},
    repositories::QdrantRepository,
    services::memo_service::MemoService,
};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub memo_service: Arc<MemoService>,
}

pub fn create_router(
    db: Arc<DatabaseConnection>,
    qdrant_repo: QdrantRepository,
    gemini_client: Arc<GeminiClient>,
) -> Router {
    let memo_service = Arc::new(MemoService::new(
        db.clone(),
        qdrant_repo,
        gemini_client as Arc<dyn Embedder>,
    ));

    let app_state = AppState { db, memo_service };

    Router::new()
        .route("/api/health", get(health_handler::health_check))
        .nest(
            "/api/memos",
            Router::new()
                .route("/", post(memo_handler::create_memo))
                .route("/", get(memo_handler::list_memos))
                .route("/:id", get(memo_handler::get_memo))
                .route("/:id", put(memo_handler::update_memo))
                .route("/:id", delete(memo_handler::delete_memo))
                .route("/:id/pin", patch(memo_handler::toggle_pin)),
        )
        .with_state(app_state)
}

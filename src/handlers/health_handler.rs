use axum::{extract::State, http::StatusCode, Json};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

pub async fn health_check(
    State(db): State<Arc<DatabaseConnection>>,
) -> (StatusCode, Json<HealthResponse>) {
    let db_status = match db.ping().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
    };

    (StatusCode::OK, Json(response))
}

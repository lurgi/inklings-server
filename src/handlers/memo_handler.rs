use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::{auth::AuthenticatedUser, AppState};
use crate::models::memo_dto::{CreateMemoRequest, MemoResponse, UpdateMemoRequest};

pub async fn create_memo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateMemoRequest>,
) -> impl IntoResponse {
    match state.memo_service.create_memo(user.id, payload).await {
        Ok(memo) => (StatusCode::CREATED, Json(MemoResponse::from(memo))).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn list_memos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match state.memo_service.list_memos(user.id).await {
        Ok(memos) => {
            let memo_responses: Vec<MemoResponse> =
                memos.into_iter().map(MemoResponse::from).collect();
            (StatusCode::OK, Json(memo_responses)).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn get_memo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.memo_service.get_memo(user.id, id).await {
        Ok(memo) => (StatusCode::OK, Json(MemoResponse::from(memo))).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn update_memo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMemoRequest>,
) -> impl IntoResponse {
    match state.memo_service.update_memo(user.id, id, payload).await {
        Ok(memo) => (StatusCode::OK, Json(MemoResponse::from(memo))).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn delete_memo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.memo_service.delete_memo(user.id, id).await {
        Ok(()) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn toggle_pin(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.memo_service.toggle_pin(user.id, id).await {
        Ok(memo) => (StatusCode::OK, Json(MemoResponse::from(memo))).into_response(),
        Err(e) => e.into_response(),
    }
}

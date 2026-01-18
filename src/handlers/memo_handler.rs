use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use super::{auth::AuthenticatedUser, AppState};
use crate::errors::ErrorResponse;
use crate::models::memo_dto::{CreateMemoRequest, MemoResponse, UpdateMemoRequest};

#[derive(Debug, Deserialize)]
pub struct ListMemosParams {
    pub project_id: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/memos",
    tag = "Memos",
    request_body = CreateMemoRequest,
    responses(
        (status = 201, description = "메모 생성 성공", body = MemoResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    get,
    path = "/api/memos",
    tag = "Memos",
    params(
        ("project_id" = Option<i32>, Query, description = "프로젝트 ID (없으면 사용자의 모든 프로젝트의 메모 조회)")
    ),
    responses(
        (status = 200, description = "메모 목록 조회 성공", body = Vec<MemoResponse>),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_memos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ListMemosParams>,
) -> impl IntoResponse {
    if let Some(project_id) = params.project_id {
        let memos = state
            .memo_service
            .list_memos_by_project(user.id, project_id)
            .await;
        match memos {
            Ok(memos) => (StatusCode::OK, Json(memos)).into_response(),
            Err(e) => e.into_response(),
        }
    } else {
        let projects = state.project_service.list_projects(user.id).await;
        match projects {
            Ok(project_list) => {
                let mut all_memos = Vec::new();
                for project in project_list {
                    if let Ok(memos) = state
                        .memo_service
                        .list_memos_by_project(user.id, project.id)
                        .await
                    {
                        all_memos.extend(memos);
                    }
                }
                (StatusCode::OK, Json(all_memos)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/memos/{id}",
    tag = "Memos",
    params(
        ("id" = i32, Path, description = "메모 ID")
    ),
    responses(
        (status = 200, description = "메모 조회 성공", body = MemoResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "메모를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    put,
    path = "/api/memos/{id}",
    tag = "Memos",
    params(
        ("id" = i32, Path, description = "메모 ID")
    ),
    request_body = UpdateMemoRequest,
    responses(
        (status = 200, description = "메모 수정 성공", body = MemoResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "메모를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    delete,
    path = "/api/memos/{id}",
    tag = "Memos",
    params(
        ("id" = i32, Path, description = "메모 ID")
    ),
    responses(
        (status = 204, description = "메모 삭제 성공"),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "메모를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    patch,
    path = "/api/memos/{id}/pin",
    tag = "Memos",
    params(
        ("id" = i32, Path, description = "메모 ID")
    ),
    responses(
        (status = 200, description = "메모 고정 토글 성공", body = MemoResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "메모를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
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

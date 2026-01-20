use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use validator::Validate;

use super::{auth::AuthenticatedUser, AppState};
use crate::errors::ErrorResponse;
use crate::models::essay_dto::{CreateEssayRequest, EssayResponse, UpdateEssayRequest};

#[derive(Debug, Deserialize)]
pub struct ListEssaysParams {
    pub project_id: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/essays",
    tag = "Essays",
    request_body = CreateEssayRequest,
    responses(
        (status = 201, description = "에세이 생성 성공", body = EssayResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_essay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateEssayRequest>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Validation failed: {}", e) })),
        )
            .into_response();
    }

    match state.essay_service.create_essay(user.id, payload).await {
        Ok(essay) => (StatusCode::CREATED, Json(essay)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/essays",
    tag = "Essays",
    params(
        ("project_id" = Option<i32>, Query, description = "프로젝트 ID (없으면 사용자의 모든 프로젝트의 에세이 조회)")
    ),
    responses(
        (status = 200, description = "에세이 목록 조회 성공", body = Vec<EssayResponse>),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_essays(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ListEssaysParams>,
) -> impl IntoResponse {
    if let Some(project_id) = params.project_id {
        let essays = state
            .essay_service
            .list_essays_by_project(user.id, project_id)
            .await;
        match essays {
            Ok(essays) => (StatusCode::OK, Json(essays)).into_response(),
            Err(e) => e.into_response(),
        }
    } else {
        let projects = state.project_service.list_projects(user.id).await;
        match projects {
            Ok(project_list) => {
                let mut all_essays = Vec::new();
                for project in project_list {
                    if let Ok(essays) = state
                        .essay_service
                        .list_essays_by_project(user.id, project.id)
                        .await
                    {
                        all_essays.extend(essays);
                    }
                }
                (StatusCode::OK, Json(all_essays)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/essays/{id}",
    tag = "Essays",
    params(
        ("id" = i32, Path, description = "에세이 ID")
    ),
    responses(
        (status = 200, description = "에세이 조회 성공", body = EssayResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "에세이를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_essay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.essay_service.get_essay(user.id, id).await {
        Ok(essay) => (StatusCode::OK, Json(essay)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/essays/{id}",
    tag = "Essays",
    params(
        ("id" = i32, Path, description = "에세이 ID")
    ),
    request_body = UpdateEssayRequest,
    responses(
        (status = 200, description = "에세이 수정 성공", body = EssayResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "에세이를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_essay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEssayRequest>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Validation failed: {}", e) })),
        )
            .into_response();
    }

    match state.essay_service.update_essay(user.id, id, payload).await {
        Ok(essay) => (StatusCode::OK, Json(essay)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/essays/{id}",
    tag = "Essays",
    params(
        ("id" = i32, Path, description = "에세이 ID")
    ),
    responses(
        (status = 204, description = "에세이 삭제 성공"),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "에세이를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_essay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.essay_service.delete_essay(user.id, id).await {
        Ok(()) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => e.into_response(),
    }
}

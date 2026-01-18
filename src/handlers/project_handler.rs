use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::{auth::AuthenticatedUser, AppState};
use crate::errors::ErrorResponse;
use crate::models::project_dto::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};

#[utoipa::path(
    post,
    path = "/api/projects",
    tag = "Projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "프로젝트 생성 성공", body = ProjectResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 409, description = "프로젝트 이름 이미 존재", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_project(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    match state
        .project_service
        .create_project(user.id, payload)
        .await
    {
        Ok(project) => (StatusCode::CREATED, Json(project)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/projects",
    tag = "Projects",
    responses(
        (status = 200, description = "프로젝트 목록 조회 성공", body = Vec<ProjectResponse>),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_projects(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match state.project_service.list_projects(user.id).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    tag = "Projects",
    params(
        ("id" = i32, Path, description = "프로젝트 ID")
    ),
    responses(
        (status = 200, description = "프로젝트 조회 성공", body = ProjectResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_project(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.project_service.get_project(user.id, id).await {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}",
    tag = "Projects",
    params(
        ("id" = i32, Path, description = "프로젝트 ID")
    ),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "프로젝트 수정 성공", body = ProjectResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 409, description = "프로젝트 이름 이미 존재", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_project(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    match state
        .project_service
        .update_project(user.id, id, payload)
        .await
    {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/projects/{id}",
    tag = "Projects",
    params(
        ("id" = i32, Path, description = "프로젝트 ID")
    ),
    responses(
        (status = 204, description = "프로젝트 삭제 성공"),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 403, description = "권한 없음", body = ErrorResponse),
        (status = 404, description = "프로젝트를 찾을 수 없음", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_project(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.project_service.delete_project(user.id, id).await {
        Ok(()) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => e.into_response(),
    }
}

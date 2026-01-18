use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::{auth::AuthenticatedUser, AppState};
use crate::errors::ErrorResponse;
use crate::models::assist_dto::{AssistRequest, AssistResponse};

#[utoipa::path(
    post,
    path = "/api/assist",
    tag = "Assist",
    request_body = AssistRequest,
    responses(
        (status = 200, description = "AI 어시스턴트 응답 성공", body = AssistResponse),
        (status = 400, description = "잘못된 요청", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 500, description = "서버 에러", body = ErrorResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn assist(
    State(_state): State<AppState>,
    _user: AuthenticatedUser,
    Json(_payload): Json<AssistRequest>,
) -> impl IntoResponse {
    todo!("Phase 4: Handler 구현 예정")
}

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::Deserialize;

// 여러 핸들러에서 공통으로 사용될 인증된 사용자 정보
#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub id: i32,
}

// TODO: 실제 인증 미들웨어 구현 후, 아래 로직을 안전한 토큰 검증 로직으로 교체해야 함
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    // 에러는 `(StatusCode, &str)` 튜플로 정의
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // HTTP 헤더에서 "X-User-Id" 값을 추출하는 임시 로직
        if let Some(user_id_header) = parts.headers.get("X-User-Id") {
            if let Ok(user_id_str) = user_id_header.to_str() {
                if let Ok(id) = user_id_str.parse::<i32>() {
                    // 성공적으로 ID를 파싱하면 AuthenticatedUser를 반환
                    return Ok(AuthenticatedUser { id });
                }
            }
        }

        // 헤더가 없거나 파싱에 실패하면 401 Unauthorized 에러 반환
        Err((
            StatusCode::UNAUTHORIZED,
            "Missing or invalid X-User-Id header",
        ))
    }
}

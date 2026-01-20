use utoipa::{Modify, OpenApi};

use crate::entities::oauth_account::OAuthProvider;
use crate::errors::ErrorResponse;
use crate::handlers::health_handler::HealthResponse;
use crate::models::assist_dto::{AssistRequest, AssistResponse, SimilarMemo};
use crate::models::essay_dto::{CreateEssayRequest, EssayResponse, UpdateEssayRequest};
use crate::models::memo_dto::{CreateMemoRequest, MemoResponse, UpdateMemoRequest};
use crate::models::project_dto::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use crate::models::user_dto::{AuthResponse, LogoutResponse, OAuthLoginRequest, UserResponse};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lekha Server API",
        version = "0.1.0",
        description = "당신의 생각이 글이 되도록 돕습니다\n\n## 인증\nOAuth 소셜 로그인(Google, Kakao, Naver)을 통해 사용자 인증을 수행합니다.\n로그인 시 HttpOnly 쿠키로 JWT 토큰이 자동 설정되며, 이후 모든 API 요청에 쿠키가 자동으로 포함됩니다.\n\n- Access Token: 15분 (자동 갱신)\n- Refresh Token: 7일 (Rotation 방식)"
    ),
    paths(
        crate::handlers::health_handler::health_check,
        crate::handlers::user_handler::oauth_login,
        crate::handlers::auth_handler::refresh,
        crate::handlers::auth_handler::logout,
        crate::handlers::auth_handler::logout_all,
        crate::handlers::project_handler::create_project,
        crate::handlers::project_handler::list_projects,
        crate::handlers::project_handler::get_project,
        crate::handlers::project_handler::update_project,
        crate::handlers::project_handler::delete_project,
        crate::handlers::essay_handler::create_essay,
        crate::handlers::essay_handler::list_essays,
        crate::handlers::essay_handler::get_essay,
        crate::handlers::essay_handler::update_essay,
        crate::handlers::essay_handler::delete_essay,
        crate::handlers::memo_handler::create_memo,
        crate::handlers::memo_handler::list_memos,
        crate::handlers::memo_handler::get_memo,
        crate::handlers::memo_handler::update_memo,
        crate::handlers::memo_handler::delete_memo,
        crate::handlers::memo_handler::toggle_pin,
        crate::handlers::assist_handler::assist,
    ),
    components(
        schemas(
            HealthResponse,
            OAuthLoginRequest,
            UserResponse,
            AuthResponse,
            LogoutResponse,
            OAuthProvider,
            CreateProjectRequest,
            UpdateProjectRequest,
            ProjectResponse,
            CreateEssayRequest,
            UpdateEssayRequest,
            EssayResponse,
            CreateMemoRequest,
            UpdateMemoRequest,
            MemoResponse,
            AssistRequest,
            AssistResponse,
            SimilarMemo,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "서버 상태 확인"),
        (name = "Users", description = "사용자 관리"),
        (name = "Auth", description = "인증 관리 (토큰 갱신, 로그아웃)"),
        (name = "Projects", description = "프로젝트 관리"),
        (name = "Essays", description = "에세이 관리"),
        (name = "Memos", description = "메모 관리"),
        (name = "Assist", description = "AI 어시스턴트"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // HttpOnly 쿠키 기반 인증 사용
        // Swagger UI에서는 쿠키를 자동으로 전송하므로 별도의 security scheme 불필요
        // 브라우저가 자동으로 access_token 쿠키를 포함하여 요청
        let _ = openapi;
    }
}

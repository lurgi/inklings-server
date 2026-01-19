use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use inklings_server::clients::{Embedder, TextGenerator};
use inklings_server::models::assist_dto::{AssistRequest, AssistResponse};
use inklings_server::models::memo_dto::CreateMemoRequest;
use inklings_server::test_utils::{MockGeminiClient, MockQdrantRepository};
use inklings_server::{db, entities, handlers, services};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};
use std::sync::Arc;
use tower::util::ServiceExt;

async fn setup() -> (
    Router,
    Arc<DatabaseConnection>,
    Arc<MockQdrantRepository>,
    Arc<MockGeminiClient>,
) {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set. Run: just setup-test-db");

    let db = Arc::new(db::create_connection(&database_url).await.unwrap());

    // Mock 클라이언트 생성
    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let gemini_client = Arc::new(MockGeminiClient::new());

    // 전체 라우터 생성
    let app = handlers::create_router(
        db.clone(),
        qdrant_repo.clone(),
        gemini_client.clone() as Arc<dyn Embedder>,
        gemini_client.clone() as Arc<dyn TextGenerator>,
    );
    (app, db, qdrant_repo, gemini_client)
}

fn generate_test_token(user_id: i32) -> String {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_min_32_chars_long".to_string());
    inklings_server::utils::jwt::generate_token(user_id, &jwt_secret, 24).unwrap()
}

async fn create_test_user(
    db: &DatabaseConnection,
    id: i32,
    username: &str,
) -> entities::user::Model {
    use chrono::Utc;
    let _ = entities::user::Entity::delete_by_id(id).exec(db).await;

    let user = entities::user::ActiveModel {
        id: Set(id),
        username: Set(username.to_owned()),
        email: Set(format!("{}@test.com", username)),
        password_hash: Set(Some("hashed_password".to_owned())),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };
    user.insert(db).await.unwrap()
}

async fn create_test_project(
    db: &DatabaseConnection,
    user_id: i32,
    name: &str,
) -> entities::project::Model {
    use chrono::Utc;

    let project = entities::project::ActiveModel {
        id: NotSet,
        user_id: Set(user_id),
        name: Set(name.to_owned()),
        description: Set(Some("Test project".to_owned())),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };
    project.insert(db).await.unwrap()
}

// ============================================================================
// A. 기본 테스트 (Happy Path)
// ============================================================================

#[tokio::test]
async fn test_assist_success() {
    let (app, db, qdrant_repo, embedder) = setup().await;

    // 사용자 생성
    let user = create_test_user(&db, 5001, "user5001").await;

    // 프로젝트 생성
    let project = create_test_project(&db, user.id, "Test Project").await;

    // 메모 생성 (setup()에서 받은 같은 Mock 인스턴스 사용)
    let memo_service =
        services::MemoService::new(db.clone(), qdrant_repo, embedder as Arc<dyn Embedder>);

    memo_service
        .create_memo(
            user.id,
            CreateMemoRequest {
                project_id: project.id,
                content: "Rust async programming".to_string(),
            },
        )
        .await
        .unwrap();

    // Assist 요청
    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Tell me about async programming".to_string(),
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let assist_res: AssistResponse = serde_json::from_slice(&body).unwrap();

    assert!(!assist_res.suggestion.is_empty());
    assert!(!assist_res.similar_memos.is_empty());
}

#[tokio::test]
async fn test_assist_no_memos() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5006, "user5006").await;
    let project = create_test_project(&db, user.id, "Empty Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Tell me something".to_string(),
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let assist_res: AssistResponse = serde_json::from_slice(&body).unwrap();

    assert!(!assist_res.suggestion.is_empty());
    assert_eq!(assist_res.similar_memos.len(), 0); // 메모 없음
}

// ============================================================================
// B. 경곗값 테스트 (Boundary Value) - 프롬프트 길이
// ============================================================================

#[tokio::test]
async fn test_assist_prompt_min_length() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5007, "user5007").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "a".to_string(), // 1자
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_assist_prompt_max_length() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5008, "user5008").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "a".repeat(10000), // 10000자
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_assist_prompt_empty() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5009, "user5009").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "".to_string(), // 0자
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_assist_prompt_too_long() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5010, "user5010").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "a".repeat(10001), // 10001자
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// B. 경곗값 테스트 (Boundary Value) - limit 범위
// ============================================================================

#[tokio::test]
async fn test_assist_limit_min() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5011, "user5011").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 1, // 최소값
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_assist_limit_max() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5012, "user5012").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 20, // 최대값
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_assist_limit_zero() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5013, "user5013").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 0, // 최소값 미만
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_assist_limit_too_large() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5014, "user5014").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 21, // 최대값 초과
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// C. 권한 테스트 (Authorization)
// ============================================================================

#[tokio::test]
async fn test_assist_unauthorized() {
    let (app, db, _, _) = setup().await;

    let user1 = create_test_user(&db, 5002, "user5002").await;
    let user2 = create_test_user(&db, 5003, "user5003").await;

    let project = create_test_project(&db, user1.id, "User1 Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 5,
    };

    let token = generate_test_token(user2.id); // user2의 토큰

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_assist_no_auth() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5015, "user5015").await;
    let project = create_test_project(&db, user.id, "Test Project").await;

    let req_body = AssistRequest {
        project_id: project.id,
        prompt: "Test prompt".to_string(),
        limit: 5,
    };

    // 토큰 없이 요청
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ============================================================================
// D. 에러 케이스 (Error Handling)
// ============================================================================

#[tokio::test]
async fn test_assist_project_not_found() {
    let (app, db, _, _) = setup().await;
    let user = create_test_user(&db, 5004, "user5004").await;

    let req_body = AssistRequest {
        project_id: 999999, // 존재하지 않는 ID
        prompt: "Test prompt".to_string(),
        limit: 5,
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assist")
                .header("content-type", "application/json")
                .header("cookie", format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

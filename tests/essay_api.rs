use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use chrono::Utc;
use http_body_util::BodyExt;
use inklings_server::{
    db,
    entities::user,
    handlers,
    models::essay_dto::{CreateEssayRequest, EssayResponse, UpdateEssayRequest},
    models::project_dto::CreateProjectRequest,
    services::{EssayService, ProjectService},
    test_utils::{MockGeminiClient, MockQdrantRepository},
};
use rand::Rng;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use std::sync::Arc;
use tower::util::ServiceExt;

async fn setup() -> (Router, Arc<DatabaseConnection>) {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set. Run: just setup-test-db");

    let db = Arc::new(db::create_connection(&database_url).await.unwrap());

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let gemini_client = Arc::new(MockGeminiClient::new());

    let app = handlers::create_router(
        db.clone(),
        qdrant_repo,
        gemini_client.clone() as Arc<dyn inklings_server::clients::Embedder>,
        gemini_client as Arc<dyn inklings_server::clients::TextGenerator>,
    );
    (app, db)
}

async fn create_test_user(db: &DatabaseConnection) -> user::Model {
    let now = Utc::now().naive_utc();
    let timestamp = now.and_utc().timestamp_micros();
    let random: u32 = rand::thread_rng().gen();
    let unique_id = format!("{}_{}", timestamp, random);

    let user = user::ActiveModel {
        id: NotSet,
        username: Set(format!("test_user_{}", unique_id)),
        email: Set(format!("test_{}@example.com", unique_id)),
        password_hash: Set(Some("hashed_password".to_owned())),
        created_at: Set(now),
        updated_at: Set(now),
    };
    user.insert(db).await.unwrap()
}

fn generate_test_token(user_id: i32) -> String {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "test_secret_key_min_32_chars_long".to_string());
    inklings_server::utils::jwt::generate_token(user_id, &jwt_secret, 24).unwrap()
}

#[tokio::test]
async fn test_create_essay_api() {
    let (app, db) = setup().await;
    let user = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user.id,
            CreateProjectRequest {
                name: "Test Project".to_string(),
                description: Some("Test project for essay".to_string()),
            },
        )
        .await
        .unwrap();

    let req_body = CreateEssayRequest {
        project_id: project.id,
        title: "Test Essay Title".to_string(),
        content: "Test essay content from integration test".to_string(),
    };

    let token = generate_test_token(user.id);

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/essays")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let essay_res: EssayResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(essay_res.title, req_body.title);
    assert_eq!(essay_res.content, req_body.content);
    assert_eq!(essay_res.project_id, project.id);
    assert!(!essay_res.is_pinned);
}

#[tokio::test]
async fn test_list_essays_api() {
    let (app, db) = setup().await;
    let user1 = create_test_user(&db).await;
    let user2 = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user1.id,
            CreateProjectRequest {
                name: "Shared Project".to_string(),
                description: Some("Project shared by multiple users".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let essay_service = Arc::new(EssayService::new(db.clone()));

    // user1이 에세이 생성
    let essay1 = essay_service
        .create_essay(
            user1.id,
            CreateEssayRequest {
                project_id: project.id,
                title: "User1 Essay 1".to_string(),
                content: "Content for user1".to_string(),
            },
        )
        .await
        .unwrap();

    let essay2 = essay_service
        .create_essay(
            user1.id,
            CreateEssayRequest {
                project_id: project.id,
                title: "User1 Essay 2".to_string(),
                content: "More content for user1".to_string(),
            },
        )
        .await
        .unwrap();

    let token = generate_test_token(user1.id);

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/api/essays")
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let essays: Vec<EssayResponse> = serde_json::from_slice(&body).unwrap();

    assert_eq!(essays.len(), 2);
    assert!(essays.iter().all(|e| e.project_id == project.id));
}

#[tokio::test]
async fn test_get_essay_unauthorized_api() {
    let (app, db) = setup().await;
    let user1 = create_test_user(&db).await;
    let user2 = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user1.id,
            CreateProjectRequest {
                name: "Unauthorized Test Project".to_string(),
                description: Some("Test project".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let essay_service = Arc::new(EssayService::new(db.clone()));

    let essay = essay_service
        .create_essay(
            user1.id,
            CreateEssayRequest {
                project_id: project.id,
                title: "Secret Essay".to_string(),
                content: "Only user1 can see this".to_string(),
            },
        )
        .await
        .unwrap();

    // user2가 user1의 에세이 접근 시도
    let token = generate_test_token(user2.id);

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/api/essays/{}", essay.id))
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_essay_api() {
    let (app, db) = setup().await;
    let user = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user.id,
            CreateProjectRequest {
                name: "Test Project".to_string(),
                description: Some("Test project for essay".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let essay_service = Arc::new(EssayService::new(db.clone()));

    let created = essay_service
        .create_essay(
            user.id,
            CreateEssayRequest {
                project_id: project.id,
                title: "Original Title".to_string(),
                content: "Original content".to_string(),
            },
        )
        .await
        .unwrap();

    let token = generate_test_token(user.id);

    let update_req = UpdateEssayRequest {
        title: "Updated Title".to_string(),
        content: "Updated content".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PUT)
                .uri(format!("/api/essays/{}", created.id))
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::from(serde_json::to_string(&update_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let essay_res: EssayResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(essay_res.title, "Updated Title");
    assert_eq!(essay_res.content, "Updated content");
    assert!(essay_res.updated_at > created.updated_at);
}

#[tokio::test]
async fn test_delete_essay_api() {
    let (app, db) = setup().await;
    let user = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user.id,
            CreateProjectRequest {
                name: "Test Project".to_string(),
                description: Some("Test project for essay".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let essay_service = Arc::new(EssayService::new(db.clone()));

    let created = essay_service
        .create_essay(
            user.id,
            CreateEssayRequest {
                project_id: project.id,
                title: "To be deleted".to_string(),
                content: "Delete me".to_string(),
            },
        )
        .await
        .unwrap();

    let token = generate_test_token(user.id);

    // 삭제
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/api/essays/{}", created.id))
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    // 삭제 후 조회 시도
    let get_response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/api/essays/{}", created.id))
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_essay_project_isolation_api() {
    let (app, db) = setup().await;
    let user = create_test_user(&db).await;

    let project_service = ProjectService::new(db.clone());

    let project1 = project_service
        .create_project(
            user.id,
            CreateProjectRequest {
                name: "Project 1".to_string(),
                description: Some("First project".to_string()),
            },
        )
        .await
        .unwrap();

    let project2 = project_service
        .create_project(
            user.id,
            CreateProjectRequest {
                name: "Project 2".to_string(),
                description: Some("Second project".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let essay_service = Arc::new(EssayService::new(db.clone()));

    // project1에 에세이 생성
    let _ = essay_service
        .create_essay(
            user.id,
            CreateEssayRequest {
                project_id: project1.id,
                title: "Project 1 - Essay 1".to_string(),
                content: "Content for project 1".to_string(),
            },
        )
        .await
        .unwrap();

    let _ = essay_service
        .create_essay(
            user.id,
            CreateEssayRequest {
                project_id: project1.id,
                title: "Project 1 - Essay 2".to_string(),
                content: "More content for project 1".to_string(),
            },
        )
        .await
        .unwrap();

    // project2에 에세이 생성
    let _ = essay_service
        .create_essay(
            user.id,
            CreateEssayRequest {
                project_id: project2.id,
                title: "Project 2 - Essay 1".to_string(),
                content: "Content for project 2".to_string(),
            },
        )
        .await
        .unwrap();

    let token = generate_test_token(user.id);

    // project1의 에세이만 조회
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("/api/essays?project_id={}", project1.id))
                .header(http::header::COOKIE, format!("access_token={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let essays: Vec<EssayResponse> = serde_json::from_slice(&body).unwrap();

    assert_eq!(essays.len(), 2);
    assert!(essays.iter().all(|e| e.title.contains("Project 1")));
}

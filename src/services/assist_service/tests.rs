use super::*;
use crate::{
    db,
    entities::user,
    models::{memo_dto::CreateMemoRequest, project_dto::CreateProjectRequest},
    services::{memo_service::MemoService, ProjectService},
    test_utils::{MockGeminiClient, MockQdrantRepository},
};
use chrono::Utc;
use rand::Rng;
use sea_orm::*;

async fn setup_test_db() -> (Arc<DatabaseConnection>, i32) {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set. Run: just setup-test-db");
    let db = Arc::new(db::create_connection(&database_url).await.unwrap());

    let now = Utc::now().naive_utc();
    let timestamp = now.and_utc().timestamp_micros();
    let random: u32 = rand::thread_rng().gen();
    let unique_id = format!("{}_{}", timestamp, random);

    let new_user = user::ActiveModel {
        id: NotSet,
        username: Set(format!("test_user_{}", unique_id)),
        email: Set(format!("test_{}@example.com", unique_id)),
        password_hash: Set(Some("test_hash".to_string())),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let user_id = new_user.insert(db.as_ref()).await.unwrap().id;

    (db, user_id)
}

async fn setup_test_db_with_project() -> (Arc<DatabaseConnection>, i32, i32) {
    let (db, user_id) = setup_test_db().await;

    let project_service = ProjectService::new(db.clone());
    let project = project_service
        .create_project(
            user_id,
            CreateProjectRequest {
                name: format!("Test Project {}", Utc::now().timestamp_micros()),
                description: Some("Test project for assist".to_string()),
            },
        )
        .await
        .unwrap();

    (db, user_id, project.id)
}

#[tokio::test]
async fn test_get_assistance() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let text_generator = Arc::new(MockGeminiClient::new());

    let memo_service = MemoService::new(
        db.clone(),
        qdrant_repo.clone(),
        embedder.clone() as Arc<dyn Embedder>,
    );

    memo_service
        .create_memo(
            user_id,
            project_id,
            CreateMemoRequest {
                content: "Rust is a systems programming language".to_string(),
            },
        )
        .await
        .unwrap();

    memo_service
        .create_memo(
            user_id,
            project_id,
            CreateMemoRequest {
                content: "Async programming in Rust".to_string(),
            },
        )
        .await
        .unwrap();

    let assist_service = AssistService::new(
        db.clone(),
        qdrant_repo as Arc<dyn QdrantRepo>,
        embedder as Arc<dyn Embedder>,
        text_generator as Arc<dyn TextGenerator>,
    );

    let req = AssistRequest {
        prompt: "Tell me about Rust programming".to_string(),
        limit: 5,
    };

    let result = assist_service
        .get_assistance(user_id, project_id, req)
        .await
        .unwrap();

    assert!(!result.suggestion.is_empty());
    assert!(result.suggestion.contains("Tell me about Rust programming"));
}

#[tokio::test]
async fn test_get_assistance_no_similar_memos() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let text_generator = Arc::new(MockGeminiClient::new());

    let assist_service = AssistService::new(
        db,
        qdrant_repo as Arc<dyn QdrantRepo>,
        embedder as Arc<dyn Embedder>,
        text_generator as Arc<dyn TextGenerator>,
    );

    let req = AssistRequest {
        prompt: "Tell me about Python".to_string(),
        limit: 5,
    };

    let result = assist_service
        .get_assistance(user_id, project_id, req)
        .await
        .unwrap();

    assert!(!result.suggestion.is_empty());
    assert_eq!(result.similar_memos.len(), 0);
}

#[tokio::test]
async fn test_get_assistance_user_isolation() {
    let (db, user1_id, project1_id) = setup_test_db_with_project().await;

    let project_service = ProjectService::new(db.clone());
    let now = Utc::now().naive_utc();
    let timestamp = now.and_utc().timestamp_micros();
    let random: u32 = rand::thread_rng().gen();
    let unique_id = format!("{}_{}", timestamp, random);
    let new_user = user::ActiveModel {
        id: NotSet,
        username: Set(format!("test_user_{}", unique_id)),
        email: Set(format!("test_{}@example.com", unique_id)),
        password_hash: Set(Some("test_hash".to_string())),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let user2_id = new_user.insert(db.as_ref()).await.unwrap().id;

    let project2 = project_service
        .create_project(
            user2_id,
            CreateProjectRequest {
                name: format!("User 2 Project {}", Utc::now().timestamp_micros()),
                description: Some("User 2 project".to_string()),
            },
        )
        .await
        .unwrap();

    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let text_generator = Arc::new(MockGeminiClient::new());

    let memo_service = MemoService::new(
        db.clone(),
        qdrant_repo.clone(),
        embedder.clone() as Arc<dyn Embedder>,
    );

    memo_service
        .create_memo(
            user1_id,
            project1_id,
            CreateMemoRequest {
                content: "User 1 memo about Rust".to_string(),
            },
        )
        .await
        .unwrap();

    memo_service
        .create_memo(
            user2_id,
            project2.id,
            CreateMemoRequest {
                content: "User 2 memo about Rust".to_string(),
            },
        )
        .await
        .unwrap();

    let assist_service = AssistService::new(
        db,
        qdrant_repo as Arc<dyn QdrantRepo>,
        embedder as Arc<dyn Embedder>,
        text_generator as Arc<dyn TextGenerator>,
    );

    let req = AssistRequest {
        prompt: "Tell me about Rust".to_string(),
        limit: 5,
    };

    let result = assist_service
        .get_assistance(user1_id, project1_id, req)
        .await
        .unwrap();

    assert!(result
        .similar_memos
        .iter()
        .all(|memo| memo.content.contains("User 1")));
    assert!(!result
        .similar_memos
        .iter()
        .any(|memo| memo.content.contains("User 2")));
}

#[tokio::test]
async fn test_get_assistance_project_isolation() {
    let (db, user_id, project1_id) = setup_test_db_with_project().await;
    let qdrant_repo = Arc::new(MockQdrantRepository::new());
    let embedder = Arc::new(MockGeminiClient::new());
    let text_generator = Arc::new(MockGeminiClient::new());

    let project_service = ProjectService::new(db.clone());
    let project2 = project_service
        .create_project(
            user_id,
            CreateProjectRequest {
                name: format!("Python Project {}", Utc::now().timestamp_micros()),
                description: Some("Python project".to_string()),
            },
        )
        .await
        .unwrap();

    let memo_service = MemoService::new(
        db.clone(),
        qdrant_repo.clone(),
        embedder.clone() as Arc<dyn Embedder>,
    );

    memo_service
        .create_memo(
            user_id,
            project1_id,
            CreateMemoRequest {
                content: "Rust is a systems programming language".to_string(),
            },
        )
        .await
        .unwrap();

    memo_service
        .create_memo(
            user_id,
            project2.id,
            CreateMemoRequest {
                content: "Python is a high-level programming language".to_string(),
            },
        )
        .await
        .unwrap();

    let assist_service = AssistService::new(
        db.clone(),
        qdrant_repo as Arc<dyn QdrantRepo>,
        embedder as Arc<dyn Embedder>,
        text_generator as Arc<dyn TextGenerator>,
    );

    let req1 = AssistRequest {
        prompt: "Tell me about Rust".to_string(),
        limit: 5,
    };

    let result1 = assist_service
        .get_assistance(user_id, project1_id, req1)
        .await
        .unwrap();
    assert!(result1
        .similar_memos
        .iter()
        .all(|m| m.content.contains("Rust")));
    assert!(!result1
        .similar_memos
        .iter()
        .any(|m| m.content.contains("Python")));

    let req2 = AssistRequest {
        prompt: "Tell me about Python".to_string(),
        limit: 5,
    };

    let result2 = assist_service
        .get_assistance(user_id, project2.id, req2)
        .await
        .unwrap();
    assert!(result2
        .similar_memos
        .iter()
        .all(|m| m.content.contains("Python")));
    assert!(!result2
        .similar_memos
        .iter()
        .any(|m| m.content.contains("Rust")));
}

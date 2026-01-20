use super::*;
use crate::{
    db, entities::user, models::essay_dto::CreateEssayRequest,
    models::project_dto::CreateProjectRequest, services::ProjectService,
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
                description: Some("Test project for essays".to_string()),
            },
        )
        .await
        .unwrap();

    (db, user_id, project.id)
}

#[tokio::test]
async fn test_create_and_get_essay() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let req = CreateEssayRequest {
        project_id,
        title: "Test Essay Title".to_string(),
        content: "Test essay content".to_string(),
    };

    let created = service.create_essay(user_id, req).await.unwrap();
    assert_eq!(created.title, "Test Essay Title");
    assert_eq!(created.content, "Test essay content");
    assert_eq!(created.project_id, project_id);
    assert!(!created.is_pinned);

    let fetched = service.get_essay(user_id, created.id).await.unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, created.title);
    assert_eq!(fetched.content, created.content);
}

#[tokio::test]
async fn test_get_essay_unauthorized() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let req = CreateEssayRequest {
        project_id,
        title: "User 1's Essay".to_string(),
        content: "User 1's content".to_string(),
    };

    let created = service.create_essay(user_id, req).await.unwrap();

    let result = service.get_essay(user_id + 999, created.id).await;
    assert!(matches!(result, Err(ServiceError::Unauthorized)));
}

#[tokio::test]
async fn test_update_essay() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let create_req = CreateEssayRequest {
        project_id,
        title: "Original Title".to_string(),
        content: "Original content".to_string(),
    };
    let created = service.create_essay(user_id, create_req).await.unwrap();

    let update_req = UpdateEssayRequest {
        title: "Updated Title".to_string(),
        content: "Updated content".to_string(),
    };
    let updated = service
        .update_essay(user_id, created.id, update_req)
        .await
        .unwrap();

    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.content, "Updated content");
    assert!(updated.updated_at > created.updated_at);
}

#[tokio::test]
async fn test_delete_essay() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let req = CreateEssayRequest {
        project_id,
        title: "To be deleted".to_string(),
        content: "Delete me".to_string(),
    };
    let created = service.create_essay(user_id, req).await.unwrap();

    service.delete_essay(user_id, created.id).await.unwrap();

    let result = service.get_essay(user_id, created.id).await;
    assert!(matches!(result, Err(ServiceError::EssayNotFound)));
}

#[tokio::test]
async fn test_list_essays_ordering() {
    let (db, user_id, project_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let essay1 = service
        .create_essay(
            user_id,
            CreateEssayRequest {
                project_id,
                title: "First".to_string(),
                content: "First content".to_string(),
            },
        )
        .await
        .unwrap();

    let essay2 = service
        .create_essay(
            user_id,
            CreateEssayRequest {
                project_id,
                title: "Second".to_string(),
                content: "Second content".to_string(),
            },
        )
        .await
        .unwrap();

    // essay1 고정 (직접 DB 업데이트)
    let mut essay1_active: crate::entities::essay::ActiveModel =
        crate::entities::essay::Entity::find_by_id(essay1.id)
            .one(&*db)
            .await
            .unwrap()
            .unwrap()
            .into();
    essay1_active.is_pinned = Set(true);
    essay1_active.updated_at = Set(Utc::now().naive_utc());
    essay1_active.update(&*db).await.unwrap();

    let essays = service
        .list_essays_by_project(user_id, project_id)
        .await
        .unwrap();

    assert!(essays[0].is_pinned);
    assert_eq!(essays[0].id, essay1.id);
    assert!(!essays[1].is_pinned);
    assert_eq!(essays[1].id, essay2.id);
}

#[tokio::test]
async fn test_essay_project_isolation() {
    let (db, user_id, project1_id) = setup_test_db_with_project().await;
    let service = EssayService::new(db.clone());

    let project_service = ProjectService::new(db.clone());
    let project2 = project_service
        .create_project(
            user_id,
            CreateProjectRequest {
                name: format!("Project 2 {}", Utc::now().timestamp_micros()),
                description: Some("Second project".to_string()),
            },
        )
        .await
        .unwrap();

    service
        .create_essay(
            user_id,
            CreateEssayRequest {
                project_id: project1_id,
                title: "Project 1 - Essay 1".to_string(),
                content: "Content for project 1".to_string(),
            },
        )
        .await
        .unwrap();

    service
        .create_essay(
            user_id,
            CreateEssayRequest {
                project_id: project1_id,
                title: "Project 1 - Essay 2".to_string(),
                content: "Another content for project 1".to_string(),
            },
        )
        .await
        .unwrap();

    service
        .create_essay(
            user_id,
            CreateEssayRequest {
                project_id: project2.id,
                title: "Project 2 - Essay 1".to_string(),
                content: "Content for project 2".to_string(),
            },
        )
        .await
        .unwrap();

    let project1_essays = service
        .list_essays_by_project(user_id, project1_id)
        .await
        .unwrap();
    assert_eq!(project1_essays.len(), 2);
    assert!(project1_essays
        .iter()
        .all(|e| e.title.contains("Project 1")));

    let project2_essays = service
        .list_essays_by_project(user_id, project2.id)
        .await
        .unwrap();
    assert_eq!(project2_essays.len(), 1);
    assert!(project2_essays
        .iter()
        .all(|e| e.title.contains("Project 2")));

    let result = service
        .list_essays_by_project(user_id + 999, project1_id)
        .await;
    assert!(matches!(result, Err(ServiceError::Unauthorized)));
}

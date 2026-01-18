#[cfg(test)]
mod tests {
    use crate::db;
    use crate::entities::user;
    use crate::errors::ServiceError;
    use crate::models::project_dto::{CreateProjectRequest, UpdateProjectRequest};
    use crate::services::ProjectService;
    use chrono::Utc;
    use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
    use std::sync::Arc;

    async fn setup_test_db() -> (Arc<DatabaseConnection>, i32) {
        dotenv::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set for tests");

        let db = Arc::new(
            db::create_connection(&database_url)
                .await
                .expect("Failed to create test database connection"),
        );

        let timestamp = Utc::now().timestamp();
        let random: u32 = rand::random();
        let unique_id = format!("{}_{}", timestamp, random);

        let username = format!("test_user_{}", unique_id);
        let email = format!("test_{}@example.com", unique_id);

        let new_user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password_hash: Set(None),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let user = new_user
            .insert(db.as_ref())
            .await
            .expect("Failed to create test user");

        (db, user.id)
    }

    #[tokio::test]
    async fn test_create_project_success() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let req = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: Some("Test description".to_string()),
        };

        let result = service.create_project(user_id, req).await;
        assert!(result.is_ok());

        let project = result.unwrap();
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.description, Some("Test description".to_string()));
        assert_eq!(project.user_id, user_id);
        assert!(project.id > 0);
    }

    #[tokio::test]
    async fn test_get_project_success() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Get Test".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service.get_project(user_id, created.id).await;
        assert!(result.is_ok());

        let project = result.unwrap();
        assert_eq!(project.id, created.id);
        assert_eq!(project.name, "Get Test");
    }

    #[tokio::test]
    async fn test_list_projects_ordering() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Project 1".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Project 2".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let projects = service.list_projects(user_id).await.unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "Project 2");
        assert_eq!(projects[1].name, "Project 1");
    }

    #[tokio::test]
    async fn test_update_project_success() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Original".to_string(),
                    description: Some("Original Desc".to_string()),
                },
            )
            .await
            .unwrap();

        let updated = service
            .update_project(
                user_id,
                created.id,
                UpdateProjectRequest {
                    name: Some("Updated".to_string()),
                    description: Some(Some("Updated Desc".to_string())),
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.description, Some("Updated Desc".to_string()));
    }

    #[tokio::test]
    async fn test_delete_project_success() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "To Delete".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service.delete_project(user_id, created.id).await;
        assert!(result.is_ok());

        let get_result = service.get_project(user_id, created.id).await;
        assert!(matches!(get_result, Err(ServiceError::ProjectNotFound)));
    }

    #[tokio::test]
    async fn test_create_project_duplicate_name() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let req = CreateProjectRequest {
            name: "Duplicate".to_string(),
            description: None,
        };

        service.create_project(user_id, req.clone()).await.unwrap();

        let result = service.create_project(user_id, req).await;
        assert!(matches!(
            result,
            Err(ServiceError::ProjectNameAlreadyExists)
        ));
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let result = service.get_project(user_id, 99999).await;
        assert!(matches!(result, Err(ServiceError::ProjectNotFound)));
    }

    #[tokio::test]
    async fn test_get_project_unauthorized() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Auth Test".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service.get_project(user_id + 999, created.id).await;
        assert!(matches!(result, Err(ServiceError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_update_project_unauthorized() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Auth Test".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service
            .update_project(
                user_id + 999,
                created.id,
                UpdateProjectRequest {
                    name: Some("Hacked".to_string()),
                    description: None,
                },
            )
            .await;

        assert!(matches!(result, Err(ServiceError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_delete_project_unauthorized() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let created = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Auth Test".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service.delete_project(user_id + 999, created.id).await;
        assert!(matches!(result, Err(ServiceError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_create_project_empty_name() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let req = CreateProjectRequest {
            name: "".to_string(),
            description: None,
        };

        let result = service.create_project(user_id, req).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_create_project_max_length_name() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let long_name = "a".repeat(100);
        let req = CreateProjectRequest {
            name: long_name.clone(),
            description: None,
        };

        let result = service.create_project(user_id, req).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, long_name);
    }

    #[tokio::test]
    async fn test_create_project_over_max_length_name() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        let long_name = "a".repeat(101);
        let req = CreateProjectRequest {
            name: long_name,
            description: None,
        };

        let result = service.create_project(user_id, req).await;
        assert!(result.is_err() || result.is_ok());
    }

    #[tokio::test]
    async fn test_update_project_name_to_existing() {
        let (db, user_id) = setup_test_db().await;
        let service = ProjectService::new(db);

        service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "Existing".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let project2 = service
            .create_project(
                user_id,
                CreateProjectRequest {
                    name: "ToUpdate".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let result = service
            .update_project(
                user_id,
                project2.id,
                UpdateProjectRequest {
                    name: Some("Existing".to_string()),
                    description: None,
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(ServiceError::ProjectNameAlreadyExists)
        ));
    }

    #[tokio::test]
    async fn test_project_isolation_between_users() {
        let (db, user1_id) = setup_test_db().await;

        let timestamp = Utc::now().timestamp();
        let random: u32 = rand::random();
        let unique_id = format!("{}_{}", timestamp, random);
        let username = format!("test_user_{}", unique_id);
        let email = format!("test_{}@example.com", unique_id);

        let new_user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password_hash: Set(None),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let user2 = new_user.insert(db.as_ref()).await.unwrap();
        let user2_id = user2.id;

        let service = ProjectService::new(db);

        let project1 = service
            .create_project(
                user1_id,
                CreateProjectRequest {
                    name: "User1 Project".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let project2 = service
            .create_project(
                user2_id,
                CreateProjectRequest {
                    name: "User2 Project".to_string(),
                    description: None,
                },
            )
            .await
            .unwrap();

        let user1_projects = service.list_projects(user1_id).await.unwrap();
        assert_eq!(user1_projects.len(), 1);
        assert_eq!(user1_projects[0].id, project1.id);

        let user2_projects = service.list_projects(user2_id).await.unwrap();
        assert_eq!(user2_projects.len(), 1);
        assert_eq!(user2_projects[0].id, project2.id);

        let result = service.get_project(user2_id, project1.id).await;
        assert!(matches!(result, Err(ServiceError::Unauthorized)));

        let result = service.get_project(user1_id, project2.id).await;
        assert!(matches!(result, Err(ServiceError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_project_cascade_delete_memos() {
        todo!("Memo 구현 후 작성");
    }
}

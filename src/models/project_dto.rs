use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::project;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Validate)]
pub struct CreateProjectRequest {
    #[schema(example = "개인 블로그 프로젝트")]
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,

    #[schema(example = "개인 블로그 작성을 위한 메모와 글 모음")]
    #[validate(length(max = 1000, message = "Description must not exceed 1000 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Validate)]
pub struct UpdateProjectRequest {
    #[schema(example = "수정된 프로젝트 이름")]
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: Option<String>,

    #[schema(example = "수정된 설명")]
    #[validate(length(max = 1000, message = "Description must not exceed 1000 characters"))]
    pub description: Option<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct ProjectResponse {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = 1)]
    pub user_id: i32,

    #[schema(example = "개인 블로그 프로젝트")]
    pub name: String,

    #[schema(example = "개인 블로그 작성을 위한 메모와 글 모음")]
    pub description: Option<String>,

    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,

    #[schema(example = "2024-01-15T10:30:00")]
    pub updated_at: NaiveDateTime,
}

impl From<project::Model> for ProjectResponse {
    fn from(project: project::Model) -> Self {
        Self {
            id: project.id,
            user_id: project.user_id,
            name: project.name,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project_request_valid() {
        let request = CreateProjectRequest {
            name: "개인 블로그 프로젝트".to_string(),
            description: Some("개인 블로그 작성을 위한 메모와 글 모음".to_string()),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_project_request_empty_name() {
        let request = CreateProjectRequest {
            name: "".to_string(),
            description: Some("설명".to_string()),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_project_request_name_too_long() {
        let request = CreateProjectRequest {
            name: "a".repeat(201),
            description: Some("설명".to_string()),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_project_request_description_too_long() {
        let request = CreateProjectRequest {
            name: "프로젝트".to_string(),
            description: Some("a".repeat(1001)),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_project_request_no_description() {
        let request = CreateProjectRequest {
            name: "프로젝트".to_string(),
            description: None,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_project_request_valid() {
        let request = UpdateProjectRequest {
            name: Some("수정된 프로젝트 이름".to_string()),
            description: Some(Some("수정된 설명".to_string())),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_project_request_empty_name() {
        let request = UpdateProjectRequest {
            name: Some("".to_string()),
            description: Some(Some("설명".to_string())),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_project_request_name_too_long() {
        let request = UpdateProjectRequest {
            name: Some("a".repeat(201)),
            description: Some(Some("설명".to_string())),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_project_request_description_too_long() {
        let request = UpdateProjectRequest {
            name: Some("프로젝트".to_string()),
            description: Some(Some("a".repeat(1001))),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_project_request_no_changes() {
        let request = UpdateProjectRequest {
            name: None,
            description: None,
        };
        assert!(request.validate().is_ok());
    }
}

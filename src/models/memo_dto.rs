use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::memo;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Validate)]
pub struct CreateMemoRequest {
    #[schema(example = 1)]
    #[validate(range(min = 1, message = "Project ID must be at least 1"))]
    pub project_id: i32,

    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다")]
    #[validate(length(min = 1, max = 1000, message = "Content must be 1-1000 characters"))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Validate)]
pub struct UpdateMemoRequest {
    #[schema(example = "Rust 비동기 프로그래밍 정리 완료. tokio와 async/await 개념 이해함")]
    #[validate(length(min = 1, max = 1000, message = "Content must be 1-1000 characters"))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct MemoResponse {
    #[schema(example = 42)]
    pub id: i32,
    #[schema(example = 1)]
    pub project_id: i32,
    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다")]
    pub content: String,
    #[schema(example = false)]
    pub is_pinned: bool,
    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-01-15T10:30:00")]
    pub updated_at: NaiveDateTime,
}

impl From<memo::Model> for MemoResponse {
    fn from(memo: memo::Model) -> Self {
        Self {
            id: memo.id,
            project_id: memo.project_id,
            content: memo.content,
            is_pinned: memo.is_pinned,
            created_at: memo.created_at,
            updated_at: memo.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_memo_request_valid() {
        let request = CreateMemoRequest {
            project_id: 1,
            content: "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_memo_request_project_id_zero() {
        let request = CreateMemoRequest {
            project_id: 0,
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_memo_request_project_id_negative() {
        let request = CreateMemoRequest {
            project_id: -1,
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_memo_request_empty_content() {
        let request = CreateMemoRequest {
            project_id: 1,
            content: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_memo_request_content_exactly_max() {
        let request = CreateMemoRequest {
            project_id: 1,
            content: "a".repeat(1000),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_memo_request_content_too_long() {
        let request = CreateMemoRequest {
            project_id: 1,
            content: "a".repeat(1001),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_memo_request_valid() {
        let request = UpdateMemoRequest {
            content: "수정된 메모 내용".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_memo_request_empty_content() {
        let request = UpdateMemoRequest {
            content: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_memo_request_content_exactly_max() {
        let request = UpdateMemoRequest {
            content: "a".repeat(1000),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_memo_request_content_too_long() {
        let request = UpdateMemoRequest {
            content: "a".repeat(1001),
        };
        assert!(request.validate().is_err());
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::essay;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct CreateEssayRequest {
    #[schema(example = 1)]
    pub project_id: i32,

    #[schema(example = "Rust 비동기 프로그래밍에 대한 고찰")]
    pub title: String,

    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍에 대해 자세히 정리해보자")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct UpdateEssayRequest {
    #[schema(example = "Rust 비동기 프로그래밍 심층 분석")]
    pub title: String,

    #[schema(example = "Rust 비동기 프로그래밍에 대해 더 자세히 알아보자")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct EssayResponse {
    #[schema(example = 42)]
    pub id: i32,
    #[schema(example = 1)]
    pub project_id: i32,
    #[schema(example = "Rust 비동기 프로그래밍에 대한 고찰")]
    pub title: String,
    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍에 대해 자세히 정리해보자")]
    pub content: String,
    #[schema(example = false)]
    pub is_pinned: bool,
    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-01-15T10:30:00")]
    pub updated_at: NaiveDateTime,
}

impl From<essay::Model> for EssayResponse {
    fn from(essay: essay::Model) -> Self {
        Self {
            id: essay.id,
            project_id: essay.project_id,
            title: essay.title,
            content: essay.content,
            is_pinned: essay.is_pinned,
            created_at: essay.created_at,
            updated_at: essay.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_essay_request_valid() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "Rust 비동기 프로그래밍에 대한 고찰".to_string(),
            content: "오늘 배운 Rust 비동기 프로그래밍에 대해 자세히 정리해보자".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_essay_request_project_id_zero() {
        let request = CreateEssayRequest {
            project_id: 0,
            title: "제목".to_string(),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_essay_request_project_id_negative() {
        let request = CreateEssayRequest {
            project_id: -1,
            title: "제목".to_string(),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_essay_request_empty_title() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "".to_string(),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_essay_request_title_exactly_max() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "a".repeat(200),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_essay_request_title_too_long() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "a".repeat(201),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_essay_request_empty_content() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "제목".to_string(),
            content: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_essay_request_content_exactly_max() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "제목".to_string(),
            content: "a".repeat(10000),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_essay_request_content_too_long() {
        let request = CreateEssayRequest {
            project_id: 1,
            title: "제목".to_string(),
            content: "a".repeat(10001),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_essay_request_valid() {
        let request = UpdateEssayRequest {
            title: "Rust 비동기 프로그래밍 심층 분석".to_string(),
            content: "Rust 비동기 프로그래밍에 대해 더 자세히 알아보자".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_essay_request_empty_title() {
        let request = UpdateEssayRequest {
            title: "".to_string(),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_essay_request_title_exactly_max() {
        let request = UpdateEssayRequest {
            title: "a".repeat(200),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_essay_request_title_too_long() {
        let request = UpdateEssayRequest {
            title: "a".repeat(201),
            content: "내용".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_essay_request_empty_content() {
        let request = UpdateEssayRequest {
            title: "제목".to_string(),
            content: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_update_essay_request_content_exactly_max() {
        let request = UpdateEssayRequest {
            title: "제목".to_string(),
            content: "a".repeat(10000),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_essay_request_content_too_long() {
        let request = UpdateEssayRequest {
            title: "제목".to_string(),
            content: "a".repeat(10001),
        };
        assert!(request.validate().is_err());
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::memo;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct CreateMemoRequest {
    #[schema(example = 1)]
    pub project_id: i32,

    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct UpdateMemoRequest {
    #[schema(example = "Rust 비동기 프로그래밍 정리 완료. tokio와 async/await 개념 이해함")]
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

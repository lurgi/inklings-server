use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AssistRequest {
    #[schema(example = 1)]
    pub project_id: i32,

    #[validate(length(min = 1, max = 10000, message = "Prompt must be 1-10000 characters"))]
    #[schema(example = "Rust 비동기 프로그래밍에 대해 알려줘")]
    pub prompt: String,

    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 20, message = "Limit must be 1-20"))]
    #[schema(example = 5)]
    pub limit: u64,
}

fn default_limit() -> u64 {
    5
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AssistResponse {
    #[schema(
        example = "Rust 비동기 프로그래밍은 tokio 런타임을 사용하여 async/await 키워드로 구현됩니다..."
    )]
    pub suggestion: String,
    pub similar_memos: Vec<SimilarMemo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SimilarMemo {
    #[schema(example = 42)]
    pub id: i32,
    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다")]
    pub content: String,
    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
}

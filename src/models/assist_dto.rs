use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssistRequest {
    #[schema(example = "Rust 비동기 프로그래밍에 대해 알려줘")]
    pub prompt: String,

    #[serde(default = "default_limit")]
    #[schema(example = 5)]
    pub limit: u64,
}

fn default_limit() -> u64 {
    5
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AssistResponse {
    #[schema(example = "Rust 비동기 프로그래밍은 tokio 런타임을 사용하여 async/await 키워드로 구현됩니다...")]
    pub suggestion: String,
    pub similar_memos: Vec<SimilarMemo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SimilarMemo {
    #[schema(example = 42)]
    pub id: i32,
    #[schema(example = "오늘 배운 Rust 비동기 프로그래밍을 정리해야겠다")]
    pub content: String,
    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
}

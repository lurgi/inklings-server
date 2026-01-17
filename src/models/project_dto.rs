use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct CreateProjectRequest {
    #[schema(example = "개인 블로그 프로젝트")]
    pub name: String,

    #[schema(example = "개인 블로그 작성을 위한 메모와 글 모음")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct UpdateProjectRequest {
    #[schema(example = "수정된 프로젝트 이름")]
    pub name: Option<String>,

    #[schema(example = "수정된 설명")]
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

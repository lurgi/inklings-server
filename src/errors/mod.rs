pub mod service_error;

pub use service_error::ServiceError;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Resource not found")]
    pub error: String,
}

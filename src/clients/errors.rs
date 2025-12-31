use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Gemini API error: {0}")]
    GeminiApi(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("JSON parsing error: {0}")]
    ParseError(String),

    #[error("Qdrant error: {0}")]
    Qdrant(String),
}

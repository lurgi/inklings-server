use crate::errors::ServiceError;

#[async_trait::async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, ServiceError>;
    fn dimension(&self) -> usize;
}

#[async_trait::async_trait]
pub trait TextGenerator: Send + Sync {
    async fn generate(
        &self,
        prompt: &str,
        context: Vec<String>,
    ) -> Result<String, ServiceError>;
}

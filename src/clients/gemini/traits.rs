use crate::clients::ClientError;

#[async_trait::async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, ClientError>;
    fn dimension(&self) -> usize;
}

#[async_trait::async_trait]
pub trait TextGenerator: Send + Sync {
    async fn generate(
        &self,
        prompt: &str,
        context: Vec<String>,
    ) -> Result<String, ClientError>;
}

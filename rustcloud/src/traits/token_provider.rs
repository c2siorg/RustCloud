use async_trait::async_trait;

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn get_token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

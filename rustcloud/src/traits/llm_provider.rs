use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::errors::CloudError;
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse, ToolDefinition,
};

pub type LlmStream = Pin<Box<dyn Stream<Item = LlmStreamEvent> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError>;
    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError>;
    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError>;
    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError>;
}

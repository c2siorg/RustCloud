use async_trait::async_trait;
use aws_sdk_bedrockruntime::Client;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, ToolCallResponse, ToolDefinition,
};

pub struct BedrockProvider {
    client: Client,
}

impl BedrockProvider {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        Self {
            client: Client::new(&config),
        }
    }

    pub fn with_client(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl LlmProvider for BedrockProvider {
    async fn generate(&self, _req: LlmRequest) -> Result<LlmResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "Bedrock generate",
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        Err(CloudError::Unsupported {
            feature: "Bedrock stream",
        })
    }

    async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "Bedrock embed",
        })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "Bedrock generate_with_tools",
        })
    }
}

#![allow(clippy::too_many_arguments, clippy::new_without_default, dead_code)]

use async_trait::async_trait;
use aws_sdk_bedrockruntime::{Client, config::BehaviorVersion};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, ModelRef, ToolCallResponse,
    ToolDefinition, UsageStats,
};

#[derive(Debug, Clone)]
pub struct AmazonBedrock {
    client: Client,
    region: String,
}

impl AmazonBedrock {
    pub fn new(region: String) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_sdk_types::region::Region::new(region.clone()))
            .load_sync();
        let client = Client::from_conf(
            aws_sdk_bedrockruntime::config::Builder::from(&config)
                .behavior_version(BehaviorVersion::latest())
                .build(),
        );
        Self { client, region }
    }

    pub async fn from_env() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let region = config.region().map(|r| r.to_string()).unwrap_or_else(|| "us-east-1".to_string());
        let client = Client::from_conf(
            aws_sdk_bedrockruntime::config::Builder::from(&config)
                .behavior_version(BehaviorVersion::latest())
                .build(),
        );
        Self { client, region }
    }

    fn resolve_model_id(&self, model: &ModelRef) -> String {
        match model {
            ModelRef::Provider(id) => id.clone(),
            ModelRef::Logical { family, tier } => {
                let suffix = tier.as_deref().unwrap_or("001");
                format!("{}:{}", family, suffix)
            }
            ModelRef::Deployment(dep) => dep.clone(),
        }
    }

    fn convert_messages(messages: &[crate::types::llm::Message]) -> Vec<serde_json::Value> {
        messages
            .iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
            .collect()
    }
}

#[async_trait]
impl LlmProvider for AmazonBedrock {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        let messages = Self::convert_messages(&req.messages);

        let body = serde_json::json!({
            "messages": messages,
            "max_tokens": req.max_tokens.unwrap_or(1024),
            "temperature": req.temperature.unwrap_or(0.7),
        });

        let body_str = serde_json::to_string(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let response = self.client
            .converse()
            .model_id(&model_id)
            .body(aws_sdk_bedrockruntime::primitives::Blob::from(body_str.as_bytes()))
            .send()
            .await
            .map_err(|e| CloudError::Provider {
                http_status: 0,
                message: format!("Bedrock API error: {}", e),
                retryable: false,
            })?;

        let output = response.output().as_model();
        let text = output
            .as_ref()
            .and_then(|o| o.content().first())
            .and_then(|c| c.text())
            .unwrap_or_default()
            .to_string();

        let usage = response.usage().map(|u| UsageStats {
            prompt_tokens: u.input_tokens(),
            completion_tokens: u.output_tokens(),
        });

        Ok(LlmResponse {
            text,
            finish_reason: FinishReason::Stop,
            usage,
        })
    }

    async fn stream(
        &self,
        _req: LlmRequest,
    ) -> Result<LlmStream, CloudError> {
        Err(CloudError::Unsupported {
            feature: "streaming not yet implemented for Bedrock",
        })
    }

    async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "embeddings not yet implemented for Bedrock",
        })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "tools not yet implemented for Bedrock",
        })
    }
}
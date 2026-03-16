use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_bedrockruntime::Client;
use serde_json::{json, Value};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef, ToolCallResponse, ToolDefinition, UsageStats};

pub struct AwsBedrockGenAI {
    client: Client,
}

impl AwsBedrockGenAI {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        
        AwsBedrockGenAI {
            client: Client::new(&config),
        }
    }

    fn resolve_model_id(&self, model: &ModelRef) -> String {
        match model {
            ModelRef::Provider(id) => id.clone(),
            ModelRef::Logical { family, tier } => {
                let suffix = tier.as_deref().unwrap_or("v1");
                format!("{}.{}", family, suffix)
            }
            ModelRef::Deployment(dep) => dep.clone(),
        }
    }
}

#[async_trait]
impl LlmProvider for AwsBedrockGenAI {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        
        let prompt = req.messages.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n");

        let max_tokens = req.max_tokens.unwrap_or(256);
        let temperature = req.temperature.unwrap_or(0.7);

        let body = json!({
            "prompt": prompt,
            "maxTokens": max_tokens,
            "temperature": temperature,
        });

        let response = self.client.invoke_model()
            .model_id(model_id)
            .content_type("application/json")
            .accept("application/json")
            .body(aws_sdk_bedrockruntime::primitives::Blob::new(body.to_string().into_bytes()))
            .send()
            .await
            .map_err(|e| CloudError::Provider { 
                http_status: 500, 
                message: format!("Bedrock error: {}", e), 
                retryable: false 
            })?;

        let output_bytes = response.body.into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap_or_default();
        let output_json: Value = serde_json::from_str(&output_str).unwrap_or_default();
        
        // Handle different Bedrock model response formats (simplified)
        let completion_text = if let Some(content_idx) = output_json.get("content").and_then(|c| c.as_array()) {
            if let Some(first_content) = content_idx.first() {
                first_content.get("text").and_then(|t| t.as_str()).unwrap_or("")
            } else {
                ""
            }
        } else if let Some(text) = output_json.get("completion").and_then(|c| c.as_str()) {
            text
        } else if let Some(text) = output_json.get("results").and_then(|r| r.as_array()).and_then(|a| a.first()).and_then(|f| f.get("outputText")).and_then(|t| t.as_str()) {
            text
        } else {
            ""
        };

        Ok(LlmResponse {
            text: completion_text.to_string(),
            finish_reason: FinishReason::Stop,
            usage: Some(UsageStats { prompt_tokens: 0, completion_tokens: 0 }),
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        Err(CloudError::Unsupported { feature: "Bedrock stream" })
    }

    async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        Err(CloudError::Unsupported { feature: "Bedrock embeddings" })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported { feature: "Bedrock tool use" })
    }
}

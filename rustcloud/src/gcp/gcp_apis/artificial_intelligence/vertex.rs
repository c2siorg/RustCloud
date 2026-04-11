use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};

use crate::errors::CloudError;
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef, ToolCallResponse, ToolDefinition, UsageStats};

pub struct GoogleVertexAI {
    client: reqwest::Client,
    project_id: String,
    location: String,
}

impl GoogleVertexAI {
    pub fn new(project_id: String, location: Option<String>) -> Self {
        GoogleVertexAI {
            client: reqwest::Client::new(),
            project_id,
            location: location.unwrap_or_else(|| "us-central1".to_string()),
        }
    }

    fn resolve_model_id(&self, model: &ModelRef) -> String {
        match model {
            ModelRef::Provider(id) => id.clone(),
            ModelRef::Logical { family, tier } => {
                let suffix = tier.as_deref().unwrap_or("001");
                format!("{}-{}", family, suffix)
            }
            ModelRef::Deployment(dep) => dep.clone(),
        }
    }
}

#[async_trait]
impl LlmProvider for GoogleVertexAI {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        let token = retrieve_token().await.map_err(|e| CloudError::Provider {
            http_status: 401,
            message: format!("GCP Auth failed: {}", e),
            retryable: true,
        })?;

        let url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            self.location, self.project_id, self.location, model_id
        );

        let contents = req.messages.iter().map(|m| {
            json!({
                "role": if m.role == "user" { "user" } else { "model" },
                "parts": [{ "text": m.content }]
            })
        }).collect::<Vec<_>>();

        let body = json!({
            "contents": contents,
            "generationConfig": {
                "maxOutputTokens": req.max_tokens.unwrap_or(256),
                "temperature": req.temperature.unwrap_or(0.7),
            }
        });

        let response = self.client.post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider {
                http_status: 500,
                message: format!("Vertex API error: {}", e),
                retryable: false,
            })?;

        let status = response.status();
        let resp_json: Value = response.json().await.map_err(|e| CloudError::Provider {
            http_status: status.as_u16(),
            message: format!("Vertex parse error: {}", e),
            retryable: false,
        })?;

        let completion_text = resp_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("").to_string();

        let finish_reason = resp_json["candidates"][0]["finishReason"]
            .as_str()
            .map(|r| match r {
                "STOP" => FinishReason::Stop,
                "MAX_TOKENS" => FinishReason::Length,
                "SAFETY" | "OTHER" => FinishReason::Other(r.to_string()),
                _ => FinishReason::Other(r.to_string()),
            })
            .unwrap_or(FinishReason::Stop);

        let usage = resp_json["usageMetadata"].as_object().map(|u| UsageStats {
            prompt_tokens: u.get("promptTokenCount").and_then(|v| v.as_u64()).map(|n| n as u32).unwrap_or(0),
            completion_tokens: u.get("candidatesTokenCount").and_then(|v| v.as_u64()).map(|n| n as u32).unwrap_or(0),
        });

        Ok(LlmResponse {
            text: completion_text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        Err(CloudError::Unsupported { feature: "Vertex stream" })
    }

    async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        Err(CloudError::Unsupported { feature: "Vertex embeddings" })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported { feature: "Vertex tool use" })
    }
}

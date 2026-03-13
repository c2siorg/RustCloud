use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use reqwest::{header::AUTHORIZATION, Client, Response};
use std::pin::Pin;

use crate::errors::CloudError;
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::artificial_intelligence::gcp_vertex_ai_types::*;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

/// Vertex AI client for GCP's modern AI platform.
///
/// This replaces the deprecated AutoML API and implements the `LlmProvider` trait
/// for Gemini model access.
pub struct VertexAI {
    client: Client,
    project_id: String,
    location: String,
}

impl VertexAI {
    /// Create a new VertexAI client.
    ///
    /// # Arguments
    /// * `project_id` - Your GCP project ID
    /// * `location` - The GCP region (e.g., "us-central1")
    pub fn new(project_id: &str, location: &str) -> Self {
        Self {
            client: Client::new(),
            project_id: project_id.to_string(),
            location: location.to_string(),
        }
    }

    /// Construct the regional base URL for Vertex AI.
    fn base_url(&self) -> String {
        format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}",
            self.location, self.project_id, self.location
        )
    }

    /// Retrieve a bearer token, mapping errors to CloudError::Auth.
    async fn get_auth_token(&self) -> Result<String, CloudError> {
        retrieve_token().await.map_err(|e| CloudError::Auth {
            message: e.to_string(),
        })
    }

    /// Resolve a ModelRef to a concrete Vertex AI model ID string.
    fn resolve_model(model: &ModelRef) -> Result<String, CloudError> {
        match model {
            ModelRef::Provider(id) => Ok(id.clone()),
            ModelRef::Deployment(id) => Ok(id.clone()),
            ModelRef::Logical { family, tier } => {
                let model_id = match (family.as_str(), tier.as_deref()) {
                    ("gemini", Some("pro")) | ("gemini", None) => "gemini-1.5-pro",
                    ("gemini", Some("flash")) => "gemini-1.5-flash",
                    ("gemini", Some("ultra")) => "gemini-ultra",
                    _ => {
                        return Err(CloudError::Unsupported {
                            feature: "unknown model family/tier combination",
                        })
                    }
                };
                Ok(model_id.to_string())
            }
        }
    }

    // ─── Dataset Management ───

    /// Create a new dataset in Vertex AI.
    pub async fn create_dataset(
        &self,
        display_name: &str,
        metadata_schema_uri: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/datasets", self.base_url());
        let dataset = VertexDataset {
            name: None,
            display_name: display_name.to_string(),
            metadata_schema_uri: metadata_schema_uri.to_string(),
            metadata: None,
        };
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&dataset)
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Get a dataset by ID.
    pub async fn get_dataset(
        &self,
        dataset_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/datasets/{}", self.base_url(), dataset_id);
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Import data into a dataset from GCS.
    pub async fn import_data(
        &self,
        dataset_id: &str,
        gcs_uris: Vec<String>,
        import_schema_uri: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/datasets/{}:import", self.base_url(), dataset_id);
        let config = ImportDataConfig {
            gcs_source: GcsSource { uris: gcs_uris },
            import_schema_uri: import_schema_uri.to_string(),
        };
        let body = serde_json::json!({ "importConfigs": [config] });
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Export a dataset to GCS.
    pub async fn export_dataset(
        &self,
        dataset_id: &str,
        gcs_output_uri: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/datasets/{}:export", self.base_url(), dataset_id);
        let body = serde_json::json!({
            "exportConfig": {
                "gcsDestination": { "outputUriPrefix": gcs_output_uri }
            }
        });
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Delete a dataset.
    pub async fn delete_dataset(
        &self,
        dataset_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/datasets/{}", self.base_url(), dataset_id);
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.into())
    }

    // ─── Model Management ───

    /// List all models in the project/location.
    pub async fn list_models(&self) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/models", self.base_url());
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Get a model by ID.
    pub async fn get_model(
        &self,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/models/{}", self.base_url(), model_id);
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Delete a model.
    pub async fn delete_model(
        &self,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/models/{}", self.base_url(), model_id);
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.into())
    }

    // ─── Endpoint Management ───

    /// Create a new endpoint for model deployment.
    pub async fn create_endpoint(
        &self,
        display_name: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/endpoints", self.base_url());
        let body = CreateEndpointRequest {
            display_name: display_name.to_string(),
        };
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Deploy a model to an endpoint.
    pub async fn deploy_model(
        &self,
        endpoint_id: &str,
        model_resource_name: &str,
        display_name: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/endpoints/{}:deployModel", self.base_url(), endpoint_id);
        let body = DeployModelRequest {
            deployed_model: DeployedModel {
                model: model_resource_name.to_string(),
                display_name: display_name.to_string(),
                dedicated_resources: None,
                automatic_resources: Some(AutomaticResources {
                    min_replica_count: 1,
                    max_replica_count: 1,
                }),
            },
        };
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    /// Undeploy a model from an endpoint.
    pub async fn undeploy_model(
        &self,
        endpoint_id: &str,
        deployed_model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/endpoints/{}:undeployModel",
            self.base_url(),
            endpoint_id
        );
        let body = UndeployModelRequest {
            deployed_model_id: deployed_model_id.to_string(),
        };
        let token = self
            .get_auth_token()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

// ─── LlmProvider Trait Implementation ───

#[async_trait]
impl LlmProvider for VertexAI {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = Self::resolve_model(&req.model)?;
        let url = format!(
            "{}/publishers/google/models/{}:generateContent",
            self.base_url(),
            model_id
        );

        let gemini_req = build_gemini_request(&req, None);
        let token = self.get_auth_token().await?;

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&gemini_req)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            return Err(CloudError::RateLimit { retry_after });
        }
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: body,
                retryable: status >= 500,
            });
        }

        let gemini_resp: GeminiResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        parse_gemini_response(gemini_resp)
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_id = Self::resolve_model(&req.model)?;
        let url = format!(
            "{}/publishers/google/models/{}:streamGenerateContent?alt=sse",
            self.base_url(),
            model_id
        );

        let gemini_req = build_gemini_request(&req, None);
        let token = self.get_auth_token().await?;

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&gemini_req)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: body,
                retryable: status >= 500,
            });
        }

        let byte_stream = response.bytes_stream();
        let event_stream = byte_stream
            .map(|chunk_result| match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    parse_sse_chunk(&text)
                }
                Err(e) => vec![LlmStreamEvent::Error(CloudError::Network { source: e })],
            })
            .flat_map(stream::iter);

        Ok(Box::pin(event_stream))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let model_id = "text-embedding-004";
        let url = format!(
            "{}/publishers/google/models/{}:predict",
            self.base_url(),
            model_id
        );

        let embed_req = EmbedRequest {
            instances: texts
                .into_iter()
                .map(|t| EmbedInstance { content: t })
                .collect(),
        };

        let token = self.get_auth_token().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&embed_req)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: body,
                retryable: status >= 500,
            });
        }

        let pred_resp: EmbedPredictResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        Ok(EmbedResponse {
            embeddings: pred_resp
                .predictions
                .into_iter()
                .map(|p| p.embeddings.values)
                .collect(),
        })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model_id = Self::resolve_model(&req.model)?;
        let url = format!(
            "{}/publishers/google/models/{}:generateContent",
            self.base_url(),
            model_id
        );

        let gemini_req = build_gemini_request(&req, Some(&tools));
        let token = self.get_auth_token().await?;

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&gemini_req)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            return Err(CloudError::RateLimit { retry_after });
        }
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: body,
                retryable: status >= 500,
            });
        }

        let gemini_resp: GeminiResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        parse_tool_call_response(gemini_resp)
    }
}

// ─── Helper Functions ───

/// Build a GeminiRequest from an LlmRequest and optional tool definitions.
fn build_gemini_request(req: &LlmRequest, tools: Option<&Vec<ToolDefinition>>) -> GeminiRequest {
    let contents: Vec<Content> = req
        .messages
        .iter()
        .map(|msg| Content {
            role: Some(map_role(&msg.role)),
            parts: vec![Part {
                text: Some(msg.content.clone()),
                function_call: None,
                function_response: None,
            }],
        })
        .collect();

    let system_instruction = req.system_prompt.as_ref().map(|prompt| Content {
        role: None,
        parts: vec![Part {
            text: Some(prompt.clone()),
            function_call: None,
            function_response: None,
        }],
    });

    let generation_config = if req.max_tokens.is_some() || req.temperature.is_some() {
        Some(GenerationConfig {
            max_output_tokens: req.max_tokens,
            temperature: req.temperature,
        })
    } else {
        None
    };

    let gemini_tools = tools.map(|t| {
        vec![GeminiTool {
            function_declarations: t
                .iter()
                .map(|td| FunctionDeclaration {
                    name: td.name.clone(),
                    description: td.description.clone(),
                    parameters: td.parameters.clone(),
                })
                .collect(),
        }]
    });

    GeminiRequest {
        contents,
        system_instruction,
        generation_config,
        tools: gemini_tools,
    }
}

/// Map standard role names to Gemini role names.
fn map_role(role: &str) -> String {
    match role {
        "assistant" => "model".to_string(),
        other => other.to_string(),
    }
}

/// Parse a GeminiResponse into an LlmResponse.
fn parse_gemini_response(resp: GeminiResponse) -> Result<LlmResponse, CloudError> {
    let candidate = resp
        .candidates
        .and_then(|c| c.into_iter().next())
        .ok_or_else(|| CloudError::Provider {
            http_status: 200,
            message: "no candidates in response".to_string(),
            retryable: false,
        })?;

    let text = candidate
        .content
        .and_then(|c| c.parts.into_iter().next())
        .and_then(|p| p.text)
        .unwrap_or_default();

    let finish_reason = match candidate.finish_reason.as_deref() {
        Some("STOP") => FinishReason::Stop,
        Some("MAX_TOKENS") => FinishReason::Length,
        Some("TOOL_CALL") | Some("FUNCTION_CALL") => FinishReason::ToolCall,
        Some(other) => FinishReason::Other(other.to_string()),
        None => FinishReason::Stop,
    };

    let usage = resp.usage_metadata.map(|u| UsageStats {
        prompt_tokens: u.prompt_token_count.unwrap_or(0),
        completion_tokens: u.candidates_token_count.unwrap_or(0),
    });

    Ok(LlmResponse {
        text,
        finish_reason,
        usage,
    })
}

/// Parse a GeminiResponse that may contain a function call.
fn parse_tool_call_response(resp: GeminiResponse) -> Result<ToolCallResponse, CloudError> {
    let candidate = resp
        .candidates
        .and_then(|c| c.into_iter().next())
        .ok_or_else(|| CloudError::Provider {
            http_status: 200,
            message: "no candidates in response".to_string(),
            retryable: false,
        })?;

    if let Some(content) = candidate.content {
        for part in &content.parts {
            if let Some(fc) = &part.function_call {
                return Ok(ToolCallResponse::ToolCall {
                    name: fc.name.clone(),
                    arguments: fc.args.clone(),
                });
            }
        }
        let text = content
            .parts
            .into_iter()
            .filter_map(|p| p.text)
            .collect::<Vec<_>>()
            .join("");
        let llm_resp = LlmResponse {
            text,
            finish_reason: FinishReason::Stop,
            usage: resp.usage_metadata.map(|u| UsageStats {
                prompt_tokens: u.prompt_token_count.unwrap_or(0),
                completion_tokens: u.candidates_token_count.unwrap_or(0),
            }),
        };
        return Ok(ToolCallResponse::Text(llm_resp));
    }

    Err(CloudError::Provider {
        http_status: 200,
        message: "empty content in response".to_string(),
        retryable: false,
    })
}

/// Parse an SSE chunk from the streamGenerateContent endpoint.
fn parse_sse_chunk(raw: &str) -> Vec<LlmStreamEvent> {
    let mut events = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if let Some(data) = line.strip_prefix("data: ") {
            if data == "[DONE]" {
                events.push(LlmStreamEvent::Done(FinishReason::Stop));
                continue;
            }
            if let Ok(chunk) = serde_json::from_str::<GeminiStreamChunk>(data) {
                if let Some(candidates) = chunk.candidates {
                    for candidate in candidates {
                        if let Some(content) = candidate.content {
                            for part in content.parts {
                                if let Some(text) = part.text {
                                    events.push(LlmStreamEvent::DeltaText(text));
                                }
                            }
                        }
                        if let Some(reason) = candidate.finish_reason {
                            let fr = match reason.as_str() {
                                "STOP" => FinishReason::Stop,
                                "MAX_TOKENS" => FinishReason::Length,
                                other => FinishReason::Other(other.to_string()),
                            };
                            events.push(LlmStreamEvent::Done(fr));
                        }
                    }
                }
                if let Some(usage) = chunk.usage_metadata {
                    events.push(LlmStreamEvent::Usage(UsageStats {
                        prompt_tokens: usage.prompt_token_count.unwrap_or(0),
                        completion_tokens: usage.candidates_token_count.unwrap_or(0),
                    }));
                }
            }
        }
    }
    events
}

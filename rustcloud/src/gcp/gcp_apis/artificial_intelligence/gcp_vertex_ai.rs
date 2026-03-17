#![allow(clippy::result_large_err)]

use async_trait::async_trait;
use futures::stream;
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse,
    ToolDefinition, UsageStats,
};

use super::super::auth::gcp_auth::retrieve_token;

/// Default embedding model for Vertex AI text embeddings.
const DEFAULT_EMBED_MODEL: &str = "text-embedding-004";

pub struct VertexAI {
    client: Client,
    project_id: String,
    location: String,
    embed_model: String,
}

// ── Vertex AI request/response types ─────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug)]
struct FunctionCall {
    name: String,
    args: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_data: Option<InlineData>,
    /// Present when the model requests a function/tool call.
    #[serde(rename = "functionCall", skip_serializing_if = "Option::is_none")]
    function_call: Option<FunctionCall>,
    /// Present when sending a tool result back (not used in provider output).
    #[serde(rename = "functionResponse", skip_serializing_if = "Option::is_none")]
    function_response: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenerateRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tool {
    function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FunctionDeclaration {
    name: String,
    description: String,
    parameters: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenerateResponse {
    candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    content: Content,
    #[serde(rename = "finishReason", default)]
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsageMetadata {
    #[serde(rename = "promptTokenCount", default)]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount", default)]
    candidates_token_count: u32,
}

// ── Embedding types ───────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug)]
struct EmbedInstance {
    content: String,
    #[serde(rename = "taskType", skip_serializing_if = "Option::is_none")]
    task_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedPrediction {
    embeddings: EmbedValues,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedValues {
    values: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedBatchResponse {
    predictions: Vec<EmbedPrediction>,
}

// ── Helper functions ──────────────────────────────────────────────────────────

fn extract_model_name(req: &LlmRequest) -> String {
    match &req.model {
        crate::types::llm::ModelRef::Provider(name) => name.clone(),
        crate::types::llm::ModelRef::Logical { family, tier: _ } => family.clone(),
        crate::types::llm::ModelRef::Deployment(name) => name.clone(),
    }
}

fn map_finish_reason(reason: &str) -> FinishReason {
    match reason {
        "STOP" => FinishReason::Stop,
        "MAX_TOKENS" => FinishReason::Length,
        _ => FinishReason::Other(reason.to_string()),
    }
}

fn build_generate_request(req: &LlmRequest, tools: Option<Vec<ToolDefinition>>) -> GenerateRequest {
    let contents: Vec<Content> = req
        .messages
        .iter()
        .map(|msg| Content {
            role: msg.role.clone(),
            parts: vec![Part {
                text: Some(msg.content.clone()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        })
        .collect();

    // Vertex AI requires system_instruction role to be "user" in the request
    let system_instruction = req.system_prompt.as_ref().map(|prompt| Content {
        role: "user".to_string(),
        parts: vec![Part {
            text: Some(prompt.clone()),
            inline_data: None,
            function_call: None,
            function_response: None,
        }],
    });

    let tools_param = tools.map(|tool_defs| {
        vec![Tool {
            function_declarations: tool_defs
                .into_iter()
                .map(|t| FunctionDeclaration {
                    name: t.name,
                    description: t.description,
                    parameters: t.parameters,
                })
                .collect(),
        }]
    });

    GenerateRequest {
        contents,
        generation_config: GenerationConfig {
            max_output_tokens: req.max_tokens,
            temperature: req.temperature,
        },
        system_instruction,
        tools: tools_param,
    }
}

async fn get_auth_token() -> Result<String, CloudError> {
    retrieve_token()
        .await
        .map_err(|e| CloudError::Auth {
            message: format!("GCP token retrieval failed: {}", e),
        })
}

// ── VertexAI implementation ───────────────────────────────────────────────────

impl VertexAI {
    pub fn new(project_id: &str, location: &str) -> Self {
        Self {
            client: Client::new(),
            project_id: project_id.to_string(),
            location: location.to_string(),
            embed_model: DEFAULT_EMBED_MODEL.to_string(),
        }
    }

    pub fn with_embed_model(mut self, model: &str) -> Self {
        self.embed_model = model.to_string();
        self
    }

    fn generate_url(&self, model_name: &str) -> String {
        format!(
            "https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}/publishers/google/models/{model}:generateContent",
            location = self.location,
            project = self.project_id,
            model = model_name,
        )
    }

    fn stream_url(&self, model_name: &str) -> String {
        format!(
            "https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}/publishers/google/models/{model}:streamGenerateContent?alt=sse",
            location = self.location,
            project = self.project_id,
            model = model_name,
        )
    }

    fn embed_url(&self) -> String {
        format!(
            "https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}/publishers/google/models/{model}:predict",
            location = self.location,
            project = self.project_id,
            model = self.embed_model,
        )
    }
}

#[async_trait]
impl LlmProvider for VertexAI {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_name = extract_model_name(&req);
        let url = self.generate_url(&model_name);
        let token = get_auth_token().await?;
        let body = build_generate_request(&req, None);

        let http_resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = http_resp.status();
        if !status.is_success() {
            let message = http_resp
                .text()
                .await
                .unwrap_or_else(|_| status.to_string());
            return Err(CloudError::Provider {
                http_status: status.as_u16(),
                message,
                retryable: status.as_u16() == 429 || status.as_u16() >= 500,
            });
        }

        let gen_response: GenerateResponse = http_resp
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let candidate = gen_response.candidates.into_iter().next().ok_or(
            CloudError::Provider {
                http_status: 200,
                message: "No candidates in response".to_string(),
                retryable: false,
            },
        )?;

        let text = candidate
            .content
            .parts
            .iter()
            .find_map(|p| p.text.clone())
            .unwrap_or_default();

        let usage = gen_response.usage_metadata.map(|m| UsageStats {
            prompt_tokens: m.prompt_token_count,
            completion_tokens: m.candidates_token_count,
        });

        Ok(LlmResponse {
            text,
            finish_reason: map_finish_reason(&candidate.finish_reason),
            usage,
        })
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_name = extract_model_name(&req);
        let url = self.stream_url(&model_name);
        let token = get_auth_token().await?;
        let body = build_generate_request(&req, None);

        let http_resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = http_resp.status();
        if !status.is_success() {
            let message = http_resp
                .text()
                .await
                .unwrap_or_else(|_| status.to_string());
            return Err(CloudError::Provider {
                http_status: status.as_u16(),
                message,
                retryable: status.as_u16() == 429 || status.as_u16() >= 500,
            });
        }

        // Vertex AI SSE stream: each event is "data: <json>\n\n"
        // Collect the full body, split on "data:" prefix lines, parse each chunk.
        let body_bytes = http_resp
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let body_str = String::from_utf8_lossy(&body_bytes);
        let mut events: Vec<LlmStreamEvent> = Vec::new();
        let mut last_finish = FinishReason::Stop;

        for line in body_str.lines() {
            let data = if let Some(stripped) = line.strip_prefix("data: ") {
                stripped.trim()
            } else {
                continue;
            };

            if data.is_empty() || data == "[DONE]" {
                continue;
            }

            match serde_json::from_str::<GenerateResponse>(data) {
                Ok(gr) => {
                    for candidate in gr.candidates {
                        last_finish = map_finish_reason(&candidate.finish_reason);
                        for part in candidate.content.parts {
                            if let Some(text) = part.text {
                                if !text.is_empty() {
                                    events.push(LlmStreamEvent::DeltaText(text));
                                }
                            }
                        }
                    }
                    if let Some(meta) = gr.usage_metadata {
                        events.push(LlmStreamEvent::Usage(UsageStats {
                            prompt_tokens: meta.prompt_token_count,
                            completion_tokens: meta.candidates_token_count,
                        }));
                    }
                }
                Err(_) => {
                    // Partial JSON or non-data line — skip silently
                }
            }
        }

        events.push(LlmStreamEvent::Done(last_finish));
        Ok(Box::pin(stream::iter(events)))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let url = self.embed_url();
        let token = get_auth_token().await?;

        // Vertex AI text-embedding-004 accepts a batch of instances in one call
        let instances: Vec<EmbedInstance> = texts
            .into_iter()
            .map(|t| EmbedInstance {
                content: t,
                task_type: Some("RETRIEVAL_DOCUMENT".to_string()),
            })
            .collect();

        let body = json!({ "instances": instances });

        let http_resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = http_resp.status();
        if !status.is_success() {
            let message = http_resp
                .text()
                .await
                .unwrap_or_else(|_| status.to_string());
            return Err(CloudError::Provider {
                http_status: status.as_u16(),
                message,
                retryable: status.as_u16() == 429 || status.as_u16() >= 500,
            });
        }

        let embed_response: EmbedBatchResponse = http_resp
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let embeddings = embed_response
            .predictions
            .into_iter()
            .map(|p| p.embeddings.values)
            .collect();

        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model_name = extract_model_name(&req);
        let url = self.generate_url(&model_name);
        let token = get_auth_token().await?;
        let body = build_generate_request(&req, Some(tools));

        let http_resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = http_resp.status();
        if !status.is_success() {
            let message = http_resp
                .text()
                .await
                .unwrap_or_else(|_| status.to_string());
            return Err(CloudError::Provider {
                http_status: status.as_u16(),
                message,
                retryable: status.as_u16() == 429 || status.as_u16() >= 500,
            });
        }

        let gen_response: GenerateResponse = http_resp
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let candidate = gen_response.candidates.into_iter().next().ok_or(
            CloudError::Provider {
                http_status: 200,
                message: "No candidates in response".to_string(),
                retryable: false,
            },
        )?;

        // Check all parts for a functionCall before falling back to text
        for part in &candidate.content.parts {
            if let Some(fc) = &part.function_call {
                return Ok(ToolCallResponse::ToolCall {
                    name: fc.name.clone(),
                    arguments: fc.args.clone(),
                });
            }
        }

        let text = candidate
            .content
            .parts
            .iter()
            .find_map(|p| p.text.clone())
            .unwrap_or_default();

        let usage = gen_response.usage_metadata.map(|m| UsageStats {
            prompt_tokens: m.prompt_token_count,
            completion_tokens: m.candidates_token_count,
        });

        Ok(ToolCallResponse::Text(LlmResponse {
            text,
            finish_reason: map_finish_reason(&candidate.finish_reason),
            usage,
        }))
    }
}

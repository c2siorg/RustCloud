use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};

use crate::errors::CloudError;
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse,
    ToolDefinition, UsageStats,
};

/// Selects which Google GenAI backend to use.
pub enum Backend {
    /// Gemini Developer API — API key auth, free tier available, no GCP project required.
    GeminiApi { api_key: String },
    /// Vertex AI — OAuth service account auth, requires GCP project and billing.
    VertexAi { project: String, region: String },
}

const GEMINI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta";
const GEMINI_EMBED_MODEL: &str = "gemini-embedding-001";
const VERTEX_EMBED_MODEL: &str = "text-embedding-004";

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct SystemInstruction {
    parts: Vec<Part>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Tool {
    function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerateRequest {
    contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: Option<String>,
    #[serde(rename = "functionCall")]
    function_call: Option<FunctionCallPart>,
}

#[derive(Deserialize)]
struct FunctionCallPart {
    name: String,
    args: serde_json::Value,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: Option<ResponseContent>,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageMetadata {
    prompt_token_count: Option<u32>,
    candidates_token_count: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateResponse {
    candidates: Vec<Candidate>,
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Serialize)]
struct GeminiEmbedPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiEmbedContent {
    parts: Vec<GeminiEmbedPart>,
}

#[derive(Serialize)]
struct GeminiEmbedRequest {
    model: String,
    content: GeminiEmbedContent,
}

#[derive(Serialize)]
struct GeminiBatchEmbedRequest {
    requests: Vec<GeminiEmbedRequest>,
}

#[derive(Deserialize)]
struct GeminiEmbedValues {
    values: Vec<f32>,
}

#[derive(Deserialize)]
struct GeminiBatchEmbedResponse {
    embeddings: Vec<GeminiEmbedValues>,
}

#[derive(Serialize)]
struct VertexEmbedInstance {
    content: String,
}

#[derive(Serialize)]
struct VertexEmbedRequest {
    instances: Vec<VertexEmbedInstance>,
}

#[derive(Deserialize)]
struct VertexEmbedValues {
    values: Vec<f32>,
}

#[derive(Deserialize)]
struct VertexEmbedPrediction {
    embeddings: VertexEmbedValues,
}

#[derive(Deserialize)]
struct VertexEmbedResponse {
    predictions: Vec<VertexEmbedPrediction>,
}

pub struct GoogleGenAiAdapter {
    client: Client,
    backend: Backend,
    model: String,
}

impl GoogleGenAiAdapter {
    pub fn new(backend: Backend, model: String) -> Self {
        Self {
            client: Client::new(),
            backend,
            model,
        }
    }

    fn generate_url(&self) -> String {
        match &self.backend {
            Backend::GeminiApi { .. } => {
                format!("{}/models/{}:generateContent", GEMINI_BASE, self.model)
            }
            Backend::VertexAi { project, region } => {
                format!(
                    "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
                    region, project, region, self.model
                )
            }
        }
    }

    fn stream_url(&self) -> String {
        match &self.backend {
            Backend::GeminiApi { .. } => {
                format!(
                    "{}/models/{}:streamGenerateContent?alt=sse",
                    GEMINI_BASE, self.model
                )
            }
            Backend::VertexAi { project, region } => {
                format!(
                    "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent?alt=sse",
                    region, project, region, self.model
                )
            }
        }
    }

    fn embed_url(&self) -> String {
        match &self.backend {
            Backend::GeminiApi { .. } => {
                format!(
                    "{}/models/{}:batchEmbedContents",
                    GEMINI_BASE, GEMINI_EMBED_MODEL
                )
            }
            Backend::VertexAi { project, region } => {
                format!(
                    "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:predict",
                    region, project, region, VERTEX_EMBED_MODEL
                )
            }
        }
    }

    async fn add_auth(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, CloudError> {
        match &self.backend {
            Backend::GeminiApi { api_key } => Ok(req.header("x-goog-api-key", api_key)),
            Backend::VertexAi { .. } => {
                let token = retrieve_token().await.map_err(|e| CloudError::Auth {
                    message: e.to_string(),
                })?;
                Ok(req.header(AUTHORIZATION, format!("Bearer {}", token)))
            }
        }
    }

    fn build_request(&self, req: &LlmRequest, tools: Option<Vec<Tool>>) -> GenerateRequest {
        let contents = req
            .messages
            .iter()
            .map(|m| Content {
                role: m.role.clone(),
                parts: vec![Part {
                    text: m.content.clone(),
                }],
            })
            .collect();

        let system_instruction = req.system_prompt.as_ref().map(|s| SystemInstruction {
            parts: vec![Part { text: s.clone() }],
        });

        let generation_config = if req.max_tokens.is_some() || req.temperature.is_some() {
            Some(GenerationConfig {
                max_output_tokens: req.max_tokens,
                temperature: req.temperature,
            })
        } else {
            None
        };

        GenerateRequest {
            contents,
            system_instruction,
            generation_config,
            tools,
        }
    }

    fn parse_response(&self, resp: GenerateResponse) -> Result<LlmResponse, CloudError> {
        let candidate = resp
            .candidates
            .into_iter()
            .next()
            .ok_or_else(|| CloudError::Provider {
                http_status: 200,
                message: "no candidates in response".to_string(),
                retryable: false,
            })?;

        let finish_reason = match candidate.finish_reason.as_deref() {
            Some("STOP") => FinishReason::Stop,
            Some("MAX_TOKENS") => FinishReason::Length,
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        let text = candidate
            .content
            .and_then(|c| c.parts.into_iter().next())
            .and_then(|p| p.text)
            .unwrap_or_default();

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
}

#[async_trait]
impl LlmProvider for GoogleGenAiAdapter {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let body = self.build_request(&req, None);
        let request = self.add_auth(self.client.post(self.generate_url())).await?;

        let response = request
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let msg = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: msg,
                retryable: status >= 500,
            });
        }

        let resp: GenerateResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        self.parse_response(resp)
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let body = self.build_request(&req, None);
        let request = self.add_auth(self.client.post(self.stream_url())).await?;

        let response = request
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let msg = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: msg,
                retryable: status >= 500,
            });
        }

        let (mut tx, rx) = mpsc::unbounded::<LlmStreamEvent>();

        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();
            'outer: while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        loop {
                            if let Some(pos) = buffer.find('\n') {
                                let line = buffer[..pos].trim().to_string();
                                buffer.drain(..=pos);
                                if line.starts_with("data: ") {
                                    let json_str = &line[6..];
                                    if json_str == "[DONE]" {
                                        break 'outer;
                                    }
                                    if let Ok(resp) =
                                        serde_json::from_str::<GenerateResponse>(json_str)
                                    {
                                        for candidate in resp.candidates {
                                            if let Some(content) = candidate.content {
                                                for part in content.parts {
                                                    if let Some(t) = part.text {
                                                        tx.send(LlmStreamEvent::DeltaText(t))
                                                            .await
                                                            .ok();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            tx.send(LlmStreamEvent::Done(FinishReason::Stop)).await.ok();
        });

        Ok(Box::pin(rx))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let request = self.add_auth(self.client.post(self.embed_url())).await?;

        match &self.backend {
            Backend::GeminiApi { .. } => {
                let model_name = format!("models/{}", GEMINI_EMBED_MODEL);
                let body = GeminiBatchEmbedRequest {
                    requests: texts
                        .into_iter()
                        .map(|t| GeminiEmbedRequest {
                            model: model_name.clone(),
                            content: GeminiEmbedContent {
                                parts: vec![GeminiEmbedPart { text: t }],
                            },
                        })
                        .collect(),
                };

                let response = request
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| CloudError::Network { source: e })?;

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let msg = response.text().await.unwrap_or_default();
                    return Err(CloudError::Provider {
                        http_status: status,
                        message: msg,
                        retryable: status >= 500,
                    });
                }

                let resp: GeminiBatchEmbedResponse = response
                    .json()
                    .await
                    .map_err(|e| CloudError::Network { source: e })?;

                Ok(EmbedResponse {
                    embeddings: resp.embeddings.into_iter().map(|e| e.values).collect(),
                })
            }
            Backend::VertexAi { .. } => {
                let body = VertexEmbedRequest {
                    instances: texts
                        .into_iter()
                        .map(|t| VertexEmbedInstance { content: t })
                        .collect(),
                };

                let response = request
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| CloudError::Network { source: e })?;

                if !response.status().is_success() {
                    let status = response.status().as_u16();
                    let msg = response.text().await.unwrap_or_default();
                    return Err(CloudError::Provider {
                        http_status: status,
                        message: msg,
                        retryable: status >= 500,
                    });
                }

                let resp: VertexEmbedResponse = response
                    .json()
                    .await
                    .map_err(|e| CloudError::Network { source: e })?;

                Ok(EmbedResponse {
                    embeddings: resp
                        .predictions
                        .into_iter()
                        .map(|p| p.embeddings.values)
                        .collect(),
                })
            }
        }
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let gemini_tools = vec![Tool {
            function_declarations: tools
                .into_iter()
                .map(|t| FunctionDeclaration {
                    name: t.name,
                    description: t.description,
                    parameters: t.parameters,
                })
                .collect(),
        }];

        let body = self.build_request(&req, Some(gemini_tools));
        let request = self.add_auth(self.client.post(self.generate_url())).await?;

        let response = request
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let msg = response.text().await.unwrap_or_default();
            return Err(CloudError::Provider {
                http_status: status,
                message: msg,
                retryable: status >= 500,
            });
        }

        let resp: GenerateResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let candidate = resp.candidates.into_iter().next().ok_or_else(|| {
            CloudError::Provider {
                http_status: 200,
                message: "no candidates in response".to_string(),
                retryable: false,
            }
        })?;

        let finish_reason = match candidate.finish_reason.as_deref() {
            Some("STOP") => FinishReason::Stop,
            Some("MAX_TOKENS") => FinishReason::Length,
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        if let Some(content) = candidate.content {
            for part in content.parts {
                if let Some(fc) = part.function_call {
                    return Ok(ToolCallResponse::ToolCall {
                        name: fc.name,
                        arguments: fc.args,
                    });
                }
                if let Some(text) = part.text {
                    return Ok(ToolCallResponse::Text(LlmResponse {
                        text,
                        finish_reason,
                        usage: None,
                    }));
                }
            }
        }

        Err(CloudError::Provider {
            http_status: 200,
            message: "empty response".to_string(),
            retryable: false,
        })
    }
}

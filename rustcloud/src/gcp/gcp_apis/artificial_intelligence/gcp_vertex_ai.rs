use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;
use reqwest::header::AUTHORIZATION;
use tokio::sync::Mutex;

use crate::errors::CloudError;
use crate::gcp::gcp_apis::auth::gcp_auth::ServiceAccountTokenProvider;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::traits::token_provider::TokenProvider;
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

struct CachedToken {
    value: String,
    expires_at: Instant,
}

pub struct VertexAiProvider {
    http: reqwest::Client,
    project: String,
    location: String,
    auth: Arc<dyn TokenProvider>,
    token: Mutex<CachedToken>,
}

impl VertexAiProvider {
    pub async fn new(
        project: impl Into<String>,
        location: impl Into<String>,
    ) -> Result<Self, CloudError> {
        let project = project.into();
        let location = location.into();
        let auth: Arc<dyn TokenProvider> = Arc::new(
            ServiceAccountTokenProvider::new(
                PathBuf::from("service-account.json"),
                vec!["https://www.googleapis.com/auth/cloud-platform".to_string()],
            )
            .map_err(|e| CloudError::Auth { message: e.to_string() })?,
        );
        let token = auth.get_token().await.map_err(|e| CloudError::Auth {
            message: e.to_string(),
        })?;
        Ok(Self {
            http: reqwest::Client::new(),
            project,
            location,
            auth,
            token: Mutex::new(CachedToken {
                value: token,
                expires_at: Instant::now() + Duration::from_secs(3600),
            }),
        })
    }

    pub fn with_http_client(
        http: reqwest::Client,
        project: impl Into<String>,
        location: impl Into<String>,
        auth: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            http,
            project: project.into(),
            location: location.into(),
            auth,
            token: Mutex::new(CachedToken {
                value: String::new(),
                expires_at: Instant::now(),
            }),
        }
    }

    pub(crate) async fn get_token(&self) -> Result<String, CloudError> {
        let mut cached = self.token.lock().await;
        if cached.expires_at.saturating_duration_since(Instant::now()) < Duration::from_secs(300) {
            let fresh = self.auth.get_token().await.map_err(|e| CloudError::Auth {
                message: e.to_string(),
            })?;
            cached.value = fresh;
            cached.expires_at = Instant::now() + Duration::from_secs(3600);
        }
        Ok(cached.value.clone())
    }
}

pub(crate) fn extract_model_id(model: &ModelRef) -> Result<String, CloudError> {
    match model {
        ModelRef::Provider(id) => Ok(id.clone()),
        ModelRef::Logical { family, tier } => Ok(match tier.as_deref() {
            Some(t) => format!("{}-{}", family, t),
            None => family.clone(),
        }),
        ModelRef::Deployment(_) => Err(CloudError::Unsupported {
            feature: "Vertex AI deployment routing",
        }),
    }
}

pub(crate) fn vertex_endpoint(
    project: &str,
    location: &str,
    model: &str,
    method: &str,
) -> String {
    format!(
        "https://{location}-aiplatform.googleapis.com/v1/projects/{project}\
/locations/{location}/publishers/google/models/{model}:{method}"
    )
}

pub(crate) fn build_vertex_request(req: &LlmRequest) -> Result<serde_json::Value, CloudError> {
    let mut contents = Vec::with_capacity(req.messages.len());
    for msg in &req.messages {
        let role = match msg.role.as_str() {
            "user" => "user",
            "assistant" => "model",
            other => {
                return Err(CloudError::Provider {
                    http_status: 0,
                    message: format!("unsupported message role: {other}"),
                    retryable: false,
                })
            }
        };
        contents.push(serde_json::json!({
            "role": role,
            "parts": [{ "text": msg.content }]
        }));
    }

    let mut body = serde_json::json!({ "contents": contents });

    let mut gen_config = serde_json::Map::new();
    if let Some(max_tokens) = req.max_tokens {
        gen_config.insert("maxOutputTokens".to_string(), serde_json::json!(max_tokens));
    }
    if let Some(temp) = req.temperature {
        gen_config.insert("temperature".to_string(), serde_json::json!(temp));
    }
    if !gen_config.is_empty() {
        body["generationConfig"] = serde_json::Value::Object(gen_config);
    }

    if let Some(system) = &req.system_prompt {
        body["systemInstruction"] = serde_json::json!({ "parts": [{ "text": system }] });
    }

    Ok(body)
}

pub(crate) fn map_finish_reason(s: &str) -> FinishReason {
    match s {
        "STOP" => FinishReason::Stop,
        "MAX_TOKENS" => FinishReason::Length,
        "TOOL_CALLS" | "FUNCTION_CALL" => FinishReason::ToolCall,
        other => FinishReason::Other(other.to_string()),
    }
}

pub(crate) fn parse_vertex_response(
    json: &serde_json::Value,
) -> Result<LlmResponse, CloudError> {
    let candidate = json["candidates"].get(0).ok_or_else(|| CloudError::Provider {
        http_status: 0,
        message: "response contained no candidates".to_string(),
        retryable: false,
    })?;

    let text = candidate["content"]["parts"]
        .as_array()
        .and_then(|p| p.first())
        .and_then(|p| p["text"].as_str())
        .unwrap_or("")
        .to_string();

    let finish_reason = candidate["finishReason"]
        .as_str()
        .map(map_finish_reason)
        .unwrap_or(FinishReason::Other("UNKNOWN".to_string()));

    let meta = &json["usageMetadata"];
    let usage = match (
        meta["promptTokenCount"].as_u64(),
        meta["candidatesTokenCount"].as_u64(),
    ) {
        (Some(p), Some(c)) => Some(UsageStats {
            prompt_tokens: p as u32,
            completion_tokens: c as u32,
        }),
        _ => None,
    };

    Ok(LlmResponse { text, finish_reason, usage })
}

pub(crate) fn map_vertex_http_error(status: u16, body: &str) -> CloudError {
    match status {
        401 | 403 => CloudError::Auth { message: body.to_string() },
        429 => CloudError::RateLimit { retry_after: None },
        400 => CloudError::Provider {
            http_status: 400,
            message: body.to_string(),
            retryable: false,
        },
        500 | 503 => CloudError::Provider {
            http_status: status,
            message: body.to_string(),
            retryable: true,
        },
        _ => CloudError::Provider {
            http_status: status,
            message: body.to_string(),
            retryable: status >= 500,
        },
    }
}

pub(crate) fn parse_sse_line(line: &str) -> Option<serde_json::Value> {
    serde_json::from_str(line.strip_prefix("data: ")?).ok()
}

pub(crate) fn sse_chunk_to_events(json: &serde_json::Value) -> Vec<LlmStreamEvent> {
    let mut events = Vec::new();
    let Some(candidate) = json["candidates"].get(0) else {
        return events;
    };

    if let Some(text) = candidate["content"]["parts"]
        .as_array()
        .and_then(|p| p.first())
        .and_then(|p| p["text"].as_str())
    {
        if !text.is_empty() {
            events.push(LlmStreamEvent::DeltaText(text.to_string()));
        }
    }

    if let Some(reason) = candidate["finishReason"].as_str() {
        if !reason.is_empty() && reason != "FINISH_REASON_UNSPECIFIED" {
            let meta = &json["usageMetadata"];
            if let (Some(p), Some(c)) = (
                meta["promptTokenCount"].as_u64(),
                meta["candidatesTokenCount"].as_u64(),
            ) {
                events.push(LlmStreamEvent::Usage(UsageStats {
                    prompt_tokens: p as u32,
                    completion_tokens: c as u32,
                }));
            }
            events.push(LlmStreamEvent::Done(map_finish_reason(reason)));
        }
    }

    events
}

pub(crate) fn build_embed_request(texts: &[String]) -> serde_json::Value {
    let instances: Vec<serde_json::Value> =
        texts.iter().map(|t| serde_json::json!({ "content": t })).collect();
    serde_json::json!({ "instances": instances })
}

pub(crate) fn parse_embed_response(json: &serde_json::Value) -> Result<EmbedResponse, CloudError> {
    let predictions = json["predictions"].as_array().ok_or_else(|| CloudError::Provider {
        http_status: 0,
        message: "response missing predictions".to_string(),
        retryable: false,
    })?;

    let mut embeddings = Vec::with_capacity(predictions.len());
    for p in predictions {
        let values = p["embeddings"]["values"].as_array().ok_or_else(|| CloudError::Provider {
            http_status: 0,
            message: "malformed embedding in response".to_string(),
            retryable: false,
        })?;
        embeddings.push(values.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect());
    }

    Ok(EmbedResponse { embeddings })
}

pub(crate) fn build_tool_request(
    req: &LlmRequest,
    tools: &[ToolDefinition],
) -> Result<serde_json::Value, CloudError> {
    let mut body = build_vertex_request(req)?;
    if !tools.is_empty() {
        let declarations: Vec<serde_json::Value> = tools
            .iter()
            .map(|t| serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            }))
            .collect();
        body["tools"] = serde_json::json!([{ "functionDeclarations": declarations }]);
    }
    Ok(body)
}

pub(crate) fn parse_tool_response(json: &serde_json::Value) -> Result<ToolCallResponse, CloudError> {
    let candidate = json["candidates"].get(0).ok_or_else(|| CloudError::Provider {
        http_status: 0,
        message: "response contained no candidates".to_string(),
        retryable: false,
    })?;

    if let Some(parts) = candidate["content"]["parts"].as_array() {
        for part in parts {
            if let Some(fc) = part.get("functionCall") {
                return Ok(ToolCallResponse::ToolCall {
                    name: fc["name"].as_str().unwrap_or("").to_string(),
                    arguments: fc["args"].clone(),
                });
            }
        }
    }

    parse_vertex_response(json).map(ToolCallResponse::Text)
}

#[async_trait]
impl LlmProvider for VertexAiProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = extract_model_id(&req.model)?;
        let token = self.get_token().await?;
        let url = vertex_endpoint(&self.project, &self.location, &model_id, "generateContent");
        let body = build_vertex_request(&req)?;

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let text = response.text().await.unwrap_or_default();
            return Err(map_vertex_http_error(status, &text));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let resp_json: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| CloudError::Serialization { source: e })?;

        parse_vertex_response(&resp_json)
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_id = extract_model_id(&req.model)?;
        let token = self.get_token().await?;
        let url = format!(
            "{}?alt=sse",
            vertex_endpoint(&self.project, &self.location, &model_id, "streamGenerateContent")
        );
        let body = build_vertex_request(&req)?;

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let text = response.text().await.unwrap_or_default();
            return Err(map_vertex_http_error(status, &text));
        }

        let (mut tx, rx) = mpsc::channel::<LlmStreamEvent>(32);

        tokio::spawn(async move {
            let mut response = response;
            let mut buffer = String::new();

            loop {
                match response.chunk().await {
                    Ok(Some(bytes)) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(pos) = buffer.find('\n') {
                            let line = buffer[..pos].trim_end_matches('\r').to_string();
                            buffer = buffer[pos + 1..].to_string();
                            if let Some(json) = parse_sse_line(&line) {
                                for event in sse_chunk_to_events(&json) {
                                    if tx.send(event).await.is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let _ = tx
                            .send(LlmStreamEvent::Error(CloudError::Network { source: e }))
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(Box::pin(rx))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        if texts.is_empty() {
            return Ok(EmbedResponse { embeddings: vec![] });
        }

        let token = self.get_token().await?;
        let url = vertex_endpoint(&self.project, &self.location, "text-embedding-004", "predict");
        let body = build_embed_request(&texts);

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let text = response.text().await.unwrap_or_default();
            return Err(map_vertex_http_error(status, &text));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let resp_json: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| CloudError::Serialization { source: e })?;

        parse_embed_response(&resp_json)
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model_id = extract_model_id(&req.model)?;
        let token = self.get_token().await?;
        let url = vertex_endpoint(&self.project, &self.location, &model_id, "generateContent");
        let body = build_tool_request(&req, &tools)?;

        let response = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status >= 400 {
            let text = response.text().await.unwrap_or_default();
            return Err(map_vertex_http_error(status, &text));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let resp_json: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| CloudError::Serialization { source: e })?;

        parse_tool_response(&resp_json)
    }
}

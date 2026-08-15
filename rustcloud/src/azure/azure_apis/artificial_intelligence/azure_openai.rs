use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

pub struct AzureOpenAiProvider {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    api_version: String,
    embed_deployment: String,
}

impl AzureOpenAiProvider {
    pub fn new() -> Result<Self, CloudError> {
        let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").map_err(|_| CloudError::Auth {
            message: "AZURE_OPENAI_ENDPOINT not set".to_string(),
        })?;
        let api_key = std::env::var("AZURE_OPENAI_API_KEY").map_err(|_| CloudError::Auth {
            message: "AZURE_OPENAI_API_KEY not set".to_string(),
        })?;
        let api_version = std::env::var("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| "2024-10-21".to_owned());
        let embed_deployment = std::env::var("AZURE_OPENAI_EMBED_DEPLOYMENT").unwrap_or_default();

        Ok(Self {
            client: reqwest::Client::new(),
            endpoint,
            api_key,
            api_version,
            embed_deployment,
        })
    }

    pub fn with_http_client(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        api_version: impl Into<String>,
        embed_deployment: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            api_version: api_version.into(),
            embed_deployment: embed_deployment.into(),
        }
    }

    pub(crate) fn azure_endpoint(&self, deployment: &str, method: &str) -> String {
        format!(
            "{}/openai/deployments/{}/{}?api-version={}",
            self.endpoint.trim_end_matches('/'), deployment, method, self.api_version
        )
    }

    pub(crate) fn request(&self, url: &str) -> reqwest::RequestBuilder {
        self.client.post(url).header("api-key", self.api_key.as_str())
    }
}

pub(crate) fn extract_deployment_name(model: &ModelRef) -> Result<String, CloudError> {
    match model {
        ModelRef::Deployment(name) => Ok(name.clone()),
        ModelRef::Provider(id) => Ok(id.clone()),
        ModelRef::Logical { .. } => Err(CloudError::Unsupported {
            feature: "Azure OpenAI model resolution from ModelRef::Logical",
        }),
    }
}

pub(crate) fn is_reasoning_model(deployment: &str) -> bool {
    let d = deployment.to_ascii_lowercase();
    d.starts_with("o1") || d.starts_with("o3")
}

pub(crate) fn build_chat_request(req: &LlmRequest, deployment: &str) -> serde_json::Value {
    let mut messages = Vec::with_capacity(req.messages.len() + 1);
    if let Some(system) = &req.system_prompt {
        messages.push(serde_json::json!({ "role": "system", "content": system }));
    }
    for msg in &req.messages {
        messages.push(serde_json::json!({ "role": msg.role, "content": msg.content }));
    }

    let mut body = serde_json::json!({ "messages": messages });
    let reasoning = is_reasoning_model(deployment);

    if let Some(max_tokens) = req.max_tokens {
        let key = if reasoning { "max_completion_tokens" } else { "max_tokens" };
        body[key] = serde_json::json!(max_tokens);
    }
    if !reasoning {
        if let Some(temperature) = req.temperature {
            body["temperature"] = serde_json::json!(temperature);
        }
    }

    body
}

pub(crate) fn build_embed_request(texts: &[String]) -> serde_json::Value {
    serde_json::json!({ "input": texts })
}

pub(crate) fn build_tool_request(
    req: &LlmRequest,
    deployment: &str,
    tools: &[ToolDefinition],
) -> serde_json::Value {
    let mut body = build_chat_request(req, deployment);

    if !tools.is_empty() {
        let functions: Vec<serde_json::Value> = tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            })
            .collect();

        body["tools"] = serde_json::json!(functions);
        body["tool_choice"] = serde_json::json!("auto");
        body["parallel_tool_calls"] = serde_json::json!(false);
    }

    body
}

pub(crate) fn retry_after_seconds(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?.parse().ok()
}

pub(crate) fn parse_azure_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v["error"]["message"].as_str().map(str::to_owned))
        .unwrap_or_else(|| body.trim().to_owned())
}

pub(crate) fn map_azure_http_error(status: u16, body: &str, retry_after: Option<u64>) -> CloudError {
    let message = parse_azure_error_message(body);
    match status {
        401 | 403 => CloudError::Auth { message },
        429 => CloudError::RateLimit { retry_after },
        400 => CloudError::Provider { http_status: 400, message, retryable: false },
        _ => CloudError::Provider { http_status: status, message, retryable: status >= 500 },
    }
}

pub(crate) fn map_finish_reason(s: &str) -> FinishReason {
    match s {
        "stop" => FinishReason::Stop,
        "length" => FinishReason::Length,
        "tool_calls" => FinishReason::ToolCall,
        other => FinishReason::Other(other.to_string()),
    }
}

pub(crate) fn parse_chat_response(json: &serde_json::Value) -> Result<LlmResponse, CloudError> {
    let choice = json["choices"].get(0).ok_or_else(|| CloudError::Provider {
        http_status: 0,
        message: "parse error: response contained no choices".to_string(),
        retryable: false,
    })?;

    let text = choice["message"]["content"].as_str().unwrap_or("").to_string();

    let finish_reason = choice["finish_reason"]
        .as_str()
        .map(map_finish_reason)
        .unwrap_or(FinishReason::Other("unknown".to_string()));

    let usage = match (
        json["usage"]["prompt_tokens"].as_u64(),
        json["usage"]["completion_tokens"].as_u64(),
    ) {
        (Some(p), Some(c)) => Some(UsageStats {
            prompt_tokens: p as u32,
            completion_tokens: c as u32,
        }),
        _ => None,
    };

    Ok(LlmResponse { text, finish_reason, usage })
}

pub(crate) fn parse_embed_response(json: &serde_json::Value) -> Result<EmbedResponse, CloudError> {
    let data = json["data"].as_array().ok_or_else(|| CloudError::Provider {
        http_status: 0,
        message: "parse error: response contained no data".to_string(),
        retryable: false,
    })?;

    let mut embeddings: Vec<Option<Vec<f32>>> = vec![None; data.len()];
    for entry in data {
        let index = entry["index"].as_u64().ok_or_else(|| CloudError::Provider {
            http_status: 0,
            message: "parse error: embedding entry missing index".to_string(),
            retryable: false,
        })? as usize;
        let values = entry["embedding"].as_array().ok_or_else(|| CloudError::Provider {
            http_status: 0,
            message: "parse error: malformed embedding in response".to_string(),
            retryable: false,
        })?;
        let slot = embeddings.get_mut(index).ok_or_else(|| CloudError::Provider {
            http_status: 0,
            message: "parse error: embedding index out of range".to_string(),
            retryable: false,
        })?;
        if slot.is_some() {
            return Err(CloudError::Provider {
                http_status: 0,
                message: "parse error: duplicate embedding index".to_string(),
                retryable: false,
            });
        }
        *slot = Some(values.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect());
    }

    let embeddings = embeddings.into_iter().map(Option::unwrap_or_default).collect();
    Ok(EmbedResponse { embeddings })
}

pub(crate) fn parse_tool_response(json: &serde_json::Value) -> Result<ToolCallResponse, CloudError> {
    let call = json["choices"].get(0).and_then(|c| c["message"]["tool_calls"].get(0));

    let Some(call) = call else {
        return parse_chat_response(json).map(ToolCallResponse::Text);
    };

    let name = call["function"]["name"].as_str().unwrap_or("").to_string();
    let raw_args = call["function"]["arguments"].as_str().unwrap_or("");
    let arguments = serde_json::from_str(raw_args).map_err(|e| CloudError::Provider {
        http_status: 0,
        message: format!("parse error: tool call arguments were not valid JSON: {e}"),
        retryable: false,
    })?;

    Ok(ToolCallResponse::ToolCall { name, arguments })
}

pub(crate) fn parse_sse_line(line: &str) -> Option<serde_json::Value> {
    let data = line.strip_prefix("data: ")?;
    if data == "[DONE]" {
        return None;
    }
    serde_json::from_str(data).ok()
}

pub(crate) fn drain_complete_lines(buffer: &mut Vec<u8>) -> Vec<serde_json::Value> {
    let mut parsed = Vec::new();
    while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
        let line_bytes: Vec<u8> = buffer.drain(..=pos).collect();
        let line = String::from_utf8_lossy(&line_bytes);
        let line = line.trim_end_matches('\n').trim_end_matches('\r');
        if let Some(json) = parse_sse_line(line) {
            parsed.push(json);
        }
    }
    parsed
}

pub(crate) fn sse_chunk_to_events(json: &serde_json::Value) -> Vec<LlmStreamEvent> {
    let mut events = Vec::new();

    if let Some(error) = json.get("error") {
        let message = error["message"]
            .as_str()
            .unwrap_or("azure returned an error with no message")
            .to_string();
        events.push(LlmStreamEvent::Error(CloudError::Provider {
            http_status: 200,
            message,
            retryable: false,
        }));
        return events;
    }

    let choice = json["choices"].get(0);

    if let Some(text) = choice.and_then(|c| c["delta"]["content"].as_str()) {
        if !text.is_empty() {
            events.push(LlmStreamEvent::DeltaText(text.to_string()));
        }
    }

    if let (Some(p), Some(c)) = (
        json["usage"]["prompt_tokens"].as_u64(),
        json["usage"]["completion_tokens"].as_u64(),
    ) {
        events.push(LlmStreamEvent::Usage(UsageStats {
            prompt_tokens: p as u32,
            completion_tokens: c as u32,
        }));
    }

    if let Some(reason) = choice.and_then(|c| c["finish_reason"].as_str()) {
        events.push(LlmStreamEvent::Done(map_finish_reason(reason)));
    }

    events
}

pub(crate) async fn emit_chunk_events(
    events: Vec<LlmStreamEvent>,
    pending_done: &mut Option<LlmStreamEvent>,
    tx: &mut mpsc::Sender<LlmStreamEvent>,
) -> bool {
    let has_usage = events.iter().any(|e| matches!(e, LlmStreamEvent::Usage(_)));

    for event in events {
        match event {
            LlmStreamEvent::Done(_) => {
                if let Some(previous) = pending_done.take() {
                    if tx.send(previous).await.is_err() {
                        return false;
                    }
                }
                *pending_done = Some(event);
            }
            other => {
                if tx.send(other).await.is_err() {
                    return false;
                }
            }
        }
    }

    if has_usage {
        if let Some(done) = pending_done.take() {
            if tx.send(done).await.is_err() {
                return false;
            }
        }
    }

    true
}

pub(crate) async fn provider_error_from_response(response: reqwest::Response) -> CloudError {
    let status = response.status().as_u16();
    let retry_after = retry_after_seconds(response.headers());
    let text = response.text().await.unwrap_or_default();
    map_azure_http_error(status, &text, retry_after)
}

pub(crate) async fn pump_stream(mut response: reqwest::Response, mut tx: mpsc::Sender<LlmStreamEvent>) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut pending_done: Option<LlmStreamEvent> = None;

    loop {
        match response.chunk().await {
            Ok(Some(bytes)) => {
                buffer.extend_from_slice(&bytes);
                for json in drain_complete_lines(&mut buffer) {
                    let events = sse_chunk_to_events(&json);
                    if !emit_chunk_events(events, &mut pending_done, &mut tx).await {
                        return;
                    }
                }
            }
            Ok(None) => {
                if !buffer.is_empty() {
                    buffer.push(b'\n');
                    for json in drain_complete_lines(&mut buffer) {
                        let events = sse_chunk_to_events(&json);
                        if !emit_chunk_events(events, &mut pending_done, &mut tx).await {
                            return;
                        }
                    }
                }
                if let Some(done) = pending_done.take() {
                    let _ = tx.send(done).await;
                }
                break;
            }
            Err(e) => {
                if let Some(done) = pending_done.take() {
                    let _ = tx.send(done).await;
                } else {
                    let _ = tx.send(LlmStreamEvent::Error(CloudError::Network { source: e })).await;
                }
                break;
            }
        }
    }
}

#[async_trait]
impl LlmProvider for AzureOpenAiProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let deployment = extract_deployment_name(&req.model)?;
        let url = self.azure_endpoint(&deployment, "chat/completions");
        let body = build_chat_request(&req, &deployment);

        let response = self
            .request(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if response.status().as_u16() >= 400 {
            return Err(provider_error_from_response(response).await);
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let resp_json: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| CloudError::Serialization { source: e })?;

        parse_chat_response(&resp_json)
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let deployment = extract_deployment_name(&req.model)?;
        let url = self.azure_endpoint(&deployment, "chat/completions");
        let mut body = build_chat_request(&req, &deployment);
        body["stream"] = serde_json::json!(true);
        body["stream_options"] = serde_json::json!({ "include_usage": true });

        let response = self
            .request(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if response.status().as_u16() >= 400 {
            return Err(provider_error_from_response(response).await);
        }

        let (tx, rx) = mpsc::channel::<LlmStreamEvent>(32);
        tokio::spawn(pump_stream(response, tx));

        Ok(Box::pin(rx))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        if texts.is_empty() {
            return Ok(EmbedResponse { embeddings: vec![] });
        }

        let url = self.azure_endpoint(&self.embed_deployment, "embeddings");
        let body = build_embed_request(&texts);

        let response = self
            .request(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if response.status().as_u16() >= 400 {
            return Err(provider_error_from_response(response).await);
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
        if tools.is_empty() {
            return Err(CloudError::Unsupported {
                feature: "generate_with_tools requires at least one tool",
            });
        }

        let deployment = extract_deployment_name(&req.model)?;
        let url = self.azure_endpoint(&deployment, "chat/completions");
        let body = build_tool_request(&req, &deployment, &tools);

        let response = self
            .request(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if response.status().as_u16() >= 400 {
            return Err(provider_error_from_response(response).await);
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

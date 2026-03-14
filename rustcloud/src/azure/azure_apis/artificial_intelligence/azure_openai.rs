use async_trait::async_trait;
use futures::stream;
use serde_json::{json, Value};
use std::env;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent,
    ModelRef, ToolCallResponse, ToolDefinition, UsageStats,
};

pub struct AzureOpenAIProvider {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    api_version: String,
    embed_deployment: String,
}

impl AzureOpenAIProvider {
    pub fn new() -> Self {
        let endpoint = env::var("AZURE_OPENAI_ENDPOINT")
            .expect("AZURE_OPENAI_ENDPOINT not set");
        let api_key = env::var("AZURE_OPENAI_API_KEY")
            .expect("AZURE_OPENAI_API_KEY not set");
        let api_version = env::var("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| "2024-10-21".to_string());
        let embed_deployment = env::var("AZURE_OPENAI_EMBED_DEPLOYMENT")
            .unwrap_or_else(|_| "text-embedding-ada-002".to_string());

        Self {
            client: reqwest::Client::new(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
            api_key,
            api_version,
            embed_deployment,
        }
    }

    pub fn with_config(
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        api_version: impl Into<String>,
        embed_deployment: impl Into<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        Self {
            client: reqwest::Client::new(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            api_version: api_version.into(),
            embed_deployment: embed_deployment.into(),
        }
    }

    fn chat_url(&self, deployment_id: &str) -> String {
        format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, deployment_id, self.api_version
        )
    }

    fn embed_url(&self) -> String {
        format!(
            "{}/openai/deployments/{}/embeddings?api-version={}",
            self.endpoint, self.embed_deployment, self.api_version
        )
    }
}

fn extract_deployment(model_ref: &ModelRef) -> Result<String, CloudError> {
    match model_ref {
        ModelRef::Deployment(id) | ModelRef::Provider(id) => Ok(id.clone()),
        ModelRef::Logical { .. } => Err(CloudError::Unsupported {
            feature: "ModelRef::Logical is not supported for Azure OpenAI; use Deployment or Provider",
        }),
    }
}

fn build_messages_json(req: &LlmRequest) -> Value {
    let mut messages = Vec::new();
    if let Some(system) = &req.system_prompt {
        messages.push(json!({"role": "system", "content": system}));
    }
    for msg in &req.messages {
        messages.push(json!({"role": msg.role, "content": msg.content}));
    }
    json!(messages)
}

fn map_finish_reason(reason: Option<&str>) -> FinishReason {
    match reason {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCall,
        Some(other) => FinishReason::Other(other.to_string()),
        None => FinishReason::Other("unknown".to_string()),
    }
}

fn parse_response_json(bytes: &[u8]) -> Result<Value, CloudError> {
    serde_json::from_slice(bytes).map_err(|e| CloudError::Serialization { source: e })
}

fn extract_usage(data: &Value) -> Option<UsageStats> {
    if data["usage"].is_object() {
        Some(UsageStats {
            prompt_tokens: data["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: data["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
        })
    } else {
        None
    }
}

#[async_trait]
impl LlmProvider for AzureOpenAIProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let deployment_id = extract_deployment(&req.model)?;
        let url = self.chat_url(&deployment_id);

        let mut body = json!({
            "messages": build_messages_json(&req),
        });
        if let Some(max_tokens) = req.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        if let Some(temp) = req.temperature {
            body["temperature"] = json!(temp);
        }

        let body_bytes = serde_json::to_vec(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .body(body_bytes)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let resp_bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;
        let data = parse_response_json(&resp_bytes)?;

        if status != 200 {
            let message = data["error"]["message"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            return Err(CloudError::Provider {
                http_status: status,
                message,
                retryable: status == 429 || status >= 500,
            });
        }

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let finish_reason =
            map_finish_reason(data["choices"][0]["finish_reason"].as_str());
        let usage = extract_usage(&data);

        println!("Generated text: {:?}", text);
        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let deployment_id = extract_deployment(&req.model)?;
        let url = self.chat_url(&deployment_id);

        let mut body = json!({
            "messages": build_messages_json(&req),
            "stream": true,
            "stream_options": { "include_usage": true },
        });
        if let Some(max_tokens) = req.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        if let Some(temp) = req.temperature {
            body["temperature"] = json!(temp);
        }

        let body_bytes = serde_json::to_vec(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .body(body_bytes)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        if status != 200 {
            let err_bytes = response
                .bytes()
                .await
                .map_err(|e| CloudError::Network { source: e })?;
            let err_text = String::from_utf8_lossy(&err_bytes).to_string();
            return Err(CloudError::Provider {
                http_status: status,
                message: err_text,
                retryable: status == 429 || status >= 500,
            });
        }

        let body_text = response
            .text()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let events: Vec<LlmStreamEvent> = body_text
            .lines()
            .filter_map(|line| {
                let data = line.strip_prefix("data: ")?;
                if data == "[DONE]" {
                    return None;
                }
                let chunk: Value = serde_json::from_str(data).ok()?;

                let content = chunk["choices"][0]["delta"]["content"].as_str();
                let finish = chunk["choices"][0]["finish_reason"].as_str();

                if let Some(text) = content {
                    if !text.is_empty() {
                        return Some(LlmStreamEvent::DeltaText(text.to_string()));
                    }
                }
                if let Some(reason) = finish {
                    if !reason.is_empty() {
                        return Some(LlmStreamEvent::Done(map_finish_reason(Some(reason))));
                    }
                }
                if chunk["usage"].is_object() {
                    return Some(LlmStreamEvent::Usage(UsageStats {
                        prompt_tokens: chunk["usage"]["prompt_tokens"]
                            .as_u64()
                            .unwrap_or(0) as u32,
                        completion_tokens: chunk["usage"]["completion_tokens"]
                            .as_u64()
                            .unwrap_or(0) as u32,
                    }));
                }
                None
            })
            .collect();

        Ok(Box::pin(stream::iter(events)))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let url = self.embed_url();
        let body = json!({ "input": texts });

        let body_bytes = serde_json::to_vec(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .body(body_bytes)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let resp_bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;
        let data = parse_response_json(&resp_bytes)?;

        if status != 200 {
            let message = data["error"]["message"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            return Err(CloudError::Provider {
                http_status: status,
                message,
                retryable: status == 429 || status >= 500,
            });
        }

        let data_arr = data["data"].as_array().ok_or(CloudError::Provider {
            http_status: 200,
            message: "missing 'data' field in embeddings response".to_string(),
            retryable: false,
        })?;

        // Azure may return embeddings out of order — sort by index field
        let mut indexed: Vec<(usize, Vec<f32>)> = data_arr
            .iter()
            .map(|item| {
                let index = item["index"].as_u64().unwrap_or(0) as usize;
                let embedding: Vec<f32> = item["embedding"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                    .collect();
                (index, embedding)
            })
            .collect();

        indexed.sort_by_key(|(i, _)| *i);
        let embeddings = indexed.into_iter().map(|(_, e)| e).collect();

        println!("Generated {} embeddings", texts.len());
        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let deployment_id = extract_deployment(&req.model)?;
        let url = self.chat_url(&deployment_id);

        let tools_json: Vec<Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            })
            .collect();

        let mut body = json!({
            "messages": build_messages_json(&req),
            "tools": tools_json,
            "tool_choice": "auto",
        });
        if let Some(max_tokens) = req.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        if let Some(temp) = req.temperature {
            body["temperature"] = json!(temp);
        }

        let body_bytes = serde_json::to_vec(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .body(body_bytes)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let resp_bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;
        let data = parse_response_json(&resp_bytes)?;

        if status != 200 {
            let message = data["error"]["message"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            return Err(CloudError::Provider {
                http_status: status,
                message,
                retryable: status == 429 || status >= 500,
            });
        }

        let finish_reason = map_finish_reason(
            data["choices"][0]["finish_reason"].as_str(),
        );

        if let Some(tool_calls) =
            data["choices"][0]["message"]["tool_calls"].as_array()
        {
            if let Some(call) = tool_calls.first() {
                let name = call["function"]["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                let arguments: Value = serde_json::from_str(
                    call["function"]["arguments"].as_str().unwrap_or("{}"),
                )
                .unwrap_or(json!({}));

                println!("Tool called: {}", name);
                return Ok(ToolCallResponse::ToolCall { name, arguments });
            }
        }

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let usage = extract_usage(&data);

        Ok(ToolCallResponse::Text(LlmResponse {
            text,
            finish_reason,
            usage,
        }))
    }
}

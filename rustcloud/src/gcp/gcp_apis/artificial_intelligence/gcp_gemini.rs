use async_trait::async_trait;
use futures::channel::mpsc;
use futures::SinkExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse,
    ToolDefinition, UsageStats,
};

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const EMBED_MODEL: &str = "gemini-embedding-001";

// ── request types ────────────────────────────────────────────────────────────

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

// ── response types ────────────────────────────────────────────────────────────

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

// ── embed types ───────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct EmbedPart {
    text: String,
}

#[derive(Serialize)]
struct EmbedContent {
    parts: Vec<EmbedPart>,
}

#[derive(Serialize)]
struct EmbedContentRequest {
    model: String,
    content: EmbedContent,
}

#[derive(Serialize)]
struct BatchEmbedRequest {
    requests: Vec<EmbedContentRequest>,
}

#[derive(Deserialize)]
struct EmbedValues {
    values: Vec<f32>,
}

#[derive(Deserialize)]
struct BatchEmbedResponse {
    embeddings: Vec<EmbedValues>,
}

// ── adapter ───────────────────────────────────────────────────────────────────

pub struct GeminiAdapter {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiAdapter {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
        }
    }

    fn generate_url(&self) -> String {
        format!("{}/models/{}:generateContent", BASE_URL, self.model)
    }

    fn stream_url(&self) -> String {
        format!(
            "{}/models/{}:streamGenerateContent?alt=sse",
            BASE_URL, self.model
        )
    }

    fn embed_url(&self) -> String {
        format!("{}/models/{}:batchEmbedContents", BASE_URL, EMBED_MODEL)
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
impl LlmProvider for GeminiAdapter {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let body = self.build_request(&req, None);

        let response = self
            .client
            .post(self.generate_url())
            .header("x-goog-api-key", &self.api_key)
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

        let response = self
            .client
            .post(self.stream_url())
            .header("x-goog-api-key", &self.api_key)
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

        let bytes = response
            .bytes()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let text = String::from_utf8_lossy(&bytes).to_string();

        tokio::spawn(async move {
            for line in text.lines() {
                let line = line.trim();
                if line.starts_with("data: ") {
                    let json_str = &line[6..];
                    if json_str == "[DONE]" {
                        break;
                    }
                    if let Ok(resp) = serde_json::from_str::<GenerateResponse>(json_str) {
                        for candidate in resp.candidates {
                            if let Some(content) = candidate.content {
                                for part in content.parts {
                                    if let Some(t) = part.text {
                                        tx.send(LlmStreamEvent::DeltaText(t)).await.ok();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            tx.send(LlmStreamEvent::Done(FinishReason::Stop)).await.ok();
        });

        Ok(Box::pin(rx))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let model_name = format!("models/{}", EMBED_MODEL);
        let body = BatchEmbedRequest {
            requests: texts
                .into_iter()
                .map(|t| EmbedContentRequest {
                    model: model_name.clone(),
                    content: EmbedContent {
                        parts: vec![EmbedPart { text: t }],
                    },
                })
                .collect(),
        };

        let response = self
            .client
            .post(self.embed_url())
            .header("x-goog-api-key", &self.api_key)
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

        let resp: BatchEmbedResponse = response
            .json()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        Ok(EmbedResponse {
            embeddings: resp.embeddings.into_iter().map(|e| e.values).collect(),
        })
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

        let response = self
            .client
            .post(self.generate_url())
            .header("x-goog-api-key", &self.api_key)
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

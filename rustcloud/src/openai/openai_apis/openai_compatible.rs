#![allow(dead_code)]

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, Message, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

#[derive(Debug, Clone)]
pub struct OpenAICompatible {
    client: Client,
    base_url: String,
    api_key: String,
    default_model: String,
}

impl OpenAICompatible {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            default_model: "gpt-3.5-turbo".to_string(),
        }
    }

    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    fn resolve_model(&self, model: &ModelRef) -> String {
        match model {
            ModelRef::Provider(id) => id.clone(),
            ModelRef::Logical { family, .. } => family.clone(),
            ModelRef::Deployment(name) => name.clone(),
        }
    }

    fn build_messages(messages: &[Message]) -> Vec<serde_json::Value> {
        messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content
                })
            })
            .collect()
    }

    fn map_finish_reason(reason: Option<&str>) -> FinishReason {
        match reason {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCall,
            Some("content_filter") => FinishReason::Other("content_filter".to_string()),
            None => FinishReason::Stop,
            Some(other) => FinishReason::Other(other.to_string()),
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: Option<ChatMessage>,
    #[serde(rename = "finish_reason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChatMessage {
    role: Option<String>,
    content: Option<String>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<UsageInfo>,
}

#[derive(Deserialize)]
struct UsageInfo {
    #[serde(rename = "prompt_tokens")]
    prompt_tokens: Option<u32>,
    #[serde(rename = "completion_tokens")]
    completion_tokens: Option<u32>,
}

#[derive(Serialize)]
struct EmbedRequest {
    input: Vec<String>,
    model: String,
}

#[derive(Deserialize)]
struct OpenAIEmbedResponse {
    data: Vec<EmbedData>,
}

#[derive(Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

#[async_trait]
impl LlmProvider for OpenAICompatible {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model = self.resolve_model(&req.model);
        let messages = Self::build_messages(&req.messages);

        let chat_req = ChatRequest {
            model: model.clone(),
            messages,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };

        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_req)
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

        let body = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        let chat_resp: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let choice = chat_resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| CloudError::Provider {
                http_status: 200,
                message: "no choices in response".to_string(),
                retryable: false,
            })?;

        let message = choice
            .message
            .ok_or_else(|| CloudError::Provider {
                http_status: 200,
                message: "no message in choice".to_string(),
                retryable: false,
            })?;

        let text = message.content.unwrap_or_default();
        let finish_reason = Self::map_finish_reason(choice.finish_reason.as_deref());

        let usage = chat_resp.usage.map(|u| UsageStats {
            prompt_tokens: u.prompt_tokens.unwrap_or(0),
            completion_tokens: u.completion_tokens.unwrap_or(0),
        });

        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        Err(CloudError::Unsupported {
            feature: "streaming not yet implemented for OpenAI-compatible providers",
        })
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let embed_req = EmbedRequest {
            input: texts,
            model: self.default_model.clone(),
        };

        let url = format!("{}/embeddings", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
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

        let body = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        let embed_resp: OpenAIEmbedResponse = serde_json::from_str(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        Ok(EmbedResponse {
            embeddings: embed_resp.data.into_iter().map(|d| d.embedding).collect(),
        })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model = self.resolve_model(&req.model);
        let messages = Self::build_messages(&req.messages);

        #[derive(Serialize)]
        struct Tool {
            #[serde(rename = "type")]
            typ: String,
            function: ToolFunction,
        }

        #[derive(Serialize)]
        struct ToolFunction {
            name: String,
            description: String,
            parameters: serde_json::Value,
        }

        let tool_specs: Vec<Tool> = tools
            .into_iter()
            .map(|t| Tool {
                typ: "function".to_string(),
                function: ToolFunction {
                    name: t.name,
                    description: t.description,
                    parameters: t.parameters,
                },
            })
            .collect();

        #[derive(Serialize)]
        struct ChatRequestWithTools {
            model: String,
            messages: Vec<serde_json::Value>,
            tools: Vec<Tool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_tokens: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            temperature: Option<f32>,
        }

        let chat_req = ChatRequestWithTools {
            model,
            messages,
            tools: tool_specs,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
        };

        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_req)
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

        let body = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        let chat_resp: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| CloudError::Serialization { source: e })?;

        let choice = chat_resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| CloudError::Provider {
                http_status: 200,
                message: "no choices in response".to_string(),
                retryable: false,
            })?;

        if let Some(message) = choice.message {
            if let Some(content) = message.content {
                if !content.is_empty() {
                    return Ok(ToolCallResponse::Text(LlmResponse {
                        text: content,
                        finish_reason: FinishReason::Stop,
                        usage: None,
                    }));
                }
            }
        }

        Err(CloudError::Provider {
            http_status: 200,
            message: "no tool call in response".to_string(),
            retryable: false,
        })
    }
}

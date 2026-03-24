#![allow(clippy::too_many_arguments, clippy::new_without_default, dead_code)]
use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::errors::CloudError;
use crate::traits::llm_provider::LlmProvider;
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, ModelRef, ToolCallResponse,
    ToolDefinition, UsageStats,
};

pub struct AzureOpenAI {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    api_version: String,
}

impl AzureOpenAI {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
            api_key,
            api_version: "2024-02-15-preview".to_string(),
        }
    }

    pub fn with_api_version(mut self, api_version: String) -> Self {
        self.api_version = api_version;
        self
    }

    fn resolve_deployment(&self, model: &ModelRef) -> String {
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
impl LlmProvider for AzureOpenAI {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let deployment = self.resolve_deployment(&req.model);

        let messages: Vec<serde_json::Value> = req
            .messages
            .iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
            .collect();

        let mut body = serde_json::json!({
            "messages": messages,
        });

        if let Some(max_tokens) = req.max_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(temperature) = req.temperature {
            body["temperature"] = serde_json::json!(temperature);
        }
        if let Some(system) = &req.system_prompt {
            body["system_message"] = serde_json::json!(system);
        }

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint, deployment, self.api_version
        );

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            return Err(CloudError::Provider {
                http_status: response.status().as_u16(),
                message: format!("Azure OpenAI API error: {}", response.text().await.unwrap_or_default()),
                retryable: false,
            });
        }

        let result: serde_json::Value = response.json().await.map_err(|e| CloudError::Network { source: e })?;

        let text = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = result.get("usage").map(|u| UsageStats {
            prompt_tokens: u["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: u["completion_tokens"].as_u64().unwrap_or(0) as u32,
        });

        let finish_reason = match result["choices"][0]["finish_reason"].as_str() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCall,
            _ => FinishReason::Other("unknown".to_string()),
        };

        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(
        &self,
        _req: LlmRequest,
    ) -> Result<crate::traits::llm_provider::LlmStream, CloudError> {
        Err(CloudError::Unsupported {
            feature: "streaming not yet implemented for Azure OpenAI",
        })
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let url = format!(
            "{}/openai/deployments/{}/embeddings?api-version={}",
            self.endpoint, "text-embedding-ada-002", self.api_version
        );

        let body = serde_json::json!({
            "input": texts
        });

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        if !response.status().is_success() {
            return Err(CloudError::Provider {
                http_status: response.status().as_u16(),
                message: format!("Azure OpenAI Embeddings error: {}", response.text().await.unwrap_or_default()),
                retryable: false,
            });
        }

        let result: serde_json::Value = response.json().await.map_err(|e| CloudError::Network { source: e })?;

        let embeddings: Vec<Vec<f32>> = result["data"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|item| {
                item["embedding"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                    .collect()
            })
            .collect();

        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "tools not yet implemented for Azure OpenAI",
        })
    }
}
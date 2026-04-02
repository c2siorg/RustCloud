#![allow(clippy::too_many_arguments, clippy::new_without_default, dead_code)]

use async_trait::async_trait;
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ContentBlockDelta, ConversationRole, ConverseStreamOutput,
    InferenceConfiguration, Message as BedrockMessage, SystemContentBlock, Tool,
    ToolConfiguration, ToolInputSchema, ToolSpecification,
};
use aws_sdk_bedrockruntime::Client;
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

const DEFAULT_EMBED_MODEL: &str = "amazon.titan-embed-text-v2:0";
const DEFAULT_TIMEOUT_SECS: u64 = 60;

#[derive(Serialize)]
struct TitanEmbedRequest {
    #[serde(rename = "inputText")]
    input_text: String,
}

#[derive(Deserialize)]
struct TitanEmbedResponse {
    embedding: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct AmazonBedrock {
    client: Client,
    embed_model: String,
    timeout: Duration,
}

impl AmazonBedrock {
    pub fn new(region: impl Into<String>) -> Self {
        let region_str = region.into();
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_sdk_types::region::Region::new(region_str.clone()))
            .load_sync();
        let client = Client::new(&config);
        Self {
            client,
            embed_model: DEFAULT_EMBED_MODEL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub async fn from_env() -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self {
            client,
            embed_model: DEFAULT_EMBED_MODEL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn with_embed_model(mut self, model: impl Into<String>) -> Self {
        self.embed_model = model.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn resolve_model_id(&self, model: &ModelRef) -> String {
        match model {
            ModelRef::Provider(id) => id.clone(),
            ModelRef::Logical { family, tier } => {
                let suffix = tier.as_deref().unwrap_or("001");
                format!("{}:{}", family, suffix)
            }
            ModelRef::Deployment(dep) => dep.clone(),
        }
    }

    fn build_messages(&self, req: &LlmRequest) -> Result<Vec<BedrockMessage>, CloudError> {
        req.messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| {
                let role = match m.role.as_str() {
                    "assistant" => ConversationRole::Assistant,
                    _ => ConversationRole::User,
                };
                BedrockMessage::builder()
                    .role(role)
                    .content(ContentBlock::Text(m.content.clone()))
                    .build()
                    .map_err(|e| CloudError::Provider {
                        http_status: 0,
                        message: e.to_string(),
                        retryable: false,
                    })
            })
            .collect()
    }

    fn build_inference_config(&self, req: &LlmRequest) -> InferenceConfiguration {
        let mut builder = InferenceConfiguration::builder();
        if let Some(max) = req.max_tokens {
            builder = builder.max_tokens(max.min(i32::MAX as u32) as i32);
        }
        if let Some(temp) = req.temperature {
            builder = builder.temperature(temp);
        }
        builder.build()
    }

    fn map_stop_reason(reason: &aws_sdk_bedrockruntime::types::StopReason) -> FinishReason {
        match reason.as_str() {
            "end_turn" => FinishReason::Stop,
            "max_tokens" => FinishReason::Length,
            "tool_use" => FinishReason::ToolCall,
            other => FinishReason::Other(other.to_string()),
        }
    }

    fn json_to_document(v: serde_json::Value) -> aws_smithy_types::Document {
        match v {
            serde_json::Value::Null => aws_smithy_types::Document::Null,
            serde_json::Value::Bool(b) => aws_smithy_types::Document::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    aws_smithy_types::Document::Number(aws_smithy_types::Number::PosInt(u))
                } else if let Some(i) = n.as_i64() {
                    aws_smithy_types::Document::Number(aws_smithy_types::Number::NegInt(i))
                } else {
                    aws_smithy_types::Document::Number(aws_smithy_types::Number::Float(
                        n.as_f64().unwrap_or(0.0),
                    ))
                }
            }
            serde_json::Value::String(s) => aws_smithy_types::Document::String(s),
            serde_json::Value::Array(arr) => {
                aws_smithy_types::Document::Array(arr.into_iter().map(Self::json_to_document).collect())
            }
            serde_json::Value::Object(obj) => aws_smithy_types::Document::Object(
                obj.into_iter()
                    .map(|(k, v)| (k, Self::json_to_document(v)))
                    .collect(),
            ),
        }
    }

    fn document_to_json(doc: aws_smithy_types::Document) -> serde_json::Value {
        match doc {
            aws_smithy_types::Document::Null => serde_json::Value::Null,
            aws_smithy_types::Document::Bool(b) => serde_json::Value::Bool(b),
            aws_smithy_types::Document::Number(n) => match n {
                aws_smithy_types::Number::PosInt(u) => serde_json::json!(u),
                aws_smithy_types::Number::NegInt(i) => serde_json::json!(i),
                aws_smithy_types::Number::Float(f) => serde_json::json!(f),
            },
            aws_smithy_types::Document::String(s) => serde_json::Value::String(s),
            aws_smithy_types::Document::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(Self::document_to_json).collect())
            }
            aws_smithy_types::Document::Object(obj) => serde_json::Value::Object(
                obj.into_iter()
                    .map(|(k, v)| (k, Self::document_to_json(v)))
                    .collect(),
            ),
        }
    }
}

#[async_trait]
impl LlmProvider for AmazonBedrock {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        let messages = self.build_messages(&req)?;
        let inference_config = self.build_inference_config(&req);

        let mut builder = self
            .client
            .converse()
            .model_id(&model_id)
            .set_messages(Some(messages))
            .inference_config(inference_config);

        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }

        let response = tokio::time::timeout(
            self.timeout,
            builder.send(),
        )
        .await
        .map_err(|_| CloudError::Provider {
            http_status: 0,
            message: "request timed out".to_string(),
            retryable: true,
        })?
        .map_err(|e| CloudError::Provider {
            http_status: 0,
            message: e.to_string(),
            retryable: false,
        })?;

        let finish_reason = self.map_stop_reason(response.stop_reason());

        let usage = response.usage().map(|u| UsageStats {
            prompt_tokens: u.input_tokens().max(0),
            completion_tokens: u.output_tokens().max(0),
        });

        let text = response
            .output()
            .and_then(|o| {
                if let aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg) = o {
                    msg.content()
                        .iter()
                        .find_map(|b| {
                            if let ContentBlock::Text(t) = b {
                                Some(t.clone())
                            } else {
                                None
                            }
                        })
                } else {
                    None
                }
            })
            .unwrap_or_default();

        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        let messages = self.build_messages(&req)?;
        let inference_config = self.build_inference_config(&req);

        let mut builder = self
            .client
            .converse_stream()
            .model_id(&model_id)
            .set_messages(Some(messages))
            .inference_config(inference_config);

        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }

        let output = tokio::time::timeout(
            self.timeout,
            builder.send(),
        )
        .await
        .map_err(|_| CloudError::Provider {
            http_status: 0,
            message: "stream request timed out".to_string(),
            retryable: true,
        })?
        .map_err(|e| CloudError::Provider {
            http_status: 0,
            message: e.to_string(),
            retryable: false,
        })?;

        let (mut tx, rx) = mpsc::unbounded::<LlmStreamEvent>();
        let timeout = self.timeout;

        tokio::spawn(async move {
            let mut stream = output.stream;
            let start = std::time::Instant::now();
            loop {
                if start.elapsed() > timeout {
                    tx.send(LlmStreamEvent::Error(CloudError::Provider {
                        http_status: 0,
                        message: "stream timed out".to_string(),
                        retryable: true,
                    }))
                    .await
                    .ok();
                    break;
                }

                match tokio::time::timeout(Duration::from_secs(30), stream.recv()).await {
                    Ok(Ok(Some(event))) => match event {
                        ConverseStreamOutput::ContentBlockDelta(e) => {
                            if let Some(ContentBlockDelta::Text(text)) = e.delta() {
                                tx.send(LlmStreamEvent::DeltaText(text.clone())).await.ok();
                            }
                        }
                        ConverseStreamOutput::MessageStop(evt) => {
                            let reason = evt
                                .stop_reason()
                                .map(|r| self.map_stop_reason(r))
                                .unwrap_or(FinishReason::Stop);
                            tx.send(LlmStreamEvent::Done(reason)).await.ok();
                            break;
                        }
                        _ => {}
                    },
                    Ok(Ok(None)) => {
                        tx.send(LlmStreamEvent::Done(FinishReason::Stop)).await.ok();
                        break;
                    }
                    Ok(Err(e)) => {
                        tx.send(LlmStreamEvent::Error(CloudError::Provider {
                            http_status: 0,
                            message: e.to_string(),
                            retryable: false,
                        }))
                        .await
                        .ok();
                        break;
                    }
                    Err(_) => {
                        tx.send(LlmStreamEvent::Error(CloudError::Provider {
                            http_status: 0,
                            message: "stream receive timeout".to_string(),
                            retryable: true,
                        }))
                        .await
                        .ok();
                        break;
                    }
                }
            }
        });

        Ok(Box::pin(rx))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let mut embeddings = Vec::with_capacity(texts.len());

        let bodies: Vec<_> = texts
            .iter()
            .map(|text| {
                let body = TitanEmbedRequest {
                    input_text: text.clone(),
                };
                Blob::new(
                    serde_json::to_vec(&body)
                        .map_err(|e| CloudError::Serialization { source: e })?,
                )
            })
            .collect();

        let futures: Vec<_> = bodies
            .into_iter()
            .map(|body| {
                let client = &self.client;
                async move {
                    let response = tokio::time::timeout(
                        self.timeout,
                        client
                            .invoke_model()
                            .model_id(&self.embed_model)
                            .content_type("application/json")
                            .body(body)
                            .send(),
                    )
                    .await
                    .map_err(|_| CloudError::Provider {
                        http_status: 0,
                        message: "embed request timed out".to_string(),
                        retryable: true,
                    })?
                    .map_err(|e| CloudError::Provider {
                        http_status: 0,
                        message: e.to_string(),
                        retryable: false,
                    })?;

                    let embed_resp: TitanEmbedResponse =
                        serde_json::from_slice(response.body().as_ref())
                            .map_err(|e| CloudError::Serialization { source: e })?;
                    Result::<_, CloudError>::Ok(embed_resp.embedding)
                }
            })
            .collect();

        let results = futures::future::join_all(futures).await;

        for result in results {
            embeddings.push(result?);
        }

        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model_id = self.resolve_model_id(&req.model);
        let messages = self.build_messages(&req)?;
        let inference_config = self.build_inference_config(&req);

        let bedrock_tools: Vec<Tool> = tools
            .into_iter()
            .map(|t| {
                Ok(Tool::ToolSpec(
                    ToolSpecification::builder()
                        .name(t.name)
                        .description(t.description)
                        .input_schema(ToolInputSchema::Json(Self::json_to_document(t.parameters)))
                        .build()
                        .map_err(|e| CloudError::Provider {
                            http_status: 0,
                            message: format!("invalid tool spec: {e}"),
                            retryable: false,
                        })?,
                ))
            })
            .collect::<Result<Vec<Tool>, CloudError>>()?;

        let tool_config = ToolConfiguration::builder()
            .set_tools(Some(bedrock_tools))
            .build()
            .map_err(|e| CloudError::Provider {
                http_status: 0,
                message: e.to_string(),
                retryable: false,
            })?;

        let mut builder = self
            .client
            .converse()
            .model_id(&model_id)
            .set_messages(Some(messages))
            .inference_config(inference_config)
            .tool_config(tool_config);

        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }

        let response = tokio::time::timeout(
            self.timeout,
            builder.send(),
        )
        .await
        .map_err(|_| CloudError::Provider {
            http_status: 0,
            message: "tool request timed out".to_string(),
            retryable: true,
        })?
        .map_err(|e| CloudError::Provider {
            http_status: 0,
            message: e.to_string(),
            retryable: false,
        })?;

        if let Some(aws_sdk_bedrockruntime::types::ConverseOutput::Message(msg)) =
            response.output()
        {
            for block in msg.content() {
                match block {
                    ContentBlock::ToolUse(tool_use) => {
                        let arguments = Self::document_to_json(tool_use.input().clone());
                        return Ok(ToolCallResponse::ToolCall {
                            name: tool_use.name().to_string(),
                            arguments,
                        });
                    }
                    ContentBlock::Text(text) => {
                        return Ok(ToolCallResponse::Text(LlmResponse {
                            text: text.clone(),
                            finish_reason: FinishReason::Stop,
                            usage: None,
                        }));
                    }
                    _ => {}
                }
            }
        }

        Err(CloudError::Provider {
            http_status: 0,
            message: "empty response".to_string(),
            retryable: false,
        })
    }
}
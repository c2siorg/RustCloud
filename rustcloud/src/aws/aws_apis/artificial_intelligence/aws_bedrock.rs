#![allow(clippy::result_large_err)]

use async_trait::async_trait;
use aws_sdk_bedrockruntime::{
    primitives::Blob,
    types::{
        ContentBlock, ContentBlockDelta, ConversationRole, ConverseOutput,
        InferenceConfiguration, Message as BedrockMessage, StopReason,
        SystemContentBlock, Tool, ToolInputSchema, ToolSpecification,
    },
    Client,
};
use aws_smithy_types::Document;
use futures::stream;
use serde_json::{json, Value};

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent,
    ModelRef, ToolCallResponse, ToolDefinition, UsageStats,
};

pub struct BedrockProvider {
    client: Client,
}

impl BedrockProvider {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        Self {
            client: Client::new(&config),
        }
    }

    pub fn with_client(client: Client) -> Self {
        Self { client }
    }
}

fn extract_model_id(model_ref: &ModelRef) -> String {
    match model_ref {
        ModelRef::Provider(id) => id.clone(),
        ModelRef::Logical { family, tier } => match tier.as_deref() {
            Some(t) => format!("{}.{}", family, t),
            None => family.clone(),
        },
        ModelRef::Deployment(id) => id.clone(),
    }
}

fn map_stop_reason(reason: Option<&StopReason>) -> FinishReason {
    match reason {
        Some(StopReason::EndTurn) => FinishReason::Stop,
        Some(StopReason::MaxTokens) => FinishReason::Length,
        Some(StopReason::ToolUse) => FinishReason::ToolCall,
        Some(other) => FinishReason::Other(other.as_str().to_string()),
        None => FinishReason::Other("unknown".to_string()),
    }
}

fn build_messages(req: &LlmRequest) -> Result<Vec<BedrockMessage>, CloudError> {
    req.messages
        .iter()
        .map(|msg| {
            let role = match msg.role.as_str() {
                "assistant" => ConversationRole::Assistant,
                _ => ConversationRole::User,
            };
            BedrockMessage::builder()
                .role(role)
                .content(ContentBlock::Text(msg.content.clone()))
                .build()
                .map_err(|e| CloudError::Provider {
                    http_status: 0,
                    message: e.to_string(),
                    retryable: false,
                })
        })
        .collect()
}

fn build_inference_config(req: &LlmRequest) -> InferenceConfiguration {
    let mut builder = InferenceConfiguration::builder();
    if let Some(max_tokens) = req.max_tokens {
        builder = builder.max_tokens(max_tokens as i32);
    }
    if let Some(temp) = req.temperature {
        builder = builder.temperature(temp);
    }
    builder.build()
}

fn json_to_document(value: Value) -> Document {
    match value {
        Value::Null => Document::Null,
        Value::Bool(b) => Document::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                Document::Number(aws_smithy_types::Number::PosInt(i))
            } else if let Some(i) = n.as_i64() {
                Document::Number(aws_smithy_types::Number::NegInt(i))
            } else {
                Document::Number(aws_smithy_types::Number::Float(
                    n.as_f64().unwrap_or(0.0),
                ))
            }
        }
        Value::String(s) => Document::String(s),
        Value::Array(arr) => {
            Document::Array(arr.into_iter().map(json_to_document).collect())
        }
        Value::Object(obj) => Document::Object(
            obj.into_iter()
                .map(|(k, v)| (k, json_to_document(v)))
                .collect(),
        ),
    }
}

#[async_trait]
impl LlmProvider for BedrockProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = extract_model_id(&req.model);
        let messages = build_messages(&req)?;
        let inference_config = build_inference_config(&req);

        let mut builder = self.client.converse().model_id(&model_id);
        for msg in messages {
            builder = builder.messages(msg);
        }
        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }
        builder = builder.inference_config(inference_config);

        let response = builder
            .send()
            .await
            .map_err(|e| CloudError::Provider {
                http_status: 0,
                message: e.to_string(),
                retryable: false,
            })?;

        let text = match response.output() {
            Some(ConverseOutput::Message(msg)) => msg
                .content()
                .iter()
                .find_map(|block| {
                    if let ContentBlock::Text(t) = block {
                        Some(t.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
            _ => String::new(),
        };

        let finish_reason = map_stop_reason(response.stop_reason());

        let usage = response.usage().map(|u| UsageStats {
            prompt_tokens: u.input_tokens() as u32,
            completion_tokens: u.output_tokens() as u32,
        });

        println!("Generated text: {:?}", text);
        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_id = extract_model_id(&req.model);
        let messages = build_messages(&req)?;
        let inference_config = build_inference_config(&req);

        let mut builder = self.client.converse_stream().model_id(&model_id);
        for msg in messages {
            builder = builder.messages(msg);
        }
        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }
        builder = builder.inference_config(inference_config);

        let output = builder
            .send()
            .await
            .map_err(|e| CloudError::Provider {
                http_status: 0,
                message: e.to_string(),
                retryable: false,
            })?;

        let event_stream = output.stream;
        let events: Vec<LlmStreamEvent> = event_stream
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|event| match event {
                Ok(e) => {
                    use aws_sdk_bedrockruntime::types::ConverseStreamOutput;
                    match e {
                        ConverseStreamOutput::ContentBlockDelta(delta) => {
                            if let Some(ContentBlockDelta::Text(text)) = delta.delta() {
                                Some(LlmStreamEvent::DeltaText(text.clone()))
                            } else {
                                None
                            }
                        }
                        ConverseStreamOutput::MessageStop(stop) => {
                            let reason = match stop.stop_reason() {
                                StopReason::EndTurn => FinishReason::Stop,
                                StopReason::MaxTokens => FinishReason::Length,
                                StopReason::ToolUse => FinishReason::ToolCall,
                                other => FinishReason::Other(other.as_str().to_string()),
                            };
                            Some(LlmStreamEvent::Done(reason))
                        }
                        ConverseStreamOutput::Metadata(meta) => meta.usage().map(|u| {
                            LlmStreamEvent::Usage(UsageStats {
                                prompt_tokens: u.input_tokens() as u32,
                                completion_tokens: u.output_tokens() as u32,
                            })
                        }),
                        _ => None,
                    }
                }
                Err(e) => Some(LlmStreamEvent::Error(CloudError::Provider {
                    http_status: 0,
                    message: e.to_string(),
                    retryable: true,
                })),
            })
            .collect();

        Ok(Box::pin(stream::iter(events)))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let mut embeddings = Vec::new();

        for text in &texts {
            let body = json!({ "inputText": text });
            let blob = Blob::new(body.to_string().into_bytes());

            let response = self
                .client
                .invoke_model()
                .model_id("amazon.titan-embed-text-v2:0")
                .body(blob)
                .content_type("application/json")
                .send()
                .await
                .map_err(|e| CloudError::Provider {
                    http_status: 0,
                    message: e.to_string(),
                    retryable: false,
                })?;

            let response_body: Value =
                serde_json::from_slice(response.body().as_ref())
                    .map_err(|e| CloudError::Serialization { source: e })?;

            let embedding: Vec<f32> = serde_json::from_value(
                response_body["embedding"].clone(),
            )
            .map_err(|e| CloudError::Serialization { source: e })?;

            println!("Embedding dimension: {}", embedding.len());
            embeddings.push(embedding);
        }

        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let model_id = extract_model_id(&req.model);
        let messages = build_messages(&req)?;
        let inference_config = build_inference_config(&req);

        let bedrock_tools: Vec<Tool> = tools
            .into_iter()
            .map(|tool| {
                let schema_doc = match json_to_document(tool.parameters) {
                    Document::Object(map) => Document::Object(map),
                    other => other,
                };
                let spec = ToolSpecification::builder()
                    .name(&tool.name)
                    .description(&tool.description)
                    .input_schema(ToolInputSchema::Json(schema_doc))
                    .build()
                    .map_err(|e| CloudError::Provider {
                        http_status: 0,
                        message: e.to_string(),
                        retryable: false,
                    });
                spec.map(Tool::ToolSpec)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut builder = self.client.converse().model_id(&model_id);
        for msg in messages {
            builder = builder.messages(msg);
        }
        if let Some(system) = &req.system_prompt {
            builder = builder.system(SystemContentBlock::Text(system.clone()));
        }
        builder = builder.inference_config(inference_config);

        // All tools must go into a single ToolConfiguration — not one per call
        let mut tool_config_builder =
            aws_sdk_bedrockruntime::types::ToolConfiguration::builder();
        for tool in bedrock_tools {
            tool_config_builder = tool_config_builder.tools(tool);
        }
        builder = builder.tool_config(
            tool_config_builder
                .build()
                .map_err(|e| CloudError::Provider {
                    http_status: 0,
                    message: e.to_string(),
                    retryable: false,
                })?,
        );

        let response = builder
            .send()
            .await
            .map_err(|e| CloudError::Provider {
                http_status: 0,
                message: e.to_string(),
                retryable: false,
            })?;

        let finish_reason = map_stop_reason(response.stop_reason());

        if matches!(response.stop_reason(), Some(StopReason::ToolUse)) {
            if let Some(ConverseOutput::Message(msg)) = response.output() {
                for block in msg.content() {
                    if let ContentBlock::ToolUse(tool_use) = block {
                        let arguments =
                            serde_json::to_value(format!("{:?}", tool_use.input()))
                                .unwrap_or(json!({}));
                        println!("Tool called: {}", tool_use.name());
                        return Ok(ToolCallResponse::ToolCall {
                            name: tool_use.name().to_string(),
                            arguments,
                        });
                    }
                }
            }
        }

        let text = match response.output() {
            Some(ConverseOutput::Message(msg)) => msg
                .content()
                .iter()
                .find_map(|block| {
                    if let ContentBlock::Text(t) = block {
                        Some(t.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
            _ => String::new(),
        };

        let usage = response.usage().map(|u| UsageStats {
            prompt_tokens: u.input_tokens() as u32,
            completion_tokens: u.output_tokens() as u32,
        });

        Ok(ToolCallResponse::Text(LlmResponse {
            text,
            finish_reason,
            usage,
        }))
    }
}

use async_trait::async_trait;
use aws_sdk_bedrockruntime::Client;
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ContentBlockDelta as StreamDelta, ConversationRole,
    ConverseOutput as ConverseOutputKind, ConverseStreamOutput as StreamEvent,
    InferenceConfiguration, Message as BedrockMessage, StopReason, SystemContentBlock,
};
use futures::channel::mpsc;
use futures::SinkExt;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
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

pub(crate) fn extract_model_id(model: &ModelRef) -> Result<String, CloudError> {
    match model {
        ModelRef::Provider(id) => Ok(id.clone()),
        ModelRef::Logical { family, tier } => Ok(match tier.as_deref() {
            Some(t) => format!("{}.{}", family, t),
            None => family.clone(),
        }),
        ModelRef::Deployment(_) => Err(CloudError::Unsupported {
            feature: "Bedrock does not use deployment names; use ModelRef::Provider",
        }),
    }
}

pub(crate) fn build_messages(req: &LlmRequest) -> Result<Vec<BedrockMessage>, CloudError> {
    req.messages
        .iter()
        .map(|msg| {
            let role = if msg.role == "assistant" {
                ConversationRole::Assistant
            } else {
                ConversationRole::User
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

pub(crate) fn build_inference_config(req: &LlmRequest) -> InferenceConfiguration {
    let mut builder = InferenceConfiguration::builder();
    if let Some(max_tokens) = req.max_tokens {
        builder = builder.max_tokens(max_tokens as i32);
    }
    if let Some(temp) = req.temperature {
        builder = builder.temperature(temp);
    }
    builder.build()
}

pub(crate) fn map_stop_reason(reason: &StopReason) -> FinishReason {
    match reason {
        StopReason::EndTurn => FinishReason::Stop,
        StopReason::MaxTokens => FinishReason::Length,
        StopReason::ToolUse => FinishReason::ToolCall,
        other => FinishReason::Other(other.as_str().to_string()),
    }
}

pub(crate) fn parse_embed_response(json: &serde_json::Value) -> Result<Vec<f32>, CloudError> {
    serde_json::from_value(json["embedding"].clone())
        .map_err(|e| CloudError::Serialization { source: e })
}

pub(crate) fn map_stream_event(event: &StreamEvent) -> Option<LlmStreamEvent> {
    match event {
        StreamEvent::ContentBlockDelta(e) => match e.delta() {
            Some(StreamDelta::Text(t)) => Some(LlmStreamEvent::DeltaText(t.clone())),
            _ => None,
        },
        StreamEvent::MessageStop(e) => {
            Some(LlmStreamEvent::Done(map_stop_reason(e.stop_reason())))
        }
        StreamEvent::Metadata(e) => e.usage().map(|u| LlmStreamEvent::Usage(UsageStats {
            prompt_tokens: u.input_tokens() as u32,
            completion_tokens: u.output_tokens() as u32,
        })),
        _ => None,
    }
}

#[async_trait]
impl LlmProvider for BedrockProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let model_id = extract_model_id(&req.model)?;
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

        let response = builder.send().await.map_err(|e| CloudError::Provider {
            http_status: 500,
            message: e.to_string(),
            retryable: false,
        })?;

        let finish_reason = map_stop_reason(response.stop_reason());

        let text = match response.output() {
            Some(ConverseOutputKind::Message(msg)) => msg
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

        Ok(LlmResponse {
            text,
            finish_reason,
            usage,
        })
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let model_id = extract_model_id(&req.model)?;
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

        let response = builder.send().await.map_err(|e| CloudError::Provider {
            http_status: 500,
            message: e.to_string(),
            retryable: false,
        })?;

        let (mut tx, rx) = mpsc::channel::<LlmStreamEvent>(32);
        let mut event_stream = response.stream;

        tokio::spawn(async move {
            loop {
                match event_stream.recv().await {
                    Ok(Some(event)) => {
                        if let Some(mapped) = map_stream_event(&event) {
                            if tx.send(mapped).await.is_err() {
                                break;
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let _ = tx
                            .send(LlmStreamEvent::Error(CloudError::Provider {
                                http_status: 500,
                                message: e.to_string(),
                                retryable: true,
                            }))
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

        let mut embeddings = Vec::with_capacity(texts.len());

        for text in &texts {
            let body_bytes = serde_json::to_vec(&serde_json::json!({"inputText": text}))
                .map_err(|e| CloudError::Serialization { source: e })?;

            let resp = self
                .client
                .invoke_model()
                .model_id("amazon.titan-embed-text-v2:0")
                .body(Blob::new(body_bytes))
                .send()
                .await
                .map_err(|e| CloudError::Provider {
                    http_status: 500,
                    message: e.to_string(),
                    retryable: false,
                })?;

            let resp_json: serde_json::Value = serde_json::from_slice(resp.body().as_ref())
                .map_err(|e| CloudError::Serialization { source: e })?;

            embeddings.push(parse_embed_response(&resp_json)?);
        }

        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::Unsupported {
            feature: "Bedrock generate_with_tools",
        })
    }
}

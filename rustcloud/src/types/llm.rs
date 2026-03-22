use serde::{Deserialize, Serialize};

use crate::errors::CloudError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelRef {
    Provider(String),
    Logical {
        family: String,
        tier: Option<String>,
    },
    Deployment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: ModelRef,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishReason {
    Stop,
    Length,
    ToolCall,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
    pub finish_reason: FinishReason,
    pub usage: Option<UsageStats>,
}

#[derive(Debug)]
pub enum LlmStreamEvent {
    DeltaText(String),
    Usage(UsageStats),
    Done(FinishReason),
    Error(CloudError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResponse {
    pub embeddings: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug)]
pub enum ToolCallResponse {
    Text(LlmResponse),
    ToolCall {
        name: String,
        arguments: serde_json::Value,
    },
}

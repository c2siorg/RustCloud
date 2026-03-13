use serde::{Deserialize, Serialize};

// ─── Gemini API Request Types ───

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiTool {
    pub function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// ─── Gemini API Response Types ───

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Option<Content>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiUsageMetadata {
    pub prompt_token_count: Option<u32>,
    pub candidates_token_count: Option<u32>,
}

// ─── Embedding Request/Response Types ───

#[derive(Debug, Serialize)]
pub struct EmbedRequest {
    pub instances: Vec<EmbedInstance>,
}

#[derive(Debug, Serialize)]
pub struct EmbedInstance {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct EmbedPredictResponse {
    pub predictions: Vec<EmbedPrediction>,
}

#[derive(Debug, Deserialize)]
pub struct EmbedPrediction {
    pub embeddings: EmbedValues,
}

#[derive(Debug, Deserialize)]
pub struct EmbedValues {
    pub values: Vec<f32>,
}

// ─── Vertex AI Dataset Types ───

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VertexDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub display_name: String,
    pub metadata_schema_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportDataConfig {
    pub gcs_source: GcsSource,
    pub import_schema_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcsSource {
    pub uris: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcsDestination {
    pub output_uri_prefix: String,
}

// ─── Vertex AI Endpoint Types ───

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEndpointRequest {
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployModelRequest {
    pub deployed_model: DeployedModel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployedModel {
    pub model: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_resources: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_resources: Option<AutomaticResources>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticResources {
    pub min_replica_count: u32,
    pub max_replica_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndeployModelRequest {
    pub deployed_model_id: String,
}

// ─── Streaming Types ───

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiStreamChunk {
    pub candidates: Option<Vec<Candidate>>,
    pub usage_metadata: Option<GeminiUsageMetadata>,
}

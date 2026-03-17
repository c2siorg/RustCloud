use crate::errors::CloudError;
use crate::genai::client::UnifiedLlmClient;
use crate::genai::routing::RoutingStrategy;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, Message, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

use async_trait::async_trait;
use futures::stream;
use serde_json::json;

// ── Stub providers for unit tests ─────────────────────────────────────────────

/// Always succeeds with a static response that includes its own name.
struct StubProvider {
    name: &'static str,
}

#[async_trait]
impl LlmProvider for StubProvider {
    async fn generate(&self, _req: LlmRequest) -> Result<LlmResponse, CloudError> {
        Ok(LlmResponse {
            text: format!("response from {}", self.name),
            finish_reason: FinishReason::Stop,
            usage: Some(UsageStats {
                prompt_tokens: 10,
                completion_tokens: 5,
            }),
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        let events = vec![
            LlmStreamEvent::DeltaText(format!("stream from {}", self.name)),
            LlmStreamEvent::Done(FinishReason::Stop),
        ];
        Ok(Box::pin(stream::iter(events)))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let embeddings = texts.iter().map(|_| vec![0.1_f32; 4]).collect();
        Ok(EmbedResponse { embeddings })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Ok(ToolCallResponse::Text(LlmResponse {
            text: format!("tool response from {}", self.name),
            finish_reason: FinishReason::Stop,
            usage: None,
        }))
    }
}

/// Always returns a transient error (RateLimit).
struct RateLimitedProvider;

#[async_trait]
impl LlmProvider for RateLimitedProvider {
    async fn generate(&self, _req: LlmRequest) -> Result<LlmResponse, CloudError> {
        Err(CloudError::RateLimit { retry_after: Some(30) })
    }
    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        Err(CloudError::RateLimit { retry_after: None })
    }
    async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        Err(CloudError::RateLimit { retry_after: None })
    }
    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        Err(CloudError::RateLimit { retry_after: None })
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn basic_request(model: ModelRef) -> LlmRequest {
    LlmRequest {
        model,
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }],
        max_tokens: Some(50),
        temperature: None,
        system_prompt: None,
    }
}

// ── Builder tests ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_builder_no_providers_returns_error() {
    let result = UnifiedLlmClient::builder().build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("at least one provider"));
}

#[tokio::test]
async fn test_builder_unknown_default_returns_error() {
    let result = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .default_provider("nonexistent")
        .build();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not registered"));
}

#[tokio::test]
async fn test_builder_first_provider_becomes_default() {
    let client = UnifiedLlmClient::builder()
        .register("first", Box::new(StubProvider { name: "first" }))
        .register("second", Box::new(StubProvider { name: "second" }))
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Provider("anything".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from first");
}

// ── Explicit routing tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_explicit_routing_uses_default() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .register("azure", Box::new(StubProvider { name: "azure" }))
        .default_provider("azure")
        .routing(RoutingStrategy::Explicit)
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Provider("gpt-4o".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from azure");
}

// ── ModelBased routing tests ──────────────────────────────────────────────────

#[tokio::test]
async fn test_model_based_routes_anthropic_to_aws() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .register("azure", Box::new(StubProvider { name: "azure" }))
        .register("gcp", Box::new(StubProvider { name: "gcp" }))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Provider(
            "anthropic.claude-3-5-haiku-20241022-v1:0".to_string(),
        )))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from aws");
}

#[tokio::test]
async fn test_model_based_routes_gemini_to_gcp() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .register("azure", Box::new(StubProvider { name: "azure" }))
        .register("gcp", Box::new(StubProvider { name: "gcp" }))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Provider("gemini-1.5-pro".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from gcp");
}

#[tokio::test]
async fn test_model_based_routes_gpt_to_azure() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .register("azure", Box::new(StubProvider { name: "azure" }))
        .register("gcp", Box::new(StubProvider { name: "gcp" }))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Deployment("gpt-4o".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from azure");
}

#[tokio::test]
async fn test_model_based_falls_back_to_default_on_unknown_model() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .register("azure", Box::new(StubProvider { name: "azure" }))
        .default_provider("azure")
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    // "llama-custom" doesn't match any rule
    let resp = client
        .generate(basic_request(ModelRef::Provider("llama-custom".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from azure");
}

// ── Fallback routing tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_fallback_skips_rate_limited_provider() {
    let client = UnifiedLlmClient::builder()
        .register("rate_limited", Box::new(RateLimitedProvider))
        .register("healthy", Box::new(StubProvider { name: "healthy" }))
        .routing(RoutingStrategy::Fallback)
        .build()
        .unwrap();

    let resp = client
        .generate(basic_request(ModelRef::Provider("any".to_string())))
        .await
        .unwrap();

    assert_eq!(resp.text, "response from healthy");
}

#[tokio::test]
async fn test_fallback_returns_last_error_when_all_fail() {
    let client = UnifiedLlmClient::builder()
        .register("p1", Box::new(RateLimitedProvider))
        .register("p2", Box::new(RateLimitedProvider))
        .routing(RoutingStrategy::Fallback)
        .build()
        .unwrap();

    let result = client
        .generate(basic_request(ModelRef::Provider("any".to_string())))
        .await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CloudError::RateLimit { .. }));
}

#[tokio::test]
async fn test_fallback_embed_skips_rate_limited_provider() {
    let client = UnifiedLlmClient::builder()
        .register("rate_limited", Box::new(RateLimitedProvider))
        .register("healthy", Box::new(StubProvider { name: "healthy" }))
        .routing(RoutingStrategy::Fallback)
        .build()
        .unwrap();

    let resp = client
        .embed(vec!["hello".to_string(), "world".to_string()])
        .await
        .unwrap();

    assert_eq!(resp.embeddings.len(), 2);
    assert_eq!(resp.embeddings[0].len(), 4);
}

// ── Stream test ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_stream_collects_events() {
    use futures::StreamExt;

    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .build()
        .unwrap();

    let mut stream = client
        .stream(basic_request(ModelRef::Provider("any".to_string())))
        .await
        .unwrap();

    let mut text_parts: Vec<String> = Vec::new();
    let mut got_done = false;

    while let Some(event) = stream.next().await {
        match event {
            LlmStreamEvent::DeltaText(t) => text_parts.push(t),
            LlmStreamEvent::Done(_) => got_done = true,
            _ => {}
        }
    }

    assert!(!text_parts.is_empty());
    assert!(got_done);
}

// ── generate_with_tools test ──────────────────────────────────────────────────

#[tokio::test]
async fn test_generate_with_tools_explicit_routing() {
    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(StubProvider { name: "aws" }))
        .build()
        .unwrap();

    let tools = vec![ToolDefinition {
        name: "get_time".to_string(),
        description: "Returns the current time".to_string(),
        parameters: json!({ "type": "object", "properties": {} }),
    }];

    let result = client
        .generate_with_tools(
            basic_request(ModelRef::Provider("any".to_string())),
            tools,
        )
        .await
        .unwrap();

    assert!(matches!(result, ToolCallResponse::Text(_)));
}

// ── Integration tests (requires env credentials) ──────────────────────────────
//
// These tests hit real cloud APIs. They are guarded by environment variable
// checks so that CI passes without credentials.

#[tokio::test]
#[ignore = "requires AWS credentials"]
async fn integration_aws_bedrock_via_unified_client() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;

    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(BedrockProvider::new().await))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Reply with exactly: OK".to_string(),
        }],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let resp = client.generate(req).await.unwrap();
    assert!(!resp.text.is_empty());
    println!("Bedrock via UnifiedLlmClient: {}", resp.text);
}

#[tokio::test]
#[ignore = "requires GCP credentials and project"]
async fn integration_vertex_ai_via_unified_client() {
    use crate::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;

    let project = std::env::var("GCP_PROJECT_ID").unwrap_or("your-project-id".to_string());
    let location = std::env::var("GCP_LOCATION").unwrap_or("us-central1".to_string());

    let client = UnifiedLlmClient::builder()
        .register("gcp", Box::new(VertexAI::new(&project, &location)))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let req = LlmRequest {
        model: ModelRef::Provider("gemini-1.5-flash".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Reply with exactly: OK".to_string(),
        }],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let resp = client.generate(req).await.unwrap();
    assert!(!resp.text.is_empty());
    println!("Vertex AI via UnifiedLlmClient: {}", resp.text);
}

#[tokio::test]
#[ignore = "requires Azure OpenAI credentials"]
async fn integration_azure_openai_via_unified_client() {
    use crate::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;

    let client = UnifiedLlmClient::builder()
        .register("azure", Box::new(AzureOpenAIProvider::new()))
        .routing(RoutingStrategy::ModelBased)
        .build()
        .unwrap();

    let req = LlmRequest {
        model: ModelRef::Deployment("gpt-4o".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Reply with exactly: OK".to_string(),
        }],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let resp = client.generate(req).await.unwrap();
    assert!(!resp.text.is_empty());
    println!("Azure OpenAI via UnifiedLlmClient: {}", resp.text);
}

#[tokio::test]
#[ignore = "requires credentials for all three providers"]
async fn integration_fallback_across_all_providers() {
    use crate::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
    use crate::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
    use crate::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;

    let project = std::env::var("GCP_PROJECT_ID").unwrap_or("your-project-id".to_string());

    let client = UnifiedLlmClient::builder()
        .register("aws", Box::new(BedrockProvider::new().await))
        .register("gcp", Box::new(VertexAI::new(&project, "us-central1")))
        .register("azure", Box::new(AzureOpenAIProvider::new()))
        .routing(RoutingStrategy::Fallback)
        .build()
        .unwrap();

    let req = LlmRequest {
        model: ModelRef::Provider("anthropic.claude-3-5-haiku-20241022-v1:0".to_string()),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Reply with exactly: OK".to_string(),
        }],
        max_tokens: Some(10),
        temperature: Some(0.0),
        system_prompt: None,
    };

    let resp = client.generate(req).await.unwrap();
    assert!(!resp.text.is_empty());
    println!("Fallback result: {}", resp.text);
}

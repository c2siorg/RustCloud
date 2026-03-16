/// Multi-cloud unified generative AI routing layer.
///
/// # Overview
///
/// This module provides [`client::UnifiedLlmClient`] — a single entry-point
/// that fans requests out to AWS Bedrock, GCP Vertex AI, or Azure OpenAI
/// based on a pluggable [`routing::RoutingStrategy`].
///
/// All backends implement the shared [`crate::traits::llm_provider::LlmProvider`]
/// trait, so switching providers (or layering fallback logic) requires no
/// changes to application code.
///
/// # Architecture
///
/// ```text
///  ┌─────────────────────────────────────────────────────┐
///  │              UnifiedLlmClient                        │
///  │                                                     │
///  │  ┌────────────┐  ┌──────────────┐  ┌────────────┐  │
///  │  │  Bedrock   │  │  VertexAI    │  │AzureOpenAI │  │
///  │  │ (aws)      │  │  (gcp)       │  │ (azure)    │  │
///  │  └────────────┘  └──────────────┘  └────────────┘  │
///  │                                                     │
///  │  RoutingStrategy: Explicit | ModelBased | Fallback  │
///  └─────────────────────────────────────────────────────┘
/// ```
///
/// # Example
///
/// ```rust,no_run
/// use rustcloud::genai::client::UnifiedLlmClient;
/// use rustcloud::genai::routing::RoutingStrategy;
/// use rustcloud::types::llm::{LlmRequest, Message, ModelRef};
/// use rustcloud::traits::llm_provider::LlmProvider;
///
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Each provider is registered under a short alias.
/// // The first registered provider is the default.
/// // let client = UnifiedLlmClient::builder()
/// //     .register("aws",   Box::new(BedrockProvider::new().await))
/// //     .register("azure", Box::new(AzureOpenAIProvider::new()))
/// //     .register("gcp",   Box::new(VertexAI::new("proj", "us-central1")))
/// //     .routing(RoutingStrategy::ModelBased)
/// //     .build()?;
/// # Ok(()) }
/// ```
pub mod client;
pub mod retry;
pub mod routing;

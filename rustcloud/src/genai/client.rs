use std::collections::HashMap;

use async_trait::async_trait;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse, ToolDefinition,
};

use super::routing::{infer_provider, is_transient, RoutingStrategy};

// ── UnifiedLlmClient ──────────────────────────────────────────────────────────

/// A provider-agnostic LLM client that delegates to one or more registered
/// backend providers based on a configurable [`RoutingStrategy`].
///
/// # Quick start
///
/// ```rust,no_run
/// # use rustcloud::genai::client::UnifiedLlmClient;
/// # use rustcloud::genai::routing::RoutingStrategy;
/// # use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
/// # use rustcloud::azure::azure_apis::artificial_intelligence::azure_openai::AzureOpenAIProvider;
/// # use rustcloud::gcp::gcp_apis::artificial_intelligence::gcp_vertex_ai::VertexAI;
/// # #[tokio::main] async fn main() {
/// let client = UnifiedLlmClient::builder()
///     .register("aws",   Box::new(BedrockProvider::new().await))
///     .register("azure", Box::new(AzureOpenAIProvider::new()))
///     .register("gcp",   Box::new(VertexAI::new("my-project", "us-central1")))
///     .default_provider("aws")
///     .routing(RoutingStrategy::ModelBased)
///     .build()
///     .expect("at least one provider must be registered");
/// # }
/// ```
pub struct UnifiedLlmClient {
    /// Providers stored in insertion order so that `Fallback` routing tries
    /// them deterministically.
    providers: Vec<(String, Box<dyn LlmProvider>)>,
    /// Fast name → index lookup.
    index: HashMap<String, usize>,
    default_provider: String,
    routing: RoutingStrategy,
}

impl UnifiedLlmClient {
    /// Begin building a new client.
    pub fn builder() -> UnifiedLlmClientBuilder {
        UnifiedLlmClientBuilder::new()
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Return the provider for the given key, or `None` if it is not registered.
    fn get(&self, key: &str) -> Option<&dyn LlmProvider> {
        self.index
            .get(key)
            .and_then(|&i| self.providers.get(i))
            .map(|(_, p)| p.as_ref())
    }

    /// Resolve which provider to use for a request using `ModelBased` routing.
    /// Falls back to `default_provider` when no rule matches or the inferred
    /// key is not registered.
    fn resolve_provider(&self, req: &LlmRequest) -> &dyn LlmProvider {
        let key = match &self.routing {
            RoutingStrategy::ModelBased => infer_provider(&req.model)
                .and_then(|k| if self.index.contains_key(k) { Some(k) } else { None })
                .unwrap_or(self.default_provider.as_str()),
            _ => self.default_provider.as_str(),
        };
        self.get(key)
            .or_else(|| self.get(&self.default_provider))
            .expect("default_provider must exist; validated in builder")
    }
}

// ── LlmProvider impl ──────────────────────────────────────────────────────────

#[async_trait]
impl LlmProvider for UnifiedLlmClient {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        match &self.routing {
            RoutingStrategy::Fallback => {
                let mut last_err = CloudError::Provider {
                    http_status: 0,
                    message: "No providers registered".to_string(),
                    retryable: false,
                };
                for (_, provider) in &self.providers {
                    match provider.generate(req.clone()).await {
                        Ok(resp) => return Ok(resp),
                        Err(e) => {
                            if is_transient(&e) {
                                last_err = e;
                                continue;
                            }
                            // Hard error — propagate immediately
                            return Err(e);
                        }
                    }
                }
                Err(last_err)
            }
            _ => self.resolve_provider(&req).generate(req).await,
        }
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        match &self.routing {
            RoutingStrategy::Fallback => {
                let mut last_err = CloudError::Provider {
                    http_status: 0,
                    message: "No providers registered".to_string(),
                    retryable: false,
                };
                for (_, provider) in &self.providers {
                    match provider.stream(req.clone()).await {
                        Ok(stream) => return Ok(stream),
                        Err(e) => {
                            if is_transient(&e) {
                                last_err = e;
                                continue;
                            }
                            return Err(e);
                        }
                    }
                }
                Err(last_err)
            }
            _ => self.resolve_provider(&req).stream(req).await,
        }
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        match &self.routing {
            RoutingStrategy::Fallback => {
                let mut last_err = CloudError::Provider {
                    http_status: 0,
                    message: "No providers registered".to_string(),
                    retryable: false,
                };
                for (_, provider) in &self.providers {
                    match provider.embed(texts.clone()).await {
                        Ok(resp) => return Ok(resp),
                        Err(e) => {
                            if is_transient(&e) {
                                last_err = e;
                                continue;
                            }
                            return Err(e);
                        }
                    }
                }
                Err(last_err)
            }
            _ => {
                // embed() has no ModelRef to route on — always use default
                self.get(&self.default_provider)
                    .expect("default_provider exists")
                    .embed(texts)
                    .await
            }
        }
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        match &self.routing {
            RoutingStrategy::Fallback => {
                let mut last_err = CloudError::Provider {
                    http_status: 0,
                    message: "No providers registered".to_string(),
                    retryable: false,
                };
                for (_, provider) in &self.providers {
                    match provider
                        .generate_with_tools(req.clone(), tools.clone())
                        .await
                    {
                        Ok(resp) => return Ok(resp),
                        Err(e) => {
                            if is_transient(&e) {
                                last_err = e;
                                continue;
                            }
                            return Err(e);
                        }
                    }
                }
                Err(last_err)
            }
            _ => {
                self.resolve_provider(&req)
                    .generate_with_tools(req, tools)
                    .await
            }
        }
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Fluent builder for [`UnifiedLlmClient`].
pub struct UnifiedLlmClientBuilder {
    providers: Vec<(String, Box<dyn LlmProvider>)>,
    default_provider: Option<String>,
    routing: RoutingStrategy,
}

impl UnifiedLlmClientBuilder {
    fn new() -> Self {
        Self {
            providers: Vec::new(),
            default_provider: None,
            routing: RoutingStrategy::default(),
        }
    }

    /// Register a backend provider under the given key.
    ///
    /// The first provider registered becomes the implicit default unless
    /// [`Self::default_provider`] is called explicitly.
    pub fn register(mut self, key: &str, provider: Box<dyn LlmProvider>) -> Self {
        self.providers.push((key.to_string(), provider));
        self
    }

    /// Set the key of the default provider.
    ///
    /// If not set, the first registered provider is used as the default.
    pub fn default_provider(mut self, key: &str) -> Self {
        self.default_provider = Some(key.to_string());
        self
    }

    /// Choose the routing strategy. Defaults to [`RoutingStrategy::Explicit`].
    pub fn routing(mut self, strategy: RoutingStrategy) -> Self {
        self.routing = strategy;
        self
    }

    /// Consume the builder and return a [`UnifiedLlmClient`].
    ///
    /// Returns `Err` when no providers have been registered or when the
    /// explicit default key does not match any registered provider.
    pub fn build(self) -> Result<UnifiedLlmClient, String> {
        if self.providers.is_empty() {
            return Err("UnifiedLlmClient requires at least one registered provider".to_string());
        }

        // Build O(1) name → index lookup
        let mut index: HashMap<String, usize> = HashMap::new();
        for (i, (key, _)) in self.providers.iter().enumerate() {
            index.insert(key.clone(), i);
        }

        let default_provider = match self.default_provider {
            Some(ref k) => {
                if !index.contains_key(k) {
                    return Err(format!(
                        "default provider '{}' is not registered; available: {}",
                        k,
                        self.providers
                            .iter()
                            .map(|(k, _)| k.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                k.clone()
            }
            // Default to the first registered provider
            None => self.providers[0].0.clone(),
        };

        Ok(UnifiedLlmClient {
            providers: self.providers,
            index,
            default_provider,
            routing: self.routing,
        })
    }
}

// ── LlmStreamEvent helper ─────────────────────────────────────────────────────

/// Convert a vector of events back into an `LlmStream` — useful in tests.
pub fn events_to_stream(
    events: Vec<LlmStreamEvent>,
) -> crate::traits::llm_provider::LlmStream {
    use futures::stream;
    Box::pin(stream::iter(events))
}

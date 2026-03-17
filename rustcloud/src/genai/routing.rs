use crate::types::llm::ModelRef;

/// Determines how [`super::client::UnifiedLlmClient`] selects a backend
/// provider for each request.
#[derive(Debug, Clone, Default)]
pub enum RoutingStrategy {
    /// Always use the registered default provider, regardless of `ModelRef`.
    #[default]
    Explicit,

    /// Infer the provider from the model identifier in `ModelRef`.
    ///
    /// Matching rules (first match wins):
    /// - `anthropic.*`, `amazon.*`, `us.*`, `meta.*`, `cohere.*` → `"aws"`
    /// - `gemini*`, `text-embedding-*`                           → `"gcp"`
    /// - `gpt-*`, `o1*`, `o3*`, or any `Deployment(_)`          → `"azure"`
    ///
    /// Falls back to the default provider when no rule matches.
    ModelBased,

    /// Attempt providers in registration order; move to the next on any
    /// *transient* error (`RateLimit`, `Network`, or
    /// `Provider { retryable: true }`).
    ///
    /// Hard errors (`Auth`, `Unsupported`) are skipped and the next provider
    /// is tried. If every provider fails, the last error is returned.
    Fallback,
}

/// Maps a `ModelRef` to a canonical provider key using the `ModelBased` rules.
/// Returns `None` when no rule matches (caller should fall back to the default).
pub fn infer_provider(model: &ModelRef) -> Option<&'static str> {
    let id: &str = match model {
        ModelRef::Provider(id) | ModelRef::Deployment(id) => id,
        ModelRef::Logical { family, .. } => family,
    };

    // Azure deployment IDs are opaque strings — treat any Deployment as Azure
    if matches!(model, ModelRef::Deployment(_)) {
        return Some("azure");
    }

    let id_lower = id.to_ascii_lowercase();

    if id_lower.starts_with("anthropic.")
        || id_lower.starts_with("amazon.")
        || id_lower.starts_with("us.")
        || id_lower.starts_with("eu.")
        || id_lower.starts_with("meta.")
        || id_lower.starts_with("cohere.")
        || id_lower.starts_with("mistral.")
    {
        return Some("aws");
    }

    if id_lower.starts_with("gemini")
        || id_lower.starts_with("text-embedding-")
        || id_lower.starts_with("textembedding-")
    {
        return Some("gcp");
    }

    if id_lower.starts_with("gpt-")
        || id_lower.starts_with("o1")
        || id_lower.starts_with("o3")
        || id_lower.starts_with("text-davinci")
    {
        return Some("azure");
    }

    None
}

/// Returns `true` for errors that are considered transient / retryable by the
/// Fallback routing strategy.
pub fn is_transient(err: &crate::errors::CloudError) -> bool {
    use crate::errors::CloudError;
    matches!(
        err,
        CloudError::RateLimit { .. }
            | CloudError::Network { .. }
            | CloudError::Provider { retryable: true, .. }
    )
}

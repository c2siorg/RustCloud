//! Adaptive Fallback Provider for High-Availability GenAI Workloads
//!
//! This module provides a `FallbackProvider` that wraps multiple LLM providers
//! and automatically falls back to the next provider when transient errors occur.

use async_trait::async_trait;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, ToolCallResponse, ToolDefinition,
};

/// Determines if a `CloudError` is transient and should trigger a fallback.
///
/// This function detects:
/// - Explicit rate limit errors (`CloudError::RateLimit`)
/// - AWS Bedrock: `ThrottlingException` or `InternalServerException`
/// - GCP Vertex AI: HTTP 429 (RESOURCE_EXHAUSTED) or 500 (INTERNAL_SERVER_ERROR)
/// - Provider errors marked as retryable
/// - Transient network errors
fn is_retryable(error: &CloudError) -> bool {
    match error {
        // Explicit rate limit - always retryable
        CloudError::RateLimit { .. } => true,

        // Provider errors - check http_status and message
        CloudError::Provider {
            http_status,
            message,
            retryable,
        } => {
            // If explicitly marked as retryable
            if *retryable {
                return true;
            }

            // GCP Vertex AI: HTTP 429 (RESOURCE_EXHAUSTED)
            if *http_status == 429 {
                return true;
            }

            // GCP Vertex AI: HTTP 500 (INTERNAL_SERVER_ERROR)
            if *http_status == 500 {
                return true;
            }

            // AWS Bedrock: ThrottlingException
            if message.contains("ThrottlingException") {
                return true;
            }

            // AWS Bedrock: InternalServerException
            if message.contains("InternalServerException") {
                return true;
            }

            false
        }

        // Network errors are typically transient
        CloudError::Network { .. } => true,

        // Auth errors are not retryable - indicates misconfiguration
        CloudError::Auth { .. } => false,

        // Serialization errors are not retryable - indicates code bug
        CloudError::Serialization { .. } => false,

        // Unsupported features won't become supported by retrying
        CloudError::Unsupported { .. } => false,
    }
}

/// A resilient LLM provider that wraps multiple providers and automatically
/// falls back to the next provider when transient errors occur.
///
/// # Example
///
/// ```ignore
/// use rustcloud::providers::FallbackProvider;
///
/// let fallback = FallbackProvider::new(vec![
///     Box::new(aws_bedrock_provider),
///     Box::new(gcp_vertex_provider),
///     Box::new(azure_openai_provider),
/// ]);
///
/// // If AWS fails with rate limit, automatically tries GCP, then Azure
/// let response = fallback.generate(request).await?;
/// ```
pub struct FallbackProvider {
    providers: Vec<Box<dyn LlmProvider + Send + Sync>>,
}

impl FallbackProvider {
    /// Creates a new `FallbackProvider` with the given list of providers.
    ///
    /// Providers are tried in order - the first provider is the primary,
    /// subsequent providers are fallbacks used when transient errors occur.
    ///
    /// # Panics
    ///
    /// Panics if the providers list is empty.
    pub fn new(providers: Vec<Box<dyn LlmProvider + Send + Sync>>) -> Self {
        assert!(!providers.is_empty(), "FallbackProvider requires at least one provider");
        Self { providers }
    }

    /// Returns the number of providers in the fallback chain.
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

#[async_trait]
impl LlmProvider for FallbackProvider {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let mut last_error: Option<CloudError> = None;

        for (idx, provider) in self.providers.iter().enumerate() {
            let request = req.clone();

            match provider.generate(request).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    let is_last = idx == self.providers.len() - 1;

                    if is_last || !is_retryable(&err) {
                        // Either this is the last provider, or the error is not retryable
                        return Err(err);
                    }

                    // Store error and continue to next provider
                    last_error = Some(err);
                }
            }
        }

        // This should never be reached due to the loop logic,
        // but handle it just in case
        Err(last_error.unwrap_or_else(|| CloudError::Provider {
            http_status: 500,
            message: "No providers available".to_string(),
            retryable: false,
        }))
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        let mut last_error: Option<CloudError> = None;

        for (idx, provider) in self.providers.iter().enumerate() {
            let request = req.clone();

            match provider.stream(request).await {
                Ok(stream) => return Ok(stream),
                Err(err) => {
                    let is_last = idx == self.providers.len() - 1;

                    if is_last || !is_retryable(&err) {
                        return Err(err);
                    }

                    last_error = Some(err);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| CloudError::Provider {
            http_status: 500,
            message: "No providers available".to_string(),
            retryable: false,
        }))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let mut last_error: Option<CloudError> = None;

        for (idx, provider) in self.providers.iter().enumerate() {
            let texts_clone = texts.clone();

            match provider.embed(texts_clone).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    let is_last = idx == self.providers.len() - 1;

                    if is_last || !is_retryable(&err) {
                        return Err(err);
                    }

                    last_error = Some(err);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| CloudError::Provider {
            http_status: 500,
            message: "No providers available".to_string(),
            retryable: false,
        }))
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let mut last_error: Option<CloudError> = None;

        for (idx, provider) in self.providers.iter().enumerate() {
            let request = req.clone();
            let tools_clone = tools.clone();

            match provider.generate_with_tools(request, tools_clone).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    let is_last = idx == self.providers.len() - 1;

                    if is_last || !is_retryable(&err) {
                        return Err(err);
                    }

                    last_error = Some(err);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| CloudError::Provider {
            http_status: 500,
            message: "No providers available".to_string(),
            retryable: false,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_retryable_rate_limit() {
        let error = CloudError::RateLimit { retry_after: Some(30) };
        assert!(is_retryable(&error));

        let error = CloudError::RateLimit { retry_after: None };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_http_429() {
        let error = CloudError::Provider {
            http_status: 429,
            message: "RESOURCE_EXHAUSTED".to_string(),
            retryable: false,
        };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_http_500() {
        let error = CloudError::Provider {
            http_status: 500,
            message: "INTERNAL_SERVER_ERROR".to_string(),
            retryable: false,
        };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_aws_throttling() {
        let error = CloudError::Provider {
            http_status: 400,
            message: "ThrottlingException: Rate exceeded".to_string(),
            retryable: false,
        };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_aws_internal() {
        let error = CloudError::Provider {
            http_status: 400,
            message: "InternalServerException: Service unavailable".to_string(),
            retryable: false,
        };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_is_retryable_explicit_flag() {
        let error = CloudError::Provider {
            http_status: 503,
            message: "Service temporarily unavailable".to_string(),
            retryable: true,
        };
        assert!(is_retryable(&error));
    }

    #[test]
    fn test_not_retryable_auth() {
        let error = CloudError::Auth {
            message: "Invalid credentials".to_string(),
        };
        assert!(!is_retryable(&error));
    }

    #[test]
    fn test_not_retryable_unsupported() {
        let error = CloudError::Unsupported {
            feature: "streaming",
        };
        assert!(!is_retryable(&error));
    }

    #[test]
    fn test_not_retryable_generic_provider_error() {
        let error = CloudError::Provider {
            http_status: 400,
            message: "Bad request: invalid model".to_string(),
            retryable: false,
        };
        assert!(!is_retryable(&error));
    }
}

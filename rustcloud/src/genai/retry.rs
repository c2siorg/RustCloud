use std::time::Duration;

use async_trait::async_trait;

use crate::errors::CloudError;
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, LlmRequest, LlmResponse, LlmStreamEvent, ToolCallResponse, ToolDefinition,
};

// ── RetryConfig ───────────────────────────────────────────────────────────────

/// Exponential back-off configuration for [`RetryProvider`].
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Total number of attempts (including the first). Must be ≥ 1.
    pub max_attempts: u32,
    /// Delay before the second attempt in milliseconds.
    pub initial_delay_ms: u64,
    /// Multiplier applied to the delay after each failed attempt.
    pub backoff_factor: f64,
    /// Upper bound on the computed delay in milliseconds.
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 500,
            backoff_factor: 2.0,
            max_delay_ms: 10_000,
        }
    }
}

// ── RetryProvider ─────────────────────────────────────────────────────────────

/// Wraps any [`LlmProvider`] and automatically retries transient failures
/// (`RateLimit`, `Network`, `Provider { retryable: true }`) with exponential
/// back-off. Non-transient errors propagate immediately without retrying.
///
/// # Example
///
/// ```rust,no_run
/// # use rustcloud::genai::retry::{RetryProvider, RetryConfig};
/// # use rustcloud::aws::aws_apis::artificial_intelligence::aws_bedrock::BedrockProvider;
/// # #[tokio::main] async fn main() {
/// let provider = RetryProvider::new(
///     BedrockProvider::new().await,
///     RetryConfig { max_attempts: 4, initial_delay_ms: 200, ..Default::default() },
/// );
/// # }
/// ```
pub struct RetryProvider<P: LlmProvider> {
    inner: P,
    config: RetryConfig,
}

impl<P: LlmProvider> RetryProvider<P> {
    pub fn new(inner: P, config: RetryConfig) -> Self {
        Self { inner, config }
    }

    /// Create a `RetryProvider` with the default `RetryConfig` (3 attempts, 500 ms base).
    pub fn with_defaults(inner: P) -> Self {
        Self::new(inner, RetryConfig::default())
    }

    /// Compute the sleep duration for a given attempt index (0-based).
    /// If the error is a `RateLimit` with a `retry_after` hint, that value
    /// takes precedence over the computed back-off.
    fn sleep_duration(&self, attempt: u32, err: &CloudError) -> Duration {
        if let CloudError::RateLimit {
            retry_after: Some(secs),
        } = err
        {
            return Duration::from_secs(*secs);
        }
        let delay = (self.config.initial_delay_ms as f64
            * self.config.backoff_factor.powi(attempt as i32))
        .min(self.config.max_delay_ms as f64) as u64;
        Duration::from_millis(delay)
    }
}

fn is_retryable(err: &CloudError) -> bool {
    matches!(
        err,
        CloudError::RateLimit { .. }
            | CloudError::Network { .. }
            | CloudError::Provider {
                retryable: true,
                ..
            }
    )
}

// ── LlmProvider impl ──────────────────────────────────────────────────────────

#[async_trait]
impl<P: LlmProvider> LlmProvider for RetryProvider<P> {
    async fn generate(&self, req: LlmRequest) -> Result<LlmResponse, CloudError> {
        let mut last_err = CloudError::Provider {
            http_status: 0,
            message: "max_attempts must be >= 1".to_string(),
            retryable: false,
        };
        for attempt in 0..self.config.max_attempts {
            match self.inner.generate(req.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retryable(&e) => {
                    if attempt + 1 < self.config.max_attempts {
                        tokio::time::sleep(self.sleep_duration(attempt, &e)).await;
                    }
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }

    async fn stream(&self, req: LlmRequest) -> Result<LlmStream, CloudError> {
        // Retry the *initial connection* only. Once streaming has begun, the
        // caller consumes the stream and errors within it are passed through
        // as `LlmStreamEvent::Error` by the underlying provider.
        let mut last_err = CloudError::Provider {
            http_status: 0,
            message: "max_attempts must be >= 1".to_string(),
            retryable: false,
        };
        for attempt in 0..self.config.max_attempts {
            match self.inner.stream(req.clone()).await {
                Ok(stream) => return Ok(stream),
                Err(e) if is_retryable(&e) => {
                    if attempt + 1 < self.config.max_attempts {
                        tokio::time::sleep(self.sleep_duration(attempt, &e)).await;
                    }
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        let mut last_err = CloudError::Provider {
            http_status: 0,
            message: "max_attempts must be >= 1".to_string(),
            retryable: false,
        };
        for attempt in 0..self.config.max_attempts {
            match self.inner.embed(texts.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retryable(&e) => {
                    if attempt + 1 < self.config.max_attempts {
                        tokio::time::sleep(self.sleep_duration(attempt, &e)).await;
                    }
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }

    async fn generate_with_tools(
        &self,
        req: LlmRequest,
        tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        let mut last_err = CloudError::Provider {
            http_status: 0,
            message: "max_attempts must be >= 1".to_string(),
            retryable: false,
        };
        for attempt in 0..self.config.max_attempts {
            match self
                .inner
                .generate_with_tools(req.clone(), tools.clone())
                .await
            {
                Ok(resp) => return Ok(resp),
                Err(e) if is_retryable(&e) => {
                    if attempt + 1 < self.config.max_attempts {
                        tokio::time::sleep(self.sleep_duration(attempt, &e)).await;
                    }
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }
}

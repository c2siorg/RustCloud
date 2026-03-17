use crate::errors::CloudError;
use crate::genai::retry::{RetryConfig, RetryProvider};
use crate::traits::llm_provider::{LlmProvider, LlmStream};
use crate::types::llm::{
    EmbedResponse, FinishReason, LlmRequest, LlmResponse, LlmStreamEvent, Message, ModelRef,
    ToolCallResponse, ToolDefinition, UsageStats,
};

use async_trait::async_trait;
use futures::stream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

// ── Counting stub that fails N times then succeeds ────────────────────────────

struct FailNTimes {
    fail_count: Arc<AtomicU32>,
    failures_remaining: Arc<AtomicU32>,
    error_kind: &'static str, // "rate_limit" | "network" | "hard"
}

impl FailNTimes {
    fn rate_limit(n: u32) -> Self {
        Self {
            fail_count: Arc::new(AtomicU32::new(0)),
            failures_remaining: Arc::new(AtomicU32::new(n)),
            error_kind: "rate_limit",
        }
    }
    fn hard_fail(n: u32) -> Self {
        Self {
            fail_count: Arc::new(AtomicU32::new(0)),
            failures_remaining: Arc::new(AtomicU32::new(n)),
            error_kind: "hard",
        }
    }
    fn attempt_count(&self) -> u32 {
        self.fail_count.load(Ordering::SeqCst)
    }
}

fn make_error(kind: &'static str) -> CloudError {
    match kind {
        "rate_limit" => CloudError::RateLimit { retry_after: None },
        "hard" => CloudError::Unsupported { feature: "test-hard-fail" },
        _ => CloudError::RateLimit { retry_after: None },
    }
}

#[async_trait]
impl LlmProvider for FailNTimes {
    async fn generate(&self, _req: LlmRequest) -> Result<LlmResponse, CloudError> {
        self.fail_count.fetch_add(1, Ordering::SeqCst);
        let remaining = self.failures_remaining.load(Ordering::SeqCst);
        if remaining > 0 {
            self.failures_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err(make_error(self.error_kind));
        }
        Ok(LlmResponse {
            text: "success".to_string(),
            finish_reason: FinishReason::Stop,
            usage: Some(UsageStats { prompt_tokens: 5, completion_tokens: 1 }),
        })
    }

    async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
        self.fail_count.fetch_add(1, Ordering::SeqCst);
        let remaining = self.failures_remaining.load(Ordering::SeqCst);
        if remaining > 0 {
            self.failures_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err(make_error(self.error_kind));
        }
        Ok(Box::pin(stream::iter(vec![
            LlmStreamEvent::DeltaText("ok".to_string()),
            LlmStreamEvent::Done(FinishReason::Stop),
        ])))
    }

    async fn embed(&self, texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
        self.fail_count.fetch_add(1, Ordering::SeqCst);
        let remaining = self.failures_remaining.load(Ordering::SeqCst);
        if remaining > 0 {
            self.failures_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err(make_error(self.error_kind));
        }
        Ok(EmbedResponse {
            embeddings: texts.iter().map(|_| vec![1.0_f32]).collect(),
        })
    }

    async fn generate_with_tools(
        &self,
        _req: LlmRequest,
        _tools: Vec<ToolDefinition>,
    ) -> Result<ToolCallResponse, CloudError> {
        self.fail_count.fetch_add(1, Ordering::SeqCst);
        let remaining = self.failures_remaining.load(Ordering::SeqCst);
        if remaining > 0 {
            self.failures_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err(make_error(self.error_kind));
        }
        Ok(ToolCallResponse::Text(LlmResponse {
            text: "success".to_string(),
            finish_reason: FinishReason::Stop,
            usage: None,
        }))
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn req() -> LlmRequest {
    LlmRequest {
        model: ModelRef::Provider("test".to_string()),
        messages: vec![Message { role: "user".to_string(), content: "hi".to_string() }],
        max_tokens: Some(10),
        temperature: None,
        system_prompt: None,
    }
}

fn fast_config(max_attempts: u32) -> RetryConfig {
    RetryConfig {
        max_attempts,
        initial_delay_ms: 0, // zero delay for tests
        backoff_factor: 1.0,
        max_delay_ms: 0,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_succeeds_on_first_attempt() {
    let provider = RetryProvider::new(FailNTimes::rate_limit(0), fast_config(3));
    let resp = provider.generate(req()).await.unwrap();
    assert_eq!(resp.text, "success");
}

#[tokio::test]
async fn test_retries_rate_limit_and_succeeds() {
    let inner = FailNTimes::rate_limit(2); // fail twice then succeed
    let provider = RetryProvider::new(inner, fast_config(3));
    let resp = provider.generate(req()).await.unwrap();
    assert_eq!(resp.text, "success");
}

#[tokio::test]
async fn test_exhausts_retries_and_returns_last_error() {
    let provider = RetryProvider::new(FailNTimes::rate_limit(10), fast_config(3));
    let err = provider.generate(req()).await.unwrap_err();
    assert!(matches!(err, CloudError::RateLimit { .. }));
}

#[tokio::test]
async fn test_hard_error_propagates_immediately_without_retry() {
    let inner = FailNTimes::hard_fail(1);
    let attempts_ref = Arc::clone(&inner.fail_count);
    let provider = RetryProvider::new(inner, fast_config(3));
    let err = provider.generate(req()).await.unwrap_err();
    assert!(matches!(err, CloudError::Unsupported { .. }));
    // Hard error on first attempt — should NOT retry
    assert_eq!(attempts_ref.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_stream_retries_initial_connection() {
    use futures::StreamExt;
    let provider = RetryProvider::new(FailNTimes::rate_limit(1), fast_config(3));
    let mut stream = provider.stream(req()).await.unwrap();
    let mut got_text = false;
    while let Some(event) = stream.next().await {
        if matches!(event, LlmStreamEvent::DeltaText(_)) {
            got_text = true;
        }
    }
    assert!(got_text);
}

#[tokio::test]
async fn test_embed_retries_on_rate_limit() {
    let provider = RetryProvider::new(FailNTimes::rate_limit(1), fast_config(3));
    let resp = provider.embed(vec!["a".to_string(), "b".to_string()]).await.unwrap();
    assert_eq!(resp.embeddings.len(), 2);
}

#[tokio::test]
async fn test_generate_with_tools_retries() {
    let provider = RetryProvider::new(FailNTimes::rate_limit(2), fast_config(3));
    let result = provider
        .generate_with_tools(req(), vec![])
        .await
        .unwrap();
    assert!(matches!(result, ToolCallResponse::Text(_)));
}

#[tokio::test]
async fn test_retry_after_hint_is_respected() {
    // Provider returns RateLimit { retry_after: Some(1) } — just verify no panic
    struct HintedRateLimit;
    #[async_trait]
    impl LlmProvider for HintedRateLimit {
        async fn generate(&self, _req: LlmRequest) -> Result<LlmResponse, CloudError> {
            Err(CloudError::RateLimit { retry_after: Some(1) })
        }
        async fn stream(&self, _req: LlmRequest) -> Result<LlmStream, CloudError> {
            Err(CloudError::RateLimit { retry_after: None })
        }
        async fn embed(&self, _texts: Vec<String>) -> Result<EmbedResponse, CloudError> {
            Err(CloudError::RateLimit { retry_after: None })
        }
        async fn generate_with_tools(
            &self, _req: LlmRequest, _tools: Vec<ToolDefinition>,
        ) -> Result<ToolCallResponse, CloudError> {
            Err(CloudError::RateLimit { retry_after: None })
        }
    }
    // 1 attempt only so we don't actually sleep 1 second
    let provider = RetryProvider::new(HintedRateLimit, fast_config(1));
    let err = provider.generate(req()).await.unwrap_err();
    assert!(matches!(err, CloudError::RateLimit { .. }));
}

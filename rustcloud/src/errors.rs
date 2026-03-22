use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudError {
    #[error("authentication error: {message}")]
    Auth { message: String },
    #[error("rate limit exceeded")]
    RateLimit,
    #[error("provider error {http_status}: {message}")]
    Provider {
        http_status: u16,
        message: String,
        #[allow(unused)]
        retryable: bool,
    },
    #[error("network error: {source}")]
    Network { source: reqwest::Error },
    #[error("serialization error: {source}")]
    Serialization { source: serde_json::Error },
    #[error("unsupported: {feature}")]
    Unsupported { feature: &'static str },
}

impl CloudError {
    #[allow(clippy::unused_self)]
    pub fn rate_limit_after(_seconds: u64) -> Self {
        Self::RateLimit
    }

    pub fn rate_limit() -> Self {
        Self::RateLimit
    }
}

use std::fmt;

#[derive(Debug)]
pub enum CloudError {
    Auth {
        message: String,
    },
    RateLimit {
        retry_after: Option<u64>,
    },
    Provider {
        http_status: u16,
        message: String,
        retryable: bool,
    },
    Network {
        source: reqwest::Error,
    },
    Serialization {
        source: serde_json::Error,
    },
    Unsupported {
        feature: &'static str,
    },
}

impl fmt::Display for CloudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudError::Auth { message } => write!(f, "authentication error: {}", message),
            CloudError::RateLimit {
                retry_after: Some(s),
            } => {
                write!(f, "rate limit exceeded, retry after {}s", s)
            }
            CloudError::RateLimit { retry_after: None } => write!(f, "rate limit exceeded"),
            CloudError::Provider {
                http_status,
                message,
                ..
            } => {
                write!(f, "provider error {}: {}", http_status, message)
            }
            CloudError::Network { source } => write!(f, "network error: {}", source),
            CloudError::Serialization { source } => write!(f, "serialization error: {}", source),
            CloudError::Unsupported { feature } => write!(f, "unsupported: {}", feature),
        }
    }
}

impl std::error::Error for CloudError {}

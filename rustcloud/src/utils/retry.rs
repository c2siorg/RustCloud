use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

use crate::errors::CloudError;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Default::default()
        }
    }

    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }
}

fn calculate_delay(attempt: u32, config: &RetryConfig) -> Duration {
    let delay_ms = (config.initial_delay.as_millis() as f64 * config.multiplier.powi(attempt as i32)) as u64;
    Duration::from_millis(delay_ms.min(config.max_delay.as_millis() as u64))
}

pub async fn retry<F, T, E>(config: RetryConfig, mut operation: F) -> Result<T, CloudError>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>>>>,
    E: Into<CloudError>,
{
    let mut last_error: Option<CloudError> = None;

    for attempt in 0..config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                let error: CloudError = e.into();
                
                if !is_retryable(&error) {
                    return Err(error);
                }

                last_error = Some(error);

                if attempt < config.max_attempts - 1 {
                    let delay = calculate_delay(attempt, &config);
                    sleep(delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or(CloudError::Provider {
        http_status: 0,
        message: "max retry attempts reached".to_string(),
        retryable: true,
    }))
}

fn is_retryable(error: &CloudError) -> bool {
    match error {
        CloudError::Network { .. } => true,
        CloudError::Provider { retryable, .. } => *retryable,
        CloudError::RateLimit { .. } => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let config = RetryConfig::new(3);
        let mut attempts = 0;

        let result = retry(config, || {
            attempts += 1;
            Box::pin(async { Ok::<_, CloudError>(42) })
        })
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let config = RetryConfig::new(3);
        let mut attempts = 0;

        let result = retry(config, || {
            attempts += 1;
            Box::pin(async {
                if attempts < 3 {
                    Err(CloudError::Network {
                        source: reqwest::Error::new(
                            reqwest::error::Kind::Request,
                            None,
                        ),
                    })
                } else {
                    Ok(42)
                }
            })
        })
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let config = RetryConfig::new(3);
        let mut attempts = 0;

        let result = retry(config, || {
            attempts += 1;
            Box::pin(async {
                Err(CloudError::Auth {
                    message: "invalid".to_string(),
                })
            })
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }
}

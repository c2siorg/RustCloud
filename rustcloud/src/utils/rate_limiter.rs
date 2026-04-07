use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use crate::errors::CloudError;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterInner>>,
}

#[derive(Debug)]
struct RateLimiterInner {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl RateLimiterInner {
    fn new(requests_per_second: f64, burst: Option<u32>) -> Self {
        let burst_size = burst.unwrap_or(requests_per_second as u32) as f64;
        Self {
            tokens: burst_size,
            max_tokens: burst_size,
            refill_rate: requests_per_second,
            last_refill: Instant::now(),
        }
    }
}

impl RateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimiterInner::new(requests_per_second, None))),
        }
    }

    pub fn with_burst(self, burst: u32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimiterInner::new(0.0, Some(burst)))),
        }
    }

    async fn refill(&self) {
        let mut inner = self.inner.lock().await;
        let elapsed = inner.last_refill.elapsed().as_secs_f64();
        let new_tokens = inner.tokens + (elapsed * inner.refill_rate);
        inner.tokens = new_tokens.min(inner.max_tokens);
        inner.last_refill = Instant::now();
    }

    pub async fn acquire(&self) -> Result<(), CloudError> {
        self.refill().await;
        
        let mut inner = self.inner.lock().await;
        
        if inner.tokens >= 1.0 {
            inner.tokens -= 1.0;
            Ok(())
        } else {
            let wait_time = (1.0 - inner.tokens) / inner.refill_rate;
            Err(CloudError::RateLimit {
                retry_after: Some(wait_time as u64),
            })
        }
    }

    pub async fn try_acquire(&self) -> bool {
        self.acquire().await.is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_second: f64,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10.0,
            burst_size: 20,
        }
    }
}

impl RateLimitConfig {
    pub fn new(requests_per_second: f64) -> Self {
        Self {
            requests_per_second,
            burst_size: requests_per_second as u32 * 2,
        }
    }

    pub fn with_burst(mut self, burst: u32) -> Self {
        self.burst_size = burst;
        self
    }

    pub fn build(&self) -> RateLimiter {
        RateLimiter::new(self.requests_per_second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_requests() {
        let limiter = RateLimiter::new(10.0);
        
        for _ in 0..10 {
            let result = limiter.acquire().await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimiter::new(1.0);
        
        let _ = limiter.acquire().await;
        let result = limiter.acquire().await;
        
        assert!(result.is_err());
        if let Err(CloudError::RateLimit { retry_after }) = result {
            assert!(retry_after.is_some());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_burst() {
        let limiter = RateLimiter::new(1.0).with_burst(5);
        
        for _ in 0..5 {
            let result = limiter.acquire().await;
            assert!(result.is_ok());
        }
    }
}

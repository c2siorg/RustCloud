use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::errors::CloudError;

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub healthy: bool,
    pub latency_ms: Option<u64>,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    pub timeout: Duration,
    pub expected_status: u16,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            expected_status: 200,
        }
    }
}

impl HealthCheckConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_expected_status(mut self, status: u16) -> Self {
        self.expected_status = status;
        self
    }
}

pub struct HealthChecker {
    checks: Vec<ProviderHealthCheck>,
}

struct ProviderHealthCheck {
    name: String,
    check_fn: Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = HealthStatus> + Send>> + Send>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    pub fn add_check<F, Fut>(mut self, name: impl Into<String>, check_fn: F) -> Self
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = HealthStatus> + Send + 'static,
    {
        self.checks.push(ProviderHealthCheck {
            name: name.into(),
            check_fn: Box::new(move || Box::pin(check_fn())),
        });
        self
    }

    pub async fn check_all(&self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();

        for check in &self.checks {
            let status = (check.check_fn)().await;
            results.insert(check.name.clone(), status);
        }

        results
    }

    pub async fn is_healthy(&self) -> bool {
        let results = self.check_all().await;
        results.values().all(|s| s.healthy)
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn check_tcp_port(host: &str, port: u16, timeout: Duration) -> HealthStatus {
    let start = Instant::now();
    
    match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(format!("{}:{}", host, port))).await {
        Ok(Ok(_stream)) => HealthStatus {
            healthy: true,
            latency_ms: Some(start.elapsed().as_millis() as u64),
            message: Some(format!("Connected to {}:{}", host, port)),
        },
        Ok(Err(e)) => HealthStatus {
            healthy: false,
            latency_ms: None,
            message: Some(e.to_string()),
        },
        Err(_) => HealthStatus {
            healthy: false,
            latency_ms: None,
            message: Some("Connection timeout".to_string()),
        },
    }
}

pub async fn check_http_endpoint(url: &str, timeout: Duration) -> HealthStatus {
    let start = Instant::now();
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    match client.get(url).send().await {
        Ok(response) => {
            let latency = start.elapsed().as_millis() as u64;
            let healthy = response.status().is_success();
            HealthStatus {
                healthy,
                latency_ms: Some(latency),
                message: Some(format!("Status: {}", response.status())),
            }
        }
        Err(e) => HealthStatus {
            healthy: false,
            latency_ms: None,
            message: Some(e.to_string()),
        },
    }
}

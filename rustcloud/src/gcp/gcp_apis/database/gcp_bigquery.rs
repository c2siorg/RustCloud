use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::{header::AUTHORIZATION, Client};
use serde_json::json;
use tokio::sync::Mutex;

use crate::errors::CloudError;
use crate::gcp::gcp_apis::auth::gcp_auth::ServiceAccountTokenProvider;
use crate::gcp::types::database::gcp_bigquery_types::*;
use crate::traits::token_provider::TokenProvider;
use std::path::PathBuf;

struct CachedToken {
    value: String,
    expires_at: Instant,
}

pub struct BigQuery {
    client: Client,
    project_id: String,
    auth: Arc<dyn TokenProvider>,
    token: Mutex<CachedToken>,
}

impl BigQuery {
    pub async fn new(project_id: &str) -> Result<Self, CloudError> {
        let auth: Arc<dyn TokenProvider> = Arc::new(
            ServiceAccountTokenProvider::new(
                PathBuf::from("service-account.json"),
                vec!["https://www.googleapis.com/auth/cloud-platform".to_string()],
            )
            .map_err(|e| CloudError::Auth { message: e.to_string() })?,
        );
        let token = auth.get_token().await.map_err(|e| CloudError::Auth {
            message: e.to_string(),
        })?;
        Ok(Self {
            client: Client::new(),
            project_id: project_id.to_string(),
            auth,
            token: Mutex::new(CachedToken {
                value: token,
                expires_at: Instant::now() + Duration::from_secs(3600),
            }),
        })
    }

    pub fn with_http_client(
        client: Client,
        project_id: &str,
        auth: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.to_string(),
            auth,
            token: Mutex::new(CachedToken {
                value: String::new(),
                expires_at: Instant::now(),
            }),
        }
    }

    pub(crate) async fn get_token(&self) -> Result<String, CloudError> {
        let mut cached = self.token.lock().await;
        if cached.expires_at.saturating_duration_since(Instant::now()) < Duration::from_secs(300) {
            let fresh = self.auth.get_token().await.map_err(|e| CloudError::Auth {
                message: e.to_string(),
            })?;
            cached.value = fresh;
            cached.expires_at = Instant::now() + Duration::from_secs(3600);
        }
        Ok(cached.value.clone())
    }

    pub async fn create_dataset(&self, dataset_id: &str) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(&self.project_id, "datasets");
        let body = CreateDataset {
            dataset_reference: DatasetReference {
                project_id: self.project_id.clone(),
                dataset_id: dataset_id.to_string(),
            },
        };
        let token = self.get_token().await?;

        let response = self
            .client
            .post(&url)
            .json(&body)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn delete_dataset(
        &self,
        dataset_id: &str,
        delete_contents: bool,
    ) -> Result<serde_json::Value, CloudError> {
        let url = format!(
            "{}?deleteContents={}",
            bigquery_url(&self.project_id, &format!("datasets/{}", dataset_id)),
            delete_contents
        );
        let token = self.get_token().await?;

        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn list_datasets(&self) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(&self.project_id, "datasets");
        let token = self.get_token().await?;

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn create_table(
        &self,
        dataset_id: &str,
        table_id: &str,
        fields: Vec<TableField>,
    ) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(&self.project_id, &format!("datasets/{}/tables", dataset_id));
        let body = CreateTable {
            table_reference: TableReference {
                project_id: self.project_id.clone(),
                dataset_id: dataset_id.to_string(),
                table_id: table_id.to_string(),
            },
            schema: TableSchema { fields },
        };
        let token = self.get_token().await?;

        let response = self
            .client
            .post(&url)
            .json(&body)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn delete_table(
        &self,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(
            &self.project_id,
            &format!("datasets/{}/tables/{}", dataset_id, table_id),
        );
        let token = self.get_token().await?;

        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn list_tables(&self, dataset_id: &str) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(&self.project_id, &format!("datasets/{}/tables", dataset_id));
        let token = self.get_token().await?;

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }

    pub async fn run_query(&self, query: &str) -> Result<serde_json::Value, CloudError> {
        let url = bigquery_url(&self.project_id, "queries");
        let body = RunQuery {
            query: query.to_string(),
            use_legacy_sql: false,
        };
        let token = self.get_token().await?;

        let response = self
            .client
            .post(&url)
            .json(&body)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| CloudError::Network { source: e })?;

        let status = response.status().as_u16();
        let text = response.text().await.map_err(|e| CloudError::Network { source: e })?;
        if status >= 400 {
            return Err(map_bigquery_http_error(status, &text));
        }

        Ok(json!({ "status": status, "body": text }))
    }
}

pub(crate) fn bigquery_url(project_id: &str, path: &str) -> String {
    format!("https://bigquery.googleapis.com/bigquery/v2/projects/{}/{}", project_id, path)
}

pub(crate) fn map_bigquery_http_error(status: u16, body: &str) -> CloudError {
    match status {
        401 | 403 => CloudError::Auth { message: body.to_string() },
        404 | 409 => CloudError::Provider {
            http_status: status,
            message: body.to_string(),
            retryable: false,
        },
        429 => CloudError::RateLimit { retry_after: None },
        500 | 503 => CloudError::Provider {
            http_status: status,
            message: body.to_string(),
            retryable: true,
        },
        _ => CloudError::Provider {
            http_status: status,
            message: body.to_string(),
            retryable: status >= 500,
        },
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;

    fn no_creds_bigquery() -> BigQuery {
        BigQuery::with_http_client(
            Client::new(),
            "test-project",
            Arc::new(MockTokenProvider::new("test-token")),
        )
    }

    #[test]
    fn test_bigquery_url_simple_path() {
        assert_eq!(
            bigquery_url("my-project", "datasets"),
            "https://bigquery.googleapis.com/bigquery/v2/projects/my-project/datasets"
        );
    }

    #[test]
    fn test_bigquery_url_nested_path() {
        assert_eq!(
            bigquery_url("my-project", "datasets/ds1/tables/tbl1"),
            "https://bigquery.googleapis.com/bigquery/v2/projects/my-project/datasets/ds1/tables/tbl1"
        );
    }

    #[test]
    fn test_map_bigquery_http_error_auth() {
        match map_bigquery_http_error(401, "denied") {
            CloudError::Auth { message } => assert_eq!(message, "denied"),
            _ => panic!("expected Auth variant"),
        }
        match map_bigquery_http_error(403, "forbidden") {
            CloudError::Auth { .. } => {}
            _ => panic!("expected Auth variant"),
        }
    }

    #[test]
    fn test_map_bigquery_http_error_not_found_not_retryable() {
        match map_bigquery_http_error(404, "missing") {
            CloudError::Provider { http_status, retryable, .. } => {
                assert_eq!(http_status, 404);
                assert!(!retryable);
            }
            _ => panic!("expected Provider variant"),
        }
        match map_bigquery_http_error(409, "conflict") {
            CloudError::Provider { retryable, .. } => assert!(!retryable),
            _ => panic!("expected Provider variant"),
        }
    }

    #[test]
    fn test_map_bigquery_http_error_rate_limit() {
        match map_bigquery_http_error(429, "slow down") {
            CloudError::RateLimit { retry_after } => assert_eq!(retry_after, None),
            _ => panic!("expected RateLimit variant"),
        }
    }

    #[test]
    fn test_map_bigquery_http_error_server_error_retryable() {
        match map_bigquery_http_error(500, "boom") {
            CloudError::Provider { retryable, .. } => assert!(retryable),
            _ => panic!("expected Provider variant"),
        }
        match map_bigquery_http_error(503, "unavailable") {
            CloudError::Provider { retryable, .. } => assert!(retryable),
            _ => panic!("expected Provider variant"),
        }
    }

    #[tokio::test]
    async fn test_get_token_refreshes_when_expired() {
        let bq = no_creds_bigquery();
        let token = bq.get_token().await.unwrap();
        assert_eq!(token, "test-token");
    }
}

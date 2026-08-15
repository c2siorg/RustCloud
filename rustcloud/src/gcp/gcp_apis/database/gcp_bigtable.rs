use crate::gcp::gcp_apis::auth::gcp_auth::DefaultTokenProvider;
use crate::gcp::types::database::gcp_bigtable_types::*;
use crate::traits::token_provider::TokenProvider;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::json;
use serde_json::to_string;
use std::sync::Arc;

pub struct Bigtable {
    client: Client,
    base_url: String,
    project_id: String,
    auth: Arc<dyn TokenProvider>,
}

impl Bigtable {
    pub fn new(project_id: &str) -> Self {
        Self::with_http_client(
            Client::new(),
            project_id,
            "https://bigtableadmin.googleapis.com".to_string(),
            Arc::new(DefaultTokenProvider),
        )
    }

    pub fn with_http_client(
        client: Client,
        project_id: &str,
        base_url: String,
        auth: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            client,
            base_url,
            project_id: project_id.to_string(),
            auth,
        }
    }

    async fn get_authorization_header(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.auth.get_token().await.map_err(|e| e.to_string())?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn list_tables(
        &self,
        parent: &str,
        page_token: Option<&str>,
        view: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}/tables", self.base_url, parent);

        let mut request_builder = self.client.get(&url);
        if let Some(token) = page_token {
            request_builder = request_builder.query(&[("pageToken", token)]);
        }
        if let Some(view) = view {
            request_builder = request_builder.query(&[("view", view)]);
        }

        let auth_header = self.get_authorization_header().await?;
        let response = request_builder
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, auth_header)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({
            "status": status.as_u16(),
            "body": body,
        }))
    }

    pub async fn delete_tables(
        &self,
        name: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}", self.base_url, name);

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, auth_header)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({
            "status": status.as_u16(),
            "body": body,
        }))
    }

    pub async fn describe_tables(
        &self,
        name: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}", self.base_url, name);

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, auth_header)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({
            "status": status.as_u16(),
            "body": body,
        }))
    }

    pub async fn create_tables(
        &self,
        parent: &str,
        table_id: &str,
        table: Table,
        initial_splits: Vec<InitialSplits>,
        cluster_states: ClusterStates,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}/tables", self.base_url, parent);

        let create_bigtable = CreateBigtable {
            table_id: table_id.to_string(),
            table,
            initial_splits,
            cluster_states,
        };
        let body = to_string(&create_bigtable).unwrap();

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, auth_header)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({
            "status": status.as_u16(),
            "body": body,
        }))
    }
}

use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::database::gcp_bigtable_types::*;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::json;
use serde_json::to_string;

pub struct Bigtable {
    client: Client,
    base_url: String,
    project_id: String,
}

impl Bigtable {
    pub fn new(project_id: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://bigtableadmin.googleapis.com".to_string(),
            project_id: project_id.to_string(),
        }
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

        let token = retrieve_token().await?;
        let response = request_builder
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
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

        let token = retrieve_token().await?;
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
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

        let token = retrieve_token().await?;
        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
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

        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
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

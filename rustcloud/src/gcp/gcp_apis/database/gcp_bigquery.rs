use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::database::gcp_bigquery_types::*;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::{json, to_string};

pub struct BigQuery {
    client: Client,
    base_url: String,
    project_id: String,
}

impl BigQuery {
    pub fn new(project_id: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://bigquery.googleapis.com/bigquery/v2".to_string(),
            project_id: project_id.to_string(),
        }
    }

    pub async fn create_dataset(
        &self,
        dataset_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/projects/{}/datasets", self.base_url, self.project_id);

        let body = to_string(&CreateDataset {
            dataset_reference: DatasetReference {
                project_id: self.project_id.clone(),
                dataset_id: dataset_id.to_string(),
            },
        })?;

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

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn delete_dataset(
        &self,
        dataset_id: &str,
        delete_contents: bool,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/datasets/{}?deleteContents={}",
            self.base_url, self.project_id, dataset_id, delete_contents
        );

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

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn list_datasets(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/projects/{}/datasets", self.base_url, self.project_id);

        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn create_table(
        &self,
        dataset_id: &str,
        table_id: &str,
        fields: Vec<TableField>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables",
            self.base_url, self.project_id, dataset_id
        );

        let body = to_string(&CreateTable {
            table_reference: TableReference {
                project_id: self.project_id.clone(),
                dataset_id: dataset_id.to_string(),
                table_id: table_id.to_string(),
            },
            schema: TableSchema { fields },
        })?;

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

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn delete_table(
        &self,
        dataset_id: &str,
        table_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables/{}",
            self.base_url, self.project_id, dataset_id, table_id
        );

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

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn list_tables(
        &self,
        dataset_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/datasets/{}/tables",
            self.base_url, self.project_id, dataset_id
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }

    pub async fn run_query(
        &self,
        query: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/projects/{}/queries", self.base_url, self.project_id);

        let body = to_string(&RunQuery {
            query: query.to_string(),
            use_legacy_sql: false,
        })?;

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

        Ok(json!({ "status": status.as_u16(), "body": body }))
    }
}

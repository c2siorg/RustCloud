use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

pub struct AzureCosmosDb {
    client: Client,
    host: String,
    token: String,
}

impl AzureCosmosDb {
    pub fn new() -> Self {
        let host = env::var("AZURE_COSMOS_HOST").expect("AZURE_COSMOS_HOST not set");
        let token = env::var("AZURE_COSMOS_TOKEN").expect("AZURE_COSMOS_TOKEN not set");
        AzureCosmosDb {
            client: Client::new(),
            host,
            token,
        }
    }

    pub fn with_config(host: &str, token: &str) -> Self {
        AzureCosmosDb {
            client: Client::new(),
            host: host.to_string(),
            token: token.to_string(),
        }
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub async fn list_databases(&self) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs", self.host);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn create_database(&self, db_id: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs", self.host);
        let body = json!({ "id": db_id });

        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn delete_database(&self, db_id: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs/{}", self.host, db_id);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn create_container(
        &self,
        db_id: &str,
        container_id: &str,
        partition_key_path: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs/{}/colls", self.host, db_id);
        let body = json!({
            "id": container_id,
            "partitionKey": {
                "paths": [partition_key_path],
                "kind": "Hash"
            }
        });

        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn delete_container(
        &self,
        db_id: &str,
        container_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs/{}/colls/{}", self.host, db_id, container_id);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn upsert_document(
        &self,
        db_id: &str,
        container_id: &str,
        document: &Value,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs/{}/colls/{}/docs", self.host, db_id, container_id);
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .header("Content-Type", "application/json")
            .header("x-ms-documentdb-is-upsert", "true")
            .json(document)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn get_document(
        &self,
        db_id: &str,
        container_id: &str,
        doc_id: &str,
        partition_key: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!(
            "{}/dbs/{}/colls/{}/docs/{}",
            self.host, db_id, container_id, doc_id
        );
        let resp = self
            .client
            .get(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .header(
                "x-ms-documentdb-partitionkey",
                format!("[\"{}\"]", partition_key),
            )
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn query_documents(
        &self,
        db_id: &str,
        container_id: &str,
        query: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/dbs/{}/colls/{}/docs", self.host, db_id, container_id);
        let body = json!({
            "query": query,
            "parameters": []
        });
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .header("x-ms-version", "2018-12-31")
            .header("Content-Type", "application/query+json")
            .header("x-ms-documentdb-isquery", "true")
            .header("x-ms-documentdb-query-enablecrosspartition", "true")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await.unwrap_or(json!(null));
        Ok(json!({ "status": status, "body": body }))
    }
}

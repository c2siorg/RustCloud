use std::collections::HashMap;

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
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}/tables", self.base_url, parent);

        let mut request_builder = self.client.get(&url);
        if let Some(token) = page_token {
            request_builder = request_builder.query(&[("pageToken", token)]);
        }
        if let Some(view) = view {
            request_builder = request_builder.query(&[("view", view)]);
        }

        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
        let response = request_builder
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let response_text=  response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        println!("{:?}", body);
        let mut list_table_response = HashMap::new();
        list_table_response.insert("status".to_string(), status.as_u16().to_string());
        list_table_response.insert("body".to_string(), body);
        Ok(list_table_response)
    }

    pub async fn delete_tables(
        &self,
        name: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}", self.base_url, name);

        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;;

        let status = response.status();
        if !status.is_success() {
            let response_text=  response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        println!("{:?}", body);
        let mut delete_table_response = HashMap::new();
        delete_table_response.insert("status".to_string(), status.as_u16().to_string());
        delete_table_response.insert("body".to_string(), body);
        Ok(delete_table_response)
    }

    pub async fn describe_tables(
        &self,
        name: &str,
        mask: &str,
        table: Table,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}?updateMask={}", self.base_url, name, mask);
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let body = serde_json::to_string(&table)
            .map_err(|e| format!("Failed to serialize request body: {}", e))?;

        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        if !status.is_success() {
            let response_text=  response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        println!("{:?}", body);
        let mut describe_table_response = HashMap::new();
        describe_table_response.insert("status".to_string(), status.as_u16().to_string());
        describe_table_response.insert("body".to_string(), body);
        Ok(describe_table_response)
    }
    

    pub async fn create_table(
        &self,
        parent: &str,
        table_id: &str,
        table: Table,
        initial_splits: Option<Vec<InitialSplits>>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/{}/tables", self.base_url, parent);
    
        let create_bigtable = CreateBigtable {
            table_id: table_id.to_string(),
            table,
            initial_splits,
        };
    
        // Serialize the request body
        let body = serde_json::to_string(&create_bigtable)
            .map_err(|e| format!("Failed to serialize request body: {}", e))?;
    
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        println!("{:?}", body);
        let mut create_table_response = HashMap::new();
        create_table_response.insert("status".to_string(), status.as_u16().to_string());
        create_table_response.insert("body".to_string(), body);
        Ok(create_table_response)
    }
}
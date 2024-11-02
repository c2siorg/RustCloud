use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::network::gcp_dns_types::*;
use chrono;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::to_string;
use std::collections::HashMap;
use std::error::Error;
use serde_json::json;
const UNIX_DATE: &str = "%a %b %e %H:%M:%S %Z %Y";
const RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";
use std::time::SystemTime;
use std::time::UNIX_EPOCH;


pub struct GoogleDns {
    client: Client,
    base_url: String,
}

impl GoogleDns {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://www.googleapis.com".to_string(),
        }
    }

    async fn get_authorization_header(&self) -> Result<String, Box<dyn Error>> {
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn list_resource_dns_record_sets(
        &self,
        project_id: String,
        options: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}/rrsets",
            self.base_url, project_id, options["managedZone"]
        );
        let mut req = self.client.get(&url);

        if let Some(max_results) = options.get("maxResults") {
            req = req.query(&[("deviceName", *max_results)]);
        }

        if let Some(page_token) = options.get("pageToken") {
            req = req.query(&[("pageToken", *page_token)]);
        }

        if let Some(sort_by) = options.get("sortBy") {
            req = req.query(&[("sortBy", *sort_by)]);
        }

        if let Some(sort_order) = options.get("sortOrder") {
            req = req.query(&[("sortOrder", *sort_order)]);
        }

        let auth_header = self.get_authorization_header().await?;
        let response = req.header(AUTHORIZATION, auth_header).send().await.map_err(|e| format!("Failed to send request: {}", e))?;
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
        let mut list_dns_response = HashMap::new();
        list_dns_response.insert("status".to_string(), status.as_u16().to_string());
        list_dns_response.insert("body".to_string(), body);
        Ok(list_dns_response)
    }

    pub async fn create_dns(
        &self,
        project_id: String,
        param: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut option = CreateDns {
            creation_time: None,
            description: None,
            dns_name: None,
            name_servers: None,
            id: None,
            kind: None,
            name: None,
            name_server_set: None,
        };
        for (key, value) in param {
            match key.as_str() {
                "CreationTime" => {
                    if let Some(val) = value.as_str() {
                        option.creation_time = Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string());
                    }
                },
                "Description" => {
                    if let Some(val) = value.as_str() {
                        option.description = Some(val.to_string());
                    }
                },
                "DnsName" => {
                    if let Some(val) = value.as_str() {
                        option.dns_name = Some(val.to_string());
                    }
                },
                "nameServers" => {
                    if let Some(val) = value.as_array() {
                        option.name_servers = Some(val.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());
                    }
                },
                "Id" => {
                    if let Some(val) = value.as_str() {
                        option.id = Some(val.to_string());
                    }
                },
                "Kind" => {
                    if let Some(val) = value.as_str() {
                        option.kind = Some(val.to_string());
                    }
                },
                "Name" => {
                    if let Some(val) = value.as_str() {
                        option.name = Some(val.to_string());
                    }
                },
                "nameServerSet" => {
                    if let Some(val) = value.as_str() {
                        option.name_server_set = Some(val.to_string());
                    }
                },
                _ => {}
            }
        }

        // let create_dns_json = serde_json::to_value(option)?;


        let body = to_string(&option).map_err(|e| format!("Failed to serialize request body: {}", e))?;
        let url = format!("{}/dns/v1/projects/{}/managedZones", self.base_url, project_id);

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
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
        let mut create_dns_response = HashMap::new();
        create_dns_response.insert("status".to_string(), status.as_u16().to_string());
        create_dns_response.insert("body".to_string(), body);
        Ok(create_dns_response)

    }

    pub async fn list_dns(
        &self,
        project_id: String,
        options: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/",
            self.base_url, project_id
        );
        let mut req = self.client.get(&url);

        if let Some(max_results) = options.get("maxResults") {
            req = req.query(&[("deviceName", *max_results)]);
        }

        if let Some(page_token) = options.get("pageToken") {
            req = req.query(&[("pageToken", *page_token)]);
        }

        let auth_header = self.get_authorization_header().await?;
        let response = req.header(AUTHORIZATION, auth_header).send().await.map_err(|e| format!("Failed to send request: {}", e))?;
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
        let mut list_dns_response = HashMap::new();
        list_dns_response.insert("status".to_string(), status.as_u16().to_string());
        list_dns_response.insert("body".to_string(), body);
        Ok(list_dns_response)
    }

    pub async fn delete_dns(
        &self,
        project_id: String,
        options: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}",
            self.base_url, project_id, options["managedZone"]
        );

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await.map_err(|e| format!("Failed to send request: {}", e))?;
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
        let mut delete_dns_response = HashMap::new();
        delete_dns_response.insert("status".to_string(), status.as_u16().to_string());
        delete_dns_response.insert("body".to_string(), body);
        Ok(delete_dns_response)
    }
}

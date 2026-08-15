use crate::gcp::gcp_apis::auth::gcp_auth::DefaultTokenProvider;
use crate::gcp::types::network::gcp_dns_types::*;
use crate::traits::token_provider::TokenProvider;
use chrono;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::to_string;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

const UNIX_DATE: &str = "%a %b %e %H:%M:%S %Z %Y";
const RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

pub struct GoogleDns {
    client: Client,
    base_url: String,
    project: String,
    auth: Arc<dyn TokenProvider>,
}

impl GoogleDns {
    pub fn new(project: &str) -> Self {
        Self::with_http_client(
            Client::new(),
            project,
            "https://www.googleapis.com".to_string(),
            Arc::new(DefaultTokenProvider),
        )
    }

    pub fn with_http_client(
        client: Client,
        project: &str,
        base_url: String,
        auth: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            client,
            base_url,
            project: project.to_string(),
            auth,
        }
    }

    async fn get_authorization_header(&self) -> Result<String, Box<dyn Error>> {
        let token = self.auth.get_token().await.map_err(|e| e.to_string())?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn list_resource_dns_record_sets(
        &self,
        options: &HashMap<&str, &str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}/rrsets",
            self.base_url, self.project, options["managedZone"]
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
        let response = req.header(AUTHORIZATION, auth_header).send().await?;
        Ok(response)
    }

    pub async fn create_dns(
        &self,
        param: &HashMap<&str, &str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let project = param["Project"];
        let option = CreateDns {
            creation_time: chrono::Utc::now().to_rfc3339(),
            description: param["Description"].to_string(),
            dns_name: param["DnsName"].to_string(),
            name_servers: param["nameServers"]
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            id: param["Id"].to_string(),
            kind: param["Kind"].to_string(),
            name: param["Name"].to_string(),
            name_server_set: param["nameServerSet"].to_string(),
        };

        let body = to_string(&option).unwrap();
        let url = format!("{}/dns/v1/projects/{}/managedZones", self.base_url, project);

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn list_dns(
        &self,
        options: &HashMap<&str, &str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/",
            self.base_url, self.project
        );
        let mut req = self.client.get(&url);

        if let Some(max_results) = options.get("maxResults") {
            req = req.query(&[("deviceName", *max_results)]);
        }

        if let Some(page_token) = options.get("pageToken") {
            req = req.query(&[("pageToken", *page_token)]);
        }

        let auth_header = self.get_authorization_header().await?;
        let response = req.header(AUTHORIZATION, auth_header).send().await?;
        Ok(response)
    }

    pub async fn delete_dns(
        &self,
        options: &HashMap<&str, &str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}",
            self.base_url, self.project, options["managedZone"]
        );

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        Ok(response)
    }
}

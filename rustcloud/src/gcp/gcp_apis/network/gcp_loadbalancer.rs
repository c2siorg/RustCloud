use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::network::gcp_loadbalancer_types::*;
use chrono::Utc;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::to_string;
use std::collections::HashMap;
use std::error::Error;
use serde_json::json;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const UNIX_DATE: &str = "%a %b %e %H:%M:%S %Z %Y";
const RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

pub struct GoogleLoadBalancer {
    client: Client,
    base_url: String,
    project: String,
}

impl GoogleLoadBalancer {
    pub fn new(project: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://www.googleapis.com".to_string(),
            project: project.to_string(),
        }
    }

    async fn get_authorization_header(&self) -> Result<String, Box<dyn Error>> {
        let token = retrieve_token().await.map_err(|e| format!("Failed to retrieve token: {}", e))?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn create_load_balancer(
        &self,
        param: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut project = String::new();
        let mut region = String::new();
        let mut option = TargetPools {
            name: None,
            health_checks: None,
            description: None,
            backup_pool: None,
            failover_ratio: None,
            id: None,
            instances: None,
            kind: None,
            session_affinity: None,
            self_link: None,
            region: None,
            creation_timestamp: None,
        };

        for (key, value) in param.iter() {
            match key.as_str() {
                "Project" => {
                    if let Some(val) = value.as_str() {
                        project = val.to_string();
                    }
                },
                "Name" => {
                    if let Some(val) = value.as_str() {
                        option.name = Some(val.to_string());
                    }
                },
                "Region" => {
                    if let Some(val) = value.as_str() {
                        region = val.to_string();
                    }
                },
                "healthChecks" => {
                    if let Some(val) = value.as_array() {
                        option.health_checks = Some(val.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());
                    }
                },
                "description" => {
                    if let Some(val) = value.as_str() {
                        option.description = Some(val.to_string());
                    }
                },
                "BackupPool" => {
                    if let Some(val) = value.as_str() {
                        option.backup_pool = Some(val.to_string());
                    }
                },
                "failoverRatio" => {
                    if let Some(val) = value.as_f64() {
                        option.failover_ratio = Some(val);
                    }
                },
                "id" => {
                    if let Some(val) = value.as_str() {
                        option.id = Some(val.to_string());
                    }
                },
                "Instances" => {
                    if let Some(val) = value.as_array() {
                        option.instances = Some(val.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());
                    }
                },
                "kind" => {
                    if let Some(val) = value.as_str() {
                        option.kind = Some(val.to_string());
                    }
                },
                "sessionAffinity" => {
                    if let Some(val) = value.as_str() {
                        option.session_affinity = Some(val.to_string());
                    }
                },
                "selfLink" => {
                    if let Some(val) = value.as_str() {
                        option.self_link = Some(val.to_string());
                    }
                },
                _ => {}
            }
        }

        option.region = Some(format!("https://www.googleapis.com/compute/v1/projects/{}/zones/{}", project, region));

        option.creation_timestamp = Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string());

        let body = to_string(&option).map_err(|e| format!("Failed to serialize GCE instance: {}", e))?;
        let url = format!(
            "{}/compute/beta/projects/{}/regions/{}/targetPools",
            self.base_url, project, region
        );

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
        let mut create_loadbalancer_response = HashMap::new();
        create_loadbalancer_response.insert("status".to_string(), status.as_u16().to_string());
        create_loadbalancer_response.insert("body".to_string(), body);
        Ok(create_loadbalancer_response)
    }

    pub async fn delete_load_balancer(
        &self,
        options: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let url = format!(
            "{}/compute/beta/projects/{}/regions/{}/targetPools/{}",
            self.base_url, options["Project"], options["Region"], options["TargetPool"]
        );

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
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
        let mut delete_loadbalancer_response = HashMap::new();
        delete_loadbalancer_response.insert("status".to_string(), status.as_u16().to_string());
        delete_loadbalancer_response.insert("body".to_string(), body);
        Ok(delete_loadbalancer_response)
    }

    pub async fn list_load_balancer(
        &self,
        options: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let url = format!(
            "{}/compute/beta/projects/{}/regions/{}/targetPools",
            self.base_url, options["Project"], options["Region"]
        );

        let auth_header = self.get_authorization_header().await?;
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
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
        let mut list_loadbalancer_response = HashMap::new();
        list_loadbalancer_response.insert("status".to_string(), status.as_u16().to_string());
        list_loadbalancer_response.insert("body".to_string(), body);
        Ok(list_loadbalancer_response)
    }

    pub async fn attach_node_with_load_balancer(
        &self,
        param: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let project = param["Project"];
        let target_pool = param["TargetPool"];
        let region = param["Region"];
        let instances: Vec<&str> = param["Instances"].split(',').collect();

        let url = format!(
            "{}/compute/beta/projects/{}/regions/{}/targetPools/{}/addInstance",
            self.base_url, project, region, target_pool
        );

        let mut json_map = HashMap::new();
        let instance_list: Vec<HashMap<&str, &str>> = instances
            .into_iter()
            .map(|i| {
                let mut map = HashMap::new();
                map.insert("instance", i);
                map
            })
            .collect();
        json_map.insert("instances", instance_list);

        let body = to_string(&json_map).map_err(|e| format!("Failed to serialize request body: {}", e))?;

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
        let mut attach_node_with_load_balancer_response = HashMap::new();
        attach_node_with_load_balancer_response.insert("status".to_string(), status.as_u16().to_string());
        attach_node_with_load_balancer_response.insert("body".to_string(), body);
        Ok(attach_node_with_load_balancer_response)
    }

    pub async fn detach_node_with_load_balancer(
        &self,
        param: &HashMap<&str, &str>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let project = param["Project"];
        let target_pool = param["TargetPool"];
        let region = param["Region"];
        let instances: Vec<&str> = param["Instances"].split(',').collect();

        let url = format!(
            "{}/compute/beta/projects/{}/regions/{}/targetPools/{}/removeInstance",
            self.base_url, project, region, target_pool
        );

        let mut json_map = HashMap::new();
        let instance_list: Vec<HashMap<&str, &str>> = instances
            .into_iter()
            .map(|i| {
                let mut map = HashMap::new();
                map.insert("instance", i);
                map
            })
            .collect();
        json_map.insert("instances", instance_list);

        let body = to_string(&json_map).map_err(|e| format!("Failed to serialize request body: {}", e))?;

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
        let mut detach_node_with_load_balancer_response = HashMap::new();
        detach_node_with_load_balancer_response.insert("status".to_string(), status.as_u16().to_string());
        detach_node_with_load_balancer_response.insert("body".to_string(), body);
        Ok(detach_node_with_load_balancer_response)
    }
}

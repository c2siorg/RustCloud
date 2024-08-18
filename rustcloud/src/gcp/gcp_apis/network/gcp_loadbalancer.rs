use reqwest::{Client, header::AUTHORIZATION};
use serde_json::to_string;
use std::collections::HashMap;
use std::error::Error;
use chrono::Utc;
use crate::gcp::types::network::gcp_loadbalancer_types::*;
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;

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
        let token = retrieve_token().await?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn create_load_balancer(&self, param: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut option = TargetPools {
            name: "".to_string(),
            health_checks: Vec::new(),
            description: "".to_string(),
            backup_pool: "".to_string(),
            failover_ratio: 0,
            id: "".to_string(),
            instances: Vec::new(),
            kind: "".to_string(),
            session_affinity: "".to_string(),
            region: "".to_string(),
            self_link: "".to_string(),
            creation_timestamp: Utc::now().to_rfc3339(),
        };

        let project = param.get("Project").unwrap().to_string();
        let region = param.get("Region").unwrap().to_string();

        for (key, value) in param {
            match *key {
                "Name" => option.name = value.to_string(),
                "Region" => option.region = value.to_string(),
                "healthChecks" => option.health_checks = value.split(',').map(|s| s.to_string()).collect(),
                "description" => option.description = value.to_string(),
                "BackupPool" => option.backup_pool = value.to_string(),
                "failoverRatio" => option.failover_ratio = value.parse().unwrap(),
                "id" => option.id = value.to_string(),
                "Instances" => option.instances = value.split(',').map(|s| s.to_string()).collect(),
                "kind" => option.kind = value.to_string(),
                "sessionAffinity" => option.session_affinity = value.to_string(),
                "selfLink" => option.self_link = value.to_string(),
                _ => (),
            }
        }

        option.region = format!("https://www.googleapis.com/compute/v1/projects/{}/zones/{}", project, region);

        let body = to_string(&option)?;
        let url = format!("{}/compute/beta/projects/{}/regions/{}/targetPools", self.base_url, project, region);

        let auth_header = self.get_authorization_header().await?;
        let response = self.client.post(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        
        Ok(response)
    }

    pub async fn delete_load_balancer(&self, options: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/compute/beta/projects/{}/regions/{}/targetPools/{}", self.base_url, options["Project"], options["Region"], options["TargetPool"]);

        let auth_header = self.get_authorization_header().await?;
        let response = self.client.delete(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        
        Ok(response)
    }

    pub async fn list_load_balancer(&self, options: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/compute/beta/projects/{}/regions/{}/targetPools", self.base_url, options["Project"], options["Region"]);

        let auth_header = self.get_authorization_header().await?;
        let response = self.client.get(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        
        Ok(response)
    }

    pub async fn attach_node_with_load_balancer(&self, param: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let project = param["Project"];
        let target_pool = param["TargetPool"];
        let region = param["Region"];
        let instances: Vec<&str> = param["Instances"].split(',').collect();

        let url = format!("{}/compute/beta/projects/{}/regions/{}/targetPools/{}/addInstance", self.base_url, project, region, target_pool);

        let mut json_map = HashMap::new();
        let instance_list: Vec<HashMap<&str, &str>> = instances.into_iter().map(|i| {
            let mut map = HashMap::new();
            map.insert("instance", i);
            map
        }).collect();
        json_map.insert("instances", instance_list);

        let body = to_string(&json_map)?;

        let auth_header = self.get_authorization_header().await?;
        let response = self.client.post(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        
        Ok(response)
    }

    pub async fn detach_node_with_load_balancer(&self, param: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let project = param["Project"];
        let target_pool = param["TargetPool"];
        let region = param["Region"];
        let instances: Vec<&str> = param["Instances"].split(',').collect();

        let url = format!("{}/compute/beta/projects/{}/regions/{}/targetPools/{}/removeInstance", self.base_url, project, region, target_pool);

        let mut json_map = HashMap::new();
        let instance_list: Vec<HashMap<&str, &str>> = instances.into_iter().map(|i| {
            let mut map = HashMap::new();
            map.insert("instance", i);
            map
        }).collect();
        json_map.insert("instances", instance_list);

        let body = to_string(&json_map)?;

        let auth_header = self.get_authorization_header().await?;
        let response = self.client.post(&url)
            .header(AUTHORIZATION, auth_header)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        
        Ok(response)
    }
}

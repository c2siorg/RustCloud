use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::compute::gcp_container_types::*;
use reqwest::{header::AUTHORIZATION, Client, Error, Response};
use serde_json::to_string;
use std::collections::HashMap;

pub struct GCPContainerClient {
    client: Client,
    base_url: String,
}

impl GCPContainerClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://container.googleapis.com".to_string(),
        }
    }

    pub async fn create_cluster(
        &self,
        request: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut option = CreateCluster::default();
        let mut project_id = String::new();
        let mut zone = String::new();
    
        // Extract parameters from the request map
        for (key, value) in request {
            match key.as_str() {
                "Project" => {
                    project_id = value
                        .as_str()
                        .ok_or("Invalid or missing 'Project'")?
                        .to_string();
                }
                "Name" => {
                    option.name = value
                        .as_str()
                        .ok_or("Invalid or missing 'Name'")?
                        .to_string();
                }
                "Zone" => {
                    zone = value
                        .as_str()
                        .ok_or("Invalid or missing 'Zone'")?
                        .to_string();
                }
                "network" => {
                    option.network = value
                        .as_str()
                        .ok_or("Invalid or missing 'network'")?
                        .to_string();
                }
                "loggingService" => {
                    option.logging_service = value
                        .as_str()
                        .ok_or("Invalid or missing 'loggingService'")?
                        .to_string();
                }
                "monitoringService" => {
                    option.monitoring_service = value
                        .as_str()
                        .ok_or("Invalid or missing 'monitoringService'")?
                        .to_string();
                }
                "initialClusterVersion" => {
                    option.initial_cluster_version = value
                        .as_str()
                        .ok_or("Invalid or missing 'initialClusterVersion'")?
                        .to_string();
                }
                "subnetwork" => {
                    option.subnetwork = value
                        .as_str()
                        .ok_or("Invalid or missing 'subnetwork'")?
                        .to_string();
                }
                "masterAuth" => {
                    let master_auth = value
                        .as_object()
                        .ok_or("Invalid or missing 'masterAuth', expected an object")?;
    
                    if let Some(username) = master_auth.get("username") {
                        option.master_auth.username = username
                            .as_str()
                            .ok_or("Invalid or missing 'username' in 'masterAuth'")?
                            .to_string();
                    }
    
                    if let Some(client_certificate_config) =
                        master_auth.get("clientCertificateConfig")
                    {
                        let client_cert_config = client_certificate_config
                            .as_object()
                            .ok_or("Invalid or missing 'clientCertificateConfig', expected an object")?;
    
                        if let Some(issue_client_certificate) =
                            client_cert_config.get("issueClientCertificate")
                        {
                            option.master_auth.client_certificate_config.issue_client_certificate =
                                issue_client_certificate
                                    .as_bool()
                                    .ok_or("Invalid or missing 'issueClientCertificate'")?;
                        }
                    }
                }
                "nodePools" => {
                    let node_pools = value
                        .as_array()
                        .ok_or("Invalid 'nodePools' field, expected an array")?;
    
                    for node_pool_value in node_pools {
                        let node_pool_map = node_pool_value
                            .as_object()
                            .ok_or("Invalid 'nodePool' entry, expected an object")?;
                        let mut node_pool = NodePool::default();
    
                        if let Some(name) = node_pool_map.get("name") {
                            node_pool.name = name
                                .as_str()
                                .ok_or("Invalid or missing 'name' in 'nodePool'")?
                                .to_string();
                        }
    
                        if let Some(initial_node_count_value) = node_pool_map.get("initialNodeCount") {
                            node_pool.initial_node_count = match initial_node_count_value {
                                serde_json::Value::Number(n) => n.as_i64()
                                    .ok_or("Invalid 'initialNodeCount' in 'nodePool'")?
                                    .try_into()
                                    .map_err(|_| "Value out of i32 range")?,
                                serde_json::Value::String(s) => s.parse::<i64>()
                                    .map_err(|_| "Invalid string in 'initialNodeCount'")?,
                                _ => return Err("Invalid or missing 'initialNodeCount' in 'nodePool'".into()),
                            };
                        }

                        // if let Some(initial_node_count) = node_pool_map.get("initialNodeCount") {
                        //     // println!("{}", node_pool_map.get("initialNodeCount").as_i64().unwrap());
                        //     node_pool.initial_node_count = initial_node_count
                        //         .as_i64()
                        //         .ok_or("Invalid or missing 'initialNodeCount' in 'nodePool'")?
                        //         .try_into()
                        //         .map_err(|_| "Value out of i32 range")?;
                        // }
    
                        if let Some(config) = node_pool_map.get("config") {
                            let config_map = config
                                .as_object()
                                .ok_or("Invalid 'config' in 'nodePool', expected an object")?;
    
                            if let Some(machine_type) = config_map.get("machineType") {
                                node_pool.config.machine_type = machine_type
                                    .as_str()
                                    .ok_or("Invalid or missing 'machineType' in 'config'")?
                                    .to_string();
                            }
    
                            if let Some(image_type) = config_map.get("imageType") {
                                node_pool.config.image_type = image_type
                                    .as_str()
                                    .ok_or("Invalid or missing 'imageType' in 'config'")?
                                    .to_string();
                            }
    
                            if let Some(disk_size_gb) = config_map.get("diskSizeGb") {
                                node_pool.config.disk_size_gb = disk_size_gb
                                    .as_i64()
                                    .ok_or("Invalid or missing 'diskSizeGb' in 'config'")?
                                    as i32;
                            }
    
                            if let Some(preemptible) = config_map.get("preemptible") {
                                node_pool.config.preemptible = preemptible
                                    .as_bool()
                                    .ok_or("Invalid or missing 'preemptible' in 'config'")?;
                            }
    
                            if let Some(oauth_scopes) = config_map.get("oauthScopes") {
                                node_pool.config.oauth_scopes = oauth_scopes
                                    .as_array()
                                    .ok_or("Invalid or missing 'oauthScopes' in 'config', expected an array")?
                                    .iter()
                                    .map(|s| s.as_str().unwrap_or("").to_string())
                                    .collect();
                            }
                        }
    
                        if let Some(autoscaling) = node_pool_map.get("autoscaling") {
                            let autoscaling_map = autoscaling
                                .as_object()
                                .ok_or("Invalid 'autoscaling' in 'nodePool', expected an object")?;
    
                            if let Some(enabled) = autoscaling_map.get("enabled") {
                                node_pool.autoscaling.enabled = enabled
                                    .as_bool()
                                    .ok_or("Invalid or missing 'enabled' in 'autoscaling'")?;
                            }
                        }
    
                        if let Some(management) = node_pool_map.get("management") {
                            let management_map = management
                                .as_object()
                                .ok_or("Invalid 'management' in 'nodePool', expected an object")?;
    
                            if let Some(auto_upgrade) = management_map.get("autoUpgrade") {
                                node_pool.management.auto_upgrade = auto_upgrade
                                    .as_bool()
                                    .ok_or("Invalid or missing 'autoUpgrade' in 'management'")?;
                            }
    
                            if let Some(auto_repair) = management_map.get("AutoRepair") {
                                node_pool.management.auto_repair = auto_repair
                                    .as_bool()
                                    .ok_or("Invalid or missing 'AutoRepair' in 'management'")?;
                            }
                        }
    
                        option.node_pools.push(node_pool);
                    }
                }
                _ => {}
            }
        }
    
        option.zone = zone.clone();
    
        let mut create_cluster_json_map = serde_json::Map::new();
        create_cluster_json_map.insert("cluster".to_string(), serde_json::to_value(&option)?);
    
        let create_cluster_json = serde_json::to_string(&create_cluster_json_map)
            .map_err(|e| format!("Failed to serialize cluster: {}", e))?;
    
        let url = format!(
            "{}/v1/projects/{}/zones/{}/clusters",
            self.base_url, project_id, zone
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(create_cluster_json)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);
    
        let mut create_cluster_response = HashMap::new();
        create_cluster_response.insert("status".to_string(), status.as_u16().to_string());
        create_cluster_response.insert("body".to_string(), body);
    
        Ok(create_cluster_response)
    }
    
    pub async fn stop_task(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request
            .get("Project")
            .ok_or("Missing 'Project'")?
            .to_string();
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone'")?
            .to_string();
        let operation_id = request
            .get("OperationId")
            .ok_or("Missing 'OperationId'")?
            .to_string();
    
        let url = format!(
            "{}/v1/projects/{}/zones/{}/operations/{}:cancel",
            self.base_url, project_id, zone, operation_id
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        let mut stop_task_response = HashMap::new();
    
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            stop_task_response.insert("status".to_string(), status.as_u16().to_string());
            stop_task_response.insert("body".to_string(), response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);
    
        stop_task_response.insert("status".to_string(), status.as_u16().to_string());
        stop_task_response.insert("body".to_string(), body);
    
        Ok(stop_task_response)
    }
    
    
    pub async fn delete_cluster(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // Extract parameters from the request map
        let project_id = request
            .get("Project")
            .ok_or("Missing 'Project'")?;
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone'")?;
        let cluster_id = request
            .get("clusterId")
            .ok_or("Missing 'clusterId'")?;
    
        // Construct the URL for the request
        let url = format!(
            "{}/v1/projects/{}/zones/{}/clusters/{}",
            self.base_url, project_id, zone, cluster_id
        );
    
        // Retrieve the authentication token
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        // Create and send the HTTP request
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        // Check the HTTP response status
        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        // Parse the response body
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);
    
        // Construct the response map
        let mut delete_cluster_response = HashMap::new();
        delete_cluster_response.insert("status".to_string(), status.as_u16().to_string());
        delete_cluster_response.insert("body".to_string(), body);
    
        Ok(delete_cluster_response)
    }

    pub async fn create_service(
        &self,
        request: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut option = NodePoolService::default();
        let mut project_id = String::new();
        let mut cluster_id = String::new();
        let mut zone = String::new();
    
        // Extract parameters from the request map
        for (key, value) in request {
            match key.as_str() {
                "Project" => {
                    project_id = value
                        .as_str()
                        .ok_or("Invalid or missing 'Project'")?
                        .to_string();
                }
                "Name" => {
                    option.name = value
                        .as_str()
                        .ok_or("Invalid or missing 'Name'")?
                        .to_string();
                }
                "Zone" => {
                    zone = value
                        .as_str()
                        .ok_or("Invalid or missing 'Zone'")?
                        .to_string();
                }
                "clusterId" => {
                    cluster_id = value
                        .as_str()
                        .ok_or("Invalid or missing 'clusterId'")?
                        .to_string();
                }
                "statusMessage" => {
                    option.status_message = value
                        .as_str()
                        .ok_or("Invalid or missing 'statusMessage'")?
                        .to_string();
                }
                "initialNodeCount" => {
                    option.initial_node_count = value
                        .as_i64()
                        .ok_or("Invalid or missing 'initialNodeCount'")?
                        as i32;
                }
                "selfLink" => {
                    option.self_link = value
                        .as_str()
                        .ok_or("Invalid or missing 'selfLink'")?
                        .to_string();
                }
                "version" => {
                    option.version = value
                        .as_str()
                        .ok_or("Invalid or missing 'version'")?
                        .to_string();
                }
                "status" => {
                    option.status = value
                        .as_str()
                        .ok_or("Invalid or missing 'status'")?
                        .to_string();
                }
                "config" => {
                    let config = value
                        .as_object()
                        .ok_or("Invalid or missing 'config', expected an object")?;
    
                    if let Some(machine_type) = config.get("machineType") {
                        option.config.machine_type = machine_type
                            .as_str()
                            .ok_or("Invalid or missing 'machineType' in 'config'")?
                            .to_string();
                    }
    
                    if let Some(image_type) = config.get("imageType") {
                        option.config.image_type = image_type
                            .as_str()
                            .ok_or("Invalid or missing 'imageType' in 'config'")?
                            .to_string();
                    }
    
                    if let Some(disk_size_gb) = config.get("diskSizeGb") {
                        option.config.disk_size_gb = disk_size_gb
                            .as_i64()
                            .ok_or("Invalid or missing 'diskSizeGb' in 'config'")?
                            as i32;
                    }
    
                    if let Some(preemptible) = config.get("preemptible") {
                        option.config.preemptible = preemptible
                            .as_bool()
                            .ok_or("Invalid or missing 'preemptible' in 'config'")?;
                    }
    
                    if let Some(oauth_scopes) = config.get("oauthScopes") {
                        option.config.oauth_scopes = oauth_scopes
                            .as_array()
                            .ok_or("Invalid or missing 'oauthScopes' in 'config', expected an array")?
                            .iter()
                            .map(|s| s.as_str().unwrap_or("").to_string())
                            .collect();
                    }
    
                    if let Some(service_account) = config.get("ServiceAccount") {
                        option.config.service_account = service_account
                            .as_str()
                            .ok_or("Invalid or missing 'ServiceAccount' in 'config'")?
                            .to_string();
                    }
    
                    if let Some(local_ssd_count) = config.get("localSsdCount") {
                        option.config.local_ssd_count = local_ssd_count
                            .as_i64()
                            .ok_or("Invalid or missing 'localSsdCount' in 'config'")?
                            as i32;
                    }
                }
                "autoscaling" => {
                    let autoscaling = value
                        .as_object()
                        .ok_or("Invalid or missing 'autoscaling', expected an object")?;
    
                    if let Some(enabled) = autoscaling.get("enabled") {
                        option.autoscaling.enabled = enabled
                            .as_bool()
                            .ok_or("Invalid or missing 'enabled' in 'autoscaling'")?;
                    }
    
                    if let Some(min_node_count) = autoscaling.get("minNodeCount") {
                        option.autoscaling.min_node_count = min_node_count
                            .as_i64()
                            .ok_or("Invalid or missing 'minNodeCount' in 'autoscaling'")?
                            as i32;
                    }
    
                    if let Some(max_node_count) = autoscaling.get("maxNodeCount") {
                        option.autoscaling.max_node_count = max_node_count
                            .as_i64()
                            .ok_or("Invalid or missing 'maxNodeCount' in 'autoscaling'")?
                            as i32;
                    }
                }
                "instanceGroupUrls" => {
                    option.instance_group_urls = value
                        .as_array()
                        .ok_or("Invalid or missing 'instanceGroupUrls', expected an array")?
                        .iter()
                        .map(|s| s.as_str().unwrap_or("").to_string())
                        .collect();
                }
                "management" => {
                    let management = value
                        .as_object()
                        .ok_or("Invalid or missing 'management', expected an object")?;
    
                    if let Some(auto_upgrade) = management.get("autoUpgrade") {
                        option.management.auto_upgrade = auto_upgrade
                            .as_bool()
                            .ok_or("Invalid or missing 'autoUpgrade' in 'management'")?;
                    }
    
                    if let Some(auto_repair) = management.get("AutoRepair") {
                        option.management.auto_repair = auto_repair
                            .as_bool()
                            .ok_or("Invalid or missing 'AutoRepair' in 'management'")?;
                    }
                }
                _ => {}
            }
        }
    
        let mut create_service_json_map = serde_json::Map::new();
        create_service_json_map.insert(
            "nodePool".to_string(),
            serde_json::to_value(&option)?,
        );

    
        let create_service_json = serde_json::to_string(&create_service_json_map)
            .map_err(|e| format!("Failed to serialize node pool: {}", e))?;
    
        let url = format!(
            "{}/v1/projects/{}/zones/{}/clusters/{}/nodePools",
            self.base_url, project_id, zone, cluster_id
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(create_service_json)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);
    
        let mut create_service_response = HashMap::new();
        create_service_response.insert("status".to_string(), status.as_u16().to_string());
        create_service_response.insert("body".to_string(), body);
    
        Ok(create_service_response)
    }
    
    pub async fn delete_service(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request
            .get("Project")
            .ok_or("Missing 'Project'")?
            .to_string();
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone'")?
            .to_string();
        let cluster_id = request
            .get("clusterId")
            .ok_or("Missing 'clusterId'")?
            .to_string();
        let node_pool_id = request
            .get("nodePoolId")
            .ok_or("Missing 'nodePoolId'")?
            .to_string();
    
        let url = format!(
            "{}/v1/projects/{}/zones/{}/clusters/{}/nodePools/{}",
            self.base_url, project_id, zone, cluster_id, node_pool_id
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        let mut delete_service_response = HashMap::new();
        
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            delete_service_response.insert("status".to_string(), status.as_u16().to_string());
            delete_service_response.insert("body".to_string(), response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
    
        println!("{:?}", body);
    
        delete_service_response.insert("status".to_string(), status.as_u16().to_string());
        delete_service_response.insert("body".to_string(), body);
    
        Ok(delete_service_response)
    }
}

use reqwest::{header::AUTHORIZATION, Client, Method};
use serde_json::json;
use std::collections::HashMap;

use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;

pub struct GCE {
    client: Client,
    base_url: String,
}

impl GCE {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://compute.googleapis.com/compute/v1".to_string(),
        }
    }

    pub async fn create_node(
        &self,
        request: HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut project_id = String::new();
        let mut zone = String::new();
        let mut gce_instance = HashMap::new();
    
        // Iterate through the request to populate fields
        for (key, value) in request {
            match key.as_str() {
                "projectid" => {
                    project_id = value
                        .as_str()
                        .ok_or("Invalid or missing 'projectid'")?
                        .to_string();
                }
                "Zone" => {
                    zone = value.as_str().ok_or("Invalid or missing 'Zone'")?.to_string();
                    gce_instance.insert("zone", value);
                }
                "selfLink" => {
                    gce_instance.insert("selfLink", value);
                }
                "Description" => {
                    gce_instance.insert("description", value);
                }
                "CanIPForward" => {
                    gce_instance.insert("canIPForward", value);
                }
                "Name" => {
                    gce_instance.insert("name", value);
                }
                "MachineType" => {
                    gce_instance.insert("machineType", value);
                }
                "Disk" => {
                    let disk_param = value
                        .as_array()
                        .ok_or("Invalid 'disk' field, expected an array")?;
                    let mut disks = vec![];
    
                    for disk_value in disk_param {
                        let disk_map = disk_value
                            .as_object()
                            .ok_or("Invalid 'disk' entry, expected an object")?;
                        let mut disk = HashMap::new();
                        let mut initialize_param = HashMap::new();
    
                        for (disk_key, disk_val) in disk_map {
                            match disk_key.as_str() {
                                "Type" => {
                                    disk.insert("type", disk_val.clone());
                                }
                                "Boot" => {
                                    disk.insert("boot", disk_val.clone());
                                }
                                "Mode" => {
                                    disk.insert("mode", disk_val.clone());
                                }
                                "AutoDelete" => {
                                    disk.insert("autoDelete", disk_val.clone());
                                }
                                "DeviceName" => {
                                    disk.insert("deviceName", disk_val.clone());
                                }
                                "InitializeParams" => {
                                    let init_params = disk_val
                                        .as_object()
                                        .ok_or("Invalid 'InitializeParams', expected an object")?;
                                    initialize_param.insert(
                                        "sourceImage",
                                        init_params.get("SourceImage")
                                            .ok_or("Missing 'SourceImage' in 'InitializeParams'")?
                                            .clone(),
                                    );
                                    initialize_param.insert(
                                        "diskType",
                                        init_params.get("DiskType")
                                            .ok_or("Missing 'DiskType' in 'InitializeParams'")?
                                            .clone(),
                                    );
                                    initialize_param.insert(
                                        "diskSizeGb",
                                        init_params.get("DiskSizeGb")
                                            .ok_or("Missing 'DiskSizeGb' in 'InitializeParams'")?
                                            .clone(),
                                    );
                                    disk.insert("initializeParams", json!(initialize_param));
                                }
                                _ => {}
                            }
                        }
                        disks.push(json!(disk));
                    }
                    gce_instance.insert("disks", json!(disks));
                }
                "NetworkInterfaces" => {
                    let network_interfaces_param = value
                        .as_array()
                        .ok_or("Invalid 'NetworkInterfaces' field, expected an array")?;
                    let mut network_interfaces = vec![];
    
                    for network_interface_value in network_interfaces_param {
                        let network_interface_map = network_interface_value
                            .as_object()
                            .ok_or("Invalid 'NetworkInterface' entry, expected an object")?;
                        let mut network_interface = HashMap::new();
                        let mut access_configs = vec![];
    
                        for (network_interface_key, network_interface_val) in network_interface_map {
                            match network_interface_key.as_str() {
                                "Network" => {
                                    network_interface.insert("network", network_interface_val.clone());
                                }
                                "Subnetwork" => {
                                    network_interface
                                        .insert("subnetwork", network_interface_val.clone());
                                }
                                "AccessConfigs" => {
                                    let access_configs_param = network_interface_val
                                        .as_array()
                                        .ok_or("Invalid 'AccessConfigs', expected an array")?;
                                    for access_config_value in access_configs_param {
                                        let access_config_map = access_config_value
                                            .as_object()
                                            .ok_or("Invalid 'AccessConfig', expected an object")?;
                                        let mut access_config = HashMap::new();
                                        access_config.insert(
                                            "name",
                                            access_config_map
                                                .get("Name")
                                                .ok_or("Missing 'Name' in 'AccessConfig'")?
                                                .clone(),
                                        );
                                        access_config.insert(
                                            "type",
                                            access_config_map
                                                .get("Type")
                                                .ok_or("Missing 'Type' in 'AccessConfig'")?
                                                .clone(),
                                        );
                                        access_configs.push(json!(access_config));
                                    }
                                    network_interface.insert("accessConfigs", json!(access_configs));
                                }
                                _ => {}
                            }
                        }
                        network_interfaces.push(json!(network_interface));
                    }
                    gce_instance.insert("networkInterfaces", json!(network_interfaces));
                }
                "scheduling" => {
                    let scheduling_param = value
                        .as_object()
                        .ok_or("Invalid 'scheduling' field, expected an object")?;
                    let mut scheduling = HashMap::new();
    
                    for (scheduling_key, scheduling_val) in scheduling_param {
                        match scheduling_key.as_str() {
                            "Preemptible" => {
                                scheduling.insert("preemptible", scheduling_val.clone());
                            }
                            "onHostMaintenance" => {
                                scheduling.insert("onHostMaintenance", scheduling_val.clone());
                            }
                            "automaticRestart" => {
                                scheduling.insert("automaticRestart", scheduling_val.clone());
                            }
                            _ => {}
                        }
                    }
                    gce_instance.insert("scheduling", json!(scheduling));
                }
                _ => {}
            }
        }
    
        // Convert gce_instance to JSON string
        let gce_instance_json = serde_json::to_string(&gce_instance)
            .map_err(|e| format!("Failed to serialize GCE instance: {}", e))?;
    
        // Construct the URL for the request
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            self.base_url, project_id, zone
        );
    
        // Retrieve the authentication token
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        // Send the HTTP request
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(gce_instance_json)
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
        let mut create_node_response = HashMap::new();
        create_node_response.insert("status".to_string(), status.as_u16().to_string());
        create_node_response.insert("body".to_string(), body);
    
        Ok(create_node_response)
    }
    
    pub async fn start_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request
            .get("projectid")
            .ok_or("Missing 'projectid' in request")?;
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone' in request")?;
        let instance = request
            .get("instance")
            .ok_or("Missing 'instance' in request")?;
    
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/start",
            self.base_url, project_id, zone, instance
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let body = "";

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Content-Length", body.len().to_string())
            .header(AUTHORIZATION, format!("Bearer {}", token))
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
        let mut start_node_response = HashMap::new();
        start_node_response.insert("status".to_string(), status.as_u16().to_string());
        start_node_response.insert("body".to_string(), body);
    
        Ok(start_node_response)
    }
    
    pub async fn stop_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request
            .get("projectid")
            .ok_or("Missing 'projectid' in request")?;
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone' in request")?;
        let instance = request
            .get("instance")
            .ok_or("Missing 'instance' in request")?;
    
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/stop",
            self.base_url, project_id, zone, instance
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
        let body = "";
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Content-Length", body.len().to_string())
            .header(AUTHORIZATION, format!("Bearer {}", token))
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
        let mut stop_node_response = HashMap::new();
        stop_node_response.insert("status".to_string(), status.as_u16().to_string());
        stop_node_response.insert("body".to_string(), body);
    
        Ok(stop_node_response)
    }
    
    pub async fn delete_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid")
        .ok_or("Missing 'projectid' in request")?;
        let zone = request.get("Zone")
        .ok_or("Missing 'Zone' in request")?;
        let instance = request.get("instance")
        .ok_or("Missing 'instance' in request")?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.base_url, project_id, zone, instance
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

        let mut delete_node_response = HashMap::new();
        delete_node_response.insert("status".to_string(), status.as_u16().to_string());
        delete_node_response.insert("body".to_string(), body);
    
        Ok(delete_node_response)
    }

    pub async fn reboot_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid")
        .ok_or("Missing 'projectid' in request")?;
        let zone = request.get("Zone").ok_or("Missing 'Zone' in request")?;
        let instance = request.get("instance").ok_or("Missing 'instance' in request")?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/reset",
            self.base_url, project_id, zone, instance
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
        if !status.is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }

        let status = response.status().as_u16().to_string();
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;


        let mut reboot_node_response = HashMap::new();
        reboot_node_response.insert("status".to_string(), status);
        reboot_node_response.insert("body".to_string(), body);

        Ok(reboot_node_response)
    }

    pub async fn list_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // Retrieve project ID and zone from the request map
        let project_id = request
            .get("projectid")
            .ok_or("Missing 'projectid' in request")?;
        let zone = request
            .get("Zone")
            .ok_or("Missing 'Zone' in request")?;
    
        let url = format!(
            "{}/projects/{}/zones/{}/instances/",
            self.base_url, project_id, zone
        );
    
        let token = retrieve_token()
            .await
            .map_err(|e| format!("Failed to retrieve token: {}", e))?;
    
        let response = self
            .client
            .request(Method::GET, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;
    
        let status = response.status();
        if !status.is_success() {
            let response_text= response.text().await?;
            println!("{:?}", response_text);
            return Err(format!("Request failed with status: {}", status).into());
        }
    
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        
        println!("{:?}", body);

        let mut list_node_response = HashMap::new();
        list_node_response.insert("status".to_string(), status.as_u16().to_string());
        list_node_response.insert("body".to_string(), body);
    
        Ok(list_node_response)
    }
}

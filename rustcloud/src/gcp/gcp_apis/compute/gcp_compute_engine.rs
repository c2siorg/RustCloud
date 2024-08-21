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

        for (key, value) in request {
            match key.as_str() {
                "projectid" => project_id = value.as_str().unwrap().to_string(),
                "Zone" => {
                    zone = value.as_str().unwrap().to_string();
                    gce_instance.insert("Zone", value);
                }
                "selfLink" => {
                    gce_instance.insert("selfLink", value);
                }
                "Description" => {
                    gce_instance.insert("Description", value);
                }
                "CanIPForward" => {
                    gce_instance.insert("CanIPForward", value);
                }
                "Name" => {
                    gce_instance.insert("Name", value);
                }
                "MachineType" => {
                    gce_instance.insert("MachineType", value);
                }
                "disk" => {
                    let disk_param = value.as_array().unwrap();
                    let mut disks = vec![];

                    for disk_value in disk_param {
                        let disk_map = disk_value.as_object().unwrap();
                        let mut disk = HashMap::new();
                        let mut initialize_param = HashMap::new();

                        for (disk_key, disk_val) in disk_map {
                            match disk_key.as_str() {
                                "Type" => {
                                    disk.insert("Type", disk_val.clone());
                                }
                                "Boot" => {
                                    disk.insert("Boot", disk_val.clone());
                                }
                                "Mode" => {
                                    disk.insert("Mode", disk_val.clone());
                                }
                                "AutoDelete" => {
                                    disk.insert("AutoDelete", disk_val.clone());
                                }
                                "DeviceName" => {
                                    disk.insert("DeviceName", disk_val.clone());
                                }
                                "InitializeParams" => {
                                    let init_params = disk_val.as_object().unwrap();
                                    initialize_param
                                        .insert("SourceImage", init_params["SourceImage"].clone());
                                    initialize_param
                                        .insert("DiskType", init_params["DiskType"].clone());
                                    initialize_param
                                        .insert("DiskSizeGb", init_params["DiskSizeGb"].clone());
                                    disk.insert("InitializeParams", json!(initialize_param));
                                }
                                _ => {}
                            }
                        }
                        disks.push(json!(disk));
                    }
                    gce_instance.insert("Disks", json!(disks));
                }
                "NetworkInterfaces" => {
                    let network_interfaces_param = value.as_array().unwrap();
                    let mut network_interfaces = vec![];

                    for network_interface_value in network_interfaces_param {
                        let network_interface_map = network_interface_value.as_object().unwrap();
                        let mut network_interface = HashMap::new();
                        let mut access_configs = vec![];

                        for (network_interface_key, network_interface_val) in network_interface_map
                        {
                            match network_interface_key.as_str() {
                                "Network" => {
                                    network_interface
                                        .insert("Network", network_interface_val.clone());
                                }
                                "Subnetwork" => {
                                    network_interface
                                        .insert("Subnetwork", network_interface_val.clone());
                                }
                                "AccessConfigs" => {
                                    let access_configs_param =
                                        network_interface_val.as_array().unwrap();
                                    for access_config_value in access_configs_param {
                                        let access_config_map =
                                            access_config_value.as_object().unwrap();
                                        let mut access_config = HashMap::new();
                                        access_config
                                            .insert("Name", access_config_map["Name"].clone());
                                        access_config
                                            .insert("Type", access_config_map["Type"].clone());
                                        access_configs.push(json!(access_config));
                                    }
                                    network_interface
                                        .insert("AccessConfigs", json!(access_configs));
                                }
                                _ => {}
                            }
                        }
                        network_interfaces.push(json!(network_interface));
                    }
                    gce_instance.insert("NetworkInterfaces", json!(network_interfaces));
                }
                "scheduling" => {
                    let scheduling_param = value.as_object().unwrap();
                    let mut scheduling = HashMap::new();

                    for (scheduling_key, scheduling_val) in scheduling_param {
                        match scheduling_key.as_str() {
                            "Preemptible" => {
                                scheduling.insert("Preemptible", scheduling_val.clone());
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
                    gce_instance.insert("Scheduling", json!(scheduling));
                }
                _ => {}
            }
        }

        let gce_instance_json = serde_json::to_string(&gce_instance).unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            self.base_url, project_id, zone
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(gce_instance_json)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut create_node_response = HashMap::new();
        create_node_response.insert("status".to_string(), status);
        create_node_response.insert("body".to_string(), body);

        Ok(create_node_response)
    }

    pub async fn start_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid").unwrap();
        let zone = request.get("Zone").unwrap();
        let instance = request.get("instance").unwrap();
        let url = format!(
            "{}/v1/projects/{}/zones/{}/instances/{}/start",
            self.base_url, project_id, zone, instance
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut start_node_response = HashMap::new();
        start_node_response.insert("status".to_string(), status);
        start_node_response.insert("body".to_string(), body);

        Ok(start_node_response)
    }

    pub async fn stop_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid").unwrap();
        let zone = request.get("Zone").unwrap();
        let instance = request.get("instance").unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/stop",
            self.base_url, project_id, zone, instance
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut stop_node_response = HashMap::new();
        stop_node_response.insert("status".to_string(), status);
        stop_node_response.insert("body".to_string(), body);

        Ok(stop_node_response)
    }

    pub async fn delete_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid").unwrap();
        let zone = request.get("Zone").unwrap();
        let instance = request.get("instance").unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.base_url, project_id, zone, instance
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut delete_node_response = HashMap::new();
        delete_node_response.insert("status".to_string(), status);
        delete_node_response.insert("body".to_string(), body);

        Ok(delete_node_response)
    }

    pub async fn reboot_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid").unwrap();
        let zone = request.get("Zone").unwrap();
        let instance = request.get("instance").unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/reset",
            self.base_url, project_id, zone, instance
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut reboot_node_response = HashMap::new();
        reboot_node_response.insert("status".to_string(), status);
        reboot_node_response.insert("body".to_string(), body);

        Ok(reboot_node_response)
    }

    pub async fn list_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project_id = request.get("projectid").unwrap();
        let zone = request.get("Zone").unwrap();
        let url = format!(
            "{}/projects/{}/zones/{}/instances/",
            self.base_url, project_id, zone
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::GET, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut list_node_response = HashMap::new();
        list_node_response.insert("status".to_string(), status);
        list_node_response.insert("body".to_string(), body);

        Ok(list_node_response)
    }
}

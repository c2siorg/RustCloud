use reqwest::Client;
use crate::gcp::types::compute::gcp_compute_engine_type::*;
use serde_json::to_string;


pub struct GoogleCompute {
    client: Client,
    base_url: String,
    project_id: String,
}

impl GoogleCompute {
    pub fn new(base_url: &str, project_id: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            project_id: project_id.to_string(),
        }
    }

    pub async fn create_vm(&self, zone: &str, name: &str, os: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/compute/v1/projects/{}/zones/{}/instances", self.base_url, self.project_id, zone);
        let request = CreateVMRequest {
            name: name.to_string(),
            machine_type: format!("zones/{}/machineTypes/n1-standard-1", zone),
            disks: vec![Disk {
                boot: true,
                auto_delete: true,
                initialize_params: InitializeParams {
                    source_image: os.to_string(),
                },
            }],
            network_interfaces: vec![NetworkInterface {
                network: "global/networks/default".to_string(),
            }],
        };
        let body = to_string(&request).unwrap();
        let response = self.client.post(&url).body(body).send().await?;
        Ok(response)
    }

    pub async fn list_vms(&self, zone: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/compute/v1/projects/{}/zones/{}/instances", self.base_url, self.project_id, zone);
        let response = self.client.get(&url).send().await?;
        Ok(response)
    }

    pub async fn start_vm(&self, zone: &str, vm_name: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/compute/v1/projects/{}/zones/{}/instances/{}/start", self.base_url, self.project_id, zone, vm_name);
        let response = self.client.post(&url).send().await?;
        Ok(response)
    }

    pub async fn stop_vm(&self, zone: &str, vm_name: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/compute/v1/projects/{}/zones/{}/instances/{}/stop", self.base_url, self.project_id, zone, vm_name);
        let response = self.client.post(&url).send().await?;
        Ok(response)
    }

    pub async fn reboot_vm(&self, zone: &str, vm_name: &str) -> Result<reqwest::Response, reqwest::Error> {
        let stop_response = self.stop_vm(zone, vm_name).await?;
        // if stop_response.status().is_success() {
            let start_response = self.start_vm(zone, vm_name).await?;
            Ok(start_response)
        // } 
        // else {
        //     Err(reqwest::Error::new(
        //         reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        //         "Failed to stop VM",
        //     ))
        // }
    }

    pub async fn destroy_vm(&self, zone: &str, vm_name: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/compute/v1/projects/{}/zones/{}/instances/{}", self.base_url, self.project_id, zone, vm_name);
        let response = self.client.delete(&url).send().await?;
        Ok(response)
    }
}

fn check_params(params: &std::collections::HashMap<&str, &str>) -> Result<(), &'static str> {
    if params.is_empty() {
        return Err("Params cannot be empty");
    }
    Ok(())
}

use std::collections::HashMap;

use reqwest::{Client, Method};
use serde_json::{Map, Value};

use crate::gcp::errors::GcpApiError;
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::middleware::{
    array_from_json_field, object_from_json_field, required_json_string, required_string_from_map,
    send_authorized_json_request,
};

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
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone, payload) = build_create_node_payload(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            self.base_url, project_id, zone
        );

        self.execute_request(Method::POST, &url, Some(payload)).await
    }

    pub async fn start_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone, instance) = extract_node_target(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/start",
            self.base_url, project_id, zone, instance
        );

        self.execute_request(Method::POST, &url, None).await
    }

    pub async fn stop_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone, instance) = extract_node_target(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/stop",
            self.base_url, project_id, zone, instance
        );

        self.execute_request(Method::POST, &url, None).await
    }

    pub async fn delete_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone, instance) = extract_node_target(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.base_url, project_id, zone, instance
        );

        self.execute_request(Method::DELETE, &url, None).await
    }

    pub async fn reboot_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone, instance) = extract_node_target(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/reset",
            self.base_url, project_id, zone, instance
        );

        self.execute_request(Method::POST, &url, None).await
    }

    pub async fn list_node(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let (project_id, zone) = extract_project_and_zone(&request)?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/",
            self.base_url, project_id, zone
        );

        self.execute_request(Method::GET, &url, None).await
    }

    async fn execute_request(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<HashMap<String, String>, GcpApiError> {
        let token = retrieve_token()
            .await
            .map_err(|error| GcpApiError::Auth {
                message: error.to_string(),
            })?;

        send_authorized_json_request(&self.client, method, url, &token, body).await
    }
}

fn build_create_node_payload(
    request: &HashMap<String, Value>,
) -> Result<(String, String, String), GcpApiError> {
    let project_id = required_json_string(request, "projectid")?;
    let zone = required_json_string(request, "Zone")?;

    let mut gce_instance = Map::new();

    for key in [
        "Zone",
        "selfLink",
        "Description",
        "CanIPForward",
        "Name",
        "MachineType",
    ] {
        if let Some(value) = request.get(key) {
            gce_instance.insert(key.to_string(), value.clone());
        }
    }

    if let Some(disks) = request.get("disk") {
        gce_instance.insert("Disks".to_string(), parse_disks(disks)?);
    }

    if let Some(network_interfaces) = request.get("NetworkInterfaces") {
        gce_instance.insert(
            "NetworkInterfaces".to_string(),
            parse_network_interfaces(network_interfaces)?,
        );
    }

    if let Some(scheduling) = request.get("scheduling") {
        gce_instance.insert("Scheduling".to_string(), parse_scheduling(scheduling)?);
    }

    let payload = serde_json::to_string(&gce_instance)?;
    Ok((project_id, zone, payload))
}

fn parse_disks(disks: &Value) -> Result<Value, GcpApiError> {
    let disk_entries = array_from_json_field(disks, "disk")?;
    let mut out = Vec::with_capacity(disk_entries.len());

    for disk in disk_entries {
        let disk_map = object_from_json_field(disk, "disk[]")?;
        let mut parsed_disk = Map::new();

        for key in ["Type", "Boot", "Mode", "AutoDelete", "DeviceName"] {
            if let Some(value) = disk_map.get(key) {
                parsed_disk.insert(key.to_string(), value.clone());
            }
        }

        if let Some(initialize_params) = disk_map.get("InitializeParams") {
            let init_map = object_from_json_field(initialize_params, "InitializeParams")?;
            let mut parsed_init = Map::new();

            for key in ["SourceImage", "DiskType", "DiskSizeGb"] {
                if let Some(value) = init_map.get(key) {
                    parsed_init.insert(key.to_string(), value.clone());
                }
            }

            if !parsed_init.is_empty() {
                parsed_disk.insert("InitializeParams".to_string(), Value::Object(parsed_init));
            }
        }

        out.push(Value::Object(parsed_disk));
    }

    Ok(Value::Array(out))
}

fn parse_network_interfaces(network_interfaces: &Value) -> Result<Value, GcpApiError> {
    let interface_entries = array_from_json_field(network_interfaces, "NetworkInterfaces")?;
    let mut out = Vec::with_capacity(interface_entries.len());

    for network_interface in interface_entries {
        let interface_map = object_from_json_field(network_interface, "NetworkInterfaces[]")?;
        let mut parsed_interface = Map::new();

        for key in ["Network", "Subnetwork"] {
            if let Some(value) = interface_map.get(key) {
                parsed_interface.insert(key.to_string(), value.clone());
            }
        }

        if let Some(access_configs) = interface_map.get("AccessConfigs") {
            let config_entries = array_from_json_field(access_configs, "AccessConfigs")?;
            let mut parsed_configs = Vec::with_capacity(config_entries.len());

            for config in config_entries {
                let config_map = object_from_json_field(config, "AccessConfigs[]")?;
                let mut parsed_config = Map::new();

                for key in ["Name", "Type"] {
                    if let Some(value) = config_map.get(key) {
                        parsed_config.insert(key.to_string(), value.clone());
                    }
                }

                parsed_configs.push(Value::Object(parsed_config));
            }

            parsed_interface.insert("AccessConfigs".to_string(), Value::Array(parsed_configs));
        }

        out.push(Value::Object(parsed_interface));
    }

    Ok(Value::Array(out))
}

fn parse_scheduling(scheduling: &Value) -> Result<Value, GcpApiError> {
    let scheduling_map = object_from_json_field(scheduling, "scheduling")?;
    let mut out = Map::new();

    for key in ["Preemptible", "onHostMaintenance", "automaticRestart"] {
        if let Some(value) = scheduling_map.get(key) {
            out.insert(key.to_string(), value.clone());
        }
    }

    Ok(Value::Object(out))
}

fn extract_project_and_zone(
    request: &HashMap<String, String>,
) -> Result<(String, String), GcpApiError> {
    let project_id = required_string_from_map(request, "projectid")?;
    let zone = required_string_from_map(request, "Zone")?;
    Ok((project_id, zone))
}

fn extract_node_target(
    request: &HashMap<String, String>,
) -> Result<(String, String, String), GcpApiError> {
    let (project_id, zone) = extract_project_and_zone(request)?;
    let instance = required_string_from_map(request, "instance")?;
    Ok((project_id, zone, instance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_create_node_payload_serializes_nested_sections() {
        let mut request = HashMap::new();
        request.insert("projectid".to_string(), json!("demo-project"));
        request.insert("Zone".to_string(), json!("us-central1-a"));
        request.insert("Name".to_string(), json!("demo-node"));
        request.insert(
            "disk".to_string(),
            json!([{
                "Type": "PERSISTENT",
                "Boot": true,
                "InitializeParams": {
                    "SourceImage": "img",
                    "DiskType": "pd-standard",
                    "DiskSizeGb": "10"
                }
            }]),
        );
        request.insert(
            "NetworkInterfaces".to_string(),
            json!([{
                "Network": "default",
                "AccessConfigs": [
                    {"Name": "External NAT", "Type": "ONE_TO_ONE_NAT"}
                ]
            }]),
        );
        request.insert(
            "scheduling".to_string(),
            json!({
                "Preemptible": false,
                "automaticRestart": true
            }),
        );

        let (project, zone, payload) = build_create_node_payload(&request).unwrap();

        assert_eq!(project, "demo-project");
        assert_eq!(zone, "us-central1-a");

        let payload_json: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(payload_json["Name"], json!("demo-node"));
        assert_eq!(payload_json["Disks"][0]["Boot"], json!(true));
        assert_eq!(
            payload_json["NetworkInterfaces"][0]["AccessConfigs"][0]["Type"],
            json!("ONE_TO_ONE_NAT")
        );
        assert_eq!(payload_json["Scheduling"]["Preemptible"], json!(false));
    }

    #[test]
    fn build_create_node_payload_rejects_missing_project_id() {
        let mut request = HashMap::new();
        request.insert("Zone".to_string(), json!("us-central1-a"));

        let error = build_create_node_payload(&request).unwrap_err();

        assert!(matches!(
            error,
            GcpApiError::MissingField { field: "projectid" }
        ));
    }

    #[test]
    fn extract_node_target_rejects_missing_instance() {
        let mut request = HashMap::new();
        request.insert("projectid".to_string(), "demo-project".to_string());
        request.insert("Zone".to_string(), "us-central1-a".to_string());

        let error = extract_node_target(&request).unwrap_err();

        assert!(matches!(
            error,
            GcpApiError::MissingField { field: "instance" }
        ));
    }
}

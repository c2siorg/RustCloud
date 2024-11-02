use crate::gcp::gcp_apis::compute::gcp_compute_engine::*;
use serde_json::json;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> GCE {
    GCE::new()
}

#[tokio::test]
async fn test_create_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    let initialize_params = json!({
        "SourceImage": "https://www.googleapis.com/compute/v1/projects/debian-cloud/global/images/debian-8-jessie-v20160301",
        "DiskType": "projects/rare-daylight-403814/zones/us-east4-c/diskTypes/pd-standard",
        "DiskSizeGb": "10",
    });

    let disk = json!([{
        "Boot": true,
        "AutoDelete": false,
        "DeviceName": "bokya",
        "Type": "PERSISTENT",
        "Mode": "READ_WRITE",
        "InitializeParams": initialize_params,
    }]);

    let access_configs = json!([{
        "Name": "external-nat",
        "Type": "ONE_TO_ONE_NAT",
    }]);

    let network_interfaces = json!([{
        "Network": "global/networks/default",
        "Subnetwork": "projects/rare-daylight-403814/regions/us-east4/subnetworks/default",
        "AccessConfigs": access_configs,
    }]);
    request.insert("projectid".to_string(), json!("rare-daylight-403814"));
    request.insert("Zone".to_string(), json!("us-east4-c"));
    request.insert("Name".to_string(), json!("alpha-123-xyz"));
    request.insert("MachineType".to_string(), json!("zones/us-east4-c/machineTypes/n1-standard-1"));
    request.insert("Disk".to_string(), disk);
    request.insert("NetworkInterfaces".to_string(), network_interfaces);

    // Add other required fields for the create_node request here.

    let result = client.create_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    request.insert("Zone".to_string(), "us-east4-c".to_string());
    request.insert("instance".to_string(), "alpha-123-xyz".to_string());

    let result = client.start_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    request.insert("Zone".to_string(), "us-east4-c".to_string());
    request.insert("instance".to_string(), "alpha-123-xyz".to_string());

    let result = client.stop_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    request.insert("Zone".to_string(), "us-east4-c".to_string());
    request.insert("instance".to_string(), "alpha-123-xyz".to_string());

    let result = client.delete_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reboot_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    request.insert("Zone".to_string(), "asia-south2-a".to_string());
    request.insert("instance".to_string(), "alpha-123-xyz".to_string());

    let result = client.reboot_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_gcp_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    // request.insert("Zone".to_string(), "us-east2-4".to_string());
    request.insert("Zone".to_string(), "us-central1-b".to_string());

    let result = client.list_node(request).await;
    assert!(result.is_ok());
}

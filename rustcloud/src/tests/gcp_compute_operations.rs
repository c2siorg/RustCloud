use crate::gcp::gcp_apis::compute::gcp_compute_engine::*;
use serde_json::json;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> GCE {
    GCE::new()
}

#[tokio::test]
async fn test_create_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), json!("your_project_id"));
    request.insert("Zone".to_string(), json!("your_zone"));
    request.insert("Name".to_string(), json!("your_instance_name"));
    // Add other required fields for the create_node request here.

    let result = client.create_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());
    request.insert("instance".to_string(), "your_instance_name".to_string());

    let result = client.start_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());
    request.insert("instance".to_string(), "your_instance_name".to_string());

    let result = client.stop_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());
    request.insert("instance".to_string(), "your_instance_name".to_string());

    let result = client.delete_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reboot_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());
    request.insert("instance".to_string(), "your_instance_name".to_string());

    let result = client.reboot_node(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_node() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());

    let result = client.list_node(request).await;
    assert!(result.is_ok());
}

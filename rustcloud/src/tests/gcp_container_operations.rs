use std::collections::HashMap;

use crate::gcp::gcp_apis::compute::gcp_container::*;
use crate::gcp::types::compute::gcp_container_types::*;
use serde_json::json;
use tokio::test;


async fn create_client() -> GCPContainerClient {
    GCPContainerClient::new()
}

#[tokio::test]
async fn test_create_gcp_container_cluster() {
    let client = create_client().await;

    let nodepools = vec![
        HashMap::from([
            ("name".to_string(), "default-pool".to_string()),
            ("initialNodeCount".to_string(), serde_json::to_string(&1).unwrap()),
        ]),
    ];

    let mut request = HashMap::new();
    request.insert("Project".to_string(), json!("rare-daylight-403814".to_string()));
    request.insert("Name".to_string(), json!("cluster-3".to_string()));
    request.insert("Zone".to_string(), json!("us-central1-a".to_string()));
    request.insert("nodePools".to_string(), json!(nodepools));

    let result = client.create_cluster(request).await;
    println!("{:?}", result);
    assert!(result.is_ok(), "Failed to create cluster");
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_delete_gcp_container_cluster() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "rare-daylight-403814".to_string());
    request.insert("clusterId".to_string(), "cluster-3".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());

    let result = client.delete_cluster(request).await;
    assert!(result.is_ok(), "Failed to delete cluster");
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_create_service_for_gcp_container() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), json!("rare-daylight-403814".to_string()));
    request.insert("clusterId".to_string(), json!("cluster-3".to_string()));
    request.insert("Zone".to_string(), json!("us-central1-a".to_string()));
    request.insert("Name".to_string(), json!("nodepool".to_string()));
    request.insert("status".to_string(), json!("STATUS_UNSPECIFIED".to_string()));

    let result = client.create_service(request).await;
    assert!(result.is_ok(), "Failed to create service");
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_delete_service_for_gcp_container() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "rare-daylight-403814".to_string());
    request.insert("clusterId".to_string(), "cluster-3".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("nodePoolId".to_string(), "nodepool".to_string());

    let result = client.delete_service(request).await;
    assert!(result.is_ok(), "Failed to delete service");
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}
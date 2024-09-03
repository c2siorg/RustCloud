use crate::gcp::gcp_apis::network::gcp_loadbalancer::*;
use std::collections::HashMap;
use tokio::test;
use serde_json::json;


async fn create_client() -> GoogleLoadBalancer {
    GoogleLoadBalancer::new("rare-daylight-403814")
}

#[tokio::test]
async fn test_create_gcp_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project".to_string(), json!("rare-daylight-403814"));
    params.insert("Name".to_string(), json!("test-2b"));
    params.insert("Region".to_string(), json!("us-central1"));
    // params.insert("healthChecks", "healthCheck1,healthCheck2");
    // params.insert("description", "Test Load Balancer");
    // params.insert("BackupPool", "backup-pool");
    // params.insert("failoverRatio", "0.1");
    // params.insert("id", "12345");
    params.insert("Instances".to_string(), json!("https://www.googleapis.com/compute/v1/projects/rare-daylight-403814/zones/us-central1-b/instances/instance-20240902-124323"));
    // params.insert("kind", "compute#targetPool");
    // params.insert("sessionAffinity", "NONE");
    // params.insert("selfLink", "");

    let result = client.create_load_balancer(params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_delete_load_balancer() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("Project", "rare-daylight-403814");
    options.insert("Region", "us-central1");
    options.insert("TargetPool", "test-lb");

    let result = client.delete_load_balancer(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_list_gcp_load_balancer() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("Project", "rare-daylight-403814");
    options.insert("Region", "us-central1");

    let result = client.list_load_balancer(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_attach_node_with_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "rare-daylight-403814");
    params.insert("TargetPool", "test-2b");
    params.insert("Region", "us-central1");
    params.insert("Instances", "https://www.googleapis.com/compute/v1/projects/rare-daylight-403814/zones/us-east4-c/instances/alpha-123-xyz");

    let result = client.attach_node_with_load_balancer(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_detach_node_with_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "rare-daylight-403814");
    params.insert("TargetPool", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("Instances", "instance1,instance2");

    let result = client.detach_node_with_load_balancer(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

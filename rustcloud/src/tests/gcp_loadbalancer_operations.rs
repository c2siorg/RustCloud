use crate::gcp::gcp_apis::network::gcp_loadbalancer::*;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> GoogleLoadBalancer {
    GoogleLoadBalancer::new("your_project_id")
}

#[tokio::test]
async fn test_create_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "your_project_id");
    params.insert("Name", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("healthChecks", "healthCheck1,healthCheck2");
    params.insert("description", "Test Load Balancer");
    params.insert("BackupPool", "backup-pool");
    params.insert("failoverRatio", "0.1");
    params.insert("id", "12345");
    params.insert("Instances", "instance1,instance2");
    params.insert("kind", "compute#targetPool");
    params.insert("sessionAffinity", "NONE");
    params.insert("selfLink", "");

    let result = client.create_load_balancer(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_delete_load_balancer() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("Project", "your_project_id");
    options.insert("Region", "us-central1");
    options.insert("TargetPool", "test-lb");

    let result = client.delete_load_balancer(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_list_load_balancer() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("Project", "your_project_id");
    options.insert("Region", "us-central1");

    let result = client.list_load_balancer(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_attach_node_with_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "your_project_id");
    params.insert("TargetPool", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("Instances", "instance1,instance2");

    let result = client.attach_node_with_load_balancer(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_detach_node_with_load_balancer() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "your_project_id");
    params.insert("TargetPool", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("Instances", "instance1,instance2");

    let result = client.detach_node_with_load_balancer(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

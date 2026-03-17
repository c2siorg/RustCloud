use crate::digiocean::digiocean_apis::kubernetes::digiocean_kubernetes::DigiOceanKubernetes;
use serde_json::{json, Value};
use std::collections::HashMap;

fn create_client() -> DigiOceanKubernetes {
    DigiOceanKubernetes::new("test-token".to_string())
}

#[tokio::test]
async fn test_list_clusters() {
    let client = create_client();
    let result = client.list_clusters().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_cluster() {
    let client = create_client();
    let mut request: HashMap<String, Value> = HashMap::new();
    request.insert("name".to_string(), json!("rustcloud-test-cluster"));
    request.insert("region".to_string(), json!("nyc1"));
    request.insert("version".to_string(), json!("1.29.1-do.0"));
    request.insert(
        "node_pools".to_string(),
        json!([{
            "size": "s-2vcpu-2gb",
            "name": "worker-pool",
            "count": 2
        }]),
    );

    let result = client.create_cluster(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_cluster() {
    let client = create_client();
    let result = client.get_cluster("test-cluster-id").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_node_pool() {
    let client = create_client();
    let mut request: HashMap<String, Value> = HashMap::new();
    request.insert("name".to_string(), json!("extra-pool"));
    request.insert("size".to_string(), json!("s-4vcpu-8gb"));
    request.insert("count".to_string(), json!(3));

    let result = client.add_node_pool("test-cluster-id", request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_kubeconfig() {
    let client = create_client();
    let result = client.get_kubeconfig("test-cluster-id").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_node_pool() {
    let client = create_client();
    let result = client
        .delete_node_pool("test-cluster-id", "test-pool-id")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = create_client();
    let result = client.delete_cluster("test-cluster-id").await;
    assert!(result.is_ok());
}

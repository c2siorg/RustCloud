use crate::azure::azure_apis::compute::azure_kubernetes_service::AzureKubernetesServiceClient;

#[tokio::test]
async fn test_create_cluster() {
    let client = AzureKubernetesServiceClient::new();

    let result = client
        .create_cluster("test-rg", "test-aks-cluster", "eastasia")
        .await;

    match &result {
        Ok(_) => println!("AKS cluster created successfully"),
        Err(e) => println!("Azure AKS creation failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_clusters() {
    let client = AzureKubernetesServiceClient::new();

    let result = client.list_clusters("test-rg").await;

    match &result {
        Ok(_) => println!("AKS clusters listed successfully"),
        Err(e) => println!("Azure list AKS clusters failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_get_cluster() {
    let client = AzureKubernetesServiceClient::new();

    let result = client.get_cluster("test-rg", "test-aks-cluster").await;

    match &result {
        Ok(_) => println!("AKS cluster fetched successfully"),
        Err(e) => println!("Azure get AKS cluster failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = AzureKubernetesServiceClient::new();

    let result = client.delete_cluster("test-rg", "test-aks-cluster").await;

    match &result {
        Ok(_) => println!("AKS cluster deleted successfully"),
        Err(e) => println!("Azure AKS deletion failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_clusters_subscription() {
    let client = AzureKubernetesServiceClient::new();

    let result = client.list_clusters_subscription().await;

    match &result {
        Ok(_) => println!("AKS clusters in subscription listed successfully"),
        Err(e) => println!("Azure list subscription clusters failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_list_node_pools() {
    let client = AzureKubernetesServiceClient::new();

    let result = client.list_node_pools("test-rg", "test-aks-cluster").await;

    match &result {
        Ok(_) => println!("Node pools listed successfully"),
        Err(e) => println!("Azure list node pools failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_get_node_pool() {
    let client = AzureKubernetesServiceClient::new();

    let result = client
        .get_node_pool("test-rg", "test-aks-cluster", "nodepool1")
        .await;

    match &result {
        Ok(_) => println!("Node pool fetched successfully"),
        Err(e) => println!("Azure get node pool failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_scale_node_pool() {
    let client = AzureKubernetesServiceClient::new();

    let result = client
        .scale_node_pool("test-rg", "test-aks-cluster", "nodepool1", 2)
        .await;

    match &result {
        Ok(_) => println!("Node pool scaled successfully"),
        Err(e) => println!("Azure scale node pool failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

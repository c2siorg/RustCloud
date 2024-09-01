use crate::gcp::gcp_apis::compute::gcp_kubernetes::*;
use crate::gcp::types::compute::gcp_kubernetes_types::*;
use tokio::test;

async fn create_client() -> GCPKubernetesClient {
    GCPKubernetesClient::new()
}

#[tokio::test]
async fn test_create_cluster() {
    let client = create_client().await;

    let request = CreateClusterRequest {
        projectId: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        // Add other required fields for CreateClusterRequest here.
    };

    let result = client.create_cluster(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = create_client().await;

    let request = DeleteClusterRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        cluster_id: "your_cluster_id".to_string(),
    };

    let result = client.delete_cluster(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_clusters() {
    let client = create_client().await;

    let request = ListClustersRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
    };

    let result = client.list_clusters(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_cluster() {
    let client = create_client().await;

    let request = GetClusterRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        cluster_id: "your_cluster_id".to_string(),
    };

    let result = client.get_cluster(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_node_pool() {
    let client = create_client().await;

    let request = CreateNodePoolRequest {
        projectId: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        clusterId: "your_cluster_id".to_string(),
        // Add other required fields for CreateNodePoolRequest here.
    };

    let result = client.create_node_pool(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_node_pool() {
    let client = create_client().await;

    let request = DeleteNodePoolRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        cluster_id: "your_cluster_id".to_string(),
        node_pool_id: "your_node_pool_id".to_string(),
    };

    let result = client.delete_node_pool(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_node_pool() {
    let client = create_client().await;

    let request = GetNodePoolRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        cluster_id: "your_cluster_id".to_string(),
        node_pool_id: "your_node_pool_id".to_string(),
    };

    let result = client.get_node_pool(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_node_pools() {
    let client = create_client().await;

    let request = ListNodePoolsRequest {
        project_id: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        cluster_id: "your_cluster_id".to_string(),
    };

    let result = client.list_node_pools(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_addons_config() {
    let client = create_client().await;

    let request = SetAddonsConfigRequest {
        projectId: "your_project_id".to_string(),
        zone: "your_zone".to_string(),
        clusterId: "your_cluster_id".to_string(),
        // Add other required fields for SetAddonsConfigRequest here.
    };

    let result = client.set_addons_config(request).await;
    assert!(result.is_ok());
}

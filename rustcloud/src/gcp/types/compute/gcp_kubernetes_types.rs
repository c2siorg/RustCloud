use serde::{Deserialize, Serialize};



// Define request and response structs based on your API specification
#[derive(Debug, Serialize)]
struct CreateClusterRequest {
    project_id: String,
    zone: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
struct CreateClusterResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct DeleteClusterRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct ListClustersResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct ListClustersRequest {
    project_id: String,
    zone: String,
    // Add other fields as required
}

#[derive(Debug, Serialize)]
struct GetClusterRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct GetClusterResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct CreateNodePoolRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
struct CreateNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct DeleteNodePoolRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
    node_pool_id: String,
}

#[derive(Debug, Deserialize)]
struct GetNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct GetNodePoolRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
    node_pool_id: String,
}

#[derive(Debug, Serialize)]
struct ListNodePoolsRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct ListNodePoolsResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
struct SetAddonsConfigRequest {
    project_id: String,
    zone: String,
    cluster_id: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
struct SetAddonsConfigResponse {
    // Define fields based on response structure
}

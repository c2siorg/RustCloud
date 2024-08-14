use serde::{Deserialize, Serialize};


// Define request and response structs based on your API specification
#[derive(Debug, Serialize)]
pub struct CreateClusterRequest {
    pub project_id: String,
    pub zone: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
pub struct CreateClusterResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct DeleteClusterRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct ListClustersResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct ListClustersRequest {
    pub project_id: String,
    pub zone: String,
    // Add other fields as required
}

#[derive(Debug, Serialize)]
pub struct GetClusterRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct GetClusterResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct CreateNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
struct CreateNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct DeleteNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub node_pool_id: String,
}

#[derive(Debug, Deserialize)]
struct GetNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct GetNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub node_pool_id: String,
}

#[derive(Debug, Serialize)]
pub struct ListNodePoolsRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
struct ListNodePoolsResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
pub struct SetAddonsConfigRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    // Add other fields as required
}

#[derive(Debug, Deserialize)]
struct SetAddonsConfigResponse {
    // Define fields based on response structure
}

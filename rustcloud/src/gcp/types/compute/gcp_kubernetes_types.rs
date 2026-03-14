use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Define request and response structs based on your API specification
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClusterRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClusterRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ListClustersResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersRequest {
    pub project_id: String,
    pub zone: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClusterRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GetClusterResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub node_pool: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreateNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub node_pool_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GetNodePoolResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNodePoolRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub node_pool_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNodePoolsRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ListNodePoolsResponse {
    // Define fields based on response structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAddonsConfigRequest {
    pub project_id: String,
    pub zone: String,
    pub cluster_id: String,
    pub addons_config: HashMap<String, String>, // Add other fields as required
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SetAddonsConfigResponse {
    // Define fields based on response structure
}

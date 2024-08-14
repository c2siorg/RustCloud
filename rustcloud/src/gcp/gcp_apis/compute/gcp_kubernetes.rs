use reqwest::{Client, Error, Response};
use crate::gcp::types::compute::gcp_kubernetes_types::*;
use serde_json::to_string;



const GCP_BASE_URL: &str = "https://container.googleapis.com/v1";

pub struct GCPKubernetesClient {
    client: Client,
    access_token: String,
}

impl GCPKubernetesClient {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    pub async fn create_cluster(&self, request: CreateClusterRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters", GCP_BASE_URL, request.project_id, request.zone);
        let body = to_string(&request).unwrap();
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .body(body)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn delete_cluster(&self, request: DeleteClusterRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.delete(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn list_clusters(&self, request: ListClustersRequest) -> Result<reqwest:: Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters", GCP_BASE_URL, request.project_id, request.zone);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn get_cluster(&self, request: GetClusterRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn create_node_pool(&self, request: CreateNodePoolRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let body = to_string(&request).unwrap();
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .body(body)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn delete_node_pool(&self, request: DeleteNodePoolRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id, request.node_pool_id);
        let response = self.client.delete(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn get_node_pool(&self, request: GetNodePoolRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id, request.node_pool_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn list_node_pools(&self, request: ListNodePoolsRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn set_addons_config(&self, request: SetAddonsConfigRequest) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/setAddons", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let body = to_string(&request).unwrap();
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .body(body)
            .send()
            .await?;
        Ok(response)
    }
}


use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;


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

    pub async fn create_cluster(&self, request: CreateClusterRequest) -> Result<CreateClusterResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters", GCP_BASE_URL, request.project_id, request.zone);
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&request)
            .send()
            .await?;
        let result = response.json::<CreateClusterResponse>().await?;
        Ok(result)
    }

    pub async fn delete_cluster(&self, request: DeleteClusterRequest) -> Result<(), Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        self.client.delete(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn list_clusters(&self, request: ListClustersRequest) -> Result<ListClustersResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters", GCP_BASE_URL, request.project_id, request.zone);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let result = response.json::<ListClustersResponse>().await?;
        Ok(result)
    }

    pub async fn get_cluster(&self, request: GetClusterRequest) -> Result<GetClusterResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let result = response.json::<GetClusterResponse>().await?;
        Ok(result)
    }

    pub async fn create_node_pool(&self, request: CreateNodePoolRequest) -> Result<CreateNodePoolResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&request)
            .send()
            .await?;
        let result = response.json::<CreateNodePoolResponse>().await?;
        Ok(result)
    }

    pub async fn delete_node_pool(&self, request: DeleteNodePoolRequest) -> Result<(), Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id, request.node_pool_id);
        self.client.delete(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_node_pool(&self, request: GetNodePoolRequest) -> Result<GetNodePoolResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools/{}", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id, request.node_pool_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let result = response.json::<GetNodePoolResponse>().await?;
        Ok(result)
    }

    pub async fn list_node_pools(&self, request: ListNodePoolsRequest) -> Result<ListNodePoolsResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/nodePools", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let result = response.json::<ListNodePoolsResponse>().await?;
        Ok(result)
    }

    pub async fn set_addons_config(&self, request: SetAddonsConfigRequest) -> Result<SetAddonsConfigResponse, Error> {
        let url = format!("{}/projects/{}/zones/{}/clusters/{}/setAddons", GCP_BASE_URL, request.project_id, request.zone, request.cluster_id);
        let response = self.client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&request)
            .send()
            .await?;
        let result = response.json::<SetAddonsConfigResponse>().await?;
        Ok(result)
    }
}


use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use crate::gcp::types::compute::gcp_kubernetes_types::*;
use reqwest::{header::AUTHORIZATION, Client, Response};
use serde_json::to_string;

pub struct GCPKubernetesClient {
    client: Client,
    base_url: String,
}

impl GCPKubernetesClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://container.googleapis.com/v1beta1".to_string(),
        }
    }

    pub async fn create_cluster(
        &self,
        request: CreateClusterRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters",
            self.base_url, request.project_id, request.zone
        );
        let body = to_string(&request)?;
        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn delete_cluster(
        &self,
        request: DeleteClusterRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}",
            self.base_url, request.project_id, request.zone, request.cluster_id
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn list_clusters(
        &self,
        request: ListClustersRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters",
            self.base_url, request.project_id, request.zone
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn get_cluster(
        &self,
        request: GetClusterRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}",
            self.base_url, request.project_id, request.zone, request.cluster_id
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn create_node_pool(
        &self,
        request: CreateNodePoolRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}/nodePools",
            self.base_url, request.project_id, request.zone, request.cluster_id
        );
        let body = to_string(&request)?;
        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn delete_node_pool(
        &self,
        request: DeleteNodePoolRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}/nodePools/{}",
            self.base_url,
            request.project_id,
            request.zone,
            request.cluster_id,
            request.node_pool_id
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn get_node_pool(
        &self,
        request: GetNodePoolRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}/nodePools/{}",
            self.base_url,
            request.project_id,
            request.zone,
            request.cluster_id,
            request.node_pool_id
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn list_node_pools(
        &self,
        request: ListNodePoolsRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}/nodePools",
            self.base_url, request.project_id, request.zone, request.cluster_id
        );
        let token = retrieve_token().await?;
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        Ok(response)
    }

    pub async fn set_addons_config(
        &self,
        request: SetAddonsConfigRequest,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/projects/{}/locations/{}/clusters/{}:setAddons",
            self.base_url, request.project_id, request.zone, request.cluster_id
        );
        let body = to_string(&request)?;
        let token = retrieve_token().await?;
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;
        Ok(response)
    }
}

impl Default for GCPKubernetesClient {
    fn default() -> Self {
        Self::new()
    }
}

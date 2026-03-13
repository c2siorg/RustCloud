use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureKubernetesServiceClient {
    client: Client,
    subscription_id: String,
}

impl AzureKubernetesServiceClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureKubernetesServiceClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_cluster(
        &self,
        resource_group: &str,
        cluster_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            cluster_name
        );

        let body = serde_json::json!({
            "location": location,
            "identity": {
                "type": "SystemAssigned"
            },
            "properties": {
                "dnsPrefix": cluster_name,
                "agentPoolProfiles": [
                    {
                        "name": "nodepool1",
                        "count": 1,
                        "vmSize": "Standard_B2s",
                        "osType": "Linux",
                        "mode": "System",
                        "type": "VirtualMachineScaleSets"
                    }
                ],
                "networkProfile": {
                    "networkPlugin": "kubenet"
                },
                "enableRBAC": true
            }
        });

        let res = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure AKS creation failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_clusters(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters?api-version=2023-09-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure list AKS clusters failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_cluster(
        &self,
        resource_group: &str,
        cluster_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            cluster_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure get AKS cluster failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn delete_cluster(
        &self,
        resource_group: &str,
        cluster_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            cluster_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure AKS deletion failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_clusters_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.ContainerService/managedClusters?api-version=2023-09-01",
        self.subscription_id
    );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(
                format!("Azure list subscription AKS clusters failed: {}", body_text).into(),
            );
        }

        Ok(())
    }

    pub async fn list_node_pools(
        &self,
        resource_group: &str,
        cluster_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        cluster_name
    );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS NODE POOLS DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure list node pools failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_node_pool(
        &self,
        resource_group: &str,
        cluster_name: &str,
        pool_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools/{}?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        cluster_name,
        pool_name
    );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS NODE POOL DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure get node pool failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn scale_node_pool(
        &self,
        resource_group: &str,
        cluster_name: &str,
        pool_name: &str,
        count: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools/{}?api-version=2023-09-01",
        self.subscription_id,
        resource_group,
        cluster_name,
        pool_name
    );

        let body = serde_json::json!({
            "properties": {
                "count": count
            }
        });

        let res = self
            .client
            .put(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE AKS SCALE NODE POOL DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure scale node pool failed: {}", body_text).into());
        }

        Ok(())
    }
}

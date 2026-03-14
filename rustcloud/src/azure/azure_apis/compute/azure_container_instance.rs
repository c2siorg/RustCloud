use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureContainerInstanceClient {
    client: Client,
    subscription_id: String,
}

impl AzureContainerInstanceClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureContainerInstanceClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_container_group(
        &self,
        resource_group: &str,
        container_name: &str,
        location: &str,
        image: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups/{}?api-version=2023-05-01",
            self.subscription_id,
            resource_group,
            container_name
        );

        let body = serde_json::json!({
            "location": location,
            "properties": {
                "containers": [
                    {
                        "name": container_name,
                        "properties": {
                            "image": image,
                            "resources": {
                                "requests": {
                                    "cpu": 1.0,
                                    "memoryInGB": 1.5
                                }
                            },
                            "ports": [
                                {
                                    "port": 80
                                }
                            ]
                        }
                    }
                ],
                "osType": "Linux",
                "ipAddress": {
                    "type": "Public",
                    "ports": [
                        {
                            "protocol": "TCP",
                            "port": 80
                        }
                    ]
                }
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
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure container creation failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn list_container_groups(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups?api-version=2023-05-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure list containers failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn get_container_group(
        &self,
        resource_group: &str,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups/{}?api-version=2023-05-01",
            self.subscription_id,
            resource_group,
            container_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure get container failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn restart_container_group(
        &self,
        resource_group: &str,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups/{}/restart?api-version=2023-05-01",
            self.subscription_id,
            resource_group,
            container_name
        );

        let res = self
            .client
            .post(url)
            .bearer_auth(token)
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure restart container failed: {}", body_text).into());
        }

        Ok(())
    }

    pub async fn delete_container_group(
        &self,
        resource_group: &str,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups/{}?api-version=2023-05-01",
            self.subscription_id,
            resource_group,
            container_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body_text = res.text().await?;

        println!("-------------------------------");
        println!("AZURE API DEBUG");
        println!("Status  : {}", status);
        println!("Response: {}", body_text);
        println!("-------------------------------");

        if !status.is_success() {
            return Err(format!("Azure container deletion failed: {}", body_text).into());
        }

        Ok(())
    }
}

use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureLoadBalancerClient {
    client: Client,
    subscription_id: String,
}

impl AzureLoadBalancerClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_load_balancer(
        &self,
        resource_group: &str,
        lb_name: &str,
        location: &str,
        public_ip_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            lb_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": {
                "name": "Standard"
            },
            "properties": {
                "frontendIPConfigurations": [
                    {
                        "name": "LoadBalancerFrontEnd",
                        "properties": {
                            "publicIPAddress": {
                                "id": public_ip_id
                            }
                        }
                    }
                ]
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
        let body = res.text().await?;

        println!("AZURE CREATE LOAD BALANCER");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Load Balancer failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_load_balancers_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers?api-version=2023-09-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST LOAD BALANCERS RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Load Balancers RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_load_balancers_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Network/loadBalancers?api-version=2023-09-01",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST LOAD BALANCERS SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Load Balancers subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_load_balancer(
        &self,
        resource_group: &str,
        lb_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            lb_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET LOAD BALANCER");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Load Balancer failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_load_balancer(
        &self,
        resource_group: &str,
        lb_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            lb_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE LOAD BALANCER");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Load Balancer failed: {}", body).into());
        }

        Ok(())
    }
}

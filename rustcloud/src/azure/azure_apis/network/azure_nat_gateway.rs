use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureNatGatewayClient {
    client: Client,
    subscription_id: String,
}

impl AzureNatGatewayClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_nat_gateway(
        &self,
        resource_group: &str,
        nat_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}?api-version=2023-09-01",
            self.subscription_id, resource_group, nat_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": {
                "name": "Standard"
            },
            "properties": {
                "idleTimeoutInMinutes": 4
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

        println!("AZURE CREATE NAT GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create NAT Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_nat_gateways_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways?api-version=2023-09-01",
            self.subscription_id, resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST NAT GATEWAYS RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List NAT Gateways RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_nat_gateways_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Network/natGateways?api-version=2023-09-01",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST NAT GATEWAYS SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List NAT Gateways subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_nat_gateway(
        &self,
        resource_group: &str,
        nat_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}?api-version=2023-09-01",
            self.subscription_id, resource_group, nat_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET NAT GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get NAT Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_nat_gateway(
        &self,
        resource_group: &str,
        nat_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}?api-version=2023-09-01",
            self.subscription_id, resource_group, nat_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE NAT GATEWAY");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete NAT Gateway failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn associate_public_ip(
        &self,
        resource_group: &str,
        nat_name: &str,
        public_ip_id: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}?api-version=2023-09-01",
            self.subscription_id, resource_group, nat_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": { "name": "Standard" },
            "properties": {
                "publicIpAddresses": [
                    { "id": public_ip_id }
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

        println!("AZURE ASSOCIATE PUBLIC IP");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Associate Public IP failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn remove_public_ips(
        &self,
        resource_group: &str,
        nat_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}?api-version=2023-09-01",
            self.subscription_id, resource_group, nat_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": { "name": "Standard" },
            "properties": {
                "publicIpAddresses": []
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

        println!("AZURE REMOVE PUBLIC IPs");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Remove Public IPs failed: {}", body).into());
        }

        Ok(())
    }
}

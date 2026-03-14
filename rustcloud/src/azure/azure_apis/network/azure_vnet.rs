use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureVNetClient {
    client: Client,
    subscription_id: String,
}

impl AzureVNetClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_vnet(
        &self,
        resource_group: &str,
        vnet_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            vnet_name
        );

        let body = serde_json::json!({
            "location": location,
            "properties": {
                "addressSpace": {
                    "addressPrefixes": ["10.0.0.0/16"]
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
            return Err(format!("Azure VNet creation failed: {}", body_text).into());
        }

        Ok(())
    }
}

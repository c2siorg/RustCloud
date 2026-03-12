use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzurePublicIPClient {
    client: Client,
    subscription_id: String,
}

impl AzurePublicIPClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_public_ip(
        &self,
        resource_group: &str,
        ip_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/publicIPAddresses/{}?api-version=2023-09-01",
            self.subscription_id,
            resource_group,
            ip_name
        );

        let body = serde_json::json!({
            "location": location,
            "properties": {
                "publicIPAllocationMethod": "Dynamic"
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
            return Err(format!("Azure Public IP creation failed: {}", body_text).into());
        }

        Ok(())
    }
}

use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;

pub struct AzureDnsClient {
    client: Client,
    subscription_id: String,
}

impl AzureDnsClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        Self {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_dns_zone(
        &self,
        resource_group: &str,
        zone_name: &str,
        location: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name
        );

        let body = serde_json::json!({
            "location": location
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

        println!("AZURE CREATE DNS ZONE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create DNS Zone failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_dns_zones_rg(
        &self,
        resource_group: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones?api-version=2018-05-01",
            self.subscription_id,
            resource_group
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST DNS ZONES RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List DNS Zones RG failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_dns_zones_subscription(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Network/dnsZones?api-version=2018-05-01",
            self.subscription_id
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST DNS ZONES SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List DNS Zones subscription failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn get_dns_zone(
        &self,
        resource_group: &str,
        zone_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE GET DNS ZONE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get DNS Zone failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_dns_zone(
        &self,
        resource_group: &str,
        zone_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE DNS ZONE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete DNS Zone failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn create_a_record(
        &self,
        resource_group: &str,
        zone_name: &str,
        record_name: &str,
        ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/A/{}?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name,
            record_name
        );

        let body = serde_json::json!({
            "properties": {
                "TTL": 300,
                "ARecords": [
                    { "ipv4Address": ip_address }
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

        println!("AZURE CREATE A RECORD");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create A Record failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn list_record_sets(
        &self,
        resource_group: &str,
        zone_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/recordsets?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name
        );

        let res = self.client.get(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE LIST RECORD SETS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Record Sets failed: {}", body).into());
        }

        Ok(())
    }

    pub async fn delete_record(
        &self,
        resource_group: &str,
        zone_name: &str,
        record_type: &str,
        record_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/{}/{}?api-version=2018-05-01",
            self.subscription_id,
            resource_group,
            zone_name,
            record_type,
            record_name
        );

        let res = self.client.delete(url).bearer_auth(token).send().await?;

        let status = res.status();
        let body = res.text().await?;

        println!("AZURE DELETE RECORD");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Record failed: {}", body).into());
        }

        Ok(())
    }
}

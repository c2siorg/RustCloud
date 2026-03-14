use crate::azure::azure_apis::auth::azure_cli_auth::AzureCliAuth;
use reqwest::Client;
use std::env;
use std::error::Error;

pub struct AzureStorageAccountClient {
    client: Client,
    subscription_id: String,
}

impl AzureStorageAccountClient {
    pub fn new() -> Self {
        let subscription_id =
            env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID not set");

        AzureStorageAccountClient {
            client: Client::new(),
            subscription_id,
        }
    }

    pub async fn create_storage_account(
        &self,
        resource_group: &str,
        account_name: &str,
        location: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}?api-version=2023-01-01",
            self.subscription_id, resource_group, account_name
        );

        let body = serde_json::json!({
            "location": location,
            "sku": { "name": "Standard_LRS" },
            "kind": "StorageV2",
            "properties": {}
        });

        let response = self
            .client
            .put(&url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE CREATE STORAGE ACCOUNT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Storage Account failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn list_storage_accounts_resource_group(
        &self,
        resource_group: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts?api-version=2023-01-01",
            self.subscription_id, resource_group
        );

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE LIST STORAGE ACCOUNTS RG");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Storage Accounts RG failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn list_storage_accounts_subscription(&self) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Storage/storageAccounts?api-version=2023-01-01",
            self.subscription_id
        );

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE LIST STORAGE ACCOUNTS SUBSCRIPTION");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Storage Accounts Subscription failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn get_storage_account(
        &self,
        resource_group: &str,
        account_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}?api-version=2023-01-01",
            self.subscription_id, resource_group, account_name
        );

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE GET STORAGE ACCOUNT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Get Storage Account failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn list_storage_account_keys(
        &self,
        resource_group: &str,
        account_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/listKeys?api-version=2023-01-01",
        self.subscription_id, resource_group, account_name
    );

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE LIST STORAGE ACCOUNT KEYS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Storage Account Keys failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn check_storage_account_name(
        &self,
        account_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
    "https://management.azure.com/subscriptions/{}/providers/Microsoft.Storage/checkNameAvailability?api-version=2023-01-01",
    self.subscription_id
);

        let body = serde_json::json!({
            "name": account_name,
            "type": "Microsoft.Storage/storageAccounts"
        });

        let response = self
            .client
            .post(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE CHECK STORAGE ACCOUNT NAME");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Check Storage Account Name failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn delete_storage_account(
        &self,
        resource_group: &str,
        account_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let token = AzureCliAuth::get_token()?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}?api-version=2023-01-01",
            self.subscription_id, resource_group, account_name
        );

        let response = self.client.delete(&url).bearer_auth(token).send().await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE DELETE STORAGE ACCOUNT");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Storage Account failed: {}", body).into());
        }

        Ok(body)
    }
}

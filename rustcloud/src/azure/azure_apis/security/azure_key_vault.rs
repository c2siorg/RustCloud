use reqwest::{header::AUTHORIZATION, Client};
use serde_json::{json, Value};
use std::env;

const API_VERSION: &str = "7.4";

/// Client for Azure Key Vault secret operations.
///
/// Reads credentials from environment variables:
/// - `AZURE_KEYVAULT_URL`   — vault endpoint, e.g. `https://my-vault.vault.azure.net`
/// - `AZURE_KEYVAULT_TOKEN` — Azure AD bearer token with `secrets` permissions
pub struct AzureKeyVault {
    client: Client,
    vault_url: String,
    token: String,
}

impl AzureKeyVault {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            vault_url: env::var("AZURE_KEYVAULT_URL")
                .expect("AZURE_KEYVAULT_URL must be set"),
            token: env::var("AZURE_KEYVAULT_TOKEN")
                .expect("AZURE_KEYVAULT_TOKEN must be set"),
        }
    }

    pub fn with_config(vault_url: &str, token: &str) -> Self {
        Self {
            client: Client::new(),
            vault_url: vault_url.to_string(),
            token: token.to_string(),
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    /// Create or update a secret. Returns the full secret object from the vault.
    pub async fn set_secret(
        &self,
        name: &str,
        value: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/secrets/{}?api-version={}",
            self.vault_url, name, API_VERSION
        );
        let body = json!({ "value": value });

        let resp = self
            .client
            .put(&url)
            .header(AUTHORIZATION, self.auth_header())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("Set secret '{}': status {}", name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// Retrieve the latest version of a secret's value.
    pub async fn get_secret(
        &self,
        name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/secrets/{}?api-version={}",
            self.vault_url, name, API_VERSION
        );

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        let body: Value = resp.json().await?;
        let value = body["value"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        println!("Retrieved secret '{}'", name);
        Ok(value)
    }

    /// Soft-delete a secret (recoverable within the vault's retention period).
    pub async fn delete_secret(
        &self,
        name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/secrets/{}?api-version={}",
            self.vault_url, name, API_VERSION
        );

        let resp = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("Deleted secret '{}': status {}", name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// List all secret names in the vault (not their values).
    pub async fn list_secrets(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/secrets?api-version={}",
            self.vault_url, API_VERSION
        );

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        let body: Value = resp.json().await?;
        let names: Vec<String> = body["value"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|item| {
                item["id"].as_str().and_then(|id| {
                    id.split('/').rev().nth(1).map(|n| n.to_string())
                })
            })
            .collect();

        for name in &names {
            println!("Secret: {}", name);
        }
        Ok(names)
    }

    /// List all versions of a secret by name.
    pub async fn get_secret_versions(
        &self,
        name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/secrets/{}/versions?api-version={}",
            self.vault_url, name, API_VERSION
        );

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("Versions for '{}': status {}", name, status);
        Ok(json!({ "status": status, "body": body }))
    }
}

use async_trait::async_trait;
use gcp_auth::{CustomServiceAccount, TokenProvider as GcpTokenProvider};
use std::path::PathBuf;

use crate::traits::token_provider::TokenProvider;

pub async fn retrieve_token() -> Result<String, Box<dyn std::error::Error>> {
    let credentials_path = PathBuf::from("service-account.json");
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = service_account.token(scopes).await?;

    Ok(token.as_str().to_string())
}

pub struct ServiceAccountTokenProvider {
    account: CustomServiceAccount,
    scopes: Vec<String>,
}

impl ServiceAccountTokenProvider {
    pub fn new(
        credentials_path: PathBuf,
        scopes: Vec<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let account = CustomServiceAccount::from_file(credentials_path)?;
        Ok(Self { account, scopes })
    }
}

#[async_trait]
impl TokenProvider for ServiceAccountTokenProvider {
    async fn get_token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let scopes: Vec<&str> = self.scopes.iter().map(|s| s.as_str()).collect();
        let token = self.account.token(&scopes).await?;
        Ok(token.as_str().to_string())
    }
}

pub struct MockTokenProvider {
    token: String,
}

impl MockTokenProvider {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl TokenProvider for MockTokenProvider {
    async fn get_token(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.token.clone())
    }
}

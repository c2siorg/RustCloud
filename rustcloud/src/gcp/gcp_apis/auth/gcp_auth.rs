use gcp_auth::{CustomServiceAccount, TokenProvider};
use std::{os::unix::process, path::PathBuf};
use std::env;


pub async fn retrieve_token() -> Result<String, Box<dyn std::error::Error>> {
    let credentials_path = PathBuf::from(env::var("GOOGLE_APPLICATION_CREDENTIALS")?);
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = service_account.token(scopes).await?;

    Ok(token.as_str().to_string())
}
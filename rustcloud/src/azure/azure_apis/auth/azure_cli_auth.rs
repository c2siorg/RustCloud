use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct AzureToken {
    accessToken: String,
}

pub struct AzureCliAuth;

impl AzureCliAuth {
    pub fn get_token() -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("az.cmd")
            .args([
                "account",
                "get-access-token",
                "--resource",
                "https://management.azure.com/",
                "--output",
                "json",
            ])
            .output()?;

        if !output.status.success() {
            return Err("Failed to get Azure CLI token".into());
        }

        let json = String::from_utf8(output.stdout)?;

        let token: AzureToken = serde_json::from_str(&json)?;

        Ok(token.accessToken)
    }
}

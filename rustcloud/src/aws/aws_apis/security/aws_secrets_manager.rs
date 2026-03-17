#![allow(clippy::result_large_err)]

use aws_sdk_secretsmanager::{Client, Error};

/// Create a new secret and return its ARN.
/// `secret_string` should be a plaintext value or a JSON-encoded key/value map.
pub async fn create_secret(
    client: &Client,
    name: &str,
    secret_string: &str,
) -> Result<String, Error> {
    let resp = client
        .create_secret()
        .name(name)
        .secret_string(secret_string)
        .send()
        .await;
    match resp {
        Ok(result) => {
            let arn = result.arn().unwrap_or_default().to_string();
            println!("Secret ARN: {}", arn);
            Ok(arn)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Retrieve the current secret value by name or ARN.
/// Returns the secret string; use `get_secret_value` directly for binary secrets.
pub async fn get_secret(client: &Client, secret_id: &str) -> Result<String, Error> {
    let resp = client.get_secret_value().secret_id(secret_id).send().await;
    match resp {
        Ok(result) => {
            let value = result.secret_string().unwrap_or_default().to_string();
            println!("Retrieved secret: {}", secret_id);
            Ok(value)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Overwrite the secret value for an existing secret. Returns the secret ARN.
pub async fn update_secret(
    client: &Client,
    secret_id: &str,
    new_secret_string: &str,
) -> Result<String, Error> {
    let resp = client
        .update_secret()
        .secret_id(secret_id)
        .secret_string(new_secret_string)
        .send()
        .await;
    match resp {
        Ok(result) => {
            let arn = result.arn().unwrap_or_default().to_string();
            println!("Updated secret ARN: {}", arn);
            Ok(arn)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// Schedule a secret for deletion.
/// `recovery_window_days` (7–30) sets the grace period before permanent deletion.
/// Pass `None` to use the AWS default (30 days).
pub async fn delete_secret(
    client: &Client,
    secret_id: &str,
    recovery_window_days: Option<i64>,
) -> Result<String, Error> {
    let mut builder = client.delete_secret().secret_id(secret_id);
    if let Some(days) = recovery_window_days {
        builder = builder.recovery_window_in_days(days);
    }
    let resp = builder.send().await;
    match resp {
        Ok(result) => {
            let arn = result.arn().unwrap_or_default().to_string();
            println!("Scheduled deletion for: {}", arn);
            Ok(arn)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

/// List all secrets in the account, returning their names.
pub async fn list_secrets(client: &Client) -> Result<Vec<String>, Error> {
    let resp = client.list_secrets().send().await;
    match resp {
        Ok(result) => {
            let names: Vec<String> = result
                .secret_list()
                .iter()
                .filter_map(|s| s.name().map(|n| n.to_string()))
                .collect();
            for name in &names {
                println!("Secret: {}", name);
            }
            Ok(names)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

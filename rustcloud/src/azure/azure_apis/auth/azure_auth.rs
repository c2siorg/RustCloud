use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

use crate::errors::CloudError;

type HmacSha256 = Hmac<Sha256>;

pub struct AzureAuth;

impl AzureAuth {
    pub fn generate_headers(
        method: &str,
        account: &str,
        resource: &str,
    ) -> Result<(String, String), CloudError> {
        let key = env::var("AZURE_STORAGE_KEY").map_err(|_| CloudError::Auth {
            message: "AZURE_STORAGE_KEY not set".to_string(),
        })?;

        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let mut path = resource.to_string();
        let mut query = String::new();

        if let Some(pos) = resource.find('?') {
            path = resource[..pos].to_string();
            query = resource[pos + 1..].to_string();
        }

        let canonicalized_resource = if query.is_empty() {
            format!("/{}{}", account, path)
        } else {
            let mut parts = query.splitn(2, '=');
            let key = parts.next().ok_or_else(|| CloudError::Auth {
                message: "invalid query in Azure resource path".to_string(),
            })?;
            let value = parts.next().ok_or_else(|| CloudError::Auth {
                message: "invalid query in Azure resource path".to_string(),
            })?;
            format!(
                "/{}{}\n{}:{}",
                account,
                path,
                key.to_lowercase(),
                value
            )
        };

        let string_to_sign = format!(
            "{}\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:{}\nx-ms-version:2020-10-02\n{}",
            method, date, canonicalized_resource
        );

        let decoded_key =
            general_purpose::STANDARD
                .decode(key)
                .map_err(|e| CloudError::Auth {
                    message: format!("invalid AZURE_STORAGE_KEY encoding: {e}"),
                })?;

        let mut mac = HmacSha256::new_from_slice(&decoded_key).map_err(|e| CloudError::Auth {
            message: format!("invalid HMAC key: {e}"),
        })?;
        mac.update(string_to_sign.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let auth_header = format!("SharedKey {}:{}", account, signature);

        Ok((auth_header, date))
    }
}

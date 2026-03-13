use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;
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
        let canonicalized_resource = canonicalize_resource(account, resource)?;

        let string_to_sign = format!(
            "{}\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:{}\nx-ms-version:2020-10-02\n{}",
            method, date, canonicalized_resource
        );

        let decoded_key =
            general_purpose::STANDARD
                .decode(key)
                .map_err(|_| CloudError::Auth {
                    message: "AZURE_STORAGE_KEY is not valid base64".to_string(),
                })?;

        let mut mac = HmacSha256::new_from_slice(&decoded_key).map_err(|e| CloudError::Auth {
            message: format!("invalid Azure signing key: {}", e),
        })?;

        mac.update(string_to_sign.as_bytes());

        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
        let auth_header = format!("SharedKey {}:{}", account, signature);

        Ok((auth_header, date))
    }
}

fn canonicalize_resource(account: &str, resource: &str) -> Result<String, CloudError> {
    let (path, query) = match resource.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (resource, None),
    };

    let mut canonicalized = format!("/{}{}", account, path);

    if let Some(query) = query.filter(|query| !query.is_empty()) {
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }

            let (raw_key, raw_value) = pair.split_once('=').ok_or_else(|| CloudError::Auth {
                message: format!("invalid Azure resource query parameter: {}", pair),
            })?;

            let key = raw_key.to_lowercase();
            let value = raw_value.to_string();

            params
                .entry(key)
                .and_modify(|existing| {
                    existing.push(',');
                    existing.push_str(&value);
                })
                .or_insert(value);
        }

        for (key, value) in params {
            canonicalized.push('\n');
            canonicalized.push_str(&format!("{}:{}", key, value));
        }
    }

    Ok(canonicalized)
}

#[cfg(test)]
mod tests {
    use super::AzureAuth;
    use crate::errors::CloudError;
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn generate_headers_fails_when_key_missing() {
        let _guard = env_lock().lock().expect("env lock poisoned");

        unsafe {
            std::env::remove_var("AZURE_STORAGE_KEY");
        }

        let result = AzureAuth::generate_headers("GET", "account", "/?comp=list");

        assert!(matches!(result, Err(CloudError::Auth { .. })));
    }

    #[test]
    fn generate_headers_fails_when_key_not_base64() {
        let _guard = env_lock().lock().expect("env lock poisoned");

        unsafe {
            std::env::set_var("AZURE_STORAGE_KEY", "not_base64");
        }

        let result = AzureAuth::generate_headers("GET", "account", "/?comp=list");

        assert!(matches!(result, Err(CloudError::Auth { .. })));
    }

    #[test]
    fn generate_headers_succeeds_with_valid_key() {
        let _guard = env_lock().lock().expect("env lock poisoned");

        unsafe {
            std::env::set_var("AZURE_STORAGE_KEY", "MTIzNA==");
        }

        let result = AzureAuth::generate_headers(
            "PUT",
            "account",
            "/container?restype=container&timeout=30",
        )
        .expect("expected valid auth header");

        assert!(result.0.starts_with("SharedKey account:"));
        assert!(!result.1.is_empty());
    }
}

use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

pub struct AzureStorageAuth;

impl AzureStorageAuth {
    pub fn generate_headers(
        method: &str,
        account: &str,
        resource: &str,
        content_length: Option<usize>,
        extra_headers: Option<Vec<(&str, &str)>>,
    ) -> (String, String) {
        let key = env::var("AZURE_STORAGE_KEY").expect("AZURE_STORAGE_KEY not set");

        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        let mut path = resource.to_string();
        let mut query = String::new();

        if let Some(pos) = resource.find('?') {
            path = resource[..pos].to_string();
            query = resource[pos + 1..].to_string();
        }

        let mut headers: Vec<(String, String)> = vec![
            ("x-ms-date".to_string(), date.clone()),
            ("x-ms-version".to_string(), "2020-10-02".to_string()),
        ];

        if let Some(extra) = extra_headers {
            for (k, v) in extra {
                headers.push((k.to_lowercase(), v.to_string()));
            }
        }

        headers.sort_by(|a, b| a.0.cmp(&b.0));

        let canonical_headers = headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        let canonicalized_resource = if query.is_empty() {
            format!("/{}{}", account, path)
        } else {
            let mut params: Vec<&str> = query.split('&').collect();
            params.sort();

            let query_string = params
                .iter()
                .map(|p| {
                    let kv: Vec<&str> = p.split('=').collect();
                    format!("{}:{}", kv[0].to_lowercase(), kv[1])
                })
                .collect::<Vec<String>>()
                .join("\n");

            format!("/{}{}\n{}", account, path, query_string)
        };

        let content_length_str = match content_length {
            Some(len) if len > 0 => len.to_string(),
            _ => "".to_string(),
        };

        let string_to_sign = format!(
            "{}\n\n\n{}\n\n\n\n\n\n\n\n\n{}\n{}",
            method, content_length_str, canonical_headers, canonicalized_resource
        );

        let decoded_key = general_purpose::STANDARD.decode(key).unwrap();

        let mut mac = HmacSha256::new_from_slice(&decoded_key).unwrap();

        mac.update(string_to_sign.as_bytes());

        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let auth_header = format!("SharedKey {}:{}", account, signature);

        (auth_header, date)
    }
}

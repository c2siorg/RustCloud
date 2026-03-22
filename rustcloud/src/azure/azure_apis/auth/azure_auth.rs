use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
type HmacSha256 = Hmac<Sha256>;
pub struct AzureAuth;


impl AzureAuth {
    pub fn generate_headers(method: &str, account: &str, resource: &str) -> (String, String) {

        let key = env::var("AZURE_STORAGE_KEY").expect("AZURE_STORAGE_KEY not set");
        
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
            let parts: Vec<&str> = query.split('=').collect();
            format!(
                "/{}{}\n{}:{}",
                account,
                path,
                parts[0].to_lowercase(),
                parts[1]
            )
        };

        let string_to_sign = format!(
            "{}\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:{}\nx-ms-version:2020-10-02\n{}",
            method, date, canonicalized_resource
      
        );

        let decoded_key = general_purpose::STANDARD.decode(key).unwrap();
        

        let mut mac = HmacSha256::new_from_slice(&decoded_key).unwrap();
        
        mac.update(string_to_sign.as_bytes());
        
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        
        let auth_header = format!("SharedKey {}:{}", account, signature);

        (auth_header, date)

    }
}

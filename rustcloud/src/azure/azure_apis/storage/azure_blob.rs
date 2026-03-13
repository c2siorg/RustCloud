use reqwest::Client;
use std::error::Error;

use crate::azure::azure_apis::auth::azure_auth::AzureAuth;


pub struct AzureBlobClient {
    client: Client,
    account: String,
    base_url: String,
}

impl AzureBlobClient {
    pub fn new(account: String) -> Self {

        let base_url = format!("https://{}.blob.core.windows.net", account);


        AzureBlobClient {
            client: Client::new(),
            account,
            base_url,
        }
    }

    pub async fn list_containers(&self) -> Result<String, Box<dyn Error>> {
        
        let resource = "/?comp=list";
        
        let (auth, date) = AzureAuth::generate_headers("GET", &self.account, resource)?;
        

        let url = format!("{}?comp=list", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("Authorization", auth)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(format!("Azure error: {}", body).into());
        }
        
        Ok(body)

    }

    
    pub async fn create_container(&self, container: &str) -> Result<String, Box<dyn Error>> {
        
        let resource = format!("/{}?restype=container", container);
        
        let (auth, date) = AzureAuth::generate_headers("PUT", &self.account, &resource)?;
        let url = format!("{}/{}?restype=container", self.base_url, container);
        
        let response = self
            .client
            .put(&url)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("Content-Length", "0")
            .header("Authorization", auth)
            .send()
            .await?;
        
        let status = response.status();
        
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(format!("Azure error: {}", body).into());
        }
        
        Ok(body)
    }
    
    
    pub async fn delete_container(&self, container: &str) -> Result<String, Box<dyn Error>> {
        
        let resource = format!("/{}?restype=container", container);
        
        let (auth, date) = AzureAuth::generate_headers("DELETE", &self.account, &resource)?;
        
        let url = format!("{}/{}?restype=container", self.base_url, container);
        
        let response = self
            .client
            .delete(&url)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("Authorization", auth)
            .send()
            .await?;
        
        let status = response.status();
        
        let body = response.text().await?;
        
        if !status.is_success() {
            return Err(format!("Azure error: {}", body).into());
        }
        
        Ok(body)
    }
}

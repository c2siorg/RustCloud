use reqwest::Client;
use std::error::Error;

use crate::azure::azure_apis::auth::azure_storage_auth::AzureStorageAuth;

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

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, resource, None, None);

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

        println!("AZURE LIST BLOB CONTAINERS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Containers failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn create_container(&self, container: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}?restype=container", container);

        let (auth, date) =
            AzureStorageAuth::generate_headers("PUT", &self.account, &resource, None, None);
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

        println!("AZURE CREATE BLOB CONTAINER");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Container failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn delete_container(&self, container: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}?restype=container", container);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

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

        println!("AZURE DELETE BLOB CONTAINER");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Container failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn upload_blob(
        &self,
        container: &str,
        blob_name: &str,
        data: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}/{}", container, blob_name);

        let (auth, date) = AzureStorageAuth::generate_headers(
            "PUT",
            &self.account,
            &resource,
            Some(data.len()),
            Some(vec![("x-ms-blob-type", "BlockBlob")]),
        );

        let url = format!("{}/{}/{}", self.base_url, container, blob_name);

        let response = self
            .client
            .put(&url)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("x-ms-blob-type", "BlockBlob")
            .header("Content-Length", data.len())
            .header("Authorization", auth)
            .body(data)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE UPLOAD BLOB");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Upload Blob failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn list_blobs(&self, container: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}?restype=container&comp=list", container);

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, &resource, None, None);

        let url = format!(
            "{}/{}?restype=container&comp=list",
            self.base_url, container
        );

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

        println!("AZURE LIST BLOBS");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Blobs failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn delete_blob(
        &self,
        container: &str,
        blob_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}/{}", container, blob_name);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

        let url = format!("{}/{}/{}", self.base_url, container, blob_name);

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

        println!("AZURE DELETE BLOB");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Blob failed: {}", body).into());
        }

        Ok(body)
    }
}

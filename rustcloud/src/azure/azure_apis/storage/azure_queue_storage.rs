use reqwest::Client;
use std::error::Error;

use crate::azure::azure_apis::auth::azure_storage_auth::AzureStorageAuth;

pub struct AzureQueueClient {
    client: Client,
    account: String,
    base_url: String,
}

impl AzureQueueClient {
    pub fn new(account: String) -> Self {
        let base_url = format!("https://{}.queue.core.windows.net", account);

        AzureQueueClient {
            client: Client::new(),
            account,
            base_url,
        }
    }

    pub async fn list_queues(&self) -> Result<String, Box<dyn Error>> {
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

        println!("AZURE LIST QUEUES");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("List Queues failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn create_queue(&self, queue: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}", queue);

        let (auth, date) =
            AzureStorageAuth::generate_headers("PUT", &self.account, &resource, None, None);

        let url = format!("{}/{}", self.base_url, queue);

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

        println!("AZURE CREATE QUEUE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Create Queue failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn delete_queue(&self, queue: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}", queue);

        let (auth, date) =
            AzureStorageAuth::generate_headers("DELETE", &self.account, &resource, None, None);

        let url = format!("{}/{}", self.base_url, queue);

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

        println!("AZURE DELETE QUEUE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Delete Queue failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn send_message(&self, queue: &str, message: &str) -> Result<String, Box<dyn Error>> {
        let body = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            message
        );

        let resource = format!("/{}/messages", queue);

        let (auth, date) = AzureStorageAuth::generate_headers(
            "POST",
            &self.account,
            &resource,
            Some(body.len()),
            None,
        );

        let url = format!("{}/{}/messages", self.base_url, queue);

        let response = self
            .client
            .post(&url)
            .header("x-ms-date", date)
            .header("x-ms-version", "2020-10-02")
            .header("Content-Length", body.len())
            .header("Authorization", auth)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        println!("AZURE SEND MESSAGE");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Send Message failed: {}", body).into());
        }

        Ok(body)
    }

    pub async fn receive_messages(&self, queue: &str) -> Result<String, Box<dyn Error>> {
        let resource = format!("/{}/messages", queue);

        let (auth, date) =
            AzureStorageAuth::generate_headers("GET", &self.account, &resource, None, None);

        let url = format!("{}/{}/messages", self.base_url, queue);

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

        println!("AZURE RECEIVE MESSAGES");
        println!("Status  : {}", status);
        println!("Response: {}", body);

        if !status.is_success() {
            return Err(format!("Receive Messages failed: {}", body).into());
        }

        Ok(body)
    }
}

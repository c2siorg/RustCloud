use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

const SB_API_VERSION: &str = "2017-04";

pub struct AzureServiceBus {
    client: Client,
    namespace_url: String,
    token: String,
}

impl AzureServiceBus {
    pub fn new() -> Self {
        let namespace =
            env::var("AZURE_SERVICEBUS_NAMESPACE").expect("AZURE_SERVICEBUS_NAMESPACE not set");
        let token =
            env::var("AZURE_SERVICEBUS_TOKEN").expect("AZURE_SERVICEBUS_TOKEN not set");
        let namespace_url = format!("https://{}.servicebus.windows.net", namespace);
        AzureServiceBus {
            client: Client::new(),
            namespace_url,
            token,
        }
    }

    pub fn with_config(namespace: &str, token: &str) -> Self {
        AzureServiceBus {
            client: Client::new(),
            namespace_url: format!("https://{}.servicebus.windows.net", namespace),
            token: token.to_string(),
        }
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub async fn create_queue(&self, queue_name: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/{}?api-version={}", self.namespace_url, queue_name, SB_API_VERSION);
        let xml_body = format!(
            r#"<entry xmlns="http://www.w3.org/2005/Atom"><content type="application/xml"><QueueDescription xmlns="http://schemas.microsoft.com/netservices/2010/10/servicebus/connect"><LockDuration>PT1M</LockDuration><MaxSizeInMegabytes>1024</MaxSizeInMegabytes></QueueDescription></content></entry>"#
        );

        let resp = self
            .client
            .put(&url)
            .header("Authorization", self.bearer())
            .header(
                "Content-Type",
                "application/atom+xml;type=entry;charset=utf-8",
            )
            .body(xml_body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn delete_queue(&self, queue_name: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/{}?api-version={}", self.namespace_url, queue_name, SB_API_VERSION);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", self.bearer())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn list_queues(&self) -> Result<Value, Box<dyn Error>> {
        let url = format!(
            "{}/$Resources/Queues?api-version={}",
            self.namespace_url, SB_API_VERSION
        );
        let resp = self
            .client
            .get(&url)
            .header("Authorization", self.bearer())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn send_message(
        &self,
        entity: &str,
        message: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/{}/messages", self.namespace_url, entity);
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .header("Content-Type", "application/json")
            .body(message.to_string())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }

    pub async fn receive_message(&self, entity: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/{}/messages/head", self.namespace_url, entity);
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", self.bearer())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let broker_props = resp
            .headers()
            .get("BrokerProperties")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({
            "status": status,
            "broker_properties": broker_props,
            "body": body
        }))
    }

    pub async fn peek_lock_message(
        &self,
        entity: &str,
        timeout_seconds: u32,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!(
            "{}/{}/messages/head?timeout={}",
            self.namespace_url, entity, timeout_seconds
        );
        let resp = self
            .client
            .post(&url)
            .header("Authorization", self.bearer())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let broker_props = resp
            .headers()
            .get("BrokerProperties")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({
            "status": status,
            "broker_properties": broker_props,
            "body": body
        }))
    }

    pub async fn complete_message(
        &self,
        entity: &str,
        sequence_number: &str,
        lock_token: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!(
            "{}/{}/messages/{}/{}",
            self.namespace_url, entity, sequence_number, lock_token
        );
        let resp = self
            .client
            .delete(&url)
            .header("Authorization", self.bearer())
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Ok(json!({ "status": status, "body": body }))
    }
}

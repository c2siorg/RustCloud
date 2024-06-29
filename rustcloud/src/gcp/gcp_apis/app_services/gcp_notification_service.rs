use reqwest::{Client, Response};
use crate::gcp::types::app_services::gcp_notification_types::*;
use serde_json::to_string;

pub struct GCP_NotificationService {
    client: Client,
    base_url: String,
}


impl GCP_NotificationService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn create_topic(&self, request: CreateTopicRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics", self.base_url, request.name);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn delete_topic(&self, request: DeleteTopicRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics/{}", self.base_url, request.name, request.topic);
        self.client.delete(&url).send().await
    }

    pub async fn get_topic_attributes(&self, request: GetTopicAttributesRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics/{}", self.base_url, request.name,request.topic);
        self.client.get(&url).send().await
    }

    pub async fn list_subscriptions(&self, request: ListSubscriptionsRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics/{}/subscriptions", self.base_url, request.name, request.topic);
        self.client.get(&url).send().await
    }

    pub async fn create_subscription(&self, request: CreateSubscriptionRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/subscriptions", self.base_url, request.name);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn publish(&self, request: PublishRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics/{}/publish", self.base_url, request.name, request.topic);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn list_topics(&self, request: ListTopicsRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/topics", self.base_url, request.project);
        self.client.get(&url).send().await
    }

    pub async fn delete_subscription(&self, request: DeleteSubscriptionRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/v1/projects/{}/subscriptions/{}", self.base_url, request.name, request.subscription);
        self.client.delete(&url).send().await
    }
}
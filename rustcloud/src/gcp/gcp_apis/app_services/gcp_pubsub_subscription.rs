use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use reqwest::{header::AUTHORIZATION, Client, Method};
use serde_json::json;
use std::collections::HashMap;

const PUBSUB_BASE: &str = "https://pubsub.googleapis.com";

#[derive(Debug, Clone)]
pub struct GcpPubSubSubscription {
    client: Client,
}

impl GcpPubSubSubscription {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn create_subscription(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let topic = request.get("Topic").expect("Topic is required");
        let subscription = request
            .get("Subscription")
            .expect("Subscription is required");
        let ack_deadline = request
            .get("AckDeadlineSeconds")
            .map(|s| s.parse::<u32>().unwrap_or(10))
            .unwrap_or(10);

        let url = format!(
            "{}/v1/projects/{}/subscriptions/{}",
            PUBSUB_BASE, project, subscription
        );

        let body = json!({
            "topic": format!("projects/{}/topics/{}", project, topic),
            "ackDeadlineSeconds": ack_deadline
        })
        .to_string();

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::PUT, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }

    pub async fn delete_subscription(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let subscription = request
            .get("Subscription")
            .expect("Subscription is required");

        let url = format!(
            "{}/v1/projects/{}/subscriptions/{}",
            PUBSUB_BASE, project, subscription
        );

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::DELETE, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }

    pub async fn list_subscriptions(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let url = format!(
            "{}/v1/projects/{}/subscriptions",
            PUBSUB_BASE, project
        );

        let mut list_req = self.client.request(Method::GET, &url);

        if let Some(page_size) = request.get("PageSize") {
            list_req = list_req.query(&[("pageSize", page_size)]);
        }
        if let Some(page_token) = request.get("PageToken") {
            list_req = list_req.query(&[("pageToken", page_token)]);
        }

        let token = retrieve_token().await?;
        let response = list_req
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }

    pub async fn pull_messages(
        &self,
        request: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let subscription = request
            .get("Subscription")
            .expect("Subscription is required");
        let max_messages = request
            .get("MaxMessages")
            .map(|s| s.parse::<u32>().unwrap_or(10))
            .unwrap_or(10);

        let url = format!(
            "{}/v1/projects/{}/subscriptions/{}:pull",
            PUBSUB_BASE, project, subscription
        );

        let body = json!({ "maxMessages": max_messages }).to_string();

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }

    pub async fn acknowledge_messages(
        &self,
        project: &str,
        subscription: &str,
        ack_ids: Vec<String>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/subscriptions/{}:acknowledge",
            PUBSUB_BASE, project, subscription
        );

        let body = json!({ "ackIds": ack_ids }).to_string();

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }

    pub async fn modify_push_config(
        &self,
        project: &str,
        subscription: &str,
        push_endpoint: Option<&str>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/subscriptions/{}:modifyPushConfig",
            PUBSUB_BASE, project, subscription
        );

        let push_config = match push_endpoint {
            Some(ep) => json!({ "pushEndpoint": ep }),
            None => json!({}),
        };
        let body = json!({ "pushConfig": push_config }).to_string();

        let token = retrieve_token().await?;
        let response = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(body)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut result = HashMap::new();
        result.insert("status".to_string(), status);
        result.insert("body".to_string(), body);
        Ok(result)
    }
}

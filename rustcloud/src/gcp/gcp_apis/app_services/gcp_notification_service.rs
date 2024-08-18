use reqwest::{Client, Method, header::AUTHORIZATION};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

// Assuming the token retrieval function is in a module named 'auth'
use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;

#[derive(Debug, Clone)]
pub struct Googlenotification {
    client: Client,
    base_url: String,
}

impl Googlenotification {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://pubsub.googleapis.com".to_string()
        }
    }

    pub async fn list_topic(&self, request: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let url = format!("{}/v1/projects/{}/topics", self.base_url, project);

        let mut list_topic_request = self.client.request(Method::GET, &url);

        if let Some(page_size) = request.get("PageSize") {
            list_topic_request = list_topic_request.query(&[("pageSize", page_size)]);
        }

        if let Some(page_token) = request.get("PageToken") {
            list_topic_request = list_topic_request.query(&[("pageToken", page_token)]);
        }

        let token = retrieve_token().await?;
        list_topic_request = list_topic_request
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token));

        let response = list_topic_request.send().await?;
        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut list_topic_response = HashMap::new();
        list_topic_response.insert("status".to_string(), status);
        list_topic_response.insert("body".to_string(), body);

        Ok(list_topic_response)
    }

    pub async fn get_topic(&self, request: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let topic = request.get("Topic").expect("Topic is required");
        let url = format!("{}/v1/projects/{}/topics/{}", self.base_url, project, topic);

        let token = retrieve_token().await?;
        let response = self.client
            .request(Method::GET, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut get_topic_response = HashMap::new();
        get_topic_response.insert("status".to_string(), status);
        get_topic_response.insert("body".to_string(), body);

        Ok(get_topic_response)
    }

    pub async fn delete_topic(&self, request: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let topic = request.get("Topic").expect("Topic is required");
        let url = format!("{}/v1/projects/{}/topics/{}", self.base_url, project, topic);

        let token = retrieve_token().await?;
        let response = self.client
            .request(Method::DELETE, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut delete_topic_response = HashMap::new();
        delete_topic_response.insert("status".to_string(), status);
        delete_topic_response.insert("body".to_string(), body);

        Ok(delete_topic_response)
    }

    pub async fn create_topic(&self, request: HashMap<String, String>) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let project = request.get("Project").expect("Project is required");
        let topic = request.get("Topic").expect("Topic is required");
        let url = format!("{}/v1/projects/{}/topics/{}", self.base_url, project, topic);

        let create_topic_json_map: HashMap<String, String> = HashMap::new();
        let create_topic_json = json!(create_topic_json_map).to_string();

        let token = retrieve_token().await?;
        let response = self.client
            .request(Method::PUT, &url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(create_topic_json)
            .send()
            .await?;

        let status = response.status().as_u16().to_string();
        let body = response.text().await?;

        let mut create_topic_response = HashMap::new();
        create_topic_response.insert("status".to_string(), status);
        create_topic_response.insert("body".to_string(), body);

        Ok(create_topic_response)
    }
}
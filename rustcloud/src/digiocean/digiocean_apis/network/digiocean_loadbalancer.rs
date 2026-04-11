use crate::errors::CloudError;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::HashMap;

pub struct DigiOceanLoadBalancer {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl DigiOceanLoadBalancer {
    pub fn new(token: String) -> Self {
        DigiOceanLoadBalancer {
            client: reqwest::Client::new(),
            base_url: "https://api.digitalocean.com/v2".to_string(),
            token,
        }
    }

    pub async fn create_load_balancer(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, CloudError> {
        let url = format!("{}/load_balancers", self.base_url);
        let body = serde_json::to_string(&request)?;

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .body(body)
            .send()
            .await?;

        let mut response: HashMap<String, Value> = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(resp.status().as_u16().into()),
        );
        let body = resp.text().await.unwrap_or_default();
        response.insert("body".to_string(), Value::String(body));
        Ok(response)
    }

    pub async fn delete_load_balancer(
        &self,
        lb_id: &str,
    ) -> Result<HashMap<String, Value>, CloudError> {
        let url = format!("{}/load_balancers/{}", self.base_url, lb_id);

        let resp = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .await?;

        let mut response: HashMap<String, Value> = HashMap::new();
        response.insert(
            "status".to_string(),
            Value::Number(resp.status().as_u16().into()),
        );
        let body = resp.text().await.unwrap_or_default();
        response.insert("body".to_string(), Value::String(body));
        Ok(response)
    }
}

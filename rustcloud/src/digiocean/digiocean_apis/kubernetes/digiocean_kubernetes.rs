use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::HashMap;

pub struct DigiOceanKubernetes {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl DigiOceanKubernetes {
    pub fn new(token: String) -> Self {
        DigiOceanKubernetes {
            client: reqwest::Client::new(),
            base_url: "https://api.digitalocean.com/v2".to_string(),
            token,
        }
    }

    pub async fn create_cluster(
        &self,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!("{}/kubernetes/clusters", self.base_url);
        let body = serde_json::to_string(&request).unwrap();

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

    pub async fn delete_cluster(
        &self,
        cluster_id: &str,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!("{}/kubernetes/clusters/{}", self.base_url, cluster_id);

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

    pub async fn list_clusters(&self) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!("{}/kubernetes/clusters", self.base_url);

        let resp = self
            .client
            .get(&url)
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

    pub async fn get_cluster(
        &self,
        cluster_id: &str,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!("{}/kubernetes/clusters/{}", self.base_url, cluster_id);

        let resp = self
            .client
            .get(&url)
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

    pub async fn add_node_pool(
        &self,
        cluster_id: &str,
        request: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!(
            "{}/kubernetes/clusters/{}/node_pools",
            self.base_url, cluster_id
        );
        let body = serde_json::to_string(&request).unwrap();

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

    pub async fn delete_node_pool(
        &self,
        cluster_id: &str,
        pool_id: &str,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!(
            "{}/kubernetes/clusters/{}/node_pools/{}",
            self.base_url, cluster_id, pool_id
        );

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

    pub async fn get_kubeconfig(
        &self,
        cluster_id: &str,
    ) -> Result<HashMap<String, Value>, reqwest::Error> {
        let url = format!(
            "{}/kubernetes/clusters/{}/kubeconfig",
            self.base_url, cluster_id
        );

        let resp = self
            .client
            .get(&url)
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

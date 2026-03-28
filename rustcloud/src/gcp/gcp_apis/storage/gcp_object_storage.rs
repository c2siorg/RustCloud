use crate::gcp::gcp_apis::auth::gcp_auth::retrieve_token;
use reqwest::{header::AUTHORIZATION, Client};
use serde_json::{json, Value};

const GCS_BASE: &str = "https://storage.googleapis.com/storage/v1";
const GCS_UPLOAD: &str = "https://storage.googleapis.com/upload/storage/v1";

/// Client for Google Cloud Storage bucket and object operations.
pub struct GcsClient {
    client: Client,
    project_id: String,
}

impl GcsClient {
    pub fn new(project_id: &str) -> Self {
        Self {
            client: Client::new(),
            project_id: project_id.to_string(),
        }
    }

    // ── Bucket operations ─────────────────────────────────────────────────────

    /// Create a new bucket in the given location (e.g. `"US-EAST1"`, `"EU"`).
    pub async fn create_bucket(
        &self,
        bucket_name: &str,
        location: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/b?project={}", GCS_BASE, self.project_id);
        let token = retrieve_token().await?;
        let body = json!({
            "name": bucket_name,
            "location": location,
        });

        let resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("create_bucket '{}': status {}", bucket_name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// Delete a bucket. The bucket must be empty before deletion.
    pub async fn delete_bucket(
        &self,
        bucket_name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/b/{}", GCS_BASE, bucket_name);
        let token = retrieve_token().await?;

        let resp = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        println!("delete_bucket '{}': status {}", bucket_name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// List all buckets in the project.
    pub async fn list_buckets(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/b?project={}", GCS_BASE, self.project_id);
        let token = retrieve_token().await?;

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("list_buckets: status {}", status);
        Ok(json!({ "status": status, "body": body }))
    }

    // ── Object operations ─────────────────────────────────────────────────────

    /// Upload bytes to `bucket/object_name` with the given content type.
    /// Uses the GCS simple media upload endpoint.
    pub async fn upload_object(
        &self,
        bucket_name: &str,
        object_name: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/b/{}/o?uploadType=media&name={}",
            GCS_UPLOAD, bucket_name, object_name
        );
        let token = retrieve_token().await?;

        let resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header("Content-Type", content_type)
            .body(data)
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("upload_object '{}/{}': status {}", bucket_name, object_name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// Download an object and return its raw bytes.
    pub async fn download_object(
        &self,
        bucket_name: &str,
        object_name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // URL-encode the object name to handle slashes and special characters
        let encoded_name = object_name.replace('/', "%2F");
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            GCS_BASE, bucket_name, encoded_name
        );
        let token = retrieve_token().await?;

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        println!(
            "download_object '{}/{}': status {}",
            bucket_name,
            object_name,
            resp.status().as_u16()
        );
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Delete a single object from a bucket.
    pub async fn delete_object(
        &self,
        bucket_name: &str,
        object_name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let encoded_name = object_name.replace('/', "%2F");
        let url = format!("{}/b/{}/o/{}", GCS_BASE, bucket_name, encoded_name);
        let token = retrieve_token().await?;

        let resp = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        println!("delete_object '{}/{}': status {}", bucket_name, object_name, status);
        Ok(json!({ "status": status, "body": body }))
    }

    /// List objects in a bucket, optionally filtered by name prefix.
    pub async fn list_objects(
        &self,
        bucket_name: &str,
        prefix: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url = format!("{}/b/{}/o", GCS_BASE, bucket_name);
        if let Some(p) = prefix {
            url = format!("{}?prefix={}", url, p);
        }
        let token = retrieve_token().await?;

        let resp = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        let status = resp.status().as_u16();
        let body: Value = resp.json().await?;
        println!("list_objects '{}': status {}", bucket_name, status);
        Ok(json!({ "status": status, "body": body }))
    }
}

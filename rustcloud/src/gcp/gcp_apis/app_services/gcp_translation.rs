use reqwest::{Client, Response};
use serde_json::to_string;
use crate::gcp::types::app_services::gcp_translation_types::*;

pub struct GCP_Translation {
    client: Client,
    base_url: String,
}


impl GCP_Translation {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn translate(&self, request: TranslateRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/translateText", self.base_url);
        let body= to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn batch_translate(&self, request: BatchTranslateRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/batchTranslateText", self.base_url);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn get_glossary(&self, request: GlossaryName) -> Result<Response, reqwest::Error> {
        let url = format!("{}/glossaries/{}", self.base_url, request.name);
        let body = to_string(&request).unwrap();
        self.client.get(body).send().await
    }

    pub async fn create_glossary(&self, request: CreateGlossaryRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/glossaries", self.base_url);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }

    pub async fn detect_language(&self, request: DetectLanguageRequest) -> Result<Response, reqwest::Error> {
        let url = format!("{}/detectLanguage", self.base_url);
        let body = to_string(&request).unwrap();
        self.client.post(&url).body(body).send().await
    }
}
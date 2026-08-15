use crate::gcp::gcp_apis::auth::gcp_auth::DefaultTokenProvider;
use crate::gcp::types::artificial_intelligence::gcp_automl_types::*;
use crate::traits::token_provider::TokenProvider;
use reqwest::{header::AUTHORIZATION, Client, Response};
use serde_json::to_string;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AutoML {
    client: Client,
    base_url: String,
    project_id: String,
    auth: Arc<dyn TokenProvider>,
}

impl AutoML {
    pub fn new(project_id: &str) -> Self {
        Self::with_http_client(
            Client::new(),
            project_id,
            "https://automl.googleapis.com".to_string(),
            Arc::new(DefaultTokenProvider),
        )
    }

    pub fn with_http_client(
        client: Client,
        project_id: &str,
        base_url: String,
        auth: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            client,
            base_url,
            project_id: project_id.to_string(),
            auth,
        }
    }

    async fn get_authorization_header(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.auth.get_token().await.map_err(|e| e.to_string())?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn create_dataset(
        &self,
        location: &str,
        name: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/datasets",
            self.base_url, self.project_id, location
        );
        let request = CreateDatasetRequest {
            parent: format!("projects/{}/locations/{}", self.project_id, location),
            dataset: Dataset {
                display_name: name.to_string(),
                tables_dataset_metadata: HashMap::new(),
            },
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_dataset(
        &self,
        location: &str,
        dataset_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/datasets/{}",
            self.base_url, self.project_id, location, dataset_id
        );
        let auth_header = self.get_authorization_header().await?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn import_data_set(
        &self,
        location: &str,
        dataset_id: &str,
        uris: Vec<String>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/datasets/{}:importData",
            self.base_url, self.project_id, location, dataset_id
        );
        let request = ImportDataSetRequest {
            name: format!(
                "projects/{}/locations/{}/datasets/{}",
                self.project_id, location, dataset_id
            ),
            input_config: InputConfig {
                gcs_source: GcsSource { input_uris: uris },
            },
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn list_models(
        &self,
        location: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models",
            self.base_url, self.project_id, location
        );
        let auth_header = self.get_authorization_header().await?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn create_model(
        &self,
        location: &str,
        dataset_id: &str,
        model_name: &str,
        column_id: &str,
        train_budget: i64,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models",
            self.base_url, self.project_id, location
        );
        let request = CreateModelRequest {
            parent: format!("projects/{}/locations/{}", self.project_id, location),
            model: Model {
                dataset_id: dataset_id.to_string(),
                display_name: model_name.to_string(),
                tables_model_metadata: TablesModelMetadata {
                    target_column_spec: TargetColumnSpec {
                        name: column_id.to_string(),
                    },
                    train_budget_milli_node_hours: train_budget,
                },
            },
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn deploy_model(
        &self,
        location: &str,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models/{}:deploy",
            self.base_url, self.project_id, location, model_id
        );
        let request = DeployModelRequest {
            name: format!(
                "projects/{}/locations/{}/models/{}",
                self.project_id, location, model_id
            ),
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn undeploy_model(
        &self,
        location: &str,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models/{}:undeploy",
            self.base_url, self.project_id, location, model_id
        );
        let request = UndeployModelRequest {
            name: format!(
                "projects/{}/locations/{}/models/{}",
                self.project_id, location, model_id
            ),
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_model(
        &self,
        location: &str,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models/{}",
            self.base_url, self.project_id, location, model_id
        );
        let auth_header = self.get_authorization_header().await?;
        self.client
            .get(&url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn export_dataset(
        &self,
        location: &str,
        dataset_id: &str,
        gcs_uri: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/datasets/{}:exportData",
            self.base_url, self.project_id, location, dataset_id
        );
        let request = ExportDatasetRequest {
            name: format!(
                "projects/{}/locations/{}/datasets/{}",
                self.project_id, location, dataset_id
            ),
            output_config: OutputConfig {
                gcs_destination: GcsDestination {
                    output_uri_prefix: gcs_uri.to_string(),
                },
            },
        };
        let body = to_string(&request).unwrap();
        let auth_header = self.get_authorization_header().await?;
        self.client
            .post(&url)
            .header(AUTHORIZATION, auth_header)
            .body(body)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_model(
        &self,
        location: &str,
        model_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/models/{}",
            self.base_url, self.project_id, location, model_id
        );
        let auth_header = self.get_authorization_header().await?;
        self.client
            .delete(&url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_dataset(
        &self,
        location: &str,
        dataset_id: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/projects/{}/locations/{}/datasets/{}",
            self.base_url, self.project_id, location, dataset_id
        );
        let auth_header = self.get_authorization_header().await?;
        self.client
            .delete(&url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

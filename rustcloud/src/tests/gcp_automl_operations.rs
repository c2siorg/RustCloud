use std::sync::Arc;

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::artificial_intelligence::gcp_automl::AutoML;
use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;

async fn mock_client(server: &MockServer) -> AutoML {
    AutoML::with_http_client(
        reqwest::Client::new(),
        "test-project",
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_create_dataset() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/test-project/locations/us-central1/datasets"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.create_dataset("us-central1", "test-dataset").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_get_dataset() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/datasets/test-dataset",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.get_dataset("us-central1", "test-dataset").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_import_data_set() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/datasets/test-dataset:importData",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client
        .import_data_set(
            "us-central1",
            "test-dataset",
            vec!["gs://bucket/file.csv".to_string()],
        )
        .await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_list_models() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/test-project/locations/us-central1/models"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.list_models("us-central1").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_create_model() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/test-project/locations/us-central1/models"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client
        .create_model("us-central1", "test-dataset", "test-model", "target_col", 1000)
        .await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_deploy_model() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/models/test-model:deploy",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.deploy_model("us-central1", "test-model").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_undeploy_model() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/models/test-model:undeploy",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.undeploy_model("us-central1", "test-model").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_get_model() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/models/test-model",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.get_model("us-central1", "test-model").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_export_dataset() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/datasets/test-dataset:exportData",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client
        .export_dataset("us-central1", "test-dataset", "gs://bucket/export/")
        .await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_delete_model() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/models/test-model",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.delete_model("us-central1", "test-model").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_delete_dataset() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/v1/projects/test-project/locations/us-central1/datasets/test-dataset",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.delete_dataset("us-central1", "test-dataset").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
#[ignore]
async fn test_list_models_live() {
    let client = AutoML::new("your_project_id");
    let result = client.list_models("your_location").await;
    assert!(result.is_ok(), "{:?}", result.err());
}

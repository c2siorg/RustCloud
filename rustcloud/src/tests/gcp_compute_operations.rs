use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::gcp::gcp_apis::compute::gcp_compute_engine::GCE;

async fn mock_client(server: &MockServer) -> GCE {
    GCE::with_http_client(
        reqwest::Client::new(),
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_create_node() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/projects/test-project/zones/us-central1-a/instances"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), json!("test-project"));
    request.insert("Zone".to_string(), json!("us-central1-a"));
    request.insert("Name".to_string(), json!("test-instance"));

    let result = client.create_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_start_node() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/instances/test-instance/start",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "test-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "test-instance".to_string());

    let result = client.start_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_stop_node() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/instances/test-instance/stop",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "test-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "test-instance".to_string());

    let result = client.stop_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_delete_node() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/instances/test-instance",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "test-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "test-instance".to_string());

    let result = client.delete_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_reboot_node() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/instances/test-instance/reset",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "test-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "test-instance".to_string());

    let result = client.reboot_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_list_node() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/instances/",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "test-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());

    let result = client.list_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
#[ignore]
async fn test_list_node_live() {
    let client = GCE::new();
    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "your_project_id".to_string());
    request.insert("Zone".to_string(), "your_zone".to_string());

    let result = client.list_node(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

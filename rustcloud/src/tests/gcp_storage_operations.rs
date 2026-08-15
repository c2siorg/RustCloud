use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::gcp::gcp_apis::storage::gcp_storage::GoogleStorage;

async fn mock_client(server: &MockServer) -> GoogleStorage {
    GoogleStorage::with_http_client(
        reqwest::Client::new(),
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_create_disk() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/projects/test-project/zones/us-central1-a/disks"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("projectid".to_string(), json!("test-project"));
    params.insert("Name".to_string(), json!("test-disk"));
    params.insert("Zone".to_string(), json!("us-central1-a"));
    params.insert("Type".to_string(), json!("pd-standard"));
    params.insert("SizeGb".to_string(), json!(10));

    let result = client.create_disk(params).await;
    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"].as_u64().unwrap(), 200);
}

#[tokio::test]
async fn test_delete_disk() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/disks/test-disk",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("projectid".to_string(), "test-project".to_string());
    params.insert("Zone".to_string(), "us-central1-a".to_string());
    params.insert("disk".to_string(), "test-disk".to_string());

    let result = client.delete_disk(params).await;
    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"].as_u64().unwrap(), 200);
}

#[tokio::test]
async fn test_create_snapshot() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/projects/test-project/zones/us-central1-a/disks/test-disk/createSnapshot",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("projectid".to_string(), json!("test-project"));
    params.insert("Name".to_string(), json!("test-snapshot"));
    params.insert("Zone".to_string(), json!("us-central1-a"));
    params.insert("disk".to_string(), json!("test-disk"));

    let result = client.create_snapshot(params).await;
    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"].as_u64().unwrap(), 200);
}

#[tokio::test]
#[ignore]
async fn test_create_disk_live() {
    let client = GoogleStorage::new();
    let mut params = HashMap::new();
    params.insert("projectid".to_string(), json!("your_project_id"));
    params.insert("Name".to_string(), json!("test-disk"));
    params.insert("Zone".to_string(), json!("us-central1-a"));
    params.insert("Type".to_string(), json!("pd-standard"));
    params.insert("SizeGb".to_string(), json!(10));

    let result = client.create_disk(params).await;
    assert!(result.is_ok(), "{:?}", result.err());
}
